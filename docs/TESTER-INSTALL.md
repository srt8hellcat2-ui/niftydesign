<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Installing the NiftyDesign alpha (Windows)

NiftyDesign is an unofficial alpha that opens older Microsoft Publisher (.pub) files. It is not made by, supported by or affiliated with Microsoft or ArtCraft. Read docs/KNOWN-LIMITATIONS.md first so you know what to expect.

## What you need

- Windows 10 or 11, 64-bit.
- The zip from the latest release on the Releases page of this repository: `NiftyDesign-<version>-windows-x64.zip`, and its `.sha256` file.

## Steps

1. **Download** the zip and the `.sha256` file from the release.
2. **Check the download (optional but recommended).** In PowerShell, in the folder you saved them to:
   `Get-FileHash .\NiftyDesign-<version>-windows-x64.zip -Algorithm SHA256`
   The hash must match the one in the `.sha256` file. If it does not, delete the zip and do not run it.
3. **Unzip** to a folder you own (for example your Documents folder). Do not run it from inside the zip.
4. **Run `designcraft.exe`.** This is the NiftyDesign app (the file name comes from the DesignCraft project it is based on).
5. **Windows will warn you.** The alpha is not code-signed, so SmartScreen shows "Windows protected your PC". Choose **More info**, then **Run anyway**. If you are not comfortable doing that, do not run it. We plan to sign later releases.
6. **Open a file** with File > Open and choose a `.pub` file. Work on a copy of anything important. Save As a `.designcraft` file to keep editing; export a PDF to share.

## What is and is not verified

- Opening and editing real .pub files: tested on a limited set of files. Most real files open with warnings. Publisher 97, 98 and 2000 files are refused on purpose.
- Printing on a real printer and use of your Windows fonts are **not verified**. If a font is missing, a substitute is used and text may shift. Export a PDF if you need to print.
- This is an alpha. It can crash and files can look wrong. Keep your original files.

## Reporting a problem

Open an issue at https://github.com/srt8hellcat2-ui/niftydesign/issues using the bug or "file failed to open" form. Issues are public, so do not post anything private. Attaching your .pub is optional and only if everyone is welcome to see it (docs/SAMPLE-REQUEST.md). Never report problems to the ArtCraft projects.
