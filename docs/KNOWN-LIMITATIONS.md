<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Known limitations

NiftyDesign is an unofficial alpha based on DesignCraft by the ArtCraft team. Bug reports go to https://github.com/srt8hellcat2-ui/niftydesign/issues, never to ArtCraft. Not supported by ArtCraft or Microsoft.

## Opening files

- Only Publisher 2002 and later files are supported. **Publisher 97, 98 and 2000 files are refused** with a message asking you to open and save the file in Publisher 2002 or later.
- **Real-file check (24 files, test files from a software project, pinned by commit):** 0 opened without warnings, 19 opened with warnings, 2 refused as Publisher 97 to 2000, 3 failed. The 3 failures are files that were deliberately damaged for testing. Earlier, 8 files with tables failed; they open now (with warnings) after a fix. Treat 24 files as a small sample: your files may behave differently, and no test of 40 or more files made by real users has been done in these docs.
- **Controlled sample files:** all 142 open (79 with no warnings, 63 with warnings). These are one-feature files, easier than real documents. They are scored against the answer files that came with them, not against Publisher.
- **Editing tests on real files:** move, resize, replace text, save as `.designcraft`, reopen and export PDF were run on two real files (the newsletter and `Sample2.pub`). The engine's results were checked. A person did not do this in a Windows window.
- **What the 24 files contain:** 15 of the 24 hold only generic content, per our scan. The newsletter and brochure hold Microsoft template content, so their images are not published.
- A warning is not proof something is missing, and no warning does not prove the file is perfect. Warnings with hex numbers (for example "unsupported character formatting 0x…") name settings the importer does not understand; no visible problem has been established for them.

## Problems found in testing

- After reopening a saved `.designcraft` in the **same session**, two pictures were missing in the interface (newsletter test). The exported PDF and a fresh reopen kept them. Cause not found.
- A controlled table sample rendered as a blank page. The importer lost the link between the table's text and its frame.
- In the newsletter test, clip art overlaps body text. Text wrap around pictures and text flow between frames are not shown to work there. Cause not isolated.
- The status bar "N errors" is the preflight panel: overset text (text that does not fit its frame) plus missing fonts. It is not a crash and not an import failure.

## Fonts

- Publisher files use Microsoft fonts that NiftyDesign cannot include.
- Arial and Times New Roman are replaced by Liberation Sans and Liberation Serif where those fonts are available.
- Arial Black, Franklin Gothic, Rockwell, Elephant, Sylfaen and others are replaced by a default font. Text can wrap differently.
- A Find/Replace Font dialog lists the missing fonts.
- Whether Windows system fonts are picked up on a real Windows PC is **UNVERIFIED**.

## Printing

- Export to PDF works (automated checks).
- The File menu has **Print...** (Ctrl+P). In-app printing is **UNVERIFIED**.
- If it fails, print the exported PDF from your PDF viewer.

## Not supported yet

- Numbered lists.
- Custom tab stops (tab characters themselves come over).
- Table cell shading (fills) and borders. Rows, columns and text stay.
- Text in merged table cells: merged cells come over, but which cell holds the text is not settled; check merged cells against the original.
- Curved, arched or circular WordArt: comes out straight.
- Publisher 97, 98 and 2000 files.
- Page-number and date fields are left out of text.
- Objects parked outside the page (scratch area) are left out.
- Subscript and strikethrough are not recognised.
- Freeform curves and some preset shapes (for example a heart) become rectangles of the same size. Work on these is blocked pending access to a public specification.
- PICT pictures show an empty frame; WMF/EMF clip art and non-rectangular crops may look wrong.
- Pattern and texture fills keep only their first colour.
- Objects with shadows are exported to PDF as images (look the same, softer when enlarged). The editable copy keeps the shadow.
- Saving back to `.pub` is not offered; save as `.designcraft` or export PDF.

## Checked only by automated tests, never by a person

- Round-trip (save `.designcraft`, reopen) and PDF export for the 142 samples: automated only.
- No side-by-side comparison against Publisher has been done, so small differences in colour, gradient direction or spacing are possible.
- 41 of 152 reference objects are image exports excluded from position scoring; their placement accuracy is UNKNOWN.

## UNVERIFIED (no evidence in the project docs)

- **The desktop GUI** of the packaged app: opening, editing, saving in a real window on Windows.
- **In-app printing** (File > Print..., Ctrl+P) and the system print dialog. Only automated checks of the engine exist. Not tried on a real Windows desktop with a printer.
- **Windows system fonts** being picked up on a real Windows PC.
- **Hosted builds:** no hosted Windows build run is recorded in the docs. The package and its contents are unconfirmed until the owner builds and checks it.
- **The web file picker** / browser version: a headless test opens a sample in a browser-style build, but dropping or picking a file in a real browser is unverified.
- **The SmartScreen / unsigned warning** wording and steps.
- **That files stay on your computer** (no network use): not stated or tested in the docs.
- Behaviour on Windows versions other than the one the owner tests. `[owner fills in]`

## Crashes

Please report any crash with what you did just before, at https://github.com/srt8hellcat2-ui/niftydesign/issues.
