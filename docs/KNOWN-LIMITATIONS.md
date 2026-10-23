<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Known limitations

NiftyDesign is an unofficial alpha based on DesignCraft by the ArtCraft team. Bug reports go to https://github.com/srt8hellcat2-ui/niftydesign/issues, never to ArtCraft. Not supported by ArtCraft or Microsoft.

## What has been tested on Windows, and what has not

The maintainer tested the downloaded alpha on **one Windows 11 PC** (NVIDIA RTX 3090 Ti, DirectX 12) and one inkjet printer (HP Smart Tank 5100). What worked there:

- The zip extracted and its file checksums matched.
- The desktop app launched, stayed responsive, and opened Apache's `Sample2.pub`.
- A flyer and a business card were built from it in the app, saved as `.designcraft`, reopened in the same session and again after a full restart (same pages, same objects), and exported to PDF.
- Both pages printed on the inkjet printer. The business card (a 3.5 x 2 inch page) printed at its real size, centered on the sheet.
- Windows system fonts (Arial, Times New Roman) used by `Sample2.pub` were found as installed fonts.

What this does **not** show: other Windows PCs, other printers, other graphics cards, or a broad set of real-world files. Treat it as one successful run, not a guarantee.

## Opening files

- Only Publisher 2002 and later files are supported. **Publisher 97, 98 and 2000 files are refused** with a message asking you to open and save the file in Publisher 2002 or later.
- **Real-file check (24 files, test files from a software project, pinned by commit):** 0 opened without warnings, 19 opened with warnings, 2 refused as Publisher 97 to 2000, 3 failed. The 3 failures are files that were deliberately damaged for testing. Earlier, 8 files with tables failed; they open now (with warnings) after a fix. Treat 24 files as a small sample: your files may behave differently, and no test of 40 or more files made by real users has been done.
- **Controlled sample files:** all 142 open (79 with no warnings, 63 with warnings). These are one-feature files, easier than real documents. They are scored against the answer files that came with them, not against Publisher.
- **What the 24 files contain:** 15 of the 24 hold only generic content, per our scan. The newsletter and brochure hold Microsoft template content, so their images are not published.
- A warning is not proof something is missing, and no warning does not prove the file is perfect. Warnings with hex numbers (for example "unsupported character formatting 0x...") name settings the importer does not understand; no visible problem has been established for them.

## Problems found in testing

- In the eight Apache test tables, rows and columns import but cell text is not shown yet. The synthetic 2x2 table draws A1, B1, A2 and B2. Cell fills, borders and uncertain merged-cell text remain unsupported.
- In the newsletter test, clip art overlaps body text. Text wrap around pictures and text flow between frames are not shown to work there. Cause not isolated.
- The status bar "N errors" is the preflight panel: overset text (text that does not fit its frame) plus missing fonts. It is not a crash and not an import failure.
- On Windows, an error message in the status bar stayed on screen after later commands succeeded, until the app was restarted. We are fixing this.
- After printing, the job stayed listed as "Retained" in the Windows print queue. The pages printed. We are looking into it.

## Fonts

- Publisher files use Microsoft fonts that NiftyDesign cannot include.
- On the Windows test PC, installed fonts such as Arial and Times New Roman were found and used.
- If a font is not installed, a default font is substituted and text can wrap differently. Arial Black, Franklin Gothic, Rockwell, Elephant, Sylfaen and others are likely to be substituted on PCs that lack them.
- A Find/Replace Font dialog lists the missing fonts.
- We have only checked this on one Windows PC. Tell us if your fonts are not found.

## Printing

File > Print... (Ctrl+P) sends pages through the system print spooler: CUPS on macOS and Linux, and a Windows PDF raster spooler. It worked on one Windows PC with one inkjet printer. Other printers, and the native print dialog, are **UNVERIFIED**. The web build cannot print; export a PDF instead. Export to PDF works.

## Not supported yet

- Numbered lists.
- Custom tab stops (tab characters themselves come over).
- Table cell shading (fills) and borders. In the eight Apache test tables, rows and columns import but cell text is not shown yet. Cell fills, borders and uncertain merged-cell text remain unsupported.
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

- Round trip (save `.designcraft`, reopen) and PDF export for the 142 samples: automated only. One manual round trip was done on Windows (see the top of this page).
- No side-by-side comparison against Publisher has been done, so small differences in colour, gradient direction or spacing are possible.
- 41 of 152 reference objects are image exports excluded from position scoring; their placement accuracy is UNKNOWN.

## UNVERIFIED

- **Clicking and typing in the desktop window.** The Windows test drove the app through its command channel and took screenshots of the window. A person has not yet worked through menus, tools and file dialogs by mouse in this build.
- **File Open and Save dialogs, and the native print dialog.**
- **Other Windows PCs, other printers, other graphics hardware.**
- **The web file picker** / browser version: a headless test opens a sample in a browser-style build, but dropping or picking a file in a real browser is unverified.
- **The SmartScreen / unsigned warning** wording and steps, as seen after downloading the zip through a browser.
- **That files stay on your computer** (no network use): not stated or tested in the docs.

## Crashes

Please report any crash with what you did just before, at https://github.com/srt8hellcat2-ui/niftydesign/issues. If you can, attach `%APPDATA%\DesignCraft\logs\designcraft.log`.

## Screenshot and demo evidence

The `windows-*` screenshots in `docs/screenshots` were taken from the running app on the Windows test PC. The other screenshots in that folder come from the app's own test renderer on Linux, not a Windows desktop capture.

The demo flyer's croissant and coffee-cup pictures are AI-generated; see [ASSETS.md](../ASSETS.md) and [assets/demo/NOTES.txt](../assets/demo/NOTES.txt).
