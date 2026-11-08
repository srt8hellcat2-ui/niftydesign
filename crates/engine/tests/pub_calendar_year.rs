// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! Table > Calendar > Set Year on a calendar imported from a Publisher file: the cells have
//! separate stories made by the importer. The file is written byte by byte by this test.
#![cfg(not(target_arch = "wasm32"))]

#[path = "../../pub/tests/support/mod.rs"]
mod support;

use designcraft_engine::Session;
use serde_json::{Value, json};
use support::{Object, compound, contents, emu, escher, list, number, shape};

const NAMES: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

/// Weekday of the first of the month (0 = Sunday), Sakamoto's formula.
fn first_weekday(y: i32, m: i32) -> usize {
    const T: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let y = if m < 3 { y - 1 } else { y };
    ((y + y / 4 - y / 100 + y / 400 + T[(m - 1) as usize] + 1) % 7) as usize
}
fn month_days(y: i32, m: i32) -> usize {
    match m {
        2 if (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}
/// The 7-column grid of a Sunday-first month: the weekday row, then the week rows.
fn grid(y: i32, m: i32) -> Vec<Vec<String>> {
    let off = first_weekday(y, m);
    let weeks = (off + month_days(y, m)).div_ceil(7);
    let mut rows = vec![["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"].map(String::from).to_vec()];
    for w in 0..weeks {
        rows.push(
            (0..7).map(|c| (w * 7 + c).checked_sub(off).filter(|d| *d < month_days(y, m)).map_or(String::new(), |d| (d + 1).to_string())).collect(),
        );
    }
    rows
}

fn units(s: &str) -> Vec<u8> {
    s.encode_utf16().flat_map(u16::to_le_bytes).collect()
}

/// The text of a table story (cells closed by CR) and its TCD section.
fn story_and_tcd(cells: &[String]) -> (String, Vec<u8>) {
    let story: String = cells.iter().map(|c| format!("{c}\r")).collect();
    let mut at = 0u32;
    let mut ends = Vec::new();
    for c in cells {
        at += c.encode_utf16().count() as u32;
        ends.push(at);
        at += 1;
    }
    *ends.last_mut().unwrap() = at;
    let mut d = (cells.len() as u32 - 1).to_le_bytes().to_vec();
    d.extend(0u32.to_le_bytes());
    d.extend(0xff00u32.to_le_bytes());
    d.extend(ends.iter().flat_map(|e| e.to_le_bytes()));
    (story, d)
}

fn quill(heads: &[String], tables: &[Vec<String>]) -> Vec<u8> {
    let mut stories: Vec<(u32, String)> = heads.iter().enumerate().map(|(i, h)| (21 + 2 * i as u32, format!("{h}\r"))).collect();
    let mut tcds = Vec::new();
    for (i, cells) in tables.iter().enumerate() {
        let (s, t) = story_and_tcd(cells);
        stories.push((41 + i as u32, s));
        tcds.push((stories.len() as u16 - 1, t));
    }
    let text: Vec<u8> = stories.iter().flat_map(|(_, s)| units(s)).collect();
    let mut syid = vec![0u8; 4];
    syid.extend((stories.len() as u32).to_le_bytes());
    syid.extend(stories.iter().flat_map(|(id, _)| id.to_le_bytes()));
    let mut strs = (stories.len() as u32).to_le_bytes().to_vec();
    strs.extend(8u32.to_le_bytes());
    strs.extend([0u8; 4]);
    strs.extend(stories.iter().flat_map(|(_, s)| (s.encode_utf16().count() as u32).to_le_bytes()));
    let mut q = vec![0u8; 512];
    q[..8].copy_from_slice(b"CHNKINK ");
    let entries: Vec<(&[u8; 4], u16, Vec<u8>)> = [(b"TEXT", 0, text), (b"SYID", 0, syid), (b"STRS", 0, strs)]
        .into_iter()
        .chain(tcds.into_iter().map(|(slot, t)| (b"TCD ", slot, t)))
        .collect();
    // One directory block of up to 8 entries, the rest in a second block.
    let first = entries.len().min(4);
    q[26..28].copy_from_slice(&(first as u16).to_le_bytes());
    let put = |q: &mut Vec<u8>, at: usize, tag: &[u8; 4], slot: u16, data: &[u8]| {
        let off = q.len() as u32;
        q[at + 2..at + 6].copy_from_slice(tag);
        q[at + 6..at + 8].copy_from_slice(&slot.to_le_bytes());
        q[at + 16..at + 20].copy_from_slice(&off.to_le_bytes());
        q[at + 20..at + 24].copy_from_slice(&(data.len() as u32).to_le_bytes());
        q.extend(data);
    };
    for (i, (tag, slot, data)) in entries.iter().take(first).enumerate() {
        put(&mut q, 32 + 24 * i, tag, *slot, data);
    }
    let rest = &entries[first..];
    let second = q.len();
    q.extend(vec![0u8; 8 + 24 * rest.len()]);
    q[28..32].copy_from_slice(&(second as u32).to_le_bytes());
    q[second + 2..second + 4].copy_from_slice(&(rest.len() as u16).to_le_bytes());
    q[second + 4..second + 8].copy_from_slice(&u32::MAX.to_le_bytes());
    for (i, (tag, slot, data)) in rest.iter().enumerate() {
        put(&mut q, second + 8 + 24 * i, tag, *slot, data);
    }
    q
}

fn dimension(ending: u32, size: u32) -> Vec<u8> {
    list(0, 0x88, &[number(1, ending), number(2, size)].concat())
}
fn table(story: u32, rows: u32, cell_list_seq: u32) -> Object {
    let (w, h) = (emu(60) as u32, emu(24) as u32);
    let mut dims: Vec<u8> = (1..=7).flat_map(|i| dimension(w * i, w)).collect();
    dims.extend((1..=rows).flat_map(|i| dimension(h * i, h)));
    let p = [
        number(0x27, story),
        number(0x66, rows),
        number(0x67, 7),
        number(0x68, w * 7),
        number(0x69, h * rows),
        number(0x6b, cell_list_seq),
        list(0x6d, 0x90, &dims),
    ]
    .concat();
    Object { record_type: 0x10, properties: p }
}
fn cell_list(rows: u32) -> Object {
    let entries: Vec<u8> =
        (0..rows).flat_map(|r| (0..7).flat_map(move |c| list(0, 0x88, &[number(1, r), number(2, r), number(3, c), number(4, c)].concat()))).collect();
    Object { record_type: 0x63, properties: [vec![1, 0x18], (rows as u16 * 7).to_le_bytes().to_vec(), list(2, 0xa0, &entries)].concat() }
}
fn heading(story: u32) -> Object {
    Object { record_type: 0x01, properties: [number(0x27, story), number(0xaa, emu(200) as u32), number(0xab, emu(24) as u32)].concat() }
}

/// May and June 2026 as an imported two-table Publisher calendar.
fn calendar_file() -> Vec<u8> {
    let months = [5, 6];
    let grids: Vec<Vec<Vec<String>>> = months.iter().map(|m| grid(2026, *m)).collect();
    let heads: Vec<String> = months.iter().map(|m| format!("{} 2026", NAMES[*m as usize - 1])).collect();
    let cells: Vec<Vec<String>> = grids.iter().map(|g| g.iter().flatten().cloned().collect()).collect();
    let objects = [
        heading(21),
        heading(23),
        table(41, grids[0].len() as u32, 261),
        cell_list(grids[0].len() as u32),
        table(42, grids[1].len() as u32, 263),
        cell_list(grids[1].len() as u32),
    ];
    let shapes = [
        shape(202, 1025, &[], [36, 20, 200, 24], 258),
        shape(202, 1026, &[], [36, 420, 200, 24], 259),
        shape(202, 1027, &[], [36, 50, 420, 24 * grids[0].len() as i32], 260),
        shape(202, 1028, &[], [36, 450, 420, 24 * grids[1].len() as i32], 262),
    ];
    compound(&contents(emu(612) as u32, emu(792) as u32, &objects), &quill(&heads, &cells), &escher(None, &shapes))
}

fn tables(s: &Session) -> Vec<Vec<Vec<String>>> {
    let doc = &s.doc().unwrap().doc;
    let mut v: Vec<_> = doc
        .stories
        .values()
        .flat_map(|st| st.tables.values())
        .map(|t| (0..t.nrows()).map(|r| (0..t.ncols()).map(|c| t.cell(r, c).unwrap().text.text.clone()).collect::<Vec<_>>()).collect::<Vec<_>>())
        .collect();
    v.sort_by_key(|t| t.len());
    v
}
fn headings(s: &Session) -> Vec<String> {
    let mut v: Vec<String> =
        s.doc().unwrap().doc.stories.values().filter(|st| st.tables.is_empty() && !st.text.is_empty()).map(|st| st.text.clone()).collect();
    v.sort();
    v
}
fn snapshot(s: &Session) -> Value {
    serde_json::to_value(&*s.doc().unwrap().doc).unwrap()
}

#[test]
fn an_imported_calendar_gets_a_new_year_in_one_undo_step() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("calendar.pub");
    std::fs::write(&path, calendar_file()).unwrap();
    let mut s = Session::new();
    s.execute("file.open", &json!({"path": path})).unwrap();
    // The importer put each cell's text in its own story.
    let mut before = tables(&s);
    before.sort_by_key(|t| t.len());
    assert_eq!(before, {
        let mut g = vec![grid(2026, 5), grid(2026, 6)];
        g.sort_by_key(Vec::len);
        g
    });
    assert_eq!(headings(&s), ["June 2026", "May 2026"]);
    let snap = snapshot(&s);
    let steps = s.doc().unwrap().history.undo.len();

    let r = s.execute("table.setCalendarYear", &json!({"year": 2027})).unwrap();
    assert_eq!(r["months"], 2, "{r}");
    let mut want = vec![grid(2027, 5), grid(2027, 6)];
    want.sort_by_key(Vec::len);
    assert_eq!(tables(&s), want, "May 2027 has 7 rows and June 2027 has 6");
    assert_eq!(headings(&s), ["June 2027", "May 2027"]);
    assert_eq!(s.doc().unwrap().history.undo.len(), steps + 1);

    s.execute("edit.undo", &json!({})).unwrap();
    assert_eq!(snapshot(&s), snap);
}
