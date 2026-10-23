<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Alpha notes: opening Publisher (.pub) files

Thanks for trying this. NiftyDesign is an early, unofficial alpha. It opens Microsoft Publisher files so you can edit them and save or export the result. It is based on DesignCraft by the ArtCraft team and is not affiliated with Microsoft or ArtCraft. Facts here come from pub-import-status.md and real-file-triage.md in this folder.

## What to know first

- Opening a `.pub` never modifies your original file on disk. NiftyDesign opens a converted copy as a new, unsaved document, and that copy can look different from the original: some features are lost or simplified (see below). Always compare it with your original.
- Save proposes a `.designcraft` file next to the original. Saving back to `.pub` is not offered.
- Only Publisher 2002 and later files are supported.

## What works

- **Opening.** All 142 controlled sample files open. These were generated for testing, not made in Publisher, and each shows one feature. Page size, orientation, page count and master pages come over.
- **Text.** Text frames, fonts, sizes, colours, alignment, indents, line spacing, bullets, tab characters, all caps, small caps, superscript and letter spacing.
- **Shapes and lines.** Position, size, rotation and flips; common shapes (rectangles, ovals, rounded rectangles, triangles, diamonds, hexagons, stars, arrows, lines); solid and gradient fills; fill transparency; line colour, weight, dashes and arrowheads; shadows; groups.
- **Pictures.** PNG, JPEG, BMP and similar, with rectangular crops and borders.
- **Tables.** Editable tables with rows, columns, sizes and merged cells in the controlled samples. Real files with tables now open, but cell text is not drawn yet.
- **WordArt.** Editable text with its font, colours, outline and shadow.
- **Saving and exporting.** Save as `.designcraft` and reopen; export PDF. An automated pass covers all 142 samples. A person has not yet checked the PDFs by hand.

## What does not work yet

- **Real files often open with warnings.** In a check of 24 real files (pinned test files from another project, not ours): none opened with no warnings, 19 opened with warnings, 2 were refused as too old, and 3 were deliberately damaged files that were correctly rejected. Check any file against the original.
- **Publisher 97, 98 and 2000 files are refused** with a message asking you to open and save the file in Publisher 2002 or later.
- **Tables in real files draw no cell text yet.** Tables lose cell shading and borders. Rows and columns stay.
- **Numbered lists and custom tab stops are lost.**
- **Page numbers and dates are left out** of the text.
- **Arched or circular WordArt comes out straight.**
- **Some freeform curves and unusual shapes become rectangles** of the same size (for example a heart).
- **Objects parked beside the page (scratch area) are left out.**
- **Subscript and strikethrough** are not recognised.
- **PICT pictures** show as an empty frame. Crops to non-rectangular shapes and some clip art (WMF/EMF) may look wrong.
- **Pattern and texture fills** keep only their first colour.
- **Fonts.** If a Microsoft font is not found, a substitute is used and text can shift or be cut off. On one Windows test PC, installed fonts such as Arial and Times New Roman were found.
- **Soft effects in PDF.** Objects with shadows are exported as images. They look the same but are softer when enlarged. The editable copy keeps the shadow.
- **Printing** worked on one Windows PC with one inkjet printer. Other printers are untested. Export a PDF if you need to print.
- **No side-by-side check against Publisher** has been done yet, so small differences in colour, gradient direction or spacing are possible.

## What the warnings mean

When a file opens, NiftyDesign lists what it could not bring over fully. In the 142-file controlled batch, 79 files opened with no warnings and 63 with warnings. Most warnings fall into three groups:

1. **Known losses** (table shading, page-number fields, straight WordArt, rectangles for freeforms, scratch-area objects). The layout is affected in the way the warning says. Check those spots against the original.
2. **Export notices** (objects exported as images in the PDF). Informational only.
3. **"Unsupported character formatting" or "paragraph formatting" with hex numbers** (for example 0x39, 0x48, 0x30). These are settings the importer does not understand yet. No visible problem has been established for them. Treat them as noise unless something looks wrong. A warning is not proof that something is missing, and no warning does not prove the file is perfect.

## How to report a problem

Open an issue at https://github.com/srt8hellcat2-ui/niftydesign/issues. Issues are public, so do not post anything private. Please include:

1. A short description of what you expected and what you saw.
2. The exact warning text, copied from the warning list.
3. For a file that will not open, the exact error message (it starts with "import:").
4. Your Windows version.
5. A screenshot of the original in Publisher beside the NiftyDesign result, if you can.
6. The `.pub` file only if everyone is welcome to see it (see SAMPLE-REQUEST.md). Never attach files with private or personal information.

Please also say if NiftyDesign crashed, and what you did just before.
