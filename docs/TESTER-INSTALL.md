<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# tester instructions for the command-line build

**Status: draft. The owner decides when (and whether) to share this.** Facts come from [open-publisher-files.md](open-publisher-files.md), [REPORTS.md](REPORTS.md) (C10, C11) and [ALPHA-NOTES.md](ALPHA-NOTES.md). Nothing here was run for this draft.

**Unverified:** no hosted build has been run yet. REPORTS.md says the Windows build workflow ("Publisher tester CLI") was written and checked locally on Linux, but "no hosted workflow run, Windows/macOS build or hosted artifact was produced or verified". The download steps below depend on the owner running it first and giving you the file.

## What this is

An **unofficial** test build of DesignCraft's command-line tool, `designcraft-cli`, for Windows. It reads Microsoft Publisher (`.pub`) files and writes a PDF and an editable DesignCraft copy of each. It is not an official release, it is not supported by the ArtCraft team, and it is not signed (see "Windows warnings" below).

It never changes your original files.

## Before you start

- Use a folder of `.pub` files that you are happy to run through a test. Publisher 2002 or later files work best. Publisher 97, 98 and 2000 files are refused with a message.
- Work on copies if you like; the tool only reads your originals.

## Steps

1. **Get the build.** The owner will send you a download (the build is named `designcraft-cli-windows`, containing `designcraft-cli.exe`). Unzip it to a folder, for example `C:\designcraft-test`.
2. **Open a command window.** Press the Windows key, type `cmd`, press Enter.
3. **Run the converter.** Type the line below, changing the two folder paths. Put the paths in quotes. Choose an output folder that does **not exist yet**; the tool never overwrites earlier output.

   ```
   C:\designcraft-test\designcraft-cli.exe batch-convert "C:\My Publisher Files" --out "C:\designcraft-output"
   ```

4. **Wait.** It searches the folder and its subfolders for `.pub` files. Each file is handled separately with a 120-second limit, and a bad file is recorded and skipped. For a longer limit add `--timeout-seconds 300`. When it finishes, it reports an unsuccessful exit status if any file failed; that is expected for some files.
5. **Find the report.** Open the output folder and look for **`report.csv`**. You can open it in a spreadsheet program. Its columns are `file`, `pages`, `objects`, `status`, `warnings`, `pdf_path` and `failure_reason`.
6. **Optionally look at the PDFs.** Each `.pub` produces a `.pub.pdf` and a `.pub.designcraft` in the matching subfolder of the output folder. Look over a few pages against the original if you can.

## What to send back

- **Send:** `report.csv` only. If you want, add a few of the PDFs, **but only for files that contain nothing private.**
- **Check `report.csv` before sending.** It contains your file names and folder names (and warning text). If a file or folder name has a person's name or other private detail, remove or rename that row, or tell the owner instead.
- **Do not send:** your original `.pub` files, the `.designcraft` copies, or any PDF made from a document with personal information (names, addresses, phone numbers, photos of people, financial details, membership or contact lists). Do not send anything from a church, school or club list of members.
- Also say: your Windows version, and whether the tool crashed or hung (and what you did just before).

## Windows warnings

The build is not signed. Windows may show a "Windows protected your PC" screen or a similar warning. This is **unverified**: the repo's docs do not record what Windows shows for this build. If you trust the person who gave it to you, the usual choice is "More info" then "Run anyway". If you are unsure, do not run it and ask the owner. The owner may give you a `SHA256SUMS` file to check the download; this is also unverified for the CLI-only build.

## What the report tells you

- A status of **OK** means it opened with no warnings. **Warnings** means it opened, but the tool lists what it could not bring over fully (for example table shading, page numbers). **Refused** means the file is Publisher 97/98/2000. **Failed** means it could not be opened (the `failure_reason` column says why).
- In the owner's check of 24 real files from other programs, none opened with no warnings, 19 opened with warnings, 2 were refused as too old and 3 failed (those 3 were deliberately damaged test files). Your results may differ.
- A warning is not proof something is wrong, and no warning is not proof the file is perfect. See [ALPHA-NOTES.md](ALPHA-NOTES.md).

## Credit

Based on DesignCraft by the ArtCraft team. Unofficial build; not supported by ArtCraft.
