// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! The colour scheme record (layout index 71) of Publisher 97, 98 and 2000 files, and the standard colour
//! palette its numbers index. See docs/pub-format/legacy-minimal-reader.md ("The colour scheme record") and
//! docs/pub-format/office-standard-palette.md for the evidence. Only what those notes support is read.

use crate::LegacyError;
use crate::contents::Layout;

/// The layout index of the colour scheme record.
const SCHEME_INDEX: u16 = 71;
/// The first of the eight four-byte entries inside the record, and how many there are.
const ENTRIES_AT: usize = 0xa0;
const ENTRY_COUNT: usize = 8;
/// The number of greys that come before the hexagon in the numbering of the palette.
const GREYS: u32 = 17;

/// One entry of the scheme: an exact colour, or a number into the standard palette.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SchemeColor {
    /// Bytes `RR GG BB 20`.
    Literal([u8; 3]),
    /// A position in the standard palette (greys first, then the hexagon row by row from the top).
    Index(u32),
    /// Anything else.
    Other,
}

impl SchemeColor {
    /// The colour, when it is an exact colour or a number whose swatch is read with confidence.
    pub fn rgb(self) -> Option<[u8; 3]> {
        match self {
            SchemeColor::Literal(rgb) => Some(rgb),
            SchemeColor::Index(n) => standard_color(n),
            SchemeColor::Other => None,
        }
    }
}

/// The document's colour scheme: its name and its eight entries, in the order they are stored.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scheme {
    pub name: String,
    pub entries: [SchemeColor; ENTRY_COUNT],
}

/// The first six rows of the hexagon (57 swatches), top to bottom, left to right. Read from a public screenshot of
/// the Standard tab of the Office 97-2003 Colors dialog (docs/pub-format/office-standard-palette.md); the later rows
/// were read less reliably and are left out.
const HEXAGON_TOP: [u32; 57] = [
    0x003366, 0x336699, 0x3366CC, 0x003399, 0x000099, 0x0000CC, 0x000066, //
    0x006666, 0x006699, 0x0099CC, 0x0066CC, 0x0033CC, 0x0000FF, 0x3333FF, 0x333399, //
    0x008080, 0x009999, 0x33CCCC, 0x00CCFF, 0x0099FF, 0x0066FF, 0x3366FF, 0x3333CC, 0x666699, //
    0x339966, 0x00CC99, 0x00FFCC, 0x00FFFF, 0x33CCFF, 0x3399FF, 0x6699FF, 0x6666FF, 0x6600FF, 0x6600CC, //
    0x339933, 0x00CC66, 0x00FF99, 0x66FFCC, 0x66FFFF, 0x66CCFF, 0x99CCFF, 0x9999FF, 0x9966FF, 0x9933FF, 0x9900FF, //
    0x006600, 0x00CC00, 0x00FF00, 0x66FF99, 0x99FFCC, 0xCCFFFF, 0xCCFFFF, 0xCCCCFF, 0xCC99FF, 0xCC66FF, 0xCC00FF, 0x9900CC,
];

/// The colour of a standard palette number, for the swatches that are read with confidence: the numbers 17 to 73
/// (the first six rows of the hexagon). The 17 greys and the later rows give `None`.
pub fn standard_color(index: u32) -> Option<[u8; 3]> {
    let at = usize::try_from(index.checked_sub(GREYS)?).ok()?;
    let v = *HEXAGON_TOP.get(at)?;
    Some([(v >> 16) as u8, (v >> 8) as u8, v as u8])
}

/// Read the colour scheme record of a legacy file, when it has one.
pub fn read_scheme(bytes: &[u8]) -> Result<Option<Scheme>, LegacyError> {
    let container = crate::open(bytes)?;
    let layout = Layout::parse(&container.contents, &container.clsid)?;
    let Some(rec) = layout.records.iter().find(|r| r.index == SCHEME_INDEX) else { return Ok(None) };
    let end = rec.e.min(layout.data.len());
    let Some(body) = layout.data.get(rec.r..end) else { return Ok(None) };
    if body.len() < ENTRIES_AT + 4 * ENTRY_COUNT {
        return Ok(None);
    }
    let name_units = body.get(0x0e..ENTRIES_AT).unwrap_or_default().as_chunks::<2>().0.iter().map(|c| u16::from_le_bytes(*c)).take_while(|u| *u != 0);
    let name: String = char::decode_utf16(name_units).map(|c| c.unwrap_or('?')).collect();
    let mut entries = [SchemeColor::Other; ENTRY_COUNT];
    for (i, slot) in entries.iter_mut().enumerate() {
        let at = ENTRIES_AT + 4 * i;
        let Some(b) = body.get(at..at + 4) else { break };
        let v = u32::from_le_bytes([b[0], b[1], b[2], b[3]]);
        *slot = if v >> 24 == 0x20 {
            SchemeColor::Literal([b[0], b[1], b[2]])
        } else if v < 0x100 {
            SchemeColor::Index(v)
        } else {
            SchemeColor::Other
        };
    }
    Ok(Some(Scheme { name, entries }))
}

#[cfg(test)]
mod tests {
    use super::{SchemeColor, standard_color};

    #[test]
    fn numbers_before_the_hexagon_and_after_its_sixth_row_are_not_resolved() {
        assert_eq!(standard_color(0), None);
        assert_eq!(standard_color(16), None);
        assert_eq!(standard_color(17 + 57), None);
    }

    #[test]
    fn the_scheme_numbers_of_the_donated_blue_schemes_resolve_to_blues() {
        // Bluebird 31, 37 and 30; Dark Blue 50; Waterfall 18 (docs/pub-format/office-standard-palette.md).
        assert_eq!(standard_color(31), Some([0x33, 0x33, 0x99]));
        assert_eq!(standard_color(37), Some([0x00, 0x66, 0xFF]));
        assert_eq!(standard_color(30), Some([0x33, 0x33, 0xFF]));
        assert_eq!(standard_color(50), Some([0x66, 0x00, 0xCC]));
        assert_eq!(standard_color(18), Some([0x33, 0x66, 0x99]));
        assert_eq!(SchemeColor::Literal([1, 2, 3]).rgb(), Some([1, 2, 3]));
        assert_eq!(SchemeColor::Other.rgb(), None);
    }
}

/// The Office 97-2003 standard colour grid (the 40 colours of the fill-colour drop-down), in reading order:
/// five rows of eight. Names and order are public (the chart "Microsoft Office colour charts" lists them); the
/// values are the long-published standard values. OBSERVATION: a direct fill number below 40 in a frame record is
/// a position in this grid: in `IHEC Postcards` the number 26 is Yellow, and the strips are yellow on the stored
/// preview. The same number in `Hotels` and `Super Bowl` (27 and 5) is not yet checked against a preview.
const STANDARD_GRID: [u32; 40] = [
    0x000000, 0x993300, 0x333300, 0x003300, 0x003366, 0x000080, 0x333399, 0x333333, //
    0x800000, 0xFF6600, 0x808000, 0x008000, 0x008080, 0x0000FF, 0x666699, 0x808080, //
    0xFF0000, 0xFF9900, 0x99CC00, 0x339966, 0x33CCCC, 0x3366FF, 0x800080, 0x969696, //
    0xFF00FF, 0xFFCC00, 0xFFFF00, 0x00FF00, 0x00FFFF, 0x00CCFF, 0x993366, 0xC0C0C0, //
    0xFF99CC, 0xFFCC99, 0xFFFF99, 0xCCFFCC, 0xCCFFFF, 0x99CCFF, 0xCC99FF, 0xFFFFFF,
];

/// The colour of a direct fill number (0 to 39) of the standard grid.
pub fn grid_color(n: u32) -> Option<[u8; 3]> {
    let v = *STANDARD_GRID.get(usize::try_from(n).ok()?)?;
    Some([(v >> 16) as u8, (v >> 8) as u8, v as u8])
}
