<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# How to open your old Publisher files

Microsoft stopped making Publisher in October 2026. If you have flyers, newsletters or cards saved as `.pub` files, DesignCraft can open them so you can keep editing them, or turn them into PDFs.

## What you need

- DesignCraft on your computer (Mac, Windows or Linux).
- Your `.pub` files, from Publisher 2002 or later. Most files saved since then are fine. Files from Publisher 97, 98 or 2000 can't be opened yet; DesignCraft will tell you if that's the problem.

You don't need Publisher installed.

## Open a file

1. Start DesignCraft.
2. Choose **File › Open…** (or press **Ctrl+O** on Windows and Linux, **Cmd+O** on a Mac), and pick your `.pub` file.
   You can also drag the file onto the DesignCraft window.
3. Your publication opens with its pages, text, pictures and shapes.

If something couldn't be brought over exactly, a message lists it, for example "Publisher table cell fills and borders were not decoded." Your document still opens; the message just tells you what to check.

## Keep a copy you can edit

NiftyDesign never modifies your original `.pub` file on disk. It opens a converted copy as a new, unsaved document. That copy can look different from what Publisher shows, because some features are lost or simplified, and not every loss gives a warning. Always compare it with your original.

- Choose **File › Save** (Ctrl+S or Cmd+S). For `flyer.pub`, the dialog proposes `flyer.designcraft` in the same folder. You can choose another name or folder. Only the DesignCraft format (`.designcraft`) is offered; your `.pub` stays unchanged.
- Next time, open that DesignCraft file instead of the `.pub`.

## Make a PDF

Choose **File › Export PDF…** and pick where to save it. A PDF opens on any computer or phone and is what most print shops want.

## Convert a folder to PDFs

With the DesignCraft command-line tool installed, run:

```sh
designcraft-cli batch-convert "/path/to/Publisher files" --out "/path/to/converted files"
```

It searches the folder and its subfolders for `.pub` files, opens each through
DesignCraft's normal import command, and writes a PDF and an editable
DesignCraft copy. For `cards/flyer.pub`, the chosen output folder receives
`cards/flyer.pub.pdf` and `cards/flyer.pub.designcraft`. Your source files stay
unchanged. Existing outputs and reports are never overwritten; choose a fresh
output folder for another run.

One `report.csv` lists `file`, `pages`, `objects`, `status`, `warnings`,
`pdf_path` and `failure_reason`. Object counts include group containers and
nested objects; cell text belongs to its table. Failed files have a failure
reason and no PDF path; unknown page/object counts are blank. Review warnings
before printing or sharing the results. CSV fields containing newlines or commas
are quoted for opening in a spreadsheet.

A bad file is recorded and the run continues. Each file has a separate worker
with a 120-second timeout; add `--timeout-seconds 300` for a longer limit. The
command returns an unsuccessful exit status after completing the run if any
file failed. Symbolic links are skipped, and an output folder inside the input
folder is excluded from the search.

## Print your pages

Choose **File › Print…** (Ctrl+P on Windows/Linux, Cmd+P on a Mac). The system print dialog opens. Choose your printer and its options, then print. Cancel closes the dialog without sending a job. You can print before saving a DesignCraft copy.

Linux uses the desktop's print portal; macOS uses Preview's system print panel; Windows uses the system printer dialog. If a print service or dialog is unavailable, DesignCraft reports the problem. You can also **Export PDF…**, open the PDF and print it from your usual PDF viewer.

## Before you print or send it

Look over each page, especially:

- **Lists and spacing.** Bullets, line spacing, indents and paragraph spacing come over. Numbering and custom tab stops are still unsupported.
- **Tables.** Rows, columns, merged cells and text come over as editable tables; cell fills, borders and other cell formatting still need checking.
- **WordArt.** The words, colours and outline come over, but curved or arched WordArt comes out straight.
- **Special lettering.** All caps, small caps, superscript and letter spacing come over. Check any formatting warnings shown when opening your file.
- **Page numbers and dates** inserted by Publisher are left out.

For the full list of what works, see [what works](pub-import-status.md).

## If a file won't open

- **"not a Microsoft Publisher file"**: it's another kind of file with a `.pub` ending (address lists and some other programs use `.pub` too).
- **"damaged Publisher file"**: the file is incomplete or corrupted. Try another copy, such as a backup or the copy you emailed someone.
- **"Publisher 97/98/2000 files are not supported yet"**: the file is too old for DesignCraft to read yet. If you still have Publisher on a computer, open the file there and save it again; newer Publisher versions save in a format DesignCraft can read.
