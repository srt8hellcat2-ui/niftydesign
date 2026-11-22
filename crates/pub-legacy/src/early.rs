// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! D text: legacy files whose Contents header has +0x04 = 200 and that have no directory. Only the control block and the
//! single-byte text buffer after it are read (docs/pub-format/legacy-minimal-reader.md, "Files with header +4 = 0x00c8").
//! The layout table is not read, so no frame, page, size or formatting is recovered.

use crate::contents::{CLSID_C, bad, need32, u16_at};
use crate::{Container, Family, LegacyError, LegacyText, MAX_SEGMENTS, Scope, Segment, Source};

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
    let mut start = 0usize;
    for piece in core.split_inclusive(|b| *b == 0x0c) {
        if segments.len() >= MAX_SEGMENTS {
            return Err(LegacyError::Limit("more text segments than the reader limit".into()));
        }
        let (text, encoding_unknown) = if piece.is_ascii() {
            (Some(piece.iter().map(|x| char::from(*x)).collect::<String>()), false)
        } else {
            unknown += 1;
            (None, true)
        };
        segments.push(Segment { source: Source::Range { bytes: start..start + piece.len() }, text, encoding_unknown });
        start += piece.len();
    }
    let mut warnings = vec![
        format!("{label}: text only. Pages, frames, positions, sizes, fonts and formatting are not read in this profile."),
        format!("{label}: the page size is not recovered; the stored directory is absent and the layout table was not read."),
    ];
    if unknown > 0 {
        warnings.push(format!(
            "{label}: {unknown} segment(s) contain bytes with the high bit set, and the code page is not established; they were not decoded."
        ));
    }
    let scope = if unknown > 0 { Scope::Partial } else { Scope::Complete };
    Ok(LegacyText { family: Family::D, segments, size: None, scope, warnings, frames: Vec::new() })
}
