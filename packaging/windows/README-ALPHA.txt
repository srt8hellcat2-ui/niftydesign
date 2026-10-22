NiftyDesign 0.9.0-nifty-alpha.1 - unofficial alpha
Windows x64 only. Not for ARM64, 32-bit Windows, macOS or Linux.

NiftyDesign is an unofficial alpha for opening and editing older Microsoft
Publisher (.pub) files, saving .designcraft documents, exporting PDF and printing.
Based on DesignCraft by the ArtCraft team. Unofficial build; not supported by
ArtCraft or Microsoft. This is not a Microsoft Publisher release.

UNSIGNED BUILD: expect Windows SmartScreen to warn about this downloaded app.
The exact SmartScreen prompt and launch experience are UNVERIFIED on a real
desktop for this build. Extract the whole zip to a folder before running
 designcraft.exe. The command-line tool is designcraft-cli.exe.

Verify the zip before extracting. Download both the zip and its separate
niftydesign-0.9.0-nifty-alpha.1-windows-x64.zip.sha256 file into the same folder.
In PowerShell, from that folder:
  $zip = 'niftydesign-0.9.0-nifty-alpha.1-windows-x64.zip'
  $expected = ((Get-Content -LiteralPath "$zip.sha256").Trim() -split '  ', 2)[0]
  if ((Get-FileHash -Algorithm SHA256 -LiteralPath $zip).Hash -ne $expected) {
    throw "SHA256 mismatch: $zip"
  }
The separate .sha256 file covers the finished zip itself. Compare its hash
before extracting. After extraction, verify the other files before running them:
  Get-Content SHA256SUMS | ForEach-Object {
    $hash, $file = $_ -split '  ', 2
    if ((Get-FileHash -Algorithm SHA256 -LiteralPath $file).Hash -ne $hash) {
      throw "SHA256 mismatch: $file"
    }
  }
SHA256SUMS inside the zip covers every other file in it, including this README;
it cannot hash itself. Hashes detect changes, but are not signatures or proof
of who sent the files. Obtain the zip and checksum through a trusted channel.

Headless tests cover .pub import, saving/reopening .designcraft, PDF export and
page rendering. Imported objects are editable in the document model.
UNVERIFIED ON A REAL DESKTOP: the desktop GUI, interactive editing, file dialogs,
and the native print dialog and physical printing. The web file picker is also
UNVERIFIED; this Windows package does not include the web app.

Known gaps: numbered lists and custom tab stops are unsupported; table cell
shading and borders are not decoded; merged-cell text is unconfirmed;
curved WordArt becomes straight text. Text inside WMF/EMF pictures is left out.
Publisher 97-2000 files are refused.
In the eight Apache test tables, rows and columns import but cell text is not shown yet. The synthetic 2x2 table now draws A1, B1, A2 and B2. Cell fills, borders and uncertain merged-cell text remain unsupported.

The earlier same-session picture disappearance was a screenshot-timing bug in the app's ui.screenshot command, now fixed. No document or image data was lost.

File > Print... (Ctrl+P) sends pages through the system print spooler: CUPS on macOS and Linux, and a Windows PDF raster spooler. Printing on a real printer is UNVERIFIED. The web build cannot print; export a PDF instead. Export to PDF works (automated checks).

Screenshots are interface renderings from the app's own test renderer on Linux, not a Windows desktop capture.

The demo flyer's background, croissant and coffee-cup pictures are AI-generated.
See ASSETS.md and the source tree's assets/demo/NOTES.txt.

Other unsupported features can produce warnings or different-looking output.
Keep the original .pub and check any exported PDF carefully.

The 24-file Apache test set in docs/real-file-triage.md produced:
  0 opened without warnings; 19 opened with warnings;
  2 refused as the Publisher 97/98/2000 family; 3 damaged files failed.
These are test files, including deliberately damaged inputs, not a representative
survey of real users' publications. A warning is not a promise of visual accuracy.

Report problems using GitHub Issues on the project owner's NiftyDesign repo:
REPO_URL_HERE
The owner must fill in that URL before sharing. Never report this unofficial
build to the ArtCraft repositories. Include the version, Windows version, steps,
error/warning text and report.csv if available. Do not attach files, screenshots
or reports containing personal information; remove private content first.
