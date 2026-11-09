// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! The shared Contents structure: header, layout table, control block, directory and records
//! (gates G02 to G09). Offsets are computed from the fields, never hard-coded from a sample.

use crate::{Family, LegacyError, SizeCandidate};
use std::collections::{HashMap, HashSet};

pub(crate) const CLSID_AB: &str = "00021201-0000-0000-00c0-000000000046";
pub(crate) const CLSID_C: &str = "00021200-0000-0000-00c0-000000000046";
const TABLE_START: usize = 0x22;
const FIRST_ENTRY: usize = 0x26;
const CONTROL_LEN: usize = 46;

pub(crate) fn bad(m: impl Into<String>) -> LegacyError {
    LegacyError::Malformed(m.into())
}
pub(crate) fn u16_at(b: &[u8], at: usize) -> Option<u16> {
    let s = b.get(at..at.checked_add(2)?)?;
    Some(u16::from_le_bytes([*s.first()?, *s.get(1)?]))
}
pub(crate) fn u32_at(b: &[u8], at: usize) -> Option<u32> {
    let s = b.get(at..at.checked_add(4)?)?;
    Some(u32::from_le_bytes([*s.first()?, *s.get(1)?, *s.get(2)?, *s.get(3)?]))
}
pub(crate) fn need16(b: &[u8], at: usize, what: &str) -> Result<u16, LegacyError> {
    u16_at(b, at).ok_or_else(|| bad(format!("{what} runs past the stream")))
}
pub(crate) fn need32(b: &[u8], at: usize, what: &str) -> Result<u32, LegacyError> {
    u32_at(b, at).ok_or_else(|| bad(format!("{what} runs past the stream")))
}

/// One layout-table entry.
#[derive(Clone, Copy, Debug)]
pub(crate) struct TEntry {
    pub word0: u16,
    pub class: u16,
}

/// One directory record with its serialized extent.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Rec {
    pub id: u16,
    pub parent: u16,
    pub r: usize,
    pub e: usize,
    pub index: u16,
    pub h: usize,
}

pub(crate) struct Layout<'a> {
    pub data: &'a [u8],
    pub family: Family,
    /// Control pointer and directory pointer.
    pub c: usize,
    pub d: usize,
    pub table: HashMap<u16, TEntry>,
    pub records: Vec<Rec>,
    pub by_id: HashMap<u16, usize>,
    pub root: usize,
    pub warnings: Vec<String>,
}

#[derive(Clone, Copy)]
enum Term {
    Zero,
    Low8,
    Low9,
}

impl Term {
    fn ends(self, w: u16) -> bool {
        match self {
            Term::Zero => w == 0,
            Term::Low8 => w & 0x00ff == 0,
            Term::Low9 => w & 0x01ff == 0,
        }
    }
}

type Walk = Vec<(u16, u16, u16, usize)>;

/// Walk `count` ordinary entries from 0x26; the walk must end exactly at `end`.
fn walk(data: &[u8], count: usize, end: usize, term: Term) -> Result<Walk, LegacyError> {
    let mut at = FIRST_ENTRY;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let w0 = need16(data, at, "layout table entry")?;
        let w1 = need16(data, at + 2, "layout table entry")?;
        let w2 = need16(data, at + 4, "layout table entry")?;
        let mut pos = at + 6;
        loop {
            if pos + 2 > end {
                return Err(bad("a layout table entry has no terminator before the control block"));
            }
            let w = need16(data, pos, "layout table tail")?;
            pos += 2;
            if term.ends(w) {
                break;
            }
        }
        out.push((w0, w1, w2, pos));
        at = pos;
    }
    if at != end {
        return Err(bad("the layout table does not end exactly at the control block"));
    }
    Ok(out)
}

impl<'a> Layout<'a> {
    pub(crate) fn parse(data: &'a [u8], clsid: &str) -> Result<Layout<'a>, LegacyError> {
        // G02 header.
        if data.len() < TABLE_START {
            return Err(LegacyError::NotLegacy("Contents is shorter than the legacy header".into()));
        }
        if data.get(..4) != Some(&[0xe8, 0xac, 0x22, 0x00]) {
            return Err(LegacyError::NotLegacy("Contents does not have the legacy discriminator".into()));
        }
        if u16_at(data, 2) != Some(34) || u16_at(data, 4) != Some(300) {
            return Err(bad("legacy header fields +0x02/+0x04 are not the documented values"));
        }
        let len = data.len();
        if need32(data, 8, "header length")? as usize != len {
            return Err(bad("the header length field disagrees with the stream length"));
        }
        let c = need32(data, 0x12, "control pointer")? as usize;
        let d = need32(data, 0x16, "directory pointer")? as usize;
        if need32(data, 0x1a, "table pointer")? as usize != TABLE_START {
            return Err(bad("the layout table does not start at 0x22"));
        }
        // G03 family and table.
        let word0 = need16(data, TABLE_START, "table cardinality")?;
        if need16(data, TABLE_START + 2, "table prefix")? != 6 {
            return Err(LegacyError::Unsupported("legacy layout table prefix is not 6".into()));
        }
        let (family, count, base) = match (clsid, word0) {
            (CLSID_AB, 89) => (Family::A, 88usize, 1u16),
            (CLSID_AB, 94) => (Family::B, 92, 2),
            (CLSID_C, 64) => (Family::C, 64, 0),
            _ => return Err(LegacyError::Unsupported(format!("root class {clsid} with table word {word0} is not a documented profile"))),
        };
        // G04 regions.
        if c < FIRST_ENTRY || c.checked_add(CONTROL_LEN).is_none_or(|e| e > d) || d > len {
            return Err(bad("the control block, directory and table regions contradict each other"));
        }
        if need16(data, c, "control marker")? != 0xf0de || need16(data, c + 2, "control length")? != CONTROL_LEN as u16 {
            return Err(bad("the control block marker or length is not the documented one"));
        }
        if need32(data, c + 0x0a, "control directory pointer")? as usize != d {
            return Err(bad("the control block names a different directory"));
        }
        let ctl_0e = need32(data, c + 0x0e, "control +0x0e")? as usize;
        match family {
            Family::A | Family::B => {
                if d != c + CONTROL_LEN || ctl_0e != d {
                    return Err(bad("the directory does not follow the control block"));
                }
            }
            Family::C => {
                if ctl_0e != c + CONTROL_LEN {
                    return Err(bad("the text buffer does not follow the control block"));
                }
            }
        }
        // Table walk. B must agree under both termination masks.
        let walked = match family {
            Family::A | Family::C => walk(data, count, c, Term::Zero)?,
            Family::B => {
                let a = walk(data, count, c, Term::Low8)?;
                let b = walk(data, count, c, Term::Low9)?;
                if a != b {
                    return Err(LegacyError::Unsupported("legacy B table terminators are ambiguous (low 8 and low 9 bit walks disagree)".into()));
                }
                a
            }
        };
        let mut table = HashMap::new();
        for (k, (w0, w1, _w2, _)) in walked.iter().enumerate() {
            let index = base.checked_add(u16::try_from(k).map_err(|_| bad("table too long"))?).ok_or_else(|| bad("table index overflow"))?;
            table.insert(index, TEntry { word0: *w0, class: *w1 });
        }
        // G05 directory.
        let n = need16(data, d, "directory count")? as usize;
        let max_id = need16(data, d + 2, "directory maximum id")?;
        if n == 0 {
            return Err(bad("the directory is empty"));
        }
        let dir_end =
            n.checked_mul(10).and_then(|x| x.checked_add(d + 4)).filter(|e| *e <= len).ok_or_else(|| bad("the directory runs past the stream"))?;
        let mut raw = Vec::with_capacity(n);
        let mut ids = HashSet::new();
        let mut best = 0u16;
        let mut last = 0usize;
        for i in 0..n {
            let at = d + 4 + 10 * i;
            let id = need16(data, at, "directory entry")?;
            let parent = need16(data, at + 2, "directory entry")?;
            let r = need32(data, at + 4, "directory entry")? as usize;
            if !ids.insert(id) {
                return Err(bad("the directory repeats a record id"));
            }
            if r < dir_end || r >= len || (i > 0 && r <= last) {
                return Err(bad("directory record starts are not distinct, ascending and inside the stream"));
            }
            last = r;
            best = best.max(id);
            raw.push((id, parent, r));
        }
        if best != max_id {
            return Err(bad("the directory maximum id does not match its entries"));
        }
        if data.get(dir_end..raw.first().map_or(dir_end, |x| x.2)).is_none_or(|g| g.iter().any(|b| *b != 0)) {
            return Err(LegacyError::Unsupported("the gap between the directory and the first record is not zero".into()));
        }
        // G06 prefixes.
        let mut warnings = Vec::new();
        let mut records = Vec::with_capacity(n);
        let mut opaque = 0usize;
        for (i, (id, parent, r)) in raw.iter().copied().enumerate() {
            let e = raw.get(i + 1).map_or(len, |x| x.2);
            let index = need16(data, r, "record prefix")?;
            let h = *data.get(r + 3).ok_or_else(|| bad("record prefix runs past the stream"))? as usize;
            if r + 4 > e || h < 4 || h > e - r {
                return Err(bad(format!("record {id:#x} has an unsafe prefix")));
            }
            match table.get(&index) {
                Some(t) if (t.word0 >> 8) as usize != h => return Err(bad(format!("record {id:#x} disagrees with its layout table entry"))),
                None => opaque += 1,
                _ => {}
            }
            records.push(Rec { id, parent, r, e, index, h });
        }
        if opaque > 0 {
            warnings.push(format!("{}: {opaque} record(s) have a layout index outside the table and stay opaque.", family.label()));
        }
        let by_id: HashMap<u16, usize> = records.iter().enumerate().map(|(i, r)| (r.id, i)).collect();
        // G07 root and parents.
        let roots: Vec<usize> = records.iter().enumerate().filter(|(_, r)| !by_id.contains_key(&r.parent)).map(|(i, _)| i).collect();
        let [root] = roots[..] else { return Err(bad("the record graph does not have exactly one root")) };
        for start in 0..records.len() {
            let mut at = start;
            let mut steps = 0usize;
            while at != root {
                steps += 1;
                let parent = records.get(at).map(|r| r.parent).ok_or_else(|| bad("record graph index"))?;
                at = *by_id.get(&parent).ok_or_else(|| bad("a record parent does not resolve"))?;
                if steps > records.len() {
                    return Err(bad("the record parent graph has a cycle"));
                }
            }
        }
        let root_rec = records.get(root).ok_or_else(|| bad("root index"))?;
        if need16(data, 0x0e, "header root id")? != root_rec.id || root_rec.index != 21 {
            return Err(bad("the root record is not the one the header names, or is not layout index 21"));
        }
        Ok(Layout { data, family, c, d, table, records, by_id, root, warnings })
    }

    pub(crate) fn rec(&self, id: u16) -> Option<&Rec> {
        self.by_id.get(&id).and_then(|i| self.records.get(*i))
    }

    pub(crate) fn class_of(&self, r: &Rec) -> Option<u16> {
        self.table.get(&r.index).map(|t| t.class)
    }

    /// The independent extent equation for the record classes the text paths consume
    /// (classes 1, 9, 25 and 2, 10). Returns the repeated count and element width.
    pub(crate) fn check_extent(&self, r: &Rec) -> Result<(usize, usize), String> {
        let class = self.class_of(r).ok_or("its layout index is not in the table")?;
        let t = r.r + r.h;
        let n = u16_at(self.data, t).ok_or("its tail runs past the stream")? as usize;
        let dup = u16_at(self.data, t + 2).ok_or("its tail runs past the stream")? as usize;
        let w = u16_at(self.data, t + 4).ok_or("its tail runs past the stream")? as usize;
        if n != dup || w == 0 {
            return Err("its repeated count or element width is inconsistent".into());
        }
        let span = r.e - r.r;
        let predicted = match class {
            1 | 9 | 25 => n.checked_mul(w).and_then(|x| x.checked_add(r.h + 10)),
            2 | 10 => w.checked_add(4).and_then(|x| x.checked_mul(n)).and_then(|x| x.checked_add(r.h + 22)),
            _ => return Err(format!("class {class} has no documented extent equation")),
        };
        if predicted != Some(span) {
            return Err("its predicted end does not equal the next record start".into());
        }
        Ok((n, w))
    }

    /// G08/G09: the root-size candidate, or a warning saying why it is withheld.
    pub(crate) fn size_candidate(&self) -> (Option<SizeCandidate>, Option<String>) {
        let label = self.family.label();
        let Some(root) = self.records.get(self.root) else { return (None, Some(format!("{label}: no root record."))) };
        let fail = |why: String| (None, Some(format!("{label}: the page size candidate was withheld: {why}.")));
        let (n, w) = match self.check_extent(root) {
            Ok(x) => x,
            Err(why) => return fail(format!("the root record failed its extent check ({why})")),
        };
        if w != 2 {
            return fail("the root tail width is not 2".into());
        }
        let t = root.r + root.h;
        for k in 0..n {
            let Some(id) = u16_at(self.data, t + 10 + 2 * k) else { return fail("a root tail id runs past the record".into()) };
            if self.rec(id).is_none_or(|c| c.parent != root.id) {
                return fail("a root tail id does not resolve to a child of the root".into());
            }
        }
        if root.r + 0x1c > root.r + root.h || root.r + 0x1c > root.e {
            return fail("the root prefix is too short for the size fields".into());
        }
        let (Some(wd), Some(ht)) = (u32_at(self.data, root.r + 0x14), u32_at(self.data, root.r + 0x18)) else {
            return fail("the size fields run past the stream".into());
        };
        if wd == 0 || ht == 0 {
            return fail("a size field is zero".into());
        }
        let size = SizeCandidate { raw_width: wd, raw_height: ht, inferred_mm: (f64::from(wd) / 36000.0, f64::from(ht) / 36000.0) };
        (
            Some(size),
            Some(format!(
                "{label}: the page size is a raw candidate (units not verified; inferred {:.1} x {:.1} mm at 36000 units per mm).",
                size.inferred_mm.0, size.inferred_mm.1
            )),
        )
    }
}
