// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! Confirmed table geometry and cell-text boundaries; see docs/pub-format/tables.md.
use crate::{
    Builder, PubError, Result, SpreadRef, pt,
    read::{Prop, count, number, slice, u32_at},
};
use designcraft_doc::{CellStroke, ItemId, ParaFormat, Story, StoryId, Table};
use designcraft_geom::Rect;
use std::ops::Range;

const MAX_CELLS: usize = 10_000;
fn bad(message: &str) -> PubError {
    PubError::Malformed(format!("table: {message}"))
}
#[derive(Clone, Debug)]
pub(crate) struct CellInfo {
    pub row: usize,
    pub last_row: usize,
    pub col: usize,
    pub last_col: usize,
}
#[derive(Clone)]
pub(crate) struct TableInfo {
    /// Native heights derived from consecutive stored row endings.
    pub rows: Vec<u32>,
    pub columns: Vec<u32>,
    pub cells: Vec<CellInfo>,
    pub cell_record: u32,
    /// Largest absolute ending-versus-size-sum difference in this table, for diagnostics only.
    pub row_ending_difference: u32,
}
impl TableInfo {
    pub(crate) fn parse(p: &[Prop<'_>]) -> Result<Option<Self>> {
        let (Some(rows), Some(cols), Some(cell_record)) = (number(p, 0x66), number(p, 0x67), number(p, 0x6b)) else { return Ok(None) };
        let rows = count(rows, 256)?;
        let cols = count(cols, 256)?;
        if rows == 0 || cols == 0 || rows * cols > MAX_CELLS {
            return Err(bad("grid exceeds limits"));
        }
        let Some(dimensions) = p.iter().find(|p| p.key == 0x6d && p.kind == 0x90) else { return Ok(None) };
        let entries = dimensions.children()?;
        if entries.len() != rows + cols {
            return Err(bad("dimension count mismatch"));
        }
        let mut sizes = Vec::new();
        let mut sum = 0u32;
        let mut previous_ending = 0u32;
        let mut row_ending_difference = 0u32;
        for (i, entry) in entries.iter().enumerate() {
            if i == cols {
                sum = 0;
            }
            let fields = entry.children()?;
            let size = number(&fields, 2).ok_or_else(|| bad("missing dimension"))?;
            if size == 0 || size > 197_510_400 {
                return Err(bad("dimension out of range"));
            }
            sum = sum.checked_add(size).ok_or_else(|| bad("dimension overflow"))?;
            let ending = number(&fields, 1).ok_or_else(|| bad("missing cumulative dimension"))?;
            let layout_size = if i < cols {
                if ending != sum {
                    return Err(bad("inconsistent cumulative dimension"));
                }
                size
            } else {
                if ending > i32::MAX as u32 || ending <= previous_ending {
                    return Err(bad("row endings must be positive and strictly increasing"));
                }
                let height = ending.checked_sub(previous_ending).ok_or_else(|| bad("row dimension overflow"))?;
                if height > 197_510_400 {
                    return Err(bad("row dimension out of range"));
                }
                previous_ending = ending;
                row_ending_difference = row_ending_difference.max(ending.abs_diff(sum));
                height
            };
            if i + 1 == cols && number(p, 0x68) != Some(sum) || i + 1 == cols + rows && number(p, 0x69) != Some(sum) {
                return Err(bad("size total mismatch"));
            }
            sizes.push(layout_size);
        }
        Ok(Some(Self {
            columns: sizes.get(..cols).ok_or_else(|| bad("missing columns"))?.to_vec(),
            rows: sizes.get(cols..).ok_or_else(|| bad("missing rows"))?.to_vec(),
            cells: Vec::new(),
            cell_record,
            row_ending_difference,
        }))
    }
    pub(crate) fn validate(&self) -> Result<()> {
        let mut occupied = vec![false; self.rows.len() * self.columns.len()];
        for c in &self.cells {
            if c.row > c.last_row || c.col > c.last_col || c.last_row >= self.rows.len() || c.last_col >= self.columns.len() {
                return Err(bad("invalid cell rectangle"));
            }
            for r in c.row..=c.last_row {
                for col in c.col..=c.last_col {
                    let slot = occupied.get_mut(r * self.columns.len() + col).ok_or_else(|| bad("cell out of range"))?;
                    if *slot {
                        return Err(bad("overlapping cells"));
                    }
                    *slot = true;
                }
            }
        }
        if occupied.iter().any(|v| !v) {
            return Err(bad("incomplete cell grid"));
        }
        Ok(())
    }
}
pub(crate) fn cells(p: &[Prop<'_>]) -> Result<Vec<CellInfo>> {
    let n = count(number(p, 1).ok_or_else(|| bad("missing cell count"))?, MAX_CELLS)?;
    let Some(array) = p.iter().find(|p| p.key == 2 && p.kind == 0xa0) else { return Err(bad("missing cell list")) };
    let entries = array.children()?;
    if entries.len() != n {
        return Err(bad("cell count mismatch"));
    }
    entries
        .iter()
        .map(|entry| {
            let p = entry.children()?;
            Ok(CellInfo {
                row: number(&p, 1).unwrap_or(0) as usize,
                last_row: number(&p, 2).unwrap_or(0) as usize,
                col: number(&p, 3).unwrap_or(0) as usize,
                last_col: number(&p, 4).unwrap_or(0) as usize,
            })
        })
        .collect()
}
pub(crate) fn text_ranges(data: &[u8], units: &[u16]) -> Result<Vec<Range<usize>>> {
    let n = count(u32_at(data, 0)?, MAX_CELLS - 1)? + 1;
    slice(data, 12, n * 4)?;
    let mut ranges = Vec::new();
    let mut start = 0;
    for i in 0..n {
        let end = u32_at(data, 12 + 4 * i)? as usize;
        if end < start || end > units.len() {
            return Err(bad("invalid cell text boundary"));
        }
        let last = i + 1 == n;
        if last && end != units.len() || !last && units.get(end) != Some(&13) {
            return Err(bad("cell text separator mismatch"));
        }
        let z = if last && end > start && units.get(end - 1) == Some(&13) { end - 1 } else { end };
        ranges.push(start..z);
        start = end + usize::from(!last);
    }
    Ok(ranges)
}

// Match text_frame's normalization while retaining the position of every UTF-16 unit.
fn byte_positions(units: &[u16]) -> Vec<usize> {
    let mut at = Vec::with_capacity(units.len() + 1);
    let mut byte = 0;
    for decoded in char::decode_utf16(units.iter().copied()) {
        let n = decoded.as_ref().map_or(1, |c| c.len_utf16());
        at.extend(std::iter::repeat_n(byte, n));
        byte += match decoded {
            Ok('\r' | '\t') => 1,
            Ok('\u{b}') => designcraft_doc::FORCED_LINE_BREAK.len_utf8(),
            Ok(c) if (c as u32) < 0x20 => 0,
            Ok(c) => c.len_utf8(),
            Err(_) => char::REPLACEMENT_CHARACTER.len_utf8(),
        };
    }
    at.push(byte);
    at
}
fn cell_story(source: &Story, range: Range<usize>, work: &mut usize) -> Result<Story> {
    let text = source.text.get(range.clone()).ok_or_else(|| bad("cell text is not a character range"))?;
    let para = source.paras.get(source.para_at(range.start)).cloned().unwrap_or_default();
    let mut out = Story::with_text(StoryId(0), text, ParaFormat { table: None, ..para });
    if let Some(first) = out.chars.first_mut() {
        first.format = source.char_format_at(range.start).clone();
    }
    for (r, fmt) in source.runs() {
        *work += 1;
        if *work > 1_000_000 {
            return Err(bad("cell formatting exceeds work limit"));
        }
        let a = r.start.max(range.start);
        let z = r.end.min(range.end);
        if a < z {
            out.format_chars(a - range.start..z - range.start, |f| *f = fmt.clone());
        }
    }
    for (i, r) in source.para_ranges().into_iter().enumerate() {
        *work += 1;
        if *work > 1_000_000 {
            return Err(bad("cell formatting exceeds work limit"));
        }
        let a = r.start.max(range.start);
        let z = r.end.min(range.end);
        if a < z
            && let Some(fmt) = source.paras.get(i)
        {
            out.format_paras(a - range.start..z - range.start, |f| *f = fmt.clone());
        }
    }
    Ok(out)
}
impl Builder<'_> {
    pub(crate) fn table_frame(&mut self, r: SpreadRef, rect: Rect, tid: u32, info: &TableInfo) -> Result<ItemId> {
        self.table_cells = self.table_cells.saturating_add(info.rows.len() * info.columns.len());
        self.table_units = self.table_units.saturating_add(self.q.stories.get(&tid).map_or(0, |s| s.units.len()));
        if self.table_cells > 100_000 || self.table_units > 16_000_000 {
            return Err(bad("aggregate table size exceeds limit"));
        }
        if info.row_ending_difference != 0 {
            self.warnings.push(format!(
                "Publisher table used stored endings for row positions; largest difference from size sums: {} EMU; cause unknown.",
                info.row_ending_difference
            ));
        }
        let id = self.text_frame(r, rect, tid)?;
        let sid = self.doc.item(id).and_then(|i| i.text_frame()).map(|t| t.story).ok_or_else(|| bad("missing host frame"))?;
        let source = self.doc.story(sid).cloned().ok_or_else(|| bad("missing host story"))?;
        let qs = self.q.stories.get(&tid).ok_or_else(|| bad("missing text story"))?;
        let ranges = qs.cell_ranges.as_ref().filter(|r| r.len() == info.cells.len());
        let merged = info.cells.iter().any(|c| c.row != c.last_row || c.col != c.last_col);
        let can_assign = ranges.is_some_and(|r| !merged || r.iter().all(Range::is_empty));
        let mut table = Table::new(self.doc.alloc(), info.rows.len(), info.columns.len(), 0, 0, rect.width());
        table.options.border = CellStroke::none();
        for (row, height) in table.rows.iter_mut().zip(&info.rows) {
            row.height = pt(f64::from(*height));
        }
        for (col, width) in table.columns.iter_mut().zip(&info.columns) {
            col.width = pt(f64::from(*width));
        }
        for cell in &mut table.cells {
            cell.strokes = std::array::from_fn(|_| CellStroke::none());
            cell.border_overrides = [true; 4];
        }
        let bytes = byte_positions(&qs.units);
        for (i, region) in info.cells.iter().enumerate() {
            let cell = table.cell_mut(region.row, region.col).ok_or_else(|| bad("missing native cell"))?;
            cell.row_span = (region.last_row - region.row + 1) as u32;
            cell.col_span = (region.last_col - region.col + 1) as u32;
            if can_assign && let Some(range) = ranges.and_then(|r| r.get(i)) {
                let a = *bytes.get(range.start).ok_or_else(|| bad("missing text start"))?;
                let z = *bytes.get(range.end).ok_or_else(|| bad("missing text end"))?;
                cell.text = cell_story(&source, a..z, &mut self.table_work)?;
            }
        }
        let mut host = if can_assign { Story::new(sid) } else { source };
        host.insert_table(0, table);
        *self.doc.story_mut(sid).ok_or_else(|| bad("missing native story"))? = host;
        if !can_assign {
            self.warnings.push("Table cell-text association is unconfirmed or missing; the original text was retained after the table.".into());
        }
        self.warnings
            .push("Publisher table cell fills, borders, padding and other cell formatting are not decoded; unpainted cells were used.".into());
        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        contents::{Contents, PageInfo, ShapeInfo},
        escher::{Drawing, EShape, Emu},
        quill::{QStory, Quill},
        read::properties,
    };
    fn grid() -> TableInfo {
        TableInfo {
            rows: vec![457_200; 2],
            columns: vec![1_828_800; 2],
            cell_record: 260,
            row_ending_difference: 0,
            cells: vec![
                CellInfo { row: 0, last_row: 0, col: 0, last_col: 0 },
                CellInfo { row: 0, last_row: 0, col: 1, last_col: 1 },
                CellInfo { row: 1, last_row: 1, col: 0, last_col: 0 },
                CellInfo { row: 1, last_row: 1, col: 1, last_col: 1 },
            ],
        }
    }
    fn table_data(ends: &[u32]) -> Vec<u8> {
        [
            (ends.len() as u32 - 1).to_le_bytes().as_slice(),
            &[0; 4],
            &0xff00u32.to_le_bytes(),
            &ends.iter().flat_map(|u| u.to_le_bytes()).collect::<Vec<_>>(),
        ]
        .concat()
    }
    fn import(info: TableInfo, text: &str, ranges: Option<Vec<Range<usize>>>) -> crate::Imported {
        let mut c = Contents { width_emu: 7_772_400, height_emu: 10_058_400, ..Default::default() };
        c.pages.push(PageInfo { seq: 257, shapes: vec![258], ..Default::default() });
        c.shapes.insert(258, ShapeInfo { text_id: Some(3), is_table: true, table: Some(info), ..Default::default() });
        let mut q = Quill::default();
        q.default_char.bold = Some(true);
        q.stories.insert(3, QStory { units: text.encode_utf16().collect(), cell_ranges: ranges, ..Default::default() });
        let dr = Drawing {
            shapes: vec![EShape {
                seq: Some(258),
                kind: 202,
                anchor: Emu { l: -2_000_000.0, t: -2_000_000.0, r: 1_657_600.0, b: -1_085_600.0 },
                ..Default::default()
            }],
            ..Default::default()
        };
        let mut b = Builder::new(&c, &q, &dr).unwrap();
        b.build().unwrap();
        crate::Imported { document: b.doc, warnings: b.warnings }
    }
    fn dimension_properties(ends: &[u32], sizes: &[u32], width: u32, height: u32) -> Vec<u8> {
        fn number(key: u8, value: u32) -> Vec<u8> {
            [vec![key, 0x20], value.to_le_bytes().to_vec()].concat()
        }
        fn list(key: u8, kind: u8, data: &[u8]) -> Vec<u8> {
            [vec![key, kind], ((data.len() + 4) as u32).to_le_bytes().to_vec(), data.to_vec()].concat()
        }
        let mut entries = Vec::new();
        for (end, size) in [(1_782_000, 1_782_000), (3_564_000, 1_782_000)].into_iter().chain(ends.iter().copied().zip(sizes.iter().copied())) {
            entries.extend(list(0, 0x88, &[number(1, end), number(2, size)].concat()));
        }
        [number(0x66, sizes.len() as u32), number(0x67, 2), number(0x68, width), number(0x69, height), number(0x6b, 260), list(0x6d, 0x90, &entries)]
            .concat()
    }
    #[test]
    fn row_endings_define_positions_and_warn_once_with_largest_difference() {
        let bytes = dimension_properties(&[219_188, 438_376, 657_564], &[216_000; 3], 3_564_000, 648_000);
        let mut info = TableInfo::parse(&properties(&bytes).unwrap()).unwrap().unwrap();
        assert_eq!(info.rows, [219_188; 3]);
        assert_eq!(info.row_ending_difference, 9564);
        info.cells = (0..3).flat_map(|row| (0..2).map(move |col| CellInfo { row, last_row: row, col, last_col: col })).collect();
        info.validate().unwrap();
        let imp = import(info, "\r\r\r\r\r\r", None);
        let table = imp.document.stories.values().next().unwrap().tables.values().next().unwrap();
        let mut position = 0.0;
        for (row, ending) in table.rows.iter().zip([219_188.0, 438_376.0, 657_564.0]) {
            position += row.height;
            assert!((position - pt(ending)).abs() < 1e-9);
        }
        let warnings: Vec<_> = imp.warnings.iter().filter(|w| w.contains("stored endings for row positions")).collect();
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0], "Publisher table used stored endings for row positions; largest difference from size sums: 9564 EMU; cause unknown.");
        assert!(warnings[0].len() < 160);
    }
    #[test]
    fn row_endings_preserve_rejection_of_invalid_grids_totals_and_overflow() {
        for ends in [[219_188, 219_188, 657_564], [219_188, 219_187, 657_564], [0, 438_376, 657_564], [u32::MAX, 438_376, 657_564]] {
            let bytes = dimension_properties(&ends, &[216_000; 3], 3_564_000, 648_000);
            assert!(TableInfo::parse(&properties(&bytes).unwrap()).is_err());
        }
        for bytes in [
            dimension_properties(&[219_188, 438_376, 657_564], &[216_000; 3], 3_564_000, 648_001),
            dimension_properties(&[219_188, 438_376, 657_564], &[216_000; 3], 3_564_001, 648_000),
            dimension_properties(&[219_188, 438_376], &[216_000; 3], 3_564_000, 648_000),
            dimension_properties(&[219_188, 438_376, 657_564], &[0; 3], 3_564_000, 0),
        ] {
            assert!(TableInfo::parse(&properties(&bytes).unwrap()).is_err());
        }
        let mut bytes = dimension_properties(&[219_188, 438_376, 657_564], &[216_000; 3], 3_564_000, 648_000);
        let marker = [vec![1, 0x20], 1_782_000u32.to_le_bytes().to_vec()].concat();
        let at = bytes.windows(marker.len()).position(|w| w == marker).unwrap() + 2;
        bytes[at..at + 4].copy_from_slice(&1_782_001u32.to_le_bytes());
        assert!(TableInfo::parse(&properties(&bytes).unwrap()).is_err());
        let bytes = dimension_properties(&(1..=32).collect::<Vec<_>>(), &[197_510_400; 32], 3_564_000, 0);
        let error = TableInfo::parse(&properties(&bytes).unwrap()).err().unwrap();
        assert!(error.to_string().contains("dimension overflow"));
    }
    #[test]
    fn imports_native_grid_text_formatting_and_warns_for_unconfirmed_cell_paint() {
        let text = "A1\rB1\rA2\rB2\r";
        let units: Vec<_> = text.encode_utf16().collect();
        let ranges = text_ranges(&table_data(&[2, 5, 8, 12]), &units).unwrap();
        let imp = import(grid(), text, Some(ranges));
        let host = imp.document.stories.values().next().unwrap();
        host.check().unwrap();
        assert!(!host.text.contains("A1"));
        let t = host.tables.values().next().unwrap();
        assert_eq!((t.nrows(), t.ncols()), (2, 2));
        assert_eq!(t.rows[0].height, 36.0);
        assert_eq!(t.columns[0].width, 144.0);
        for (cell, text) in t.cells.iter().zip(["A1", "B1", "A2", "B2"]) {
            assert_eq!(cell.text.text, text);
            cell.text.check().unwrap();
            assert!(!cell.has_fill());
            assert!(cell.strokes.iter().all(|s| !s.is_visible()));
            assert_eq!(cell.text.char_format_at(0).over.font_style.as_deref(), Some("Bold"));
        }
        assert!(!t.options.border.is_visible());
        assert!(imp.warnings.iter().any(|w| w.contains("cell fills, borders")));
    }
    #[test]
    fn merged_regions_use_coordinates_and_keep_covered_cells_empty() {
        let mut info = grid();
        info.cells = vec![info.cells[2].clone(), info.cells[3].clone(), CellInfo { row: 0, last_row: 0, col: 0, last_col: 1 }];
        info.validate().unwrap();
        let ranges = text_ranges(&table_data(&[0, 1, 3]), &[13, 13, 13]).unwrap();
        let imp = import(info, "\r\r\r", Some(ranges));
        let t = imp.document.stories.values().next().unwrap().tables.values().next().unwrap();
        assert_eq!(t.cell(0, 0).unwrap().col_span, 2);
        assert_eq!(t.cell(0, 1).unwrap().text.text, "");
        // Same rectangle representation can safely express a row span; no file claims it.
        let mut vertical = grid();
        vertical.cells = vec![CellInfo { row: 0, last_row: 1, col: 0, last_col: 0 }, vertical.cells[1].clone(), vertical.cells[3].clone()];
        vertical.validate().unwrap();
    }
    #[test]
    fn ambiguous_cell_text_stays_after_the_native_table() {
        let imp = import(grid(), "Never lose this\r", None);
        let host = imp.document.stories.values().next().unwrap();
        assert!(host.text.contains("Never lose this"));
        assert_eq!(host.tables.len(), 1);
        assert!(imp.warnings.iter().any(|w| w.contains("original text was retained")));
        host.check().unwrap();
    }
    #[test]
    fn cell_boundaries_support_internal_paragraphs_and_reject_hostile_offsets() {
        let units: Vec<_> = "A\rB\rC\r".encode_utf16().collect();
        assert_eq!(text_ranges(&table_data(&[3, 6]), &units).unwrap(), vec![0..3, 4..5]);
        for ends in [&[u32::MAX][..], &[1, 2][..], &[4, 3][..], &[0, 6][..]] {
            assert!(text_ranges(&table_data(ends), &units).is_err());
        }
        assert!(text_ranges(&u32::MAX.to_le_bytes(), &units).is_err());
        assert!(text_ranges(&[0; 12], &units).is_err());
        assert_eq!(byte_positions(&[0xd83d, 0xde00, 13, 0xb, 9, 1]), vec![0, 0, 4, 5, 8, 9, 9]);
    }
    #[test]
    fn rejects_overlapping_missing_and_out_of_bounds_cells() {
        let mut info = grid();
        info.cells.push(info.cells[0].clone());
        assert!(info.validate().is_err());
        info.cells = vec![];
        assert!(info.validate().is_err());
        info.cells.push(CellInfo { row: 0, last_row: usize::MAX, col: 0, last_col: 0 });
        assert!(info.validate().is_err());
        let mut bytes = vec![0x66, 0x22];
        bytes.extend(u32::MAX.to_le_bytes());
        bytes.extend([0x67, 0x22, 2, 0, 0, 0, 0x6b, 0x70, 4, 1, 0, 0]);
        assert!(TableInfo::parse(&properties(&bytes).unwrap()).is_err());
    }
}
