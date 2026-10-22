<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# NiftyDesign unsigned alpha — draft release notes

NiftyDesign is an unofficial alpha that opens older Microsoft Publisher (.pub)
files, lets you edit their content, and exports PDF. It is based on DesignCraft
by the ArtCraft team. It is not affiliated with or supported by ArtCraft or
Microsoft. This download is for Windows x64.

## Unsigned download and checksum

The app is not signed. Windows SmartScreen will warn. After verifying the
checksum, if you choose to run it, select **More info**, then **Run anyway**.
Extract the entire zip before starting `designcraft.exe`.

Download the zip and its separate `.zip.sha256` file into the same folder.
In PowerShell, change the version below to the version you downloaded:

```powershell
$zip = 'NiftyDesign-0.9.0-nifty-alpha.1-windows-x64.zip'
$line = (Get-Content -LiteralPath "$zip.sha256").Trim()
$hash, $filename = $line -split '  ', 2
if ($filename -ne $zip -or $hash -notmatch '^[0-9a-fA-F]{64}$' -or
    (Get-FileHash -Algorithm SHA256 -LiteralPath $zip).Hash -ne $hash) {
    throw 'Checksum mismatch; do not run or extract this download'
}
'Checksum matches'
```

The sidecar verifies the finished zip before extraction. `SHA256SUMS` inside the
zip lists the hashes of its other files; it cannot hash itself. A checksum
checks integrity, not the identity of the sender; it is not a signature.

## Verified and unverified

Automated checks cover opening the controlled synthetic sample, saving/reopening
it and exporting a non-empty PDF. The release workflow itself is **UNTESTED until
the maintainer dispatches it**; local packaging tests use executable stand-ins,
not a real Windows build.

Printing on a real printer and Windows system fonts are **UNVERIFIED**. Publisher
97/98/2000 files are refused. Real files often open with warnings and can differ
from the original layout. Keep your original files and check the exported PDF.
See [Known limitations](https://github.com/srt8hellcat2-ui/niftydesign/blob/main/docs/KNOWN-LIMITATIONS.md).

## Reporting problems

Report issues at https://github.com/srt8hellcat2-ui/niftydesign/issues.
Issues are public: attach a file only if everyone may see it, and do not attach
personal information. Follow the
[sample request guidance](https://github.com/srt8hellcat2-ui/niftydesign/blob/main/docs/SAMPLE-REQUEST.md).
