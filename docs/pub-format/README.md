<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Microsoft Publisher (`.pub`) format notes

Plain-English facts about the parts of the `.pub` format that DesignCraft's importer reads from the **`Contents`** stream and the **Quill text store**. They are written so that someone who has never seen any `.pub` reader can write one from these notes, the public Microsoft specifications and sample files. There is no code here, on purpose (see `SOURCES.md`).

| File | Covers |
|---|---|
| `property-lists.md` | The key/kind/data encoding that both streams use. |
| `contents-stream.md` | The `Contents` stream: header, record directory, document, pages and masters, objects, palette, colour references. |
| `tables.md` | Confirmed table dimensions, cell rectangles/merges and TCD cell text; unconfirmed fills/borders and safety limits. |
| `text-store.md` | The Quill text store: section directory, text, story ids and lengths, fonts, text colours, default styles, character and paragraph runs. |
| `spot-checks.md` | Three sample files decoded by hand with these rules. |
| `SOURCES.md` | Everything read to write these notes (the clean-room record). |

## How a `.pub` file fits together

A `.pub` file is an OLE2 compound file (**MS-CFB**). Streams used:

| Stream | What it holds | Described in |
|---|---|---|
| `Contents` | Publication structure: page size, page order, masters, which objects are on which page, per-object text links, the palette. | `contents-stream.md` |
| `Quill/QuillSub/CONTENTS` | All text and its formatting. | `text-store.md` |
| `Escher/EscherStm` | Drawing records: every object's position, size, rotation, shape type, fill and line. | **MS-ODRAW** (OfficeArt), structures OfficeArtDgContainer, OfficeArtSpgrContainer, OfficeArtSpContainer, OfficeArtFSP, OfficeArtFOPT and the property sets of section 2.3, OfficeArtClientAnchor, OfficeArtChildAnchor and OfficeArtClientData (record type 0xF011; its contents are host-defined, see `contents-stream.md` §6). Not repeated here. |
| `Escher/EscherDelayStm` | Picture data (blips) referenced from the drawing records. | **MS-ODRAW** structures OfficeArtBStoreContainer, OfficeArtFBSE and the OfficeArtBlip records. |

Also present but not read: `CompObj`, `Quill/QuillSub/CompObj`, `Envelope`, `Internal`, `SummaryInformation`, `DocumentSummaryInformation` (the last two are standard OLE property sets, **MS-OLEPS**).

How the pieces link:

1. The `Contents` **record directory** gives every record a **sequence number** (its position in the directory).
2. The **document record** lists the page records by sequence number; each **page record** lists its objects by sequence number and may name a master page by sequence number.
3. Each drawing record in `Escher/EscherStm` carries its object's sequence number in its client data (id 0x6801), which joins the drawing to the page lists.
4. An **object record** in `Contents` may name a **story id**; the Quill store's story-id list maps that id to a slice of the text.
5. Character runs refer to fonts (Quill `FONT` list) and text colours (Quill `PL  ` list); a text colour may point into the `Contents` **palette**.

## Versions

- Publisher **2002 to 2021** write the layout described here. The `Contents` stream starts with `E8 AC 2C 00`.
- Publisher **97, 98 and 2000** start `Contents` with `E8 AC 22 00` and use a different layout, not described here. The importer refuses them with a message suggesting re-saving in a newer Publisher.
- All 142 pubsmith samples were made with Publisher 2021 and all start `E8 AC 2C 00 E8 03 0A 00`. No sample from 2002–2019 was available, so differences between those versions are **not known**; the existing importer treats them all alike, and the one Govdocs1 file (age unknown) imports with it.
- `hello-nifty.pub` was written by another program, not Publisher. It is minimal: no placeholder pages, only two style sheets with no entries, and a one-font list. A reader must accept such files.

## Glossary (names used in these notes)

| Name | Meaning |
|---|---|
| **property** | One key byte, one kind byte, and data whose size the kind fixes. |
| **property list** | Properties laid end to end. |
| **length-prefixed list** | A 32-bit length that counts itself, then a property list. |
| **nested list** | A property whose data is a length-prefixed list. |
| **array** | A nested list whose entries all use key 0x00. |
| **record** | A length-prefixed list in `Contents`, found through the record directory. |
| **record directory** | The array in the `Contents` trailer that lists every record's type and offset. |
| **sequence number** | A record's position in the record directory; used for all references. |
| **record type** | The number in a directory entry saying what a record is (page, object, …). |
| **document record**, **page record**, **object record**, **palette record** | Records of type 0x44, 0x43, 0x01/0x10/0x20/0x30, 0x5C. |
| **master label** | Text property of a page record; non-empty means master page. |
| **placeholders** | The four entries Publisher appends to the page list, including the scratch area. |
| **story** | One continuous text, shown in one text box or a chain of linked ones. |
| **story id** | The number that links an object record to a story. |
| **section**, **section tag**, **section directory** | The parts of the Quill stream, their 4-letter names, and the list that locates them. |
| **formatting page** | A 512-byte `FDPC` or `FDPP` section listing runs. |
| **run** | A span of text sharing one format record. |
| **format record** | The length-prefixed list of character or paragraph keys for a run, or for a default style. |
| **colour reference** | A 32-bit colour value whose top byte says whether it's RGB or a palette index. |
| **EMU** | English Metric Unit; 914 400 per inch, 12 700 per point. |

## Coverage checklist

Every structure the existing `Contents` and Quill readers use, and where it is described.

**`Contents` stream**
- [x] Format signature, and the 97/98/2000 signature: `contents-stream.md` §1
- [x] Trailer offset at 0x1A: §1
- [x] Trailer record and finding the directory in it: §2
- [x] Directory entries (type, offset), sequence-number counting, empty slots: §3
- [x] Record-type numbers for document, page, object, table, other object, group, palette: §3
- [x] Document record: page list, page width and height: §4
- [x] Page record: objects, master used, master label: §5
- [x] Telling masters, ordinary pages and placeholders apart; scratch-area warning; blank-publication fallback: §5
- [x] Object record: story id, chain position, vertical alignment, table flag: §6
- [x] Palette record and its entries: §7
- [x] RGB byte order: §7, §8
- [x] Property encoding, kinds and sizes, nested lists, text properties, error rules: `property-lists.md`

**Quill text store**
- [x] Section directory, its chaining and loop guard: `text-store.md` §2
- [x] `TEXT`: §3
- [x] `SYID` story ids: §4
- [x] `STRS` story lengths, pairing with ids, clamping: §4
- [x] `FONT` list: §5
- [x] `PL  ` text colours: §6
- [x] Second `STSH`: default character and paragraph formats: §7
- [x] `FDPC` / `FDPP` formatting pages, run ends, record offsets, run-to-story mapping: §8
- [x] Character keys: bold, italic, underline, size, font, colour (two forms): §9
- [x] Paragraph keys: alignment, space before/after, first/left/right indent: §10

**Not covered here on purpose:** the compound-file container (MS-CFB) and the Escher drawing and picture streams (MS-ODRAW), which come from public specifications.

**Known gaps:** chain position (object key 0x28) and the plain-index colour key (character key 0x2E) are described from the existing code only; no sample contains them. Line spacing, bullets, small/all caps, superscript and tracking are listed under "seen but not read" with sample values but not decoded.
