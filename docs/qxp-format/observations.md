<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# QuarkXPress documents: what the files show (2026-11-28)

Method: black-box reading of three donated QuarkXPress documents and the PDFs that QuarkXPress 7.01 exported from them. No QuarkXPress program was run, no installation or code was read, and no third-party reader (open or not) was consulted. The donated files are private; they are not in this repository and nothing below quotes their contents. The PDFs are used to score what we recover (do the recovered paragraphs match the exported ones), not to infer how anything is encoded.

## Container (QuarkXPress 7.x family)

- The file starts `00 00` then the ASCII marker `MMXPR3` (big-endian "Motorola" form), followed by a few small fields; the rest of the first 1 KB block is zero. The file length is a multiple of 1024: the file is a sequence of 1024-byte blocks.
- Block 1 is a table of big-endian u32 numbers. It begins with a few small values and then holds several lists of block numbers (ascending runs such as 2, 3, 4, ... and stepped runs such as 0x29, 0x2b, 0x2d ...). Some block numbers appear in more than one list, so the lists are not simply "blocks in use". What each list means is not known.
- Names are UTF-16 big-endian: colours (White, Black, Cyan, Magenta, Yellow, Registration, web colours), colour-profile names (a "QuarkXPress 7.0 Default" set), and an embedded job-jacket in XML with the file's path on the author's disk.
- A built-in table holds the default paper size as 612.0 (fixed point 0x02640000) at the same offsets in all three files; it is not the document's page.

## Story text

- Story text is plain UTF-16BE, paragraph breaks are 0x000D, and each story starts at the first byte of its own 1 KB block, followed by zeros. A story longer than 1 KB continues in later blocks.
- The length of a story is not marked where the text is. A block can hold the current text followed by leftover characters of an earlier, longer version (a headline followed by a repeated tail, a sentence with a few words from before an edit). A later block can hold the clean copy. So a reader must know which blocks are live; reading every printable run returns stale text too.
- One document that prints ten identical cards stores each card's two stories in separate blocks, ten times.

## What is not readable

- Page and frame geometry (positions, sizes), and the font list, do not appear as plain numbers. The blocks that should hold them look random: all 128 eight-byte words of such a block differ, no words are shared between the three documents, and none of DES, Blowfish, IDEA, CAST5, TEA or XTEA with a key taken from any offset in the first 8 KB of the file turned one of them into structured data. This agrees with a public issue discussion (designcraft #73) which reports a block cipher with a per-document key. We stop here: recovering geometry would need the cipher and key, which the files alone do not give, and is a decision for the maintainers (the Craft clean-room standard puts numbers measured from the app's exports under "needs a decision").
- Embedded previews of placed pictures are the other high-entropy blocks; they are zlib data and have not been opened yet.

## Plan

A recovery import like the Publisher 97/98/2000 one: the live stories as text with their paragraph breaks, the colour and style names, and the embedded pictures, in a new document with a notice that the layout is not recovered. Score each import against the exported PDFs by text.
