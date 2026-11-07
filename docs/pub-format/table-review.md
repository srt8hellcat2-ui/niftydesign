<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Table dimension review — independent check (2026-10-19, Claude)

Scope: read-only evidence review of the Codex diagnosis in `tables.md`. Labels: **OBSERVATION** = read from the files; **UNKNOWN** = not settled by these files. Nothing here assigns a meaning to any value. No code, test or task status was changed.

## Inputs

Data only; downloaded one by one from raw.githubusercontent.com into a directory outside the repo; none committed.

| Repo @ commit | File | SHA-256 |
|---|---|---|
| apache/poi @ `bf7c9b6a1490046517cbd8e1504e702a89be4bc9`, `test-data/publisher/` | `Sample.pub` | `6fefdef46b87c767150878dc384549cb2d2ec2ac54de25f8ddb3a5628301107e` |
| | `Sample2.pub` | `c87ea7cc5606023c5fa99376631bbd45a8e9bdaba34c958057995b0e7ac8cc86` |
| | `Sample3.pub` | `424c69173ff08948c2529c8084b4ac2403f1ff1057146f4edd02fc29b44481fc` |
| | `Sample4.pub` | `42195f7ad23d911219fea3ec88e66e867e9b9a6821a16dd1b535e2aa9d57a11b` |
| | `Sample2_2010.pub` | `931ff20980b8f72aab227fa3cbe61b017be8d17c337fecb9aca3c97075940777` |
| | `Sample3_2010.pub` | `baa9555a72d9e4264e3fd722896e2e43b960d20b4afdb96f75cb721a6133ff91` |
| | `Sample4_2010.pub` | `544e2043f91aed664c0a09f9ae45da42291b6730f107f99f3cdf9462ed38fb02` |
| | `Sample_2010.pub` | `f0a90aa566d72eb822eb48b4f378c33007610e35f3f1b999cdb88f2c9683858b` |
| FuncularLabs/pubsmith @ `0ccdf27` (the short hash resolved; no fallback to main), `tests/fixtures/pub/` | `table__2x2.pub` | `72d2127f4ffdbe998f4f94414a28584e49af817991b70a564fda885dcb1a6a7d` |
| | `table__3x2.pub` | `12c45cd9057553f748d0cf23ab5348d901be294b1d6e790eb80c3331d57d2616` |
| | `table__merged.pub` | `3df51f5e3e29f247b8636627725d43f5973dc7ea887338f32aec8f4b49a23b30` |

All 11 hashes equal those already in `SOURCES.md` / `tables.md`.

Method: a throwaway Python script (outside the repo, not committed) that reads the `Contents` stream from the CFB container, walks the property lists as in `property-lists.md`, finds the record directory, and prints every type-0x10 record with the documented keys 0x66, 0x67, 0x68, 0x69 and the 0x6D entries (keys 0x01, 0x02). Each file has exactly one type-0x10 record. Its sequence number and offset also match Codex's.

## (1) Do Codex's numbers reproduce?

**OBSERVATION: yes, for all 11 files**, including sequence numbers, offsets, row/column counts, widths, heights and every stored value. "Ending" = key 0x01, "size" = key 0x02, EMU.

| File | Seq | Offset | Rows × cols | Width / Height | Column (ending, size) | Row (ending, size) | Row ending − running size sum |
|---|---:|---:|---|---|---|---|---|
| Sample.pub | 299 | 8708 | 3 × 2 | 3564000 / 648000 | (1782000, 1782000), (3564000, 1782000) | (219188, 216000), (438376, 216000), (657564, 216000) | +3188, +6376, +9564 |
| Sample2.pub | 299 | 8720 | same | same | same | same | same |
| Sample3.pub | 299 | 8710 | same | same | same | same | same |
| Sample4.pub | 299 | 8710 | same | same | same | same | same |
| Sample2_2010.pub | 299 | 8692 | same | same | same | same | same |
| Sample3_2010.pub | 299 | 8690 | same | same | same | same | same |
| Sample4_2010.pub | 299 | 8690 | same | same | same | same | same |
| Sample_2010.pub | 299 | 8690 | same | same | same | same | same |
| table__2x2.pub | 293 | 7696 | 2 × 2 | 3657600 / 914400 | (1828800, 1828800), (3657600, 1828800) | (457200, 457200), (914400, 457200) | 0, 0 |
| table__3x2.pub | 293 | 7696 | 3 × 2 | 3657600 / 1371600 | same as 2x2 | (457200, 457200), (914400, 457200), (1371600, 457200) | 0, 0, 0 |
| table__merged.pub | 293 | 7702 | 2 × 2 | 3657600 / 914400 | same as 2x2 | same as 2x2 | 0, 0 |

Additional **OBSERVATION**s, beyond Codex's text:
- In the eight Apache files the row endings step by 219188 each (219188, 438376 = 2×219188, 657564 = 3×219188), while each size is 216000. So the ending exceeds the sum of sizes by 3188 per row, not by a fixed amount.
- In all eight, the sum of column sizes equals width 0x68 and the sum of row sizes equals height 0x69. The last row ending (657564) is 9564 above the height 648000.
- In all 11 files the column endings equal the running sums of the column sizes.

## (2) Is the proposed change consistent with all 11 files?

Proposal: use row sizes for layout; stop rejecting a table solely because an ending differs from the running sum of sizes; keep rejecting when the sizes do not add up to the declared width or height; warn when endings differ.

- **OBSERVATION:** the sizes add up to the declared width and height in all 11 files, so the table would not be rejected. The three pubsmith controls have no differing endings, so they would produce no warning and be unchanged. The eight Apache files would import with one warning each. **No counter-example among the 11 files.**
- **OBSERVATION:** the proposal never uses the endings for layout, so the eight files would get a 648000 EMU tall grid while the last stored ending is 657564.
- **UNKNOWN:** whether the sizes or the endings are the correct layout values for the Apache files. All 11 files are consistent with the proposal, but they are also consistent with the opposite choice (using endings for layout and rejecting on a width/height mismatch). Nothing in the 11 files discriminates between them. Also the eight files are near-identical (same grid, same numbers), so they are effectively one sample, not eight independent ones.
- **UNKNOWN:** whether the sizes stay correct when rows differ in height (every table here has equal row heights) or in tables with other row counts.
- **UNKNOWN:** whether comparing against the table frame's height in the drawing stream would discriminate between the two readings. I did not look at it; it was outside the allowed keys for this task.

## (3) Other differences in the table record between the eight Apache files and the three controls

Only the type-0x10 record's own properties and the 0x6D entries were compared. Values are listed without interpretation.

**OBSERVATION:** the 0x6D entries use only keys 0x01 and 0x02, both kind 0x20, in all 11 files. No extra keys, no other kinds, no entry-level difference except the values in (1).

Table record properties (key/kind = value):

| Property | Eight Apache files | Three pubsmith controls |
|---|---|---|
| 0x03 / 0x08 | present (empty) | absent |
| 0x04 / 0x10 | 3 | 259 |
| 0x27 / 0x20 | 6 | 3 |
| 0x6B / 0x70 | 301 | 295 (2x2, 3x2), 296 (merged) |
| 0x70 / 0x20 | 4294967294 (0xFFFFFFFE) | 4294967293 (0xFFFFFFFD) |
| 0xB7 / 0x20 | absent | 0 |
| 0x2B / 0x08, 0x2C / 0x08 | present (empty) in the four `*_2010` files; absent in the other four | absent |

Present and equal in all 11: 0x02/0x08 (empty), 0x0C/0x28 and 0x0D/0x28 (eight zero bytes), 0x2A/0x08 (empty), 0x66, 0x67, 0x68, 0x69, 0x6D (but with different values as in (1)), 0x6B kind 0x70. Key 0x04 and 0x27 values and the sequence numbers are expected to vary by file, and are listed only because they differ. I did not compare the type-0x63 cell-list record, Quill, or Escher data.

## (4) What remains unexplained

- **UNKNOWN:** why the Apache row endings are 3188 higher per row than the sum of sizes, and why the controls have none.
- **UNKNOWN:** whether the eight files are valid and Publisher itself reads them with the stored sizes, the stored endings, or something else. No export or rendering of these tables was available.
- **UNKNOWN:** which of the property differences in (3) (if any) relate to the endings. A difference between files does not show relevance.
- **Unverified hypothesis (not supported by these files):** the 3188 EMU gap might come from something the sample tables have and the controls lack (for example a cell or border setting). I looked for no evidence of this and have none.

Controlled sample files that would settle it (made with the same tool, one change at a time, with the source document available):
1. The same 3×2 table with only the border width changed (none, thin, thick), and with borders on and off.
2. The same table with different row heights (e.g. 216000 / 300000 / 400000 EMU) and a table whose rows have different heights from each other.
3. The same table with different row counts (1, 2, 4, 5) to see whether the per-row excess is constant or scales.
4. The same table with different cell margins and with text in the cells, to see whether content moves the ending.
5. One of these tables saved by the same Publisher version as the Apache files and again by another version, to separate version from content.
6. Ideally, each file with a Publisher PDF or PNG export, so the real row positions can be measured and compared with sizes and endings.

## Disallowed material

I needed nothing that was not allowed. I did not read any other .pub reader, its source or documentation, did not use web search for table-row behaviour, and did not open the type-0x63, Quill or Escher data for this review.
