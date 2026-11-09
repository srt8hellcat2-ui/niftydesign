// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! A/B text: the Quill text store (gates AB01 to AB08) and the Contents story links.

use crate::contents::{Layout, u16_at, u32_at};
use crate::{MAX_QUILL_BLOCKS, MAX_SEGMENTS, Recovered, Scope, Segment, Source};
use std::collections::{HashMap, HashSet};

const CR: u16 = 0x000d;

struct Section {
    tag: [u8; 4],
    off: usize,
    len: usize,
}

/// AB01 and AB02: follow the bounded directory chain and collect every section.
fn sections(q: &[u8]) -> Result<Vec<Section>, String> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    let mut at = 0x18usize;
    while at != u32::MAX as usize {
        if seen.len() >= MAX_QUILL_BLOCKS || !seen.insert(at) {
            return Err("the Quill directory chain loops or is too long".into());
        }
        let count = u16_at(q, at + 2).ok_or("a Quill directory block runs past the stream")? as usize;
        let next = u32_at(q, at + 4).ok_or("a Quill directory block runs past the stream")? as usize;
        for k in 0..count {
            let e = k.checked_mul(24).and_then(|x| x.checked_add(at + 8)).ok_or("Quill directory overflow")?;
            let tag: [u8; 4] = q.get(e + 2..e + 6).and_then(|s| s.try_into().ok()).ok_or("a Quill directory entry runs past the stream")?;
            let off = u32_at(q, e + 16).ok_or("a Quill directory entry runs past the stream")? as usize;
            let len = u32_at(q, e + 20).ok_or("a Quill directory entry runs past the stream")? as usize;
            if off.checked_add(len).is_none_or(|end| end > q.len()) {
                return Err("a Quill section runs past the stream".into());
            }
            out.push(Section { tag, off, len });
            if out.len() > 100_000 {
                return Err("too many Quill sections".into());
            }
        }
        at = next;
    }
    Ok(out)
}

fn unique<'a>(s: &'a [Section], tag: &[u8; 4]) -> Result<Option<&'a Section>, String> {
    let mut it = s.iter().filter(|x| &x.tag == tag);
    let first = it.next();
    if it.next().is_some() {
        return Err(format!("the Quill store has more than one {} section", String::from_utf8_lossy(tag).trim_end()));
    }
    Ok(first)
}

/// SYID ids and the TEXT units cut by STRS, validated (AB03 to AB05).
struct Catalog {
    ids: Vec<u32>,
    units: Vec<u16>,
    ranges: Vec<std::ops::Range<usize>>,
}

fn catalog(q: &[u8]) -> Result<Catalog, String> {
    let secs = sections(q)?;
    let syid = unique(&secs, b"SYID")?.ok_or("the Quill store has no SYID section")?;
    let text = unique(&secs, b"TEXT")?;
    let strs = unique(&secs, b"STRS")?;
    let sy = q.get(syid.off..syid.off + syid.len).ok_or("SYID range")?;
    let n = u32_at(sy, 4).ok_or("SYID is shorter than 8 bytes")? as usize;
    if n > MAX_SEGMENTS {
        return Err("SYID holds more ids than the reader limit".into());
    }
    let id_end = n.checked_mul(4).and_then(|x| x.checked_add(8)).filter(|e| *e <= sy.len()).ok_or("the SYID ids run past the section")?;
    let ids: Vec<u32> = (0..n).filter_map(|k| u32_at(sy, 8 + 4 * k)).collect();
    if ids.len() != n || id_end > sy.len() {
        return Err("the SYID ids run past the section".into());
    }
    let mut seen = HashSet::new();
    if ids.iter().any(|i| *i == 0 || !seen.insert(*i)) {
        return Err("SYID has a zero or repeated story id".into());
    }
    if n == 0 {
        if text.is_some() || strs.is_some() {
            return Err("SYID is empty but TEXT or STRS is present (an unsupported empty-store variant)".into());
        }
        return Ok(Catalog { ids, units: Vec::new(), ranges: Vec::new() });
    }
    let (text, strs) = (text.ok_or("the Quill store has no TEXT section")?, strs.ok_or("the Quill store has no STRS section")?);
    let st = q.get(strs.off..strs.off + strs.len).ok_or("STRS range")?;
    if u32_at(st, 0).ok_or("STRS is too short")? as usize != n {
        return Err("STRS and SYID disagree about the story count".into());
    }
    if u32_at(st, 4).ok_or("STRS is too short")? != 8 {
        return Err("STRS has a length-list displacement other than 8".into());
    }
    let lens_at = 12usize;
    if lens_at.checked_add(n * 4).is_none_or(|e| e > st.len()) {
        return Err("the STRS lengths run past the section".into());
    }
    let tx = q.get(text.off..text.off + text.len).ok_or("TEXT range")?;
    if tx.len() % 2 != 0 {
        return Err("TEXT has an odd byte count".into());
    }
    let units: Vec<u16> = tx.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
    let mut ranges = Vec::with_capacity(n);
    let mut start = 0usize;
    for k in 0..n {
        let len = u32_at(st, lens_at + 4 * k).ok_or("STRS length")? as usize;
        let end = start.checked_add(len).filter(|e| *e <= units.len()).ok_or("STRS lengths run past TEXT")?;
        if len == 0 || units.get(end - 1) != Some(&CR) {
            return Err("a story slice is empty or does not end in a paragraph mark".into());
        }
        ranges.push(start..end);
        start = end;
    }
    if start != units.len() {
        return Err("the STRS lengths do not add up to the whole TEXT section".into());
    }
    Ok(Catalog { ids, units, ranges })
}

/// Read the A/B text store through the Contents story links.
pub(crate) fn read_quill(layout: &Layout<'_>, quill: Option<&[u8]>) -> Result<Recovered, String> {
    let label = layout.family.label();
    let q = quill.ok_or("there is no Quill text stream (AB01)")?;
    let cat = catalog(q)?;
    let by_id: HashMap<u32, usize> = cat.ids.iter().enumerate().map(|(i, id)| (*id, i)).collect();
    let mut warnings = Vec::new();
    let mut segments: Vec<Segment> = Vec::new();
    let mut emitted: HashMap<u32, usize> = HashMap::new();
    let mut withheld = 0usize;
    for rec in &layout.records {
        // Candidate link records: index 8 (class 9, H=118, width 2) and index 10 (class 1, H=116, width 8).
        let (anchor, class, h, width) = match rec.index {
            8 => (0x58usize, 9u16, 118usize, 2usize),
            10 => (0x66, 1, 116, 8),
            _ => continue,
        };
        if let Err(why) = layout.check_profile(rec, class, h, width) {
            withheld += 1;
            warnings.push(format!("{label}: record {:#x} was not used as a story link: {why}.", rec.id));
            continue;
        }
        let Some(v) = u32_at(layout.data, rec.r + anchor) else {
            withheld += 1;
            warnings.push(format!("{label}: record {:#x} is too short for a story link.", rec.id));
            continue;
        };
        if v == 0 {
            continue;
        }
        if cat.ids.is_empty() {
            return Err("the Quill store is empty but a Contents link points into it (AB08)".into());
        }
        if v > 0xffff {
            withheld += 1;
            warnings.push(format!("{label}: record {:#x} holds a story link of unsupported width.", rec.id));
            continue;
        }
        let Some(&slot) = by_id.get(&v) else {
            withheld += 1;
            warnings.push(format!("{label}: record {:#x} links to story {v}, which the Quill store does not list.", rec.id));
            continue;
        };
        if let Some(&at) = emitted.get(&v) {
            warnings.push(format!("{label}: story {v} is linked from more than one record; it is reported once with every linking record."));
            if let Some(Segment { source: Source::Story { records, .. }, .. }) = segments.get_mut(at) {
                records.push(rec.id);
            }
            continue;
        }
        if segments.len() >= MAX_SEGMENTS {
            return Err("more stories than the reader limit".into());
        }
        let range = cat.ranges.get(slot).cloned().ok_or("story slot")?;
        let slice = cat.units.get(range.clone()).ok_or("story slice")?;
        match String::from_utf16(slice) {
            Ok(text) => {
                emitted.insert(v, segments.len());
                segments.push(Segment {
                    source: Source::Story { id: v, units: range, records: vec![rec.id] },
                    text: Some(text),
                    encoding_unknown: false,
                });
            }
            Err(_) => {
                withheld += 1;
                warnings.push(format!("{label}: story {v} is not valid UTF-16 and was withheld."));
            }
        }
    }
    if cat.ids.is_empty() {
        // Known-empty only when every candidate record passed its checks and every probe was zero.
        let scope = if withheld == 0 { Scope::EmptyKnownStore } else { Scope::Partial };
        if withheld > 0 {
            warnings.push(format!("{label}: a candidate story link failed its checks, so the store is not asserted to be empty."));
        }
        return Ok(Recovered { segments, scope, warnings });
    }
    let unlinked = cat.ids.iter().filter(|id| !emitted.contains_key(id)).count();
    if unlinked > 0 {
        warnings
            .push(format!("{label}: {unlinked} story id(s) in the Quill store have no validated Contents link and were not reported as stories."));
    }
    let scope = if withheld > 0 || segments.is_empty() { Scope::Partial } else { Scope::Complete };
    if segments.is_empty() {
        warnings.push(format!("{label}: no story was linked from Contents; the text is unrecovered, not known to be absent."));
    }
    Ok(Recovered { segments, scope, warnings })
}
