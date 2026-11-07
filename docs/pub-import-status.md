<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Publisher import: what works

Status of opening Microsoft Publisher (`.pub`) files in DesignCraft, feature by feature.
Refreshed 2026-10-19 from the docs merged on `pub-import` (through `9019750`): the task results in `TASKS.md`, `REPORTS.md`, `warning-triage.md` and `real-file-triage.md`. Scores are against the 142 pubsmith sample files (`0ccdf27`), each made in Publisher to show one feature. Nothing was re-run for this refresh.

Update 2026-10-19 (through `d4067bc`): two further Codex changes merged (`table-rows-cautious`, `table-rows-drawn-height`). Both were documentation only and changed no code, scores or warnings. The drawn table frame height matches the last stored row ending exactly; the cause of the eight table failures is still unexplained and they still fail (`REPORTS.md`).

Update 2026-10-20 (through `6ae953d`): Items C1 to C6 and C10 to C12 have merged. From REPORTS.md and ROADMAP.md: the eight table failures are fixed by using stored row endings, so the real-file check is now 0 OK / 19 with warnings / 2 refused / 3 failed (the 3 are deliberately damaged files), 31 pages, 280 objects. Controlled files: 79 OK / 63 with warnings / 0 failed; corpus 142/142 imports, 152/152 objects, 111/111 scored shape/text positions, 52/52 texts, 111/111 styles, unchanged. Some 0x39, 0x48 and 0x3C warnings are now quiet. C7 to C9 (freeform curves, preset shapes, WordArt) are blocked: the public specification was unreachable. A Windows CLI workflow and a web test build workflow exist but have not been run on GitHub. Sections below that still say 11 of 24 or eight table failures predate this note. Nothing was re-run for this refresh.

Update 2026-10-23 (T5–T8): `60685.pub` retains all four source text stories; the source Education section is blank. Its top strip retains all 38 drawing children and measured bounds; Publisher appearance is not independently verified. Page-number/date expansion still needs verified field-definition/association notes; no values were guessed. Print retention has no established cause in our submission code; preflight's broad RGB-swatch rule is inherited unchanged from the initial export. No behavior changed for these four investigations. Independent per-item before/after runs preserve corpus **142/142, 152/152, 111/111 scored positions, 52/52, 111/111** and real **9 OK with notes / 10 warnings / 2 refused / 3 damaged**, **31 pages / 280 objects**. Details and missing evidence are in [real-file-triage.md](real-file-triage.md).

Update 2026-10-23 (T4): successful `.pub` opens show a non-modal converted-copy notice in the app and in the CLI report (`notice` column), independently of warnings/notes. It says that appearance can differ, features can be lost/simplified, not every loss is warned, and to compare the original. All 19 successful real imports carry it. Before/after corpus **142/142, 152/152, 111/111 scored positions, 52/52, 111/111** and real **9 OK with notes / 10 warnings / 2 refused / 3 damaged** are unchanged. Engine/UI-harness and CLI tests cover it; no Windows desktop capture was made.

Update 2026-10-23 (T3): generic unidentified-formatting diagnostics are now **notes**, with all field IDs retained; confirmed losses stay warnings. Corpus unchanged: **142/142, 152/152, 111/111 scored positions, 52/52, 111/111**. Real-file status: **0 OK / 19 warnings / 2 refused / 3 damaged → 9 OK with notes / 10 warnings / 2 refused / 3 damaged**. Controlled status: **79 OK / 63 warnings → 130 OK / 12 warnings**. These status changes reflect severity only; all 19 successful real native documents are unchanged except timestamps. “OK” does not establish visual fidelity.

Update 2026-10-23 (T1, `codex/importer-improvements`): before/after corpus scores are unchanged: **142/142 imports, 152/152 objects, 111/111 scored shape/text positions within 1 pt, 52/52 exact texts, 111/111 styles**. The 24-file batch remains **0 OK / 19 warnings / 2 refused / 3 damaged**, **31 pages / 280 objects**. The eight Apache Sample* tables now contain **6 nonempty cells and 6 composed glyph lines each (48 cell texts total)**, with zero duplicate fallback lines and no table overset in the checked composition. All 48 expected cell texts are also present in exported PDF page 2. This verifies content and rendering, not parity with Publisher. The two table-text-association warnings affect **8 files before / 0 after**; cell-formatting and row-ending warnings remain. Cell fills/borders remain unverified. Earlier figures below are historical.

- **Works**: comes over and stays editable; checked by the corpus test or a unit test.
- **Partly**: comes over, but looks different or loses something.
- **Not yet**: lost or replaced by something simpler. DesignCraft lists most of these as warnings when you open the file; *silent* means it doesn't.

## Files and pages

| Feature | Status | Notes |
|---|---|---|
| Publisher 2002–2021 files | Works | All 142 samples open. Of 24 files from real users of other programs (Apache POI set), 11 open, 11 fail and 2 are refused as old; see "Real-file check" below. |
| Publisher 97/98/2000 files | Not yet | Refused with a clear message (`Sample98.pub`, `Sample2000.pub`). |
| Page size, orientation, page count | Works | `pages__*` (letter, landscape, 5.75×4.75 in, 1–3 pages). |
| Master pages | Works | Become DesignCraft parent pages, with their shapes and text. |
| Objects beside the page (scratch area) | Not yet | Left out, warned (`pages__size-5.75x4.75`). |

## Text

| Feature | Status | Notes |
|---|---|---|
| Text in frames, edited text, long text | Works | 52/52 texts match Publisher's exactly. |
| Linked text frames | Works | One story threaded through the frames in order. |
| Font, size, bold, italic, underline, colour | Works | 111/111 checked styles match, including runs inside a paragraph. |
| Alignment (left, centre, right, justified) | Works | `text__align-*`. |
| Indents, space before/after | Works | `text__indent-*`, `text__space-*`; explicit zero values tested. |
| Frame margins, vertical alignment, rotated frames | Works | `text__margins-10`, `text__vanchor-*`, `text__rotated-270`. |
| Text frame fill and border | Works | `text__frame-fill-line`. |
| Line spacing | Works | Exact, multiple and inherited defaults (`text__ls-*`, all five). |
| Bullets | Works | Marker character and font, hanging indent (`text__bullets`). The marker keeps Publisher's Symbol-font character; glyph look may differ. |
| Tab characters | Works | Literal tabs survive. |
| Numbered lists, custom tab stops | Not yet | No sample or format notes yet; unrecognised list data is warned. Task 4.1. |
| All caps, small caps, superscript, letter spacing | Works | Come over as DesignCraft character settings. |
| Subscript, strikethrough | Not yet | No encoding confirmed; not guessed. Unknown character settings are warned by number. |
| Page numbers, dates and other fields | Not yet | Left out of the text, warned (also in `SampleBrochure`, `SampleNewsletter`). |

## Shapes and lines

| Feature | Status | Notes |
|---|---|---|
| Position, size, rotation, flips | Works | All 111 position-checked objects are within 1 pt (`geometry__*`). |
| Rectangles, ovals, rounded rectangles, triangles, diamonds, hexagons, stars, arrows, lines | Works | `shapes__*`. |
| Freeforms (curves, polylines) and less common Office shapes | Partly | Some become rectangles of the same size, warned (`shapes__curve`, `shapes__polyline`, `shapes__heart`, shape type 74). |
| Solid fills, scheme colours, no fill | Works | `fill__*`. |
| Gradient fills | Works | Become gradient swatches; centre and outline shades are approximated with radial gradients. Direction not yet checked against Publisher by eye. |
| Pattern and texture fills | Partly | Their first colour, warned. |
| Fill transparency | Works | The old "opaque" warning is gone from the batch (`fill__transparency-50`); not yet compared with Publisher by eye. |
| Line colour, weight, none | Works | `line__*`. |
| Dashed lines, arrowheads | Works | `line__dash-*`, `shapes__line-arrow`. |
| Shape shadows | Works | Colour, opacity and offset become a DesignCraft drop shadow (`shadow__on`, `shadow__offset`); unrecognised shadow types are warned. WordArt shadows too. |
| Groups | Works | Stay groups, rotation and flips kept (`group__*`). |

## Pictures

| Feature | Status | Notes |
|---|---|---|
| PNG, JPEG, BMP and other bitmaps | Works | `picture__*`: moved, stretched, rotated, flipped, two pictures. |
| Rectangular crops, picture borders | Works | `picture__crop-*`, `picture__border`. |
| Crops to other shapes | Partly | Not supported in general; `picture__oval-mask` has not been checked visually. |
| WMF/EMF clip art | Partly | Vector artwork without its text, bitmaps, arcs and pattern fills. |
| PICT pictures | Not yet | An empty frame keeps the place, warned. |

## Tables and WordArt

| Feature | Status | Notes |
|---|---|---|
| Tables | Partly | Real editable tables: rows, columns, sizes, cell text and merged cells (`table__*`). Cell fills, borders and padding are not decoded and warned; text in merged cells that can't be placed with certainty is kept after the table, warned. Eight real-user files with tables fail (see below). Task 4.2. |
| WordArt | Partly | Editable text with its font, colours, outline and shadow, sized to fit (`wordart__*`). Arches, circles and Publisher's stretching to fill the box are lost; curved ones are warned. |

## Saving and exporting

| Feature | Status | Notes |
|---|---|---|
| Open as new unsaved document, edit, add items | Works | Never changes the `.pub`; checked by the 5.4 workflow tests. |
| Save as `.designcraft`, reopen | Works | Save proposes `flyer.designcraft` beside the `.pub`; `.pub` is never offered. Automated pass over all 142 samples (`crates/engine/tests/pub_corpus.rs`). |
| Export PDF | Works | All 142 samples export; page counts match. Not yet checked by hand (task 5.1). |
| Print | Partly | File › Print opens the system dialog and shares PDF preparation. Tested by automated checks only; physical printing and native dialogs still need a person at a desktop. |
| Soft effects in PDF | Partly | Shadowed objects are exported as images, with a notice (3 files). Looks the same, but is softer when enlarged. |
| Batch folder conversion | Works | `designcraft-cli batch-convert`: 142 PDFs plus editable copies, 79 clean, 63 with warnings, 0 failed. |

## Score (task 3.4, unchanged on `pub-import` since)

| Measure | 2.5 baseline | Now |
|---|---:|---:|
| Files imported | 142/142 | 142/142 |
| Objects found | 152/152 | 152/152 |
| Scored shape/text positions within 1 pt | 111/111 | 111/111 |
| Texts exact | 52/52 | 52/52 |
| Styles match | 111/111 | 111/111 |

The 41 image-labelled export elements were excluded from that historical position check. They have exported bounds; the separate picture baseline now compares only imported pictures (see [picture-positions.md](pub-format/picture-positions.md)).

## Open warnings

From the 142-file batch (`warning-triage.md`): 79 files open clean, 63 with warnings, 9 distinct warnings, 120 file/warning pairs.

| Files | Warning | What it means |
|---:|---|---|
| 54 | Unsupported character formatting, defaults 0x39/0x48 | Meaning unknown; no visible defect established. |
| 53 | Unsupported paragraph formatting, default 0x30 | Meaning unknown; no visible defect established. |
| 3 | Soft effects exported as images | PDF only; the editable copy keeps the shadow. |
| 3 | Table cell fills, borders, padding not decoded | Tables lose shading and rules. |
| 2 | Freeforms imported as rectangles | Curved or polyline decorations lose their outline. |
| 2 | WordArt shapes imported as straight text | Arched or circular headings come out straight. |
| 1 | Paragraph formatting, default 0x30 with run 0x3C | Text stays justified; extra effect unknown. |
| 1 | Scratch-area objects not imported | Pasteboard items are absent. |
| 1 | Shape type 74 imported as rectangle | A heart becomes a rectangle. |

## Real-file check (24 Apache POI files, 2026-10-19)

`real-file-triage.md`: 0 clean, 11 open with warnings (15 pages, 224 objects, PDFs not blank), 2 refused as Publisher 97/98/2000, **11 fail**. Eight ordinary files fail with "table: inconsistent cumulative dimension" (row sizes in the file add up slightly differently than the importer expects; whether these files are damaged is unknown, and a describer must explain it). Three are deliberately damaged fuzz files, correctly rejected. Newer warnings from these files: unidentified character and paragraph settings, an unconfirmed list marker in `SampleNewsletter`, and page-number/date fields left out. This is the main open risk before shipping.

## How this was checked

- `PUB_CORPUS=… cargo test -p designcraft-pub --test corpus -- --ignored --nocapture`: 142/142 import; 152/152 objects found; 111 of the 111 position-checked objects within 1 pt; 52/52 texts; 111/111 styles. Publisher's own layout export shows 41 objects (pictures, WordArt, tables, some shapes and lines) only as rendered images. That historical test counted overlap for those objects but skipped their positions, which is why earlier reports said "111/152". The separate picture baseline is documented in [picture-positions.md](pub-format/picture-positions.md).
- Unit tests in `crates/pub/src/lib.rs` for gradients, dashes, arrowheads, groups, linked frames and WordArt, plus synthetic and real-fixture tests for tables, bullets, line spacing, indents, tabs, character formats and shadows.
- Settings the importer doesn't recognise are reported by number in a warning rather than dropped silently; this is why the old "silent" gaps are gone.
- Not done yet: side-by-side comparison with Publisher's renders (task 5.2).
