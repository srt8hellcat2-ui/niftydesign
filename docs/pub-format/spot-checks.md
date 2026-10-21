<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Spot checks against sample files

Three files, decoded by hand from their raw streams using only the rules in these notes. Offsets are within the named stream. `hello-nifty.pub` is in the repo (`crates/pub/tests/data/`); the two others are from the pubsmith fixtures (`PUB_CORPUS`, pubsmith commit `0ccdf27`), which are not in the repo.

## 1. `hello-nifty.pub` (written by "Nifty Publisher Lite", not by Publisher)

### `Contents` (784 bytes)

```
000000  e8 ac 2c 00 e8 03 0a 00 10 03 00 00 1a 00 00 01
000010  00 00 00 00 00 00 00 00 00 00 98 00 00 00 28 00
```
- `E8 AC 2C 00`: Publisher 2002+ layout. At 0x1A: `98 00 00 00` → trailer at 0x98.

```
000098  78 02 00 00 01 20 04 01 00 00 02 20 03 01 00 00
```
- Trailer length 0x278. Key 0x01 kind 0x20 = 0x104 (260 directory entries); key 0x02 kind 0x20 = 0x103; then key 0x03 kind 0x90 = the record directory (at 0xA8).
- The directory (length 0x266) starts with 256 empty slots, each the two bytes `00 78` (key 0x00, kind 0x78, no data). Entry 256:

```
0002ae  00 88 12 00 00 00 02 18 44 00 04 b8 1e 00 00 00
0002be  06 10 02 01
```
- Key 0x00 kind 0x88, length 0x12: key 0x02 kind 0x18 = **0x44 (document)**, key 0x04 kind 0xB8 = **offset 0x1E**, key 0x06 kind 0x10 = 0x102 (not read). So sequence 256 = document record at 0x1E. Entries 257–259 decode the same way to: 257 = page record (0x43) at 0x46, 258 and 259 = object records (0x01) at 0x72 and 0x88.

Document record:
```
00001e  28 00 00 00 01 20 01 00 00 00 02 a0 0a 00 00 00
00002e  00 70 01 01 00 00 12 88 10 00 00 00 01 20 f0 98
00003e  76 00 02 20 a0 7a 99 00
```
- Length 0x28. Key 0x02 kind 0xA0 (page list, length 10) → one entry, key 0x00 kind 0x70 = **page 0x101 (257)**.
- Key 0x12 kind 0x88 (page size): width `F0 98 76 00` = 7 772 400 EMU = **612 pt**; height `A0 7A 99 00` = 10 058 400 EMU = **792 pt**.

Page record (sequence 257):
```
000046  2c 00 00 00 01 20 02 00 00 00 02 a0 10 00 00 00
000056  00 70 02 01 00 00 00 70 03 01 00 00 0e c0 06 00
000066  00 00 00 00 0f c0 06 00 00 00 00 00
```
- Objects: 0x102 (258) and 0x103 (259). Key 0x0E kind 0xC0, length 6 → just the 0x0000 terminator: empty master label, so an **ordinary page**. Only one entry in the page list → no placeholders (rule 3 in `contents-stream.md` §5).

Object record (sequence 258):
```
000072  16 00 00 00 27 20 02 00 00 00 aa 20 80 cf 37 00
000082  ab 20 e0 f3 0d 00
```
- Key 0x27 kind 0x20 = **story 2**. Keys 0xAA/0xAB not read. Sequence 259 has no key 0x27: no text.

### `Quill/QuillSub/CONTENTS` (2 116 bytes)

```
000000  43 48 4e 4b 49 4e 4b 20 04 00 07 00 08 00 00 03
000010  00 02 00 00 44 08 00 00 f8 01 08 00 ff ff ff ff
000020  18 00 54 45 58 54 00 00 01 00 00 00 54 45 58 54
000030  00 02 00 00 40 00 00 00
```
- `CHNKINK `; stream length 0x844 = 2 116. Directory at 0x18: 8 entries, next = 0xFFFFFFFF (last block). First entry: tag `TEXT`, offset 0x200, length 0x40.
- All 8 entries: `TEXT` 0x200/64, `STSH` 0x240/20, `STSH` 0x254/20, `FDPP` 0x400/512, `FDPC` 0x600/512, `SYID` 0x800/12, `FONT` 0x80C/40, `STRS` 0x834/16.

```
000200  48 00 65 00 6c 00 6c 00 6f 00 20 00 66 00 72 00   "Hello fr"
...
000230  65 00 72 00 20 00 4c 00 69 00 74 00 65 00 0d 00   "er Lite" CR
000800  02 00 00 00 01 00 00 00 02 00 00 00               SYID
000834  01 00 00 00 08 00 00 00 ff 00 00 00 20 00 00 00   STRS
```
- `SYID`: count 1, id **2**. `STRS`: count 1, lengths start at 4 + 8 = 12, length **0x20 = 32 units**. So story 2 = "Hello from Nifty Publisher Lite" + CR, which is the object at sequence 258.

```
00080c  00 00 00 00 01 00 00 00 00 00 00 00 00 00 00 00
00081c  00 00 00 00 00 00 00 00 05 00 41 00 72 00 69 00
00082c  61 00 6c 00 00 00 00 00
```
- `FONT`: count 1; entries start at 20 + 4 × 1 = 24 (0x824): length 5, **`Arial`**, then 4 bytes.

```
000600  01 00 01 00 00 00 00 00 40 02 00 00 fc 01 00 00   FDPC page
0007fc  04 00 00 00                                       its format record
```
- One character run ending at stream offset 0x240 = unit (0x240 − 0x200) ÷ 2 = 32, format record at 0x600 + 0x1FC = 0x7FC, length 4 = **no formatting**. The second `STSH` has count 0, so there are no defaults either. `FDPP` is the same shape.
- **Result:** one US Letter page, two objects, one with the text "Hello from Nifty Publisher Lite" in Arial with no other formatting. This matches what the importer test expects for this file.

## 2. `text__run-red-word.pub` (Publisher 2021): colours and character runs

Palette record (`Contents` 0x1AEC):
```
001aec  80 00 00 00 01 20 08 00 00 00 02 a0 5a 00 00 00
001afc  00 78 00 88 0a 00 00 00 01 20 5b 9b d5 00 00 88
```
- Key 0x02 kind 0xA0 (array). Entry 0: `00 78` (empty → black). Entry 1: key 0x00 kind 0x88 length 0x0A → key 0x01 kind 0x20 = `5B 9B D5 00` → **RGB(91, 155, 213)**. Entries 2–7 follow (the last is `FF FF FF` = white).

Quill `PL  ` section (count 3):
```
2a 00 00 00 01 22 00 00 00 08 02 22 ff ff ff ff ...   entry 0: key 0x01 = 0x08000000 → palette colour 0
12 00 00 00 01 22 00 00 00 00 06 82 06 00 00 00 00 00 entry 1: 0x00000000 → black
12 00 00 00 01 22 ff 00 00 00 06 82 06 00 00 00 00 00 entry 2: 0x000000FF → RGB(255, 0, 0)
```

Quill `FDPC` page (0x4C00):
```
04 00 01 00 00 00 00 00 0c 02 00 00 1a 02 00 00 30 02 00 00 32 02 00 00 f0 01 be 01 f0 01 b4 01
```
- 4 runs ending at 0x20C, 0x21A, 0x230, 0x232 → units 6, 13, 24, 25 of "Hello Example Text world" CR.
- Run 2 ("Example", units 6–13) has its record at 0x4C00 + 0x1BE = 0x4DBE, containing `44 8a 0a 00 00 00 00 22 02 00 00 00` → key 0x44 nested list, key 0x00 = **colour table index 2** → red.
- **Result:** "Example" is red, the rest uses the default colour. Matches the sample's name and Publisher's export (the corpus test's text and style checks pass for this file).

## 3. `text__vanchor-middle.pub` and `text__bold.pub` (Publisher 2021): object facts, masters, placeholders

Object record sequence 0x125 in `text__vanchor-middle.pub` contains `27 20 02 00 00 00` (story 2) and `35 20 01 00 00 00` → vertical alignment **1 = middle**. (In `text__vanchor-bottom.pub` the same key holds 2.)

Document record of `text__bold.pub`: page list = 0x107, 0x10A, 0x10D, 0x110, 0x113, 0x117.
- 0x107: page record with key 0x0E = `A` and key 0x0F = `Master Page A` → **master page**.
- 0x10A: page record with an empty key 0x0E, objects [0x125], key 0x0D kind 0x68 = 0x107 → **ordinary page using master A**.
- 0x10D, 0x110, 0x113, 0x117: the four trailing non-master entries; none lists objects → **placeholders**, not imported.

Quill of `text__bold.pub`: `FDPC` runs end at 0x230 and 0x232; both records start `02 0a` → key 0x02 kind 0x0A → **bold**. Second `STSH`: count 0x48; entry 0 at section + 20 + 0x120 + 2 starts `a2 01 00 00 0c 22 18 f0 01 00 …` → default size 0x1F018 = **10 pt**.
- **Result:** one page using master A, one text box, "Hello Example Text world" in bold 10 pt Calibri. Matches the corpus test's truth for this file.
