// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! Gate tests on files this module writes byte by byte (docs/pub-format/legacy-minimal-reader.md,
//! "Synthetic fixture specification"). Nothing here is Publisher-made, and passing them confirms the
//! reader's guards, not the format. One mutation per case.
use super::*;
use std::io::Write;

const CLSID_AB: &str = "00021201-0000-0000-00c0-000000000046";
const CLSID_C: &str = "00021200-0000-0000-00c0-000000000046";

fn p16(v: &mut Vec<u8>, x: u16) {
    v.extend(x.to_le_bytes());
}
fn p32(v: &mut Vec<u8>, x: u32) {
    v.extend(x.to_le_bytes());
}
fn set16(b: &mut [u8], at: usize, x: u16) {
    b[at..at + 2].copy_from_slice(&x.to_le_bytes());
}
fn set32(b: &mut [u8], at: usize, x: u32) {
    b[at..at + 4].copy_from_slice(&x.to_le_bytes());
}

/// One record: layout index, class, prefix length H and the tail written after the prefix.
#[derive(Clone)]
struct R {
    id: u16,
    parent: u16,
    index: u16,
    class: u16,
    h: usize,
    /// Bytes of the prefix to set: (offset, little-endian u32).
    set: Vec<(usize, u32)>,
    tail: Vec<u8>,
}

fn array_tail(n: u16, w: u16, extra: &[u8]) -> Vec<u8> {
    let mut t = Vec::new();
    p16(&mut t, n);
    p16(&mut t, n);
    p16(&mut t, w);
    t.extend([0u8; 4]);
    t.extend(extra);
    t
}

#[derive(Clone)]
struct Model {
    clsid: &'static str,
    word0: u16,
    base: u16,
    count: usize,
    b_terms: bool,
    records: Vec<R>,
    root_id: u16,
    /// C only: the text core (without trailer) and slot/boundary values.
    core: Vec<u8>,
    p_override: Option<usize>,
    quill: Option<Vec<u8>>,
    contents_extra: usize,
    header_len_delta: i64,
    dir_swap: Option<(usize, usize)>,
    dup_start: bool,
}

fn root(children: &[u16], w: u32, h: u32) -> R {
    let mut tail = array_tail(children.len() as u16, 2, &[]);
    for c in children {
        p16(&mut tail, *c);
    }
    R { id: 0x200, parent: 0xffff, index: 21, class: 25, h: 0x20, set: vec![(0x14, w), (0x18, h)], tail }
}

fn link_rec(id: u16, index: u16, class: u16, h: usize, anchor: usize, story: u32, w: u16) -> R {
    R { id, parent: 0x200, index, class, h, set: vec![(anchor, story)], tail: array_tail(0, w, &[]) }
}

fn ab_model(clsid_b: bool) -> Model {
    let records =
        vec![root(&[0x201, 0x202], 7_560_000, 10_692_000), link_rec(0x201, 8, 9, 0x60, 0x58, 5, 2), link_rec(0x202, 10, 1, 0x6c, 0x66, 9, 8)];
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

fn c_model() -> Model {
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
        root(&[0x201, 0x202, 0x203], 7_772_400, 10_058_400),
        R { id: 0x201, parent: 0x200, index: 0, class: 9, h: 0x5c, set: vec![], tail: array_tail(0, 2, &[]) },
        R { id: 0x202, parent: 0x200, index: 1, class: 1, h: 0x60, set: vec![], tail: array_tail(0, 14, &[]) },
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

fn build_contents(m: &Model) -> Vec<u8> {
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
fn quill(ids: &[u32], lens: &[u32], text: &str, next: Option<u32>) -> Vec<u8> {
    let units: Vec<u8> = text.encode_utf16().flat_map(u16::to_le_bytes).collect();
    quill_raw(ids, lens, &units, next)
}

fn quill_raw(ids: &[u32], lens: &[u32], text: &[u8], next: Option<u32>) -> Vec<u8> {
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

fn compound(clsid: &str, contents: &[u8], quill: Option<&[u8]>) -> Vec<u8> {
    let mut cf = cfb::CompoundFile::create(std::io::Cursor::new(Vec::new())).unwrap();
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

fn file(m: &Model) -> Vec<u8> {
    compound(m.clsid, &build_contents(m), m.quill.as_deref())
}

fn refused(bytes: &[u8], kind: fn(&LegacyError) -> bool, needle: &str) {
    match read_legacy(bytes) {
        Err(e) => {
            assert!(kind(&e) && e.to_string().contains(needle), "wrong refusal: {e}");
        }
        Ok(r) => panic!("expected a refusal containing {needle:?}, got {:?} {:?}", r.scope, r.warnings),
    }
}
fn is_bad(e: &LegacyError) -> bool {
    matches!(e, LegacyError::Malformed(_))
}
fn is_unsupported(e: &LegacyError) -> bool {
    matches!(e, LegacyError::Unsupported(_))
}
fn text_of(r: &LegacyText) -> Vec<Option<String>> {
    r.segments.iter().map(|s| s.text.clone()).collect()
}
fn warned(r: &LegacyText, needle: &str) -> bool {
    r.warnings.iter().any(|w| w.contains(needle))
}

// ---------- positive models ----------

#[test]
fn a_positive_recovers_two_linked_stories_and_a_raw_size() {
    let r = read_legacy(&file(&ab_model(false))).unwrap();
    assert_eq!(r.family, Family::A);
    assert_eq!(r.scope, Scope::Complete);
    assert_eq!(text_of(&r), [Some("A\r".to_string()), Some("B\r".to_string())]);
    assert_eq!(r.segments[0].source, Source::Story { id: 5, units: 0..2 });
    assert_eq!(r.segments[1].source, Source::Story { id: 9, units: 2..4 });
    let size = r.size.unwrap();
    assert_eq!((size.raw_width, size.raw_height), (7_560_000, 10_692_000));
    assert!((size.inferred_mm.0 - 210.0).abs() < 1e-9 && (size.inferred_mm.1 - 297.0).abs() < 1e-9);
    assert!(warned(&r, "experimental legacy A") && warned(&r, "not recovered"));
}

#[test]
fn b_positive_uses_the_masked_terminators_and_reports_family_b() {
    let r = read_legacy(&file(&ab_model(true))).unwrap();
    assert_eq!(r.family, Family::B);
    assert_eq!(text_of(&r), [Some("A\r".to_string()), Some("B\r".to_string())]);
    assert!(r.warnings.iter().all(|w| !w.contains("legacy A")));
}

#[test]
fn a_empty_store_is_an_explicit_empty_state() {
    let mut m = ab_model(false);
    m.quill = Some(quill(&[], &[], "", None));
    m.records[1].set = vec![];
    m.records[2].set = vec![];
    let r = read_legacy(&file(&m)).unwrap();
    assert_eq!(r.scope, Scope::EmptyKnownStore);
    assert!(r.segments.is_empty());
}

#[test]
fn c_positive_recovers_two_ascii_segments_without_the_trailer() {
    let r = read_legacy(&file(&c_model())).unwrap();
    assert_eq!(r.family, Family::C);
    assert_eq!(r.scope, Scope::Complete);
    assert_eq!(text_of(&r), [Some("Alpha\r\n\u{c}".to_string()), Some("Beta\r\n\u{c}".to_string())]);
    assert_eq!(r.segments[0].source, Source::Record { id: 0x201, bytes: 0..8 });
    assert_eq!(r.segments[1].source, Source::Record { id: 0x202, bytes: 8..15 });
    assert_eq!(r.size.unwrap().raw_width, 7_772_400);
}

#[test]
fn relocated_structures_still_parse() {
    // A different root id, and an extra bounded record, so no sample offset or id is hard-coded.
    let mut m = ab_model(false);
    m.root_id = 0x300;
    m.records[0].id = 0x300;
    m.records[1].parent = 0x300;
    m.records[2].parent = 0x300;
    m.records[0].tail = {
        let mut t = array_tail(2, 2, &[]);
        p16(&mut t, 0x201);
        p16(&mut t, 0x202);
        t
    };
    assert_eq!(read_legacy(&file(&m)).unwrap().segments.len(), 2);
}

// ---------- G01 container and budgets ----------

#[test]
fn g01_container_failures_are_refused_without_carving() {
    refused(b"not a compound file at all", |e| matches!(e, LegacyError::NotLegacy(_)), "compound file");
    let good = file(&ab_model(false));
    let mut cut = good.clone();
    cut.truncate(700);
    assert!(read_legacy(&cut).is_err());
    assert!(!looks_legacy(b"plain bytes"));
    assert!(looks_legacy(&good));
}

// ---------- G02 header ----------

#[test]
fn g02_header_checks() {
    let mut m = ab_model(false);
    m.header_len_delta = 1;
    refused(&file(&m), is_bad, "header length");
    let mut c = build_contents(&ab_model(false));
    set16(&mut c, 4, 301);
    refused(&compound(CLSID_AB, &c, ab_model(false).quill.as_deref()), is_bad, "+0x02/+0x04");
    let mut c = build_contents(&ab_model(false));
    c[0] = 0xe9;
    refused(&compound(CLSID_AB, &c, None), |e| matches!(e, LegacyError::NotLegacy(_)), "discriminator");
}

// ---------- G03 family and table ----------

#[test]
fn g03_unknown_cardinality_and_ambiguous_b_terminators_are_refused() {
    let mut m = ab_model(false);
    m.word0 = 90;
    refused(&file(&m), is_unsupported, "not a documented profile");
    // B: a non-terminating tail word 0x0100 ends the low-8 walk but not the low-9 walk.
    let m = ab_model(true);
    let mut c = build_contents(&m);
    // The first entry's non-terminating word is at 0x26 + 6.
    set16(&mut c, 0x26 + 6, 0x0100);
    refused(&compound(CLSID_AB, &c, m.quill.as_deref()), is_unsupported, "terminators are ambiguous");
}

// ---------- G04 regions ----------

#[test]
fn g04_contradictory_regions_are_refused() {
    let m = ab_model(false);
    let mut c = build_contents(&m);
    let ctl = u32_at_test(&c, 0x12);
    set32(&mut c, 0x16, (ctl + 20) as u32);
    refused(&compound(CLSID_AB, &c, m.quill.as_deref()), is_bad, "regions contradict");
    let mut m = c_model();
    m.p_override = Some(10_000);
    c_refused(&m, "text buffer bounds");
}

fn u32_at_test(b: &[u8], at: usize) -> usize {
    u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]]) as usize
}

// ---------- G05 directory ----------

#[test]
fn g05_directory_failures() {
    let mut m = ab_model(false);
    m.dup_start = true;
    refused(&file(&m), is_bad, "distinct, ascending");
    let mut m = ab_model(false);
    m.records[2].id = 0x201;
    refused(&file(&m), is_bad, "repeats a record id");
    let mut m = ab_model(false);
    m.dir_swap = Some((1, 2));
    refused(&file(&m), is_bad, "ascending");
}

// ---------- G06 prefixes ----------

#[test]
fn g06_prefix_disagreeing_with_the_table_is_refused() {
    let m = ab_model(false);
    let mut c = build_contents(&m);
    let d = u32_at_test(&c, 0x16);
    let first = u32_at_test(&c, d + 4 + 4);
    c[first + 3] = 0x21; // H stays inside its span but no longer matches the table entry
    refused(&compound(CLSID_AB, &c, m.quill.as_deref()), is_bad, "disagrees with its layout table");
}

// ---------- G07 root and parents ----------

#[test]
fn g07_two_roots_and_cycles_are_refused() {
    let mut m = ab_model(false);
    m.records[1].parent = 0xfffe;
    refused(&file(&m), is_bad, "exactly one root");
    let mut m = ab_model(false);
    m.records[1].parent = 0x202;
    m.records[2].parent = 0x201;
    // The two children now point at each other, leaving the real root with no children.
    refused(&file(&m), is_bad, "cycle");
    let mut m = ab_model(false);
    m.root_id = 0x999;
    refused(&file(&m), is_bad, "header names");
}

// ---------- G08 consumed extents and G09 size ----------

#[test]
fn g08_a_broken_link_record_is_withheld_and_a_broken_root_withholds_only_the_size() {
    let mut m = ab_model(false);
    m.records[1].tail = array_tail(3, 2, &[]); // count 3 but no array bytes
    let r = read_legacy(&file(&m)).unwrap();
    assert_eq!(r.scope, Scope::Partial);
    assert_eq!(text_of(&r), [Some("B\r".to_string())]);
    assert!(warned(&r, "extent") || warned(&r, "predicted end"));
    let mut m = ab_model(false);
    m.records[0].tail = {
        let mut t = array_tail(2, 2, &[]);
        p16(&mut t, 0x201);
        p16(&mut t, 0x777); // a dangling child id
        t
    };
    let r = read_legacy(&file(&m)).unwrap();
    assert!(r.size.is_none() && warned(&r, "size candidate was withheld"));
    assert_eq!(r.segments.len(), 2);
}

#[test]
fn g09_zero_width_withholds_the_size_but_not_the_text() {
    let mut m = ab_model(false);
    m.records[0].set = vec![(0x14, 0), (0x18, 10)];
    let r = read_legacy(&file(&m)).unwrap();
    assert!(r.size.is_none() && warned(&r, "a size field is zero"));
    assert_eq!(r.segments.len(), 2);
}

// ---------- AB01..AB08 ----------

fn text_refused(m: &Model, needle: &str) {
    let r = read_legacy(&file(m)).unwrap();
    assert_eq!(r.scope, Scope::TextRefused, "{:?}", r.warnings);
    assert!(r.segments.is_empty());
    assert!(warned(&r, needle), "{:?}", r.warnings);
    assert!(r.size.is_some(), "the size candidate survives a text refusal");
}

#[test]
fn ab01_missing_or_cyclic_quill_is_refused() {
    let mut m = ab_model(false);
    m.quill = None;
    text_refused(&m, "no Quill text stream");
    let mut m = ab_model(false);
    m.quill = Some(quill(&[5, 9], &[2, 2], "A\rB\r", Some(0x18)));
    text_refused(&m, "loops");
}

#[test]
fn ab02_an_overrunning_or_duplicated_section_is_refused() {
    let mut q = quill(&[5, 9], &[2, 2], "A\rB\r", None);
    set32(&mut q, 0x18 + 8 + 24 + 20, 100_000); // TEXT length overruns
    let mut m = ab_model(false);
    m.quill = Some(q);
    text_refused(&m, "runs past");
    let mut q = quill(&[5, 9], &[2, 2], "A\rB\r", None);
    // Turn STRS into a second SYID section.
    q[0x18 + 8 + 24 * 2 + 2..0x18 + 8 + 24 * 2 + 6].copy_from_slice(b"SYID");
    let mut m = ab_model(false);
    m.quill = Some(q);
    text_refused(&m, "more than one SYID");
}

#[test]
fn ab03_repeated_ids_are_refused() {
    let mut m = ab_model(false);
    m.quill = Some(quill(&[5, 5], &[2, 2], "A\rB\r", None));
    text_refused(&m, "repeated story id");
}

#[test]
fn ab04_count_and_displacement_mismatches_are_refused() {
    let mut m = ab_model(false);
    m.quill = Some(quill(&[5, 9], &[4], "A\rB\r", None));
    text_refused(&m, "disagree about the story count");
    let mut q = quill(&[5, 9], &[2, 2], "A\rB\r", None);
    let strs_off = u32_at_test(&q, 0x18 + 8 + 24 * 2 + 16);
    set32(&mut q, strs_off + 4, 12);
    let mut m = ab_model(false);
    m.quill = Some(q);
    text_refused(&m, "displacement");
}

#[test]
fn ab05_overrunning_and_odd_texts_are_refused() {
    let mut m = ab_model(false);
    m.quill = Some(quill(&[5, 9], &[2, 3], "A\rB\r", None));
    text_refused(&m, "run past TEXT");
    let mut m = ab_model(false);
    m.quill = Some(quill_raw(&[5, 9], &[2, 2], &[b'A', 0, 13, 0, b'B', 0, 13, 0, 0], None));
    text_refused(&m, "odd byte count");
    let mut m = ab_model(false);
    m.quill = Some(quill(&[5, 9], &[2, 2], "A\rB\r\r\r", None));
    text_refused(&m, "do not add up");
    let mut m = ab_model(false);
    m.quill = Some(quill(&[5, 9], &[2, 2], "A\rBB", None));
    text_refused(&m, "paragraph mark");
}

#[test]
fn ab06_wide_and_dangling_links_withhold_only_that_segment() {
    let mut m = ab_model(false);
    m.records[1].set = vec![(0x58, 0x0001_0005)];
    let r = read_legacy(&file(&m)).unwrap();
    assert_eq!(r.scope, Scope::Partial);
    assert_eq!(text_of(&r), [Some("B\r".to_string())]);
    assert!(warned(&r, "unsupported width"));
    let mut m = ab_model(false);
    m.records[1].set = vec![(0x58, 77)];
    let r = read_legacy(&file(&m)).unwrap();
    assert!(warned(&r, "does not list"));
    assert_eq!(r.segments.len(), 1);
}

#[test]
fn ab06_a_repeated_link_is_reported_once() {
    let mut m = ab_model(false);
    m.records[2].set = vec![(0x66, 5)];
    let r = read_legacy(&file(&m)).unwrap();
    assert_eq!(r.segments.len(), 1);
    assert!(warned(&r, "more than one record"));
}

#[test]
fn ab07_invalid_utf16_withholds_that_story_without_replacement_characters() {
    let mut text: Vec<u8> = vec![0x00, 0xd8, 13, 0]; // unpaired high surrogate, CR
    text.extend([b'B', 0, 13, 0]);
    let mut m = ab_model(false);
    m.quill = Some(quill_raw(&[5, 9], &[2, 2], &text, None));
    let r = read_legacy(&file(&m)).unwrap();
    assert_eq!(r.scope, Scope::Partial);
    assert_eq!(text_of(&r), [Some("B\r".to_string())]);
    assert!(warned(&r, "not valid UTF-16"));
    assert!(r.segments.iter().all(|s| !s.text.as_deref().unwrap_or("").contains('\u{fffd}')));
}

#[test]
fn ab08_a_positive_probe_in_an_empty_store_is_a_contradiction() {
    let mut m = ab_model(false);
    m.quill = Some(quill(&[], &[], "", None));
    text_refused(&m, "empty but a Contents link");
}

// ---------- C01..C06 ----------

fn c_refused(m: &Model, needle: &str) {
    let r = read_legacy(&file(m)).unwrap_or_else(|e| panic!("file refused: {e}"));
    assert_eq!(r.scope, Scope::TextRefused, "{:?}", r.warnings);
    assert!(r.segments.is_empty() && warned(&r, needle), "{:?}", r.warnings);
}

#[test]
fn c01_buffer_failures() {
    let mut m = c_model();
    m.core = b"Alpha\r\n\x0cBeta\r\nX".to_vec(); // does not end in form feed
    c_refused(&m, "form feed");
}

#[test]
fn c02_two_maps_are_ambiguous() {
    let mut m = c_model();
    let mut second = m.records[3].clone();
    second.id = 0x204;
    m.records.push(second);
    m.records[0].tail = {
        let mut t = array_tail(4, 2, &[]);
        for c in [0x201u16, 0x202, 0x203, 0x204] {
            p16(&mut t, c);
        }
        t
    };
    c_refused(&m, "not exactly one index-26");
}

#[test]
fn c03_an_odd_count_fails_the_even_pair_rule() {
    let mut m = c_model();
    let mut tail = Vec::new();
    p16(&mut tail, 3);
    p16(&mut tail, 3);
    p16(&mut tail, 10);
    tail.extend([0u8; 12]);
    for v in [0u32, 8, 15, 15] {
        p32(&mut tail, v);
    }
    tail.extend(vec![0u8; 30]);
    m.records[3].tail = tail;
    c_refused(&m, "positive and even");
}

#[test]
fn c04_overlapping_ranges_are_refused_whole() {
    let mut m = c_model();
    // Change the second start from 8 to 7: slot 2 is at tail offset 18 + 8.
    let mut t = m.records[3].tail.clone();
    t[18 + 8..18 + 12].copy_from_slice(&7u32.to_le_bytes());
    m.records[3].tail = t;
    c_refused(&m, "positive, adjacent and ascending");
}

#[test]
fn c05_a_bad_target_or_kind_is_refused() {
    let mut m = c_model();
    let mut t = m.records[3].tail.clone();
    let elems = 18 + 4 * 4 + 4;
    t[elems..elems + 2].copy_from_slice(&0x777u16.to_le_bytes());
    m.records[3].tail = t;
    c_refused(&m, "not in the directory");
    let mut m = c_model();
    let mut t = m.records[3].tail.clone();
    t[18 + 16 + 4 + 20 + 2..18 + 16 + 4 + 20 + 4].copy_from_slice(&1u16.to_le_bytes()); // second target kind flipped
    m.records[3].tail = t;
    c_refused(&m, "target index");
}

#[test]
fn c06_high_bytes_are_not_decoded_but_the_other_segment_is() {
    let mut m = c_model();
    m.core[0] = 0x93;
    let r = read_legacy(&file(&m)).unwrap();
    assert_eq!(r.scope, Scope::Partial);
    assert!(r.segments[0].text.is_none() && r.segments[0].encoding_unknown);
    assert_eq!(r.segments[1].text.as_deref(), Some("Beta\r\n\u{c}"));
    assert!(warned(&r, "code page is not established"));
}

// ---------- O01 and bounds ----------

#[test]
fn o01_no_stronger_claim_than_the_gates_allow() {
    let r = read_legacy(&file(&ab_model(false))).unwrap();
    assert!(warned(&r, "units not verified"));
    assert!(warned(&r, "frames, pages, fonts and formatting are not recovered"));
}

#[test]
fn hostile_truncations_never_panic() {
    let good = build_contents(&ab_model(false));
    for cut in (0..good.len()).step_by(7) {
        let _ = read_legacy(&compound(CLSID_AB, &good[..cut], None));
    }
    let c = build_contents(&c_model());
    for cut in (0..c.len()).step_by(11) {
        let _ = read_legacy(&compound(CLSID_C, &c[..cut], None));
    }
    // A single flipped byte anywhere is a refusal or a warning, never a panic.
    for at in (0..good.len()).step_by(5) {
        let mut b = good.clone();
        b[at] ^= 0xff;
        let _ = read_legacy(&compound(CLSID_AB, &b, ab_model(false).quill.as_deref()));
    }
}

// ---------- the pinned real files, when supplied ----------

#[test]
#[ignore = "needs PUB_REAL_CORPUS with the pinned Apache sample files"]
fn real_sample98_and_sample2000_match_the_contract_numbers() {
    let dir = std::env::var("PUB_REAL_CORPUS").expect("set PUB_REAL_CORPUS");
    for (name, family) in [("Sample98.pub", Family::A), ("Sample2000.pub", Family::B)] {
        let bytes = std::fs::read(std::path::Path::new(&dir).join(name)).unwrap();
        let r = read_legacy(&bytes).unwrap();
        assert_eq!(r.family, family, "{name}");
        assert_eq!(r.scope, Scope::Complete, "{name}");
        let units: Vec<usize> = r
            .segments
            .iter()
            .map(|s| match &s.source {
                Source::Story { units, .. } => units.len(),
                Source::Record { .. } => 0,
            })
            .collect();
        assert_eq!(units, [86, 98, 62, 80, 87, 75], "{name}");
        assert_eq!(units.iter().sum::<usize>(), 488);
        let size = r.size.unwrap();
        assert_eq!((size.raw_width, size.raw_height), (7_560_000, 10_692_000), "{name}");
    }
}
