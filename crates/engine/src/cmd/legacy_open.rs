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

use super::{bad, str_param};
use crate::{DocState, EngineError, Result, Session};

const DETAIL: &str = "Original frames, fonts, formatting, page order, tables and pictures are not recovered. The order of recovered text may not be the reading order. Keep the original file. If possible, open it in Publisher 2002 or later and save a new copy.";
const NOTICE_OK: &str = concat!(
    "Experimental text recovery. Recovered text is ready to edit. This is a new document, not the original layout. ",
    "Original frames, fonts, formatting, page order, tables and pictures are not recovered. The order of recovered text may not be the reading order. Keep the original file. If possible, open it in Publisher 2002 or later and save a new copy."
);
const NOTICE_PARTIAL: &str = concat!(
    "Experimental text recovery. Some text could not be recovered. Missing text is reported below; text we could not read is marked in the document. This is a new document, not the original layout. ",
    "Original frames, fonts, formatting, page order, tables and pictures are not recovered. The order of recovered text may not be the reading order. Keep the original file. If possible, open it in Publisher 2002 or later and save a new copy."
);
const NOTICE_EMPTY: &str = concat!(
    "Experimental text recovery found no text in the supported text store. This does not mean the original file is blank. The empty page is a recovery workspace, not an original page. ",
    "Original frames, fonts, formatting, page order, tables and pictures are not recovered. The order of recovered text may not be the reading order. Keep the original file. If possible, open it in Publisher 2002 or later and save a new copy."
);
const PLACEHOLDER: &str = "[Text could not be read. No replacement text was guessed.]";
const A4: (f64, f64) = (595.276, 841.89);
const MARGIN: f64 = 36.0;
const MAX_PAGES: usize = 2000;

fn page_size(r: &LegacyText) -> ((f64, f64), &'static str) {
    if let Some(size) = r.size {
        let (w, h) = (size.inferred_mm.0 * 72.0 / 25.4, size.inferred_mm.1 * 72.0 / 25.4);
        if (72.0..=15_552.0).contains(&w) && (72.0..=15_552.0).contains(&h) {
            return ((w, h), "inferred-from-raw");
        }
    }
    (A4, "default-A4")
}

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
        let why = r.warnings.iter().find(|w| w.contains("refused")).cloned().unwrap_or_else(|| "no text was recovered".into());
        return Err(refuse(why));
    }
    let ((w, h), size_status) = page_size(&r);
    // Lay the frames out first, then build the document with exactly that many pages.
    let body_w = (w - 2.0 * MARGIN).max(36.0);
    let body_h = (h - 2.0 * MARGIN).max(36.0);
    let mut placed: Vec<(usize, Rect, String)> = Vec::new();
    let (mut page, mut y) = (0usize, MARGIN);
    for seg in &r.segments {
        let text = match &seg.text {
            Some(t) => visible(t),
            None => PLACEHOLDER.to_string(),
        };
        let height = frame_height(&text, body_w, body_h);
        if y > MARGIN && y + height > h - MARGIN {
            page += 1;
            y = MARGIN;
        }
        if page >= MAX_PAGES {
            return Err(refuse("more recovered text than the page limit".into()));
        }
        placed.push((page, Rect::new(MARGIN, y, MARGIN + body_w, y + height), text));
        y += height + 8.0;
    }
    let nd = NewDocument {
        title: title.clone(),
        width: w,
        height: h,
        pages: page + 1,
        facing_pages: false,
        primary_text_frame: false,
        ..Default::default()
    };
    let mut d = Document::new(&nd);
    let layer = d.layers.first().map(|l| l.id).ok_or_else(|| bad("file.open", "new document has no layer"))?;
    for (pg, rect, text) in &placed {
        d.add_text_frame(SpreadRef::Doc(*pg), *rect, layer, text, ParaFormat::default()).map_err(|e| bad("file.open", e.to_string()))?;
    }
    let notice = match r.scope {
        Scope::EmptyKnownStore => NOTICE_EMPTY,
        Scope::Complete => NOTICE_OK,
        _ => NOTICE_PARTIAL,
    };
    let mut warnings = r.warnings.clone();
    warnings.push(if size_status == "default-A4" {
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
    warnings.push(DETAIL.to_string());
    let mut state = DocState::new(d, None);
    state.import_notes = warnings.clone();
    state.import_notice = Some(notice);
    state.import_save_path = str_param(p, "path").filter(|_| str_param(p, "base64").is_none()).map(|path| {
        let source = std::path::absolute(path).unwrap_or_else(|_| std::path::PathBuf::from(path));
        source.with_extension("designcraft").to_string_lossy().into_owned()
    });
    let i = s.add_document(state);
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
            "segmentCount": r.segments.len(), "decodedCount": decoded, "encodingUnknownCount": encoding_unknown, "undecodedCount": undecoded,
            "rawSizePair": r.size.map(|z| json!([z.raw_width, z.raw_height])),
            "sizeStatus": size_status,
            "recoveryPageSize": {"width": w, "height": h, "unit": "pt"},
            "provenance": provenance, "storageOrderOnly": true,
        }
    }))
}
