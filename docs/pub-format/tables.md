<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Publisher tables — builder observations (task 4.2)

Source: only the binary Contents, Quill and Escher streams in the allowed
pubsmith fixtures at `0ccdf27`. A directory-only scan of all 142 Contents
streams found type-0x10 records only in `table__2x2.pub`, `table__3x2.pub`, and
`table__merged.pub`. No external Publisher reader or truth JSON was consulted.

## Contents: table and grid

A table is the type-0x10 object already described in `contents-stream.md`.
Its key 0x27 is the Quill story id (3 in all three samples). Additional fields:

| Key | Kind | Observed meaning |
|---|---|---|
| 0x66 | 0x20 | Row count: 2, 3, 2 respectively. |
| 0x67 | 0x20 | Column count: 2 in all three. |
| 0x68 / 0x69 | 0x20 | Overall width / height in EMU. |
| 0x6B | 0x70 | Sequence number of the type-0x63 cell-list record. |
| 0x6D | 0x90 | Dimension list: first `columns` entries describe columns, then `rows` entries describe rows. |

Each dimension entry is kind 0x88, with key 0x01 the cumulative ending position
and key 0x02 its size, both kind 0x20 in EMU. Columns are 1828800 EMU (144 pt)
wide; rows are 457200 EMU (36 pt) high. Width is 3657600 EMU (288 pt); heights
are 914400 / 1371600 / 914400 EMU. Validate positive dimensions, cumulative
positions, declared totals and bounded counts before allocating a grid.

## Type-0x63 cell list and merges

Key 0x01 / kind 0x18 gives the number of actual cell records (4 / 6 / 3).
Key 0x02 / kind 0xA0 is an array of kind-0x88 cell records. Numeric fields:

| Keys | Meaning |
|---|---|
| 0x01 / 0x02 | Inclusive first / last row, zero-based, omitted zero. |
| 0x03 / 0x04 | Inclusive first / last column, zero-based, omitted zero. |

The ordinary grids enumerate cells row-major. The merged sample instead lists
(1,1,0,0), (1,1,1,1), (0,0,0,1): bottom-left, bottom-right, top row spanning
two columns. Preserve coordinates, not array index, to place cells. Compute
spans as inclusive end minus start plus one. Covered grid slots remain blank
in DesignCraft. Reject out-of-range, overlapping or incomplete cell rectangles.
A row-spanning example is absent; the same inclusive rectangle representation
can be exercised synthetically, but is not independently confirmed by a file.

Other repeated cell fields are 0x09 = 228180, 0x0A–0x0D = 36576, and 0x0E =
114300. Their precise meanings are not confirmed by differing samples; do not
use them to invent fills, borders, padding or alignment.

## Quill cell text: `TCD `

All three files have exactly one story and one `TCD ` section. At section
position 0 is a u32 equal to cell-record count minus one. At positions 4 and 8
are 0 and 0xFF00 (meaning unconfirmed). Starting at position 12 are `count+1`
u32 text boundaries in UTF-16 units within the story:

- 2×2: 2, 5, 8, 12; TEXT is `A1 CR B1 CR A2 CR B2 CR`.
- 3×2: 0, 1, 2, 3, 4, 6; TEXT is six CRs (empty cells).
- Merged: 0, 1, 3; TEXT is three CRs (empty cells).

Intermediate boundaries point at the separating CR (excluded from cell text).
The next cell starts one unit later. The final boundary is the story length,
including its final CR, which is stripped as usual. Validate separator units,
monotonic boundaries and exact final length. Do not split on every CR: cells
may contain paragraph breaks, and only TCD boundaries are cell separators.

For the unmerged 2×2 sample, cell text order and cell-record order are row-major,
confirmed by `A1`, `B1`, `A2`, `B2`. All merged cells in the available fixture are
empty: text association for nonempty merged cells is **not confirmed**. Preserve
such unassigned text after the table with a warning, rather than guessing which
merged cell owns it. Association across multiple stories/TCD sections is also
unconfirmed; retain text and warn when that association is ambiguous.

## Cell fills and borders: unconfirmed

The three fixtures have no independent cell-fill or border-setting variants.
Their Quill `MCLD` sections contain one formatting block per cell (130 bytes per
block in these files). Block fields include 0x00–0x03 values near 0x1F00 and
0x06–0x09 = 36576; no differing setting establishes their interpretation.
Escher supplies table-frame line properties but does not establish per-cell
formatting. Do not reinterpret the unknown MCLD integers as RGB colours, line
weights, or swatch indices. Import confirmed geometry, merges and text; use
unpainted cell fills/edges and warn explicitly that cell fills, borders and
other cell formatting were not decoded. In particular, clear DesignCraft's
normal default black borders rather than inventing them.

## Native mapping and limits

DesignCraft `Table` has row heights, column widths, a row-major cell grid and
inclusive-region spans represented as top-left owners plus row/column spans.
Each cell owns a frameless `Story`; keep supported character and paragraph
formatting from the imported story when cutting cell text. A table is anchored
in its host story with `Story::insert_table`, rather than a set of text boxes.
Use at most 256 rows/columns, 10000 grid slots and 10000 TCD cells; reject
malformed dimensions/rectangles/ranges. Bound total formatting-copy work too.

Across a publication, cap parsed/linked/native grid cells at 100000, table text
processing at 16000000 UTF-16 units, and formatting-copy work at 1000000 steps.
These aggregate caps also guard against repeated references to the same records.

## Task 4.2b: complete fixture audit (2026-10-18)

Rescanned every record directory in the 142 allowed pubsmith binaries at
`0ccdf27`. Only the following three contain a type-0x10 table:

| File | Actual cells / MCLD blocks | SHA-256 of `.pub` |
|---|---|---|
| `table__2x2.pub` | 4 | `72d2127f4ffdbe998f4f94414a28584e49af817991b70a564fda885dcb1a6a7d` |
| `table__3x2.pub` | 6 | `12c45cd9057553f748d0cf23ab5348d901be294b1d6e790eb80c3331d57d2616` |
| `table__merged.pub` | 3 | `3df51f5e3e29f247b8636627725d43f5973dc7ea887338f32aec8f4b49a23b30` |

Inspected all 13 MCLD cell-format blocks and compared the Contents cell records
and Escher properties. Complete MCLD records differ between cells, including
opaque fields; that is not evidence that an arbitrary field is a fill colour
or border style. There is no otherwise-identical cell shading/border variant,
export oracle, or documented mapping establishing those fields. No new cell
paint encoding is confirmed. Keep the explicit warning and unpainted cells.
The real-fixture assertion checks all three grids and verifies the warning,
absent fills and invisible edges. More differentiated table samples are needed.

Migrated this audit from our own `codex/4.2b-cell-style` branch into
designcraft-pub on 2026-10-18; all three binary hashes still match. The user
now permits Publisher's `truth/*.json` layout exports as data. Those three
exports represent tables as flattened images, so they do not establish an
editable cell-fill/border encoding or resolve the audit's open question.

## Table cumulative-dimension diagnosis — 2026-10-19

**Decision: case (c), needs describer notes. Confidence: high that the present
notes do not justify a relaxation; the files' actual validity and the cause of
the discrepancy remain undetermined.** This is a numeric observation using the
eight task-named Apache POI samples at pinned commit
`bf7c9b6a1490046517cbd8e1504e702a89be4bc9`, not a new table-encoding rule.
Original hashes/provenance are recorded in `SOURCES.md`. No other Apache file
was examined for this numeric diagnosis, and no new repository download occurred.

### What raises the error

The check is in `crates/pub/src/tables.rs`, `TableInfo::parse` (the error at
line 54 in the diagnosed base `pub-import@18731e16`). `contents::parse` calls
it for type-0x10 records and propagates the error with `?`; the comparison is
not implemented in `contents.rs` itself.

It reads rows 0x66, columns 0x67, overall width/height 0x68/0x69, and 0x6D's
dimension entries. It requires `rows + columns` entries, positive bounded sizes,
and checked sums. For each entry, add key 0x02 (size) to a running sum, resetting
to zero at the first row after the column entries. Require key 0x01 to equal
that sum **exactly**, with no tolerance:

```text
sum += size
if number(fields, 1) != Some(sum): reject with "table: inconsistent cumulative dimension"
```

The next checks require the final column/row size sums to equal the declared
width/height. The current notes above describe key 0x01 as a cumulative ending
position and key 0x02 as size, and require validating cumulative positions and
totals. They do not establish a non-additive relationship or permitted error.

### Each of the eight failing samples

All numbers below are raw **EMU**, read using the already documented property
keys/kinds. Offsets are decimal bytes within the Contents stream; sequence ids
are decimal directory indexes. Each file has one type-0x10 table. All have
column sizes `[1782000, 1782000]` and row sizes `[216000, 216000, 216000]`.
The first rejected entry is dimension entry 3 (one-based), i.e. row 1.

| File | Sequence | Record offset | Rows × columns | Width | Height | First stored row ending | Required size sum | Stored minus required |
|---|---:|---:|---|---:|---:|---:|---:|---:|
| `Sample.pub` | 299 | 8708 | 3 × 2 | 3564000 | 648000 | 219188 | 216000 | +3188 |
| `Sample2.pub` | 299 | 8720 | 3 × 2 | 3564000 | 648000 | 219188 | 216000 | +3188 |
| `Sample2_2010.pub` | 299 | 8692 | 3 × 2 | 3564000 | 648000 | 219188 | 216000 | +3188 |
| `Sample3.pub` | 299 | 8710 | 3 × 2 | 3564000 | 648000 | 219188 | 216000 | +3188 |
| `Sample3_2010.pub` | 299 | 8690 | 3 × 2 | 3564000 | 648000 | 219188 | 216000 | +3188 |
| `Sample4.pub` | 299 | 8710 | 3 × 2 | 3564000 | 648000 | 219188 | 216000 | +3188 |
| `Sample4_2010.pub` | 299 | 8690 | 3 × 2 | 3564000 | 648000 | 219188 | 216000 | +3188 |
| `Sample_2010.pub` | 299 | 8690 | 3 × 2 | 3564000 | 648000 | 219188 | 216000 | +3188 |

Continuing the arithmetic on all entries (the importer itself stops at row 1)
gives the following identical observations in **each** of the eight files:

| Entry | Stored size | Sum of sizes on axis | Stored cumulative ending | Difference |
|---|---:|---:|---:|---:|
| Column 1 | 1782000 | 1782000 | 1782000 | 0 |
| Column 2 | 1782000 | 3564000 | 3564000 | 0 |
| Row 1 | 216000 | 216000 | 219188 | +3188 |
| Row 2 | 216000 | 432000 | 438376 | +6376 |
| Row 3 | 216000 | 648000 | 657564 | +9564 |

Both declared totals match the sums of the respective sizes: width 3564000,
height 648000. The stored last row ending is instead 657564, exceeding that
height by 9564 EMU. The first-row difference is approximately 0.2510 pt
(3188 / 12700), not an exact match. These are observations only: no meaning or
cause is assigned to the deltas, and no tolerance or alternate row sizes are
proposed from them.

### The three allowed table controls

The same extraction/arithmetic was applied to the existing pubsmith fixtures
at `0ccdf27`. All dimension counts and sums match exactly, so they pass the
same cumulative and declared-total checks. The real-fixture table test also
passes for all three, including the merged-cell sample.

| Fixture | Rows × columns | Width / height | Column sizes → cumulative endings | Row sizes → cumulative endings | Maximum absolute difference |
|---|---|---|---|---|---:|
| `table__2x2.pub` | 2 × 2 | 3657600 / 914400 | [1828800, 1828800] → [1828800, 3657600] | [457200, 457200] → [457200, 914400] | 0 |
| `table__3x2.pub` | 3 × 2 | 3657600 / 1371600 | [1828800, 1828800] → [1828800, 3657600] | [457200, 457200, 457200] → [457200, 914400, 1371600] | 0 |
| `table__merged.pub` | 2 × 2 | 3657600 / 914400 | [1828800, 1828800] → [1828800, 3657600] | [457200, 457200] → [457200, 914400] | 0 |

### Why case (c), not (a) or (b)

The eight files contradict the **current interpretation**, but that does not
prove the publications themselves are malformed (case a). The three original
controls only established exact additive grids. Nothing in these notes explains
which quantities should govern row placement when the stored endings differ
from the size sums, or supplies a tolerance that would justify a safe relaxation
(case b). The repeated discrepancy does not establish such a rule by itself.
A describer must explain this relationship, applicable conditions and totals
validation with independent evidence before builder changes are made.

No production code or tests were changed, and `real-file-triage.md` was left
unchanged as required when code is unchanged. The fresh all-24 batch retained
**0 OK / 11 warnings / 2 refused / 11 failed**, identical to before. The raw
converter CSV combines refusals/failures into 13 failed rows. Exact failure
messages and warning lists are unchanged, including these eight table errors;
no successful import of these eight is claimed. The 142-fixture scores also
remain 142/142 imports, 152/152 objects, 111/152 positions within 1 pt, 52/52
texts exact and 111/111 styles.

## Cautious row handling: drawn-height prerequisite blocked — 2026-10-19

**No code change. Drawn-height case: undetermined, not an observed “neither.”**
The task requires measuring the table frame using only drawing keys already
documented in `docs/pub-format/` before changing validation. In merged base
`pub-import@71d7c55c`, these notes document the Contents values below, and
`contents-stream.md` §6 documents the drawing client-data link 0x6801 to the
Contents sequence number. They do **not** document the Publisher client-anchor
coordinate fields, their signed/unit interpretation, or the applicable group
coordinate mapping needed for a table-frame height measurement.

The retained `escher.rs` contains an anchor implementation/comment, but using
its undocumented fields as new format evidence would exceed this task's
“keys already documented” constraint. No drawing bytes were decoded using
those fields. The independent `table-review.md` likewise records the drawn
height as unknown; it does not supply these missing keys. General references
to MS-ODRAW/drawing geometry do not establish Publisher-specific anchor fields.

### OBSERVATION: existing Contents values, not a drawing measurement

These are the numeric observations already in the diagnosis and independent
review; no value in the “drawn height” column was measured or inferred. All
values are EMU. Each declared height equals the sum of key-0x02 row sizes.

| File | Declared height / row-size sum | Last stored ending | Drawn height | Match |
|---|---:|---:|---|---|
| `Sample.pub` | 648000 | 657564 | not measured | undetermined |
| `Sample2.pub` | 648000 | 657564 | not measured | undetermined |
| `Sample3.pub` | 648000 | 657564 | not measured | undetermined |
| `Sample4.pub` | 648000 | 657564 | not measured | undetermined |
| `Sample2_2010.pub` | 648000 | 657564 | not measured | undetermined |
| `Sample3_2010.pub` | 648000 | 657564 | not measured | undetermined |
| `Sample4_2010.pub` | 648000 | 657564 | not measured | undetermined |
| `Sample_2010.pub` | 648000 | 657564 | not measured | undetermined |
| `table__2x2.pub` | 914400 | 914400 | not measured | undetermined |
| `table__3x2.pub` | 1371600 | 1371600 | not measured | undetermined |
| `table__merged.pub` | 914400 | 914400 | not measured | undetermined |

Absence of a permitted measurement is not an observation that the drawn height
matches neither value, and is not evidence satisfying the proposed change's
precondition. No row-ending tolerance, size-layout rule or alternate anchor
interpretation was added. The existing cumulative check remains unchanged.
The prior case-(c) conclusion remains unresolved pending the required notes.

### Needs you — missing drawing notes

Please have a describer document the table frame's Publisher drawing anchor
record/coordinate keys, lengths and signed units, linkage to the table's
Contents id, and any group transforms needed to compare its drawn height with
the declared table height. The permitted measurement can then be made without
promoting the retained implementation into a new encoding rule. No additional
input was acquired to fill this gap.

The requested uncertainties also remain **UNKNOWN**: why stored endings differ;
whether 3188 is constant outside these near-identical tables; whether it relates
to a border; and how unequal row heights behave. No cause is asserted. Conditional
new-behavior tests and eight-file import/PDF confirmation were not performed,
because the code change could not pass its initial documented-key prerequisite.

Unaffected verification completed: before/after all-24 counts are **0 OK /
11 warnings / 2 refused / 11 failed**, with the eight still rejected. The
three-control real-fixture test still passes. Fmt, CLI/pub clippy and existing
full tests pass; the separate 142-fixture score remains **142/142 imports,
152/152 objects, 111/152 positions within 1 pt, 52/52 texts, 111/111 styles**.
`real-file-triage.md` remains unchanged because code is unchanged. This section
records an incomplete, input-blocked task, not a completed implementation.

## OBSERVATION: drawn table height via retained reader — 2026-10-19

**Case: endings. Stop; no code change.** The current task's clarification
permits measuring the retained reader's existing output. This resolves the
previous section's unmeasured-height status; it adds no anchor key, unit or
group-transform rule.

Used a throwaway, uncommitted Rust harness calling **`escher::parse`** directly
on the existing drawing streams, independently of table import. Located each
returned shape using its already documented Contents sequence association.
There was exactly one match per file. The returned rotation was **0 degrees**
in all 11 measurements; the frame height was cross-checked read-only against
**`Builder::rect`**'s existing bounds calculation. No retained function was
changed. Heights below are measured from the existing reader's **EMU output**;
point figures use the importer's existing **`pt`** conversion. Only measurement
numbers and existing-function identities are recorded here, not a new decoding
rule. Source files and hashes are those of the diagnosis/independent review.

| File | Contents seq | Row-size sum / declared height (EMU) | Last ending (EMU) | Drawn height (EMU) | Drawn height (pt) | Drawn − sizes (pt) | Drawn − ending (pt) | Closest/exact match |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| `Sample.pub` | 299 | 648000 | 657564 | 657564 | 51.776692913 | 0.753070866 | 0.000000000 | endings, exact |
| `Sample2.pub` | 299 | 648000 | 657564 | 657564 | 51.776692913 | 0.753070866 | 0.000000000 | endings, exact |
| `Sample3.pub` | 299 | 648000 | 657564 | 657564 | 51.776692913 | 0.753070866 | 0.000000000 | endings, exact |
| `Sample4.pub` | 299 | 648000 | 657564 | 657564 | 51.776692913 | 0.753070866 | 0.000000000 | endings, exact |
| `Sample2_2010.pub` | 299 | 648000 | 657564 | 657564 | 51.776692913 | 0.753070866 | 0.000000000 | endings, exact |
| `Sample3_2010.pub` | 299 | 648000 | 657564 | 657564 | 51.776692913 | 0.753070866 | 0.000000000 | endings, exact |
| `Sample4_2010.pub` | 299 | 648000 | 657564 | 657564 | 51.776692913 | 0.753070866 | 0.000000000 | endings, exact |
| `Sample_2010.pub` | 299 | 648000 | 657564 | 657564 | 51.776692913 | 0.753070866 | 0.000000000 | endings, exact |
| `table__2x2.pub` | 293 | 914400 | 914400 | 914400 | 72.000000000 | 0.000000000 | 0.000000000 | both, exact |
| `table__3x2.pub` | 293 | 1371600 | 1371600 | 1371600 | 108.000000000 | 0.000000000 | 0.000000000 | both, exact |
| `table__merged.pub` | 293 | 914400 | 914400 | 914400 | 72.000000000 | 0.000000000 | 0.000000000 | both, exact |

For every Apache sample, the drawn height equals **657564 EMU exactly**,
matching the stored ending with zero difference. It exceeds the row-size sum
**648000 EMU** by **9564 EMU**, approximately **0.753070866 pt**. Thus both
candidates are within 1 pt; the 1-pt threshold alone does not distinguish them.
The exact match is nevertheless to the endings, triggering the task's explicit
**step 4 stop condition**. It is not a sizes-only or “neither” observation.
The three controls exactly match both their row-size sums and last endings.

No row-validation relaxation, new warning, size-based layout rule or new-behavior
regression test was introduced. Existing control tests and checks still pass.
This observation alone does not establish that endings should govern layout in
other tables or explain the differing numeric quantities. Why endings differ,
whether 3188 is constant elsewhere, whether it relates to a border, and behavior
with unequal row heights all remain **UNKNOWN**. No cause is asserted.

Because the endings case requires stopping with docs only, no new all-24 batch
was run and no successful eight-file import/nonblank PDF is claimed. The last
verified batch on identical application/importer code remains **0 OK / 11 with
warnings / 2 refused / 11 failed**; these are retained baseline counts, not a
new after-run result. Code, tests, `real-file-triage.md`, owners and task
checkboxes remain unchanged. The fresh corpus score still meets all five 3.4
baseline totals.

## Observation-based row-position rule — 2026-10-19

This task's specific change uses the stored row endings for row positions,
based on the exact drawn-height observation above: the eight Apache frames
are 657564 EMU high, matching the last ending; the controls match both values.
This supersedes the former requirement that each row ending equal the running
sum of row sizes. It does not reinterpret why the quantities differ.

For rows, use consecutive key-0x01 endings, beginning at zero, to derive native
row heights/positions. Leave the native row-height mode at its previous default
(`AtLeast`); the stored data does not establish a fixed-height mode. Require positive strictly increasing endings within the
signed coordinate range, and positive bounded derived heights. Keep the
existing positive/bounded key-0x02 sizes, checked size sums, dimension/cell
counts, grid bounds and overflow checks. Size sums must still equal declared
width/height; **the last row ending need not equal declared height**. Columns
retain their existing size-based layout, cumulative validation and exact
size-sum/declared-width check. No column-ending exception is established here.

If any row ending differs from its running size sum, emit one bounded warning
per affected table: stored endings were used for row positions, the largest
absolute difference in EMU, and “cause unknown.” Do not infer a cause. No new
anchor key, unit or group-transform rule is added. Why endings differ, whether
3188 is constant, any relationship to a border, and real tables with unequal
row heights remain unexplained. Synthetic checks do not establish new sample
encodings. Validation/results for this change are recorded in REPORTS.md.

### Row-ending implementation validation

The synthetic 3188-per-row case imports with native heights 219188 EMU and
one warning whose maximum difference is 9564 EMU. Tests retain rejection of
non-increasing/negative/zero endings, bad dimension counts, invalid sizes,
size-total mismatches, column cumulative mismatches and checked-sum overflow.
The three real controls produce no new warning. Native rows keep their previous
default `AtLeast` mode: ending-derived heights are retained, while text layout
may expand a row to fit content. No fixed-height Publisher flag was established.

| Imported file | Configured native row-height sum (pt) | Previously measured drawn height (EMU) | Match |
|---|---:|---:|---|
| `Sample.pub` | 51.776692913 | 657564 | exact to numeric precision |
| `Sample2.pub` | 51.776692913 | 657564 | exact to numeric precision |
| `Sample2_2010.pub` | 51.776692913 | 657564 | exact to numeric precision |
| `Sample3.pub` | 51.776692913 | 657564 | exact to numeric precision |
| `Sample3_2010.pub` | 51.776692913 | 657564 | exact to numeric precision |
| `Sample4.pub` | 51.776692913 | 657564 | exact to numeric precision |
| `Sample4_2010.pub` | 51.776692913 | 657564 | exact to numeric precision |
| `Sample_2010.pub` | 51.776692913 | 657564 | exact to numeric precision |
| `table__2x2.pub` | 72 | 914400 | exact |
| `table__3x2.pub` | 108 | 1371600 | exact |
| `table__merged.pub` | 72 | 914400 | exact |

All 24 real-file entries were rerun. Before → after: **0 → 0 OK,
11 → 19 with warnings, 2 → 2 refused, 11 → 3 failed**. The remaining failures
are the same three damaged clusterfuzz files. The eight repaired imports each
produce two nonblank PDF pages. **Limitation, not fixed:** their six table
cells remain empty; existing multi-story/TCD association warnings retain the
original story after the table. Visual review shows empty-looking table areas
on page 2; native heights matching the drawing is not a text/layout fidelity
guarantee. No new failure was found and this separate text issue was not fixed.
The 11 previously successful documents are unchanged after excluding creation/
modification timestamps, and their 15 rendered PDF pages are pixel-identical.
The fresh 142-fixture scores and warning inventory are unchanged.

### Follow-up: preserve the previous default row mode — 2026-10-19

Removed the explicit native `Exactly` assignment. The stored endings still
supply the configured row heights, checks and diagnostic exactly as above, but
the import no longer changes the existing default `AtLeast` mode. The observed
frame heights establish numeric geometry, not a fixed-height text-layout flag.
The synthetic test retains its row-position and single-warning assertions and
no longer asserts a mode. Configured heights matching drawn bounds are not a
promise that content-driven layout cannot expand rows.
