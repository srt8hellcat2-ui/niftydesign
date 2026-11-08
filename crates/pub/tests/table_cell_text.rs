// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! Cell text association for several tables (docs/pub-format/tables.md, "Cell text association").
//! Every file here is written byte by byte by this test; no Publisher-made file is involved.

mod support;

use designcraft_pub::import_pub;
use support::{Object, compound, contents, emu, escher, list, number, shape};

const CR: &str = "\r";

#[derive(Clone)]
struct Case {
    /// Cell texts of the 2x3 table (story 41) and the 2x2 table (story 42).
    a: Vec<&'static str>,
    b: Vec<&'static str>,
    /// Edits that break one documented check.
    tcd_a_extra_bytes: usize,
    tcd_a_count_field: Option<u32>,
    tcd_a_boundary: Option<(usize, u32)>,
    a_separator_replaced_at: Option<usize>,
    duplicate_id: bool,
    merge_first_two_cells_of_a: bool,
}

impl Case {
    fn good() -> Self {
        Self {
            a: vec!["Sun", "", "Tue", "1", "22", ""],
            b: vec!["A1", "B1", "", "D\u{e9}"],
            tcd_a_extra_bytes: 0,
            tcd_a_count_field: None,
            tcd_a_boundary: None,
            a_separator_replaced_at: None,
            duplicate_id: false,
            merge_first_two_cells_of_a: false,
        }
    }
}

fn story_of(cells: &[&str]) -> String {
    cells.iter().map(|c| format!("{c}{CR}")).collect()
}

/// The `TCD ` section: header, then the unit position of each cell's closing separator; the
/// last boundary is the story length.
fn tcd_of(cells: &[&str], count_field: Option<u32>, extra: usize, patch: Option<(usize, u32)>) -> Vec<u8> {
    let mut ends = Vec::new();
    let mut at = 0u32;
    for c in cells {
        at += c.encode_utf16().count() as u32;
        ends.push(at);
        at += 1;
    }
    if let Some(last) = ends.last_mut() {
        *last = at;
    }
    if let Some((i, v)) = patch {
        ends[i] = v;
    }
    let mut d = count_field.unwrap_or(cells.len() as u32 - 1).to_le_bytes().to_vec();
    d.extend(0u32.to_le_bytes());
    d.extend(0xff00u32.to_le_bytes());
    d.extend(ends.iter().flat_map(|e| e.to_le_bytes()));
    d.extend(std::iter::repeat_n(0u8, extra));
    d
}

fn units(s: &str) -> Vec<u8> {
    s.encode_utf16().flat_map(u16::to_le_bytes).collect()
}

/// A Quill store: six short stories, then the two table stories; TEXT, SYID, STRS and the first
/// TCD entry are in the first directory block, the second TCD entry is in a later block.
fn quill(case: &Case) -> Vec<u8> {
    let heads = ["May", "June", "July", "Aug", "Sept", "Oct"].map(|h| format!("{h}{CR}"));
    let mut story_a = story_of(&case.a);
    if let Some(i) = case.a_separator_replaced_at {
        let mut u: Vec<u16> = story_a.encode_utf16().collect();
        u[i] = u16::from(b'x');
        story_a = String::from_utf16_lossy(&u);
    }
    let story_b = story_of(&case.b);
    let mut ids = [21u32, 23, 25, 27, 29, 31, 41, 42];
    if case.duplicate_id {
        ids[7] = 41;
    }
    let mut lens: Vec<u32> = heads.iter().map(|h| h.encode_utf16().count() as u32).collect();
    lens.extend([story_a.encode_utf16().count() as u32, story_b.encode_utf16().count() as u32]);
    let text: Vec<u8> = heads.iter().flat_map(|h| units(h)).chain(units(&story_a)).chain(units(&story_b)).collect();
    let mut syid = vec![0u8; 4];
    syid.extend((ids.len() as u32).to_le_bytes());
    syid.extend(ids.iter().flat_map(|i| i.to_le_bytes()));
    let mut strs = (lens.len() as u32).to_le_bytes().to_vec();
    strs.extend(8u32.to_le_bytes());
    strs.extend([0u8; 4]);
    strs.extend(lens.iter().flat_map(|l| l.to_le_bytes()));
    let tcd_a = tcd_of(&case.a, case.tcd_a_count_field, case.tcd_a_extra_bytes, case.tcd_a_boundary);
    let tcd_b = tcd_of(&case.b, None, 0, None);

    let mut q = vec![0u8; 512];
    q[..8].copy_from_slice(b"CHNKINK ");
    let put = |q: &mut Vec<u8>, entry_at: usize, tag: &[u8; 4], slot: u16, data: &[u8]| {
        q[entry_at + 2..entry_at + 6].copy_from_slice(tag);
        q[entry_at + 6..entry_at + 8].copy_from_slice(&slot.to_le_bytes());
        let at = q.len() as u32;
        q[entry_at + 16..entry_at + 20].copy_from_slice(&at.to_le_bytes());
        q[entry_at + 20..entry_at + 24].copy_from_slice(&(data.len() as u32).to_le_bytes());
        q.extend(data);
    };
    // First directory block at 0x18: four entries, then a pointer to a second block.
    q[26..28].copy_from_slice(&4u16.to_le_bytes());
    for (i, (tag, data)) in [(b"TEXT", &text), (b"SYID", &syid), (b"STRS", &strs)].into_iter().enumerate() {
        put(&mut q, 32 + 24 * i, tag, 0, data);
    }
    put(&mut q, 32 + 24 * 3, b"TCD ", 6, &tcd_a);
    let second = q.len();
    q.extend(vec![0u8; 8 + 24]);
    q[28..32].copy_from_slice(&(second as u32).to_le_bytes());
    q[second + 2..second + 4].copy_from_slice(&1u16.to_le_bytes());
    q[second + 4..second + 8].copy_from_slice(&u32::MAX.to_le_bytes());
    put(&mut q, second + 8, b"TCD ", 7, &tcd_b);
    q
}

fn dimension(ending: u32, size: u32) -> Vec<u8> {
    list(0, 0x88, &[number(1, ending), number(2, size)].concat())
}

/// A type-0x10 table object: story, grid, sizes and the sequence number of its cell list.
fn table(story: u32, rows: u32, cols: u32, cell_list_seq: u32) -> Object {
    let (w, h) = (emu(100) as u32, emu(30) as u32);
    let mut dims: Vec<u8> = (1..=cols).flat_map(|i| dimension(w * i, w)).collect();
    dims.extend((1..=rows).flat_map(|i| dimension(h * i, h)));
    let properties = [
        number(0x27, story),
        number(0x66, rows),
        number(0x67, cols),
        number(0x68, w * cols),
        number(0x69, h * rows),
        number(0x6b, cell_list_seq),
        list(0x6d, 0x90, &dims),
    ]
    .concat();
    Object { record_type: 0x10, properties }
}

/// A type-0x63 cell list with the given inclusive (first row, last row, first column, last column).
fn cell_list(cells: &[[u32; 4]]) -> Object {
    let entries: Vec<u8> =
        cells.iter().flat_map(|c| list(0, 0x88, &[number(1, c[0]), number(2, c[1]), number(3, c[2]), number(4, c[3])].concat())).collect();
    let properties = [vec![1, 0x18], (cells.len() as u16).to_le_bytes().to_vec(), list(2, 0xa0, &entries)].concat();
    Object { record_type: 0x63, properties }
}

fn row_major(rows: u32, cols: u32) -> Vec<[u32; 4]> {
    (0..rows).flat_map(|r| (0..cols).map(move |c| [r, r, c, c])).collect()
}

fn heading(story: u32) -> Object {
    Object { record_type: 0x01, properties: [number(0x27, story), number(0xaa, emu(100) as u32), number(0xab, emu(20) as u32)].concat() }
}

fn file(case: &Case) -> Vec<u8> {
    // Sequence numbers: 258, 259 headings; 260 table A, 261 its cells; 262 table B, 263 its cells.
    let a_cells = if case.merge_first_two_cells_of_a {
        let mut cells = vec![[0, 0, 0, 1]];
        cells.extend(row_major(2, 3).into_iter().skip(2));
        cells
    } else {
        row_major(2, 3)
    };
    let objects = [heading(21), heading(23), table(41, 2, 3, 261), cell_list(&a_cells), table(42, 2, 2, 263), cell_list(&row_major(2, 2))];
    let shapes = [
        shape(202, 1025, &[], [36, 36, 100, 20], 258),
        shape(202, 1026, &[], [200, 36, 100, 20], 259),
        shape(202, 1027, &[], [36, 100, 300, 60], 260),
        shape(202, 1028, &[], [36, 300, 200, 60], 262),
    ];
    let mut q = quill(case);
    if case.merge_first_two_cells_of_a {
        // The merged table has five cells, so its text store needs five boundaries to be a fair test.
        let merged: Vec<&str> = case.a.iter().copied().skip(1).collect();
        let mut five = case.clone();
        five.a = merged;
        five.merge_first_two_cells_of_a = false;
        q = quill(&five);
    }
    compound(&contents(emu(612) as u32, emu(792) as u32, &objects), &q, &escher(None, &shapes))
}

struct Seen {
    a: Vec<String>,
    b: Vec<String>,
    host_text: String,
    warnings: Vec<String>,
}

fn import(case: &Case) -> Seen {
    let imported = import_pub(&file(case)).unwrap();
    let mut seen = Seen { a: Vec::new(), b: Vec::new(), host_text: String::new(), warnings: imported.warnings };
    for story in imported.document.stories.values() {
        story.check().unwrap();
        seen.host_text.push_str(&story.text);
        for table in story.tables.values() {
            let texts = table.cells.iter().map(|c| c.text.text.clone()).collect();
            match table.ncols() {
                3 => seen.a = texts,
                2 => seen.b = texts,
                _ => panic!("unexpected table"),
            }
        }
    }
    seen
}

fn warned(seen: &Seen, needle: &str) -> bool {
    seen.warnings.iter().any(|w| w.contains(needle))
}

const LOSS: &str = "table text across multiple stories/TCD sections is not decoded";
const KEPT: &str = "was not placed in its cells";

#[test]
fn two_tables_receive_their_own_cell_slices() {
    let seen = import(&Case::good());
    assert_eq!(seen.a, ["Sun", "", "Tue", "1", "22", ""]);
    assert_eq!(seen.b, ["A1", "B1", "", "D\u{e9}"]);
    // Only the anchors stay in the hosts; the separate heading stories are kept.
    assert!(!seen.host_text.contains("Tue") && !seen.host_text.contains("A1"), "{:?}", seen.host_text);
    assert!(seen.host_text.contains("May") && seen.host_text.contains("June"));
    assert!(!warned(&seen, LOSS) && !warned(&seen, KEPT) && !warned(&seen, "unconfirmed or missing"), "{:?}", seen.warnings);
}

/// The table keeps its old behaviour: empty cells and the story text after the table.
fn assert_fallback_for_a(seen: &Seen, reason: &str) {
    assert!(seen.a.iter().all(String::is_empty), "{:?}", seen.a);
    assert!(seen.host_text.contains("Tue"), "the text stays after the table");
    assert!(warned(seen, KEPT) && warned(seen, reason), "{:?}", seen.warnings);
    assert!(warned(seen, LOSS), "the generic loss warning stays: {:?}", seen.warnings);
    assert!(warned(seen, "unconfirmed or missing"));
}

#[test]
fn a_tcd_cell_count_that_disagrees_is_not_placed() {
    let seen = import(&Case { tcd_a_count_field: Some(4), ..Case::good() });
    assert_fallback_for_a(&seen, "cell count differs");
    assert_eq!(seen.b, ["A1", "B1", "", "D\u{e9}"], "the other table is still placed");
}

#[test]
fn a_repeated_story_id_places_nothing() {
    let seen = import(&Case { duplicate_id: true, ..Case::good() });
    assert!(seen.a.iter().chain(&seen.b).all(String::is_empty));
    assert!(warned(&seen, "repeats an id"));
    assert!(warned(&seen, LOSS));
}

#[test]
fn a_boundary_past_the_story_is_not_placed() {
    let seen = import(&Case { tcd_a_boundary: Some((2, 4000)), ..Case::good() });
    assert_fallback_for_a(&seen, "do not fit the story text");
    assert_eq!(seen.b.len(), 4);
    assert_eq!(seen.b[1], "B1");
}

#[test]
fn a_separator_that_is_not_a_paragraph_mark_is_not_placed() {
    // Unit 3 is the separator after "Sun".
    let seen = import(&Case { a_separator_replaced_at: Some(3), ..Case::good() });
    assert_fallback_for_a(&seen, "do not fit the story text");
    assert_eq!(seen.b[0], "A1");
}

#[test]
fn a_merged_cell_table_is_not_placed_by_ordinal() {
    let seen = import(&Case { merge_first_two_cells_of_a: true, ..Case::good() });
    assert_fallback_for_a(&seen, "merged or not in row-major order");
    assert_eq!(seen.b[3], "D\u{e9}");
}

#[test]
fn a_tcd_entry_of_the_wrong_size_is_not_placed() {
    let seen = import(&Case { tcd_a_extra_bytes: 4, ..Case::good() });
    assert_fallback_for_a(&seen, "wrong size");
    assert_eq!(seen.b[0], "A1");
}
