//! File > Open for Publisher 97, 98 and 2000 files: EXPERIMENTAL text recovery.
//!
//! The 2002+ importer runs first and is never changed; this path is reached only when it has
//! refused the file as an older family *and* the file's Contents carries the older discriminator.
//! The reader (`designcraft-pub-legacy`) returns text, provenance and a raw size candidate; this
//! adapter lays that text out as plain recovery pages. It makes no claim about the original layout.
//! See docs/pub-format/legacy-open-notice.md.

use designcraft_doc::build::NewDocument;
use designcraft_doc::{Document, ParaFormat, SpreadRef};
use designcraft_geom::Rect;
use designcraft_pub_legacy::{LegacyText, Scope, Source, read_legacy};
use serde_json::{Value, json};
use std::collections::HashMap;

use super::{bad, str_param};
use crate::{DocState, EngineError, Result, Session};

const DETAIL: &str = "Shapes, fills, tables, most colours and the exact page order are not recovered. Text positions, fonts, bitmap pictures and the stored drawings of WordArt and clip art are recovered only where the stored numbers could be read, and may be wrong. The order of recovered text may not be the reading order. Keep the original file. If possible, open it in Publisher 2002 or later and save a new copy.";
const NOTICE_OK: &str = concat!(
    "Experimental text recovery. Recovered text is ready to edit. This is a new document, not the original layout. ",
    "Shapes, fills, tables, most colours and the exact page order are not recovered. Text positions, fonts, bitmap pictures and the stored drawings of WordArt and clip art are recovered only where the stored numbers could be read, and may be wrong. The order of recovered text may not be the reading order. Keep the original file. If possible, open it in Publisher 2002 or later and save a new copy."
);
const NOTICE_PARTIAL: &str = concat!(
    "Experimental text recovery. Some text could not be recovered. Missing text is reported below; text we could not read is marked in the document. This is a new document, not the original layout. ",
    "Shapes, fills, tables, most colours and the exact page order are not recovered. Text positions, fonts, bitmap pictures and the stored drawings of WordArt and clip art are recovered only where the stored numbers could be read, and may be wrong. The order of recovered text may not be the reading order. Keep the original file. If possible, open it in Publisher 2002 or later and save a new copy."
);
const NOTICE_EMPTY: &str = concat!(
    "Experimental text recovery found no text in the supported text store. This does not mean the original file is blank. The empty page is a recovery workspace, not an original page. ",
    "Shapes, fills, tables, most colours and the exact page order are not recovered. Text positions, fonts, bitmap pictures and the stored drawings of WordArt and clip art are recovered only where the stored numbers could be read, and may be wrong. The order of recovered text may not be the reading order. Keep the original file. If possible, open it in Publisher 2002 or later and save a new copy."
);
const PLACEHOLDER: &str = "[Text could not be read. No replacement text was guessed.]";
const A4: (f64, f64) = (595.276, 841.89);
const MARGIN: f64 = 36.0;
const MAX_PAGES: usize = 2000;

/// The largest side accepted as a recovered page. The stored size is only a raw candidate whose units are
/// not verified, so a size far beyond a normal sheet is not shown as if it were the original page.
const MAX_PLAUSIBLE_MM: f64 = 450.0;

/// The recovery page size, its status, and whether a stored candidate was rejected as implausible.
fn page_size(r: &LegacyText) -> ((f64, f64), &'static str, bool) {
    let mut rejected = false;
    if let Some(size) = r.size {
        let (w, h) = (size.inferred_mm.0 * 72.0 / 25.4, size.inferred_mm.1 * 72.0 / 25.4);
        let max_pt = MAX_PLAUSIBLE_MM * 72.0 / 25.4;
        if (72.0..=max_pt).contains(&w) && (72.0..=max_pt).contains(&h) {
            return ((w, h), "inferred-from-raw", false);
        }
        rejected = true;
    }
    (A4, "default-A4", rejected)
}

/// The most source text kept per segment in the command result, and how many segments keep any.
const MAX_SOURCE_CHARS: usize = 4096;
const MAX_SOURCE_SEGMENTS: usize = 1000;

/// Paragraph text for a frame: CR LF and CR become paragraph ends; form feeds and other controls
/// are not turned into anything visible.
fn visible(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut chars = raw.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\r' => {
                if chars.peek() == Some(&'\n') {
                    chars.next();
                }
                out.push('\n');
            }
            '\n' => out.push('\n'),
            '\t' => out.push('\t'),
            c if (c as u32) < 0x20 => {}
            c => out.push(c),
        }
    }
    out.trim_end_matches('\n').to_string()
}

/// The stored rectangle of a story's first positioned record, converted to page points (stored values are
/// centered on the page, in 1/914400 inch), and the candidate page container. `None` unless the whole box
/// lies on the page, so a bad number can never put a frame off the page.
fn stored_rect(r: &LegacyText, seg: &designcraft_pub_legacy::Segment, w: f64, h: f64) -> Option<(u16, Rect, designcraft_pub::FrameLook)> {
    let Source::Story { records, .. } = &seg.source else { return None };
    let f = r.frames.iter().find(|f| records.contains(&f.record))?;
    let (container, rect) = frame_to_rect(f, r.size?, w, h)?;
    Some((container, rect, frame_look(f)))
}

/// OBSERVATION (Advertisement - Hotels, 17 frames): a frame whose byte at +0x2a is 2 and whose colour reference
/// is the "automatic" 0xC0E40000 shows as a dark band behind white text, so it is drawn with a black fill and
/// white automatic text.
///
/// OBSERVATION (donated files, stored page previews, 20 frames with byte +0x2a = 2): a reference whose top byte
/// is 0x20 carries a literal colour, red in the lowest byte (the byte order the text colours use). In Wireless
/// Flyer the full-page frame `a52cc320` shows as the purple page background; in Flyer 200201 and 200204 the
/// `cccc9920` frames show as the grey column and the `ffffcc20` frames as the white box above it. The previews
/// hold only 16 colours, so the exact shade is not checked. INFERENCE: those frames have that literal fill.
/// UNKNOWN: the small scheme numbers and the 0x1a/0x1b references (no fill is drawn for them), fill transparency,
/// and the frames whose byte +0x2a has other bits set (0x8c, 0x8e): their references are not read.
fn frame_look(f: &designcraft_pub_legacy::FrameRect) -> designcraft_pub::FrameLook {
    // A border is drawn black: its colour reference is the automatic one in every frame that has a border.
    let border = (f.border > 0).then_some(f64::from(f.border).clamp(0.25, 36.0));
    if f.fill_mode == 2 && f.color_ref == 0xC0E4_0000 {
        designcraft_pub::FrameLook { fill: Some([0, 0, 0]), text: Some([255, 255, 255]), border }
    } else if f.fill_mode == 2 && f.color_ref >> 24 == 0x20 {
        let [r, g, b, _] = f.color_ref.to_le_bytes();
        designcraft_pub::FrameLook { fill: Some([r, g, b]), border, ..Default::default() }
    } else {
        designcraft_pub::FrameLook { border, ..Default::default() }
    }
}

/// A stored rectangle (centered on the page, 1/914400 inch) as a page rectangle in points, and its candidate
/// page container. `None` unless the whole box lies on the page.
fn frame_to_rect(f: &designcraft_pub_legacy::FrameRect, size: designcraft_pub_legacy::SizeCandidate, w: f64, h: f64) -> Option<(u16, Rect)> {
    let pt = |v: i32, half: u32| (f64::from(v) + f64::from(half) / 2.0) / 12700.0;
    let (l, t) = (pt(f.left, size.raw_width), pt(f.top, size.raw_height));
    let (rt, b) = (pt(f.right, size.raw_width), pt(f.bottom, size.raw_height));
    let slack = 1.0;
    if l < -slack || t < -slack || rt > w + slack || b > h + slack || rt - l < 4.0 || b - t < 4.0 {
        return None;
    }
    Some((f.container, Rect::new(l.max(0.0), t.max(0.0), rt.min(w), b.min(h))))
}

/// A rough frame height: 14.4 pt per line at about 6 pt per character, capped at the page body.
fn frame_height(text: &str, width: f64, body: f64) -> f64 {
    let per_line = (width / 6.0).max(8.0);
    let lines: f64 = text.split('\n').map(|p| (p.chars().count() as f64 / per_line).ceil().max(1.0)).sum();
    (lines * 14.4 + 8.0).clamp(24.0, body)
}

pub(crate) fn open_legacy(s: &mut Session, p: &Value, bytes: &[u8], title: String, refusal: &str) -> Result<Value> {
    let refuse = |reason: String| {
        EngineError::Other(format!(
            "can't open the Publisher file: unsupported Publisher file: {refusal}\nExperimental text recovery could not open this file. Reason: {reason}"
        ))
    };
    let r = read_legacy(bytes).map_err(|e| refuse(e.to_string()))?;
    let decoded = r.segments.iter().filter(|g| g.text.is_some()).count();
    let undecoded = r.segments.len() - decoded;
    let encoding_unknown = r.segments.iter().filter(|g| g.encoding_unknown).count();
    if r.segments.is_empty() && r.scope != Scope::EmptyKnownStore {
        let why = r
            .warnings
            .iter()
            .find(|w| w.contains("refused"))
            .or_else(|| r.warnings.iter().find(|w| !w.contains("raw candidate") && !w.contains("text and a size candidate only")))
            .cloned()
            .unwrap_or_else(|| "no text was recovered".into());
        return Err(refuse(why));
    }
    let ((w, h), size_status, size_rejected) = page_size(&r);
    // Lay the frames out first, then build the document with exactly that many pages.
    // Margins shrink on a very small page so a frame never reaches the page edge.
    let margin = MARGIN.min(w / 4.0).min(h / 4.0);
    let body_w = w - 2.0 * margin;
    let body_h = h - 2.0 * margin;
    let mut placed: Vec<(usize, Rect, String)> = Vec::new();
    let mut trimmed_segments = 0usize;
    let mut source_text: Vec<Value> = Vec::new();
    let (mut page, mut y) = (0usize, margin);
    // Stories whose record carried a plausible stored rectangle keep that position, one page per candidate
    // container in order of first appearance (page identity and order are not established). Stories without a
    // position keep the plain recovery flow on pages after them.
    let mut positioned: Vec<Positioned> = Vec::new();
    // The Quill store also carries fonts, sizes, styles and alignment; it is only used for stories placed at a stored rectangle.
    let formats = if matches!(r.family, designcraft_pub_legacy::Family::A | designcraft_pub_legacy::Family::B) {
        designcraft_pub_legacy::quill_bytes(bytes).and_then(|q| designcraft_pub::StoryFormats::parse(&q).ok())
    } else {
        None
    };
    let mut format_notes: Vec<String> = Vec::new();
    let mut formatted_frames = 0usize;
    let mut containers: Vec<u16> = Vec::new();
    let mut ranks: HashMap<u16, u32> = r.frames.iter().map(|f| (f.container, f.container_rank)).collect();
    let page_rects = size_status == "inferred-from-raw";
    for (index, seg) in r.segments.iter().enumerate() {
        let text = match &seg.text {
            Some(t) => {
                let shown = visible(t);
                // Form feeds, other controls and extra trailing paragraph marks are not shown; say so.
                // One closing paragraph mark is the normal end of a story and is not counted.
                let kept = t.chars().filter(|c| !matches!(c, '\r' | '\n' | '\t') && (*c as u32) >= 0x20).count();
                let trailing_marks = t.chars().rev().take_while(|c| matches!(c, '\r' | '\n')).count();
                if t.chars().any(|c| (c as u32) < 0x20 && !matches!(c, '\r' | '\n' | '\t'))
                    || shown.chars().filter(|c| *c != '\n' && *c != '\t').count() != kept
                    || trailing_marks > 2
                    || t.ends_with("\r\r")
                    || t.ends_with("\n\n")
                {
                    trimmed_segments += 1;
                    // Keep the stored text, controls included, in the command result only (bounded).
                    if source_text.len() < MAX_SOURCE_SEGMENTS {
                        let total = t.chars().count();
                        let kept_text: String = t.chars().take(MAX_SOURCE_CHARS).collect();
                        source_text.push(json!({"segment": index, "text": kept_text, "truncated": total > MAX_SOURCE_CHARS}));
                    }
                }
                shown
            }
            None => PLACEHOLDER.to_string(),
        };
        if page_rects && let Some((container, rect, look)) = stored_rect(&r, seg, w, h) {
            let pg = containers.iter().position(|c| *c == container).unwrap_or_else(|| {
                containers.push(container);
                containers.len() - 1
            });
            let story = if let Source::Story { id, .. } = &seg.source { Some(*id) } else { None };
            positioned.push((pg, rect, text, story, look));
            continue;
        }
        let height = frame_height(&text, body_w, body_h);
        if y > margin && y + height > h - margin {
            page += 1;
            y = margin;
        }
        if page >= MAX_PAGES {
            return Err(refuse("more recovered text than the page limit".into()));
        }
        placed.push((page, Rect::new(margin, y, margin + body_w, y + height), text));
        y += height + 8.0;
    }
    let (mut art, art_notes) = collect_art(bytes, &r, page_rects, (w, h), &mut containers, &mut ranks);
    order_pages(&mut containers, &ranks, &mut positioned, &mut art);
    let positioned_pages = containers.len();
    let flow_pages = if placed.is_empty() { 0 } else { page + 1 };
    let nd = NewDocument {
        title: title.clone(),
        width: w,
        height: h,
        pages: (positioned_pages + flow_pages).max(1),
        facing_pages: false,
        primary_text_frame: false,
        ..Default::default()
    };
    let mut d = Document::new(&nd);
    let layer = d.layers.first().map(|l| l.id).ok_or_else(|| bad("file.open", "new document has no layer"))?;
    let (placed_previews, placed_pictures) = place_art(&mut d, layer, &art);
    let mut frame_ids: Vec<u64> = Vec::new();
    for (pg, rect, text, story, look) in &positioned {
        // Fonts, sizes, styles and alignment come from the Quill store when it has this story; any failure
        // leaves the plain recovered text in the frame instead.
        let styled = match (&formats, story) {
            (Some(f), Some(id)) if f.has_story(*id) => f.add_frame(&mut d, SpreadRef::Doc(*pg), layer, *rect, *id, *look).ok(),
            _ => None,
        };
        match styled {
            Some((fid, notes)) => {
                frame_ids.push(fid.0);
                formatted_frames += 1;
                for n in notes {
                    if !format_notes.contains(&n) {
                        format_notes.push(n);
                    }
                }
            }
            None => {
                let (id, _) =
                    d.add_text_frame(SpreadRef::Doc(*pg), *rect, layer, text, ParaFormat::default()).map_err(|e| bad("file.open", e.to_string()))?;
                if let Some(rgb) = look.fill {
                    designcraft_pub::solid_fill(&mut d, id, rgb);
                }
                if let Some(weight) = look.border {
                    designcraft_pub::solid_border(&mut d, id, weight);
                }
                frame_ids.push(id.0);
            }
        }
    }
    for (pg, rect, text) in &placed {
        let (id, _) = d
            .add_text_frame(SpreadRef::Doc(positioned_pages + *pg), *rect, layer, text, ParaFormat::default())
            .map_err(|e| bad("file.open", e.to_string()))?;
        frame_ids.push(id.0);
    }
    let notice = match r.scope {
        Scope::EmptyKnownStore => NOTICE_EMPTY,
        Scope::Complete => NOTICE_OK,
        _ => NOTICE_PARTIAL,
    };
    let mut warnings = r.warnings.clone();
    recovery_notes(&mut warnings, (placed_pictures, placed_previews, formatted_frames), (&art_notes, &format_notes), formats.as_ref());
    warnings.push(if size_rejected {
        "Original page size is unverified. A standard A4 recovery page is being used.".to_string()
    } else if size_status == "default-A4" {
        "Original page size is unknown. A standard A4 recovery page is being used.".to_string()
    } else {
        "Page size is estimated from stored numbers. It has not been verified.".to_string()
    });
    if undecoded > 0 {
        warnings.push(format!("{undecoded} segment(s) could not be read and are marked in the document."));
    }
    let overset = placed.iter().any(|(_, rect, text)| frame_height(text, body_w, f64::MAX) > rect.height() + 0.5);
    if overset {
        warnings.push("Some recovered text extends beyond its recovery page. The full text is retained for editing.".to_string());
    }
    if trimmed_segments > 0 {
        warnings.push(format!("{trimmed_segments} segment(s) had stored control characters or trailing paragraph marks that are not shown; their provenance is in the report."));
    }
    warnings.push(DETAIL.to_string());
    let mut state = DocState::new(d, None);
    state.import_notes = warnings.clone();
    state.import_notice = Some(notice);
    state.import_save_path = str_param(p, "path").filter(|_| str_param(p, "base64").is_none()).map(|path| {
        let source = std::path::absolute(path).unwrap_or_else(|_| std::path::PathBuf::from(path));
        source.with_extension("designcraft").to_string_lossy().into_owned()
    });
    let i = s.add_document(state);
    let grown = grow_overset_frames(s, &frame_ids);
    let provenance: Vec<Value> = r
        .segments
        .iter()
        .map(|g| match &g.source {
            Source::Story { id, units, records } => {
                json!({"kind": "story", "id": id, "units": [units.start, units.end], "records": records, "decoded": g.text.is_some()})
            }
            Source::Record { id, map, bytes } => {
                json!({"kind": "record", "id": id, "map": map, "bytes": [bytes.start, bytes.end], "decoded": g.text.is_some()})
            }
        })
        .collect();
    Ok(json!({
        "index": i, "documentFonts": [], "warnings": warnings, "notes": [], "notice": notice,
        "legacy": {
            "route": "experimental-legacy",
            "family": r.family.label(),
            "scope": format!("{:?}", r.scope),
            "openResult": if r.scope == Scope::EmptyKnownStore { "empty-recovery-workspace" } else { "recovery-document" },
            "segmentCount": r.segments.len(), "grownFrames": grown, "decodedCount": decoded, "encodingUnknownCount": encoding_unknown, "undecodedCount": undecoded,
            "rawSizePair": r.size.map(|z| json!([z.raw_width, z.raw_height])),
            "sizeStatus": size_status,
            "recoveryPageSize": {"width": w, "height": h, "unit": "pt", "widthMm": w * 25.4 / 72.0, "heightMm": h * 25.4 / 72.0},
            "withheldCount": "unknown",
            "readerRevision": concat!("designcraft-pub-legacy ", env!("CARGO_PKG_VERSION")),
            "buildCommit": option_env!("NIFTY_BUILD_COMMIT").unwrap_or("unknown"),
            "provenance": provenance, "storageOrderOnly": true,
            "sourceText": source_text,
        }
    }))
}

/// A picture or an embedded object's preview drawing, placed on a candidate page.
struct Art {
    page: usize,
    rect: Rect,
    name: String,
    mime: &'static str,
    data: Vec<u8>,
    pixels: (u32, u32),
    preview: bool,
}

/// Embedded-object previews and bitmap pictures at their owning objects' stored rectangles, with the candidate
/// page each belongs on (new containers get their own page after the text's pages). Previews come first so
/// pictures and text lie above them.
fn collect_art(
    bytes: &[u8],
    r: &LegacyText,
    enabled: bool,
    (w, h): (f64, f64),
    containers: &mut Vec<u16>,
    ranks: &mut HashMap<u16, u32>,
) -> (Vec<Art>, Vec<String>) {
    let mut items = Vec::new();
    let mut notes: Vec<String> = Vec::new();
    let Some(size) = r.size.filter(|_| enabled) else { return (items, notes) };
    let mut page_of = |container: u16| {
        containers.iter().position(|c| *c == container).unwrap_or_else(|| {
            containers.push(container);
            containers.len() - 1
        })
    };
    for pv in designcraft_pub_legacy::read_object_previews(bytes).unwrap_or_default() {
        if let Some(f) = &pv.frame {
            ranks.entry(f.container).or_insert(f.container_rank);
        }
        let Some((container, rect)) = pv.frame.as_ref().and_then(|f| frame_to_rect(f, size, w, h)) else { continue };
        let emu = |v: u32| i32::try_from(v.saturating_mul(360)).unwrap_or(i32::MAX);
        let Some(svg) = designcraft_pub::metafile_to_svg(&pv.wmf, (emu(pv.width), emu(pv.height))) else { continue };
        for n in svg.warnings {
            if !notes.contains(&n) {
                notes.push(n);
            }
        }
        items.push(Art {
            page: page_of(container),
            rect,
            name: format!("object-{}.svg", pv.number),
            mime: "image/svg+xml",
            data: svg.data,
            pixels: svg.size,
            preview: true,
        });
    }
    for pic in designcraft_pub_legacy::read_pictures(bytes).unwrap_or_default() {
        if let Some(f) = &pic.frame {
            ranks.entry(f.container).or_insert(f.container_rank);
        }
        let Some((container, rect)) = pic.frame.as_ref().and_then(|f| frame_to_rect(f, size, w, h)) else { continue };
        items.push(Art {
            page: page_of(container),
            rect,
            name: format!("picture-{}.bmp", pic.record),
            mime: "image/bmp",
            data: pic.bmp,
            pixels: (pic.width, pic.height),
            preview: false,
        });
    }
    (items, notes)
}

/// Add one piece of art as a picture frame filling its rectangle.
fn add_art(d: &mut Document, layer: designcraft_doc::LayerId, art: &Art) -> Result<()> {
    let size = (f64::from(art.pixels.0.max(1)), f64::from(art.pixels.1.max(1)));
    // The document stores identical data once and returns the existing asset, so use the id it returns.
    let new_id = designcraft_doc::AssetId(d.alloc());
    let asset = d.add_asset(designcraft_doc::Asset {
        id: new_id,
        name: art.name.clone(),
        mime: art.mime.into(),
        data: std::sync::Arc::new(art.data.clone()),
        pixels: Some(art.pixels),
        ..Default::default()
    });
    let id = designcraft_doc::ItemId(d.alloc());
    let rect = art.rect;
    let mut item = designcraft_doc::Item::new(id, layer, designcraft_doc::Shape::Rectangle, designcraft_geom::shapes::rectangle(rect));
    let xf = designcraft_geom::Affine::translate((rect.x0, rect.y0))
        * designcraft_geom::Affine::scale_non_uniform(rect.width() / size.0, rect.height() / size.1);
    item.content = designcraft_doc::Content::Graphic(designcraft_doc::Graphic {
        layer_options: None,
        asset,
        size,
        xf,
        clip: None,
        auto_fit: Default::default(),
        fit_align: 4,
        crop: [0.0; 4],
        wrap: Default::default(),
        fill: designcraft_doc::Fill::none(),
    });
    d.insert_item(SpreadRef::Doc(art.page), item, None).map_err(|e| bad("file.open", e.to_string()))?;
    Ok(())
}

/// Add the collected art to the document; returns (previews placed, bitmaps placed).
fn place_art(d: &mut Document, layer: designcraft_doc::LayerId, items: &[Art]) -> (usize, usize) {
    let (mut previews, mut bitmaps) = (0usize, 0usize);
    for art in items {
        if add_art(d, layer, art).is_ok() {
            if art.preview {
                previews += 1;
            } else {
                bitmaps += 1;
            }
        }
    }
    (previews, bitmaps)
}

/// Notes about what the recovered pictures, previews and text formatting do and do not cover.
fn recovery_notes(
    warnings: &mut Vec<String>,
    (placed_pictures, placed_previews, formatted_frames): (usize, usize, usize),
    (art_notes, format_notes): (&[String], &[String]),
    formats: Option<&designcraft_pub::StoryFormats>,
) {
    if placed_pictures > 0 {
        warnings.push(format!(
            "{placed_pictures} bitmap picture(s) were read from the stored pictures and placed at their stored positions. Cropping, rotation, borders and colour changes are not applied; other pictures and shapes are not recovered."
        ));
    }
    if placed_previews > 0 {
        warnings.push(format!(
            "{placed_previews} embedded object preview(s) (stored drawings of clip art, WordArt or other objects) were placed at their stored positions as pictures; they are not editable."
        ));
        warnings.extend(art_notes.iter().cloned());
    }
    if formatted_frames > 0 {
        warnings.push(format!(
            "Fonts, sizes, styles and alignment were read from the stored text formatting for {formatted_frames} frame(s). Colours, pictures, shapes and other formatting are not recovered, and a font that is not installed is replaced."
        ));
        warnings.extend(format_notes.iter().cloned());
        if let Some(f) = formats {
            for n in f.warnings() {
                if !warnings.contains(n) {
                    warnings.push(n.clone());
                }
            }
        }
    }
}

type Positioned = (usize, Rect, String, Option<u32>, designcraft_pub::FrameLook);

/// Put the candidate pages in stored order and leave out master-like ones.
///
/// OBSERVATION (Isreal - Rome Vacation, Club 75): records directly below the root are stored in page order,
/// which put the stored page preview's page first; a container whose only text is a page-number stub
/// ("- # -") and that holds no picture is a master page, not a document page.
fn order_pages(containers: &mut Vec<u16>, ranks: &HashMap<u16, u32>, positioned: &mut Vec<Positioned>, art: &mut Vec<Art>) {
    let has_content: Vec<bool> = (0..containers.len())
        .map(|i| positioned.iter().any(|p| p.0 == i && p.2.chars().any(char::is_alphanumeric)) || art.iter().any(|a| a.page == i))
        .collect();
    let mut order: Vec<usize> = (0..containers.len()).filter(|i| has_content.get(*i).copied().unwrap_or(false)).collect();
    // Keep the first-seen order when a container has no stored rank.
    order.sort_by_key(|i| containers.get(*i).and_then(|c| ranks.get(c)).copied().unwrap_or(u32::MAX));
    let mut new_index: Vec<Option<usize>> = vec![None; containers.len()];
    for (pos, old) in order.iter().enumerate() {
        if let Some(slot) = new_index.get_mut(*old) {
            *slot = Some(pos);
        }
    }
    positioned.retain_mut(|p| match new_index.get(p.0).copied().flatten() {
        Some(n) => {
            p.0 = n;
            true
        }
        None => false,
    });
    art.retain_mut(|a| match new_index.get(a.page).copied().flatten() {
        Some(n) => {
            a.page = n;
            true
        }
        None => false,
    });
    *containers = order.iter().filter_map(|i| containers.get(*i).copied()).collect();
}

/// Make recovered frames just tall enough for their text. The fonts the file names are often not installed, and
/// their stand-ins are usually wider, so text that fitted in Publisher can overflow; growing the frame keeps all
/// of it visible and avoids false "overset" errors. A frame grows in small steps, at most by 360 pt in all.
/// Returns how many frames grew.
fn grow_overset_frames(s: &mut Session, ids: &[u64]) -> usize {
    const STEP: f64 = 6.0;
    const MAX_STEPS: usize = 60;
    let mut grown = 0usize;
    for id in ids {
        let mut grew = false;
        for _ in 0..MAX_STEPS {
            let fits = s.execute("story.get", &json!({"frame": id})).map(|r| r["overset"].is_null()).unwrap_or(true);
            if fits {
                break;
            }
            let Some(b) = s.doc().ok().and_then(|st| st.doc.item(designcraft_doc::ItemId(*id)).map(|it| it.bounds())) else { break };
            let from = [b.x0, b.y0, b.x1, b.y1];
            let to = [b.x0, b.y0, b.x1, b.y1 + STEP];
            if s.execute("transform.resize", &json!({"from": from, "to": to, "ids": [id]})).is_err() {
                break;
            }
            grew = true;
        }
        if grew {
            grown += 1;
        }
    }
    grown
}

#[cfg(test)]
mod frame_look_tests {
    use super::frame_look;
    use designcraft_pub_legacy::FrameRect;

    fn frame(fill_mode: u8, color_ref: u32) -> FrameRect {
        FrameRect { record: 1, left: 0, top: 0, right: 1, bottom: 1, container: 0, fill_mode, color_ref, border: 0, container_rank: 0 }
    }

    #[test]
    fn a_literal_reference_fills_with_red_in_the_lowest_byte() {
        // Bytes cc cc 99 20 (little endian) are the stored reference seen in the donated flyers.
        assert_eq!(frame_look(&frame(2, 0x2099_cccc)).fill, Some([0xcc, 0xcc, 0x99]));
        assert_eq!(frame_look(&frame(2, 0x20c3_2ca5)).fill, Some([0xa5, 0x2c, 0xc3]));
    }

    #[test]
    fn the_automatic_reference_keeps_its_black_fill_and_white_text() {
        let look = frame_look(&frame(2, 0xC0E4_0000));
        assert_eq!((look.fill, look.text), (Some([0, 0, 0]), Some([255, 255, 255])));
    }

    #[test]
    fn unread_modes_and_references_get_no_fill() {
        assert_eq!(frame_look(&frame(0, 0x2099_cccc)).fill, None, "mode 0: no fill flag");
        assert_eq!(frame_look(&frame(0x8e, 0x2099_0066)).fill, None, "other mode bits are not read");
        assert_eq!(frame_look(&frame(2, 0x0000_001a)).fill, None, "palette-like numbers are not decoded");
        assert_eq!(frame_look(&frame(2, 0x0300_e4c0)).fill, None, "scheme numbers are not decoded");
    }
}
