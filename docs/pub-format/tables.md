<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Publisher tables

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


## Row positions from stored row endings

Eight of the real-world samples (`Sample*.pub`, Apache POI test data at
`bf7c9b6a1490046517cbd8e1504e702a89be4bc9`) store row endings that differ from
the running sum of row sizes. Their drawn table frames are 657564 EMU high,
matching the last ending; the three controlled table samples match both values.
So row positions come from the stored endings. Why the quantities differ is not
known.

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
encodings.

## One table story among several

When a publication has several stories, one table and one `TCD ` section, the
table's text is recovered only if the `Contents` link (key 0x27) names the
story, the `TCD ` boundaries match that story and no other, and the cell records
are row-major. Otherwise the text is kept after the table with a warning; no
story is chosen by order and the directory occurrence field is not interpreted.

## Inconsistent stored dimensions — import approximation (issue #44)

A calendar publication (one table per month) was refused with "table:
inconsistent cumulative dimension". A table whose stored cumulative endings
do not add up no longer fails the file. The importer now:

- lays out columns from the stored sizes (key 0x02) and, when the column
  endings (key 0x01) disagree with their running sums, imports the table
  anyway and warns;
- lays out rows from the stored endings when they are positive, strictly
  increasing and in range (as before), and from the stored row sizes when they
  are not, with a warning.

Each warning names the spread (or master page), the grid size and the frame
position, and says what was approximated. Still refused: wrong dimension
count, zero or out-of-range sizes, size overflow, and sizes that do not add up
to the declared width or height. Which of size or ending is "right" for such
files is unknown; no sample of the reported file was available, so the
choice (sizes for columns, sizes for rows when endings are unusable) is an
approximation. Tests build the bytes in code; no Publisher file is committed.
Pinned counts before and after: 142/142 imports, 152/152 objects, 111/111
positions, 52/52 texts, 111/111 styles; real batch 9 / 10 / 2 / 3 unchanged.

## Cell text association (calendar sample)

OBSERVATION: this describer checkpoint starts from
`04795e6568ad89ad780691f568b279b38c817fa1` on the task's named base branch.
The user-supplied may-aug-2026.pub is exactly 1610752 bytes, SHA-256
`AD730D7AEBC2EB1BF93F04C537CC6F31C2B8F76504070A19D44147872C0CBF3A`,
verified before inspection. It is data only, not committed or redistributed.
Only root Contents (34224 bytes) and Quill CONTENTS (51200 bytes) are extracted
in an ignored research directory. No full text, raw bytes, pictures or private
metadata are reproduced here. Generic month/weekday labels are used only to
check associations. No external Publisher reader, specification, reference
output or scoring export supplies an encoding inference.

OBSERVATION: evidence is the attachment, the existing Contents/property-list,
Quill and table notes, bounded experiments, and read-only inspection of this
repository's current parsing/building paths. No importer, environment script
or setting changes are made. Fonts, colours, borders and cell fills are not
decoded. The section records a sampled association rule, not a finished fix
or a claim of native visual fidelity.

### Four tables, eight stories, four text partitions

OBSERVATION: offsets throughout this section are decimal absolute byte
positions in the named stream, unless prefixed `0x`. Record sequence numbers
and story IDs are decimal. Ranges are end-exclusive. All numeric probes are
little-endian. The existing directory and page-list rules yield four ordinary
pages with page records 266,310,317,324; each directly lists one type-0x10
table. There are also two labelled master records and four empty placeholders.
No drawing or appearance decoding is needed to establish these memberships.

| Basis | Page / month heading | Table sequence / Contents record offset | Grid / actual cells | Linked Quill story / UTF-16 length / global TEXT range | Cell-list sequence / Contents offset |
| --- | --- | --- | --- | --- | --- |
| OBSERVATION | 1 / May | 341 / 19178 | 7 rows x 7 columns / 49 | 41 / 123 / [41,164) | 342 / 19512 |
| OBSERVATION | 2 / June | 344 / 22298 | 6 rows x 7 columns / 42 | 42 / 114 / [164,278) | 345 / 22614 |
| OBSERVATION | 3 / July | 347 / 24992 | 6 rows x 7 columns / 42 | 43 / 116 / [278,394) | 349 / 25360 |
| OBSERVATION | 4 / August | 351 / 27738 | 7 rows x 7 columns / 49 | 44 / 123 / [394,517) | 352 / 28072 |

OBSERVATION: Quill's directory has two blocks: at 24, 19 entries and next
pointer 50688; at 50688, six entries and terminal pointer `0xFFFFFFFF`.
All 25 entries fit in the 51200-byte stream. TEXT is at 512, length 1034
bytes, exactly 517 UTF-16 units. SYID is at 22016, length 40, with count 8
at +4 and eight u32 IDs beginning at 22024. STRS is at 26012, length 86,
with count 8, length-array displacement 8 at +4, and lengths beginning at
26024 (= STRS+4+8). Their paired entries are:

| Basis | SYID zero-based slot | Story ID | STRS length / global TEXT unit range | Role supported by the association |
| --- | --- | --- | --- | --- |
| OBSERVATION | 0 | 21 | 9 / [0,9) | First month heading. |
| OBSERVATION | 1 | 23 | 10 / [9,19) | Second month heading. |
| OBSERVATION | 2 | 25 | 10 / [19,29) | Third month heading. |
| OBSERVATION | 3 | 27 | 12 / [29,41) | Fourth month heading. |
| OBSERVATION | 4 | 41 | 123 / [41,164) | First table's text. |
| OBSERVATION | 5 | 42 | 114 / [164,278) | Second table's text. |
| OBSERVATION | 6 | 43 | 116 / [278,394) | Third table's text. |
| OBSERVATION | 7 | 44 | 123 / [394,517) | Fourth table's text. |

OBSERVATION: all lengths sum to 517 without clamping. Each table story has
one TCD partition; the heading stories have none. The directory also names
TOKN entries with slots 4,5,6,7, and STSH, FDPP, FDPC, SGP, INK, BTEP, BTEC,
FONT, MCLD and PL sections. Their bodies are not needed or decoded for this
association. In particular, no cell-style or colour interpretation is derived
from MCLD, FONT or PL.

### Exact fields and the association chain

OBSERVATION: the table-object fields below are property-header positions in
Contents, meaning key at P, kind at P+1, numeric value at P+2. Key 0x27 is
kind 0x20 and selects the Quill story ID. Keys 0x66/0x67 give row/column
counts; key 0x6B is the cell-list sequence link. The existing notes establish
these keys; the attachment independently supplies the following values and
cross-stream joins.

| Basis | Table sequence | Key 0x27 header / value | Key 0x66 header / rows | Key 0x67 header / columns | Key 0x6B header / linked sequence |
| --- | --- | --- | --- | --- | --- |
| OBSERVATION | 341 | 19204 / 41 | 19212 / 7 | 19218 / 7 | 19236 / 342 |
| OBSERVATION | 344 | 22324 / 42 | 22332 / 6 | 22338 / 7 | 22356 / 345 |
| OBSERVATION | 347 | 25018 / 43 | 25026 / 6 | 25032 / 7 | 25050 / 349 |
| OBSERVATION | 351 | 27764 / 44 | 27772 / 7 | 27778 / 7 | 27796 / 352 |

OBSERVATION: a Quill section entry at E has tag at E+2, a u16 field at E+6,
section offset at E+16 and section length at E+20. For TCD, the E+6 values
are 4,5,6,7. Each equals the zero-based SYID/STRS slot of the table story
selected by Contents key 0x27. They are neither the actual story IDs
41,42,43,44 nor TCD-local occurrence ordinals 0,1,2,3. The earlier description
of E+6 as a same-tag occurrence number must not be applied universally to
these TCD entries without this new evidence and its checks.

| Basis | TCD directory entry E / E+6 field position | Stored slot / selected story | Section offset / size | Cell-boundary array / final value |
| --- | --- | --- | --- | --- |
| OBSERVATION | 464 / 470 | 4 / 41 | 23664 / 208 | [23676,23872) / 123 |
| OBSERVATION | 50696 / 50702 | 5 / 42 | 24388 / 180 | [24400,24568) / 114 |
| OBSERVATION | 50720 / 50726 | 6 / 43 | 25096 / 180 | [25108,25276) / 116 |
| OBSERVATION | 50744 / 50750 | 7 / 44 | 25804 / 208 | [25816,26012) / 123 |

OBSERVATION: each TCD has u32 (actual-cell-count minus one) at +0, zero at
+4, `0xFF00` at +8, then one u32 boundary per actual cell at +12+4*i.
The respective first values are 48,41,41,48. Exact section size is
`12+4*actual_cell_count`, with no ignored boundary or trailing bytes.
All boundaries refer to units within the selected story, not global TEXT
positions, section offsets or individual cell story IDs.

OBSERVATION: the type-0x63 cell-list record begins with the usual self-counting
record length. Its key-1/kind-0x18 count header is at record+4 (u16 value at
+6); key-2/kind-0xA0 array header is at +8, followed by the nested length
at +10 and first child entry at +14. The first child positions are
19526,22628,25374,28086. Parse each child's own key-0/kind-0x88 self-counting
list rather than assuming a fixed stride. Cell keys 1,2,3,4 give inclusive
first/last row and first/last column, with absent zero values as in the
existing notes. All 182 cells are unmerged, complete, non-overlapping and
serialized row-major. Thus child ordinal i agrees with row=i/7,col=i%7
and with TCD boundary ordinal i in this sample. No new per-cell text key is
needed to recover these ranges; unknown cell properties remain opaque.

INFERENCE: the supported calendar association chain is
Contents table key 0x27 -> SYID slot and STRS range -> TCD directory E+6
matching that slot -> TCD cell ordinal -> linked cell-list child ordinal
and its stored grid coordinates. This is a sample-supported rule, with
independent boundary/content checks below. It is not evidence that every
Quill tag's E+6 field has this meaning or that merged/non-row-major tables
can be assigned by ordinal without further evidence.

### Ranges, labels and false-positive controls

OBSERVATION: let b[i] be boundary i and N the actual cell count. Set a[0]=0;
for i>0, a[i]=b[i-1]+1. For nonfinal cells require story[b[i]]=CR and assign
[a[i],b[i]). For the final cell require b[N-1]=story length and remove its
terminal CR from the assigned range. Empty ranges are valid. Every boundary
is monotonic with respect to that next start, every separator is CR, and all
182 resulting ranges stay inside their story. Global TEXT byte positions
are `TEXT_offset + 2*(story_start + local_unit_position)`.

| Basis | Story | Weekday payload units | Day payload units / numbered cells | Empty cells | Total cell payload + separator units = story units |
| --- | --- | --- | --- | --- | --- |
| OBSERVATION | 41 | 21 | 53 / 31 | 11 | 74 + 49 = 123 |
| OBSERVATION | 42 | 21 | 51 / 30 | 5 | 72 + 42 = 114 |
| OBSERVATION | 43 | 21 | 53 / 31 | 4 | 74 + 42 = 116 |
| OBSERVATION | 44 | 21 | 53 / 31 | 11 | 74 + 49 = 123 |

OBSERVATION: each first row consists of seven generic three-character weekday
labels in Sunday-first order. Numbered cells form the consecutive day sequence
for the associated month, and their first/last slots match the independently
calculated month weekday and day count. No other payload category appears.
The first numbered-cell ordinals are 12,8,10,13 and the last are 42,37,40,43;
these are derived grid positions, not reproduced sample text. Month headings
are separate stories and must not be inserted into calendar cells. Complete
cell ranges are checked privately; only derived examples are recorded here:

| Basis | Story / cell ordinal / row,column | Story-local range | Check |
| --- | --- | --- | --- |
| OBSERVATION | All four / 0 / 0,0 | [0,3) | First weekday label, followed by its CR. |
| OBSERVATION | All four / 6 / 0,6 | [24,27) | Last weekday label in the heading row. |
| OBSERVATION | 41 / 12 / 1,5 | [33,34) | First numbered cell. |
| OBSERVATION | 42 / 8 / 1,1 | [29,30) | First numbered cell. |
| OBSERVATION | 43 / 10 / 1,3 | [31,32) | First numbered cell. |
| OBSERVATION | 44 / 13 / 1,6 | [34,35) | First numbered cell. |
| OBSERVATION | 41 and 44 / 48 / 6,6 | [122,122) | Final empty cell; terminal CR excluded. |

OBSERVATION: the complete false-positive matrix tries every TCD against all
eight stories, using exact final length, all separator positions and bounded
ranges. Exactly four of 32 combinations pass, the intended slot-selected
ones; 28 fail. The heading stories all fail. In particular, stories 41 and
44 both have length 123 and grid size 7x7, but swapping their TCD sections
fails separator checks in both directions. Merely matching length/count
would incorrectly accept those alternatives. Of all 24 permutations of the
four table-story associations, only the intended one passes every check.
Treating E+6 directly as a story ID matches none; shifting the proposed SYID
slot by one produces three length mismatches and one out-of-range slot,
so it also fails. No generic field-value density or significance claim is used.

INFERENCE: exact range validation, existing Contents story links and independent
calendar label/day-position checks support assigning all 182 unmerged cells
in this file. The equal-length negative control is stronger than choosing a
story because it is next in directory order. UNKNOWN: a second independent
multi-table file with identical boundary patterns is still needed to test
whether E+6 is generally a SYID ordinal rather than some correlated index.
When independent checks cannot disambiguate, retain text and warn.

### Why the existing importer retains an overset story

OBSERVATION: the current Quill path assigns cell ranges directly only for
one TCD and one story. Otherwise it emits the reported multiple-stories/TCD
warning. The later single-table recovery returns without assigning ranges
when there is more than one table. This sample has four, so neither recovery
path supplies cell ranges. The table builder consequently creates the grid
with empty cell stories, inserts its anchor into the original frame story,
and retains the original text after the table with the reported warning.
That code-path explanation requires no guessed cell association or appearance.

OBSERVATION: the frame-text conversion removes the final CR and normalizes
remaining CRs to newlines. Here every unit is ASCII, so the four retained
texts have 122,113,115,122 characters. Inserting the table at the beginning
adds one table anchor and one newline, giving host totals 124,115,117,124.
These calculated totals align with the user-reported 115-to-124 range.
The four stories contain 294 cell-payload units and 182 separators in total;
their characters belong to the 182 cell ranges above, not to an extra free
text paragraph following an empty calendar. The four month-heading stories
are separate and are not that overflow.

INFERENCE: populating the verified cell stories and removing the retained
duplicate source text from the host would address the identified association
failure. UNKNOWN: exact composed hidden counts and visual fit were not rerun
or proved here. No font metrics or layout explanation is inferred, and a text
association fix alone does not prove every remaining layout issue disappears.
No sample render, native save or importer modification is performed.

### Builder requirements and limits

INFERENCE: a builder can use this sample-bounded association when all checks
below succeed; otherwise preserve the source text with a bounded warning.

- INFERENCE: bound the Contents trailer/directory, self-counting property
  lengths and cell-list links; require type-0x10 table and type-0x63 cell list,
  positive bounded row/column counts and exact cell-count agreement.
- INFERENCE: traverse every Quill directory block, with cycle/depth and total
  entry limits; bound E+6/E+16/E+20 and every section. This sample crosses a
  directory-block boundary between its first and remaining TCD entries.
- INFERENCE: bound SYID/STRS counts and arrays; reject duplicate/ambiguous story
  IDs or range overruns rather than relying on clamping. Require key 0x27 to
  resolve to the same SYID slot selected by the candidate TCD E+6 value.
- INFERENCE: require exactly one independently valid TCD association per table
  story, no conflicting reuse, exact `12+4*N` size, the observed header, and
  N matching actual cells. Require the terminal boundary to equal story length
  and every intermediate separator to be CR, including empty cells.
- INFERENCE: require the unmerged row-major cell correspondence verified here;
  validate stored rectangles, occupancy and all local/global range arithmetic.
  Do not silently reuse ordinal mapping for merged or non-row-major cases.
- INFERENCE: convert UTF-16 ranges to the normalized story representation with
  checked character boundaries, preserving empty cells and excluding separators.
  Invalid surrogate handling or normalization must not shift later ranges.
- INFERENCE: create each cell story from its verified slice, then keep only the
  table anchor in the host instead of retaining the duplicate source text.
  Preserve the separate heading stories. Maintain the existing aggregate limits:
  100000 grid cells, 16000000 table-text units and bounded range/copy work.

UNKNOWN: ranked questions are (1) general E+6 association outside this one multi-table file,
especially identical boundary patterns and reordered story lists;
(2) merged/non-row-major cells and threaded/shared table stories, absent in
this sample; (3) exact visual overset after a builder change; (4) malformed
or ambiguous inputs beyond these checks. Fonts, colours, borders, fills and
other appearance remain outside this research, not inferred from text fields.

INFERENCE: the next smallest discriminating step is an independently authored
two-table sample with equal story lengths and identical separator patterns,
so boundary validation alone cannot choose a story; vary story-directory order
to test whether E+6 still selects the SYID slot. Until then, retain independent
Contents/range checks rather than treating section order as authoritative.

OBSERVATION: this is the first calendar-sample checkpoint. Original identity,
bounded directory/property parsing, all four joins, all 182 cell ranges and
rectangles, the 32-way association matrix and 24 permutations pass their stated
checks; wrong equal-length pairings are explicitly rejected. The calculated
fallback host totals are checked against the read-only source paths, not a
render. Only this appended section and one source-ledger line change. No full
CI, importer edit, environment script/setting change, environment save/publication,
pull request or extra remote is involved. Sample and extracted bytes remain
ignored and uncommitted.

## Cell text association: implementation status

IMPLEMENTATION (checked by synthetic byte-built files in `crates/pub/tests/table_cell_text.rs`; no Publisher-made multi-table file is used by the tests): a table's cell text is placed when all of these hold.

- The Quill directory is walked across every block with cycle, depth (64 blocks) and entry (100000) limits, and each `TCD ` entry's E+6 field and bounded section are read.
- The SYID and STRS arrays are bounded, have the same count, hold no repeated story id, and their lengths tile TEXT exactly with no clamping.
- The table's story (Contents key 0x27) is used by that table only, appears once in SYID, and its zero-based SYID slot equals the E+6 field of exactly one `TCD ` entry.
- The cells are complete, unmerged and in row-major order, and the grid size equals the cell count.
- The `TCD ` entry has the exact size `12+4*N`, the header `N-1`, 0, `0xFF00`, and N equal to the cell count; the last boundary equals the story length; the story ends with CR; every earlier boundary lands on CR; ranges are checked and never clamped.

The cell stories are then cut from the verified slices (empty cells kept, separators left out), and only the table anchor stays in the host frame. The month-heading stories are untouched. With one table and one `TCD ` entry the earlier rule is kept (the Contents link plus boundary fit, no E+6 check).

FALLBACK: a table that fails any check keeps today's behaviour (empty cells, the story text after the table, the "unconfirmed or missing" warning) and gets one warning naming the reason; at most 8 such warnings are written, then one count line. The older "multiple stories/TCD sections" warning is removed only when every table and every `TCD ` entry was placed. Other tables in the same file are still placed.

UNKNOWN (unchanged): whether E+6 is a SYID slot in files other than the one calendar sample; merged or non-row-major cells, and threaded or shared table stories (all fall back); the visual fit after the change (not rendered here).

FOLLOW-UP (calendar pages 3 and 4, last row missing): INFERENCE from the code and a reproduction, not from the real file. The host story of a placed table is the table anchor plus one newline (2 characters), which matches the report of 2 characters that do not fit. The composer grows each row to fit its text, so a table can be taller than the frame stored for it; the rows that do not fit the frame are then dropped and the story is reported as overset. The cell ranges, including the last, were not at fault: the byte-built reproduction (`a_month_ending_mid_row_keeps_its_last_row_when_the_frame_is_short`) has a last row holding two numbers and an empty cell, and it lost that row only because the frame was shorter than the rows. Fix: after the frame is fully set up, a host that holds only the table anchor is composed, and if it is overset its frame grows downward to the height the composer reports (text insets included), at most 4000 pt, with one warning giving the growth. A frame that already fits is untouched; text kept after a table (fallback) is never hidden by growing. UNKNOWN: which cell content or stored size made the July and August rows taller than their frames; this cannot be told without those files.
