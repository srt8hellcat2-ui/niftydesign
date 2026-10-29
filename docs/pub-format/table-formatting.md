# Table formatting observations

## Scope and baseline

OBSERVATION: notes from main `104ad73`, using only the pinned 142 controlled binaries and 24 Apache binaries, the existing container, Escher and Quill readers, and retained format notes. All 317 recorded sizes and SHA-256 values match: 166 binaries, 142 scoring JSON files and nine reference PNGs. Scoring data supplies no storage interpretation. Data remains outside the checkout in `/tmp/niftydata`.

OBSERVATION: the untouched-main gate is 142/142 imports, 152/152 objects, 111/111 scored shape/text positions within 1 pt, 52/52 exact texts and 111/111 styles. The picture check separately reports 12/41 matched passes and 29 unmatched references. The real batch is 9 with notes, 10 with warnings, 2 refused old-family files and 3 rejected damaged files, 31 pages and 280 objects.

## Inventory and representation

OBSERVATION: the current Escher parser returns the following eleven type-201 table frames. The Contents table objects are type 0x10; row/column counts use keys 0x66/0x67, dimensions 0x6D, and cell-list links 0x6B to type-0x63 records. Quill TCD boundaries partition table text. Counts below agree with the retained table notes. No other successfully readable binary has a type-201 table frame. UNKNOWN: damaged and refused binaries cannot be certified table-free.

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
| Escher complex properties | none | none | 0 of 11 frames; no cell attribution | OBSERVATION: no returned complex payload on a table frame. |
| Contents cell 0x09, 0x0A–0x0D, 0x0E | retained notes: 228180, 36576 each, 114300 | UNKNOWN pending direct field audit | 3 files / 13 cells in retained notes | OBSERVATION of prior measurements; UNKNOWN meanings. |
| Quill MCLD blocks | retained notes: 130 bytes per block, fields 0x00–0x03 near 0x1F00, 0x06–0x09 = 36576 | UNKNOWN pending direct block audit | 3 files / 13 blocks in retained notes | UNKNOWN fill/border/padding mapping; differing opaque blocks do not establish it. |

UNKNOWN: cell fill, each edge's colour/width/style, padding precedence and style inheritance cannot yet be mapped. UNVERIFIED HYPOTHESIS: MCLD may hold these settings; its cell-block count makes it a candidate, not a confirmed encoding. INFERENCE: drawing frame margin definitions alone do not license applying those margins to every table cell. The published margin names and source pages are recorded in [text-fit.md](text-fit.md) and [SOURCES.md](SOURCES.md).

## Row positions

OBSERVATION: no Apache scoring export or Publisher reference image is supplied by the recorded input recipe. All eight measured per-row differences against such an export are therefore UNKNOWN. The nine available PNGs contain no table. The three controlled scoring JSONs contain flattened whole-table images, not row coordinates.

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

OBSERVATION from retained [tables.md](tables.md): all eight have row sizes 216000 EMU each and endings 219188, 438376, 657564 EMU. At 12700 EMU/pt, ending-minus-size-sum is 0.251023622, 0.502047244, 0.753070866 pt. Top-of-row offsets relative to size-only arithmetic are 0, 0.251023622, 0.502047244 pt. These are stored-value comparisons, **not measured errors against Publisher**. The table frame height is exactly 657564 EMU, matching the last ending; the declared size total is 648000 EMU. The control frames match both sums and endings.

UNVERIFIED HYPOTHESIS: a constant per-row contribution explains the stored difference. Confirm a broader rule with unequal-height and different-border tables; kill a universal rule if their differences vary independently. OBSERVATION: the existing eight-file measurements support a constant 3188 EMU increment only within this related family.

UNVERIFIED HYPOTHESIS: a border contributes that increment. UNKNOWN: no paired border-only variants establish this. Changing row arithmetic on this basis would guess an encoding.

UNVERIFIED HYPOTHESIS: native AtLeast row expansion could add a further displacement. INFERENCE: compare configured heights with composed row positions while keeping text/fonts fixed; equality would kill expansion as the cause for that row. UNKNOWN: without an Apache scoring export, neither a native-layout comparison nor a stored-size calculation identifies the reported visual discrepancy's cause.

## Samples needed

INFERENCE: request 15 small publications, each with an owner-provided scoring export and a reference image; use distinct short labels in every cell and record the intended settings.

- INFERENCE: 1 baseline 3 × 2 table with equal rows and columns, no fill or border, explicit equal padding.
- INFERENCE: 2 baseline copies changing only one cell fill to two different solid colours.
- INFERENCE: 4 baseline copies changing only the top, right, bottom or left edge of one cell.
- INFERENCE: 2 baseline copies changing only that edge's width or dash style.
- INFERENCE: 2 baseline copies changing only one cell's left or top padding.
- INFERENCE: 1 unequal-row-height table with otherwise baseline settings.
- INFERENCE: 1 long-text table that forces a row to expand.
- INFERENCE: 2 labelled merged-cell tables, one horizontal and one vertical.

## Recommendation and validation limits

INFERENCE: **wait for samples** before implementing cell paint or a row-drift correction. The main risk is treating an opaque per-cell field or frame-level property as a confirmed cell setting, or correcting a displacement that lacks a scoring reference.

INFERENCE: allow roughly 2–4 working days for a later bounded implementation and regression checks after differentiated samples establish mappings; allow another 1–2 days for sample analysis. UNKNOWN: effort for style inheritance, shared edges, merged-cell text and layout expansion may exceed that estimate.

OBSERVATION: this note changes no importer or renderer behavior. UNKNOWN: final workspace checks and regenerated pixel outputs are not yet recorded in this first draft; the baseline results above were run before writing it.
