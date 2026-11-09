// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! C text: the single-byte text buffer inside Contents and its index-26 map (gates C01 to C06).

use crate::contents::{Layout, u16_at, u32_at};
use crate::{MAX_SEGMENTS, Recovered, Scope, Segment, Source};
use std::collections::HashSet;

pub(crate) fn read_map(layout: &Layout<'_>) -> Result<Recovered, String> {
    let label = layout.family.label();
    let data = layout.data;
    // C01 buffer.
    let s = layout.c + 46;
    let p = u32_at(data, layout.c + 0x12).ok_or("the text end pointer runs past the stream")? as usize;
    if u32_at(data, layout.c + 0x0e).map(|v| v as usize) != Some(s) || s >= p || p.checked_add(2).is_none_or(|e| e > layout.d) {
        return Err("the text buffer bounds contradict the control block (C01)".into());
    }
    let m = p - s;
    if data.get(p - 1) != Some(&0x0c) || data.get(p..p + 2) != Some(&[0x0d, 0x0a]) {
        return Err("the text buffer does not end in form feed and CR LF (C01)".into());
    }
    let boundary = (p + 2).div_ceil(512) * 512;
    if boundary > layout.d || data.get(p + 2..boundary).is_none_or(|g| g.iter().any(|b| *b != 0)) {
        return Err("the padding after the text buffer is not zero up to the next 512-byte boundary (C01)".into());
    }
    // C02 source map: exactly one index 26, class 10, H = 4.
    let mut maps = layout.records.iter().filter(|r| r.index == 26);
    let (Some(map), None) = (maps.next(), maps.next()) else { return Err("there is not exactly one index-26 map record (C02)".into()) };
    if layout.class_of(map) != Some(10) || map.h != 4 {
        return Err("the map record is not class 10 with H=4 (C02)".into());
    }
    // C03 arrays.
    let (n, w) = layout.check_extent(map).map_err(|why| format!("the map record failed its extent check: {why} (C03)"))?;
    if n == 0 || n % 2 != 0 || w != 10 {
        return Err("the map count is not positive and even, or its element width is not 10 (C03)".into());
    }
    if n / 2 > MAX_SEGMENTS {
        return Err("more segments than the reader limit".into());
    }
    let t = map.r + map.h;
    let a = t + 18;
    let b = t + 22 + 4 * n;
    if u32_at(data, a + 4 * n).map(|v| v as usize) != Some(m) {
        return Err("the extra boundary does not equal the text buffer size (C03)".into());
    }
    // C04 ranges and C05 links.
    let mut segments = Vec::with_capacity(n / 2);
    let mut warnings = Vec::new();
    let mut ids = HashSet::new();
    let mut expect_start = 0usize;
    let mut unknown = 0usize;
    for j in 0..n / 2 {
        let start = u32_at(data, a + 8 * j).ok_or("a map slot runs past the stream (C04)")? as usize;
        let end = u32_at(data, a + 8 * j + 4).ok_or("a map slot runs past the stream (C04)")? as usize;
        if start != expect_start || end <= start || end > m || end - start < 3 {
            return Err("the map ranges are not positive, adjacent and ascending over the buffer (C04)".into());
        }
        let bytes = data.get(s + start..s + end).ok_or("a map range runs past the stream (C04)")?;
        let ff = bytes.iter().filter(|x| **x == 0x0c).count();
        if ff != 1 || bytes.last() != Some(&0x0c) || bytes.get(bytes.len() - 3..bytes.len() - 1) != Some(&[0x0d, 0x0a]) {
            return Err("a map range does not end in CR LF then form feed with exactly one form feed (C04)".into());
        }
        expect_start = end;
        let el = b + 20 * j;
        let id = u16_at(data, el).ok_or("a map element runs past the stream (C05)")?;
        let kind = u16_at(data, el + 2).ok_or("a map element runs past the stream (C05)")?;
        if u16_at(data, el + 10) != Some(0) {
            return Err("an odd map element has a non-zero id field (C05)".into());
        }
        let target = layout.rec(id).ok_or("a map element names a record that is not in the directory (C05)")?;
        let want_index = match kind {
            1 => 0,
            0 => 1,
            _ => return Err("a map element has an unknown kind (C05)".into()),
        };
        if target.index != want_index || !ids.insert(id) {
            return Err("a map element's target index or uniqueness is wrong (C05)".into());
        }
        layout.check_extent(target).map_err(|why| format!("map target {id:#x} failed its extent check: {why} (C05)"))?;
        // C06 decode: exact for ASCII-only, otherwise encoding-unknown.
        let (text, encoding_unknown) = if bytes.is_ascii() {
            (Some(bytes.iter().map(|x| char::from(*x)).collect::<String>()), false)
        } else {
            unknown += 1;
            (None, true)
        };
        segments.push(Segment { source: Source::Record { id, bytes: start..end }, text, encoding_unknown });
    }
    if expect_start != m {
        return Err("the map ranges do not cover the whole text buffer (C04)".into());
    }
    if unknown > 0 {
        warnings
            .push(format!("{label}: {unknown} segment(s) contain bytes at or above 0x80 whose code page is not established; they were not decoded."));
    }
    let scope = if unknown > 0 { Scope::Partial } else { Scope::Complete };
    Ok(Recovered { segments, scope, warnings })
}
