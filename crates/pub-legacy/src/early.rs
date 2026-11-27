// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! D text: legacy files whose Contents header has +0x04 = 200 and that have no directory. Only the control block and the
//! single-byte text buffer after it are read (docs/pub-format/legacy-minimal-reader.md, "Files with header +4 = 0x00c8").
//! The layout table is not read, so no frame, page, size or formatting is recovered.

use crate::codepage::{Decoded, decode};
use crate::contents::{CLSID_C, bad, need32, u16_at, u32_at};
use crate::{Container, Family, LegacyError, LegacyText, MAX_SEGMENTS, Scope, Segment, SizeCandidate, Source};

const CONTROL_LEN: usize = 46;
const HEADER_LEN: usize = 0x22;

/// Whether the header and root class are the ones this profile is defined by. Admission is only complete once
/// [`read`] has checked every relationship below.
pub(crate) fn admits(c: &Container) -> bool {
    let d = &c.contents;
    c.clsid == CLSID_C && d.get(..4) == Some(&[0xe8, 0xac, 0x22, 0x00]) && u16_at(d, 2) == Some(34) && u16_at(d, 4) == Some(200)
}

pub(crate) fn read(c: &Container) -> Result<LegacyText, LegacyError> {
    let label = Family::D.label();
    let data = &c.contents;
    let len = data.len();
    if need32(data, 8, "header length")? as usize != len {
        return Err(bad("the header length field disagrees with the stream length"));
    }
    let ctl = need32(data, 0x12, "control pointer")? as usize;
    if ctl < HEADER_LEN || ctl.checked_add(CONTROL_LEN).is_none_or(|e| e > len) {
        return Err(bad("the control block lies outside the stream"));
    }
    if u16_at(data, ctl) != Some(0xf0de) || u16_at(data, ctl + 2) != Some(CONTROL_LEN as u16) {
        return Err(bad("the control block marker or length is not the documented one"));
    }
    if need32(data, ctl + 0x0a, "control directory pointer")? as usize != len {
        return Err(LegacyError::Unsupported(
            "this profile is admitted only when the control block's directory pointer equals the stream length".into(),
        ));
    }
    let s = ctl + CONTROL_LEN;
    let p = need32(data, ctl + 0x12, "text end pointer")? as usize;
    if need32(data, ctl + 0x0e, "text start pointer")? as usize != s || s >= p || p.checked_add(2).is_none_or(|e| e > len) {
        return Err(bad("the text buffer bounds contradict the control block"));
    }
    if data.get(p - 1) != Some(&0x0c) || data.get(p..p + 2) != Some(&[0x0d, 0x0a]) {
        return Err(bad("the text buffer does not end in form feed and CR LF"));
    }
    let core = data.get(s..p).ok_or_else(|| bad("the text buffer runs past the stream"))?;
    let mut segments = Vec::new();
    let mut unknown = 0usize;
    let mut inferred = 0usize;
    let mut start = 0usize;
    for piece in core.split_inclusive(|b| *b == 0x0c) {
        if segments.len() >= MAX_SEGMENTS {
            return Err(LegacyError::Limit("more text segments than the reader limit".into()));
        }
        let (text, encoding_unknown, encoding_inferred) = match decode(piece) {
            Decoded::Ascii(t) => (Some(t), false, false),
            Decoded::Inferred(t) => {
                inferred += 1;
                (Some(t), false, true)
            }
            Decoded::Unknown => {
                unknown += 1;
                (None, true, false)
            }
        };
        segments.push(Segment { source: Source::Range { bytes: start..start + piece.len() }, text, encoding_unknown, encoding_inferred });
        start += piece.len();
    }
    let mut warnings = vec![format!("{label}: text only. Pages, frames, positions, fonts and formatting are not read in this profile.")];
    let size = match page_size(data) {
        Ok(size) => {
            warnings.push(format!(
                "{label}: the page size is a raw candidate (read in 1/914400 inch: {:.1} x {:.1} mm; every sample file gives a standard paper size, but it is not checked against Publisher's own page setup).",
                size.inferred_mm.0, size.inferred_mm.1
            ));
            Some(size)
        }
        Err(why) => {
            warnings.push(format!("{label}: the page size candidate was withheld: {why}."));
            None
        }
    };
    if unknown > 0 {
        warnings.push(format!("{label}: {unknown} segment(s) contain a byte with no meaning in the inferred code page; they were not decoded."));
    }
    if inferred > 0 {
        warnings.push(format!(
            "{label}: {inferred} segment(s) were decoded as Windows-1252. That is inferred from the stored quotes and apostrophes, not confirmed."
        ));
    }
    let scope = if unknown > 0 { Scope::Partial } else { Scope::Complete };
    Ok(LegacyText { family: Family::D, segments, size, scope, warnings, frames: Vec::new() })
}

/// The page size pair of the root record. The record directory is at header +0x16: a count, a maximum id and four
/// more bytes, then 10-byte entries (record offset u32, attribute u16, id u16, parent u16). The root is the one
/// record whose first word is 21 and whose prefix length (byte +3) covers the size fields at +0x14 and +0x18.
fn page_size(data: &[u8]) -> Result<SizeCandidate, String> {
    let dir = need32(data, 0x16, "directory pointer").map_err(|_| "the directory pointer runs past the stream".to_string())? as usize;
    let count = u16_at(data, dir).ok_or("the record directory runs past the stream")? as usize;
    let end = count.checked_mul(10).and_then(|n| n.checked_add(dir + 8)).filter(|e| *e <= data.len());
    if count == 0 || end.is_none() {
        return Err("the record directory is empty or runs past the stream".into());
    }
    let mut roots = Vec::new();
    let mut previous = 0usize;
    for k in 0..count {
        let at = dir + 8 + 10 * k;
        let r = need32(data, at, "directory entry").map_err(|_| "a directory entry runs past the stream".to_string())? as usize;
        if r <= previous || r >= data.len() {
            return Err("the record offsets are not ascending and inside the stream".into());
        }
        previous = r;
        let h = data.get(r + 3).copied().map_or(0, usize::from);
        if u16_at(data, r) == Some(21) && h >= 0x1c && r + 0x1c <= data.len() {
            roots.push(r);
        }
    }
    let [root] = roots[..] else { return Err("there is not exactly one root record".into()) };
    let (Some(w), Some(h)) = (u32_at(data, root + 0x14), u32_at(data, root + 0x18)) else { return Err("the size fields run past the stream".into()) };
    if w == 0 || h == 0 {
        return Err("a size field is zero".into());
    }
    Ok(SizeCandidate { raw_width: w, raw_height: h, inferred_mm: (f64::from(w) / 36000.0, f64::from(h) / 36000.0) })
}
