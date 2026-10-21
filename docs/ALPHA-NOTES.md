<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Alpha notes for testers: opening Publisher (.pub) files

Thanks for trying this. This is an early test build. It opens Microsoft Publisher files in DesignCraft so you can edit them and save or export the result. Facts here come from [pub-import-status.md](pub-import-status.md) (refreshed 2026-10-19) and [real-file-triage.md](real-file-triage.md). Nothing was re-run for these notes.

## What to know first

- Opening a `.pub` never changes it. The file opens as a new, unsaved DesignCraft document.
- Save proposes a `.designcraft` file next to the original. Saving back to `.pub` is not offered.
- Only Publisher 2002 and later files are supported.

## What works

- **Opening.** All 142 sample files made in Publisher open (each shows one feature). Page size, orientation, page count and master pages come over.
- **Text.** Text frames, fonts, sizes, colours, alignment, indents, line spacing, bullets, tab characters, all caps, small caps, superscript and letter spacing.
- **Shapes and lines.** Position, size, rotation and flips; common shapes (rectangles, ovals, rounded rectangles, triangles, diamonds, hexagons, stars, arrows, lines); solid and gradient fills; fill transparency; line colour, weight, dashes and arrowheads; shadows; groups.
- **Pictures.** PNG, JPEG, BMP and similar, with rectangular crops and borders.
- **Tables.** Real, editable tables with rows, columns, sizes, cell text and merged cells.
- **WordArt.** Editable text with its font, colours, outline and shadow.
- **Saving and exporting.** Save as `.designcraft` and reopen; export PDF. An automated pass covers all 142 samples. A person has not yet checked the PDFs by hand.

## What does not work yet

- **Some real-user files do not open.** In a check of 24 files made by users of other programs, 11 opened (all with warnings), 2 were refused as too old, and 11 failed. Eight of the failures are ordinary files with a table, shown as "table: inconsistent cumulative dimension". Whether those files are damaged is not known. The other three were deliberately damaged test files and were correctly rejected. **A file with a table may not open at all.**
- **Publisher 97, 98 and 2000 files are refused** with a message asking you to open and save the file in Publisher 2002 or later.
- **Tables lose cell shading and borders.** Rows, columns and text stay.
- **Numbered lists and custom tab stops are lost.**
- **Page numbers and dates are left out** of the text.
- **Arched or circular WordArt comes out straight.**
- **Some freeform curves and unusual shapes become rectangles** of the same size (for example a heart).
- **Objects parked beside the page (scratch area) are left out.**
- **Subscript and strikethrough** are not recognised.
- **PICT pictures** show as an empty frame. Crops to non-rectangular shapes and some clip art (WMF/EMF) may look wrong.
- **Pattern and texture fills** keep only their first colour.
- **Soft effects in PDF.** Objects with shadows are exported as images. They look the same but are softer when enlarged. The editable copy keeps the shadow.
- **Printing** is tested only by automated checks. Physical printing and the native print dialog have not been checked on a real desktop.
- **No side-by-side check against Publisher** has been done yet, so small differences in colour, gradient direction or spacing are possible.

## What the warnings mean

When a file opens, DesignCraft lists what it could not bring over fully. In the 142-file batch, 79 files opened with no warnings and 63 with warnings. Most warnings fall into three groups:

1. **Known losses** (table shading, page-number fields, straight WordArt, rectangles for freeforms, scratch-area objects). The layout is affected in the way the warning says. Check those spots against the original.
2. **Export notices** (objects exported as images in the PDF). Informational only.
3. **"Unsupported character formatting" or "paragraph formatting" with hex numbers** (for example 0x39, 0x48, 0x30). These are settings the importer does not understand yet. They appear in about 54 of 142 files and no visible problem has been established. Treat them as noise unless something looks wrong. A warning is not proof that something is missing, and no warning does not prove the file is perfect.

## How to report a problem

Send the owner who gave you this build:

1. A short description of what you expected and what you saw.
2. The exact warning text, copied from the warning list.
3. For a file that will not open, the exact error message (it starts with "import:").
4. Your operating system.
5. A screenshot of the original in Publisher beside the DesignCraft result, if you can.
6. The `.pub` file only if it is not private. Do not send files with private or personal information.

Please also say if DesignCraft crashed, and what you did just before.
