# Table formatting observations

## Scope and baseline

OBSERVATION: notes from main `104ad73`, using only the pinned 142 controlled binaries and 24 Apache binaries, the existing container, Escher and Quill readers, and retained format notes. All 317 recorded sizes and SHA-256 values match: 166 binaries, 142 scoring JSON files and nine reference PNGs. Scoring data supplies no storage interpretation. Data remains outside the checkout in `/tmp/niftydata`.

OBSERVATION: the untouched-main gate is 142/142 imports, 152/152 objects, 111/111 scored shape/text positions within 1 pt, 52/52 exact texts and 111/111 styles. The picture check separately reports 12/41 matched passes and 29 unmatched references. The real batch is 9 with notes, 10 with warnings, 2 refused old-family files and 3 rejected damaged files, 31 pages and 280 objects.

## Inventory and representation

OBSERVATION: temporary read-only diagnostics stayed outside the checkout. The public importer supplied native tables and composed row positions; the existing bounded readers supplied property values. No parser, importer or renderer was edited.

OBSERVATION: the current Escher parser returns the following eleven type-201 table frames. The Contents table objects are type 0x10; row/column counts use keys 0x66/0x67, dimensions 0x6D, and cell-list links 0x6B to type-0x63 records. Quill TCD boundaries partition table text. Counts below agree with the fresh Contents scan and retained table notes; each Apache cell list has six unmerged rectangles. The fresh Contents record-directory scan also finds exactly these eleven type-0x10 table objects. No other successfully readable binary has a type-201 table frame. UNKNOWN: damaged and refused binaries cannot be certified table-free.

| File | Tables | Rows × columns | Cell records | Contents sequence | Import report |
| --- | ---: | --- | ---: | ---: | --- |
| table__2x2.pub | 1 | 2 × 2 | 4 | 293 | 1 page, 1 item, formatting warning |
| table__3x2.pub | 1 | 3 × 2 | 6 | 293 | 1 page, 1 item, formatting warning |
| table__merged.pub | 1 | 2 × 2 | 3 | 293 | 1 page, 1 item, formatting warning |
| Sample.pub | 1 | 3 × 2 | 6 | 299 | 2 pages, 7 items, 2 warnings |
| Sample2.pub | 1 | 3 × 2 | 6 | 299 | 2 pages, 7 items, 2 warnings |
| Sample2_2010.pub | 1 | 3 × 2 | 6 | 299 | 2 pages, 7 items, 2 warnings |
| Sample3.pub | 1 | 3 × 2 | 6 | 299 | 2 pages, 7 items, 2 warnings |
| Sample3_2010.pub | 1 | 3 × 2 | 6 | 299 | 2 pages, 7 items, 2 warnings |
| Sample4.pub | 1 | 3 × 2 | 6 | 299 | 2 pages, 7 items, 2 warnings |
| Sample4_2010.pub | 1 | 3 × 2 | 6 | 299 | 2 pages, 7 items, 2 warnings |
| Sample_2010.pub | 1 | 3 × 2 | 6 | 299 | 2 pages, 7 items, 2 warnings |

OBSERVATION: the importer constructs a native Table anchored in a host story, with row-major cells, individual cell stories and owner-cell spans. It does not create a separate Escher shape per cell. The merged control has a top-row two-column span and empty text; nonempty merged-cell association remains UNKNOWN. Cells have unpainted fills and invisible edges, with a warning that formatting is not decoded. Apache tables also warn that stored endings supplied row positions, maximum difference 9564 EMU, cause unknown.

## Candidate properties

OBSERVATION: these are merged property values returned by the existing Escher parser, not a new raw-property decoder. All three controlled frames have identical properties; all eight Apache frames have identical properties. There are 13 controlled cell records and 48 Apache cell records. Frame properties are counted as frames/files, **not** as observations on 61 individual cells. No per-cell Escher property count is established.

| Property or field | Controlled values | Apache values | Files/frames present; cell count | Label and limit |
| --- | --- | --- | --- | --- |
| 0x0080 | 3 | 6 | 11; no cell attribution | OBSERVATION: story identifier in existing mapping; differs between sets. |
| 0x0081–0x0084 | all 0 | all 0 | each in 11; no cell attribution | INFERENCE: MS-ODRAW dxTextLeft, dyTextTop, dxTextRight, dyTextBottom define shape text margins, not proof of cell padding. |
| 0x017F | 61865984 | 3145728 | 11; no cell attribution | OBSERVATION: differing geometry boolean word; UNKNOWN cell-format relevance. |
| 0x01C0 / 0x01C2 | 134217728 / 134217735 | same | each in 11; no cell attribution | OBSERVATION: unchanged frame values; UNKNOWN per-cell use. |
| 0x01CB | 25400 | absent | 3; no cell attribution | OBSERVATION: frame value only; cannot establish cell-border width. |
| 0x01FF | 4718592 | 4718656 | 11; no cell attribution | OBSERVATION: differing boolean word; UNKNOWN per-cell use. |
| 0x0201 | 134217728 | 134217732 | 11; no cell attribution | OBSERVATION: differing frame value; UNKNOWN cell colour relationship. |
| 0x0384–0x0387 | all 36576 | all 36576 | each in 11; no cell attribution | UNVERIFIED HYPOTHESIS: numeric similarity to cell fields might relate to padding; no differential evidence. |
| 0x0540/0x0580/0x05C0/0x0600/0x0640 | all 134217728 | same | each in 11; no cell attribution | OBSERVATION: repeated values; UNKNOWN semantics here. |
| 0x0542/0x0582/0x05C2/0x0602/0x0642 | all 134217735 | same | each in 11; no cell attribution | OBSERVATION: repeated values; UNKNOWN semantics here. |
| 0x054B/0x058B/0x05CB/0x060B | all 0 | same | each in 11; no cell attribution | OBSERVATION: no differing border variant. |
| 0x06FF | 131074 | same | 11; no cell attribution | OBSERVATION: unchanged boolean word; UNKNOWN cell relevance. |
| 0x0180–0x0183 | absent | absent | 0 of 11 frames; no cell attribution | OBSERVATION: none returned; absence does not locate cell fill storage. |
| Escher complex properties | none | none | 0 of 11 frames; no cell attribution | OBSERVATION: no returned complex payload on a table frame. |
| Contents cell 0x09, kind 0x20 | 228180 | 219188 in column 0; absent in column 1 | 3 files / 13 cells at 228180; 8 files / 24 cells at 219188; absent in 24 Apache cells | OBSERVATION: not a uniformly stored field. UNKNOWN meaning. |
| Contents cell 0x0A–0x0D, kind 0x20 | each 36576 | each 36576 | each in 11 files / 61 cells | OBSERVATION: constant; UNKNOWN padding interpretation. |
| Contents cell 0x0E, kind 0x20 | 114300 | 114300 | 11 files / 61 cells | OBSERVATION: constant; UNKNOWN meaning. |

### Quill block audit

OBSERVATION: the eleven MCLD sections contain 101 length-delimited formatting blocks: 4, 6 and 3 in the controls, and 11 in each Apache file. Each Apache section contains five one-block groups and one six-block group. Calling every MCLD block a table cell would overcount by 40. The six-block groups have field 0x04 = 1782000 and 0x05 = 219188; these match table column widths and ending-derived row heights. Their position after the first five groups also agrees with table story id 6. INFERENCE: these are the candidate table-cell blocks, 48 across eight files. UNKNOWN: a general MCLD-to-story/cell association rule, particularly for nonempty merges, is not established by this correspondence.

OBSERVATION: the following counts cover the 13 controlled blocks and 48 candidate Apache table blocks, all 130 bytes including their four-byte record lengths. A block count is not an independently confirmed cell association. Numeric fields are kind 0x22 unless stated. The existing bounded property-list reader produced the fields; section offsets and lengths came from the documented Quill directory. The grouping described here is an observed bounded traversal of these files, not a new importer rule.

| MCLD field | Values observed | Files / candidate table blocks present | Label and limit |
| --- | --- | --- | --- |

| 0x00 | 2473, 2478, 2483, 7299, 7314, 7329, 7889, 7923, 7957, 11301, 11324, 11347 | 11 / 61 | OBSERVATION: varies within and between files; UNKNOWN interpretation. |
| 0x01 | 2476, 2516, 7308, 7429, 7974, 8110, 11315, 11502 | 11 / 61 | OBSERVATION: varies within and between files; UNKNOWN interpretation. |
| 0x02 | 2477, 2482, 2487, 7313, 7328, 7343, 7922, 7956, 7990, 11323, 11346, 11369 | 11 / 61 | OBSERVATION: varies within and between files; UNKNOWN interpretation. |
| 0x03 | 2515, 2556, 7428, 7549, 8109, 8245, 11501, 11688 | 11 / 61 | OBSERVATION: varies within and between files; UNKNOWN interpretation. |
| 0x04 | 1782000 (8 files/48 blocks), 1828800 (3/12), 3657600 (1/1) | 11 / 61 | INFERENCE: width candidate, matching six unmerged Apache columns and the control merged width; no fill/border meaning. |
| 0x05 | 219188 (8/48), 457200 (3/13) | 11 / 61 | INFERENCE: height candidate, matching stored endings in Apache and control row heights. |
| 0x06–0x09 | each 36576 | each 11 / 61 | OBSERVATION: constant and numerically equal to Contents 0x0A–0x0D and Escher 0x0384–0x0387. UNVERIFIED HYPOTHESIS: related padding fields; side order and precedence UNKNOWN. |
| 0x0A, 0x0D, 0x11, 0x16, 0x18 | each 0 | each 11 / 61 | OBSERVATION: no differential evidence for a paint meaning. |
| 0x0B, kind 0x1A | 0 | 11 / 61 | OBSERVATION; UNKNOWN meaning. |
| 0x12 | 219456000 | 11 / 61 | OBSERVATION; UNKNOWN meaning. |
| 0x13, kind 0x12 | 255 (3 files/13 blocks), 41727 (8/48) | 11 / 61 | OBSERVATION: differs between sets; UNKNOWN meaning. |
| 0x14 | no payload, kind 0x0A (6 files/31 blocks) or 0x02 (5/30) | 11 / 61 | OBSERVATION: differs within/between files; UNKNOWN meaning. |
| 0x15 | 1 | 11 / 61 | OBSERVATION; UNKNOWN meaning. |
| 0x1A, kind 0x02 | no payload | 11 / 61 | OBSERVATION; UNKNOWN meaning. |
| 0x1D, kind 0x8A; child 0x00 kind 0x22 | child value 4294967292 | 11 / 61 | OBSERVATION: same nested field in all candidates; UNKNOWN meaning. |

UNKNOWN: cell fill, each edge's colour/width/style, padding precedence and style inheritance cannot yet be mapped. UNVERIFIED HYPOTHESIS: MCLD may hold these settings; its cell-block count makes it a candidate, not a confirmed encoding. INFERENCE: drawing frame margin definitions alone do not license applying those margins to every table cell. The published margin names and source pages are recorded in [text-fit.md](text-fit.md) and [SOURCES.md](SOURCES.md).

## Row positions

OBSERVATION: no Apache scoring export or Publisher reference image is supplied by the recorded input recipe. All eight measured per-row differences against such an export are therefore UNKNOWN. The nine available PNGs contain no table. The three controlled scoring JSONs contain flattened whole-table images, not row coordinates. INFERENCE: unchanged corpus totals therefore do not validate cell paint or individual row accuracy.

| Apache file | Row 1 ours − export (pt) | Row 2 (pt) | Row 3 (pt) | Grows with row index? |
| --- | --- | --- | --- | --- |
| Sample.pub | UNKNOWN | UNKNOWN | UNKNOWN | UNKNOWN |
| Sample2.pub | UNKNOWN | UNKNOWN | UNKNOWN | UNKNOWN |
| Sample2_2010.pub | UNKNOWN | UNKNOWN | UNKNOWN | UNKNOWN |
| Sample3.pub | UNKNOWN | UNKNOWN | UNKNOWN | UNKNOWN |
| Sample3_2010.pub | UNKNOWN | UNKNOWN | UNKNOWN | UNKNOWN |
| Sample4.pub | UNKNOWN | UNKNOWN | UNKNOWN | UNKNOWN |
| Sample4_2010.pub | UNKNOWN | UNKNOWN | UNKNOWN | UNKNOWN |
| Sample_2010.pub | UNKNOWN | UNKNOWN | UNKNOWN | UNKNOWN |

OBSERVATION: fresh bounded property reads reproduce [tables.md](tables.md): all eight have row sizes 216000 EMU each and endings 219188, 438376, 657564 EMU. At 12700 EMU/pt, ending-minus-size-sum is 0.251023622, 0.502047244, 0.753070866 pt. Top-of-row offsets relative to size-only arithmetic are 0, 0.251023622, 0.502047244 pt. These are stored-value comparisons, **not measured errors against Publisher**. The table frame height is exactly 657564 EMU, matching the last ending; the declared size total is 648000 EMU. The control frames match both sums and endings.

UNVERIFIED HYPOTHESIS: a constant per-row contribution explains the stored difference. Confirm a broader rule with unequal-height and different-border tables; kill a universal rule if their differences vary independently. OBSERVATION: the existing eight-file measurements support a constant 3188 EMU increment only within this related family.

UNVERIFIED HYPOTHESIS: a border contributes that increment. UNKNOWN: no paired border-only variants establish this. OBSERVATION: 0x01CB is absent from all eight Apache table frames, and all three controls have the same explicit 25400 value while their ending/size differences are zero. INFERENCE: frame line width alone cannot establish a universal 3188-EMU cell-border contribution; a border-only pair with unchanged endings would kill that local explanation. Changing row arithmetic on this basis would guess an encoding.

UNVERIFIED HYPOTHESIS: native AtLeast row expansion could add a further displacement. INFERENCE: compare configured heights with composed row positions while keeping text/fonts fixed; equality would kill expansion as the cause for that row. OBSERVATION: this cheap check was run through the unchanged public importer and compositor on all eleven tables. Every Apache configured height is 17.258897637795275 pt; every composed row has that height to floating-point precision. All three control tables similarly retain their configured 36 pt rows. INFERENCE: extra content-driven row growth is ruled out for these files on this machine, but not for other fonts or documents.

OBSERVATION: each of the eight Apache tables has the same composed row tops shown below, measured in the existing frame inner coordinate space. These are ours only; no scoring-export values were substituted.

| File | Row 1 top (pt) | Row 2 top (pt) | Row 3 top (pt) | Extra growth beyond configured heights (pt) |
| --- | ---: | ---: | ---: | --- |
| Sample.pub | 221.102362205 | 238.361259843 | 255.620157480 | 0, 0, 0 |
| Sample2.pub | 221.102362205 | 238.361259843 | 255.620157480 | 0, 0, 0 |
| Sample2_2010.pub | 221.102362205 | 238.361259843 | 255.620157480 | 0, 0, 0 |
| Sample3.pub | 221.102362205 | 238.361259843 | 255.620157480 | 0, 0, 0 |
| Sample3_2010.pub | 221.102362205 | 238.361259843 | 255.620157480 | 0, 0, 0 |
| Sample4.pub | 221.102362205 | 238.361259843 | 255.620157480 | 0, 0, 0 |
| Sample4_2010.pub | 221.102362205 | 238.361259843 | 255.620157480 | 0, 0, 0 |
| Sample_2010.pub | 221.102362205 | 238.361259843 | 255.620157480 | 0, 0, 0 |

OBSERVATION: the composed bottom is 272.879055118 pt; subtracting the first row top gives 51.776692913 pt, matching the Escher frame height and stored final ending. The 9564 EMU difference from the declared size total is 0.010459318 inch. INFERENCE: that is a numerical match to the approximate historical 0.01-inch description, not proof that it is an error relative to Publisher.

UNVERIFIED HYPOTHESIS: a constant placement offset rather than per-row growth could explain a visual difference. INFERENCE: compare the first row and later rows with an independent scoring export; equal signed offsets would support it, growing offsets would kill a constant-only explanation. OBSERVATION: our composed relative row tops exactly equal 0, 219188/12700 and 438376/12700; no additional per-row increment occurs after reading the stored endings. UNKNOWN: without an Apache scoring export, neither the native-layout comparison nor stored-size arithmetic identifies the reported visual discrepancy's cause.

## Samples needed

INFERENCE: request 17 small publications, each with an owner-provided scoring export and a reference image; use distinct short labels in every cell and record the intended settings.

- INFERENCE: 1 baseline 3 × 2 table with equal rows and columns, no fill or border, explicit equal padding.
- INFERENCE: 2 baseline copies changing only one cell fill to two different solid colours.
- INFERENCE: 4 baseline copies changing only the top, right, bottom or left edge of one cell.
- INFERENCE: 2 baseline copies changing only that edge's width or dash style.
- INFERENCE: 4 baseline copies changing only one cell's left, top, right or bottom padding.
- INFERENCE: 1 unequal-row-height table with otherwise baseline settings.
- INFERENCE: 1 long-text table that forces a row to expand.
- INFERENCE: 2 labelled merged-cell tables, one horizontal and one vertical.

## Recommendation and validation limits

INFERENCE: **wait for samples** before implementing cell paint or a row-drift correction. The main risk is treating an opaque per-cell field or frame-level property as a confirmed cell setting, or correcting a displacement that lacks a scoring reference.

INFERENCE: allow roughly 2–4 working days for a later bounded implementation and regression checks after differentiated samples establish mappings; allow another 1–2 days for sample analysis. UNKNOWN: effort for style inheritance, shared edges, merged-cell text and layout expansion may exceed that estimate.

OBSERVATION: this note changes no importer or renderer behavior. Final controlled results remain 142/142 imports, 152/152 objects, 111/111 scored shape/text positions within 1 pt, 52/52 texts and 111/111 styles. The separate picture result remains 12/41 matched passes and 29 unmatched references. All 142 native save/reopen/edit/PDF cases pass. The real-file batch regression passes with the same 9 notes, 10 warnings, 2 refused and 3 damaged files; the import census still totals 31 pages and 280 objects.

OBSERVATION: formatting and lint checks pass. All seven workspace checks pass on the sequential rerun, including 3103 tests passed, zero failed and 25 ignored, plus asset, layer and WebAssembly checks. The first workspace run stopped at three batch process-launch checks with `Os { code: 2, kind: NotFound, message: "No such file or directory" }`. Each passes in the sequential rerun without a source change. UNKNOWN: the exact initial launch-failure cause; that run overlapped other builds, but overlap alone does not establish causation.

OBSERVATION: the recorded recipe rebuilt `/tmp/niftydata/pixel-report` and current `/tmp/niftydata/pixel-snapshots/table-notes` and `table-notes-final`, including complete manifests. All nine reference scores match the last merged font-width report within 1e-9, with no size mismatch. Current own-output comparison reports 142 compared, zero changed and zero added/removed. UNKNOWN: the original machine's historical snapshot files/manifests are unavailable; these current regenerated runs do not recover those historical bytes. Neither current nor historical snapshot comparisons provide a Publisher scoring export for the eight Apache tables.

OBSERVATION: only this note and the source-use appendix changed. No environment configuration, setup script or setting was changed; no release action was run.
