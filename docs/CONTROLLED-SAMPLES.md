<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Controlled samples: a recipe for small Publisher documents

> **For the owner or a volunteer with Publisher.** This file only says what to create. It describes nothing about how `.pub` files are encoded.

## Why these files

[pub-import-status.md](pub-import-status.md) lists numbered lists, custom tab stops, table cell fills, table cell borders and padding as not yet supported, and says the allowed samples do not cover them (`text__*` and `table__*` have no such variants). [warning-triage.md](warning-triage.md) says controlled cell shading and border variants are what is needed for tables, and [SHIP.md](SHIP.md) lists the same two gaps as blocking work. The merged-cell text and unequal-row-height cases come from the table notes and the eight real files with tables that fail to open ([real-file-triage.md](real-file-triage.md)).

The existing pubsmith samples work because each differs from a base file by one feature. This recipe copies that method.

## Rules for every file

1. **One change per file.** A variant differs from its base file in exactly one setting. Do not tidy, resize or retype anything else.
2. **Base next to variant.** Each group below has a base file. Make the base first, save it, then use Save As for each variant so everything else is identical.
3. **Same page every time.** One blank page, same page size and orientation, same margins, same default font and size, in every file.
4. **Export each file.** For every `.pub`, also save a PDF export (PNG is fine if PDF is not offered) with the same base name. Export at default settings and do not edit it.
5. **Write down the settings.** Keep a text file `NOTES.txt` listing, per file, the exact values you typed. Use the values in the tables below so notes stay short.
6. **Plain content.** Use only made-up text such as `Alpha`, `Beta`, `Gamma`. No private or copyrighted content, no images.
7. **Naming.** `group__description.pub`, for example `list__numbered-1.pub`, with the matching `.pdf` or `.png`.
8. **Keep out of the repository.** Do not commit the files. The owner decides where they are stored and shared.

## Group 1: numbered lists

Base `list__base.pub`: one text frame with three paragraphs, `Alpha`, `Beta`, `Gamma`, no list formatting.

| File | The one change | Values |
|---|---|---|
| `list__numbered-1.pub` | Turn on numbering for all three paragraphs | Numbering style `1, 2, 3`, start at 1, separator `.` |
| `list__numbered-start5.pub` | Same, change start number | Start at 5 |
| `list__numbered-letters.pub` | Same as numbered-1, change style | Style `a, b, c` |
| `list__numbered-roman.pub` | Same as numbered-1, change style | Style `i, ii, iii` |

Make each of the last three from `list__numbered-1`, changing only the named setting. Add `list__numbered-mixed.pub`: base text, numbering on for only the middle paragraph.

## Group 2: custom tab stops

Base `tab__base.pub`: one frame, one line `A<Tab>B<Tab>C`, no custom tab stops set.

| File | The one change | Values |
|---|---|---|
| `tab__left-72.pub` | One left tab stop | At 72 pt (1 in) |
| `tab__left-144.pub` | Same, other position | At 144 pt (2 in) |
| `tab__center-144.pub` | One centre tab stop | At 144 pt |
| `tab__right-144.pub` | One right tab stop | At 144 pt |
| `tab__decimal-144.pub` | One decimal tab stop; change the line to `A<Tab>1.50` | At 144 pt |
| `tab__leader-dots.pub` | Left tab at 144 pt with a dot leader | At 144 pt, dots |
| `tab__two-stops.pub` | Two left tab stops | At 72 pt and 180 pt |

If Publisher lists tab positions in inches or centimetres, set the equivalent and note the unit in `NOTES.txt`.

## Group 3: table cell fills

Base `tablefill__base.pub`: a table of 2 columns by 2 rows, default style, no fill, text `A1 B1 / A2 B2`, no borders changes.

| File | The one change | Values |
|---|---|---|
| `tablefill__one-cell-red.pub` | Fill one cell | Cell A1, solid red (255, 0, 0) |
| `tablefill__one-cell-blue.pub` | Same, other colour and cell | Cell B2, solid blue (0, 0, 255) |
| `tablefill__row.pub` | Fill a whole row | Row 1, solid red |
| `tablefill__all.pub` | Fill the whole table | All cells, solid red |
| `tablefill__transparency.pub` | Fill one cell with transparency | Cell A1, red, 50% transparency |

## Group 4: table cell borders and padding

Base `tableborder__base.pub`: the same 2 by 2 table as above, with no borders on any cell.

| File | The one change | Values |
|---|---|---|
| `tableborder__bottom-A1.pub` | One border on one cell | A1 bottom edge only, black, 1 pt, solid |
| `tableborder__bottom-A1-4pt.pub` | Same, change width | 4 pt |
| `tableborder__bottom-A1-blue.pub` | Same as the 1 pt file, change colour | Blue (0, 0, 255) |
| `tableborder__bottom-A1-dash.pub` | Same as the 1 pt file, change style | Dashed |
| `tableborder__left-A1.pub` | Other side | A1 left edge only, black, 1 pt |
| `tableborder__all-cells.pub` | Borders on every cell edge | All sides, black, 1 pt |

Padding uses the table base file with default borders:

| File | The one change | Values |
|---|---|---|
| `tablepad__base.pub` | 2 by 2 table with default cell margins | Leave margins at default; write the default values in `NOTES.txt` |
| `tablepad__all-10.pub` | Cell margins on all cells | 10 pt on all four sides |
| `tablepad__left-20.pub` | Left margin only, one cell | Cell A1, left 20 pt, others default |

## Group 5: merged-cell text

Base `tablemerge__base.pub`: a 3 column by 2 row table, no merges, text `A1 B1 C1 / A2 B2 C2`.

| File | The one change | Values |
|---|---|---|
| `tablemerge__across.pub` | Merge two cells in a row | Merge A1 and B1; the merged cell contains `A1 B1` (keep both words, one paragraph each) |
| `tablemerge__across-text-one.pub` | Same merge, text only in the first cell before merging | A1 contains `Merged`, B1 empty |
| `tablemerge__down.pub` | Merge two cells in a column | Merge C1 and C2; the merged cell contains `C1 C2` as two paragraphs |
| `tablemerge__text-after.pub` | Same as across, but text typed after merging | Merge A1 and B1 first, then type `Merged` |

These show where text in a merged cell is stored when it can't be placed with certainty (see pub-import-status.md, Tables).

## Group 6: unequal row heights

Base `tablerows__base.pub`: a 3 column by 3 row table with equal rows (default sizing), one word per cell.

| File | The one change | Values |
|---|---|---|
| `tablerows__row1-tall.pub` | Make one row taller by dragging or by setting height | Row 1 height 60 pt; others default |
| `tablerows__row2-short.pub` | Make one row shorter | Row 2 as short as Publisher allows; write the value down |
| `tablerows__all-different.pub` | Three different heights | Row 1 30 pt, row 2 60 pt, row 3 90 pt |
| `tablerows__text-grows.pub` | Make a row grow from its text, no manual height | Row 2, cell B2 holds five lines of text (`one` to `five`, one per paragraph) |

The 3 by 2 table is the shape shared by the eight failing real-user files, so also add `tablerows__3x2-base.pub` (3 columns by 2 rows, equal rows) and `tablerows__3x2-row1-tall.pub` (row 1 at 60 pt).

## What to send back

- All `.pub` files and their PDF or PNG exports, in one folder, plus `NOTES.txt`.
- The Publisher version and the operating system used.
- Anything that was not possible (an option missing, a different unit). Say so rather than guessing.

## Order, if time is short

1. Group 4 and group 3 (tables lose shading and rules today).
2. Group 2 and group 1 (a numbered order of service and tab-aligned lists are common in bulletins).
3. Groups 5 and 6.
