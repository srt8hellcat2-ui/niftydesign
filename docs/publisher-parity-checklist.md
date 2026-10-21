<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Publisher (.pub) parity checklist for NiftyDesign

> Evidence is from the project’s own automated checks, as recorded in the clean-room sources, import status, real-file/warning triage and gallery/screenshot notes. No real Windows desktop or new format investigation is claimed here.

This is a **presence** checklist: does NiftyDesign handle each thing in an existing Microsoft Publisher document? It covers importing, displaying, editing, saving as `.designcraft` and exporting PDF of existing Publisher files. Each row is scored **D** (done), **P** (partial) or **M** (missing), weighted by priority: **P0** is core to opening a simple flyer or newsletter correctly (weight 3), **P1** weight 2, **P2** weight 1. D = 1, P = 0.5, M = 0, the same weights as ArtCraft's checklist. Rows the documents do not support are marked `unscored`, are left out of the sums and are listed under Unscored.

Caveat: rows were scored from the project's own documents, not from a side-by-side comparison with Publisher. That comparison has not been done (the status doc lists it as task 5.2). Most "done" rows mean "an automated test passes on a sample file made to show that one feature", not "a person checked the result against Publisher". Read the summary as an upper bound. Priorities are my reading of "core to a simple flyer or newsletter", not a measured ranking.

## Summary (computed by hand)

Scored rows: 60 (21 P0, 25 P1, 14 P2). Unscored rows: 11, not counted.

Weighted score = sum of (weight x status value) / sum of weights.

| Priority | Rows | Done | Partial | Missing | Arithmetic | Parity |
|---|---:|---:|---:|---:|---|---:|
| P0 | 21 | 18 | 3 | 0 | (18 + 0.5 x 3) x 3 = 58.5 of 21 x 3 = 63 | 92.9% |
| P1 | 25 | 14 | 5 | 6 | (14 + 0.5 x 5) x 2 = 33 of 25 x 2 = 50 | 66.0% |
| P2 | 14 | 2 | 4 | 8 | (2 + 0.5 x 4) x 1 = 4 of 14 x 1 = 14 | 28.6% |
| **All** | 60 | 34 | 12 | 14 | 58.5 + 33 + 4 = 95.5 of 63 + 50 + 14 = 127 | **75.2%** |

What changed from the first pass (76.8%, 96 of 125): the "Save as .designcraft and reopen" row went from D to P (pictures missing in the interface after a same-session reopen), which takes 1.5 points off P0. The "Missing-font substitution" row went from unscored to P (P1, 1 point), which adds 1 to the earned and 2 to the possible points. Net: earned 96 - 1.5 + 1 = 95.5, possible 125 + 2 = 127. 95.5 / 127 = 75.2%. The new unscored row "Text wrap around pictures" is not counted.

By area (earned weighted points of possible weighted points over the scored rows in that area):

| Area | Scored rows | D | P | M | Earned of possible | Parity |
|---|---:|---:|---:|---:|---:|---:|
| Files and versions | 6 | 3 | 1 | 2 | 8.5 of 13 | 65.4% |
| Pages and masters | 4 | 2 | 0 | 2 | 6 of 9 | 66.7% |
| Text frames | 4 | 4 | 0 | 0 | 10 of 10 | 100.0% |
| Character formatting | 4 | 2 | 1 | 1 | 6 of 8 | 75.0% |
| Paragraph formatting | 3 | 3 | 0 | 0 | 9 of 9 | 100.0% |
| Lists and tabs | 4 | 2 | 0 | 2 | 4 of 8 | 50.0% |
| Shapes and lines | 6 | 4 | 2 | 0 | 12.5 of 14 | 89.3% |
| Fills and effects | 6 | 4 | 2 | 0 | 10 of 11 | 90.9% |
| Pictures | 6 | 2 | 2 | 2 | 6.5 of 10 | 65.0% |
| Tables | 4 | 0 | 2 | 2 | 2.5 of 8 | 31.2% |
| WordArt | 3 | 1 | 0 | 2 | 2 of 5 | 40.0% |
| Groups and layout | 1 | 1 | 0 | 0 | 3 of 3 | 100.0% |
| Save and export | 5 | 3 | 1 | 1 | 6.5 of 9 | 72.2% |
| Editing in the app | 3 | 3 | 0 | 0 | 8 of 8 | 100.0% |
| Printing | 1 | 0 | 1 | 0 | 1 of 2 | 50.0% |

The area table sums to the same totals (earned 95.5, possible 127). Open P0 items: Publisher files opened without warnings in the real-file set (P in Files and versions), table cell text and layout (Tables), save and reopen showing pictures in the interface (Save and export), and the unscored desktop-window check (Editing in the app).

## Rows

| Area | Feature | Priority | Status | Note |
|---|---|---|---|---|
| Files and versions | Publisher 2002 to 2021 files open | P0 | P | Controlled 142/142 open; real-user set 0 clean, 19 with warnings. |
| Files and versions | Opens as a new unsaved document; the .pub is never changed | P0 | D | Checked by the 5.4 workflow tests. |
| Files and versions | Damaged files refused with a message, no crash | P1 | D | 3 damaged files fail with "import: ..." messages; no worker crash or timeout reported. |
| Files and versions | Warning list names what could not come over | P1 | D | Unknown settings are warned by number, not dropped silently. |
| Files and versions | Publisher 97, 98 and 2000 files | P1 | M | Refused with a message asking for a re-save in 2002 or later. |
| Files and versions | Publisher version reported to the user | P2 | M | "not reported" for every real file. |
| Files and versions | Opening a file through the web file picker | P1 | unscored | UNVERIFIED: only a headless test exists. |
| Pages and masters | Page size, orientation, page count | P0 | D | pages__* samples. |
| Pages and masters | Master pages with their shapes and text | P0 | D | Become parent pages. |
| Pages and masters | Page-number and date fields | P1 | M | Left out of the text, warned. |
| Pages and masters | Objects in the scratch area beside the page | P2 | M | Left out, warned. |
| Text frames | Text in frames (edited, long text) | P0 | D | 52/52 texts match. |
| Text frames | Linked text frames | P0 | D | One story threaded in order. |
| Text frames | Frame margins, vertical alignment, rotated frames | P1 | D | text__margins, vanchor, rotated. |
| Text frames | Frame fill and border | P1 | D | text__frame-fill-line. |
| Text frames | Columns, autofit and overflow behaviour | P1 | unscored | Not mentioned in the documents. |
| Character formatting | Font, size, bold, italic, underline, colour | P0 | D | 111/111 styles match. |
| Character formatting | All caps, small caps, superscript, letter spacing | P1 | D | Come over as character settings. |
| Character formatting | Subscript, strikethrough | P2 | M | Not recognised. |
| Character formatting | Missing-font substitution | P1 | P | Microsoft fonts cannot be included. Arial and Times New Roman go to Liberation Sans and Liberation Serif where available. Arial Black, Franklin Gothic, Rockwell, Elephant, Sylfaen and others go to a default font, so text can wrap differently. A Find/Replace Font dialog lists missing fonts. Windows system fonts on a real PC: UNVERIFIED. |
| Character formatting | Settings with unknown meaning (0x39, 0x48, 0x3C) | P2 | unscored | Warned; no visible defect established. |
| Paragraph formatting | Alignment (left, centre, right, justified) | P0 | D | text__align-*. |
| Paragraph formatting | Indents, space before and after | P0 | D | text__indent-*, text__space-*. |
| Paragraph formatting | Line spacing (exact, multiple, inherited) | P0 | D | text__ls-*. |
| Paragraph formatting | Default paragraph setting 0x30 | P2 | unscored | Warned in 53 files; meaning unknown. |
| Lists and tabs | Bullets | P1 | D | Marker and hanging indent; marker glyph look may differ. |
| Lists and tabs | Numbered lists | P1 | M | Not supported. |
| Lists and tabs | Tab characters | P1 | D | Literal tabs survive. |
| Lists and tabs | Custom tab stops | P1 | M | Not supported. |
| Shapes and lines | Position, size, rotation, flips | P0 | D | 111/111 position-checked objects within 1 pt. |
| Shapes and lines | Common preset shapes (rectangle, oval, rounded rectangle, triangle, star, arrow) | P0 | D | shapes__*. |
| Shapes and lines | Line colour, weight, none | P0 | D | line__*. |
| Shapes and lines | Dashed lines and arrowheads | P1 | D | line__dash-*, shapes__line-arrow. |
| Shapes and lines | Freeform curves and polylines | P1 | P | Same-size rectangle, warned; the status doc calls it Partly. Work blocked in ROADMAP. |
| Shapes and lines | Less common preset shapes (heart, type 74) | P2 | P | Same-size rectangle, warned; scored as the status doc labels it. |
| Fills and effects | Solid fills, scheme colours, no fill | P0 | D | fill__*. |
| Fills and effects | Gradient fills | P1 | D | Direction not compared with Publisher by eye. |
| Fills and effects | Fill transparency | P1 | D | Not compared with Publisher by eye. |
| Fills and effects | Shadows | P1 | D | Become drop shadows; unrecognised types warned. |
| Fills and effects | Pattern and texture fills | P2 | P | First colour only. |
| Fills and effects | Soft effects in exported PDF | P2 | P | Shadowed objects become images in the PDF; the editable copy keeps the shadow. |
| Fills and effects | Glow, reflection and other effects | P2 | unscored | Not mentioned in the documents. |
| Pictures | PNG, JPEG, BMP pictures (moved, stretched, rotated, flipped) | P0 | D | picture__*. |
| Pictures | Rectangular crops and picture borders | P1 | D | picture__crop-*, picture__border. |
| Pictures | Crops to other shapes | P2 | P | Not supported in general; picture__oval-mask not checked visually. |
| Pictures | WMF and EMF clip art | P1 | P | Vector art without text, bitmaps, arcs, pattern fills. |
| Pictures | Text inside WMF pictures | P2 | M | Left out, warned. |
| Pictures | PICT pictures | P2 | M | Empty frame keeps the place. |
| Pictures | Placement accuracy of pictures | P0 | unscored | 41 of 152 objects are image-only in the reference data. |
| Tables | Tables: rows, columns, sizes, editable cell text | P0 | P | Real tables; the 8 earlier failures now open. Table cell text placement is not decoded. The headless table sample rendered blank because the importer lost the host story's frame linkage (docs/gallery/README.md, REPORTS.md). |
| Tables | Merged cells | P1 | P | Text in merged cells that cannot be placed is kept after the table, warned. |
| Tables | Cell shading and borders | P1 | M | Not decoded, warned. |
| Tables | Cell padding | P2 | M | Not decoded, warned. |
| WordArt | Straight WordArt as editable text (font, colour, outline, shadow) | P1 | D | wordart__*. |
| WordArt | Curved, arched and circular WordArt | P1 | M | Comes out straight, warned. |
| WordArt | Stretch-to-fill of the box | P2 | M | Lost. |
| Groups and layout | Groups with rotation and flips | P0 | D | group__*. |
| Groups and layout | Layers, stacking order, guides | P1 | unscored | Not mentioned in the documents. |
| Save and export | Save as .designcraft and reopen | P0 | P | Engine round trip passes over all 142 samples and on two real files (automated). But after reopening a saved file in the SAME session, two pictures were missing in the interface (newsletter test); the PDF and a fresh reopen kept them. Cause not found (screenshots/real-file/README.md). |
| Save and export | Export PDF | P0 | D | All 142 export, page counts match; not yet checked by hand or against Publisher. |
| Save and export | Export page images (CLI) | P2 | D | Gallery PNGs; the headless table sample came out blank (cause above). |
| Save and export | Batch folder conversion | P2 | D | designcraft-cli batch-convert. |
| Save and export | Save back to .pub | P2 | M | Not offered. |
| Editing in the app | Select, move and resize imported items | P0 | D | Engine workflow test, and two real files (newsletter, Sample2.pub) with engine-checked bounds; the desktop GUI itself is UNVERIFIED. |
| Editing in the app | Edit imported text | P0 | D | Engine test, screenshot 04, and the same two real files (replace text, save, reopen, export PDF); desktop GUI UNVERIFIED. |
| Editing in the app | Add text, shapes and pictures | P1 | D | Engine workflow test. |
| Editing in the app | Using all of the above in a real desktop window | P0 | unscored | UNVERIFIED: screenshots come from a test renderer, not a desktop. |
| Editing in the app | Editing table cells or master pages of an imported file | P1 | unscored | Not mentioned in the documents. |
| Editing in the app | Text wrap around pictures; text flow between frames in a real file | P1 | unscored | Reported not working in the newsletter test. The docs record only that clip art overlaps body text there, cause not isolated. No check is recorded, so no status. |
| Printing | In-app File > Print | P1 | P | The File menu now has Print... (Ctrl+P), an existing engine command exposed in the menu. Automated tests check the engine side. Whether it works on a real Windows desktop and printer is UNVERIFIED. Export to PDF works (automated checks). |
| Printing | Physical printing and the native print dialog | P1 | unscored | UNVERIFIED: never observed on a desktop. |

## Unscored

These rows have no supporting statement in the documents, so no status was guessed. UNVERIFIED means the docs say it has never been observed.


- **Files and versions: Opening a file through the web file picker** (P1). UNVERIFIED: only a headless test exists.
- **Text frames: Columns, autofit and overflow behaviour** (P1). Not mentioned in the documents.
- **Character formatting: Settings with unknown meaning (0x39, 0x48, 0x3C)** (P2). Warned; no visible defect established.
- **Paragraph formatting: Default paragraph setting 0x30** (P2). Warned in 53 files; meaning unknown.
- **Fills and effects: Glow, reflection and other effects** (P2). Not mentioned in the documents.
- **Pictures: Placement accuracy of pictures** (P0). 41 of 152 objects are image-only in the reference data.
- **Groups and layout: Layers, stacking order, guides** (P1). Not mentioned in the documents.
- **Editing in the app: Using all of the above in a real desktop window** (P0). UNVERIFIED: screenshots come from a test renderer, not a desktop.
- **Editing in the app: Editing table cells or master pages of an imported file** (P1). Not mentioned in the documents.
- **Editing in the app: Text wrap around pictures; text flow between frames in a real file** (P1). Reported not working in the newsletter test; no check is recorded in the docs.
- **Printing: Physical printing and the native print dialog** (P1). UNVERIFIED: never observed on a desktop.

## Out of scope

Publisher's authoring-only features are not scored: mail merge and catalog merge, the design checker, the templates and building-blocks catalogue, Publisher's own print and pack-and-go wizards, and editing features that matter only when creating a new publication in Publisher. Saving back to `.pub` is in scope only as a row (M); the docs say it is not offered.

## Measured results

These are numbers already in the docs. Nothing was re-run.

**142 controlled files** (one-feature controlled sample files, from the pubsmith set. The records do not say whether a person or a generator tool made them). Scored against the answer files that came with those samples, **not against Publisher**:

| Measure | Result |
|---|---|
| Files imported | 142/142 |
| Objects found | 152/152 |
| Positions within 1 pt | 111/152 (the other 41 are image objects with no position in the answer files, so they are unscored) |
| Texts exact | 52/52 |
| Styles match | 111/111 |

Of the 142, 79 open with no warnings and 63 with warnings; 0 fail. These files are easier than real documents.

**24 real files** (Apache sample set, pinned by commit):

| Result | Files |
|---|---:|
| Opened without warnings | 0 |
| Opened with warnings | 19 |
| Refused (Publisher 97 to 2000) | 2 |
| Damaged (deliberately, fuzz test files) | 3 |

The 19 successful files give 31 pages and 280 objects. Those are importer counts, not an independent estimate of what the files contain. 24 files is a small sample, and every one of the 19 opened with at least one warning.

Editing (move, resize, replace text, save as `.designcraft`, reopen, export PDF) was run on two of the real files, the newsletter and `Sample2.pub`, with engine-checked results. Both kept all edits after a reopen. From the content scan, 15 of the 24 contain only generic content; the newsletter and brochure contain Microsoft template content, so their images are not published. The status bar "N errors" is the preflight panel (overset text plus missing fonts), not a crash: 12 for the newsletter, 3 for `Sample2.pub`.

## Caveat

The summary above is higher than the real-file results justify. Rows were scored from the project's own documents, mostly by the agents that built the features, using sample files made to show one feature at a time. Not one real file has been compared with what Publisher itself shows. The 142-file score is measured against the answer files that came with the samples, not against Publisher, and 41 of the 152 objects cannot be position-checked at all. No real file opened without warnings, and the desktop window, in-app printing and the web file picker have never been observed. Read 75.2% as an upper bound, and treat the P0 figure (92.9%) with the most suspicion, because it rests on one-feature samples.

## How to get a real number

1. Collect real Publisher files from people who use Publisher: flyers, newsletters, brochures, with at least 40 files and a mix of versions (2002 to 2021). Do not use files with private information.
2. For each, have Publisher export a PDF of every page. Keep that PDF as the reference.
3. Open each file in NiftyDesign, record the warnings, and export its PDF.
4. Render both PDFs to images at the same size and compare page by page: layout, text wrap and breaks, fonts, colours, pictures, tables. Record a pass, a visible difference or a failure per page, and what differed.
5. Re-score each row of this checklist from those findings, and replace the "unscored" rows with evidence.
6. Report pages that match, pages with visible differences and files that fail, out of the total, as a plain count.

## Needs you

- **Printing.** Print... (Ctrl+P) is now in the File menu. Please try it on a real Windows PC with a printer and tell us the result. Until then the row stays P and UNVERIFIED.
- **Fonts.** The Liberation Sans/Serif replacement comes from the task instructions; no doc in docs/ records it. Please confirm it.
- **Do you want priorities changed?** P0, P1 and P2 are my judgement, not from the docs.
- **Real files.** Step 1 above needs files from you or testers.
