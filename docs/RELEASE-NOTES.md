<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# NiftyDesign unsigned alpha: release notes

NiftyDesign is an unofficial alpha that opens Microsoft Publisher (.pub) files from Publisher 2002 and newer, lets you edit their content, and exports PDF. It is the full DesignCraft desktop app, built from DesignCraft 0.9.0 by the ArtCraft team, with a Publisher importer added. It is not affiliated with or supported by ArtCraft or Microsoft. This download is for Windows x64.

## What changed since alpha.1

- Table cell text is now drawn.
- Added the converted-copy notice and import notes.
- Clearer note wording for skipped formatting settings.
- Status bar message fixes.
- Test-only maintenance.
- Desktop demo workflow fix.

## This is a test build, and we need your help

The maintainer has run this app on one Windows 11 PC with one inkjet printer. There it launched, opened Apache's public Sample2.pub test file, a flyer and a business card were built in it, saved, reopened after a restart, exported to PDF and printed. That is the only Windows testing so far, so you may find problems we have not. We also have no collection of real people's Publisher files: so far we tested with public Apache POI sample files and with files we generated. That is why we are asking you to try your own files and tell us what happens. Whatever you find, good or bad, is useful.

## What opening a file does

Opening a .pub never modifies your original file on disk. NiftyDesign opens a converted copy as a new, unsaved document. That copy can look different from what Publisher shows. Some features are lost or simplified (table fills, borders and padding, curved WordArt, numbered lists and tab stops, page numbers and dates, for example), and not every loss gives a warning. Always compare it with your original. Publisher 97, 98 and 2000 files are refused.

## Unsigned download, SmartScreen and Smart App Control

The app is not signed. Windows SmartScreen will warn. After verifying the checksum, if you choose to run it, select **More info**, then **Run anyway**. Extract the entire zip before starting `designcraft.exe`.

On Windows 11 PCs where Smart App Control is turned On, Windows can block an unsigned app completely, with the message "An Application Control policy has blocked this file" and no Run anyway option. This happened on the maintainer's own PC to a rebuilt copy of this app, which was blocked the first time and ran normally a few hours later with no change on our side. If you see it, read the Smart App Control section of Known limitations for your options. We plan to apply for free open-source code signing.

## Check your download

Download the zip and its separate `.zip.sha256` file into the same folder. In PowerShell, change the version below to the version you downloaded:

```powershell
$zip = 'NiftyDesign-0.9.0-nifty-alpha.2-windows-x64.zip'
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

Worked on the maintainer's one Windows 11 PC: launching, opening a .pub, editing, saving and reopening, PDF export, printing on one inkjet printer, and Arial and Times New Roman found as installed fonts.

Not verified: other PCs and other printers, the native print dialog and file dialogs, and mouse-driven editing in a full session. Real files often open with warnings and can differ from the original layout. Keep your original files and check the exported PDF. See [Known limitations](https://github.com/srt8hellcat2-ui/niftydesign/blob/main/docs/KNOWN-LIMITATIONS.md).

## Reporting problems

Report issues at https://github.com/srt8hellcat2-ui/niftydesign/issues. Issues are public: attach a file only if everyone may see it, and do not attach personal information. Follow the [sample request guidance](https://github.com/srt8hellcat2-ui/niftydesign/blob/main/docs/SAMPLE-REQUEST.md).
