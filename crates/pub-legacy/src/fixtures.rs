// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! Byte-built legacy files for tests (docs/pub-format/legacy-minimal-reader.md, "Synthetic fixture
//! specification"). Nothing here is Publisher-made. Compiled for this crate's tests and, with the
//! `fixtures` feature, for other crates' tests.
#![allow(dead_code, clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::indexing_slicing)]
use std::io::Write;

pub(crate) const CLSID_AB: &str = "00021201-0000-0000-00c0-000000000046";
pub(crate) const CLSID_C: &str = "00021200-0000-0000-00c0-000000000046";

pub(crate) fn p16(v: &mut Vec<u8>, x: u16) {
    v.extend(x.to_le_bytes());
}
pub(crate) fn p32(v: &mut Vec<u8>, x: u32) {
    v.extend(x.to_le_bytes());
}
pub(crate) fn set16(b: &mut [u8], at: usize, x: u16) {
    b[at..at + 2].copy_from_slice(&x.to_le_bytes());
}
pub(crate) fn set32(b: &mut [u8], at: usize, x: u32) {
    b[at..at + 4].copy_from_slice(&x.to_le_bytes());
}

/// One record: layout index, class, prefix length H and the tail written after the prefix.
#[derive(Clone)]
pub(crate) struct R {
    pub(crate) id: u16,
    pub(crate) parent: u16,
    pub(crate) index: u16,
    pub(crate) class: u16,
    pub(crate) h: usize,
    /// Bytes of the prefix to set: (offset, little-endian u32).
    pub(crate) set: Vec<(usize, u32)>,
    pub(crate) tail: Vec<u8>,
}

pub(crate) fn array_tail(n: u16, w: u16, extra: &[u8]) -> Vec<u8> {
    let mut t = Vec::new();
    p16(&mut t, n);
    p16(&mut t, n);
    p16(&mut t, w);
    t.extend([0u8; 4]);
    t.extend(extra);
    t
}

#[derive(Clone)]
pub(crate) struct Model {
    pub(crate) clsid: &'static str,
    pub(crate) word0: u16,
    pub(crate) base: u16,
    pub(crate) count: usize,
    pub(crate) b_terms: bool,
    pub(crate) records: Vec<R>,
    pub(crate) root_id: u16,
    /// C only: the text core (without trailer) and slot/boundary values.
    pub(crate) core: Vec<u8>,
    pub(crate) p_override: Option<usize>,
    pub(crate) quill: Option<Vec<u8>>,
    pub(crate) contents_extra: usize,
    pub(crate) header_len_delta: i64,
    pub(crate) dir_swap: Option<(usize, usize)>,
    pub(crate) dup_start: bool,
}

pub(crate) fn root(children: &[u16], w: u32, h: u32, root_h: usize) -> R {
    let mut tail = array_tail(children.len() as u16, 2, &[]);
    for c in children {
        p16(&mut tail, *c);
    }
    R { id: 0x200, parent: 0xffff, index: 21, class: 25, h: root_h, set: vec![(0x14, w), (0x18, h)], tail }
}

pub(crate) fn link_rec(id: u16, index: u16, class: u16, h: usize, anchor: usize, story: u32, w: u16) -> R {
    R { id, parent: 0x200, index, class, h, set: vec![(anchor, story)], tail: array_tail(0, w, &[]) }
}

pub(crate) fn ab_model(clsid_b: bool) -> Model {
    let records =
        vec![root(&[0x201, 0x202], 7_560_000, 10_692_000, 178), link_rec(0x201, 8, 9, 118, 0x58, 5, 2), link_rec(0x202, 10, 1, 116, 0x66, 9, 8)];
    Model {
        clsid: CLSID_AB,
        word0: if clsid_b { 94 } else { 89 },
        base: if clsid_b { 2 } else { 1 },
        count: if clsid_b { 92 } else { 88 },
        b_terms: clsid_b,
        records,
        root_id: 0x200,
        core: Vec::new(),
        p_override: None,
        quill: Some(quill(&[5, 9], &[2, 2], "A\rB\r", None)),
        contents_extra: 0,
        header_len_delta: 0,
        dir_swap: None,
        dup_start: false,
    }
}

pub(crate) fn c_model() -> Model {
    let core = b"Alpha\r\n\x0cBeta\r\n\x0c".to_vec();
    // The map: n=4 slots 0,8,8,15, extra boundary 15, then four 10-byte elements.
    let mut extra = Vec::new();
    for v in [0u32, 8, 8, 15, 15] {
        p32(&mut extra, v);
    }
    let mut elems = Vec::new();
    for (id, kind) in [(0x201u16, 1u16), (0, 0), (0x202, 0), (0, 0)] {
        p16(&mut elems, id);
        p16(&mut elems, kind);
        elems.extend([0u8; 6]);
    }
    let mut map_tail = Vec::new();
    p16(&mut map_tail, 4);
    p16(&mut map_tail, 4);
    p16(&mut map_tail, 10);
    map_tail.extend([0u8; 12]);
    map_tail.extend(extra);
    map_tail.extend(elems);
    let records = vec![
        root(&[0x201, 0x202, 0x203], 7_772_400, 10_058_400, 136),
        R { id: 0x201, parent: 0x200, index: 0, class: 9, h: 92, set: vec![], tail: array_tail(0, 2, &[]) },
        R { id: 0x202, parent: 0x200, index: 1, class: 1, h: 96, set: vec![], tail: array_tail(0, 14, &[]) },
        R { id: 0x203, parent: 0x200, index: 26, class: 10, h: 4, set: vec![], tail: map_tail },
    ];
    Model {
        clsid: CLSID_C,
        word0: 64,
        base: 0,
        count: 64,
        b_terms: false,
        records,
        root_id: 0x200,
        core,
        p_override: None,
        quill: None,
        contents_extra: 0,
        header_len_delta: 0,
        dir_swap: None,
        dup_start: false,
    }
}

pub(crate) fn build_contents(m: &Model) -> Vec<u8> {
    let c_family = m.clsid == CLSID_C;
    // Table: entries for every index; the used indices carry their class and H.
    let mut table = Vec::new();
    p16(&mut table, m.word0);
    p16(&mut table, 6);
    for k in 0..m.count {
        let index = m.base + k as u16;
        let used = m.records.iter().find(|r| r.index == index);
        let (class, h) = used.map_or((0u16, 4usize), |r| (r.class, r.h));
        p16(&mut table, ((h as u16) << 8) | 0x11);
        p16(&mut table, class);
        p16(&mut table, 0x0007);
        p16(&mut table, 0x0abc); // a non-terminating tail word
        if m.b_terms {
            p16(&mut table, [0u16, 0x0600, 0x0800, 0xa600][k % 4]);
        } else {
            p16(&mut table, 0);
        }
    }
    let c = 0x22 + table.len();
    let mut out = vec![0u8; 0x22];
    out[..4].copy_from_slice(&[0xe8, 0xac, 0x22, 0x00]);
    set16(&mut out, 4, 300);
    set16(&mut out, 0x0e, m.root_id);
    set32(&mut out, 0x12, c as u32);
    set32(&mut out, 0x1a, 0x22);
    out.extend(&table);
    // Control block (46 bytes), text buffer for C, then the directory.
    let mut control = vec![0u8; 46];
    set16(&mut control, 0, 0xf0de);
    set16(&mut control, 2, 46);
    out.extend(&control);
    let mut d = c + 46;
    if c_family {
        let s = c + 46;
        let p = s + m.core.len();
        out.extend(&m.core);
        out.extend([0x0d, 0x0a]);
        let boundary = (p + 2).div_ceil(512) * 512;
        out.resize(boundary, 0);
        d = boundary;
        set32(&mut out, c + 0x0e, s as u32);
        set32(&mut out, c + 0x12, m.p_override.unwrap_or(p) as u32);
    } else {
        set32(&mut out, c + 0x0e, d as u32);
    }
    set32(&mut out, c + 0x0a, d as u32);
    set32(&mut out, 0x16, d as u32);
    // Directory, a zero gap, then the records.
    let n = m.records.len();
    let dir_end = d + 4 + 10 * n;
    let gap = 10;
    let mut starts = Vec::new();
    let mut body = Vec::new();
    for r in &m.records {
        starts.push(dir_end + gap + body.len());
        let mut rec = vec![0u8; r.h];
        set16(&mut rec, 0, r.index);
        rec[2] = 0x11;
        rec[3] = r.h as u8;
        for (off, v) in &r.set {
            if *off + 4 <= rec.len() {
                set32(&mut rec, *off, *v);
            }
        }
        rec.extend(&r.tail);
        body.extend(rec);
    }
    if m.dup_start && n > 1 {
        starts[1] = starts[0];
    }
    if let Some((a, b)) = m.dir_swap {
        starts.swap(a, b);
    }
    out.resize(d, 0);
    p16(&mut out, n as u16);
    p16(&mut out, m.records.iter().map(|r| r.id).max().unwrap_or(0));
    for (r, s) in m.records.iter().zip(&starts) {
        p16(&mut out, r.id);
        p16(&mut out, r.parent);
        p32(&mut out, *s as u32);
        p16(&mut out, 0x1640);
    }
    out.extend(vec![0u8; gap]);
    out.extend(body);
    out.extend(vec![0u8; m.contents_extra]);
    let len = (out.len() as i64 + m.header_len_delta) as u32;
    set32(&mut out, 8, len);
    out
}

/// A Quill stream with TEXT, SYID and STRS (`ids`, `lens`, `text`); `next` overrides the first block's next pointer.
pub(crate) fn quill(ids: &[u32], lens: &[u32], text: &str, next: Option<u32>) -> Vec<u8> {
    let units: Vec<u8> = text.encode_utf16().flat_map(u16::to_le_bytes).collect();
    quill_raw(ids, lens, &units, next)
}

pub(crate) fn quill_raw(ids: &[u32], lens: &[u32], text: &[u8], next: Option<u32>) -> Vec<u8> {
    let mut syid = vec![0u8; 4];
    p32(&mut syid, ids.len() as u32);
    for i in ids {
        p32(&mut syid, *i);
    }
    let mut strs = Vec::new();
    p32(&mut strs, lens.len() as u32);
    p32(&mut strs, 8);
    p32(&mut strs, 0);
    for l in lens {
        p32(&mut strs, *l);
    }
    let mut sections: Vec<(&[u8; 4], Vec<u8>)> = vec![(b"SYID", syid)];
    if !ids.is_empty() {
        sections.push((b"TEXT", text.to_vec()));
        sections.push((b"STRS", strs));
    }
    let mut q = vec![0u8; 512];
    q[..8].copy_from_slice(b"CHNKINK ");
    set16(&mut q, 0x18 + 2, sections.len() as u16);
    set32(&mut q, 0x18 + 4, next.unwrap_or(u32::MAX));
    for (i, (tag, data)) in sections.into_iter().enumerate() {
        let e = 0x18 + 8 + 24 * i;
        q[e + 2..e + 6].copy_from_slice(tag);
        let off = q.len() as u32;
        set32(&mut q, e + 16, off);
        set32(&mut q, e + 20, data.len() as u32);
        q.extend(data);
    }
    q
}

pub(crate) fn compound(clsid: &str, contents: &[u8], quill: Option<&[u8]>) -> Vec<u8> {
    let mut cf = cfb::CompoundFile::create_with_version(cfb::Version::V3, std::io::Cursor::new(Vec::new())).unwrap();
    cf.set_storage_clsid("/", uuid::Uuid::parse_str(clsid).unwrap()).unwrap();
    cf.create_stream("/Contents").unwrap().write_all(contents).unwrap();
    if let Some(q) = quill {
        cf.create_storage("/Quill").unwrap();
        cf.create_storage("/Quill/QuillSub").unwrap();
        cf.create_stream("/Quill/QuillSub/CONTENTS").unwrap().write_all(q).unwrap();
    }
    cf.flush().unwrap();
    cf.into_inner().into_inner()
}

pub(crate) fn file(m: &Model) -> Vec<u8> {
    compound(m.clsid, &build_contents(m), m.quill.as_deref())
}

/// A complete invented A-profile file: two stories, "A" and "B", each ending in a paragraph mark.
pub fn legacy_a_file() -> Vec<u8> {
    file(&ab_model(false))
}

/// A complete invented C-profile file with two ASCII segments.
pub fn legacy_c_file() -> Vec<u8> {
    file(&c_model())
}

/// An invented A-profile file whose Quill text store is missing, so only the text path is refused.
pub fn legacy_a_file_without_text_store() -> Vec<u8> {
    let mut m = ab_model(false);
    m.quill = None;
    file(&m)
}
