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

/// A text-only recovery does not place text, so a stored page whose shorter side is below this (4 inches) would only
/// spread the text over many tiny pages. It is flowed on A4 instead.
const TEXT_ONLY_MIN_PT: f64 = 288.0;

/// The recovery page size, its status, and whether a stored candidate was rejected as implausible.
fn page_size(r: &LegacyText) -> ((f64, f64), &'static str, bool) {
    let mut rejected = false;
    if let Some(size) = r.size {
        let (w, h) = (size.inferred_mm.0 * 72.0 / 25.4, size.inferred_mm.1 * 72.0 / 25.4);
        let max_pt = MAX_PLAUSIBLE_MM * 72.0 / 25.4;
        if (72.0..=max_pt).contains(&w) && (72.0..=max_pt).contains(&h) {
            if r.family == designcraft_pub_legacy::Family::D && w.min(h) < TEXT_ONLY_MIN_PT {
                return (A4, "text-only-A4", false);
            }
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
fn stored_rect(
    r: &LegacyText,
    seg: &designcraft_pub_legacy::Segment,
    w: f64,
    h: f64,
    scheme: Option<&designcraft_pub_legacy::Scheme>,
) -> Option<(u16, Rect, designcraft_pub::FrameLook)> {
    let Source::Story { records, .. } = &seg.source else { return None };
    let f = r.frames.iter().find(|f| records.contains(&f.record))?;
    let (container, rect) = frame_to_rect(f, r.size?, w, h)?;
    Some((container, rect, frame_look_in_scheme(f, scheme)))
}

/// OBSERVATION (docs/pub-format/legacy-minimal-reader.md, "The colour scheme record"; three files agree): a fill
/// reference 0xC0E4_000N with N from 1 to 5 is the Nth entry of the document's colour scheme. In `Wireless Flyer`
/// the panels with N = 3 are pale on the stored preview where the scheme's third entry is `d6ebff`; in the Dark Blue
/// files they are silver where it is `cccce6`. N = 0 is the automatic (black) reference, N = 6 and 7 are not read.
fn scheme_fill(f: &designcraft_pub_legacy::FrameRect, scheme: Option<&designcraft_pub_legacy::Scheme>) -> Option<[u8; 3]> {
    let n = usize::try_from(f.color_ref & 0xffff).ok()?;
    if f.fill_mode != 2 || f.color_ref >> 16 != 0xC0E4 || !(1..=5).contains(&n) {
        return None;
    }
    scheme?.entries.get(n)?.rgb()
}

/// [`frame_look`], plus the fill a colour-scheme reference gives when the scheme says which colour it is.
fn frame_look_in_scheme(f: &designcraft_pub_legacy::FrameRect, scheme: Option<&designcraft_pub_legacy::Scheme>) -> designcraft_pub::FrameLook {
    let mut look = frame_look(f);
    if look.fill.is_none() {
        look.fill = scheme_fill(f, scheme);
    }
    look
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
    if f.fill_mode == 2 && f.color_ref == 0xC0E4_0000 && f.frame_flags & 0x40 == 0 {
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
    let scheme = designcraft_pub_legacy::read_scheme(bytes).ok().flatten();
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
        if page_rects && let Some((container, rect, look)) = stored_rect(&r, seg, w, h, scheme.as_ref()) {
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
    let (mut art, art_notes, mut decor) = collect_art(bytes, &r, page_rects, (w, h), &mut containers, &mut ranks, scheme.as_ref());
    order_pages(&mut containers, &ranks, &mut positioned, &mut art, &mut decor);
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
    // Backgrounds and panels first, so the pictures and the text lie above them.
    let placed_decor = decor.iter().filter(|dc| add_decor(&mut d, layer, dc)).count();
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
    if placed_decor > 0 {
        warnings.push(format!(
            "{placed_decor} frame(s) without text were recovered as empty panels with the fill (black or an exact colour) and border read from the file. Fills taken from colour schemes, shapes and lines are not recovered."
        ));
    }
    warnings.push(if size_rejected {
        "Original page size is unverified. A standard A4 recovery page is being used.".to_string()
    } else if size_status == "text-only-A4" {
        "The stored page is small and this file is recovered as text only, so the text is flowed on a standard A4 recovery page.".to_string()
    } else if size_status == "default-A4" {
        "Original page size is unknown. A standard A4 recovery page is being used.".to_string()
    } else {
        "Page size is read from the file. In every sample file it is an exact standard paper size, but it has not been compared with Publisher's own page setup.".to_string()
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
    let grown = grow_overset_frames(s, &frame_ids, h);
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
            Source::Range { bytes } => json!({"kind": "range", "bytes": [bytes.start, bytes.end], "decoded": g.text.is_some()}),
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
            "readerRevision": concat!("legacy reader ", env!("CARGO_PKG_VERSION")),
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

/// A frame without text that has a fill or a border we can draw (a panel, band or background).
struct Decor {
    page: usize,
    rect: Rect,
    look: designcraft_pub::FrameLook,
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
    scheme: Option<&designcraft_pub_legacy::Scheme>,
) -> (Vec<Art>, Vec<String>, Vec<Decor>) {
    let mut items = Vec::new();
    let mut notes: Vec<String> = Vec::new();
    let mut decor: Vec<Decor> = Vec::new();
    let Some(size) = r.size.filter(|_| enabled) else { return (items, notes, decor) };
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
    // Frames without text that have a fill or border we can draw: panels, bands and backgrounds.
    let linked: Vec<u16> = r.frames.iter().map(|f| f.record).collect();
    for f in designcraft_pub_legacy::read_decor_frames(bytes, &linked).unwrap_or_default() {
        let look = frame_look_in_scheme(&f, scheme);
        if look.fill.is_none() && look.border.is_none() {
            continue;
        }
        ranks.entry(f.container).or_insert(f.container_rank);
        let Some((container, rect)) = frame_to_rect(&f, size, w, h) else { continue };
        decor.push(Decor { page: page_of(container), rect, look });
    }
    (items, notes, decor)
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
        data: designcraft_doc::AssetData::embedded(art.data.clone()),
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
/// Add an empty frame with the fill and border of a panel, band or background.
fn add_decor(d: &mut Document, layer: designcraft_doc::LayerId, dc: &Decor) -> bool {
    let Ok((id, _)) = d.add_text_frame(SpreadRef::Doc(dc.page), dc.rect, layer, "", ParaFormat::default()) else { return false };
    if let Some(rgb) = dc.look.fill {
        designcraft_pub::solid_fill(d, id, rgb);
    }
    if let Some(weight) = dc.look.border {
        designcraft_pub::solid_border(d, id, weight);
    }
    true
}

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
fn order_pages(containers: &mut Vec<u16>, ranks: &HashMap<u16, u32>, positioned: &mut Vec<Positioned>, art: &mut Vec<Art>, decor: &mut Vec<Decor>) {
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
    // A page that holds only panels and no text or picture is not a page of its own (masters hold such frames).
    decor.retain_mut(|dc| match new_index.get(dc.page).copied().flatten() {
        Some(n) => {
            dc.page = n;
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
fn grow_overset_frames(s: &mut Session, ids: &[u64], page_h: f64) -> usize {
    const STEP: f64 = 6.0;
    const MAX_STEPS: usize = 60;
    const MAX_SHRINKS: usize = 5;
    let mut grown = 0usize;
    for id in ids {
        let mut grew = false;
        let mut limit: Option<(f64, f64)> = None;
        for _ in 0..MAX_STEPS {
            let fits = s.execute("story.get", &json!({"frame": id})).map(|r| r["overset"].is_null()).unwrap_or(true);
            if fits {
                break;
            }
            let Some(b) = s.doc().ok().and_then(|st| st.doc.item(designcraft_doc::ItemId(*id)).map(|it| it.bounds())) else { break };
            let (ceiling, floor) = *limit.get_or_insert_with(|| room_around(s, *id, b, page_h));
            let down = STEP.min(floor - b.y1);
            let up = STEP.min(b.y0 - ceiling);
            // Grow downward first; when the neighbour or the page edge is in the way, grow upward into free space.
            let (top, bottom) = if down >= 0.5 {
                (b.y0, b.y1 + down)
            } else if up >= 0.5 {
                (b.y0 - up, b.y1)
            } else {
                break;
            };
            let from = [b.x0, b.y0, b.x1, b.y1];
            let to = [b.x0, top, b.x1, bottom];
            if s.execute("transform.resize", &json!({"from": from, "to": to, "ids": [id]})).is_err() {
                break;
            }
            grew = true;
        }
        // Blocked by a neighbour or the page edge: shrink the type a little rather than overlap or run off the page.
        for _ in 0..MAX_SHRINKS {
            if s.execute("story.get", &json!({"frame": id})).map(|r| r["overset"].is_null()).unwrap_or(true) {
                break;
            }
            if !shrink_text(s, *id) {
                break;
            }
            grew = true;
        }
        // Still too tall: blank paragraphs at the very end hold no text; drop them so they cannot cause an error.
        if s.execute("story.get", &json!({"frame": id})).map(|r| !r["overset"].is_null()).unwrap_or(false) && trim_trailing_blank_paragraphs(s, *id) {
            grew = true;
        }
        if grew {
            grown += 1;
        }
    }
    grown
}

/// Remove empty paragraphs at the end of a frame's story (keeping the last paragraph's text). Returns whether any went.
fn trim_trailing_blank_paragraphs(s: &mut Session, id: u64) -> bool {
    s.edit(|d, _| {
        let Some(sid) = d.item(designcraft_doc::ItemId(id)).and_then(|it| match &it.content {
            designcraft_doc::Content::Text(t) => Some(t.story),
            _ => None,
        }) else {
            return Ok(false);
        };
        let Some(st) = d.story_mut(sid) else { return Ok(false) };
        let len = st.text.trim_end_matches('\n').len();
        if len == st.text.len() {
            return Ok(false);
        }
        let total = st.text.len();
        st.delete(len..total);
        Ok(true)
    })
    .unwrap_or(false)
}

/// Scale every explicit type size in a frame's story down by 6 % (never below 5 pt). Returns whether anything changed.
fn shrink_text(s: &mut Session, id: u64) -> bool {
    s.edit(|d, _| {
        let Some(sid) = d.item(designcraft_doc::ItemId(id)).and_then(|it| match &it.content {
            designcraft_doc::Content::Text(t) => Some(t.story),
            _ => None,
        }) else {
            return Ok(false);
        };
        let styles = d.styles.clone();
        let Some(st) = d.story_mut(sid) else { return Ok(false) };
        // The size each run really has: its own, else the one its styles give. A run that takes its size from a
        // style (the usual case for a heading) gets an explicit, smaller size.
        let base = st.paras.first().map(|p| styles.resolve_para(p).1);
        let runs: Vec<(std::ops::Range<usize>, f64)> = st
            .runs()
            .map(|(r, cf)| {
                let size = match (cf.over.size, base.as_ref()) {
                    (Some(z), _) => z,
                    (None, Some(base)) => styles.resolve_char(base, cf).size,
                    (None, None) => 12.0,
                };
                (r, size)
            })
            .filter(|(_, z)| *z > 5.0)
            .collect();
        let changed = !runs.is_empty();
        for (r, z) in runs {
            st.format_chars(r, |cf| cf.over.size = Some((z * 0.94).max(5.0)));
        }
        // Lines set to an exact height must shrink with the type, or the text stays too tall.
        for r in st.para_ranges() {
            st.format_paras(r, |pf| {
                if let Some(designcraft_doc::Leading::Points(p)) = pf.chars.leading {
                    pf.chars.leading = Some(designcraft_doc::Leading::Points((p * 0.94).max(5.0)));
                }
            });
        }
        Ok(changed)
    })
    .unwrap_or(false)
}

/// How far a recovered frame may grow, as (ceiling, floor): down to just above the next object below it that
/// shares part of its width and never past the page bottom, and up to just below the next object above it and
/// never past the page top. Objects that already overlap the frame (backgrounds, pictures behind text) are not
/// obstacles. The ceiling is never below the frame's top edge, nor the floor above its bottom edge.
fn room_around(s: &Session, id: u64, b: designcraft_geom::Rect, page_h: f64) -> (f64, f64) {
    const GAP: f64 = 2.0;
    let mut ceiling = 0.0_f64.min(b.y0);
    let mut floor = page_h.max(b.y1);
    let Ok(st) = s.doc() else { return (ceiling, floor) };
    for other in st.doc.all_items() {
        if other.0 == id {
            continue;
        }
        let Some(o) = st.doc.item(other).map(|it| it.bounds()) else { continue };
        if !(o.x0 < b.x1 - 1.0 && o.x1 > b.x0 + 1.0) {
            continue;
        }
        // An object that contains the whole frame is a background, not a neighbour.
        if o.x0 <= b.x0 + 1.0 && o.x1 >= b.x1 - 1.0 && o.y0 <= b.y0 + 0.5 && o.y1 >= b.y1 - 0.5 {
            continue;
        }
        // A neighbour below: it starts under the frame's top and reaches its bottom edge or beyond. One that only
        // overlaps the frame's lower part leaves no room (the floor stays at the frame's own bottom).
        if o.y0 > b.y0 + 0.5 && o.y1 > b.y1 - 0.5 && o.y0 - GAP < floor {
            floor = (o.y0 - GAP).max(b.y1);
        }
        // A neighbour above, likewise: it starts at or above the frame's top and ends above its bottom.
        if o.y0 < b.y0 + 0.5 && o.y1 < b.y1 - 0.5 && o.y1 + GAP > ceiling {
            ceiling = (o.y1 + GAP).min(b.y0);
        }
    }
    (ceiling, floor)
}

#[cfg(test)]
mod frame_look_tests {
    use super::{frame_look, frame_look_in_scheme};
    use designcraft_pub_legacy::{FrameRect, Scheme, SchemeColor};

    fn scheme() -> Scheme {
        // Island's entries: black, blue, lime, pale blue, silver, a grey number, hyperlink, a grey number.
        let lit = |r, g, b| SchemeColor::Literal([r, g, b]);
        Scheme {
            name: "Island".into(),
            entries: [
                SchemeColor::Index(0),
                lit(0x33, 0x99, 0xff),
                lit(0xcc, 0xff, 0x00),
                lit(0xd6, 0xeb, 0xff),
                lit(0xcc, 0xcc, 0xcc),
                SchemeColor::Index(14),
                lit(0x66, 0, 0xcc),
                SchemeColor::Index(1),
            ],
        }
    }

    #[test]
    fn a_scheme_reference_takes_the_fill_of_its_entry_in_the_files_own_scheme() {
        let s = scheme();
        assert_eq!(frame_look_in_scheme(&frame(2, 0xC0E4_0003), Some(&s)).fill, Some([0xd6, 0xeb, 0xff]));
        assert_eq!(frame_look_in_scheme(&frame(2, 0xC0E4_0001), Some(&s)).fill, Some([0x33, 0x99, 0xff]));
    }

    #[test]
    fn a_scheme_reference_without_a_known_colour_gets_no_fill() {
        let s = scheme();
        // No scheme, a grey number (not read), the entries 6 and 7 (not read), a tinted reference, and mode 0.
        assert_eq!(frame_look_in_scheme(&frame(2, 0xC0E4_0003), None).fill, None);
        assert_eq!(frame_look_in_scheme(&frame(2, 0xC0E4_0005), Some(&s)).fill, None);
        assert_eq!(frame_look_in_scheme(&frame(2, 0xC0E4_0006), Some(&s)).fill, None);
        assert_eq!(frame_look_in_scheme(&frame(2, 0xC0E4_0007), Some(&s)).fill, None);
        assert_eq!(frame_look_in_scheme(&frame(2, 0x819E_0003), Some(&s)).fill, None);
        assert_eq!(frame_look_in_scheme(&frame(0, 0xC0E4_0003), Some(&s)).fill, None);
    }

    fn frame(fill_mode: u8, color_ref: u32) -> FrameRect {
        FrameRect {
            record: 1,
            left: 0,
            top: 0,
            right: 1,
            bottom: 1,
            container: 0,
            fill_mode,
            color_ref,
            frame_flags: 3,
            border: 0,
            container_rank: 0,
        }
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
    fn the_black_fill_needs_bit_0x40_of_byte_0x17_clear() {
        let mut f = frame(2, 0xC0E4_0000);
        assert_eq!(frame_look(&f).fill, Some([0, 0, 0]));
        f.frame_flags = 0x43;
        assert_eq!(frame_look(&f).fill, None, "a pale divider in Flyer 200404 carries 0x43");
        f.frame_flags = 0x83;
        assert_eq!(frame_look(&f).fill, Some([0, 0, 0]), "0x83 was black on three of four frames");
    }

    #[test]
    fn unread_modes_and_references_get_no_fill() {
        assert_eq!(frame_look(&frame(0, 0x2099_cccc)).fill, None, "mode 0: no fill flag");
        assert_eq!(frame_look(&frame(0x8e, 0x2099_0066)).fill, None, "other mode bits are not read");
        assert_eq!(frame_look(&frame(2, 0x0000_001a)).fill, None, "palette-like numbers are not decoded");
        assert_eq!(frame_look(&frame(2, 0x0300_e4c0)).fill, None, "scheme numbers are not decoded");
    }
}
