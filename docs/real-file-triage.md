<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Real-file robustness triage — 2026-10-19, updated for stored row endings

## T5–T8 investigations — 2026-10-23

No reader, rendering, printing or preflight logic changed for these items. Per-item regression runs used the same pinned inputs, with `truth/*.json` used by the corpus scorer only, never to infer encodings. All samples, extracted text, CSVs and rendered real-file images remain outside this checkout.

### T5 — 60685.pub: text accounted for; appearance not independently verified

The existing Quill directory/SYID/STRS/TEXT description was used to inspect the pinned file. Its main story has **1,882 UTF-16 units and 20 CR terminators**; three other stories contain only a CR. All four stories match the imported native text after the documented paragraph normalization and removal of the final paragraph terminator. In the main story, the exact heading `Education` is followed by an **empty paragraph**, then `Work Experience`. There is no education-entry text in that interval to recover. No resume contact or personal details are copied here.

The odd strip is already a source drawing group, Contents sequence **302**, with **38** children: **26** rectangle shapes (kind 1) and **12** line shapes (kind 20). A throwaway read-only probe called our existing `escher::parse`; its mapped bounds, converted from EMU at 12,700 EMU/pt with the existing Letter-page offset, are **(36, 36)–(576, 48.96) pt**. The imported group has the same 38 kinds and every child's axis-aligned bounds agrees within **0.001 pt**. The exported PDF was rasterized and inspected outside the checkout; it shows this strip and the empty Education section. This establishes source content and retained geometry, **not** that the strip looks the same in Publisher. Colours/styling and the author's intended appearance have no independent reference here; no fidelity fix is justified by these observations.

### T6 — page numbers and dates: not implemented

`pub-format/text-store.md` §3 describes unhandled control units as fields but does not establish field type, expansion value, page association, date format, or the metadata connecting a placeholder to its definition. The two warning-bearing files contain **4** (`SampleBrochure.pub`) and **2** (`SampleNewsletter.pub`) TEXT units `0x000A`. These observations do not identify what any occurrence means. No value, page number or date was guessed. The existing full warning remains: `Fields and special characters (page numbers, dates) were left out of the text.`

Needed: describer notes verifying the relevant field-definition records and their association with exact story offsets, plus synthetic pairs changing only a known page-number/date field (including format and page context). Without them, this item is blocked; no protected reader was edited.

### T7 — Windows retained print job: cause not established; no change

Read our `crates/engine/src/cmd/print.rs` and the already-pinned `craft-print` implementation/embedded `windows_print.ps1` at `d991bf6792f8ade99c51358bf3c1d902c8cd96b5`. The app passes the current `DocState::title()`; the script assigns `PrintDocument.DocumentName`, uses `StandardPrintController`, calls `Print()`, and disposes the print document in `finally`. The Rust caller waits for PowerShell's output and drops the temporary PDF before returning. No keep-printed-jobs/retain/hold/pause operation is set by this path. This does not prove the HP driver or spooler behaved correctly or identify the reason for “Printing, Retained”. Windows queue/driver settings and actual job state need inspection on the affected PC. No print behavior changed; no Windows or physical-printer test was available here.

### T8 — RGB preflight: inherited broad rule; no change

`preflight.rs` checks **every RGB swatch** in a print document, without checking whether artwork uses it. Despite its “in use” comment, the implementation is not a usage analysis. The same loop/message exists unchanged in the initial source export `d82d525`; it was not introduced by this importer work. This is evidence of inherited behavior in this tree, not a test of today's external upstream. The actual default palette in `designcraft_color::default_swatches()` is CMYK (apart from special swatches). A fresh headless `file.new` with `intent: print` followed by `preflight.run` reports **0 errors / 0 warnings** here, so the owner's particular RGB-black/blue swatches were not reproduced by that minimal case. It remains unknown which UI/style action added them on that PC and whether either was used. The broad rule can warn about unused RGB swatches, but changing it would require a verified usage-aware policy; no color/preflight behavior changed.

### Per-item before/after checks

Scores are ordered **imports / objects / positions within 1 pt / exact texts / styles**. Each row has its own corpus and 24-file batch rerun, not an inferred result. The batch's five unsuccessful entries are the same **2 old-version refusals + 3 damaged inputs** in every run. Each successful file's warnings, notes, page count and object count match the T4 report.

| Item | Corpus before | Corpus after | Real batch before: OK / warnings / refused / damaged | Real batch after | Pages / objects before → after |
|---|---|---|---|---|---|
| T5 | 142/142; 152/152; 111/111 scored positions; 52/52; 111/111 | same | 9 / 10 / 2 / 3 | 9 / 10 / 2 / 3 | 31 / 280 → 31 / 280 |
| T6 | 142/142; 152/152; 111/111 scored positions; 52/52; 111/111 | same | 9 / 10 / 2 / 3 | 9 / 10 / 2 / 3 | 31 / 280 → 31 / 280 |
| T7 | 142/142; 152/152; 111/111 scored positions; 52/52; 111/111 | same | 9 / 10 / 2 / 3 | 9 / 10 / 2 / 3 | 31 / 280 → 31 / 280 |
| T8 | 142/142; 152/152; 111/111 scored positions; 52/52; 111/111 | same | 9 / 10 / 2 / 3 | 9 / 10 / 2 / 3 | 31 / 280 → 31 / 280 |

The nine OK entries still have informational notes; their status is the T3 severity reclassification, not a fidelity claim. Fmt and clippy (`-D warnings`, all targets of pub/engine/UI/CLI) pass. Default pub tests: **48 passed**; CLI: **27 passed**; engine library: **846 passed**; UI library: **548 passed**. Intentionally ignored tests are separate; the external corpus was explicitly run for every item. Windows desktop/spooler behavior and comparison against Publisher remain unverified here. No protected readers or owner-maintained files were edited.

## T4 converted-copy notice — 2026-10-23

Every successful `.pub` open now returns a separate notice in the engine result. The active imported document shows it as non-modal information in the app; the CLI batch CSV appends a `notice` column. It is not a new loss warning and does not replace or suppress warnings/notes:

> This is a converted copy. It can look different from what Publisher shows; some features are lost or simplified, and not every loss gives a warning. Compare it with your original.

Engine and UI-harness tests check delivery, retained diagnostics, visible non-modal text and document association; a CLI test checks report delivery/round-trip. All **19** successful real imports carry the notice. Before/after real counts are **9 OK with notes / 10 warnings / 2 refused / 3 damaged**, **31 pages / 280 objects**; each file's status, warning list and notes match T3. Before/after corpus scores remain **142/142, 152/152, 111/111 scored positions, 52/52, 111/111** (truth files scored only). Native Windows appearance is not tested here.

## T3 diagnostic-severity update — 2026-10-23

Before/after corpus: **142/142 imports, 152/152 objects, 111/111 scored shape/text positions, 52/52 texts, 111/111 styles**, unchanged. Real batch before **0 OK / 19 warnings / 2 refused / 3 damaged**; after **9 OK with informational notes / 10 warnings / 2 refused / 3 damaged**. Successful output totals remain **31 pages / 280 objects**. All **19** successful native document JSONs match T1 outputs after excluding creation/modification timestamps. This is diagnostic reclassification, not a fidelity improvement. Every unidentified formatting field remains in the separate notes column/array; named, confirmed losses remain full warnings. Controlled conversion status changes from **79 OK / 63 warnings** to **130 OK / 12 warnings**, with **142/142** conversions and unchanged corpus scores.

## Deferred T2: exact paired-sample request — 2026-10-23

Needed: redistributable Publisher 2002-or-newer pairs with the same small 2×2 unmerged table, distinctive text in every cell, and multiple ordinary text boxes outside it. Keep text, fonts, grid, dimensions and all unrelated settings identical within each pair. Supply the original `.pub` files plus a plain description identifying the row/column and the one changed setting; a Publisher screenshot or PDF can identify the intended appearance, but is not an encoding specification.

- **Cell fills:** baseline with no fill versus the same file with a solid fill in one named cell (e.g. row 1, column 2); a second pair changing only that fill's colour. State exact colour model/values and opacity/tint. Keep table-frame/background fill unchanged.
- **Cell borders:** separate pairs changing only one named cell edge's visibility, colour, weight, or line style. Include an interior edge and an outside edge; identify both adjacent cells for an interior edge. Keep table-frame stroke unchanged. Do not change several edge properties at once.
- **Association:** repeat a known one-cell change on a different cell without other edits, so record-to-cell mapping can be tested independently. Include an ordinary text box before and after the table to distinguish story/table associations. A second table can be added as a separate association pair once the one-table cases are established.

No personal information, third-party branding or copied template artwork. A statement that contributors may redistribute these synthetic samples is needed. Padding, merged-cell formatting and other features require separate single-setting pairs; do not combine them with the fill/border pairs above. T2 remains deferred; no fill/border encoding is guessed.

## T1 validation update — 2026-10-23

Current branch: `codex/importer-improvements`, based on `main@fc1f3f1`. The remaining sections below describe earlier runs.

| Measurement | Before T1 | After T1 |
|---|---:|---:|
| Corpus imported | 142/142 | 142/142 |
| Corpus objects found | 152/152 | 152/152 |
| Corpus scored shape/text positions within 1 pt | 111/111 | 111/111 |
| Corpus exact texts | 52/52 | 52/52 |
| Corpus styles | 111/111 | 111/111 |
| Real files OK / warnings / refused / damaged | 0 / 19 / 2 / 3 | 0 / 19 / 2 / 3 |
| Real successful pages / objects | 31 / 280 | 31 / 280 |
| Nonempty native cells in the eight Sample* tables | 0 | 48 |
| Table-cell glyph lines in those eight imports | not remeasured | 48 |
| Files with multi-story/TCD association warning (historical W4) | 8 | 0 |
| Files with retained-after-table association warning (historical W6) | 8 | 0 |
| Files with unsupported cell-formatting warning (W3) | 8 | 8 |
| Files with stored-ending difference warning (W5) | 8 | 8 |

All eight source tables link Contents sequence **299** to Quill story **6**, with a **3×2** unmerged row-major grid. Each file has **6** stories and **1** TCD section. Its six boundaries **15, 25, 39, 54, 66, 80** uniquely validate against that linked 80-unit story, including the CR separators. The three controlled table fixtures still import. The rendered eight tables each have **6** nonempty cells, **6** cell glyph lines, **0** duplicate fallback lines and **0** reported table overset. All **48** expected texts were extracted from the exported PDFs; all **16** pages from these eight files were rendered outside the checkout. These are importer/composition/export checks, not an independent Publisher layout comparison.

Reused only the existing 24 Apache binaries at `bf7c9b6a1490046517cbd8e1504e702a89be4bc9` and the controlled binaries at `0ccdf27`; hashes of all 24 Apache files were checked against `.github/test-data/publisher-fixtures.csv`. Inputs, CSVs, PDFs, stream extracts and rendered images stay outside the checkout. The corpus runner used Publisher `truth/*.json` solely to score imports before/after; no encoding was inferred from those files. Existing source provenance is in `pub-format/SOURCES.md`; that protected file was not edited.

T2 changes no importer behavior, so its before/after numbers are the T1-after column above. No fill/border interpretation was added. At the end of the earlier T1 run, T3–T8 had not been started because of its stop-at-unverified instruction. The subsequent T3–T8 entries supersede that historical status.

## Inputs and current run

Current run: `codex/queue`, after merging `pub-import@96580a94`; C2/C3 warning rules applied. The earlier row-ending baseline is retained for comparison.
Reused the same 24 `.pub` binaries from `apache/poi`, `trunk`, folder
`test-data/publisher/`, pinned to `bf7c9b6a1490046517cbd8e1504e702a89be4bc9`.
License/source hashes remain recorded in [SOURCES.md](pub-format/SOURCES.md).
No new download or other Apache file/source/README/documentation/reader output
was accessed. Only the eight task-named table files were used for row-geometry
investigation; all 24 were converted for the requested regression run.

Inputs remain outside the checkout at
`/workspace/sample-inputs/apache-poi-publisher-bf7c9b6a1490` and all 24 hashes
still match the recorded manifest. The three clusterfuzz inputs are deliberately
damaged, as specified in the original triage task. No sample/CSV is committed.

```sh
designcraft-cli batch-convert /workspace/sample-inputs/apache-poi-publisher-bf7c9b6a1490 --out /workspace/designcraft-pub/.builder/queue-C6again-real
```

The existing converter uses normal import, one isolated worker per file and
a 120-second per-file timeout. All 24 entries were processed; exit 1 records
five unsuccessful inputs after completion. This triage distinguishes explicit
older-version refusals from other failures; the raw CSV groups both as failed.

| Status | Before row-ending change | Current |
|---|---:|---:|
| OK | 0 | 0 |
| warnings | 11 | 19 |
| refused | 2 | 2 |
| failed | 11 | 3 |
| Total | 24 | 24 |

The eight former table-dimension failures now import with warnings. Successful
outputs total **31 pages and 280 editable objects**, including group containers
and nested objects. Counts are importer results, not an independent estimate
of source contents. Publisher version remains unreported unless the importer
explicitly refuses the undifferentiated 97/98/2000 family. Filenames do not
establish versions. Unavailable counts are blank, not zero.

## Per-file results

Warning IDs refer to exact texts in the inventory below; error/refusal IDs
refer to verbatim messages. No partial document or complete warning list is
returned for failed/refused imports. Older samples have only their refusal
recorded; no older-version support was added.

| File | Publisher version reported | Status | Pages | Objects | Warnings | Refusal/failure |
|---|---|---|---:|---:|---|---|
| `51318.pub` | not reported | warnings | 1 | 17 | W11 | — |
| `60685.pub` | not reported | warnings | 1 | 43 | W8, W12 | — |
| `LinkAt0And10.pub` | not reported | warnings | 1 | 1 | W1 | — |
| `LinkAt10.pub` | not reported | warnings | 1 | 1 | W1 | — |
| `LinkAt10And20And30.pub` | not reported | warnings | 1 | 1 | W1 | — |
| `LinkAt10And20And30And40.pub` | not reported | warnings | 1 | 1 | W1 | — |
| `LinkAt10Longer.pub` | not reported | warnings | 1 | 1 | W1 | — |
| `LinkAt20.pub` | not reported | warnings | 1 | 1 | W1 | — |
| `Sample.pub` | not reported | warnings | 2 | 7 | W2, W1, W4, W5, W6, W3 | — |
| `Sample2.pub` | not reported | warnings | 2 | 7 | W2, W1, W4, W5, W6, W3 | — |
| `Sample2000.pub` | 97/98/2000 family; see R1 | refused |  |  | — | R1 |
| `Sample2_2010.pub` | not reported | warnings | 2 | 7 | W2, W1, W4, W5, W6, W3 | — |
| `Sample3.pub` | not reported | warnings | 2 | 7 | W2, W1, W4, W5, W6, W3 | — |
| `Sample3_2010.pub` | not reported | warnings | 2 | 7 | W2, W1, W4, W5, W6, W3 | — |
| `Sample4.pub` | not reported | warnings | 2 | 7 | W2, W1, W4, W5, W6, W3 | — |
| `Sample4_2010.pub` | not reported | warnings | 2 | 7 | W2, W1, W4, W5, W6, W3 | — |
| `Sample98.pub` | 97/98/2000 family; see R1 | refused |  |  | — | R1 |
| `SampleBrochure.pub` | not reported | warnings | 2 | 57 | W10, W14, W7 | — |
| `SampleNewsletter.pub` | not reported | warnings | 4 | 99 | W9, W13, W7 | — |
| `Sample_2010.pub` | not reported | warnings | 2 | 7 | W2, W1, W4, W5, W6, W3 | — |
| `Simple.pub` | not reported | warnings | 1 | 2 | W1 | — |
| `clusterfuzz-testcase-minimized-POIHPBFFuzzer-4701121678278656.pub` | not reported | failed |  |  | — | E3 |
| `clusterfuzz-testcase-minimized-POIHPBFFuzzer-4918886059278336.pub` | not reported | failed |  |  | — | E2 |
| `clusterfuzz-testcase-minimized-POIHPBFFuzzer-6325615354773504.pub` | not reported | failed |  |  | — | E1 |

### Exact refusal and failure messages

**E1**

```text
import: can't open the Publisher file: damaged Publisher file: compound file: Invalid CFB byte order mark (expected 0xFFFE, found 0xFFFA)
```

**E2**

```text
import: can't open the Publisher file: damaged Publisher file: compound file: Malformed FAT (FAT has 244 entries, but file has only 64 sectors)
```

**E3**

```text
import: can't open the Publisher file: damaged Publisher file: compound file: Malformed FAT (sector 129 pointed to twice)
```

**R1**

```text
import: can't open the Publisher file: unsupported Publisher file: Publisher 97/98/2000 files are not supported yet; open and save the file in Publisher 2002 or later
```

## Robustness and visible limitations

No worker crash or timeout was reported. The same three deliberately damaged
clusterfuzz inputs still fail compound-file validation; the two old-version
samples retain their exact refusal. No new failure appeared. All per-file statuses and page/object counts remain unchanged; C2/C3 update only warning texts, as inventoried below.

Every successful PDF/native pair exists, and native/PDF page counts match the
CSV. Native row heights for each repaired table total **657564 EMU /
51.776692913 pt**, matching the previously measured drawing frame. Configured row heights use the
stored endings and retain the previous default AtLeast mode; content may expand them. The three pubsmith controls also match their
measured frame heights and have no new row-ending warning. See
[tables.md](pub-format/tables.md) for the 11-file height check.

Rasterized all **31 successful PDF pages** at a bounded 600-pixel size; every
page contains nonwhite pixels, including all **16 pages from the eight repaired
imports**. Visually reviewed those 16 pages. The page-2 table areas look empty:
all six native table cells are empty, and the importer warns that multi-story/
TCD cell-text association is not decoded and original text is retained after
the table. This is a visible limitation, **not fixed in this task**. Native
host stories retain text; that does not guarantee text is displayed in its
intended cells or fully visible in the PDF. Nonblank pages are not proof of
complete tables or original visual fidelity.

The 11 previously successful native documents are identical to the preceding
batch after excluding creation/modification timestamps; all 15 previously
successful PDF pages are pixel-identical at the checked scale. No Publisher
rendering or another reader's output was used for comparison. No new decoding
of text association, styling, version or unknown fields was attempted.

## Warnings ranked by affected files

Deduplicate exact warning text per file, keeping field-list variants separate,
as in [warning-triage.md](warning-triage.md). Current final batch: **14 exact
texts / 64 file-warning pairs / 19 affected files**, compared with **17 / 72 / 19**
before C2/C3. Two exact texts also appear in the refreshed 142-fixture inventory;
12 do not. “New” denotes only a different literal warning, not a new encoding.
Errors and refusals remain separate. IDs below are refreshed for this run.

| ID | Files | Exact warning text | Compared with warning-triage.md | Affected files |
|---|---:|---|---|---|
| W1 | 15 | Publisher paragraph formatting is unsupported for fields: defaults [0x30]; runs [none]. Supported attributes were retained. | already present (exact text) | `LinkAt0And10.pub`, `LinkAt10.pub`, `LinkAt10And20And30.pub`, `LinkAt10And20And30And40.pub`, `LinkAt10Longer.pub`, `LinkAt20.pub`, `Sample.pub`, `Sample2.pub`, `Sample2_2010.pub`, `Sample3.pub`, `Sample3_2010.pub`, `Sample4.pub`, `Sample4_2010.pub`, `Sample_2010.pub`, `Simple.pub` |
| W2 | 8 | Publisher character formatting is unsupported for fields: defaults [none]; runs [0x00, 0x37, 0x38]. Supported attributes were retained. | new exact text | `Sample.pub`, `Sample2.pub`, `Sample2_2010.pub`, `Sample3.pub`, `Sample3_2010.pub`, `Sample4.pub`, `Sample4_2010.pub`, `Sample_2010.pub` |
| W3 | 8 | Publisher table cell fills, borders, padding and other cell formatting are not decoded; unpainted cells were used. | already present (exact text) | `Sample.pub`, `Sample2.pub`, `Sample2_2010.pub`, `Sample3.pub`, `Sample3_2010.pub`, `Sample4.pub`, `Sample4_2010.pub`, `Sample_2010.pub` |
| W4 | 8 | Publisher table text across multiple stories/TCD sections is not decoded. | new exact text | `Sample.pub`, `Sample2.pub`, `Sample2_2010.pub`, `Sample3.pub`, `Sample3_2010.pub`, `Sample4.pub`, `Sample4_2010.pub`, `Sample_2010.pub` |
| W5 | 8 | Publisher table used stored endings for row positions; largest difference from size sums: 9564 EMU; cause unknown. | new exact text | `Sample.pub`, `Sample2.pub`, `Sample2_2010.pub`, `Sample3.pub`, `Sample3_2010.pub`, `Sample4.pub`, `Sample4_2010.pub`, `Sample_2010.pub` |
| W6 | 8 | Table cell-text association is unconfirmed or missing; the original text was retained after the table. | new exact text | `Sample.pub`, `Sample2.pub`, `Sample2_2010.pub`, `Sample3.pub`, `Sample3_2010.pub`, `Sample4.pub`, `Sample4_2010.pub`, `Sample_2010.pub` |
| W7 | 2 | Fields and special characters (page numbers, dates) were left out of the text. | new exact text | `SampleBrochure.pub`, `SampleNewsletter.pub` |
| W8 | 1 | Publisher character formatting is unsupported for fields: defaults [0x58]; runs [none]. Supported attributes were retained. | new exact text | `60685.pub` |
| W9 | 1 | Publisher character formatting is unsupported for fields: defaults [none]; runs [0x00, 0x1B, 0x22, 0x37]. Supported attributes were retained. | new exact text | `SampleNewsletter.pub` |
| W10 | 1 | Publisher character formatting is unsupported for fields: defaults [none]; runs [0x2D, 0x37]. Supported attributes were retained. | new exact text | `SampleBrochure.pub` |
| W11 | 1 | Publisher paragraph formatting is unsupported for fields: defaults [0x30]; runs [0x19, 0x30]. Supported attributes were retained. | new exact text | `51318.pub` |
| W12 | 1 | Publisher paragraph formatting is unsupported for fields: defaults [0x30]; runs [0x19]. Supported attributes were retained. | new exact text | `60685.pub` |
| W13 | 1 | Publisher paragraph formatting is unsupported for fields: defaults [none]; runs [0x06, 0x19, 0x1D, 0x30, 0x32, 0x57]. Supported attributes were retained. Not fully imported: list marker (unconfirmed descriptor). | new exact text | `SampleNewsletter.pub` |
| W14 | 1 | Publisher paragraph formatting is unsupported for fields: defaults [none]; runs [0x19, 0x30]. Supported attributes were retained. | new exact text | `SampleBrochure.pub` |

Unknown numeric fields remain unidentified. The row-position warning reports
that stored endings were used and the largest absolute discrepancy is 9564 EMU;
its cause remains unknown. It is emitted once per affected table, not per row.
No border or other cause is assigned. Unconfirmed list-marker data is not
labeled numbering, and filenames do not establish file contents or encodings.

## Validation

Fmt, clippy for pub and CLI with `-D warnings`, and full pub/CLI tests passed:
**69 passed / 10 ignored**. The opt-in fixture warning test passed, including
justification. The opt-in real harness was also explicitly run successfully.
Final corpus: **142/142 imports, 152/152 objects, 111/111 scored shape/text positions within 1 pt,
52/52 texts exact, 111/111 styles**, unchanged. Truth exports were for scoring
only, never for this warning diagnosis or encoding inference.

Compared all 19 successful real-file native `document.json` payloads with the
post-merge baseline: only creation timestamps differ. C2/C3 alter warning
reporting only. No other protected reader, drawing code or table logic changed.
The old C6 summary below is historical; the current continuation summary follows.

## 2026-10-19 · C4 scoring audit: 41 unscored image exports

The premise that 41 objects miss by more than 1 pt is not established by the
current scorer. `crates/pub/tests/corpus.rs::score` increments the geometry
numerator only for non-image truth elements. The 152 scoring elements comprise
59 shapes, 52 texts and 41 images. All 111 evaluated non-image frames pass;
all 152 elements are found. The fresh corpus emits no `off by` or `missing`
notes. Thus **111/152 remains the historical log ratio**, with **111/111 evaluated
frames within 1 pt**, rather than 41 measured failures.

The existing `Builder::rect` page-centre conversion, `pt` EMU conversion and
`escher::map` child-space mapping already implement the settled rules in
`anchor-coordinates.md`. No measured regression justifies changing them.
Rotation sign/centre, nested group behavior and the scratch-area ambiguity
remain outside those settled rules. No encoding was inferred from truth data.

Each excluded element is listed below, using one-based page and element indices
from the scoring oracle. Category **other / UNKNOWN** means that its geometry
was not scored, not that an offset or a cause was measured. For **each row**, the
sample needed to settle it is a Publisher-authored reference for that same
fixture retaining the original object's frame and transform instead of only
its flattened picture export. Such reference data would be for scoring only;
any encoding change would still require separate allowed describer notes.

| Fixture | Page | Element | Classification |
|---|---:|---:|---|
| `fill__gradient.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `group__rotated.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `line__dash-2.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `line__dash-4.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `line__dash-5.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `picture__base.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `picture__border.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `picture__crop-all.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `picture__crop-left.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `picture__crop-top.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `picture__flip-h.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `picture__moved.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `picture__oval-mask.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `picture__rotated-30.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `picture__stretched.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `picture__two.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `picture__two.pub` | 1 | 2 | other / UNKNOWN — image geometry unscored |
| `shadow__offset.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `shadow__on.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `shapes__curve.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `shapes__diamond.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `shapes__heart.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `shapes__hexagon.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `shapes__line-arrow.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `shapes__line.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `shapes__polyline.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `shapes__rightarrow.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `shapes__roundrect.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `shapes__star5.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `shapes__triangle.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `table__2x2.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `table__3x2.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `table__merged.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `wordart__arch.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `wordart__base.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `wordart__circle.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `wordart__fill-red.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `wordart__font.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `wordart__line-white.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `wordart__shadow.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |
| `wordart__text.pub` | 1 | 1 | other / UNKNOWN — image geometry unscored |

## Opt-in real-file regression harness (C5)

The ignored CLI integration test runs the normal batch command against the
external pinned folder and checks **0 OK / 19 warnings / 2 refused / 3 failed**.
The CLI's JSON/CSV groups the last two statuses as five failures; the harness
separates the two explicit unsupported-version messages. Outputs use a temporary
folder that is removed afterwards. No input sample is committed.

```sh
PUB_REAL_CORPUS=/outside/repo/apache-poi-publisher-bf7c9b6a1490 \
  cargo test -p designcraft-cli --test batch \
  real_publisher_corpus_keeps_expected_status_counts -- --ignored --nocapture
```

Use the same 24 files at the commit and SHA-256 hashes recorded in SOURCES.md;
the environment variable must point outside this checkout. This is a regression
baseline, not an assertion that warnings or deliberately damaged files are OK.

## 2026-10-19 · Codex queue final regression counts (C6)

Final run: `.builder/queue-C6-real/report.csv` (not committed), using the same
24 pinned external inputs. Queue start → final: **0 OK / 19 warnings / 2 refused /
3 failed → 0 / 19 / 2 / 3**, **31 successful pages / 280 objects** throughout.
The exact-warning inventory remains **17 texts / 72 file-warning pairs**.
No new crash, timeout or changed failure reason appeared. The existing table
cell-text and rendering limitations recorded above remain; no production import
code changed in this queue. C2/C3's warning changes remain blocked by scope.

All item runs passed fmt, clippy with warnings denied, CLI/pub tests and the
corpus. Final ordinary suites: **67 passed / 10 ignored**; the new C5 opt-in
24-file test additionally passed when explicitly enabled. Corpus baseline →
final: **142/142 imports, 152/152 objects, 111/111 scored shape/text positions, 52/52 texts,
111/111 styles → unchanged**. The C4 audit above explains the 41 image elements
excluded from the position numerator; no evaluated position miss was found.
The separate fresh controlled batch was **79 OK / 63 warnings / 0 failed**.

## 2026-10-19 · C6 continuation final counts

Before → after: **0 OK / 19 warnings / 2 refused / 3 failed → unchanged**,
**31 pages / 280 objects**. Exact warning texts **17 → 14**; file-warning pairs
**72 → 64**. Controlled batch **79 OK / 63 warnings / 0 failures**, with exact
texts **9 → 7**, file-warning pairs **120 → 66**. Counts of files with warnings
stay unchanged because other unsupported fields, especially 0x30, still warn.
C2/C3 are complete; C7/C8/C9 are BLOCKED on public MS-ODRAW access (proxy CONNECT
403, no specification text obtained). C4's 41 image exports remain unscored
geometry, not measured errors. No new crash, hang or failure appeared.
