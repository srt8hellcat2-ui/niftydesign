// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! Story formatting from a Quill text store, for callers that already know which story belongs in which
//! frame (the Publisher 97/98/2000 text recovery). It reads the same `Quill/QuillSub/CONTENTS` structure
//! that `quill.rs` decodes and builds a formatted text frame the way the 2002+ importer does, but takes
//! no part in the 2002+ import itself. Colours are not applied: the old files' colour tables are not
//! decoded.
use crate::{PubError, Result, quill};
use designcraft_doc::{Document, ItemId, LayerId, ParaFormat, SpreadRef};
use designcraft_geom::Rect;
use serde_json::json;

/// Decoded formatting for every story in one Quill stream.
pub struct StoryFormats {
    q: quill::Quill,
}

impl StoryFormats {
    /// Decode a `Quill/QuillSub/CONTENTS` stream.
    pub fn parse(stream: &[u8]) -> Result<Self> {
        Ok(Self { q: quill::parse(stream)? })
    }

    /// Whether the stream holds a story with this id.
    pub fn has_story(&self, id: u32) -> bool {
        self.q.stories.contains_key(&id)
    }

    /// Notes about formatting that could not be applied.
    pub fn warnings(&self) -> &[String] {
        &self.q.warnings
    }

    /// Add a text frame holding story `tid` at `rect`, with its character and paragraph formatting.
    pub fn add_frame(&self, doc: &mut Document, r: SpreadRef, layer: LayerId, rect: Rect, tid: u32) -> Result<(ItemId, Vec<String>)> {
        let q = &self.q;
        let mut warnings: Vec<String> = Vec::new();
        let Some(qs) = q.stories.get(&tid) else { return Err(PubError::Malformed("missing story".into())) };
        // UTF-16 → UTF-8, keeping each unit's byte offset; Publisher ends every paragraph with CR.
        let mut text = String::new();
        let mut at = Vec::with_capacity(qs.units.len() + 1);
        let mut units = qs.units.clone();
        if units.last() == Some(&0x0d) {
            units.pop();
        }
        let mut skipped = false;
        for res in char::decode_utf16(units.iter().copied()) {
            let len16 = res.as_ref().map_or(1, |c| c.len_utf16());
            for _ in 0..len16 {
                at.push(text.len());
            }
            match res {
                Ok('\r') => text.push('\n'),
                Ok('\u{000b}') => text.push(designcraft_doc::FORCED_LINE_BREAK),
                Ok('\t') => text.push('\t'),
                Ok(c) if (c as u32) < 0x20 => skipped = true,
                Ok(c) => text.push(c),
                Err(_) => text.push(char::REPLACEMENT_CHARACTER),
            }
        }
        at.push(text.len());
        if skipped {
            warnings.push("Fields and special characters (page numbers, dates) were left out of the text.".into());
        }
        let (id, sid) = doc.add_text_frame(r, rect, layer, &text, ParaFormat::default()).map_err(|e| PubError::Malformed(e.to_string()))?;
        let byte = |u: usize| at.get(u.min(at.len().saturating_sub(1))).copied().unwrap_or(0);
        let defaults = q.default_char.clone();
        let mut char_ops = Vec::new();
        let runs: Vec<(std::ops::Range<usize>, quill::CharStyle)> =
            if qs.chars.is_empty() { vec![(0..units.len(), defaults.clone())] } else { qs.chars.clone() };
        for (range, style) in runs {
            let (a, z) = (byte(range.start), byte(range.end));
            if z <= a {
                continue;
            }
            let mut ops: Vec<(&'static str, serde_json::Value)> = Vec::new();
            let font = style.font.or(defaults.font).and_then(|f| q.fonts.get(f as usize)).cloned();
            if let Some(f) = font.filter(|f| !f.is_empty()) {
                ops.push(("fontFamily", json!(f)));
            }
            let (bold, italic) = (style.bold.or(defaults.bold).unwrap_or(false), style.italic.or(defaults.italic).unwrap_or(false));
            let face = match (bold, italic) {
                (true, true) => "Bold Italic",
                (true, false) => "Bold",
                (false, true) => "Italic",
                _ => "Regular",
            };
            ops.push(("fontStyle", json!(face)));
            if let Some(s) = style.size_pt.or(defaults.size_pt).filter(|s| (0.1..=1296.0).contains(s)) {
                ops.push(("size", json!(s)));
            }
            if style.underline.or(defaults.underline).unwrap_or(false) {
                ops.push(("underline", json!(true)));
            }
            if let Some(rgb) = style.color.or(defaults.color).and_then(literal_rgb) {
                let name = format!("R={} G={} B={}", rgb[0], rgb[1], rgb[2]);
                if !doc.swatches.iter().any(|s| s.name == name) {
                    let mut s = designcraft_color::Swatch::color(&name, designcraft_color::Color::rgb8(rgb[0], rgb[1], rgb[2]));
                    s.named = false;
                    doc.swatches.push(s);
                }
                ops.push(("fill", json!(name)));
            }
            if let Some(value) = style.capitalization.or(defaults.capitalization) {
                ops.push(("capitalization", json!(value)));
            }
            if let Some(value) = style.position.or(defaults.position) {
                ops.push(("position", json!(value)));
            }
            if let Some(value) = style.tracking.or(defaults.tracking).filter(|v| v.abs() <= 10_000.0) {
                ops.push(("tracking", json!(value)));
            }
            char_ops.push((a..z, ops));
        }
        let defaults_p = q.default_para.clone();
        let mut para_ops = Vec::new();
        let para_runs = if qs.paras.is_empty() { vec![(0..units.len(), defaults_p.clone())] } else { qs.paras.clone() };
        for (range, style) in &para_runs {
            let (a, z) = (byte(range.start), byte(range.end.saturating_sub(1)).max(byte(range.start)));
            let align = match style.align.or(defaults_p.align) {
                Some(1) => "right",
                Some(2) => "center",
                Some(6) => "leftJustified",
                _ => "left",
            };
            let mut ops: Vec<(&'static str, serde_json::Value)> = vec![("align", json!(align))];
            for (k, v) in [
                ("spaceBefore", style.space_before_pt.or(defaults_p.space_before_pt)),
                ("spaceAfter", style.space_after_pt.or(defaults_p.space_after_pt)),
                ("firstLineIndent", style.first_indent_pt.or(defaults_p.first_indent_pt)),
                ("leftIndent", style.left_indent_pt.or(defaults_p.left_indent_pt)),
                ("rightIndent", style.right_indent_pt.or(defaults_p.right_indent_pt)),
            ] {
                if let Some(v) = v.filter(|v| v.is_finite() && v.abs() < 10_000.0) {
                    ops.push((k, json!(v)));
                }
            }
            if let Some(bullet) = style.bullet.as_ref().or(defaults_p.bullet.as_ref()) {
                ops.push(("listType", json!("bullets")));
                if let Some(character) = bullet.character.or_else(|| defaults_p.bullet.as_ref().and_then(|b| b.character)) {
                    ops.push(("bulletChar", json!(character.to_string())));
                }
                if let Some(font) = bullet.font.or_else(|| defaults_p.bullet.as_ref().and_then(|b| b.font)).and_then(|f| q.fonts.get(f as usize)) {
                    ops.push(("bulletFont", json!(font)));
                }
            }
            let spacing = style.line_spacing.or(defaults_p.line_spacing);
            if let Some(quill::LineSpacing::Multiple(factor)) = spacing {
                ops.push(("autoLeading", json!(factor)));
            }
            para_ops.push((a..z, ops, spacing));
        }
        let mut failed = false;
        if let Some(st) = doc.story_mut(sid) {
            for (range, ops) in char_ops {
                st.format_chars(range, |cf| {
                    for (k, v) in &ops {
                        failed |= cf.over.set_json(k, v).is_err();
                    }
                });
            }
            for (range, ops, spacing) in para_ops {
                st.format_paras(range, |pf| {
                    if let Some(spacing) = spacing {
                        pf.chars.leading = Some(match spacing {
                            quill::LineSpacing::Exact(points) => designcraft_doc::Leading::Points(points),
                            quill::LineSpacing::Multiple(_) => designcraft_doc::Leading::Auto,
                        });
                    }
                    for (k, v) in &ops {
                        failed |= pf.para.set_json(k, v).is_err();
                    }
                });
            }
        }
        if failed {
            warnings.push("Some text formatting could not be applied.".into());
        }
        Ok((id, warnings))
    }
}

/// OBSERVATION (docs/pub-format/legacy-minimal-reader.md, "Text colour"): in the 97/98/2000 Quill stores a
/// character colour whose top byte is 0x90 carries a literal colour in its low three bytes as
/// 0x00BBGGRR (red in the lowest byte). Other tags (small palette numbers, 0x80 scheme/system values,
/// 0xe0e4 markers) are not decoded and leave the colour unset.
fn literal_rgb(v: u32) -> Option<[u8; 3]> {
    (v >> 24 == 0x90).then(|| [(v & 0xff) as u8, ((v >> 8) & 0xff) as u8, ((v >> 16) & 0xff) as u8])
}

#[cfg(test)]
mod tests {
    use super::literal_rgb;

    #[test]
    fn a_0x90_colour_is_blue_green_red_in_the_low_three_bytes() {
        assert_eq!(literal_rgb(0x9099_6600), Some([0x00, 0x66, 0x99]));
        assert_eq!(literal_rgb(0x9033_6633), Some([0x33, 0x66, 0x33]));
    }

    #[test]
    fn other_tags_are_left_unset() {
        for v in [3, 0x8000_000c, 0xe0e4_0001, 0xffff_fffe, 0x0099_6600] {
            assert_eq!(literal_rgb(v), None);
        }
    }
}
