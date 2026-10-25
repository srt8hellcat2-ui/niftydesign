<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# NiftyDesign unsigned alpha: release notes

NiftyDesign is an unofficial alpha that opens Microsoft Publisher (.pub) files from Publisher 2002 and newer, lets you edit their content, and exports PDF. It is the full DesignCraft desktop app, built from DesignCraft 0.12.0 by the ArtCraft team, with a Publisher importer added. It is not affiliated with or supported by ArtCraft or Microsoft. This download is for Windows x64.

## What changed since 0.12.0-nifty-alpha.3

- Shapes drawn with Publisher's curve and polyline tools now come in as editable paths instead of rectangles. Segment types we can't draw still give a warning.
- The heart shape now draws as a heart.
- Upward-arch WordArt now comes in as editable curved text. Circle WordArt and other WordArt shapes still come in as straight text with a warning. Letter shapes and spacing still differ from Publisher's.
- Missing fonts are replaced by the closest available font by type and width instead of always the default (for example Arial Black now uses a heavy sans-serif). Original font names are kept in the file. The Find/Replace Font window no longer opens by itself when you open a Publisher file; it is still in the menu. Line breaks and headline fit can change, and some headlines can still be cut off.
- On our checks the controlled sample results are unchanged (142 of 142 files import, 152 of 152 objects found, all 111 scored objects within 1 point (the 41 pictures are not scored for position), 52 of 52 texts and 111 of 111 styles correct) and the 24 real Apache test files give the same outcome (9 with notes, 10 with warnings, 2 refused, 3 damaged). Those checks don't measure how the new curves, heart and arch look. We compared those with Publisher's own images for only nine test files.
- Not changed: table fills, borders and padding; numbered lists and tab stops; other WordArt shapes; page numbers and dates; Publisher 97, 98 and 2000 files.

## This is a test build, and we need your help

The maintainer has run an earlier alpha on one Windows 11 PC with one inkjet printer. There it launched, opened Apache's public Sample2.pub test file, a flyer and a business card were built in it, saved, reopened after a restart, exported to PDF and printed. That is the only Windows PC tried by the maintainer so far, so you may find problems we have not. We also have no collection of real people's Publisher files: so far we tested with public Apache POI sample files and with files we generated. That is why we are asking you to try your own files and tell us what happens. Whatever you find, good or bad, is useful.

## What opening a file does

Opening a .pub never modifies your original file on disk. NiftyDesign opens a converted copy as a new, unsaved document. That copy can look different from what Publisher shows. Some features are lost or simplified (table fills, borders and padding, WordArt letter shapes and spacing, numbered lists and tab stops, page numbers and dates, for example), and not every loss gives a warning. Always compare it with your original. Publisher 97, 98 and 2000 files are refused.

## Unsigned download, SmartScreen and Smart App Control

The app is not signed. Windows SmartScreen will warn. After verifying the checksum, if you choose to run it, select **More info**, then **Run anyway**. Extract the entire zip before starting `designcraft.exe`.

On Windows 11 PCs where Smart App Control is turned On, Windows can block an unsigned app completely, with the message "An Application Control policy has blocked this file" and no Run anyway option. This happened on the maintainer's own PC to a rebuilt copy of this app, which was blocked the first time and ran normally a few hours later with no change on our side. If you see it, read the Smart App Control section of Known limitations for your options. We plan to apply for free open-source code signing.

## Check your download

Download the zip and its separate `.zip.sha256` file into the same folder. In PowerShell, change the version below to the version you downloaded:

```powershell
$zip = 'NiftyDesign-0.12.0-nifty-alpha.4-windows-x64.zip'
$line = (Get-Content -LiteralPath "$zip.sha256").Trim()
$hash, $filename = $line -split '  ', 2
if ($filename -ne $zip -or $hash -notmatch '^[0-9a-fA-F]{64}$' -or
    (Get-FileHash -Algorithm SHA256 -LiteralPath $zip).Hash -ne $hash) {
    throw 'Checksum mismatch; do not run or extract this download'
}
'Checksum matches'
```

The sidecar verifies the finished zip before extraction. `SHA256SUMS` inside the zip lists the hashes of its other files; it cannot hash itself. A checksum checks integrity, not the identity of the sender; it is not a signature.

## Verified and unverified

This version (alpha.4): built and smoke-tested automatically on Windows. Not yet tried by hand on a Windows PC.

An earlier alpha worked on the maintainer's one Windows 11 PC: launching, opening a .pub, editing, saving and reopening, PDF export, printing on one inkjet printer, and Arial and Times New Roman found as installed fonts.

Not verified: other PCs and other printers, the native print dialog and file dialogs, and mouse-driven editing in a full session. Real files often open with warnings and can differ from the original layout. Keep your original files and check the exported PDF. See [Known limitations](https://github.com/srt8hellcat2-ui/niftydesign/blob/main/docs/KNOWN-LIMITATIONS.md).

## Reporting problems

Report issues at https://github.com/srt8hellcat2-ui/niftydesign/issues. Issues are public: attach a file only if everyone may see it, and do not attach personal information. Follow the [sample request guidance](https://github.com/srt8hellcat2-ui/niftydesign/blob/main/docs/SAMPLE-REQUEST.md).
