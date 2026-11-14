// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! Gate tests on files this module writes byte by byte (docs/pub-format/legacy-minimal-reader.md,
//! "Synthetic fixture specification"). Nothing here is Publisher-made, and passing them confirms the
//! reader's guards, not the format. One mutation per case.
use super::*;

use crate::fixtures::*;

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
    assert_eq!(r.segments[0].source, Source::Story { id: 5, units: 0..2, records: vec![0x201] });
    assert_eq!(r.segments[1].source, Source::Story { id: 9, units: 2..4, records: vec![0x202] });
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
    assert_eq!(r.segments[0].source, Source::Record { id: 0x201, map: 0x203, bytes: 0..8 });
    assert_eq!(r.segments[1].source, Source::Record { id: 0x202, map: 0x203, bytes: 8..15 });
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
    refused(&compound(CLSID_AB, &c, m.quill.as_deref()), is_unsupported, "terminator");
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
    c[first + 3] = 0xb3; // H stays inside its span but no longer matches the table entry
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
    assert!(warned(&r, "profile") || warned(&r, "extent") || warned(&r, "predicted end") || warned(&r, "expected class"));
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

// ---------- hardening from the review ----------

#[test]
fn g03_an_agreeing_but_unlisted_b_terminator_is_unsupported() {
    let m = ab_model(true);
    let mut c = build_contents(&m);
    // The first entry's terminator is the word after its non-terminating tail word (0x26 + 8).
    set16(&mut c, 0x26 + 8, 0x0200);
    refused(&compound(CLSID_AB, &c, m.quill.as_deref()), is_unsupported, "not one of the observed terminators");
}

#[test]
fn g08_exact_profiles_are_enforced_not_only_the_equation() {
    // A link record with a different (but self-consistent) H is not used as a story link.
    let mut m = ab_model(false);
    m.records[1] = link_rec(0x201, 8, 9, 0x70, 0x58, 5, 2);
    let r = read_legacy(&file(&m)).unwrap();
    assert_eq!(r.scope, Scope::Partial);
    assert_eq!(r.segments.len(), 1);
    assert!(warned(&r, "not the expected class 9 record with H=118"), "{:?}", r.warnings);
    // A root with another width is withheld from the size only.
    let mut m = ab_model(false);
    m.records[0].tail = {
        let mut t = array_tail(2, 4, &[]);
        t.extend([0u8; 4]);
        p16(&mut t, 0x201);
        p16(&mut t, 0x202);
        t
    };
    let r = read_legacy(&file(&m)).unwrap();
    assert!(r.size.is_none() && warned(&r, "profile check"));
}

#[test]
fn ab07_every_linking_record_is_kept_as_provenance() {
    let mut m = ab_model(false);
    m.records[2].set = vec![(0x66, 5)];
    let r = read_legacy(&file(&m)).unwrap();
    assert_eq!(r.segments.len(), 1);
    assert_eq!(r.segments[0].source, Source::Story { id: 5, units: 0..2, records: vec![0x201, 0x202] });
}

#[test]
fn ab08_zero_length_sections_and_failed_candidates_do_not_make_an_empty_store() {
    // A present but zero-length TEXT is not the absent-section empty profile.
    let mut q = quill(&[], &[], "", None);
    set16(&mut q, 0x18 + 2, 2);
    q[0x18 + 8 + 24 + 2..0x18 + 8 + 24 + 6].copy_from_slice(b"TEXT");
    let mut m = ab_model(false);
    m.quill = Some(q);
    text_refused(&m, "unsupported empty-store variant");
    // A candidate record that fails its profile prevents the known-empty claim.
    let mut m = ab_model(false);
    m.quill = Some(quill(&[], &[], "", None));
    m.records[1].set = vec![];
    m.records[2].set = vec![];
    m.records[2].tail = array_tail(3, 8, &[]);
    let r = read_legacy(&file(&m)).unwrap();
    assert_eq!(r.scope, Scope::Partial);
    assert!(warned(&r, "not asserted to be empty"));
}

#[test]
fn positive_catalog_ids_without_a_link_are_counted_not_claimed() {
    let mut m = ab_model(false);
    m.quill = Some(quill(&[5, 9, 11], &[2, 2, 2], "A\rB\rC\r", None));
    let r = read_legacy(&file(&m)).unwrap();
    assert_eq!(r.segments.len(), 2);
    assert!(warned(&r, "1 story id(s)") && warned(&r, "no validated Contents link"));
}

#[test]
fn c_with_an_extra_quill_store_warns_and_still_recovers() {
    let mut m = c_model();
    m.quill = Some(quill(&[5], &[2], "A\r", None));
    let r = read_legacy(&file(&m)).unwrap();
    assert_eq!(r.scope, Scope::Complete);
    assert!(warned(&r, "unexpected Quill text store"));
}

#[test]
fn a_long_parent_chain_is_validated_in_linear_work() {
    // 3000 records chained to one root: the memoised walk finishes at once.
    let mut m = ab_model(false);
    let mut prev = 0x200u16;
    for k in 0..3000u16 {
        let id = 0x300 + k;
        m.records.push(R { id, parent: prev, index: 3, class: 0, h: 4, set: vec![], tail: vec![] });
        prev = id;
    }
    let r = read_legacy(&file(&m)).unwrap();
    assert_eq!(r.segments.len(), 2);
}

#[test]
fn unvalidated_known_records_are_counted_as_opaque() {
    let mut m = ab_model(false);
    m.records.push(R { id: 0x300, parent: 0x200, index: 33, class: 0, h: 4, set: vec![], tail: vec![9, 9, 9] });
    let r = read_legacy(&file(&m)).unwrap();
    assert_eq!(r.segments.len(), 2);
    assert!(warned(&r, "stay opaque"));
}

#[test]
fn a_quill_directory_pointer_outside_the_stream_is_refused() {
    let mut m = ab_model(false);
    m.quill = Some(quill(&[5, 9], &[2, 2], "A\rB\r", Some(0xffff_fff0)));
    text_refused(&m, "outside the stream");
}
