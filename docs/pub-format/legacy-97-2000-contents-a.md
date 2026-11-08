# Legacy Contents variant A: first structural checkpoint

## Scope and evidence

OBSERVATION: task AP starts from main `b90d876c0d069c903ea3714e2be96db9c2ca0a4c`.
The evidence is the pinned binary `Sample98.pub` (A), followed by a bounded
comparison with `Sample2000.pub` (B), and the existing repository notes. The
names A/B are task labels, not independently established product releases.
No external Publisher reader, format description, scoring JSON or reference
image was used to infer storage. No new external specification was consulted.

OBSERVATION: the existing restore command completed with 317 files restored,
zero failures, and all recorded sizes/hashes verified. A separate binary-only
check matched all 166 publications against the fixture CSV; inventories are
142 controlled and 24 Apache publications. Only the two named publications
were opened for this investigation. The originals remain outside the checkout;
only the needed Contents, Quill CONTENTS and root CompObj stream copies are in
the ignored `target/ap-research/` directory.

| Basis | File | File bytes | SHA-256 |
| --- | --- | ---: | --- |
| OBSERVATION | Sample98.pub | 61440 | `8912f295ff0cd1f55b873b3b98a182218f33c420877e025d2f0fd795f12dd0c0` |
| OBSERVATION | Sample2000.pub | 61440 | `40701ca47b26d04771cdd58467764e9ab39d3da69b59fbc529afd16d263d2f86` |

OBSERVATION: the owner's `mspub.pub` and `zmspub.pub` are not available here;
no matching filename was found under `/workspace`. UNKNOWN: their reported
identical 4152-byte Contents streams, hashes, layout and comparison with A
could not be verified.

## Container facts

OBSERVATION: the existing compound-file dependency opens both containers as
version 3 and returns root CLSID `{00021201-0000-0000-00C0-000000000046}`.
Every stream below was opened and read; its actual byte count equals its
directory length. Control characters in paths are escaped.

| Basis | Stream | Sample98 bytes | Sample2000 bytes |
| --- | --- | ---: | ---: |
| OBSERVATION | `/\x01CompObj` | 94 | 94 |
| OBSERVATION | `/\x03Internal` | 12 | 12 |
| OBSERVATION | `/\x05DocumentSummaryInformation` | 152 | 152 |
| OBSERVATION | `/\x05SummaryInformation` | 44640 | 44640 |
| OBSERVATION | `/Contents` | 5930 | 6042 |
| OBSERVATION | `/Envelope` | 0 | 0 |
| OBSERVATION | `/Escher/EscherDelayStm` | 0 | 0 |
| OBSERVATION | `/Escher/EscherStm` | 0 | 0 |
| OBSERVATION | `/Quill/QuillSub/\x01CompObj` | 86 | 86 |
| OBSERVATION | `/Quill/QuillSub/CONTENTS` | 6144 | 6144 |

OBSERVATION: root CompObj bytes are identical and contain the NUL-terminated
ASCII strings `Microsoft Publisher 3.0` and `MSPublisher.3` (the latter starts
at CompObj byte 60). UNKNOWN: these strings and the root CLSID do not verify
the product release that last saved either publication. Empty Escher streams
do not demonstrate empty visible publications.

## Header and regions

OBSERVATION: all offsets below are hexadecimal byte positions relative to
root Contents; all numeric checks use little-endian integers. There is no
four-byte alignment rule: important u32 fields occur at `0x12`, `0x16` and
`0x1A`, and directory entries are ten bytes wide.

| Basis | Offset / width | A value | B value | Check and supported interpretation |
| --- | --- | --- | --- | --- |
| OBSERVATION | `0x00 / 4` | `E8 AC 22 00` | same | Exact prefix comparison; existing old-family selector, not an exact release. |
| INFERENCE | `0x02 / 2` | 34 | 34 | Equals the first table position and the u32 at `0x1A`; candidate header byte length, not independently varied. Overlaps the selector. |
| UNKNOWN | `0x04 / 2` | 300 | 300 | Literal value only; version meaning unverified. |
| UNKNOWN | `0x06 / 2` | `0x0105` | same | Literal value only. |
| OBSERVATION | `0x08 / 4` | 5930 | 6042 | Equals the complete extracted Contents length in each file, including the header; zero-based word 2. |
| UNKNOWN | `0x0C / 2` | 616 | 717 | Neither equals the observed directory count, maximum record ID, layout-table extent or stream length. No count/ID role established. |
| INFERENCE | `0x0E / 2` | 256 | 256 | Equals the first directory ID and its root-like record; candidate root ID. Constant agreement alone does not settle semantics. |
| UNKNOWN | `0x10 / 2` | 0 | 0 | No independent meaning established. |
| OBSERVATION | `0x12 / 4` | `0x37E` | `0x3BA` | Points exactly to `DE F0 2E 00`; the 46-byte control region ends at the directory pointer. |
| OBSERVATION | `0x16 / 4` | `0x3AC` | `0x3E8` | Points exactly to a 54-entry directory with in-range, ascending payload offsets. |
| OBSERVATION | `0x1A / 4` | `0x22` | `0x22` | Points to the pre-control table header, not a current-family trailer. A's descriptor walk ends exactly at `0x37E`. |
| UNKNOWN | `0x1E / 2` | 248 | 252 | No length/count/ID role established. |
| UNKNOWN | `0x20 / 2` | 12 | 12 | No entry-width meaning established; the measured directory stride is 10. |

OBSERVATION: A partitions into `[0,0x22)` header; `[0x22,0x37E)` layout-table
candidate; `[0x37E,0x3AC)` control region; `[0x3AC,0x5CC)` directory;
`[0x5CC,0x5E0)` twenty zero bytes; `[0x5E0,0x172A)` directory-addressed
payload spans. These regions total exactly 5930 bytes.

OBSERVATION: both control regions begin with u16 `0xF0DE`, then u16 46.
At control-relative `+0x0A` and `+0x0E`, u32 values both equal that file's
directory offset. The other control bytes are identical between A and B.
INFERENCE: 46 is a self-inclusive control-region length, supported by both
exact boundary checks. UNKNOWN: the two pointer roles, `0xF0DE`, and other
control fields are not semantically decoded.

## A's pre-control table

OBSERVATION: A has u16 89 at `0x22`, u16 6 at `0x24`, then a reproducible
walk starting at `0x26`: read three u16 words, followed by u16 values through
and including a zero terminator. Repeating this consumes exactly 88 entries
and stops at the header's control offset `0x37E`, without leftover bytes.
Zero-valued entries still occupy an entry number.

INFERENCE: 89 is a one-past entry index, or a slot count including an implicit
zero slot; the observed 88 entries support both interpretations. Calling it
89 serialized entries would consume control bytes and is wrong for A.
UNKNOWN: the meaning of 6, the three words, and the terminated values remains
unproved. The latter resemble field offsets but are not established as such.

| Basis | One-based entry | Descriptor position | Three words | Terminated nonzero values |
| --- | ---: | --- | --- | --- |
| OBSERVATION | 1 | `0x26` | `6868, 0001, 0008` | none |
| OBSERVATION | 8 | `0x6E` | `7688, 0009, 0002` | none |
| OBSERVATION | 10 | `0x7E` | `747D, 0001, 0008` | `006A,006C,006E` |
| OBSERVATION | 20 | `0xD6` | `1E1E, 0019, 0002` | `0004,0006,001A,001C` |
| OBSERVATION | 21 | `0xE6` | `B2D2, 0019, 0002` | 32 values; first `0004,0006,0008`, last `008A,00B0,00D0` |
| OBSERVATION | 51 | `0x22E` | `0404, 0004, 0000` | none |
| OBSERVATION | 78 | `0x326` | `0A0A, 0001, 002D` | none |
| OBSERVATION | 88 | `0x376` | `0404, 0004, 0000` | none |

OBSERVATION: every directory-addressed A payload starts with a u16 in `1..88`.
Repeated values select similarly headed payloads, and several next words
match the corresponding descriptor's first word. They do not always match:
entry 8 has descriptor word `0x7688`, but its payloads have `0x767E`;
entry 10 has descriptor word `0x747D`, but its payloads have `0x7473`.
INFERENCE: the first u16 is a layout/type-table index. UNKNOWN: differences
between descriptor and instance words, inheritance, flags and variable tails
remain undecoded; the first word is not a verified self-inclusive byte length.

OBSERVATION: B has u16 94 at `0x22`. Applying A's simple descriptor walk does
not preserve the boundary/count relationship. Its table is deliberately left
undecoded at this checkpoint. INFERENCE: A's walk must not become a shared
old-family reader based on this one sample.

## Directory and record framing

OBSERVATION: at A `0x3AC`, the first u16 is 54. Following a four-byte directory
header, exactly 54 ten-byte entries fit before twenty zero bytes and the first
addressed payload. Each entry has these independently checkable fields:

| Basis | Relative offset / width | Evidence |
| --- | --- | --- |
| OBSERVATION | `+0 / 2` | 54 distinct identifiers; first `0x100`; not consecutive directory ordinals. |
| INFERENCE | `+2 / 2` | Owner/parent candidate: 53 values resolve to another directory ID; root `0x100` instead has `0xFF`. Candidate page collections agree with these relationships. |
| OBSERVATION | `+4 / 4` | 54 distinct ascending absolute Contents offsets, all after the directory and before end-of-stream; first `0x5E0`, last `0x1714`. |
| UNKNOWN | `+8 / 2` | Four literal words: `1640,16C0,16E0,1740`; flags/class meaning not decoded. |

OBSERVATION: directory-header u16 at `0x3AE` is `0x141`, exactly the largest
entry ID. INFERENCE: maximum-ID/high-water field; its maintenance rule is
UNKNOWN. The final entry is ID `0x114`, demonstrating that ID order is not
physical record order. B repeats the count, maximum, ID order and ownership
relationships, with relocated offsets; its directory ends at `0x608` and
has ten zero bytes before first payload `0x612`.

OBSERVATION: the table below records every A payload's directory-derived
span. End is the next directory offset, or stream length for the final entry.
IDs, parent candidates and starts are hexadecimal; spans and indices are
decimal. These are bounded spans, not independently proved logical lengths: an
unrecognized tail or padding may be included. Index is the payload's first
u16. All rows are OBSERVATION.

| ID | Parent candidate | Start | Span bytes | Index |
| --- | --- | --- | ---: | ---: |
| 100 | FF | 05E0 | 202 | 21 |
| 11F | 100 | 06AA | 294 | 41 |
| 120 | 11F | 07D0 | 112 | 54 |
| 123 | 100 | 0840 | 194 | 40 |
| 12C | 100 | 0902 | 60 | 20 |
| 133 | 12C | 093E | 150 | 2 |
| 134 | 12C | 09D4 | 150 | 3 |
| 135 | 12C | 0A6A | 67 | 4 |
| 136 | 12C | 0AAD | 70 | 5 |
| 137 | 12C | 0AF3 | 61 | 6 |
| 138 | 12C | 0B30 | 49 | 7 |
| 139 | 12C | 0B61 | 128 | 8 |
| 13A | 12C | 0BE1 | 61 | 9 |
| 13B | 12C | 0C1E | 126 | 10 |
| 13C | 12C | 0C9C | 86 | 11 |
| 102 | 100 | 0CF2 | 34 | 67 |
| 103 | 100 | 0D14 | 30 | 49 |
| 11E | 103 | 0D32 | 26 | 50 |
| 119 | 100 | 0D4C | 26 | 30 |
| 105 | 100 | 0D66 | 26 | 31 |
| 101 | 100 | 0D80 | 30 | 49 |
| 104 | 101 | 0D9E | 26 | 50 |
| 131 | 100 | 0DB8 | 14 | 55 |
| 132 | 100 | 0DC6 | 14 | 56 |
| 124 | 100 | 0DD4 | 56 | 57 |
| 11D | 100 | 0E0C | 192 | 71 |
| 11A | 100 | 0ECC | 8 | 69 |
| 11B | 11A | 0ED4 | 70 | 76 |
| 11C | 11A | 0F1A | 244 | 82 |
| 106 | 100 | 100E | 40 | 51 |
| 117 | 100 | 1036 | 40 | 20 |
| 107 | 100 | 105E | 40 | 20 |
| 121 | 107 | 1086 | 58 | 44 |
| 108 | 107 | 10C0 | 22 | 51 |
| 10A | 100 | 10D6 | 44 | 20 |
| 10B | 10A | 1102 | 16 | 51 |
| 125 | 10A | 1112 | 128 | 8 |
| 126 | 10A | 1192 | 128 | 8 |
| 127 | 100 | 1212 | 50 | 20 |
| 128 | 127 | 1244 | 22 | 51 |
| 12A | 127 | 125A | 128 | 8 |
| 12B | 127 | 12DA | 166 | 10 |
| 141 | 12B | 1380 | 290 | 78 |
| 140 | 12B | 14A2 | 86 | 79 |
| 13F | 12B | 14F8 | 50 | 80 |
| 12E | 127 | 152A | 70 | 5 |
| 12F | 127 | 1570 | 128 | 8 |
| 130 | 127 | 15F0 | 128 | 8 |
| 10D | 100 | 1670 | 40 | 20 |
| 10E | 10D | 1698 | 22 | 51 |
| 110 | 100 | 16AE | 40 | 20 |
| 111 | 110 | 16D6 | 22 | 51 |
| 113 | 100 | 16EC | 40 | 20 |
| 114 | 113 | 1714 | 22 | 51 |

OBSERVATION: payload starts `0xAAD`, `0xAF3`, `0xB61` and `0xBE1` are odd.
Treating every payload as an aligned u32-length-prefixed property list would
not reproduce this framing. UNKNOWN: a standalone sequential record-length
rule without the directory has not been decoded.

## Quill anchors and content

OBSERVATION: the documented section-directory checks on both actual Quill
streams locate TEXT at `0x200`, length 976 bytes; SYID at `0xC00`, length 32;
STRS at `0x11FE`, length 82; and TCD at `0xFA6`, length 36. SYID contains six
IDs `1,3,4,6,7,8`. STRS's first six lengths at `0x120A` are
`86,98,62,80,87,75` UTF-16 units, summing to exactly TEXT's 488 units.
Each resulting text slice ends with CR. Trailing STRS bytes are not decoded
by that length extraction.

OBSERVATION: TEXT bytes are identical between A and B (SHA-256
`a196274032d36cc28738b12ef0a8a030c4830cc785a2f6e9219782902a2abaf6`).
The earlier note's extra newline came from inspecting a contiguous readable
run beyond the TEXT section: the directory-bounded TEXT here is 488 units,
not 489. No frame rendering or visible-page validation was performed.

OBSERVATION: index numbers below are decimal, record IDs and positions are
hexadecimal. The following u16 values in directory-addressed payloads equal
all six Quill IDs, in their text order. Short text prefixes identify stored
content, not independently verified visible output. Positions are absolute
Contents offsets; ranges are UTF-16 units relative to Quill TEXT.

| Basis | Record ID / index | A start / field | B start / field | Value | TEXT range; stored prefix |
| --- | --- | --- | --- | ---: | --- |
| OBSERVATION | `125 / 8` | `1112 / 116A` | `117A / 11D2` | 1 | [0,86); This is some text on the first page |
| OBSERVATION | `126 / 8` | `1192 / 11EA` | `11FA / 1252` | 3 | [86,184); This is in bold and italic |
| OBSERVATION | `12A / 8` | `125A / 12B2` | `12C4 / 131C` | 4 | [184,246); This is the second page |
| OBSERVATION | `12B / 10` | `12DA / 1340` | `1344 / 13AA` | 6 | [246,326); Table on page 2 |
| OBSERVATION | `12F / 8` | `1570 / 15C8` | `15DA / 1632` | 7 | [326,413); This text is on page two |
| OBSERVATION | `130 / 8` | `15F0 / 1648` | `165A / 16B2` | 8 | [413,488); More text, more hyperlinks |

INFERENCE: index-8 payload relative `+0x58` and index-10 relative `+0x66`
are story-link candidates. The six exact ID matches, section-bounded text
ranges and ownership pattern support this association. UNKNOWN: whether
these fields are semantically u16 or wider integers with zero upper bytes,
and whether the offsets apply beyond these two files.

OBSERVATION: A record `0x10A` at `0x10D6` ends with u16 IDs `0x125,0x126`
at relative `+0x28`; those two entries name `0x10A` as owner/parent candidate.
Record `0x127` at `0x1212` ends with `0x12A,0x12B,0x12E,0x12F,0x130`
at relative `+0x28`; all five entries name `0x127` as owner/parent candidate.
INFERENCE: these are page-like collections: the first owns text mentioning
the first page, and the second owns text mentioning the second page and its
table. UNKNOWN: page ordering, master/scratch roles, complete list framing,
geometry and actual visibility are not established by those words alone.

## Constants, differences and stopping point

OBSERVATION: between A and B the selector, header table pointer, control
length, directory count, maximum ID, directory ID/parent order, six candidate
story links and actual text all remain constant. Contents grows by 112 bytes;
control/directory positions move by 60, first payload moves by 50, and later
record starts move by different amounts. For example `0x12C` spans 60/62
bytes, `0x10A` spans 44/46, and `0x127` spans 50/52, while the six linked
text/table payloads keep their respective 128/166-byte spans.

OBSERVATION: within A, distinct text payloads share index 8, but carry distinct
candidate story IDs and other differing bytes. Two non-user-linked index-8/10
payloads also occur under `0x12C`; a type-like index alone does not prove a
visible text frame. The repeated indices and descriptor words are structural
constants within this sample, not universal constants across releases.

UNKNOWN: no changed-text or changed-layout control exists in this pair, so no
byte difference between A and B can be attributed causally to content. The
text is unchanged despite Contents differences. Exact release, save-history
and allocation effects are not separated. There is no claim that A is fully
decoded or B understood.

## Ranked unknowns and next smallest step

1. UNKNOWN: general descriptor semantics and reference relocation; B defeats
   the simple A walk. OBSERVATION: AP2 narrows A index-20 to a boundary of
   30 bytes, a ten-byte tail header, two-byte IDs and four fixed-reference
   selectors, all checked on eight records. OBSERVATION: AP3 extends those
   checks to six agreeing records in a second structurally matching sample.
   OBSERVATION: AP4 checks a shared prefix high-byte/descriptor-high-byte
   boundary in all 93 records and exact array-tail extents in 44 records.
   OBSERVATION: AP10 audits the 41 selector families previously identified
   for descriptors 10/20/21/41/49: fifteen have repeated directory-parent
   corroboration, nine have one corroborated hit, sixteen are zero-only,
   and descriptor 21's +208 is outside both record spans. Descriptor 41's
   +16 has one child-ID hit and one zero, so remains probable.
   OBSERVATION: AP11 extends the bounds census to all 170 instantiated
   selector occurrences: +208 fails twice and descriptors 2/3's +370 fail
   four times. Record-relative u16 reads preserve all 46 prior corroborated
   targets; tested tail bases and a uniform -32 shift preserve none.
   UNKNOWN: authoritative duplicate fields in index-20, general relocation
   (including +208/+370 mappings), optional versus stale +208 applicability,
   zero-only roles and the meaning of descriptor `0x0019`; descriptor stride
   is not universal. No conditional flag/count rule for +208 is established.
   OBSERVATION: AP12 parks +208/+370 as UNKNOWN at the task's direction;
   their bounds failures are not used to block the content-structure research.
2. UNKNOWN: universal framing and internal padding interpretation beyond
   the tested extent rules; semantic meanings of all four directory
   attribute values; header words `0x0C`, `0x1E`, `0x20`, `0x24`.
   OBSERVATION: AP4 accounts for 70 spans by three field-driven arithmetic
   rules. AP6/AP7 add nine sampled class-2/class-10 extents with the
   unchanged field-driven rule `H+22+n*(4+w)`. AP8 adds all twelve class-3
   records with `H+18+q`, verified by a count-selected relative-offset table
   and stored q, including unequal payload widths in index 30. AP9 extends
   that field/table rule to both class-195/index-41 records and freshly
   checks all class-selected predictions: 93/93 endpoints now agree
   independently of their directory bounds, with no unexplained sampled
   extent. UNKNOWN: duplicate count roles, nonempty index-49 behavior,
   internal padding, general framing outside these files and flag-bit
   meanings remain unproved. No single tested field layout covers every
   class; the fixed-width rule is not universal.
   OBSERVATION: the nine-file replication adds zmspub as a third A file;
   unchanged class-selected checks pass 132/132 A records across 3/9 files.
   Five newly supplied files instead have a distinct zero-based 64-entry
   layout and fail complete transfer; they are not additional A instances.
3. UNKNOWN: root/page ordering and master/scratch classification. OBSERVATION:
   AP2 accounts for all 24 direct directory children of the eight index-20
   records: 17 tail IDs and seven fixed-field IDs. OBSERVATION: AP3 accounts
   for another 12 matching directory edges, eleven tail IDs and one fixed ID.
   OBSERVATION: the conservative AP10 selector graph, augmented only with
   prior proven index-20 tails, has 38/27 edges and 16/12 indegree-zero
   vertices in Sample98/mspub; both are partial forests, with no cycles or
   dangling included IDs and every edge agreeing with the parent candidate.
   OBSERVATION: AP12 verifies all thirteen root-list references against
   directory parents and establishes six Sample98 text-linked record chains
   through collections `0x10A`/`0x127` to root-like `0x100`. The two groups
   have first-/second-page wording, but no independent page-count field is
   decoded; the eight/seven direct class-25 collections are not proven pages.
   OBSERVATION: AP13 links the single Sample98 TCD section uniquely to story
   6 in record `0x12B` under `0x127`; its six boundaries subdivide that one
   story, not the six text records or two collections. TCD supplies no
   independently decoded page number or cross-collection grouping.
   UNKNOWN: reference roles and child-list rules for other classes or beyond
   these two samples; page identity/order and visual reading order remain
   unproved. File/list order is not independently established page order.
4. UNKNOWN: general text-frame semantics, field widths, geometry, formatting,
   table-cell ownership and displayed visibility. OBSERVATION: AP12 rechecks
   six Sample98 Quill associations at index-8 +0x58 and index-10 +0x66;
   ranges come from Quill SYID/STRS/TEXT, with no nearby literal range pair
   proven in Contents. mspub has no TEXT/STRS and zero SYID count, supplying
   a negative control, not a second positive text-link example.
   OBSERVATION: AP13 independently verifies TCD directory slot 3 against
   SYID's fourth ID, 6, and six CR-bounded local ranges totaling the 80-unit
   story; all five alternative story associations fail. No legacy cell-grid
   association is inferred. The existing import failure on kind `0x19` was
   not repaired or bypassed into a complete import.
   OBSERVATION: AP14 finds root +0x14/+0x18 size candidates matching A4 and
   Letter at one common scale, and a unique common signed32 window at
   +0x06/+0x0A/+0x0E/+0x12 for all six positive text-linked records. A
   half-page translation makes every candidate rectangle positive, distinct
   and contained. UNKNOWN: the physical scale and origin are still inferred;
   alternative contained translations and three background-field hits prevent
   treating numerical plausibility as proof of geometry semantics. mspub's
   index-8/index-10 fields are zero and provide no positive geometry replication.
5. UNKNOWN: independent saving versions and wider coverage, including any
   confirmed Publisher 97-specific sample. OBSERVATION: AP3 verifies the
   supplied mspub.pub size/hash and structural checks; its earlier unavailability
   is resolved. OBSERVATION: the nine-file replication verifies zmspub.pub
   and classifies it as A, with 39/39 agreeing extents and an empty Quill
   store. Its Contents differs from mspub at 49 byte positions, including
   A4 versus Letter size candidates; the claimed Contents equality is
   refuted for these hashes. UNKNOWN: independent manufacture remains
   owner-reported provenance; no new positive A text/frame example is added.
   OBSERVATION: AP15 inventories 170 resolved publication files in the searched
   workspace/restored-data locations; only Sample98 and mspub qualify as A.
   One matches B, 164 match the newer-family discriminator, and three fail
   container validation and remain unclassified. No additional positive
   replication is possible from these available files.
   OBSERVATION: that AP15 availability statement is superseded by the seven
   verified attachments: the nine named files now comprise A 3/9, B 1/9,
   and neither 5/9. New size probes do not supply known physical positions.

INFERENCE: after AP14, the smallest discriminating position test is an
independently saved variant-A page of known physical size with one unrotated
text frame at documented noncentral left/top/right/bottom positions. Test
the candidate scale and half-page origin against that known rectangle before
promoting them to builder rules. UNKNOWN: page roles/order, a second positive
geometry example and actual visible placement remain unverified; selectors
stay parked and the kind-0x19 failure remains outside this research.

## What a reader must do before implementation

- INFERENCE: validate the container, selector, actual stream length and each
  unaligned header pointer with bounded arithmetic; never identify a release
  from the filename or CompObj alone.
- INFERENCE: decode the applicable descriptor-table framing independently;
  preserve explicit unknowns instead of forcing B through A's walk.
- INFERENCE: bound directory counts and payload spans, retain explicit IDs,
  validate references, and guard cycles before assigning ownership semantics.
- INFERENCE: establish actual record lengths and variable-tail rules; do not
  substitute the current-family property-list/trailer encoding.
- INFERENCE: read section-bounded Quill text and validate ID/length joins;
  retain text separately from unproved visibility or geometry claims.
- INFERENCE: require differentiated samples or equivalent direct byte evidence
  before implementing page order, shape placement, tables or formatting.

## Checks and limits

OBSERVATION: the local bounded structural check passes for both stream lengths,
control boundaries/pointers, 54-entry directories, unique IDs, resolvable
non-root parent candidates, ordered in-range spans, all six Quill slices and
candidate links, and identical text/CompObj bytes. A's 88-descriptor walk and
all 54 in-range one-based indices also pass. B's descriptor layout is excluded
from that success claim. The full directory-derived A span table was retained
to make these checks reproducible without publishing sample bytes.

OBSERVATION: this is a documentation-only checkpoint. No importer, protected
reader, test suite, environment script or setting was changed. No full CI,
corpus import, application render, native save, deployment or environment
publication was run. Research scratch files remain ignored and uncommitted.

## Index-20 child-ID tail

OBSERVATION: AP2 continues from `70ed64d1f8a25e31c286d806fe132197fdfc439b`,
using only the same pinned Sample98.pub and existing notes. A fresh extraction
of root Contents matches the prior 5930-byte stream and SHA-256
`55fd826e7fa0b1b08ffc6be72c9927c7eab10810934e266cea08a5033f977553`;
the original publication still matches the sample hash above. The corpus
folders contain 142 controlled and 24 Apache publications. No new evidence
source was used, so SOURCES.md is unchanged. No B table, Quill payload or
other record-class body was decoded in this step; the directory and each
payload's leading index were sufficient to select every reachable index-20.

OBSERVATION: numbers prefixed `0x` are hexadecimal; counts and byte sizes are
decimal. Record-relative offsets include the leading two-byte index. The
44-byte `0x10A` starts at Contents `0x10D6` and ends at the next directory
offset `0x1102`; the 50-byte `0x127` starts at `0x1212` and ends at `0x1244`.
In both, bytes `+0x00..+0x03` are `14 00 1E 1E`: index 20 followed by two
equal boundary-like bytes. The complete observed tail-header shape is two
count-like u16 words, a u16 width and four zero bytes, then u16 IDs.

| Basis | Field / record-relative offset | 0x10A absolute bytes / value | 0x127 absolute bytes / value | Check and limit |
| --- | --- | --- | --- | --- |
| OBSERVATION | Boundary-like bytes `+0x02`, `+0x03` | `0x10D8..0x10DA`: `1E 1E` | `0x1214..0x1216`: `1E 1E` | Each byte is 30: exactly the offset where the first count-like word begins. Same value in descriptor word 0's two bytes. |
| OBSERVATION | First count-like u16 `+0x1E` | `0x10F4..0x10F6`: `02 00` = 2 | `0x1230..0x1232`: `05 00` = 5 | Equals actual number of tail IDs resolving to matching directory children; ten-byte header plus count times width reaches the next directory offset exactly. |
| OBSERVATION | Second count-like u16 `+0x20` | `0x10F6..0x10F8`: `02 00` = 2 | `0x1232..0x1234`: `05 00` = 5 | Repeats the first count and the actual number of IDs. A distinct used-count, allocated-count or element-length role is not distinguished by these equal values. |
| OBSERVATION | Width u16 `+0x22` | `0x10F8..0x10FA`: `02 00` = 2 | `0x1234..0x1236`: `02 00` = 2 | Independently, remaining bytes / actual ID count is `4/2 = 2` and `10/5 = 2`; matches descriptor word 2. |
| UNKNOWN | Four-byte field `+0x24` | `0x10FA..0x10FE`: all zero | `0x1236..0x123A`: all zero | Four bytes occur before the IDs; pointer, reserved or other meaning is not established. |
| OBSERVATION | ID data `+0x28` | `0x10FE..0x1102`: `25 01 26 01` | `0x123A..0x1244`: `2A 01 2B 01 2E 01 2F 01 30 01` | Two-byte LE decoding gives IDs `125,126` and `12A,12B,12E,12F,130`, respectively; all resolve to the corresponding parent candidate. |

OBSERVATION: ranges in the field table are end-exclusive. The two examples'
different nonzero counts independently constrain the element width. One-byte
elements would consume only 2/5 bytes, leaving 2/5 bytes unexplained and
splitting the observed IDs; four-byte elements require 8/20 bytes and overrun
the record spans. Two-byte elements consume exactly 4/10 bytes and each
decoded value matches a distinct directory ID with the expected parent.

OBSERVATION: there is no separate proven byte-length word here. The second
count-like word is 2/5, not the ID byte length 4/10, the full tail length
14/20, or record span 44/50. The proven extent relationship in these records
is `30 + 10 + 2*n`, namely `30 + 10 + 4 = 44` and
`30 + 10 + 10 = 50`. INFERENCE: either `0x1E` byte can serve as the fixed-core
boundary/length for a bounded experimental walk. UNKNOWN: which repeated
byte is the actual format length field, and whether either duplicate
count-like word has a distinct length/capacity role. No label promotes those
undistinguished alternatives into a decoded semantic field.

### Descriptor rule and exact byte selections

OBSERVATION: A descriptor 20 occupies Contents `[0xD6,0xE6)`:
`1E 1E 19 00 02 00 04 00 06 00 1A 00 1C 00 00 00`.
The three initial u16 words are numbered 0, 1 and 2 here; following them
are the previously observed terminated selector words.

| Basis | Descriptor field | Measured selection in an index-20 record at R |
| --- | --- | --- |
| OBSERVATION | Word 0 at `0xD6`: `0x1E1E` | Its low byte and high byte are both 30, matching both bytes at `R+2` and selecting tail-header start `T = R+0x1E`. For `0x10A`, T is `0x10F4`; for `0x127`, `0x1230`. |
| UNKNOWN | Word 1 at `0xD8`: `0x0019` | Constant in this sole descriptor; no bit meaning, count role or variable-tail flag is established. The rule below does not use it to guess a layout. |
| OBSERVATION | Word 2 at `0xDA`: `0x0002` | Matches width at `T+4` and the independently measured two-byte stride. Element j occupies `[T+10+2*j,T+12+2*j)`. |
| OBSERVATION | Selector at `0xDC`: `0x0004` | Selects fixed u16 `[R+4,R+6)`; only record `0x107` has a nonzero ID here, `0x121`. |
| OBSERVATION | Selector at `0xDE`: `0x0006` | Selects fixed u16 `[R+6,R+8)`; all eight values are zero. Its reference role is not positively demonstrated. |
| OBSERVATION | Selector at `0xE0`: `0x001A` | Selects fixed u16 `[R+0x1A,R+0x1C)`; all eight values are zero. Its reference role is not positively demonstrated. |
| OBSERVATION | Selector at `0xE2`: `0x001C` | Selects fixed u16 `[R+0x1C,R+0x1E)`; six nonzero values resolve to matching direct directory children, including `0x10B` at `0x10F2` and `0x128` at `0x122E`. |
| OBSERVATION | Terminator at `0xE4`: zero | Ends the descriptor's selector list before descriptor 21 at `0xE6`; not an extra zero-offset selector. |

INFERENCE: the reproducible, sample-bounded rule is to match the leading
index to descriptor 20; take one of word 0's equal bytes as boundary B=30;
use word 2 as stride W=2; require the record's two boundary bytes, both
count-like words and width to match the observed relationships; skip the
ten-byte tail header; read n two-byte IDs; and require `R+B+10+n*W` to equal
the next directory offset. Separately, apply the four selector offsets from
the record start, not from the end of its index or from T. All seven nonzero
fixed selections resolve to direct directory children in this file.

UNKNOWN: these equal-byte checks do not prove that a writer or reader uses
the descriptor rather than instance bytes as its authoritative boundary or
width. Both produce identical selections here. The arithmetic establishes
an observed rule for this class and sample, not the universal meaning of
descriptor words or a way to decode `0x0019` by analogy with another format.
No mutation of these sample bytes would provide independent evidence of the
original format's intended behavior.

### Records tested and disagreements

OBSERVATION: scanning all 54 directory-addressed leading indices finds
exactly eight index-20 records. Every one is tested below. The two count-like
words agree in all eight; the width and descriptor stride are always 2; the
four-byte tail-header field is always zero. The table gives absolute count
and data positions. ID lists are hexadecimal, and an empty list is explicit.

| Basis | ID | Start / span | First count at / n | Second count at / value | ID data start / bytes | Tail IDs | Nonzero fixed-selector IDs |
| --- | --- | --- | --- | --- | --- | --- | --- |
| OBSERVATION | `12C` | `0902 / 60` | `0920 / 10` | `0922 / 10` | `092A / 20` | `133,134,135,136,137,138,139,13A,13B,13C` | empty |
| OBSERVATION | `117` | `1036 / 40` | `1054 / 0` | `1056 / 0` | `105E / 0` | empty | empty |
| OBSERVATION | `107` | `105E / 40` | `107C / 0` | `107E / 0` | `1086 / 0` | empty | `121,108` |
| OBSERVATION | `10A` | `10D6 / 44` | `10F4 / 2` | `10F6 / 2` | `10FE / 4` | `125,126` | `10B` |
| OBSERVATION | `127` | `1212 / 50` | `1230 / 5` | `1232 / 5` | `123A / 10` | `12A,12B,12E,12F,130` | `128` |
| OBSERVATION | `10D` | `1670 / 40` | `168E / 0` | `1690 / 0` | `1698 / 0` | empty | `10E` |
| OBSERVATION | `110` | `16AE / 40` | `16CC / 0` | `16CE / 0` | `16D6 / 0` | empty | `111` |
| OBSERVATION | `113` | `16EC / 40` | `170A / 0` | `170C / 0` | `1714 / 0` | empty | `114` |

OBSERVATION: the framing formula agrees in **8/8** reachable records, with
zero discrepancies: **3/3 nonempty tails** and **5/5 empty tails**. Empty
tails still have the ten-byte header and end at R+40. Across the eight
records there are 17 tail IDs and seven nonzero fixed-selector IDs; the
combined 24 distinct direct-child references equal the directory's 24
matching parent-candidate edges, without omissions or duplicates per owner.

OBSERVATION: for `0x10A`, directory entry `0x518` contains ID `0x125`, parent
`0x10A`; entry `0x522` contains ID `0x126`, parent `0x10A`. Its additional
direct child `0x10B` is in entry `0x50E` and is selected by fixed offset
`+0x1C`, not the tail. For `0x127`, tail IDs resolve in directory entries
`0x540,0x54A,0x572,0x57C,0x586`, respectively, all with parent `0x127`.
The additional direct child `0x128` is entry `0x536`, again selected at
fixed offset `+0x1C`. Only these directory fields are used; the children’s
body layouts are outside this step.

OBSERVATION: the interpretation “the tail lists every direct directory
child” disagrees in **6/8** records: `107,10A,127,10D,110,113` each have
additional fixed-selector children. It agrees only for `12C` and the
childless `117`. This is a semantic overreach rejected by the check, not
a discrepancy in the two-byte tail framing. An empty tail cannot be used
as evidence of no children or of a blank visible page.

### What stays unknown and the checkpoint limit

UNKNOWN: count versus allocation/element-length roles of the two equal
u16 words; which equal boundary byte is authoritative; a separate total
record-byte-length field; descriptor `0x0019`; the four zero tail-header
bytes; semantics of the two always-zero fixed selectors; ordering/roles
of the references; page/master/scratch meaning; malformed-input behavior;
and applicability beyond Sample98 index-20. A unique descriptor algorithm
cannot be proved from duplicates that never differ in these records.

OBSERVATION: this is the first AP2 checkpoint. A fresh, bounded byte check
passes all eight framing equations and all directory-reference comparisons;
the documented field positions and ID lists are checked against that output.
No importer or protected-reader change, other class decoding, B-table work,
Quill repair, full CI, environment script/setting change, environment save
or publication was performed. Only this appended section and the narrowed
ranked-unknowns section change; the local evidence remains ignored.

## Second variant-A sample

OBSERVATION: AP3 starts from `5dac32658bfd7693a3c530a24734afdacea5f412`.
The attached owner-supplied mspub.pub is exactly **19968 bytes**, SHA-256
`0EFF0498172805BF1E52C74A6334ECC4CDC84FB2E531C0EA25734CFAC7923C7D`,
matching the requested identity before inspection. The owner describes it
as an independently made older publication taken from a commercial software
image. UNKNOWN: its manufacture and exact last-saving product release were
not independently authenticated from the bytes. The earlier AP unavailable-file
statement is historical; this attachment now supplies mspub.pub, not zmspub.pub.

OBSERVATION: the sample was treated as data only. The original remains outside
the checkout, unmodified and uncommitted; only root Contents was extracted to
the ignored research directory. No sample text, pictures or raw byte sequences
are reproduced here. Evidence below consists of structural measurements and
numeric field values. No external format description, reference image or
scoring JSON was used. Other payload-class bodies, B's table and Quill were
not decoded, and no importer code was changed.

### Container and variant checks

OBSERVATION: the compound-file container opens as version 3 and lists exactly
these six streams. Contents was read to its directory-reported length of
4152 bytes. Other stream sizes below are directory metadata, not claims of
successful application-level parsing.

| Basis | Stream | Bytes |
| --- | --- | ---: |
| OBSERVATION | `/\x01CompObj` | 94 |
| OBSERVATION | `/\x03Internal` | 12 |
| OBSERVATION | `/\x05SummaryInformation` | 8232 |
| OBSERVATION | `/Contents` | 4152 |
| OBSERVATION | `/Quill/QuillSub/\x01CompObj` | 86 |
| OBSERVATION | `/Quill/QuillSub/CONTENTS` | 1024 |

OBSERVATION: unlike Sample98's ten streams, this file has no Envelope,
DocumentSummaryInformation or either Escher stream. These are absent entries,
not zero-length streams. SummaryInformation and Quill CONTENTS are smaller.
UNKNOWN: absent drawing streams do not establish absence of visible objects.

OBSERVATION: all positions in the following tables are hexadecimal byte
offsets relative to root Contents; sizes/counts are decimal unless marked
with `0x`. Numeric reads are little-endian. The structural checks are the
same bounded checks used for Sample98, with sample-specific directory counts
and offsets rather than Sample98's constants.

| Basis | Field / position | mspub.pub result | Comparison/check |
| --- | --- | --- | --- |
| OBSERVATION | Contents family selector | Matches Sample98 | Equality check on the existing four-byte discriminator; not an exact saving-version test. |
| OBSERVATION | u32 `0x08` | 4152 | Equals actual Contents bytes, versus Sample98's 5930. |
| OBSERVATION | u32 `0x12` | `0x37E` | Same control start as Sample98; control length 46 reaches `0x3AC` exactly. |
| OBSERVATION | u32 `0x16` | `0x3AC` | Same directory start as Sample98; both control-relative pointers at `+0x0A` and `+0x0E` equal it. |
| OBSERVATION | u32 `0x1A` | `0x22` | Same layout-table start; table header values are 89 and 6. |
| OBSERVATION | Descriptor walk `0x26..0x37E` | 88 entries, exact end | Three u16 words plus zero-terminated selectors per entry; no leftover bytes. Entire `[0x22,0x37E)` table equals Sample98's. |
| OBSERVATION | Descriptor 20 at `0xD6` | Same three words and four selectors | Boundary bytes 30/30, middle word 25, stride 2; selectors 4, 6, 26, 28. |
| OBSERVATION | Directory count at `0x3AC` | 39 | Versus 54 in Sample98; exactly 39 ten-byte entries after the four-byte header. |
| OBSERVATION | Directory end / first payload | `0x536 / 0x540` | Ten zero gap bytes, versus Sample98's twenty. |
| OBSERVATION | Directory IDs / offsets / leading indices | 39 unique IDs; increasing bounded offsets; indices in 1..88 | First payload starts at `0x540`, last starts at `0x1010`; final span ends at actual stream length `0x1038`. Only leading indices of other classes are inspected. |
| OBSERVATION | u16 `0x0E`, first directory ID and directory-header maximum at `0x3AE` | All equal `0x127` | All differ from Sample98's root-like ID `0x100`; its maximum is instead `0x141`. Here the maximum and first ID happen to coincide. |
| OBSERVATION | Unresolved parent candidate | Only ID `0x127` with parent `0xFE` | Other 38 parents resolve; Sample98 instead has only ID `0x100` with unresolved parent `0xFF`. A fixed root ID or fixed unresolved-parent sentinel is not supported across these files. |

INFERENCE: mspub.pub is a second **structurally matching variant-A sample**
under the tested discriminator, header, table, control, directory and index-20
criteria. This does not prove every record class or the publication's complete
format semantics. Differences in stream inventory, root-like IDs, directory
count, gaps and payload positions do not defeat those structural checks.

### Index-20 fields and every tested record

OBSERVATION: all 39 directory-addressed leading indices were checked; exactly
six select descriptor 20. All six have boundary bytes at `R+0x02` and
`R+0x03` equal to 30. Both count-like u16 words, at `R+0x1E` and `R+0x20`,
are equal in each record. The byte at `R+0x22` is 2 and the next byte is zero;
reading those two bytes as a u16 also gives 2. The four-byte field at `R+0x24`
is zero in each record. UNKNOWN: byte versus u16 semantics for the width
cannot be distinguished when its high byte is always zero.

| Basis | ID | R / observed span | Boundary-byte positions | First count at / n | Second count at / value | Width byte at / value | IDs start / actual ID bytes | Formula span / result |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| OBSERVATION | `0x10E` | `0x7B6 / 62` | `0x7B8,0x7B9` | `0x7D4 / 11` | `0x7D6 / 11` | `0x7D8 / 2` | `0x7DE / 22` | 62 / agrees |
| OBSERVATION | `0x11F` | `0xF0E / 40` | `0xF10,0xF11` | `0xF2C / 0` | `0xF2E / 0` | `0xF30 / 2` | `0xF36 / 0` | 40 / agrees |
| OBSERVATION | `0x11E` | `0xF36 / 40` | `0xF38,0xF39` | `0xF54 / 0` | `0xF56 / 0` | `0xF58 / 2` | `0xF5E / 0` | 40 / agrees |
| OBSERVATION | `0x11D` | `0xF98 / 40` | `0xF9A,0xF9B` | `0xFB6 / 0` | `0xFB8 / 0` | `0xFBA / 2` | `0xFC0 / 0` | 40 / agrees |
| OBSERVATION | `0x11C` | `0xFC0 / 40` | `0xFC2,0xFC3` | `0xFDE / 0` | `0xFE0 / 0` | `0xFE2 / 2` | `0xFE8 / 0` | 40 / agrees |
| OBSERVATION | `0x11A` | `0x1010 / 40` | `0x1012,0x1013` | `0x102E / 0` | `0x1030 / 0` | `0x1032 / 2` | `0x1038 / 0` | 40 / agrees |

OBSERVATION: span is measured to the next directory offset, or Contents end
for the last entry. For `0x10E`, `30+10+2*11 = 62`; its 22 ID bytes occupy
`[0x7DE,0x7F4)`, exactly reaching the next directory offset. Two-byte reads
give eleven distinct IDs descending from `0x10D` through `0x103`; every one
resolves to a directory entry whose parent candidate is `0x10E`. The quotient
22 bytes / 11 IDs independently repeats the observed element width of 2.
Each of the other five records has zero ID bytes and satisfies
`30+10+2*0 = 40`, including the final record ending exactly at stream end.

OBSERVATION: descriptor 20's fixed selectors also give bounded u16 reads.
Only one selected value is nonzero: record `0x11E` at `0xF36` has ID `0x10F`
at relative `+0x04`, absolute `0xF3A`. Its matching directory entry has parent
`0x11E`. Thus eleven tail references plus one fixed reference cover all twelve
direct directory children of the six index-20 records. No child's body is
decoded by this check. UNKNOWN: ordering and semantic roles remain unverified.

### Disagreements and the count question

OBSERVATION: **6/6 records agree**, comprising one nonempty tail and five
empty tails. The exact list of disagreements with the requested boundary,
count-equality, width, ID-resolution and `30+10+2*count` checks is **empty**.
No unequal count-like pair exists among the six reachable index-20 records,
and no record span differs from that formula. Sample98's prior 8/8 and this
sample's fresh 6/6 give fourteen agreeing observations across two supplied
files; that is still limited sample coverage, not a universal rule.

UNKNOWN: count versus capacity versus element-length is **not resolved**.
The words are either both 11 or both zero; any of those roles could agree
when used and allocated lengths coincide. Neither word gives the nonempty
list's byte length 22 or record length 62. The duplicate boundary bytes also
remain equal. No separate total-record byte-length field, authoritative copy,
or behavior for unequal counts is established. The width high byte remaining
zero does not distinguish a one-byte width from a wider field either.

INFERENCE: the next smallest discriminating step remains a controlled,
independently produced variant-A index-20 example with unequal count-like
words, ideally after a documented deletion leaving spare allocation. That
is a proposed experiment, not an assertion that deletion creates such bytes.
It would test used count versus capacity without changing another class or
guessing from the two currently equal copies.

UNKNOWN: saving release, original visible content, page/master/scratch roles,
general descriptor semantics, reference relocation and malformed-input
handling remain outside this checkpoint. The owner pair's identical Contents
claim cannot be checked without zmspub.pub. No text or picture was inspected
to fill those gaps.

OBSERVATION: the fresh bounded check passes the required original size/hash,
4152-byte Contents length, all four requested header values, exact table
walk, control boundaries/pointers, directory bounds/IDs/leading indices,
all six tail equations and all twelve reference resolutions. The original
sample remains unchanged. No full CI, environment save/publication,
environment script/setting change, importer change or pull request was made.

## Record extent, padding and directory flags

OBSERVATION: AP4 starts from `3b7b75de1e3e8335f469298fdd1865283dbfc3b5`.
The pinned restore check verifies all 317 inputs, restoring none and reporting
zero failures. Both requested originals are present and match their previously
recorded sizes and SHA-256 values: Sample98.pub is 61440 bytes and mspub.pub
is 19968 bytes. Fresh root Contents extractions have lengths 5930 and 4152.
The attachment therefore remains available; no single-sample fallback is used.
The same evidence sources are already listed, so SOURCES.md is unchanged.

OBSERVATION: this step examines every directory entry and only structural
prefixes, descriptors and tail headers. It treats every tail body as opaque;
no geometry, text or formatting is decoded. Only derived structural facts are
published here; the private original and extracted bytes remain uncommitted.

### Directory fields: observation versus interpretation

OBSERVATION: in both files the four-byte directory header is followed by
ten-byte entries. D below is an entry's absolute Contents position, R its
addressed payload start, and E the next directory offset or stream length
for the final entry. Positions and IDs are hexadecimal; counts and sizes
are decimal. All numeric reads are little-endian.

| Basis | Entry field | Evidence across all entries | Interpretation limit |
| --- | --- | --- | --- |
| OBSERVATION | u16 at `D+0` | 54/54 and 39/39 distinct values; referenced values resolve to these IDs. First IDs are `0x100` and `0x127`, respectively. | An explicit ID, not the directory ordinal or a demonstrated byte length; visible-object role is not implied. |
| INFERENCE | u16 at `D+2` | 53/54 and 38/39 values resolve to another directory ID; only the root-like entries use unresolved values `0xFF` and `0xFE`. Previous fixed/tail links corroborate the relationship. | Owner/parent candidate; exact hierarchy semantics remain unknown. |
| OBSERVATION | u32 at `D+4` | All 93 values are strictly increasing, after the directory and inside Contents; they address valid leading descriptor indices. At the first entry `0x3B0`, field `0x3B4` selects `0x5E0` / `0x540`. | Absolute start offset, not a size. Full logical extent needs an independent structural check. |
| UNKNOWN | u16 at `D+8` | Values `0x1640,0x16C0,0x16E0,0x1740`; none equals its record span. Each is larger than every observed span (maximum 294). | Attribute/flags candidate, not an unscaled record-byte-length field; no bit meaning established. |

OBSERVATION: the ten bytes are exhausted by these fields; there is no additional
directory length word in this framing. A packed length interpretation of the
attribute word is not proven or ruled out merely by rejecting its literal
numeric value as a length. IDs and parent candidates must not be substituted
for byte lengths either.

### Bounded extent rules

OBSERVATION: for each record define S=E-R, L=the byte at R+2, H=the byte at
R+3, and I=the leading u16 descriptor index. Define C as the selected
descriptor's second u16 word, numbered from zero. C is a **numeric class
label for this audit**, not a newly decoded semantic type. For all 93 records,
H equals the high byte of descriptor word 0. L does not supply a universal
total length. No payload interpretation is needed for these comparisons.

| Basis | Numeric C / rule label | Independent predicted span | Sample98 agreeing / tested | mspub agreeing / tested |
| --- | --- | --- | --- | --- |
| OBSERVATION | 0 / F | H | 8/8 | 7/7 |
| OBSERVATION | 1,9,25 / A | `H + 10 + n*w`, with u16 n at `R+H`, repeated u16 at `R+H+2`, u16 w at `R+H+4` | 27/27 | 17/17 |
| OBSERVATION | 4 / O | `H + 8 + q`, with u32 q at `R+H` | 9/9 | 2/2 |
| UNKNOWN | 2,3,10,195 / U | No independent end rule established in this checkpoint | 10 unresolved | 13 unresolved |

OBSERVATION: n equals its repeated word in all 44 A records; w is positive.
The A header's final four bytes are not required to be zero. The O header's
other four bytes remain opaque; q is used only as an observed byte-count
relationship, not to decode the following content. All reads and predicted
ends are bounded before comparison with E. These rules are selected by C,
not by the directory attribute word.

INFERENCE: H marks a fixed-prefix boundary in the observed F/A/O classes:
F ends there, A adds a ten-byte header and n elements, O adds an eight-byte
header and q bytes. These three predictions exactly reach E in **70/70**
applicable records. The remaining **23** are unresolved, not failed examples
of those class-restricted rules and not silently treated as padding. This
does not establish encoder intent or malformed-input behavior.

OBSERVATION: using descriptor word 2 universally as A's element width fails
at index 67. Sample98 ID `0x102`, R=`0xCF2`, has H=10, n=7 and w=2 at
`0xD00`, but descriptor word 2 is 8. Using the instance width predicts 34,
the observed span; substituting 8 predicts 76 and crosses the next offset
`0xD14` by 42 bytes. In mspub ID `0x119`, R=`0xC36`, w=2 at `0xC44`
also differs from descriptor width 8, but n=0 makes both extent predictions
20. These are the only two descriptor/instance-width disagreements among
the 44 A records. UNKNOWN: the reason; index-20's matching stride alone
must not be generalized to every descriptor.

### Padding, gaps and literal-length counterexamples

OBSERVATION: the directory-to-first-payload zero-filled gap is twenty bytes
in Sample98 (`0x5CC..0x5E0`) and ten in mspub (`0x536..0x540`). Neither
gap belongs to a directory-addressed record. INFERENCE: padding/reserved
space is a plausible role; its allocation or alignment rule is UNKNOWN.

OBSERVATION: all 93 starts are ordered and bounded, producing 54/39 disjoint
directory-derived spans. Four starts in each sample are odd, refuting a
universal two- or four-byte record alignment requirement. The three
class-restricted predictions leave no unaccounted bytes between predicted
end and next start for their 70 records. For the 23 U records, the presence,
absence or size of internal trailing padding remains UNKNOWN: E is a bound,
not an independently recovered logical end.

OBSERVATION: Sample98's final ID `0x114`, R=`0x1714`, is O: H=4, q=10,
so predicted span 22 ends at `0x172A`, exactly the 5930-byte stream end.
mspub's final ID `0x11A`, R=`0x1010`, is A: H=30, n=0, w=2, so predicted
span 40 ends at `0x1038`, exactly 4152 bytes. No unexplained trailing stream
gap follows either final structurally accounted record. Compound-file sector
slack outside the stream is not part of this extent calculation.

OBSERVATION: the universal candidate S=H fails at **46/54** Sample98 and
**32/39** mspub records; only the 15 F records match. Candidate S=L fails
at **53/54** and **35/39**, respectively. Its only matches are Sample98
`0x120` and mspub `0x10A,0x109,0x107,0x115`. Treating the two bytes at
R+2 as one u16 total length matches **0/93**; treating the leading u32 at
R as a total length also matches **0/93**. A schema index is not a current-family
length prefix. No universal own-length-field rule is proven.

OBSERVATION: the complete per-entry tables below list every counterexample,
not just examples. Totals lists which literal byte candidates equal S:
`H` means L fails, `L+H` means both match, and `none` means both fail. All
rows fail the u16-at-R+2 and u32-at-R total-length candidates. U rows list
every record whose true end remains unresolved; F/A/O rows match their
class-restricted prediction without exception. Each row is OBSERVATION;
U's unresolved logical length remains UNKNOWN. L/H, indices and C are
decimal; ID, R and attribute values are hexadecimal.

#### Sample98: all 54 entries

OBSERVATION: complete measured census for this sample.

| ID | R | S | Index | C | Attribute | L/H | Rule | Totals |
| --- | --- | ---: | ---: | ---: | --- | --- | --- | --- |
| 100 | 05E0 | 202 | 21 | 25 | 16C0 | 210/178 | A | none |
| 11F | 06AA | 294 | 41 | 195 | 16C0 | 56/56 | U | none |
| 120 | 07D0 | 112 | 54 | 0 | 16C0 | 112/112 | F | L+H |
| 123 | 0840 | 194 | 40 | 3 | 16C0 | 28/28 | U | none |
| 12C | 0902 | 60 | 20 | 25 | 1640 | 30/30 | A | none |
| 133 | 093E | 150 | 2 | 1 | 1640 | 178/140 | A | none |
| 134 | 09D4 | 150 | 3 | 1 | 1640 | 180/140 | A | none |
| 135 | 0A6A | 67 | 4 | 0 | 1640 | 57/67 | F | H |
| 136 | 0AAD | 70 | 5 | 0 | 1640 | 60/70 | F | H |
| 137 | 0AF3 | 61 | 6 | 0 | 1640 | 55/61 | F | H |
| 138 | 0B30 | 49 | 7 | 0 | 1640 | 39/49 | F | H |
| 139 | 0B61 | 128 | 8 | 9 | 1640 | 126/118 | A | none |
| 13A | 0BE1 | 61 | 9 | 4 | 1640 | 43/53 | O | none |
| 13B | 0C1E | 126 | 10 | 1 | 1640 | 115/116 | A | none |
| 13C | 0C9C | 86 | 11 | 4 | 1640 | 68/78 | O | none |
| 102 | 0CF2 | 34 | 67 | 9 | 16C0 | 10/10 | A | none |
| 103 | 0D14 | 30 | 49 | 2 | 16C0 | 8/8 | U | none |
| 11E | 0D32 | 26 | 50 | 3 | 16C0 | 4/4 | U | none |
| 119 | 0D4C | 26 | 30 | 3 | 16C0 | 8/4 | U | none |
| 105 | 0D66 | 26 | 31 | 3 | 16E0 | 4/4 | U | none |
| 101 | 0D80 | 30 | 49 | 2 | 16C0 | 8/8 | U | none |
| 104 | 0D9E | 26 | 50 | 3 | 16C0 | 4/4 | U | none |
| 131 | 0DB8 | 14 | 55 | 1 | 16E0 | 4/4 | A | none |
| 132 | 0DC6 | 14 | 56 | 1 | 16E0 | 4/4 | A | none |
| 124 | 0DD4 | 56 | 57 | 0 | 16E0 | 88/56 | F | H |
| 11D | 0E0C | 192 | 71 | 1 | 16E0 | 150/150 | A | none |
| 11A | 0ECC | 8 | 69 | 0 | 16C0 | 56/8 | F | H |
| 11B | 0ED4 | 70 | 76 | 10 | 16C0 | 18/18 | U | none |
| 11C | 0F1A | 244 | 82 | 2 | 16C0 | 18/18 | U | none |
| 106 | 100E | 40 | 51 | 4 | 1640 | 4/4 | O | none |
| 117 | 1036 | 40 | 20 | 25 | 16C0 | 30/30 | A | none |
| 107 | 105E | 40 | 20 | 25 | 16C0 | 30/30 | A | none |
| 121 | 1086 | 58 | 44 | 1 | 16C0 | 24/24 | A | none |
| 108 | 10C0 | 22 | 51 | 4 | 1640 | 4/4 | O | none |
| 10A | 10D6 | 44 | 20 | 25 | 16C0 | 30/30 | A | none |
| 10B | 1102 | 16 | 51 | 4 | 1640 | 4/4 | O | none |
| 125 | 1112 | 128 | 8 | 9 | 1740 | 126/118 | A | none |
| 126 | 1192 | 128 | 8 | 9 | 1740 | 126/118 | A | none |
| 127 | 1212 | 50 | 20 | 25 | 16C0 | 30/30 | A | none |
| 128 | 1244 | 22 | 51 | 4 | 1640 | 4/4 | O | none |
| 12A | 125A | 128 | 8 | 9 | 1740 | 126/118 | A | none |
| 12B | 12DA | 166 | 10 | 1 | 1640 | 115/116 | A | none |
| 141 | 1380 | 290 | 78 | 1 | 1640 | 10/10 | A | none |
| 140 | 14A2 | 86 | 79 | 1 | 1640 | 10/10 | A | none |
| 13F | 14F8 | 50 | 80 | 1 | 1640 | 10/10 | A | none |
| 12E | 152A | 70 | 5 | 0 | 1640 | 60/70 | F | H |
| 12F | 1570 | 128 | 8 | 9 | 1740 | 126/118 | A | none |
| 130 | 15F0 | 128 | 8 | 9 | 1740 | 126/118 | A | none |
| 10D | 1670 | 40 | 20 | 25 | 16C0 | 30/30 | A | none |
| 10E | 1698 | 22 | 51 | 4 | 1640 | 4/4 | O | none |
| 110 | 16AE | 40 | 20 | 25 | 16C0 | 30/30 | A | none |
| 111 | 16D6 | 22 | 51 | 4 | 1640 | 4/4 | O | none |
| 113 | 16EC | 40 | 20 | 25 | 16C0 | 30/30 | A | none |
| 114 | 1714 | 22 | 51 | 4 | 1640 | 4/4 | O | none |

#### mspub: all 39 entries

OBSERVATION: complete measured census for this sample.

| ID | R | S | Index | C | Attribute | L/H | Rule | Totals |
| --- | --- | ---: | ---: | ---: | --- | --- | --- | --- |
| 127 | 0540 | 200 | 21 | 25 | 16C0 | 210/178 | A | none |
| 111 | 0608 | 236 | 41 | 195 | 16C0 | 56/56 | U | none |
| 102 | 06F4 | 194 | 40 | 3 | 16C0 | 28/28 | U | none |
| 10E | 07B6 | 62 | 20 | 25 | 1640 | 30/30 | A | none |
| 10D | 07F4 | 114 | 1 | 1 | 1640 | 104/104 | A | none |
| 10C | 0866 | 150 | 2 | 1 | 1640 | 188/140 | A | none |
| 10B | 08FC | 150 | 3 | 1 | 1640 | 190/140 | A | none |
| 10A | 0992 | 67 | 4 | 0 | 1640 | 67/67 | F | L+H |
| 109 | 09D5 | 70 | 5 | 0 | 1640 | 70/70 | F | L+H |
| 108 | 0A1B | 61 | 6 | 0 | 1640 | 65/61 | F | H |
| 107 | 0A58 | 49 | 7 | 0 | 1640 | 49/49 | F | L+H |
| 106 | 0A89 | 128 | 8 | 9 | 1640 | 136/118 | A | none |
| 105 | 0B09 | 61 | 9 | 4 | 1640 | 53/53 | O | none |
| 104 | 0B46 | 126 | 10 | 1 | 1640 | 125/116 | A | none |
| 103 | 0BC4 | 86 | 11 | 4 | 1640 | 78/78 | O | none |
| 115 | 0C1A | 28 | 64 | 0 | 16C0 | 28/28 | F | L+H |
| 119 | 0C36 | 20 | 67 | 9 | 16C0 | 10/10 | A | none |
| 101 | 0C4A | 50 | 35 | 3 | 16C0 | 4/4 | U | none |
| 122 | 0C7C | 30 | 49 | 2 | 16E0 | 8/8 | U | none |
| 121 | 0C9A | 26 | 50 | 3 | 16E0 | 4/4 | U | none |
| 110 | 0CB4 | 114 | 30 | 3 | 16C0 | 8/4 | U | none |
| 120 | 0D26 | 26 | 31 | 3 | 16E0 | 4/4 | U | none |
| 126 | 0D40 | 30 | 49 | 2 | 16E0 | 8/8 | U | none |
| 125 | 0D5E | 26 | 50 | 3 | 16E0 | 4/4 | U | none |
| 124 | 0D78 | 30 | 49 | 2 | 16E0 | 8/8 | U | none |
| 123 | 0D96 | 26 | 50 | 3 | 16E0 | 4/4 | U | none |
| 100 | 0DB0 | 14 | 55 | 1 | 16E0 | 4/4 | A | none |
| 116 | 0DBE | 56 | 57 | 0 | 16E0 | 88/56 | F | H |
| 113 | 0DF6 | 192 | 71 | 1 | 16E0 | 150/150 | A | none |
| 118 | 0EB6 | 8 | 69 | 0 | 16C0 | 56/8 | F | H |
| 117 | 0EBE | 40 | 76 | 10 | 16E0 | 18/18 | U | none |
| 114 | 0EE6 | 40 | 82 | 2 | 16C0 | 18/18 | U | none |
| 11F | 0F0E | 40 | 20 | 25 | 16C0 | 30/30 | A | none |
| 11E | 0F36 | 40 | 20 | 25 | 16C0 | 30/30 | A | none |
| 10F | 0F5E | 58 | 44 | 1 | 16C0 | 24/24 | A | none |
| 11D | 0F98 | 40 | 20 | 25 | 1640 | 30/30 | A | none |
| 11C | 0FC0 | 40 | 20 | 25 | 16C0 | 30/30 | A | none |
| 11B | 0FE8 | 40 | 65 | 25 | 1640 | 30/30 | A | none |
| 11A | 1010 | 40 | 20 | 25 | 1640 | 30/30 | A | none |

### Attribute-word correlations

OBSERVATION: each row below groups the complete directory census by the
literal attribute word. Span ranges are inclusive extrema, not all sizes
inside the range. Indices and numeric C labels are decimal. These are
correlations, not assigned flag semantics.

| Basis | Sample | Attribute | Entries | Span range | Descriptor indices | Numeric classes C | Meaning |
| --- | --- | --- | ---: | --- | --- | --- | --- |
| OBSERVATION | Sample98 | `0x1640` | 23 | 16–290 | 2,3,4,5,6,7,8,9,10,11,20,51,78,79,80 | 0,1,4,9,25 | UNKNOWN |
| OBSERVATION | Sample98 | `0x16C0` | 21 | 8–294 | 20,21,30,40,41,44,49,50,54,67,69,76,82 | 0,1,2,3,9,10,25,195 | UNKNOWN |
| OBSERVATION | Sample98 | `0x16E0` | 5 | 14–192 | 31,55,56,57,71 | 0,1,3 | UNKNOWN |
| OBSERVATION | Sample98 | `0x1740` | 5 | 128–128 | 8 | 9 | UNKNOWN |
| OBSERVATION | mspub | `0x1640` | 15 | 40–150 | 1,2,3,4,5,6,7,8,9,10,11,20,65 | 0,1,4,9,25 | UNKNOWN |
| OBSERVATION | mspub | `0x16C0` | 13 | 8–236 | 20,21,30,35,40,41,44,64,67,69,82 | 0,1,2,3,9,25,195 | UNKNOWN |
| OBSERVATION | mspub | `0x16E0` | 11 | 14–192 | 31,49,50,55,57,71,76 | 0,1,2,3,10 | UNKNOWN |

OBSERVATION: the aggregate counts are 38 at `0x1640`, 34 at `0x16C0`,
16 at `0x16E0`, and five at `0x1740`. The first three values have multiple
sizes and indices, and every one of all four values occurs on a span longer
than its H boundary somewhere. None identifies a literal total length.
The first three values occur with both F and non-F rules; `0x1740` occurs
only with A here, without establishing a general variable-extent flag meaning.

OBSERVATION: `0x1740` occurs only in Sample98, all five with index 8,
C=9 and span 128. Yet Sample98 ID `0x139` and mspub ID `0x106` have that
same index, class and span with attribute `0x1640`. The attribute therefore
cannot be derived uniquely from that size/index combination. Similarly,
mspub's index-20 IDs `0x11F` and `0x11D` both have span 40 and C=25 but
attributes `0x16C0` and `0x1640`. Index 49/span 30 and index 50/span 26
occur under both `0x16C0` and `0x16E0` across these files.

UNKNOWN: semantic meanings of **all four attribute values**, and each bit
that differs between them. For `0x1740`, exclusivity to one observed
size/index does not establish a role because the same combination also
occurs under another value. No names such as visible, deleted, allocated,
compressed or master are assigned to any bit. The only explained negative
result is that the whole word is not an unscaled record byte length.

### Checkpoint and remaining limits

OBSERVATION: the final bounded check covers all 93 entries, both original
size/hash checks, directory bounds/uniqueness/parent candidates, H-to-descriptor
agreement, all 70 class-selected predictions, the two known pre-payload gaps,
both independently accounted final ends, and the complete flag census.
Two initial diagnostic assertions had incorrect expected census/final-class
assumptions; those assertions were corrected, without changing any sample or
product code. The final checks pass; no failed extent comparison was suppressed.

UNKNOWN: independent logical ends or padding in the 23 U records; a general
descriptor algorithm; flag-bit semantics; count/capacity distinctions; exact
product versions and broader input validity. Passing 70 arithmetic checks
does not prove complete publication decoding or that all zero bytes are padding.

INFERENCE: the next smallest byte-only step is the five class-2/index-49
records named in the complete tables: explain their common 22-byte tail
after H=8 using structural lengths/counts only. Keep all body meanings and
the other unresolved indices outside that follow-up until its rule is checked.

OBSERVATION: this is the first AP4 checkpoint. Only this appended section and
the narrowed ranked-unknowns section change; no new SOURCES entry is needed.
No full CI, importer edit, protected-reader edit, B work, Quill repair,
geometry/text/formatting decoding, environment script/setting change,
environment save/publication or pull request was performed. Raw evidence and
the private attachment remain uncommitted, and no extra remote was added.

## Class-2 index-49 tails

OBSERVATION: AP5 starts from `60890173b273a995bb555a8d85dbc54fce94e00f`.
The restore check verifies all 317 pinned inputs with zero failures and no
downloads. Both originals are present: Sample98.pub matches its recorded
61440-byte size/hash, and the attached mspub.pub matches 19968 bytes and
SHA-256 `0EFF0498172805BF1E52C74A6334ECC4CDC84FB2E531C0EA25734CFAC7923C7D`.
Fresh Contents extractions remain in the ignored research directory. No
single-sample fallback or new evidence source is used; SOURCES.md is unchanged.

OBSERVATION: the check selects every directory-addressed leading index 49,
then examines only the prefix, selected descriptor, record bounds and candidate
tail-header fields. Opaque tails are compared for equality in memory without
displaying them or interpreting their contents. No other payload class, B
table, Quill, geometry, text, colour or formatting is decoded in this step.
The private original and extracted stream bytes are not committed or reproduced.

### Prefix, descriptor and bounds

OBSERVATION: descriptor index 49 selects the descriptor at Contents `0x21C`
in both samples. Its numeric words are word 0 = `0x0808`, word 1 = 2,
word 2 = 4; its sole nonzero selector is 6. The terminator is at `0x224`.
Word 1 supplies the numeric class-2 label used by this audit, not a decoded
semantic type. The same descriptor was already inventoried before AP5.

OBSERVATION: for each selected record at R, the leading u16 is 49, and the
bytes at R+2 and R+3 both have numeric value 8. They match both bytes of
descriptor word 0. The existing H-boundary convention therefore divides
the directory span into `[R,R+8)` and the opaque residual `[R+8,E)`.
The directory entry's offset field at D+4 supplies R; the next directory
entry's offset field supplies E. Every selected span is 30, leaving 22
bytes after H=8. No alignment rounding is used.

INFERENCE: R+8 is the prefix-suggested fixed-part end and tail start for
these five records. UNKNOWN: the equal prefix/descriptor bytes do not tell
which copy a general reader should treat as authoritative; the word 4 is
not independently proven to be a width merely by occurring in a descriptor.

OBSERVATION: in the fields table below, positions and IDs are hexadecimal;
sizes/counts are decimal. Reads are little-endian. Ranges are end-exclusive.
The two candidate tail fields are numeric probes, not named decoded fields.

| Basis | Sample / ID | Directory D | R / E / span | Prefix bytes at / values | H boundary / tail bytes | Candidate u32 at / value | Next candidate u32 at / value |
| --- | --- | --- | --- | --- | --- | --- | --- |
| OBSERVATION | Sample98 / `0x103` | `0x450` | `0xD14 / 0xD32 / 30` | `0xD16,0xD17 / 8,8` | `0xD1C / 22` | `0xD1C / 0` | `0xD20 / 4` |
| OBSERVATION | Sample98 / `0x101` | `0x478` | `0xD80 / 0xD9E / 30` | `0xD82,0xD83 / 8,8` | `0xD88 / 22` | `0xD88 / 0` | `0xD8C / 4` |
| OBSERVATION | mspub / `0x122` | `0x464` | `0xC7C / 0xC9A / 30` | `0xC7E,0xC7F / 8,8` | `0xC84 / 22` | `0xC84 / 0` | `0xC88 / 4` |
| OBSERVATION | mspub / `0x126` | `0x48C` | `0xD40 / 0xD5E / 30` | `0xD42,0xD43 / 8,8` | `0xD48 / 22` | `0xD48 / 0` | `0xD4C / 4` |
| OBSERVATION | mspub / `0x124` | `0x4A0` | `0xD78 / 0xD96 / 30` | `0xD7A,0xD7B / 8,8` | `0xD80 / 22` | `0xD80 / 0` | `0xD84 / 4` |

OBSERVATION: the first candidate is at R+8; the next is at R+12. Reading
their low u16 instead also gives 0 and 4. These examples cannot distinguish
u16 from u32 storage semantics when the higher bytes are zero. No meaning
is assigned to the remaining fourteen tail bytes after those two probes.

### Tail-length rules and counterexamples

OBSERVATION: **2/2 Sample98 and 3/3 mspub records** satisfy the constant
observation `E = R + 8 + 22`. All five complete tails are identical in the
in-memory equality check. There is no counterexample to that fixed-22-byte
fit among these five records. This is an exact observation of supplied
instances, not an independently stored total length or a varied-length test.

| Basis | Candidate rule | Predicted value | Measured value / result |
| --- | --- | --- | --- |
| OBSERVATION | Constant tail size 22 | Tail 22, record 30 | 5/5 fit; no counterexamples in this set. |
| OBSERVATION | First candidate u32 is total tail-byte length | Tail 0 | Refuted in 5/5; actual tail 22. |
| OBSERVATION | Next candidate u32 is total tail-byte length | Tail 4 | Refuted in 5/5; actual tail 22. |
| OBSERVATION | L or H is total record length | Record 8 | Refuted in 5/5; actual record span 30. |
| OBSERVATION | Entire tail is a headerless list with descriptor width 4 | Whole number of four-byte elements | Refuted in 5/5: 22 leaves remainder 2 on division by 4. |
| OBSERVATION | Reuse AP4 A: `H+10+n*w`, probing u16 n at R+8 and w at R+12 | Record 18 | Refuted in 5/5; under-consumes by 12. Class 2 is outside A's documented applicability. |
| OBSERVATION | Reuse AP4 O: `H+8+q`, probing u32 q at R+8 | Record 16 | Refuted in 5/5; under-consumes by 14. Class 2 is outside O's documented applicability. |

OBSERVATION: every refutation in the table applies to the complete same list:
Sample98 IDs `0x103,0x101`; mspub IDs `0x122,0x126,0x124`, with the exact
positions in the fields table. There are no omitted exceptions. The failed
A/O transplants do not contradict AP4's rules: those rules explicitly
exclude numeric class 2. Treating under-consumed bytes as padding would
invent a framing rule, so no such padding is discarded.

UNKNOWN: no variable tail-length rule is proven. A first value of zero could
be a count for an empty container with non-element overhead; equality to
zero is not proof of that meaning. The matching descriptor/instance value
4 could be a width, a size or another field. A header plus fixed-size
elements remains possible, as does an opaque fixed-size structure or a
different length representation not established here. None is selected
from five identical tails.

UNKNOWN: the tail cannot yet be classified as a proven fixed-element list
or a proven stored-length byte run. That is **neither interpretation proven**,
not proof that the actual format is neither. Only the headerless four-byte
list and the two specifically probed whole-tail byte-length interpretations
are refuted. No content meaning is needed or inferred from these checks.

### Extent accounting and checkpoint

OBSERVATION: the prior **70/93 field-driven extent checks remain explained**.
The five selected records add five exact constant-tail fits, giving **75/93
fits only if that provisional constant is included**. They do not raise the
independently length-derived count to 75: the 22-byte size was measured from
directory ends, and no stored field or varying example independently predicts
it. Of the 23 cases lacking an independent rule, these five are now narrowed
to identical opaque tails and the other eighteen remain outside this step.

INFERENCE: the next smallest available discriminating check is the two
class-2/index-82 records already listed by AP4: their H=18 and spans 244/40
leave residual sizes 226/22. Compare only structural count/length fields and
boundaries, keeping both payloads opaque. Such differing residual sizes can
test a shared class-2 framing rule; no claim that they actually share one
is made here. No index-82 body was inspected during AP5.

UNKNOWN: stored count/length/width roles, whether this is an empty container
or an intrinsically fixed-size tail, any internal padding, general class-2
framing, flag meanings, and behavior for nonidentical index-49 tails remain
unverified. Sample independence is owner-reported provenance, not established
by tail equality. No geometry, text, colour or formatting meaning was decoded.

OBSERVATION: this is the first AP5 checkpoint. Fresh bounded checks pass both
original identity checks, descriptor selection, all five directory spans and
H boundaries, tail sizes/equality and the candidate-rule refutations. Only
this appended section and the narrowed ranked-unknowns section change. No
full CI, importer/protected-reader edit, environment script/setting change,
environment save/publication, B work, Quill repair or pull request was made.
No sample or extracted stream bytes are committed, and no extra remote was added.

## Class-2 index-82 comparison

OBSERVATION: AP6 starts from `6e01c1b754788fc2fb4c49a6112ac58db3885d79`.
The restore check verifies all 317 pinned inputs, restores none and reports
zero failures. The corpus folders contain 142 controlled and 24 Apache
publications. Sample98.pub matches its recorded 61440-byte size/hash; the
attachment remains available and matches 19968 bytes and SHA-256
`0EFF0498172805BF1E52C74A6334ECC4CDC84FB2E531C0EA25734CFAC7923C7D`.
Fresh root Contents extractions remain in the ignored research directory.
Only structural measurements and equality classes are reproduced; no private
sample bytes, text or pictures are reproduced. SOURCES.md already lists both
inputs and is unchanged. No single-sample fallback is needed.

### Record locations and field evidence

OBSERVATION: positions below are hexadecimal absolute Contents offsets;
counts and sizes are decimal. Ranges are end-exclusive; integer probes use
little-endian reads. A fresh descriptor walk ends at `0x37E` with 88 entries
in each file. Descriptor 82 at `0x346` has words `0x1212`, 2, 30 and no
nonzero selectors; its zero terminator is at `0x34C`, next entry at `0x34E`.
Numeric class 2 is descriptor word 1, with no semantic type name assigned.

| Basis | Field/check | Sample98.pub | mspub.pub |
| --- | --- | --- | --- |
| OBSERVATION | Directory ordinal / ID / entry D | 29 / `0x11C` / `0x4C8` | 32 / `0x114` / `0x4E6` |
| OBSERVATION | D+4 u32 gives R | `0x4CC` gives `0xF1A` | `0x4EA` gives `0xEE6` |
| OBSERVATION | Next D+4 u32 gives E | `0x4D6` gives `0x100E` | `0x4F4` gives `0xF0E` |
| OBSERVATION | Leading u16 / prefix bytes +2,+3 | `0xF1A`: 82; `0xF1C,0xF1D`: 18,18 | `0xEE6`: 82; `0xEE8,0xEE9`: 18,18 |
| OBSERVATION | H boundary / fixed part / residual | H=18; `[0xF1A,0xF2C)`; tail 226 | H=18; `[0xEE6,0xEF8)`; tail 22 |
| OBSERVATION | u16 at T=R+H / n | `0xF2C` / 6 | `0xEF8` / 0 |
| OBSERVATION | u16 at T+2 / repeated n | `0xF2E` / 6 | `0xEFA` / 0 |
| OBSERVATION | u16 at T+4 / w | `0xF30` / 30 | `0xEFC` / 30 |
| OBSERVATION | T+6 four-byte field | `[0xF32,0xF36)` zero | `[0xEFE,0xF02)` zero |
| OBSERVATION | Eight opaque overhead bytes T+10..T+18 | `[0xF36,0xF3E)` | `[0xF02,0xF0A)` |
| OBSERVATION | n four-byte slots starting T+18 | `[0xF3E,0xF56)`; six distinct slots | `[0xF0A,0xF0A)`; zero slots |
| OBSERVATION | Common four-byte boundary word | `[0xF56,0xF5A)` | `[0xF0A,0xF0E)`; equal in memory |
| OBSERVATION | n blocks of w bytes after boundary word | `[0xF5A,0x100E)`; 6 times 30 | `[0xF0E,0xF0E)`; zero blocks |
| OBSERVATION | Predicted total span / directory difference | `18+22+6*(4+30)=244`; agrees | `18+22+0*(4+30)=40`; agrees |

OBSERVATION: prefix bytes match descriptor word 0's two bytes, fixing the
same H=18 convention as earlier sections. Descriptor word 2 matches the
instance w in both records. Directory endpoints are used only to validate
the prediction after reading these fields. Neither prefix byte is a total
record length. UNKNOWN: which duplicate boundary/count/width source is
authoritative, and whether w is semantically u16 or a byte with a zero high
byte, remain undistinguished.

### Complete byte-class comparison without content interpretation

OBSERVATION: all 18 fixed-part bytes compare equal between the two index-82
records; there are no differing fixed-part positions. At the same relative
tail positions within their common first 22 bytes, equality and difference
partition every position as follows. Relative offsets in this paragraph are
decimal: equal = `1,3,4..11,13..17`; different = `0,2,12,18..21`.
The count changes occupy low bytes 0 and 2; the width field is unchanged.
The four zero bytes at 6..9 are unchanged. The eight opaque bytes at 10..17
differ only at byte 12. No meaning is assigned to that difference.

OBSERVATION: the final four bytes of the short tail equal long-tail bytes
42..45, not bytes 18..21. Thus the boundary word shifts by 24 bytes, exactly
six four-byte slots. The full short 22-byte tail does not occur as a contiguous
substring anywhere in the long tail. The long tail is not simply the short
tail followed by 204 appended bytes. The six slots are distinct four-byte
values; no reference, ordering or content role is assigned to them here.

OBSERVATION: after that common boundary word, the six candidate 30-byte
blocks start at `0xF5A,0xF78,0xF96,0xFB4,0xFD2,0xFF0` and end exactly at
`0x100E`. Comparing corresponding bytes across all six partitions every
block-relative position (decimal): equal = `0..3,5..7,17..29`;
varying = `4,8..16`. A u32 probe at block+4 gives the sequence 1 through 6.
These repeated-position checks support the descriptor/instance width of 30
without decoding the varying bytes or assigning any payload meaning.

INFERENCE: the surviving structural model is fixed overhead plus two
counted regions: n four-byte slots, then a common four-byte boundary word,
then n blocks of width w. The ten-byte tail header and eight opaque overhead
bytes precede the slots. This is a framing model, not proof that the format
names these regions as two arrays or that slots associate with blocks in
their physical order. A single abstract list of n entries of width w+4 has
the same total arithmetic, but does not by itself explain the observed
separated repeated regions. UNKNOWN: that abstract alternative is not
mathematically excluded by two lengths alone.

### Competing structure tests on all seven records

OBSERVATION: with T=R+H, the tested rule is
`E = R + H + 22 + n*(4+w)`. Equivalently, the block start is
`B = T+18+4*n+4` and the predicted end is `B+n*w`. The overhead 22 is
present in all six empty examples; using that overhead and the stored
nonzero n and w independently predicts the populated endpoint. The 204-byte
difference decomposes into 24 slot bytes and 180 block bytes. Simply using
n descriptor-width elements with overhead 22 predicts Sample98 span 220,
24 short of 244. No remainder is discarded as padding.

| Basis | Sample / ID / index | H / n / repeated n / w | Rule span / measured span | Result |
| --- | --- | --- | --- | --- |
| OBSERVATION | Sample98 / `0x103` / 49 | 8 / 0 / 0 / 4 | 30 / 30 | agrees |
| OBSERVATION | Sample98 / `0x101` / 49 | 8 / 0 / 0 / 4 | 30 / 30 | agrees |
| OBSERVATION | Sample98 / `0x11C` / 82 | 18 / 6 / 6 / 30 | 244 / 244 | agrees |
| OBSERVATION | mspub / `0x122` / 49 | 8 / 0 / 0 / 4 | 30 / 30 | agrees |
| OBSERVATION | mspub / `0x126` / 49 | 8 / 0 / 0 / 4 | 30 / 30 | agrees |
| OBSERVATION | mspub / `0x124` / 49 | 8 / 0 / 0 / 4 | 30 / 30 | agrees |
| OBSERVATION | mspub / `0x114` / 82 | 18 / 0 / 0 / 30 | 40 / 40 | agrees |

OBSERVATION: index-49 field locations remain exactly those in AP5's fields
table. Reading T and T+2 separately resolves its earlier u32-at-T probe into
two zero count-like words; T+4 matches descriptor width 4. All seven have
the same four-byte boundary word at T+18+4*n, checked by in-memory equality.
The rule has **7/7 agreements and no counterexamples** in the complete
selected set, including **one nonempty and six empty** tails.

| Basis | Competing interpretation | Populated Sample98 index 82 | Empty mspub index 82 and five index-49 records |
| --- | --- | --- | --- |
| OBSERVATION | Entire tail is only n equal elements, with no overhead | Refuted: `6*30=180`, not 226 | Refuted: zero elements do not consume 22 bytes |
| OBSERVATION | Overhead 22 plus n elements of descriptor width w, no slots | Refuted: 202-byte tail leaves 24 bytes | Fits all six; empty cases cannot expose missing slots |
| OBSERVATION | Overhead 22 plus n four-byte slots and n w-byte blocks | Fits both region boundaries and total | Fits; zero slots/blocks, retained overhead |
| OBSERVATION | First or second u16 is total tail-byte length | Refuted: 6 is not 226 | Refuted: 0 is not 22 |
| OBSERVATION | u16 at T+4 is total tail-byte length | Refuted: 30 is not 226 | Refuted: 30 or 4 is not 22 |
| OBSERVATION | Universal constant 22-byte tail / fixed structure without variable regions | Refuted: tail 226 | Fits all six; cannot separate from empty counted structure |
| INFERENCE | Fixed overhead with counted fixed-width regions | Survives and explains 204-byte excess | Survives; nonempty width-4 behavior still untested |
| UNKNOWN | Byte run with some other stored length or indirect length | No such length established | Not separable without identifying that field |

OBSERVATION: the failed literal byte-length interpretations apply to every
record in the seven-row census. The AP4 A/O rules still do not apply to
class 2; transplanting them also disagrees in all seven checks. No direct
total tail-length field is established. The observed count is an element
count for the tested framing arithmetic, not the full tail's byte length.
UNKNOWN: count versus capacity and the distinct roles of the two equal
words remain unresolved. A stored-length byte run is not disproved in every
possible representation merely by rejecting these particular fields.

### Updated extent count, unknowns and checkpoint

OBSERVATION: AP4's 70 independently field-driven extents plus these seven
sampled field-driven class-2 extents give **77/93**, comprising **47/54**
Sample98 and **30/39** mspub. None of the seven now relies solely on AP5's
constant-tail fit: a single count/width equation is checked on both zero
and nonzero counts. There are **zero additional constant-only provisional
fits counted here**, and **16 remaining unexplained extents** (7/54 and
9/39). This count concerns endpoint predictions in the supplied samples,
not universal decoding or proof that every byte has a semantic field name.

UNKNOWN: the only populated class-2 example is Sample98 index 82. Extending
the equation to nonzero index-49 counts or a second nonzero index-82 count
remains provisional and is not counted as additional tested records.
Duplicate count/capacity roles, the opaque eight-byte overhead, the boundary
word's role, the four-byte slots' meaning and relationship to blocks, internal
padding inside opaque blocks, saving versions and malformed-input behavior
remain unverified. No geometry, text, colour or formatting was decoded.

INFERENCE: the next smallest available step is the two class-10/index-76
records from AP4's inventory, with H=18 and residual sizes 52/22. Compare
only their own framing fields, without importing the class-2 equation by
analogy. No index-76 body was inspected in AP6. UNKNOWN: discriminating
duplicate count roles still requires an unequal pair or other independent
evidence absent from these seven records.

OBSERVATION: this is the first AP6 checkpoint. The bounded check passes both
original identities, fresh descriptor/directory walks, seven field-driven
endpoints, complete fixed/shared-tail equality masks, the common boundary
word and the six repeated-block checks. Only this appended section and the
narrowed ranked-unknowns section change. No full CI, importer/protected-reader
edit, B work, Quill repair, environment script/setting change, environment
save/publication or pull request was performed. No sample or stream bytes
are committed, and no extra remote was added.

## Class-10 index-76 comparison

OBSERVATION: AP7 starts from `29fa02c2f0b0ede510e741f58c16e352c7c0b6b8`,
titled Explain counted regions in legacy record tails. Fetch succeeds and
the named branch is already at that commit. The restore verifies 317 pinned
inputs, restores none and reports zero failures; the binary folders contain
142 controlled and 24 Apache publications. Sample98.pub matches its recorded
61440-byte size and hash. The attachment remains present and matches 19968
bytes and SHA-256
`0EFF0498172805BF1E52C74A6334ECC4CDC84FB2E531C0EA25734CFAC7923C7D`.
Fresh root Contents extractions remain in the ignored research directory.
No private bytes or content are reproduced; only derived structural facts
and equality classes are recorded. Both sources are already listed in
SOURCES.md, which is unchanged; no single-sample fallback is needed.

### Fields and directory bounds, side by side

OBSERVATION: positions below are hexadecimal Contents offsets, sizes/counts
are decimal, integer reads are little-endian and ranges are end-exclusive.
The fresh table walk reaches `0x37E` after 88 descriptors in both files.
Descriptor 76 is at `0x316`: word 0 is `0x1212`, word 1 is 10, word 2 is
2, and the selector list is empty. The zero terminator is at `0x31C`, with
the next descriptor at `0x31E`. Numeric class 10 names only word 1 here,
not a decoded semantic type.

| Basis | Field/check | Sample98.pub | mspub.pub |
| --- | --- | --- | --- |
| OBSERVATION | Directory ordinal / ID / entry D | 28 / `0x11B` / `0x4BE` | 31 / `0x117` / `0x4DC` |
| OBSERVATION | D+4 u32 supplies R | `0x4C2`: `0xED4` | `0x4E0`: `0xEBE` |
| OBSERVATION | Next D+4 u32 supplies E | `0x4CC`: `0xF1A` | `0x4EA`: `0xEE6` |
| OBSERVATION | Leading u16 index / prefix bytes +2,+3 | `0xED4`: 76; `0xED6,0xED7`: 18,18 | `0xEBE`: 76; `0xEC0,0xEC1`: 18,18 |
| OBSERVATION | Fixed part / T=R+H / residual | `[0xED4,0xEE6)` / `0xEE6` / 52 | `[0xEBE,0xED0)` / `0xED0` / 22 |
| OBSERVATION | First u16 count-like field T / n | `0xEE6` / 5 | `0xED0` / 0 |
| OBSERVATION | Second u16 count-like field T+2 | `0xEE8` / 5 | `0xED2` / 0 |
| OBSERVATION | u16 at T+4 / w | `0xEEA` / 2 | `0xED4` / 2 |
| OBSERVATION | Four-byte field at T+6 | `[0xEEC,0xEF0)` zero | `[0xED6,0xEDA)` zero |
| OBSERVATION | Eight opaque overhead bytes T+10..T+18 | `[0xEF0,0xEF8)` | `[0xEDA,0xEE2)` |
| OBSERVATION | n four-byte slots from T+18 | `[0xEF8,0xF0C)`; five distinct slots | `[0xEE2,0xEE2)`; zero slots |
| OBSERVATION | Common four-byte boundary word | `[0xF0C,0xF10)` | `[0xEE2,0xEE6)`; equal in memory |
| OBSERVATION | n w-byte blocks after boundary word | `[0xF10,0xF1A)`; five two-byte blocks | `[0xEE6,0xEE6)`; zero blocks |
| OBSERVATION | Predicted / directory-bounded span | `18+22+5*(4+2)=70` / 70 | `18+22+0*(4+2)=40` / 40 |

OBSERVATION: H=18 matches both prefix bytes and both bytes of descriptor
word 0. The instance width at T+4 matches descriptor word 2 in each file.
The next directory offset supplies an independent endpoint check, not the
value used to choose n or w. Neither H nor either count is a total-record
byte length. UNKNOWN: authoritative duplicate fields and byte-versus-u16
width semantics remain undistinguished because the duplicate values agree
and the width high byte is zero.

### Byte classes and counted regions

OBSERVATION: all 18 fixed-part bytes are equal between these two records;
no fixed-part position differs. In the shared first 22 tail bytes, the
complete position partition is (decimal relative to T): equal =
`1,3,4..11,13,15..17`; different = `0,2,12,14,18..21`.
Bytes 0 and 2 are the changing low bytes of the count-like pair. Bytes
4..5 (width) and 6..9 (zero field) are unchanged. Of the eight opaque
overhead bytes at 10..17, only positions 12 and 14 differ. Their meanings
remain UNKNOWN; no interpretation is assigned to those values.

OBSERVATION: the short tail's last four bytes equal long-tail bytes 38..41,
and differ from long-tail bytes 18..21. This is a 20-byte displacement,
matching five four-byte slots. The complete short tail is not a contiguous
substring of the long tail. The long tail is therefore not the unchanged
short tail with 30 bytes appended. The boundary word also equals the one
checked in all seven prior index-49/index-82 records, by in-memory comparison.

OBSERVATION: five distinct four-byte slots occupy `[0xEF8,0xF0C)`.
The five distinct two-byte blocks start at
`0xF10,0xF12,0xF14,0xF16,0xF18` and end exactly at `0xF1A`.
Across those blocks, byte position 1 is identical and byte position 0
varies in all five instances. Their contents remain opaque; no link or
semantic interpretation is assigned to either region. The short sample
has zero slots and zero blocks under the same boundary calculation.

### Rule test and scope

OBSERVATION: the AP6 equation **survives unchanged**:
`span = H + 22 + n*(4+w)`. The longer residual's 30-byte excess is exactly
`5*4 + 5*2`: 20 slot bytes plus ten block bytes. With
`B=T+18+4*n+4`, both predicted ends `B+n*w` equal the next directory
offset. There is no gap, overrun or rounding in either check. No fixed
overhead, count-field or width term fails, so no replacement is needed.

| Basis | Sample / ID / index / numeric class | H / n / repeated n / w | Predicted span / measured span |
| --- | --- | --- | --- |
| OBSERVATION | Sample98 / `0x103` / 49 / 2 | 8 / 0 / 0 / 4 | 30 / 30 |
| OBSERVATION | Sample98 / `0x101` / 49 / 2 | 8 / 0 / 0 / 4 | 30 / 30 |
| OBSERVATION | Sample98 / `0x11C` / 82 / 2 | 18 / 6 / 6 / 30 | 244 / 244 |
| OBSERVATION | mspub / `0x122` / 49 / 2 | 8 / 0 / 0 / 4 | 30 / 30 |
| OBSERVATION | mspub / `0x126` / 49 / 2 | 8 / 0 / 0 / 4 | 30 / 30 |
| OBSERVATION | mspub / `0x124` / 49 / 2 | 8 / 0 / 0 / 4 | 30 / 30 |
| OBSERVATION | mspub / `0x114` / 82 / 2 | 18 / 0 / 0 / 30 | 40 / 40 |
| OBSERVATION | Sample98 / `0x11B` / 76 / 10 | 18 / 5 / 5 / 2 | 70 / 70 |
| OBSERVATION | mspub / `0x117` / 76 / 10 | 18 / 0 / 0 / 2 | 40 / 40 |

OBSERVATION: all seven earlier class-2 records were freshly rechecked,
including AP6's equality masks and six 30-byte block boundaries. Together
with these two new records the equation has **9/9 agreements**, zero
counterexamples, two populated tails and seven empty tails. Populated
widths now include 30 and 2. AP4's other 70 predictions retain their own
class-selected F/A/O rules; this comparison does not replace those rules
or claim that class 2/10 framing applies universally. A fresh walk compares
all 93 directory entries, offsets, leading indices and descriptor classes
with the prior inventory, without inspecting other class bodies.

OBSERVATION: in the new populated record, overhead 22 plus only n*w predicts
span 50, not 70: omitting the slots loses 20 bytes. A universal fixed
22-byte tail fails on its measured 52-byte residual. Treating either count
as a whole-tail length predicts 5/0 instead of 52/22, and treating w as
that length predicts 2 instead of either. The empty record alone cannot
separate counted regions from a constant structure. No direct stored total
byte-length field is established by these tests.

INFERENCE: the smallest surviving shared structural model remains fixed
overhead plus n four-byte slots and n fixed-width blocks, with a common
boundary word between those regions. This predicts extents across the
tested class-2 and class-10 records without interpreting the blocks.
UNKNOWN: parallel-array semantics, slot-to-block association and an abstract
single list of width w+4 are not uniquely distinguished by the arithmetic.

### Updated extent count and complete unexplained list

OBSERVATION: the retained 70 AP4 predictions plus seven AP6 predictions
plus two new predictions give **79/93 field-driven sampled extents**:
**48/54 Sample98** and **31/39 mspub**. **14 remain unexplained** (6 and
8 respectively); zero additional constant-only provisional fits are counted.
The directory and descriptor inventory supplies the complete remaining list
below. IDs are hexadecimal; indices and class labels are decimal. No body
of an unexplained record was decoded during AP7.

| Basis | Index / numeric class | Sample98 IDs | mspub IDs | Unexplained count |
| --- | --- | --- | --- | --- |
| OBSERVATION | 30 / 3 | `0x119` | `0x110` | 2 |
| OBSERVATION | 31 / 3 | `0x105` | `0x120` | 2 |
| OBSERVATION | 35 / 3 | none | `0x101` | 1 |
| OBSERVATION | 40 / 3 | `0x123` | `0x102` | 2 |
| OBSERVATION | 41 / 195 | `0x11F` | `0x111` | 2 |
| OBSERVATION | 50 / 3 | `0x11E,0x104` | `0x121,0x125,0x123` | 5 |

UNKNOWN: 79 endpoints agreeing does not establish a universal record reader,
semantic contents or padding inside opaque regions. Each of indices 82 and
76 still has only one populated example, and index 49 has none. Count versus
capacity and the roles of the two equal words, authoritative descriptor
versus instance fields, the eight-byte overhead, boundary word meaning,
slot/block relationship, flags, exact saving versions and malformed-input
behavior remain unverified. No geometry, text, colour or formatting was decoded.

INFERENCE: the next smallest available discriminating step is the two
class-3/index-30 records already inventoried by AP4, with H=4 and spans
26/114, leaving residual sizes 22/110. Compare their own framing fields
and boundaries without presuming this equation applies. Their bodies were
not inspected during AP7. UNKNOWN: unequal duplicate count roles still
require evidence absent from all nine class-2/class-10 records.

OBSERVATION: this is the first AP7 checkpoint. Original identity checks,
fresh descriptor/directory comparisons, all nine equation checks, fixed/shared
byte equality partitions, boundary-word equality and counted-region checks
pass. The only changes are this appended section and the narrowed ranked
unknowns. SOURCES.md is unchanged. No full CI, importer/protected-reader edit,
B work, Quill repair, environment script/setting change, environment
save/publication or pull request was performed. Sample and stream bytes remain
uncommitted; no extra remote was added.

## Class-3 index-30 comparison

OBSERVATION: AP8 starts from `eaf5b9a24275b9502e932fd777de790110db5d9e`,
titled Check counted framing in two more legacy records. Fetch succeeds and
the branch is at that commit. The restore verifies all 317 pinned inputs,
restores none and reports zero failures. The binary folders contain 142
controlled and 24 Apache publications. Sample98.pub matches its recorded
61440-byte size/hash; the attachment remains available and matches 19968
bytes and SHA-256
`0EFF0498172805BF1E52C74A6334ECC4CDC84FB2E531C0EA25734CFAC7923C7D`.
Fresh Contents extracts remain in the ignored research directory. No private
sample bytes or content are reproduced; all facts below are structural
measurements or equality classes. Both samples are already listed in
SOURCES.md, which is unchanged. No single-sample fallback is needed.

### Index-30 fields and bounds, side by side

OBSERVATION: positions are hexadecimal absolute Contents offsets; sizes,
counts and relative offset-table values are decimal. Reads are little-endian
and ranges are end-exclusive. A fresh descriptor walk ends at `0x37E` after
88 entries in each file. Descriptor 30 at `0x17E` has words `0x0408`, 3,
198, no nonzero selectors, and a terminator at `0x184`; the next descriptor
starts at `0x186`. Numeric class 3 names only descriptor word 1.

| Basis | Field/check | Sample98.pub | mspub.pub |
| --- | --- | --- | --- |
| OBSERVATION | Directory ordinal / ID / D | 19 / `0x119` / `0x464` | 21 / `0x110` / `0x478` |
| OBSERVATION | D+4 u32 supplies R | `0x468`: `0xD4C` | `0x47C`: `0xCB4` |
| OBSERVATION | Next D+4 u32 supplies E | `0x472`: `0xD66` | `0x486`: `0xD26` |
| OBSERVATION | Leading index / prefix +2,+3 | `0xD4C`: 30; `0xD4E,0xD4F`: 8,4 | `0xCB4`: 30; `0xCB6,0xCB7`: 8,4 |
| OBSERVATION | H / fixed part / T / residual | 4 / `[0xD4C,0xD50)` / `0xD50` / 22 | 4 / `[0xCB4,0xCB8)` / `0xCB8` / 110 |
| OBSERVATION | u32 at T / n | `0xD50` / 0 | `0xCB8` / 3 |
| OBSERVATION | u32 at T+4 / repeated n | `0xD54` / 0 | `0xCBC` / 3 |
| OBSERVATION | u32 at T+8 / q | `0xD58` / 4 | `0xCC0` / 92 |
| OBSERVATION | Six opaque header bytes at T+12..T+18 | `[0xD5C,0xD62)` | `[0xCC4,0xCCA)`; equal in memory |
| OBSERVATION | Relative-offset base O=T+18 | `0xD62` | `0xCCA` |
| OBSERVATION | n+1 u32 offsets at O | `[0xD62,0xD66)`: 4 | `[0xCCA,0xCDA)`: 16,50,66,92 |
| OBSERVATION | Framed payload spans | none; `[0xD66,0xD66)` | `[0xCDA,0xCFC)`, `[0xCFC,0xD0C)`, `[0xD0C,0xD26)` |
| OBSERVATION | Payload widths from successive offsets | none | 34,16,26; total 76 |
| OBSERVATION | Predicted span H+18+q / measured span | 26 / 26 | 114 / 114 |

OBSERVATION: H is the high prefix byte and high byte of descriptor word 0,
both 4. The low byte is 8 in both descriptor and instance, so substituting
that byte for H would move the tail start four bytes too far. The low byte
is not silently treated as the fixed-part length. Descriptor word 2 is 198,
not any of the measured payload widths; it does not supply the fixed width
used by the earlier class-2/class-10 equation in these records.

### Complete byte classes and variable spans

OBSERVATION: all four fixed-part bytes compare equal; no fixed-part position
differs. Across the first 22 tail bytes, every position is accounted for
by this partition (decimal relative to T): equal =
`1..3,5..7,9..17,19..21`; different = `0,4,8,18`.
The differences are the low bytes of n, repeated n, q and the first offset.
The six opaque bytes at T+12..T+18 are unchanged. Their meaning remains
UNKNOWN; none is decoded as geometry, text, colour or formatting.

OBSERVATION: the short tail's final offset word does not appear anywhere
as an equal four-byte word in the long tail; its complete 22 bytes are also
not a contiguous substring of that tail. The terminal word in class 3 is
the relative end offset q, not the common constant boundary word observed
in class 2/10. The long tail has an offset table of 16 bytes and 76 opaque
payload bytes; the short tail has a four-byte offset table and zero payload
bytes. The excess 88 bytes consists of twelve extra table bytes plus 76
payload bytes. The three framed spans have unequal widths 34,16,26.

OBSERVATION: this provides a stored count and repeated framing, but not a
fixed-width list of the three counted payloads. Keeping overhead 22 and
three four-byte slots would leave 76 bytes for three elements, which is
not divisible by three. Thus no single integer w can make the original
equation fit that record with n=3. UNKNOWN: smaller atomic units inside
the opaque spans are not ruled out, and their contents are not inspected
for a meaning or decoded as another class.

### Failed fixed-width rule and the smallest alternative

OBSERVATION: applying the earlier field positions literally gives u16 n at
T = 0/3, u16 at T+2 = 0/0 and u16 w at T+4 = 0/3. The populated record's
predicted span is `4+22+3*(4+3)=47`, not 114: under-consumption by 67.
T+4 is the repeated count here, not a width; T+2 is the zero upper half
of the first u32 probe, not the repeated count. The numeric first count
remains usable, but the duplicate/width field layout changes. The 22-byte
base overhead need not change. Substituting descriptor width 198 also
fails, predicting 632. The empty record fits 26 without testing width.

INFERENCE: the smallest arithmetic generalization retaining that overhead
and count is `span = H+22+4*n+sum(payload widths)`. In the earlier nine
class-2/class-10 records, the sum remains n*w, so all nine old predictions
are unchanged. In class 3, widths come from a relative-offset table rather
than a stored common width. This is a shared arithmetic relationship, not
a claim that the two families share field representations or offset tables.

OBSERVATION: the directly field-driven class-3 check is `span = H+18+q`.
At T, T+4 and T+8 read n, repeated n and q with u32 probes. At O=T+18
read n+1 u32 offsets; require first offset `4*(n+1)`, monotonic increasing
boundaries for nonempty spans, and final offset q. Payload j is bounded by
`[O+offset[j],O+offset[j+1])`; every table read and span must fit before
O+q. The two index-30 examples give first/last offsets 4/4 and 16/92.
Predicted O+q then equals E in both, independently of the directory difference.

UNKNOWN: these observations do not establish that the three fields are
semantically u32 rather than narrower values with zero upper bytes. Count
versus capacity is still unresolved because the first two values agree.
The six header bytes, relative-offset base meaning beyond these samples,
and authoritative descriptor/instance choices remain unverified.

### Every other class-3 record and earlier checks

OBSERVATION: the table covers all twelve class-3 records, including the
index-30 pair. D/R/T/O/E are hexadecimal; offsets, widths and spans are
decimal. For each row, n and repeated n agree. The count and q are read
at T and T+8; the repeated count is at T+4. Offset entries begin at O.
These positions specify each field's absolute evidence without reproducing
the opaque payload. The original-rule column uses its literal u16 reads
at T and T+4. Every revised prediction agrees with the directory span.

| Basis | Sample / ID / index | D / R / T / O / E | n / q | Relative offsets | Payload widths | Old span / revised span / measured span |
| --- | --- | --- | --- | --- | --- | --- |
| OBSERVATION | Sample98 / `0x123` / 40 | `0x3CE / 0x840 / 0x85C / 0x86E / 0x902` | 6 / 148 | 28,48,68,88,108,128,148 | 20,20,20,20,20,20 | 110 / 194 / 194 |
| OBSERVATION | Sample98 / `0x11E` / 50 | `0x45A / 0xD32 / 0xD36 / 0xD48 / 0xD4C` | 0 / 4 | 4 | empty | 26 / 26 / 26 |
| OBSERVATION | Sample98 / `0x119` / 30 | `0x464 / 0xD4C / 0xD50 / 0xD62 / 0xD66` | 0 / 4 | 4 | empty | 26 / 26 / 26 |
| OBSERVATION | Sample98 / `0x105` / 31 | `0x46E / 0xD66 / 0xD6A / 0xD7C / 0xD80` | 0 / 4 | 4 | empty | 26 / 26 / 26 |
| OBSERVATION | Sample98 / `0x104` / 50 | `0x482 / 0xD9E / 0xDA2 / 0xDB4 / 0xDB8` | 0 / 4 | 4 | empty | 26 / 26 / 26 |
| OBSERVATION | mspub / `0x102` / 40 | `0x3C4 / 0x6F4 / 0x710 / 0x722 / 0x7B6` | 6 / 148 | 28,48,68,88,108,128,148 | 20,20,20,20,20,20 | 110 / 194 / 194 |
| OBSERVATION | mspub / `0x101` / 35 | `0x45A / 0xC4A / 0xC4E / 0xC60 / 0xC7C` | 4 / 28 | 20,22,24,26,28 | 2,2,2,2 | 58 / 50 / 50 |
| OBSERVATION | mspub / `0x121` / 50 | `0x46E / 0xC9A / 0xC9E / 0xCB0 / 0xCB4` | 0 / 4 | 4 | empty | 26 / 26 / 26 |
| OBSERVATION | mspub / `0x110` / 30 | `0x478 / 0xCB4 / 0xCB8 / 0xCCA / 0xD26` | 3 / 92 | 16,50,66,92 | 34,16,26 | 47 / 114 / 114 |
| OBSERVATION | mspub / `0x120` / 31 | `0x482 / 0xD26 / 0xD2A / 0xD3C / 0xD40` | 0 / 4 | 4 | empty | 26 / 26 / 26 |
| OBSERVATION | mspub / `0x125` / 50 | `0x496 / 0xD5E / 0xD62 / 0xD74 / 0xD78` | 0 / 4 | 4 | empty | 26 / 26 / 26 |
| OBSERVATION | mspub / `0x123` / 50 | `0x4AA / 0xD96 / 0xD9A / 0xDAC / 0xDB0` | 0 / 4 | 4 | empty | 26 / 26 / 26 |

OBSERVATION: the original formula fits **8/12** empty class-3 records and
fails on all four populated cases: Sample98 index 40 under by 84; mspub
index 40 under by 84; mspub index 35 over by 8; mspub index 30 under by 67.
These are every counterexample in class 3, not suppressed exceptions.
The revised field/table rule predicts **12/12**, with zero disagreements:
eight empty and four populated tails. The index-35 and index-40 payloads
happen to have equal widths within each record, whereas index 30 does not.
Descriptor word 2 is zero for indices 31/35/40/50, so it cannot supply their
nonzero populated widths. No fixed-width claim follows from the empty cases.

OBSERVATION: all earlier nine class-2/class-10 checks, including their equality
and boundary checks, were rerun successfully. The generalized sum therefore
fits **21/21** tested class-2/3/10 records. AP4's other 70 class-selected
F/A/O predictions remain separate; this step does not apply either new
field layout to them. Fresh leading-index/directory/descriptor comparisons
match all 93 inventory entries. The class-195 bodies are not inspected.

### Updated extent count and checkpoint

OBSERVATION: of AP7's fourteen unexplained extents, all twelve class-3 cases
are now predicted by stored fields and checked offset tables. None refutes
that revised rule; the two class-195 cases stay unexplained. The total is
**91/93 field-driven sampled extent predictions**, comprising **53/54**
Sample98 and **38/39** mspub. Zero extra constant-only provisional fits
are counted. The remaining complete list is:

| Basis | Index / numeric class | Sample98 ID / H / span | mspub ID / H / span | Unexplained count |
| --- | --- | --- | --- | --- |
| OBSERVATION | 41 / 195 | `0x11F` / 56 / 294 | `0x111` / 56 / 236 | 2 |

UNKNOWN: agreeing endpoints do not prove internal padding, semantic payload
types, valid content or a universal reader. Duplicate count roles, nonempty
index-49 behavior, the role of descriptor word 2 in class 3, the six opaque
header bytes, relative-offset semantics outside these twelve records,
flags, saving versions and malformed-input behavior remain unverified.
Only one populated variable-width index-30 example is available. No geometry,
text, colour or formatting meaning was decoded.

INFERENCE: the next smallest step is the remaining pair of class-195/index-41
records already inventoried by AP4. H=56 and spans 294/236 leave residuals
238/180. Compare structural length/count fields and bounds without assuming
class-3 or class-2/10 framing applies. No index-41 body was inspected in AP8.

OBSERVATION: this is the first AP8 checkpoint. Both original identities,
fresh descriptor/directory comparisons, the fixed/shared-tail equality
partition, all twelve count/length/offset-table checks and the earlier nine
checks pass. Only this appended section and the narrowed ranked-unknowns
section change. No full CI, importer/protected-reader edit, B work, Quill
repair, environment script/setting change, environment save/publication or
pull request was performed. Sample and stream bytes remain uncommitted;
no extra remote was added.

## Class-195 index-41 comparison

OBSERVATION: AP9 starts from `dfab4296bc0a966388a3e82d1dfd0716df694398`,
titled Explain offset tables in legacy record tails. Fetch succeeds and the
named branch is at that commit. The restore verifies 317 pinned inputs,
restores none and reports zero failures; binary folder counts are 142
controlled and 24 Apache publications. Sample98.pub matches its recorded
61440-byte size/hash. The attachment remains present and matches 19968
bytes and SHA-256
`0EFF0498172805BF1E52C74A6334ECC4CDC84FB2E531C0EA25734CFAC7923C7D`.
Fresh root Contents extractions remain in the ignored research directory.
Only derived structural measurements and equality classes are reproduced;
private bytes, text and pictures remain uncommitted and undisplayed. Both
inputs are already listed in SOURCES.md, which is unchanged. No single-sample
fallback is needed.

### Fields, descriptors and directory bounds

OBSERVATION: positions are hexadecimal absolute Contents offsets; counts,
sizes and relative offset-table values are decimal. All integer probes
are little-endian and ranges are end-exclusive. Both fresh descriptor walks
consume 88 entries to `0x37E`. Descriptor 41 at `0x1D8` has words `0x3838`,
195, 0; its sole selector at `0x1DE` is 16, its terminator is at `0x1E0`,
and the next descriptor begins at `0x1E2`. Numeric 195 is the descriptor
class label, without a decoded semantic type or bit interpretation.

| Basis | Field/check | Sample98.pub | mspub.pub |
| --- | --- | --- | --- |
| OBSERVATION | Directory ordinal / ID / entry D | 2 / `0x11F` / `0x3BA` | 2 / `0x111` / `0x3BA` |
| OBSERVATION | D+4 u32 supplies R | `0x3BE`: `0x6AA` | `0x3BE`: `0x608` |
| OBSERVATION | Next D+4 u32 supplies E | `0x3C8`: `0x7D0` | `0x3C8`: `0x6F4` |
| OBSERVATION | Leading u16 index / prefix +2,+3 | `0x6AA`: 41; `0x6AC,0x6AD`: 56,56 | `0x608`: 41; `0x60A,0x60B`: 56,56 |
| OBSERVATION | H / fixed part / T=R+H / residual | 56 / `[0x6AA,0x6E2)` / `0x6E2` / 238 | 56 / `[0x608,0x640)` / `0x640` / 180 |
| OBSERVATION | u32 at T / n | `0x6E2` / 2 | `0x640` / 2 |
| OBSERVATION | u32 at T+4 / repeated n | `0x6E6` / 2 | `0x644` / 2 |
| OBSERVATION | u32 at T+8 / q | `0x6EA` / 220 | `0x648` / 162 |
| OBSERVATION | Six opaque bytes T+12..T+18 | `[0x6EE,0x6F4)` | `[0x64C,0x652)`; equal in memory |
| OBSERVATION | Relative-offset base O=T+18 | `0x6F4` | `0x652` |
| OBSERVATION | n+1 u32 offsets at O | `[0x6F4,0x700)`: 12,116,220 | `[0x652,0x65E)`: 12,112,162 |
| OBSERVATION | Two offset-bounded opaque payloads | `[0x700,0x768)`, `[0x768,0x7D0)` | `[0x65E,0x6C2)`, `[0x6C2,0x6F4)` |
| OBSERVATION | Payload widths / total | 104,104 / 208 | 100,50 / 150 |
| OBSERVATION | Predicted H+18+q / directory span | 294 / 294 | 236 / 236 |

OBSERVATION: both prefix bytes and both bytes of descriptor word 0 are 56,
so the existing H convention selects T unambiguously in these examples,
without proving which copy is authoritative. Descriptor word 2 is zero,
not an observed payload width. The selector at +16 is recorded as table
metadata; its target's role is not decoded here. E is the next directory
offset, used to check the field-driven prediction, not to supply q or widths.

### Complete byte classes, common prefix and suffix

OBSERVATION: fixed-part positions differing between the files are exactly
`16..18,38..40` (decimal relative to R). The other 50 positions are equal:
`0..15,19..37,41..55`. No meaning is assigned to these differing fields.

OBSERVATION: the following table partitions the shared first 180 tail bytes.
Every position not in the differing-position column is equal within its
listed range; offsets here are decimal relative to T. Sample98 alone has
the additional bytes at 180..237. This comparison does not transcribe any
opaque byte values.

| Basis | Tail range, end-exclusive | Differing positions in that range | Structural check |
| --- | --- | --- | --- |
| OBSERVATION | 0..18 | 8 | Counts agree; only the low byte of q differs; six opaque header bytes agree. |
| OBSERVATION | 18..30 | 22,26 | First offset 12 agrees; middle and final offsets differ. |
| OBSERVATION | 30..64 | 34,38,42,46,50 | Opaque payload equality classes only. |
| OBSERVATION | 64..130 | Even positions 64 through 124 | All other positions in this range agree. |
| OBSERVATION | 130..156 | 130,134,138,142,146,150,154 | All other positions in this range agree. |
| OBSERVATION | 156..168 | 156..158,160..162,164..166 | Positions 159,163,167 agree. |
| OBSERVATION | 168..180 | Even positions 168 through 178 | Odd positions agree. |

OBSERVATION: the exact common tail prefix is eight bytes; byte 8 differs.
The exact common end-aligned suffix is nine bytes; the preceding byte differs.
End-aligned suffix equality is a separate comparison from the same-offset
table above. These equalities do not prove padding or a suffix field meaning.
Both tails share the structural prefix of an 18-byte header plus twelve-byte
offset table. Each has two payload spans; the first widths differ by four,
the second by 54, accounting for the total residual-size difference of 58.
No common-width or common-content interpretation is inferred from equality.

### Three length-rule tests

OBSERVATION: both tails pass all of AP8's class-3 framing checks: n equals
the second count-like u32 probe; first offset equals `4*(n+1)=12`;
offsets strictly increase; final offset equals stored q; and every framed
span lies after the table and ends at or before O+q. The stored q counts
the offset table and payloads, not the whole tail: it is 220/162 while
the whole residual is 238/180. Predicted O+q equals E in both files.

| Basis | Rule / field interpretation | Sample98 predicted / actual span | mspub predicted / actual span | Result |
| --- | --- | --- | --- | --- |
| OBSERVATION | Original `H+22+n*(4+w)` with its literal u16 n at T and u16 w at T+4 | 90 / 294 | 90 / 236 | Refuted in both; under by 204 and 146. |
| OBSERVATION | Generalized `H+22+4*n+sum(payload widths)` from the offset table | 294 / 294 | 236 / 236 | Both agree. |
| OBSERVATION | Class-3 `H+18+q`, q=u32 at T+8 | 294 / 294 | 236 / 236 | Both agree with independently read q and offsets. |

OBSERVATION: the original field layout mistakes the second count at T+4
for width 2 and reads its repeated-count candidate at T+2 as zero. Neither
2 nor descriptor word 2's zero is a measured payload width. Fixed overhead
22 and numeric n=2 still work in the generalized sum; the fixed-width field
interpretation fails. Sample98's equal 104-byte spans permit a constant-width
arithmetic fit if 104 is taken from the offsets, but no stored common-width
field is identified. mspub's measured 100/50 split refutes equal widths for
its two offset-framed payloads. Choosing 75 from total/2 would ignore that
stored split, not decode a width field.

OBSERVATION: the generalized sum and class-3 rule cannot be separated by
these records: `sum(widths)=q-4*(n+1)`, so they are algebraically identical
after the offset-table checks. The directly stored-q form is sufficient for
predicting the endpoint; the table additionally proves the observed count
and payload framing. No new alternative equation is required.

INFERENCE: class 195 shares the tested relative-offset framing of class 3
in these two examples. UNKNOWN: that does not establish a bit-mask rule,
inheritance, semantic relationship between class labels, or behavior beyond
the supplied records. The equal first/second counts still do not distinguish
used count, capacity or authoritative copy, and all upper count bytes are
zero, leaving narrower storage alternatives unseparated.

### All 93 extents and per-class length rules

OBSERVATION: a fresh bounded pass rechecks every class-selected prediction
from the stream fields, not just the two new records: **54/54 Sample98 and
39/39 mspub, 93/93 total**, with zero disagreements and zero unexplained
sampled extents. No extra constant-only provisional fits are counted. H is
the byte at R+3 and equals descriptor word 0's high byte in all 93. Define
T=R+H. Each row below is OBSERVATION over the complete matching class census;
class numbers are numeric labels, not semantic names.

| Basis | Descriptor class | Field-driven span rule | Field locations / framing | Sample98 / mspub / total |
| --- | --- | --- | --- | --- |
| OBSERVATION | 0 | H | No residual tail in these instances. | 8 / 7 / 15 |
| OBSERVATION | 1 | `H+10+n*w` | u16 n at T, equal repeat at T+2, u16 w at T+4. | 11 / 7 / 18 |
| OBSERVATION | 2 | `H+22+n*(4+w)` | u16 n/repeat/w at T/T+2/T+4; n four-byte slots and n w-byte blocks. | 3 / 4 / 7 |
| OBSERVATION | 3 | `H+18+q` | u32 n/repeat/q at T/T+4/T+8; n+1 offsets relative to T+18. | 5 / 7 / 12 |
| OBSERVATION | 4 | `H+8+q` | u32 q at T; other header bytes opaque. | 9 / 2 / 11 |
| OBSERVATION | 9 | `H+10+n*w` | Same A rule as class 1; instance w checked, not universally descriptor stride. | 7 / 2 / 9 |
| OBSERVATION | 10 | `H+22+n*(4+w)` | Same tested fields/regions as class 2. | 1 / 1 / 2 |
| OBSERVATION | 25 | `H+10+n*w` | Same A rule as class 1. | 9 / 8 / 17 |
| OBSERVATION | 195 | `H+18+q` | Same tested count/length/relative-offset checks as class 3. | 1 / 1 / 2 |

OBSERVATION: there are five tested rule families, selected by descriptor
class. The generalized counted-regions sum covers the 23 class-2/3/10/195
records with class-specific field layouts; the other 70 retain F/A/O rules.
For example, class 0 ends at H, not at H+22, and class 4 reads q at T rather
than T+8. No single tested length-field layout or fixed-width formula covers
all 93. Rewriting everything as H plus an unspecified tail would not be an
independent length prediction. Every listed class still needs its applicable
field rule; no record remains without one in these two files.

UNKNOWN: exact endpoint agreement does not establish internal padding inside
opaque spans, semantic contents, a universal malformed-input reader or wider
release coverage. Nonempty index-49 behavior, duplicate counts, descriptor
word/selector meanings, directory flag bits, unresolved header fields and
exact saving releases remain unverified. The shared nine-byte suffix is not
discarded as padding. No geometry, text, colour or formatting was decoded.

INFERENCE: the next smallest available step is the sole descriptor-41
selector at record-relative +16: check bounded u16 selections against each
file's directory IDs, without assigning content/reference meaning or assuming
u16 is the semantic width. That tests one small unresolved descriptor rule
after the sampled extent census closes. No selected value was interpreted
as a reference in AP9.

OBSERVATION: this is the first AP9 checkpoint. Original identities, all prior
bounded comparison checks, fresh descriptor/directory walks, complete fixed
and tail equality masks, both new offset tables and all 93 field-driven
endpoints pass. Only this appended section and the narrowed ranked-unknowns
section change. No full CI, importer/protected-reader edit, B work, Quill
repair, environment script/setting change, environment save/publication or
pull request was performed. Sample and stream bytes remain uncommitted;
no extra remote was added.

## Reference selectors and the record graph

OBSERVATION: AP10 starts from `c0b317908a37f7c0602ae0824abc66e57ee9be0d`,
titled Complete the sampled legacy record extent census. Fetch succeeds and
the named branch is at that commit. The restore verifies 317 pinned inputs,
restores none and reports zero failures; binary folder counts are 142 and
24. Both originals match their recorded identities: Sample98.pub is 61440
bytes; mspub.pub is 19968 bytes with SHA-256
`0EFF0498172805BF1E52C74A6334ECC4CDC84FB2E531C0EA25734CFAC7923C7D`.
Fresh Contents extractions and derived checks remain ignored. No private
bytes or content are reproduced. SOURCES.md already lists both inputs and
is unchanged; the attachment remains available, so no fallback is used.

OBSERVATION: scope is every selector of the five descriptors with nonempty
selector lists already identified in this note: 10,20,21,41,49. This is
41 descriptor/offset pairs, including all 32 selectors of descriptor 21.
The existing descriptor walk and every directory entry are checked afresh.
Other descriptors' selector roles and variable-tail values other than the
previously proven index-20 ID lists are not expanded into this graph.
All selected reads use bounded little-endian u16 probes at R+selector;
the upper/lower representation of a genuine reference remains UNKNOWN.

### Descriptor 41: one ascending child-ID candidate, not a sequence

OBSERVATION: descriptor 41 at `0x1D8` selects +16 through its word at
`0x1DE`. Exactly one directory record uses it in each sample. Positions
and IDs in the table are hexadecimal; sizes and selector offsets are decimal.

| Basis | Sample | Source ID / directory entry / R | Selected position / value | Independent directory check |
| --- | --- | --- | --- | --- |
| OBSERVATION | Sample98 | `0x11F / 0x3BA / 0x6AA` | `0x6BA / 0x120` | ID `0x120` exists at directory entry `0x3C4`; its parent candidate is `0x11F`. |
| OBSERVATION | mspub | `0x111 / 0x3BA / 0x608` | `0x618 / 0` | Zero is not an ID in this file; no target is assigned. |

OBSERVATION: the one nonzero value resolves to a direct child according to
the independent directory +2 field. Its ID is source ID plus one, and its
directory entry follows the source's entry. It is neither the source's
parent nor a sibling under the directory relation. There is only one
nonzero observation, so no ascending-list or general ordering rule is tested.
The zero sample cannot prove a null encoding or absence of all references.

INFERENCE: descriptor 41's selector is a probable reference. ID membership
plus one independently matching child-parent pair supports that reading;
it does not establish universal semantics, width, relocation or the role
of the zero value. It is excluded from the proven-only graph below.

### False-positive controls and classification standard

OBSERVATION: the 16-bit uniform-field baseline for a single random probe is
54/65536 = 0.08240% in Sample98 and 39/65536 = 0.05951% in mspub. For
m probes, expected ID hits are m times that density; no independence is
needed for that expectation. This model assumes uniformly distributed field
values, not actual publication fields. Conditional on already falling in
the ID interval, density is much higher: 54/66 = 81.82% over `0x100..0x141`
and 39/40 = 97.5% over `0x100..0x127`. A small number in that interval
can match accidentally. No p-value or semantic proof is derived from density.

OBSERVATION: a matched same-width background control enumerates every
even-offset u16 in each scoped record's fixed part, from +4 through H-2,
excluding that descriptor's selectors and the leading prefix. Uniformly
choosing one member of that pool would have the empirical hit rate shown
below. The complete pool is used instead of random sampling, avoiding seed
dependence. These controls may include unknown references; they are a
background comparison, not a certified set of non-reference fields. No
control values or payload meanings are reproduced.

| Basis | Descriptor / selector instances per file | Sample98 control ID hits / fields | mspub control ID hits / fields | Uniform expected hits per fully bounded selector, Sample98 / mspub |
| --- | --- | --- | --- | --- |
| OBSERVATION | 10 / 2,1 | 0 / 106 | 0 / 53 | 0.001648 / 0.000595 |
| OBSERVATION | 20 / 8,6 | 2 / 72 | 2 / 54 | 0.006592 / 0.003571 |
| OBSERVATION | 21 / 1,1 | 0 / 56 | 0 / 56 | 0.000824 / 0.000595 |
| OBSERVATION | 41 / 1,1 | 0 / 25 | 0 / 25 | 0.000824 / 0.000595 |
| OBSERVATION | 49 / 2,3 | 0 / 2 | 0 / 3 | 0.001648 / 0.001785 |

OBSERVATION: all four control ID hits fail the direct-child parent match:
zero of 452 control probes has that corroboration. Across actual selectors,
there are 72 bounded probes with 27 ID hits in Sample98 and 62 with 19 hits
in mspub. All other bounded values are zero; every nonzero selected value
resolves to an ID whose directory parent candidate equals its source ID.
Neither file has a nonzero non-ID selected value in this scoped census.
The aggregate uniform baseline predicts approximately 0.0593/0.0369 hits,
but structure and dependence prevent treating that as a significance test.

INFERENCE: classification is deliberately sample-bounded. P means proven
reference correspondence in these samples: at least two nonzero selected
values, every nonzero value an existing ID, and every such target independently
naming the source as parent candidate. R means probable: the same checks
but only one nonzero observation. Repetition can be within one file; it is
not necessarily independent manufacture. These criteria do not prove writer
intent or validity on unseen files. The threshold is a conservative evidence
policy, not a statistical theorem or a decoded format field.

UNKNOWN: zero-only selectors cannot honestly be proved not to be optional
references. N below means **not an observed reference in these instances**,
not a universal negative. The out-of-bounds selector is not a readable
literal field; its intended mapping remains unknown. A stronger three-way
claim that every zero-only field is definitely non-reference is unsupported.

### Complete selector table

OBSERVATION: each grouped row applies separately to every listed selector;
the lists partition all 41 descriptor/offset pairs without omissions.
Offsets are decimal. Hit/probe counts include zeros in the denominator;
all hits have independent direct-child corroboration. The matching control
pool and uniform expectation for each row are in the descriptor table above.

| Basis | Descriptor | Selectors, each tested separately | Sample98 ID hits / bounded probes, each | mspub ID hits / bounded probes, each | Classification / limit |
| --- | --- | --- | --- | --- | --- |
| OBSERVATION | 10 | 106,108,110 | 1 / 2 | 0 / 1 | R; one nonzero per selector, remaining values zero. |
| OBSERVATION | 20 | 4 | 1 / 8 | 1 / 6 | P; repeated corroboration across files. |
| OBSERVATION | 20 | 28 | 6 / 8 | 0 / 6 | P; six corroborated nonzero instances, all in Sample98. |
| OBSERVATION | 20 | 6,26 | 0 / 8 | 0 / 6 | N; zero-only, optional role UNKNOWN. |
| OBSERVATION | 21 | 4,6,8,64,72,74,76,88,94,104,106,110 | 1 / 1 | 1 / 1 | P; repeated corroboration across files. |
| OBSERVATION | 21 | 52,70,86 | 0 / 1 | 1 / 1 | R; one nonzero per selector. |
| OBSERVATION | 21 | 96,138 | 1 / 1 | 0 / 1 | R; one nonzero per selector. |
| OBSERVATION | 21 | 66,68,78,80,82,84,90,92,98,108,132,134,136,176 | 0 / 1 | 0 / 1 | N; zero-only, optional role UNKNOWN. |
| OBSERVATION | 21 | 208 | 0 / 0; one rejected probe | 0 / 0; one rejected probe | N; outside both record spans, mapping UNKNOWN. |
| OBSERVATION | 41 | 16 | 1 / 1 | 0 / 1 | R; detailed evidence above. |
| OBSERVATION | 49 | 6 | 2 / 2 | 3 / 3 | P; five corroborated nonzero instances. |

OBSERVATION: the totals are **15 P, nine R, sixteen zero-only N and one
out-of-bounds N**. Descriptor 21's +208 fails before any ID read: its
record spans are 202/200, while a u16 at +208 would require at least 210
bytes. The attempted absolute positions would be `0x6B0` and `0x610`,
beyond E=`0x6AA` and `0x608`; they are not read as selected fields. Counting
the next record's bytes would invent evidence and could create spurious
references. No adjustment or relocation rule is inferred in this step.

### Proven-only graph, with scope kept explicit

OBSERVATION: vertices are all 54/39 directory IDs, including isolated ones.
An edge points from the record containing a proven selected field to the
target ID; duplicates would be collapsed, zeros are excluded, and R/N
selectors contribute no edges. The selector-only layer has 21/16 distinct
edges. The expanded layer adds only AP2/AP3's previously proven index-20
tail IDs: 17/11 edges, with no duplicate selector edges. Both layers are
checked independently for indegrees, cycles and endpoint membership.

| Basis | Sample / proven-only layer | Vertices / edges | Indegree-zero vertices / isolated vertices | Cycles / dangling included IDs | Shape |
| --- | --- | --- | --- | --- | --- |
| OBSERVATION | Sample98 / selectors only | 54 / 21 | 33 / 26 | 0 / 0 | Forest, 33 components. |
| OBSERVATION | mspub / selectors only | 39 / 16 | 23 / 20 | 0 / 0 | Forest, 23 components. |
| OBSERVATION | Sample98 / selectors plus prior proven tails | 54 / 38 | 16 / 9 | 0 / 0 | Forest, 16 components. |
| OBSERVATION | mspub / selectors plus prior proven tails | 39 / 27 | 12 / 9 | 0 / 0 | Forest, 12 components. |

OBSERVATION: every included vertex has indegree at most one, every component
is acyclic, and there are no self-edges or dangling targets. These forests
are DAGs, but neither is one connected tree. All 38/27 expanded edges go
from the directory parent candidate to its child, the inverse orientation
of reading a child record's +2 parent field. That agreement was part of the
reference-correspondence criterion, not an independent discovery of parent
semantics. Graph indegree-zero vertices are roots only of this partial graph,
not established publication roots. Missing probable/untested edges can change
connectivity, root count or cycles in the full unknown reference graph.

UNKNOWN: no complete record graph, general reference width, null encoding,
ownership semantics or universal selector mapping is proven. A single-hit
candidate is not included merely because a uniform-random match is rare.
Zero-only fields could be optional references; descriptor 21's +208 could
require a mapping not yet established. Other descriptor selectors and other
tail relationships remain outside this checkpoint. No geometry, text, colour
or formatting meaning is decoded, and the 93/93 extent census remains unchanged.

INFERENCE: the next smallest step is to investigate the bounded mapping of
descriptor 21's +208 selector. Test schema/instance position relationships
against the existing two record boundaries without reading into neighboring
records or assuming any candidate relocation is valid.

OBSERVATION: this is the first AP10 checkpoint. Both original identities,
all earlier bounded checks, 93 extent predictions, all 41 scoped selector
families, 134 bounded probes, two rejected probes, all 452 control fields,
the prior 28 tail references and both graph layers pass their stated checks.
Only this appended section and the narrowed ranked-unknowns section change.
No full CI, importer/protected-reader edit, B work, Quill repair, environment
script/setting change, environment save/publication or pull request was
performed. Sample and extracted bytes remain uncommitted; no extra remote
was added.

## Descriptor 21 offset +208 and the selector base

OBSERVATION: AP11 starts from `ca435a96877b186a0401db339471c527e72ec137`,
titled Check reference selectors against record directories. Fetch succeeds
and the branch is at that commit. The restore verifies all 317 pinned inputs,
restores none and reports zero failures; binary folders contain 142 controlled
and 24 Apache publications. Sample98.pub matches its recorded 61440-byte
size/hash. The attachment remains available and matches 19968 bytes and
SHA-256 `0EFF0498172805BF1E52C74A6334ECC4CDC84FB2E531C0EA25734CFAC7923C7D`.
Fresh Contents extractions remain ignored. Evidence consists of derived
structural facts, with no private bytes or content reproduced. SOURCES.md
already lists both samples and is unchanged. No fallback is needed.

### Every descriptor-21 instance and selector

OBSERVATION: each file has exactly one directory-addressed descriptor-21
record. Descriptor 21 at Contents `0xE6` has words `0xB2D2`, 25, 2. Its
32 nonzero selectors, in serialized order and decimal, are
`4,6,8,52,64,66,68,70,72,74,76,78,80,82,84,86,88,90,92,94,96,98,104,136,108,132,110,134,106,138,176,208`.
The final selector word is at `0x12A`, followed by a zero terminator at
`0x12C` and descriptor 22 at `0x12E`. Both descriptor lists are identical.
No selector is an inferred extra entry after the terminator.

OBSERVATION: positions and IDs below are hexadecimal; counts, widths and
sizes are decimal. Numeric reads are little-endian, ranges end-exclusive.
H is the high prefix byte; L is its low companion, not a proven fixed length.

| Basis | Field/check | Sample98.pub | mspub.pub |
| --- | --- | --- | --- |
| OBSERVATION | ID / directory D / R / E | `0x100 / 0x3B0 / 0x5E0 / 0x6AA` | `0x127 / 0x3B0 / 0x540 / 0x608` |
| OBSERVATION | L / H at R+2,R+3 | `0x5E2,0x5E3`: 210 / 178 | `0x542,0x543`: 210 / 178 |
| OBSERVATION | Tail start T=R+H | `0x692` | `0x5F2` |
| OBSERVATION | u16 n / repeat / w at T,T+2,T+4 | `0x692,0x694,0x696`: 7 / 7 / 2 | `0x5F2,0x5F4,0x5F6`: 6 / 6 / 2 |
| OBSERVATION | Proven span H+10+n*w | `178+10+7*2=202` | `178+10+6*2=200` |
| OBSERVATION | Directory attribute at D+8 | `0x3B8`: `0x16C0` | `0x3B8`: `0x16C0` |
| OBSERVATION | R+208, not dereferenced | `0x6B0`, beyond E by 6 | `0x610`, beyond E by 8 |
| OBSERVATION | Literal selector +176 | `0x690`, bounded u16 is zero | `0x5F0`, bounded u16 is zero |
| OBSERVATION | Selectors outside record for u16 | 208 only | 208 only |

OBSERVATION: +208 is outside every available descriptor-21 record, not just
one shorter example; no longer counterexample exists in these two files.
A one-, two- or four-byte read would require spans at least 209,210,212,
respectively, so changing width alone cannot rescue either record. All other
31 literal selectors fit inside the fixed part `[R,R+178)`, including +176.
The record ends are independently field-predicted and equal the next directory
offsets; no neighbor bytes are read to manufacture a +208 value.

### Bounds across every descriptor, not only AP10's five

OBSERVATION: walking all 88 descriptors finds 17 nonempty selector lists.
Across all 93 directory records there are 86/84 instantiated selector
occurrences in Sample98/mspub, 170 total. Exactly six fail a literal
record-relative u16 bounds check, listed completely below. The remaining
164 are bounded, without asserting their semantic reference roles.
Uninstantiated descriptors cannot supply field-value evidence.

| Basis | Sample / ID | Descriptor / selector | Directory D / R / E | Span / H | Bounds result |
| --- | --- | --- | --- | --- | --- |
| OBSERVATION | Sample98 / `0x100` | 21 / 208 | `0x3B0 / 0x5E0 / 0x6AA` | 202 / 178 | Requires 210 bytes; rejected. |
| OBSERVATION | Sample98 / `0x133` | 2 / 370 | `0x3E2 / 0x93E / 0x9D4` | 150 / 140 | Requires 372 bytes; rejected. |
| OBSERVATION | Sample98 / `0x134` | 3 / 370 | `0x3EC / 0x9D4 / 0xA6A` | 150 / 140 | Requires 372 bytes; rejected. |
| OBSERVATION | mspub / `0x127` | 21 / 208 | `0x3B0 / 0x540 / 0x608` | 200 / 178 | Requires 210 bytes; rejected. |
| OBSERVATION | mspub / `0x10C` | 2 / 370 | `0x3E2 / 0x866 / 0x8FC` | 150 / 140 | Requires 372 bytes; rejected. |
| OBSERVATION | mspub / `0x10B` | 3 / 370 | `0x3EC / 0x8FC / 0x992` | 150 / 140 | Requires 372 bytes; rejected. |

OBSERVATION: +208 is therefore not the only out-of-bounds selector across
all instantiated descriptors. AP10's narrower five-descriptor scope had one
failing family; the full bounds census adds descriptors 2 and 3's +370.
Their bodies and selector meanings are not decoded in this step.

### The three explanations and bounded countertests

| Basis | Explanation/test | Evidence and result |
| --- | --- | --- |
| OBSERVATION | (a) Add selector to fixed-part end T=R+H | +208 would require relative end 388, versus spans 202/200. Refuted as a rescue for +208. |
| OBSERVATION | (a) Add selector to A-tail data start T+10 | +208 would require relative end 398. Refuted as a rescue for +208. |
| OBSERVATION | Width or units alone | Widths 1/2/4 still overrun; multiplying 208 by two increases the overrun. These interpretations do not rescue +208. |
| OBSERVATION | Shift every selector by -(L-H)=-32 | +208 maps to existing +176, which is zero in both files; selectors +4,+6,+8 would map before R. Refuted as one uniform record-relative relocation. |
| UNKNOWN | Apply -32 only to the last selector | Numerically bounded but duplicates the existing zero-only +176 selection. No independent positive target distinguishes it from an arbitrary adjustment. Not proven or adopted. |
| UNKNOWN | (b) Conditional live selector in a longer record | For the observed H=178,w=2 equation, n>=11 would make a u16 +208 bounded. Current n=7/6 do not test that condition; no live-field meaning follows merely from length. |
| UNKNOWN | (b) Flag-dependent applicability | Both directory attributes are `0x16C0`; both L/H pairs and descriptors match. No discriminating flag change or proven gating bit is available. |
| OBSERVATION | (c) Ordinary zero padding or a word after the selector terminator | Refuted: 208 is a nonzero word before the verified terminator, not zero padding or an extra post-terminator entry. |
| UNKNOWN | (c) Stale, unused or broader-schema selector | Compatible with its being unused in both instances. L=210 equals 208+2, while H=178; that numeric fit could suggest a larger layout but does not establish what L means. No example distinguishes stale metadata from a conditional selector or another mapping. |

OBSERVATION: every positive additive base located at or after R makes the
already invalid +208 position no smaller; it cannot explain this overrun.
The tested negative shift fails a shared-base check. UNKNOWN: an encoded
selector, a per-field relocation, a genuine conditional field and a stale
entry are not exhaustively excluded. The cause is therefore narrowed, not
uniquely decoded. No flag/count gating rule is invented from the two short
records, and no width or base change is silently applied to preserve a theory.

### Shared convention tested on all prior P/R families

OBSERVATION: all 15 sample-proven and nine probable selector families from
AP10 are tested, 24 pairs with 78 instantiated probes. Their 46 previously
corroborated nonzero target IDs are the identity anchors. The table below
counts bounded probes, direct-child ID hits, and preservation of those exact
prior targets; accidental hits on a different child do not preserve an anchor.
T+10 is specifically the known A-tail data base where applicable, and only
an arithmetic comparison base in the other families, not a decoded tail rule.

| Basis | u16 candidate base | Bounded / 78 probes | Direct-child ID hits | Exact prior targets retained / 46 |
| --- | --- | --- | --- | --- |
| OBSERVATION | R, the record start | 78 | 46 | 46 |
| OBSERVATION | T=R+H | 29 | 2 | 0 |
| OBSERVATION | T+10 | 16 | 9 | 0 |
| OBSERVATION | R-32 | 37 | 8 | 0 |

OBSERVATION: alternate bases sometimes land on already framed tail IDs,
which explains why simple membership hits alone do not establish a base.
None retains any of the 46 original nonzero targets. With base R unchanged,
one-, two- and four-byte unsigned whole-field reads are all bounded on the
78 P/R probes, but yield 0,46,23 directory child-ID hits respectively.
One byte cannot express these IDs, all at least `0x100`; four-byte whole-ID
interpretation loses half the anchored hits. This rejects a universal full-u32
ID interpretation, not a possible wider container whose low 16 bits encode
an ID alongside other data. Descriptor 49 alone cannot separate widths two
and four because its following bytes are zero in these examples.

OBSERVATION: the per-family convention supported by the identity anchors is
below. Offsets and probe counts are decimal; each listed reference selection
is also bounded within its observed fixed prefix. This restates a sample
extraction convention, not a universal writer specification.

| Basis | Descriptor family | Supported base / width extraction | P/R probes / corroborated targets | Limit |
| --- | --- | --- | --- | --- |
| OBSERVATION | 10 | u16 at R+s | 9 / 3 | All three selector families remain probable; no tail-relative replacement retains their targets. |
| OBSERVATION | 20 | u16 at R+s | 28 / 8 | Applies to sample-proven +4,+28; zero-only +6,+26 remain UNKNOWN. |
| OBSERVATION | 21 | u16 at R+s for bounded P/R selectors | 34 / 29 | +208 is not covered; zero-only fields do not prove a mapping. |
| OBSERVATION | 41 | u16 at R+16 | 2 / 1 | Probable, not promoted by this base test. |
| OBSERVATION | 49 | u16 at R+6 | 5 / 5 | Low-u16 identity extraction works; wider storage remains possible. |
| UNKNOWN | 2,3 literal +370 | No live-field base established for this selection | Four out-of-bounds instances | Bounds-only census here; reference interpretation untested. |

INFERENCE: use record-relative bounded u16 extraction for the existing
sample anchors, while retaining explicit exceptions. There is no evidence
for changing their shared base to a tail base or shifting every field by 32.
UNKNOWN: +208's invalidity does not prove those other selectors use a different
base, and their successful extraction does not prove every listed selector
is live in every record. AP10's classifications and graph are unchanged;
all 93 extent predictions are also rechecked unchanged.

### Checkpoint, remaining unknowns and next smallest step

UNKNOWN: the requested unique explanation for +208 cannot be verified with
these two available instances. Conditional long-record use, stale/unused
metadata and a selector-specific mapping remain compatible with the bytes.
No general width/relocation rule, flag bit, optional-null meaning or full
reference graph is proven. Exact product releases, broader sample behavior,
duplicate count roles and internal padding remain unknown. No geometry,
text, colour or formatting meaning is decoded.

INFERENCE: the next smallest available byte-only check is the four +370
instances of descriptors 2/3. Compare literal +370 with the bounded candidate
+114 (370 modulo 256), using ID resolution, independent directory-parent
checks and matched controls before accepting a tagged-offset hypothesis.
No such target is inspected or hypothesis accepted in AP11. Distinguishing
conditional +208 use directly would instead need a discriminating independent
record, such as the same layout with n>=11; mere bounds success in that record
would still not prove that a writer uses the selector.

OBSERVATION: this is the first AP11 checkpoint. Both original identities,
all prior bounded checks, 93 extents, all 170 selector bounds, both descriptor-21
tail equations, 24 P/R base/width comparisons and exact target retention
checks pass. Only this appended section and the narrowed ranked-unknowns
section change. SOURCES.md is unchanged. No full CI, importer/protected-reader
edit, B work, Quill repair, environment script/setting change, environment
save/publication or pull request was performed. Sample and extracted bytes
remain uncommitted; no extra remote was added.

## Text records, page candidates and order

OBSERVATION: AP12 starts from `14d879fbf15d748a105dd2495c8ec72568e13270`,
titled Bound selector offsets and test their reference bases. Fetch succeeds
and the branch is at that commit. The restore verifies all 317 pinned inputs,
restores none and reports zero failures. Both originals match their recorded
identities: Sample98.pub is 61440 bytes; the attachment is 19968 bytes with
SHA-256 `0EFF0498172805BF1E52C74A6334ECC4CDC84FB2E531C0EA25734CFAC7923C7D`.
Fresh root Contents and Quill CONTENTS extracts remain ignored. Only the
needed SummaryInformation header/property prefixes (112/264 bytes) are
retained from its 44640/8232-byte streams. Only derived facts are committed; no private bytes or content
are reproduced. Both evidence files are already listed in SOURCES.md, which
is unchanged. The attachment remains available; no fallback is used.

OBSERVATION: the task parks selectors +208/+370 as UNKNOWN; no attempt is
made to map or dereference them. The investigation uses bounded directory,
extent, reference and Quill section framing. It does not run a complete
import, render a page or repair the kind-0x19 failure. Stored text is evidence
of content bytes, not independently verified visible output. No geometry,
colour, formatting or picture content is decoded.

### Quill sections and lengths, including the negative sample

OBSERVATION: the section directory is rewalked with bounded entries and a
seen-set for its next-block chain, following the existing text-store notes.
Only TEXT, SYID and STRS supply text/range evidence. Other section bodies
are not decoded. Positions below are hexadecimal relative to their named
stream; lengths and counts are decimal.

| Basis | Quill fact | Sample98.pub | mspub.pub |
| --- | --- | --- | --- |
| OBSERVATION | Quill stream length | 6144 | 1024 |
| OBSERVATION | TEXT section | `0x200`, 976 bytes = 488 UTF-16 units | Absent from the directory. |
| OBSERVATION | SYID section / count field | `0xC00`, 32 bytes; u32 at `0xC04` is 6 | `0x25A`, 8 bytes; u32 at `0x25E` is 0. |
| OBSERVATION | SYID list | Six u32 IDs at `0xC08`: 1,3,4,6,7,8 | Empty; no story IDs to resolve. |
| OBSERVATION | STRS section / count | `0x11FE`, 82 bytes; count 6 | Absent from the directory. |
| OBSERVATION | STRS length list | At `0x120A`: 86,98,62,80,87,75 | No length list. |
| OBSERVATION | Text-length check | Lengths sum to 488; each section-bounded slice ends with CR. | No TEXT to partition. |

OBSERVATION: Sample98's six slices have respectively 2,3,3,6,4,4 CR units,
including the final CR. They are multi-paragraph stored ranges, not six
single-line strings. Lengths are UTF-16 units including control units, not
glyph counts or a measurement of visible frame capacity. Neither clamping
nor scanning beyond TEXT is used: the six lengths consume exactly its 976
bytes. mspub cannot provide a positive replication of the text linkage;
no text is inferred from its structural record types alone.

### Contents-to-Quill fields and controls

OBSERVATION: the table gives every positive association in Sample98. Probe
width is u16; Quill IDs are u32 list entries. R is a root Contents record
start; Quill range columns are UTF-16 units relative to TEXT. SYID and STRS
field positions are relative to the separate Quill stream. The range start
is the sum of earlier STRS lengths, not a decoded neighboring Contents field.

| Basis | Contents ID / index / numeric class | R / relative probe / absolute probe | Quill ID / SYID field | STRS length field / units | Derived start / end |
| --- | --- | --- | --- | --- | --- |
| OBSERVATION | `0x125` / 8 / 9 | `0x1112 / +0x58 / 0x116A` | 1 / `0xC08` | `0x120A` / 86 | 0 / 86 |
| OBSERVATION | `0x126` / 8 / 9 | `0x1192 / +0x58 / 0x11EA` | 3 / `0xC0C` | `0x120E` / 98 | 86 / 184 |
| OBSERVATION | `0x12A` / 8 / 9 | `0x125A / +0x58 / 0x12B2` | 4 / `0xC10` | `0x1212` / 62 | 184 / 246 |
| OBSERVATION | `0x12B` / 10 / 1 | `0x12DA / +0x66 / 0x1340` | 6 / `0xC14` | `0x1216` / 80 | 246 / 326 |
| OBSERVATION | `0x12F` / 8 / 9 | `0x1570 / +0x58 / 0x15C8` | 7 / `0xC18` | `0x121A` / 87 | 326 / 413 |
| OBSERVATION | `0x130` / 8 / 9 | `0x15F0 / +0x58 / 0x1648` | 8 / `0xC1C` | `0x121E` / 75 | 413 / 488 |

OBSERVATION: scanning every directory-addressed index-8/index-10 record gives
these six positive fields and four zero controls across the two files:

| Basis | Sample / Contents ID / index | Probe position | Value / actual Quill resolution |
| --- | --- | --- | --- |
| OBSERVATION | Sample98 / `0x139` / 8 | `0xBB9` | 0; not a SYID ID. |
| OBSERVATION | Sample98 / `0x13B` / 10 | `0xC84` | 0; not a SYID ID. |
| OBSERVATION | mspub / `0x106` / 8 | `0xAE1` | 0; SYID has no IDs. |
| OBSERVATION | mspub / `0x104` / 10 | `0xBAC` | 0; SYID has no IDs. |

OBSERVATION: all ten anchor positions are inside the known fixed prefix.
The upper probe byte and following two bytes are zero, so reading the same
position as a byte, u16 or u32 yields the same small ID in these instances.
The semantic storage width is therefore UNKNOWN. Zero is not a Quill ID
here; a general null convention is not established from these examples.

OBSERVATION: false-positive controls enumerate even-offset u16 fields from
+4 through H-2 in each index-8/index-10 fixed part, excluding the anchor.
This does not assign any meaning to other fields. A cohort control compares
fields at the same relative offset against each positive record's specific
expected story ID, not merely membership in the small ID set.

| Basis | Sample / index | Anchor matches / records | Background matches to this file's SYID set / fields | Specific-ID cohort result |
| --- | --- | --- | --- | --- |
| OBSERVATION | Sample98 / 8 | 5 / 6 | 18 / 336 | Only +88 matches all five distinct positive IDs; +36,+86,+98 each match one of five. |
| OBSERVATION | Sample98 / 10 | 1 / 2 | 6 / 110 | +102 is the only cohort offset matching the single positive ID 6. |
| OBSERVATION | mspub / 8 | 0 / 1 | 0 / 56 | Empty local SYID set; no positive cohort. |
| OBSERVATION | mspub / 10 | 0 / 1 | 0 / 55 | Empty local SYID set; no positive cohort. |

OBSERVATION: ordinary small fields produce many accidental story-ID hits:
24/446 Sample98 control probes match some ID. A uniform 16-bit model would
predict 6/65536 per probe, but that model is plainly not a faithful model
of these structured fields. If Sample98's six IDs are improperly borrowed
for mspub, its control pools have 4/56 and 3/55 hits despite having no TEXT.
Thus generic low-number matches do not establish text. No significance
claim is made from these densities or the post-selected cohort.

INFERENCE: index-8 +0x58 has a strong sample-bounded story-link association:
five distinct positive IDs, the unique common cohort offset, six-way Quill
coverage together with index 10, bounded text partitions and zero controls.
Index-10 +0x66 is a probable story link, supported by one positive and one
same-index zero control in Sample98 and the zero mspub record. It lacks a
second populated independent sample. These are text-linked record candidates;
a numeric record class alone does not prove a visible text frame or distinguish
other text-hosting structures. No importer behavior is inferred or changed.

OBSERVATION: literal range-field tests examine every byte alignment for
unsigned two-/four-byte values in the bounded window anchor-16 through H:
index 8 `[+0x48,+0x76)` and index 10 `[+0x56,+0x74)`.
None of the six records contains its corresponding length or global end
in that window, in either UTF-16 units or bytes; none of the five nonzero
global starts appears either. The first story's start zero has many coincidental
matches and is not discriminating. No nearby start/length pair is proven.
UNKNOWN: zero fields could represent local start zero; encoded, indirect
or more distant ranges are not excluded. For now, ranges must be derived
from SYID/STRS/TEXT rather than guessed from adjacent zeros.

### Reference-backed parent chains

OBSERVATION: AP10's partial selector graph alone did not supply every path
to the document-level root candidate. AP12 additionally reads the already
proven class-25 A-tail framing of the root-like index-21 records and checks
every listed u16 against directory IDs and their independent parent fields.
All seven Sample98 entries and six mspub entries resolve and name the root
as parent. These thirteen verified correspondences add the needed collection
links without filling graph gaps from assumptions about record meanings.

OBSERVATION: Sample98 root `0x100` has ordered tail IDs at `0x69C`:
`0x117,0x107,0x10A,0x127,0x10D,0x110,0x113`, each index 20/class 25.
mspub root `0x127` has ordered tail IDs at `0x5FC`:
`0x11F,0x11E,0x11D,0x11C,0x11B,0x11A`; `0x11B` is index 65/class 25,
the others index 20/class 25. Fixed selector +8 additionally selects
`0x12C` / `0x10E`, respectively, already corroborated in AP10. Those extra
collections are not entries in the root tail list.

| Basis | Sample98 text-linked records / Quill IDs | Child-to-parent chain | Record index / class pattern |
| --- | --- | --- | --- |
| OBSERVATION | `0x125,0x126` / 1,3 | Each record -> `0x10A` -> `0x100` | 8/9 -> 20/25 -> 21/25 |
| OBSERVATION | `0x12A,0x12F,0x130` / 4,7,8 | Each record -> `0x127` -> `0x100` | 8/9 -> 20/25 -> 21/25 |
| OBSERVATION | `0x12B` / 6 | Record -> `0x127` -> `0x100` | 10/1 -> 20/25 -> 21/25 |

OBSERVATION: the six text-to-collection edges are the previously checked
index-20 child-tail correspondences, and collection-to-root edges now have
both root-list and directory-parent corroboration. There are no cycles on
these chains. All six end at root-like `0x100`, whose parent value `0xFF`
is outside the directory. mspub has no text-linked records; its structurally
corresponding zero-link records `0x106,0x104` sit below collection `0x10E`
and root-like `0x127` (outside-parent value `0xFE`). Sample98's zero-link
`0x139,0x13B` likewise sit below `0x12C` and root-like `0x100`.

OBSERVATION: the text-bearing immediate collections share index 20/class 25,
but their text-linked children do not share a single class: one collection
has two index-8/class-9 records, the other three such records plus an
index-10/class-1 record. Empty-link records also use indices 8 and 10.
Thus neither class 9 nor class 1 alone is a sufficient visible-text test,
and class 25 alone cannot distinguish the collection from the root.

### Page hypotheses and independent-evidence limits

OBSERVATION: this table separates observed group counts from actual page
counts. A root-like record here means the unique directory record whose
parent candidate is outside the ID set, not an AP10 partial-graph root.

| Basis | Hypothesis/test | Sample98 evidence | mspub evidence | Supported conclusion |
| --- | --- | --- | --- | --- |
| OBSERVATION | One root-like record equals one page | One root-like record, but text linked through two immediate collections. | One root-like record; no text groups to compare. | Root count is measured; root=page is not established. |
| OBSERVATION | Direct class-25 records identify all pages | Eight such records: seven root-tail entries plus fixed +8 collection. Only two carry linked text. | Seven such records: six root-tail entries plus fixed +8 collection. None has a Quill story. | Class membership is not a proven page discriminator; absent text does not imply absent pages. |
| OBSERVATION | Independent page-count metadata | Bounded SummaryInformation property-directory probes find IDs 1,4,17; the only type-2/3 numeric candidate is 1252 at `0x54`. | Directory contains only property 17; no type-2/3 numeric candidate. | No page-count field is independently decoded by these probes; property meanings are not inferred as page counts. |
| OBSERVATION | Text divides into page-like groups | IDs 1,3 under `0x10A` include stored first-page wording; IDs 4,6,7,8 under `0x127` include second-page wording. | No TEXT section; no comparable wording. | Two text-bearing groups are established, not an independently verified total page count. |

INFERENCE: `0x10A` and `0x127` are probable page-like collection candidates
in Sample98 because independent reference grouping aligns with the already
documented stored first-/second-page wording. UNKNOWN: wording is authored
content, not an authoritative page-count declaration or visible-page oracle.
It could occur on a different page or in non-visible content. Neither total
page count nor the page role of other class-25 collections is proven.
No thumbnail, picture, native render or external reader is used to supply
that missing evidence. SummaryInformation's private strings/picture data
are not decoded or copied. No named page-count property interpretation is
borrowed from another format description.

### Stored order versus page and reading order

OBSERVATION: Quill SYID/STRS defines the stored story sequence
`1,3,4,6,7,8` and cumulative TEXT ranges. The root's ordered reference list
places `0x10A` third and `0x127` fourth, not first and second. Their own
ordered child tails yield story IDs 1,3 and 4,6,7,8 after skipping non-story
child `0x12E`; these orders agree with the Quill storage order and increasing
physical record positions for the six linked records. The root reference
list itself is not numerically sorted, and record/story IDs have gaps.
These are reproducible serialized orders, not page ordinal fields.

OBSERVATION: the previously corroborated fixed selectors point to directory
children rather than proving a sibling chain. Counts give list sizes (7/6
at the root), not a decoded number of visible pages. No page-number index,
next-page pointer, next-text-frame chain or semantic reading-order field is
proven by the bounded checks in this step.

UNKNOWN: physical/list/story order may agree with visual order, but the
samples provide no independent rendering or explicit ordering contract to
prove that. Text mentioning first/second page gives supporting content
context, not proof that list ordinals 3/4 are page numbers 1/2. The one
positive text sample cannot establish behavior for threaded frames, hidden
content, blank pages or altered ordering. No complete page/text-frame reader
is claimed from these associations.

INFERENCE: the next smallest available content-structure check is the
already listed 36-byte Sample98 Quill TCD section at `0xFA6`. Test only
numeric ID/range/order candidates against the six SYID/STRS associations and
matched controls, without decoding formatting or repairing kind 0x19.
No TCD body field is decoded here; mspub has no TCD section for replication.
Independent page-count or visible-order evidence would still be needed for
stronger page claims.

OBSERVATION: this is the first AP12 checkpoint. Original identities, all 93
extent rules, both Quill directories, the six ID/range partitions and final
CR checks, ten anchor probes, four zero controls, 557 background u16 fields,
cohort/range countertests, thirteen root-list parent resolutions and all six
text-record chains pass their stated checks. An initial diagnostic assumed
STRS existed in both files and failed its lookup on mspub; the final bounded
check explicitly asserts its absence and the zero SYID count rather than
skipping that sample. Only this appended section and
the narrowed ranked-unknowns section change. SOURCES.md is unchanged. No full
CI, importer/protected-reader edit, B work, kind-0x19 repair, environment
script/setting change, environment save/publication or pull request was
performed. Sample and stream bytes remain uncommitted; no extra remote was added.

## Quill TCD section and page evidence

OBSERVATION: AP13 resumes from `7c01a6a9e170f4cf7a48127f01dacdeece0199a3`,
titled Trace legacy text links and page candidates. The named-branch fetch
succeeds. The restore verifies 317 pinned inputs, restores none and reports
zero failures. Both originals match their existing sizes/hashes: Sample98.pub
is 61440 bytes; mspub.pub remains present, 19968 bytes, SHA-256
`0EFF0498172805BF1E52C74A6334ECC4CDC84FB2E531C0EA25734CFAC7923C7D`.
Fresh root Contents and Quill CONTENTS extracts remain ignored and match the
prior extractions exactly. No private sample bytes or content are reproduced.
The attachment is available, so no single-sample fallback is needed; all
positive TCD findings below are nevertheless Sample98-only observations.

OBSERVATION: existing Quill and table notes supplied comparison layouts, not
permission to assume older-format semantics. The own newer-format calendar
note at commit `8e01c0bc6c841e7af5a3eecf53a8387cd3ca4a93`, tables.md section
Cell text association (calendar sample), is consulted only as a comparison.
No calendar bytes, new third-party format source or scoring data are used.
Every older-format association below is checked from the older sample's own
section directory, SYID/STRS/TEXT, Contents link and parent chains.

### Location, bounds and actual entry shape

OBSERVATION: offsets below are hexadecimal within the named stream, sizes
and counts decimal, and ranges end-exclusive. Numeric probes are little-endian.
Both Quill directories are freshly walked with bounded 24-byte entries,
cycle detection and section-extent checks. Sample98 has 17 section entries
and exactly one `TCD ` entry; mspub has six and no TCD entry of any size.

| Basis | Field / position | Sample98 result / evidence | mspub control |
| --- | --- | --- | --- |
| OBSERVATION | Quill stream bounds | `[0,0x1800)`, 6144 bytes | `[0,0x400)`, 1024 bytes |
| OBSERVATION | TCD directory entry E | `0x140`; tag at `0x142` | No matching tag in the complete directory. |
| OBSERVATION | u16 directory field E+6 | `0x146`: 3 | No field or empty TCD header is synthesized. |
| OBSERVATION | u32 section offset E+16 | `0x150`: `0xFA6` | Absent. |
| OBSERVATION | u32 section length E+20 | `0x154`: 36 | Absent. |
| OBSERVATION | TCD section extent | `[0xFA6,0xFCA)`; next listed TOKN data begins at `0xFCA` | No TCD extent. |
| OBSERVATION | u32 at section+0 | `0xFA6`: 5, equals boundary count minus one | Not applicable. |
| OBSERVATION | u32 at section+4 | `0xFAA`: 0; role UNKNOWN | Not applicable. |
| OBSERVATION | u32 at section+8 | `0xFAE`: `0xFF00`; role UNKNOWN | Not applicable. |
| OBSERVATION | Boundary array | Six four-byte slots at `[0xFB2,0xFCA)` | No array. |

OBSERVATION: the framing is `12 + 4*(5+1) = 36`, with a twelve-byte header
and six u32 boundary probes, no leftover bytes. There are not six separately
identified story records, and no start/length/ID tuple repeated six times.
Four bytes is the observed boundary-slot stride. UNKNOWN: narrower numerical
storage plus zero padding cannot be separated here because all upper bytes
are zero; the header's count maintenance rule is not tested on a second
populated older sample. No TCD field is decoded as formatting or geometry.

### Directory slot, story link and field-by-field controls

OBSERVATION: SYID's six IDs are `1,3,4,6,7,8` at Quill `0xC08` onward.
Directory value 3 selects the fourth entry at `0xC14`, value 6. STRS slot 3
at `0x1216` gives length 80, and earlier lengths sum to global TEXT start
246. Contents record `0x12B`, index 10/class 1, has the same ID 6 at its
bounded +0x66 field, absolute Contents `0x1340`. Its verified chain is
`0x12B -> 0x127 -> 0x100`. No other anchored text record carries this ID.

| Basis | Candidate match | Positive check | False-positive control / limit |
| --- | --- | --- | --- |
| OBSERVATION | Directory E+6 is a zero-based SYID slot | Slot 3 -> ID 6 -> 80-unit story -> record `0x12B` | Treating 3 directly as an ID also finds an existing ID, but selects story 3 of length 98 and fails TCD range validation. |
| OBSERVATION | TCD-body field is a story ID | None of the three header probes or six boundary probes equals a SYID ID | Zero body ID matches; six boundaries must not be zipped to the six story IDs merely because their counts coincide. |
| OBSERVATION | Boundaries refer to one story's local UTF-16 units | Final 80 equals only story 6's STRS length; every intermediate boundary points at CR within that story | Trying all six stories gives one full match and five failures, including the direct-ID interpretation above. |
| OBSERVATION | Boundaries are global story-range endpoints | No boundary equals any of the six global starts or ends | The header's zero happens to equal the first global start, but supplies no successful story/range association. |
| OBSERVATION | Units are bytes rather than UTF-16 units | Actual story is 160 text bytes / 80 units | Interpreting terminal 80 as bytes gives only 40 units and fails exact extent. |
| OBSERVATION | Position is a separator, not a nearby arbitrary number | Five intermediate positions are CR; terminal position equals story length including final CR | Shifting each intermediate position by +1 loses all five CR matches. |
| OBSERVATION | The selected slot is correct | Slot 3 passes exact length and separators | Adjacent slots 2/4 select lengths 62/87 and fail. |

INFERENCE: the old-file directory field at E+6 is strongly associated with
the story-list slot in this example. The newer-format finding is consistent
with that interpretation but is not evidence that makes it true here.
UNKNOWN: general meaning across older files, a field width beyond these
probes and behavior after story reordering remain unverified with one
positive old-format TCD section. No universal tag-occurrence interpretation
or importer change follows from this match.

### Six bounded text subdivisions, not six frame ranges

OBSERVATION: the six boundary values are 15,25,39,54,66,80. Begin the first
range at zero and each subsequent range one unit after the preceding boundary.
Exclude each intermediate CR; exclude the final CR at local position 79.
All units are read only within the already established story-6 slice.

| Basis | Boundary ordinal / Quill field | Stored local boundary | Assigned local payload range | Global TEXT payload range |
| --- | --- | --- | --- | --- |
| OBSERVATION | 1 / `0xFB2` | 15 | [0,15) | [246,261) |
| OBSERVATION | 2 / `0xFB6` | 25 | [16,25) | [262,271) |
| OBSERVATION | 3 / `0xFBA` | 39 | [26,39) | [272,285) |
| OBSERVATION | 4 / `0xFBE` | 54 | [40,54) | [286,300) |
| OBSERVATION | 5 / `0xFC2` | 66 | [55,66) | [301,312) |
| OBSERVATION | 6 / `0xFC6` | 80 | [67,79) | [313,325) |

OBSERVATION: payload lengths are 15,9,13,14,11,12, totaling 74 units; the
five intermediate and one final CR account for the remaining six units.
The enclosing story remains [246,326), not any of the six full story ranges
from AP12. In this story those are all six CRs, so the boundary partition
also happens to separate its six CR-delimited text pieces. This coincidence
does not establish that every paragraph is a cell in other files.

INFERENCE: the six subdivisions have the same text-boundary shape as the
existing table-text notes and are cell-like text ranges. UNKNOWN: older
Contents cell-list/grid correspondence, rows/columns, merged-cell handling
and cell order are not decoded here. Calling these six boundaries six pages,
six text frames or six validated grid cells would exceed the evidence.
The section is compatible with a table-associated text host under `0x127`,
without establishing that host's complete structural meaning.

### Collection and page hypotheses after the TCD check

OBSERVATION: the TCD match independently reinforces the association of story
6 with the 80-unit text host `0x12B` in collection `0x127`. It supplies no
boundary group for story IDs 1/3 under `0x10A`, or IDs 4/7/8 elsewhere under
`0x127`. There is one TCD section, not two collection-indexed sections or a
six-record story catalogue. Its count-like 5 happens also to equal `0x127`'s
five child-list entries, but its six boundaries validate only one child's
story. Those child entries include a non-story child and four text-linked
records; they cannot be paired one-for-one with the six boundaries.

INFERENCE: `0x10A` and `0x127` remain **probable page-like collections** from
AP12's reference grouping and authored first-/second-page wording. The TCD
finding makes the text structure inside `0x127` firmer; it provides no new
independent evidence that either collection is a page. UNKNOWN: actual page
identity, total count and master/scratch roles remain unproven. No TCD field
is independently identified as a page number; opaque header values are not
assigned page semantics by coincidence.

OBSERVATION: mspub has no TCD or TEXT or STRS section, and its SYID count is
zero. It confirms that no association should be manufactured in this sample;
it does not replicate the older positive TCD mapping or prove that a text-free
publication has zero pages. No private text or TCD bytes are exposed.

### Order and checkpoint limits

OBSERVATION: the boundary values increase within story 6 and preserve the
stored order of its six text subdivisions. They do not order the six anchored
text records or group by their two parent collections. E+6=3 selects the
fourth stored story, not a proven page number. The previously observed child
lists still give IDs 1,3 under `0x10A` and 4,6,7,8 under `0x127`; only the
second ID in the latter list participates in this TCD. There is no TCD
entry sequence that can independently confirm cross-frame child-list order.

UNKNOWN: page order and visual reading order remain unproven. Serialized
story/child order agreeing is not an independent visible-order contract, and
TCD only adds local text-subdivision order. No native render, geometry,
colours, formatting, pictures, variant-B table or kind-0x19 repair is used.
The existing 93/93 extent coverage and reference classifications are unchanged.

INFERENCE: the next smallest discriminating page experiment is an independent
variant-A sample with a documented page count and a known text item per page,
ideally a controlled one-/two-page pair. Compare collection/root/story links
without assigning page roles from names. The current single positive TCD and
text-free negative do not provide another independent page discriminator.

OBSERVATION: this is the first AP13 checkpoint. Both original identities,
fresh stream/directory bounds, prior 93 extent and AP12 checks, all nine TCD
body probes, six story controls, adjacent-slot/direct-ID/unit/shift controls
and all six local/global text ranges pass their stated checks. An initial
scratch extractor did not compile because its buffer was moved before its
length was reported; borrowing the buffer corrected that diagnostic, without
changing a sample or importer. Only this appended section, the narrowed ranked
unknowns and one comparison-source line change. No full CI, importer edit,
environment script/setting change, environment save/publication or pull request
is performed. Sample bytes remain ignored and uncommitted; no extra remote
is added.

## Page size and text frame rectangles

OBSERVATION: AP14 starts from `7868f7bd703648d3349e166190b9b8f4e982726c`.
The named-branch fetch succeeds. The restore verifies 317 pinned inputs,
restores none and reports zero failures. Both originals match their recorded
identities: Sample98.pub is 61440 bytes; mspub.pub remains present, 19968
bytes with SHA-256
`0EFF0498172805BF1E52C74A6334ECC4CDC84FB2E531C0EA25734CFAC7923C7D`.
Fresh Contents extractions match the earlier streams exactly and remain
ignored. No private bytes are reproduced. Evidence is limited to these
files, their prior structural/text links and bounded numeric experiments;
SOURCES.md already lists both inputs and is unchanged. No fallback is needed.

OBSERVATION: this section separates raw field measurements from physical
interpretation. Page sizes, endpoints and containment can be tested for
plausibility without a native rendering, but numerical fit alone cannot
uniquely prove a page-coordinate origin or physical scale. No importer,
variant-B work or kind-0x19 repair is performed. Rotation, object anchors,
margins, columns, fonts, colours and pictures are not decoded.

### Root size candidates and exact standard-size controls

OBSERVATION: positions are hexadecimal unless stated otherwise. Numeric field
values and counts are decimal. All reads are little-endian and ranges are
end-exclusive. Each root-like index-21 record has two positive 32-bit values
at R+0x14 and R+0x18 (decimal +20/+24), inside its H=178 fixed prefix.

| Basis | Sample / root ID / R | Width candidate field / value | Height candidate field / value | Standard-size plausibility at 36000 units/mm |
| --- | --- | --- | --- | --- |
| OBSERVATION | Sample98 / `0x100` / `0x5E0` | `0x5F4` / 7560000 | `0x5F8` / 10692000 | Exactly 210 x 297 mm, A4 portrait. |
| OBSERVATION | mspub / `0x127` / `0x540` | `0x554` / 7772400 | `0x558` / 10058400 | Exactly 215.9 x 279.4 mm, or 8.5 x 11 inches, Letter portrait. |

OBSERVATION: the same dimension pairs recur at index-44 record `0x121`
fields `0x108A,0x108E` in Sample98, and record `0x10F` fields `0xF62,0xF66`
in mspub. These are that record's +4/+8 fields. Prior proven reference
correspondences connect them through `0x107` / `0x11E` to their respective
root: the collection is in the root's ordered tail and its +4 selector
resolves to the size-pair record. Internal duplication corroborates a size
association, but is not an independent physical measurement of a page.

INFERENCE: root +0x14/+0x18 are strong document/page-width and height
candidates at a scale of 36000 units/mm, equivalent to 914400 units/inch
and 12700 units/point. This is an EMU-compatible scale. The same scale gives
two exact, differently proportioned standards in independently supplied files
and round millimetre positions for nearly all text-frame endpoints below.
UNKNOWN: actual author-selected physical sizes, the native unit definition,
page-size inheritance and any per-collection override are not independently
verified; a different physical interpretation cannot be excluded by ratios
and containment alone. The positive values do not separate signed from
unsigned page-size storage semantics.

OBSERVATION: a search control finds other exact Letter-looking pairs in
Sample98 at `0x71A,0x782`, both inside class-195 record `0x11F`, despite
the root's A4-looking pair. In mspub, Letter pairs occur at root `0x554`,
class-195 positions `0x678,0x6DC`, and index-44 position `0xF62`.
Their other meanings are not decoded. Thus locating any standard-looking
pair anywhere would not identify the page size. The consistent root offsets
and reference-backed duplicate distinguish the candidate; there is still no
native page-size oracle in this evidence.

### Endpoint candidates for the two text-record families

OBSERVATION: all six positive text-linked records have a common four-field
window inside their fixed prefixes. The probe layout is the same for
index 8/class 9 and index 10/class 1:

| Basis | Record-relative byte offset | Probe width / candidate meaning |
| --- | --- | --- |
| OBSERVATION | +0x06, decimal +6 | Four-byte signed integer, candidate left endpoint. |
| OBSERVATION | +0x0A, decimal +10 | Four-byte signed integer, candidate top endpoint. |
| OBSERVATION | +0x0E, decimal +14 | Four-byte signed integer, candidate right endpoint. |
| OBSERVATION | +0x12, decimal +18 | Four-byte signed integer, candidate bottom endpoint. |

OBSERVATION: candidate physical placement uses the inferred root W/H and
translation `(W/2,H/2)`: left/right each add W/2 and top/bottom each add H/2.
Before translation all six left values are negative, and several top/bottom
values are negative. The following table records exact signed probes and
exact translated values; translation is a hypothesis being tested, not a
stored top-left coordinate field.

| Basis | Sample98 ID / index / R | Raw candidate left,top,right,bottom | Translated left,top,right,bottom in candidate units |
| --- | --- | --- | --- |
| OBSERVATION | `0x125` / 8 / `0x1112` | -2700000,-4446000,2376000,-3474000 | 1080000,900000,6156000,1872000 |
| OBSERVATION | `0x126` / 8 / `0x1192` | -2700000,-3294000,2484000,-2214000 | 1080000,2052000,6264000,3132000 |
| OBSERVATION | `0x12A` / 8 / `0x125A` | -2952000,-4410000,2412000,-2862000 | 828000,936000,6192000,2484000 |
| OBSERVATION | `0x12B` / 10 / `0x12DA` | -2412000,-2538000,1152000,-1880436 | 1368000,2808000,4932000,3465564 |
| OBSERVATION | `0x12F` / 8 / `0x1570` | -2376000,-90000,2016000,594000 | 1404000,5256000,5796000,5940000 |
| OBSERVATION | `0x130` / 8 / `0x15F0` | -2340000,882000,2196000,2610000 | 1440000,6228000,5976000,7956000 |

OBSERVATION: dividing translated coordinates by 36000 gives millimetre
candidates `(30,25,171,52)`, `(30,57,174,87)`, `(23,26,172,69)`,
`(38,78,137,3465564/36000)`, `(39,146,161,165)` and
`(40,173,166,221)`. The index-10 bottom is about 96.265667 mm; it is not
rounded to a whole millimetre. Every right exceeds left and every bottom
exceeds top; all six boxes lie inside the inferred A4 bounds. The two boxes
under `0x10A` differ, and all four under `0x127` differ; all six are distinct.
No positive text-linked candidate is outside that inferred page.

OBSERVATION: record `0x12B` has a further numeric control: its u32 at +0x5C
is 3564000, exactly candidate right minus left. The next u32 at +0x60 is
648000, whereas candidate bottom minus top is 657564. This is a retained
mismatch, not discarded noise or evidence for rounding. UNKNOWN: the second
quantity's role and the reason for the difference are not decoded. No claim
that it is an alternative frame-height field is adopted.

### Window, width, sign and other-record controls

OBSERVATION: every byte-aligned candidate window of four signed32 integers
from record offsets 0 through 100 is tested on all six positive text records:
101 common windows. Requiring positive dimensions, inferred page containment
and distinct boxes leaves only the window beginning at +6. Four-byte probes
are unaligned relative to the record; alignment padding must not be inserted.

| Basis | Interpretation/control at the candidate positions | Positive/contained result across six text records |
| --- | --- | --- |
| OBSERVATION | Signed 32-bit left/top/right/bottom plus half-page translation | 6/6; only common valid window among 101 tested starts. |
| OBSERVATION | Unsigned whole-32-bit endpoints | 0/6; negative encoded endpoints become large positive values. |
| OBSERVATION | Signed or unsigned 16-bit values at those same four positions | 2/6 for each; neither preserves the common rectangle test. |
| OBSERVATION | Signed or unsigned 64-bit values at those positions | 0/6 for each. |
| OBSERVATION | Raw left/top/right/bottom with origin at page top-left | 0/6 contained; all six left values are negative. |
| OBSERVATION | Third/fourth values are positive width/height instead of endpoints | Only 2/6 even have both positive; not a shared interpretation. |

OBSERVATION: the same offsets exist in Sample98's zero-link index-8 record
`0x139` and index-10 record `0x13B`, and mspub's zero-link index-8 record
`0x106` and index-10 record `0x104`. All four give `(0,0,0,0)`, a degenerate
box before or after translation. They are negative controls, not established
live empty frames. mspub therefore supplies no positive frame-position
replication. A builder must not create a page-center frame from those zeros
or infer visible text merely from index/class membership.

OBSERVATION: the background control tests the same four offsets on all other
record classes, requiring the window to fit the fixed prefix before treating
it as a comparable fixed-field probe. These are not certified non-geometry
records; a passing numeric test alone is not a false-positive-free frame label.

| Basis | Numeric descriptor class, excluding indices 8/10 | Sample98 prefix-eligible / positive-contained hits | mspub prefix-eligible / positive-contained hits |
| --- | --- | --- | --- |
| OBSERVATION | 0 | 7 / 1 | 6 / 0 |
| OBSERVATION | 1 | 4 / 0 | 5 / 0 |
| OBSERVATION | 2 | 0 / 0 | 0 / 0 |
| OBSERVATION | 3 | 1 / 1 | 1 / 1 |
| OBSERVATION | 4 | 2 / 0 | 2 / 0 |
| OBSERVATION | 9 | 0 / 0 | 0 / 0 |
| OBSERVATION | 10 | 0 / 0 | 0 / 0 |
| OBSERVATION | 25 | 9 / 0 | 8 / 0 |
| OBSERVATION | 195 | 1 / 0 | 1 / 0 |

OBSERVATION: totals are 2/24 and 1/23. Passing backgrounds are Sample98
`0x123` (index 40/class 3), `0x12E` (index 5/class 0), and mspub `0x102`
(index 40/class 3). Their meanings are not assigned from this result.
Allowing probes into tails instead increases eligible other-record counts
to 42/34 and hits to 3/1; Sample98 index-78 record `0x141` adds a passing
window outside its fixed prefix. This demonstrates why blind rectangle
scanning or applying a class-1 rule to every class-1 record is unsafe.
Out-of-span reads are never performed, and out-of-bounds selectors stay parked.

### What is still inferred, including the origin control

INFERENCE: a centered, signed32 endpoint representation at the shared offsets
and an EMU-compatible scale is the strongest common candidate tested for the
six known text hosts. The exact standard-size pairs, unique common window,
round positions, positive/nonidentical boxes and width duplication support
it. UNKNOWN: true physical placement, whether these are outer bounds or
another rectangle, and any native coordinate-origin contract remain unverified.

OBSERVATION: containment does not uniquely prove the half-page origin.
All six boxes still fit after adding another 36000 units to both translated
axes. Across the six raw boxes, feasible translations span X from 2952000
to 5076000 and Y from 4446000 to 8082000. Many origins therefore satisfy the
same inequalities. Even substituting the alternative Letter-looking page
pair for Sample98 still contains all six boxes. Those are explicit controls
against claiming that containment proves either the size or origin uniquely.

UNKNOWN: no identified positive text candidate lies outside the inferred
page, but that is not a census of every actual frame in either publication.
Other record classes, non-text frames and the zero-link records do not have
independently established live-frame roles. Their passing/failing numeric
windows must not be reported as real inside/outside frames. In particular,
there is no evidence here to claim that every frame in mspub is inside, outside
or absent. Page roles of `0x10A`/`0x127` and inherited page sizes also remain
probable or unknown as previously documented.

### Bounded builder requirements and checkpoint

INFERENCE: for an experimental placement of one of the six text-host candidates,
a builder needs the following exact data and checks; these are not promoted
to a universally proven format rule without an independent position control.

- INFERENCE: open the bounded Contents stream, use its verified directory and
  extent rules, and resolve the selected index-8 or index-10 record. Require
  `[R,R+0x16)` to fit its fixed prefix and stream before reading four values.
  Do not generalize by numeric class alone or by a random passing window.
- INFERENCE: resolve its already checked Quill story and collection/root chain.
  Read the root-like index-21 width/height candidates at +0x14/+0x18 only
  when `[root,root+0x1C)` fits its fixed prefix. Require positive, bounded
  dimensions and checked conversion arithmetic; do not substitute unrelated
  standard-looking pairs from another record.
- INFERENCE: read signed32 endpoints at +0x06,+0x0A,+0x0E,+0x12. Reject
  nonpositive width/height; treat all-zero inactive controls as unplaced.
  With the proposed origin, use checked wide arithmetic such as
  `L2=2*left+W`, `R2=2*right+W`, `T2=2*top+H`, `B2=2*bottom+H`.
  Require `0<=L2<R2<=2*W` and `0<=T2<B2<=2*H` for the contained case;
  do not clip silently or pretend these checks identify every legal off-page
  object. Divide by two only after preserving any half-unit precision.
- INFERENCE: under the candidate scale, divide by 36000 for millimetres or
  12700 for points. Place only after a page-assignment decision supported
  separately from these geometry candidates. Preserve explicit uncertainty
  about the origin/physical scale until known-position evidence confirms it.
- INFERENCE: attach the verified story range from SYID/STRS/TEXT; do not invent
  a new range from geometry or infer rotation, object anchors, margins, columns,
  fonts, colours or pictures. The kind-0x19 failure is not repaired here.

UNKNOWN: ranked remaining position questions are the absolute origin and
native physical unit, actual page-size inheritance and page roles, a second
positive geometry example, the rectangle's exact role and the index-10
height discrepancy. Wider frame classes and independent visible placement
remain unverified. These limits materially prevent claiming a fully proven
variant-A page renderer from this checkpoint alone.

INFERENCE: the next smallest discriminating experiment is an independently
saved variant-A page with known physical size and one unrotated text frame
at documented noncentral left/top/right/bottom coordinates. It would separate
the half-page candidate from the equally contained shifted-origin controls;
a second all-zero file or merely another plausible rectangle would not.

OBSERVATION: this is the first AP14 checkpoint. Both original identities,
prior structural/text checks, root and referenced size pairs, 101 common
windows, all six positive boxes, four zero controls, all eligible other-class
probes, width/sign/origin controls and the height mismatch pass their stated
checks or are retained as explicit limitations. Only this appended section
and the narrowed ranked-unknowns section change. SOURCES.md is unchanged.
No full CI, importer edit, environment script/setting change, environment
save/publication, variant-B work or pull request is performed. Sample and
extracted bytes remain ignored and uncommitted; no extra remote is added.

## Replication on further variant A files

OBSERVATION: AP15 inventories 170 physical `.pub` files: 142 restored controlled binaries, 24 restored Apache binaries, two owner/user attachments and two existing repository fixtures. The restore verified all 317 pinned inputs (including non-publication scoring assets), skipped 317, restored zero and failed zero. Only publication binaries and existing repository notes supplied this census; scoring assets were not inspected. Every restored publication size/hash agrees with publisher-fixtures.csv.

OBSERVATION: the search covered `/workspace` recursively (excluding `.git` and dependency `node_modules` directories), plus `/tmp/niftydata`. `/workspace/niftydata` resolves to the same restored directory: its 166 aliases account for 336 discovered paths, not 336 independent samples. The table lists each resolved file once; repeated hashes, if any, do not become independent evidence. Controlled and Apache rows resolve under `/tmp/niftydata/pubsmith-fixtures` and `/tmp/niftydata/apache-pub`; repository rows under `crates/pub/tests/data`; attachment rows are the supplied files. UNKNOWN: files outside these searched locations, inaccessible files and unprovided older samples are not inventoried.

OBSERVATION: classification uses only compound-container opening and root Contents checks. A requires the documented old-family header, actual length at +0x08, table pointer +0x1A = 34, table header 89/6, the 88-entry three-word/zero-terminated walk ending exactly at the control pointer, a 46-byte control-to-directory interval, and a bounded ten-byte directory with distinct ascending in-stream record starts. Both known A files satisfy these checks. B is the already documented Sample2000 header tuple (length 6042, control 954, directory 1000, table 34, table count 94); its table/payload are not decoded. Neither means the documented newer-family discriminator is present; it does not assign an exact saving version. UNKNOWN means container validation failed before a usable Contents header could be read; it does not prove neither A nor B.

OBSERVATION: results are two A, one B, 164 neither and three UNKNOWN. No further variant-A file was found. The attachment mspub identity remains exactly 19968 bytes and the required SHA-256. No zmspub.pub is present in the searched locations.

| Basis | Set | File | Bytes | SHA-256 | Classification |
| --- | --- | --- | ---: | --- | --- |
| OBSERVATION | Attachment | mspub.pub | 19968 | `0eff0498172805bf1e52c74a6334ecc4cdc84fb2e531c0ea25734cfac7923c7d` | A |
| OBSERVATION | Attachment | may-aug-2026.pub | 1610752 | `ad730d7aebc2eb1bf93f04c537cc6f31c2b8f76504070a19d44147872c0cbf3a` | Neither |
| OBSERVATION | Apache | 51318.pub | 88576 | `3ab75a6a9196e0a51fc9b0aa759459501c71d313030c06652aabffbae0a2ab09` | Neither |
| OBSERVATION | Apache | 60685.pub | 119296 | `0d3cecf0ff000a20931d0de3ebc48040bea10e03833ade3f9902f996c20e7184` | Neither |
| OBSERVATION | Apache | LinkAt0And10.pub | 64512 | `c59b7160c21a531e5872e76141c4cd43a4d74f30d78401f64175467fa40c44fc` | Neither |
| OBSERVATION | Apache | LinkAt10.pub | 64512 | `f1b7f65cb9646fddc2cfc77a795109bb958f629e4fbf7c9fc7a4d1b045074c35` | Neither |
| OBSERVATION | Apache | LinkAt10And20And30.pub | 64512 | `202ac6f82e2a33ef0181158728b5c4df6781ccb105e012f520d331acafbcebf1` | Neither |
| OBSERVATION | Apache | LinkAt10And20And30And40.pub | 64512 | `b54068d4f82dd0ee533f2a655a7edd2acbdc02619621b32fb23cb9ce842ed669` | Neither |
| OBSERVATION | Apache | LinkAt10Longer.pub | 64512 | `a917a9e575f5cfc1ba8e78226981b662b06f96da2d7f2eb3e2b00a4b4f3dada5` | Neither |
| OBSERVATION | Apache | LinkAt20.pub | 64512 | `cec79a2bd2f6bc71dd276bd6584c34d66fa6f710d9adbed951bd9cb32c5b2fed` | Neither |
| OBSERVATION | Apache | Sample.pub | 72192 | `6fefdef46b87c767150878dc384549cb2d2ec2ac54de25f8ddb3a5628301107e` | Neither |
| OBSERVATION | Apache | Sample2.pub | 72704 | `c87ea7cc5606023c5fa99376631bbd45a8e9bdaba34c958057995b0e7ac8cc86` | Neither |
| OBSERVATION | Apache | Sample2000.pub | 61440 | `40701ca47b26d04771cdd58467764e9ab39d3da69b59fbc529afd16d263d2f86` | B |
| OBSERVATION | Apache | Sample2_2010.pub | 73216 | `931ff20980b8f72aab227fa3cbe61b017be8d17c337fecb9aca3c97075940777` | Neither |
| OBSERVATION | Apache | Sample3.pub | 72192 | `424c69173ff08948c2529c8084b4ac2403f1ff1057146f4edd02fc29b44481fc` | Neither |
| OBSERVATION | Apache | Sample3_2010.pub | 72704 | `baa9555a72d9e4264e3fd722896e2e43b960d20b4afdb96f75cb721a6133ff91` | Neither |
| OBSERVATION | Apache | Sample4.pub | 72192 | `42195f7ad23d911219fea3ec88e66e867e9b9a6821a16dd1b535e2aa9d57a11b` | Neither |
| OBSERVATION | Apache | Sample4_2010.pub | 72704 | `544e2043f91aed664c0a09f9ae45da42291b6730f107f99f3cdf9462ed38fb02` | Neither |
| OBSERVATION | Apache | Sample98.pub | 61440 | `8912f295ff0cd1f55b873b3b98a182218f33c420877e025d2f0fd795f12dd0c0` | A |
| OBSERVATION | Apache | SampleBrochure.pub | 161792 | `ffed034ac87e679f0bd08ff9cf74ad11c0e0e510a42b1bc1a7502415f6c29c87` | Neither |
| OBSERVATION | Apache | SampleNewsletter.pub | 291840 | `6a825ba26ba35d6e885acdc62e859591ed37cb0ff7480b554b9cb362b644dfcf` | Neither |
| OBSERVATION | Apache | Sample_2010.pub | 72704 | `f0a90aa566d72eb822eb48b4f378c33007610e35f3f1b999cdb88f2c9683858b` | Neither |
| OBSERVATION | Apache | Simple.pub | 65536 | `2606f530052a818c7949b88c9bf245dbc0a74ddc6b5a60004f37b387fe4b955d` | Neither |
| OBSERVATION | Apache | clusterfuzz-testcase-minimized-POIHPBFFuzzer-4701121678278656.pub | 331491 | `07b48fcd8a3b85a83b054ea9722352d5269591d2482fb710002794da49f48b26` | UNKNOWN |
| OBSERVATION | Apache | clusterfuzz-testcase-minimized-POIHPBFFuzzer-4918886059278336.pub | 32832 | `2641d2d48a345f0eb52b4dd0151330142e3037505fea614c272ec7eba4b3e9b0` | UNKNOWN |
| OBSERVATION | Apache | clusterfuzz-testcase-minimized-POIHPBFFuzzer-6325615354773504.pub | 2042 | `e06b81d1a11a592d890e8d77c7f243c0529a36a0ec9bc4f18e8b1afb9e4f7bd2` | UNKNOWN |
| OBSERVATION | Controlled | fill__fill-green.pub | 95232 | `140bc6d105976866a894c5ad869cae4c3d4e53792182cd8b95e72ca2356ef03e` | Neither |
| OBSERVATION | Controlled | fill__fill-grey.pub | 95232 | `f653a0279aed2777a32ecc4f0c07178116655d158a7c13ef8558e171fb1e72dd` | Neither |
| OBSERVATION | Controlled | fill__fill-orange.pub | 95232 | `679755392fd23b3e78abc496678c2a28af91fcb8b1fdd1fb05c53bcf07126432` | Neither |
| OBSERVATION | Controlled | fill__fill-red.pub | 95232 | `dd52ac497bedbe04d6b7d304fc27ef7769dbfb7c1a060c6f41b215857519f1b6` | Neither |
| OBSERVATION | Controlled | fill__fill-white.pub | 95232 | `b42326eeee9eb0a3213510b67f4795cdcabd49d75c090910f345e19b40f0fe61` | Neither |
| OBSERVATION | Controlled | fill__gradient.pub | 95232 | `3de0b9e1abc3100335cd6d021b6c15d80f4a04731db883adde2148ae39a7af29` | Neither |
| OBSERVATION | Controlled | fill__no-fill.pub | 95232 | `29a1a6f2b432b288bf98cd2b00eae4c91ca5a84feb8d6d74bbf78fe02fb23e05` | Neither |
| OBSERVATION | Controlled | fill__scheme-accent1.pub | 95232 | `402ececd0e5ad6cf320eda56cc673bd92d553dc5c5c94110ec91a27b86baa4a2` | Neither |
| OBSERVATION | Controlled | fill__transparency-50.pub | 95232 | `9f39d034e94c082c605e74cde10b32fe5ec6f8a96ceb0d15d96e542686d6d449` | Neither |
| OBSERVATION | Controlled | geometry__base.pub | 95232 | `1e3d6873946014b99d13fc8e47ee9f3c9c8ef368bfa0b754a725e023e4d4c34d` | Neither |
| OBSERVATION | Controlled | geometry__flip-h.pub | 95232 | `f7dd8434332adac8a67fdf4e253cd104b4d8b3ad16f3448abd501a59503a9f9f` | Neither |
| OBSERVATION | Controlled | geometry__flip-v.pub | 95232 | `d2d4da9aa9ba00615f42c1a27e0639fc59af0793b5bbaf7442e48b442b7192fe` | Neither |
| OBSERVATION | Controlled | geometry__height-10.pub | 95232 | `da1a0124946404d017ff51e4ed21d2bab0614af75b01403036cd4489e15b057a` | Neither |
| OBSERVATION | Controlled | geometry__height-300.pub | 95232 | `d5c4203342c1538cf38009a85691542439844b3cbdb137a8c314ccaa911ed73d` | Neither |
| OBSERVATION | Controlled | geometry__left-0.pub | 95232 | `2d003f7cb246a601a1224975d1aa94d6ea0e81d29b24f394e3ced2ce0086ca45` | Neither |
| OBSERVATION | Controlled | geometry__left-1.pub | 95232 | `fea0686a4ecfaadc6ceef84037cd356c5a493a20759700b344c0d6e9df7be186` | Neither |
| OBSERVATION | Controlled | geometry__left-100.pub | 95232 | `7eca9f50be551171e99e6cd7314e5927f41092db5d676504da87131f7937ed76` | Neither |
| OBSERVATION | Controlled | geometry__left-306.pub | 95232 | `fc55f9528ee597e14e9b90831443e7bb2975c7d7fd52b5bc150d50b122f21a4b` | Neither |
| OBSERVATION | Controlled | geometry__left-500.pub | 95232 | `97179df47ce218e0be341a88668445958bf4f11439a13caa1dc3bc20e3a2dbcb` | Neither |
| OBSERVATION | Controlled | geometry__left-73.pub | 95232 | `c9dce9f6ded024cdeb5dbba9d6497c63274f28d69f4f236cce7faed4cc98eda3` | Neither |
| OBSERVATION | Controlled | geometry__rot-1.pub | 95232 | `6ae4fcd65f3a40e0a860f2fc946a862df2f939e47ba516bb972471c15ead0310` | Neither |
| OBSERVATION | Controlled | geometry__rot-180.pub | 95232 | `517e9c739f56672e6486f71a4e53768d0ff15ab5ea13cdf7a2cde40f46b471e4` | Neither |
| OBSERVATION | Controlled | geometry__rot-270.pub | 95232 | `d719acb601ad60c9eb3eaacadc3a9fc562e9b96be3a77d25a45a2397b7657c09` | Neither |
| OBSERVATION | Controlled | geometry__rot-30.pub | 95232 | `d4b4fa2c4db0d6533d9fddc722af0024fc39c30b3c2d7b76e7dbad67d30e2168` | Neither |
| OBSERVATION | Controlled | geometry__rot-359.pub | 95232 | `772bb25fbfd595bb7829f798d3caf52cb53ec6a8645ee4026c830cdd982e44bb` | Neither |
| OBSERVATION | Controlled | geometry__rot-90.pub | 95232 | `e360634ee3df510142dffb699c4a60a8bfe50c1cbd3a27b01682b5f3a0799a87` | Neither |
| OBSERVATION | Controlled | geometry__top-0.pub | 95232 | `c846a7e17cd34d20ddfc9b6e95e11d6eff1d4472d6a80cdc6e76def8375ceb86` | Neither |
| OBSERVATION | Controlled | geometry__top-396.pub | 95232 | `f2dec8a67177808512b34d4c908fd6f1e83b3a4fd75cbeb290163d6db44b5b68` | Neither |
| OBSERVATION | Controlled | geometry__top-700.pub | 95232 | `91e4f802c8cf3de112add7e5b3bae2506edb5d511b81a0c5701fd044226cabf5` | Neither |
| OBSERVATION | Controlled | geometry__top-74.pub | 95232 | `850dd82b85c9382843ba2037348ca416ae1d773035e09ab4f55eb1c719519f4e` | Neither |
| OBSERVATION | Controlled | geometry__two-rects-swapped.pub | 95744 | `a952da95e108fa3697ecceb4ee0936f7b3a2b780f4eff751eebf18b88e17231d` | Neither |
| OBSERVATION | Controlled | geometry__two-rects.pub | 95744 | `834cb91a54880211143b29a63ee61297a05bdc506dedd316b211195f5a0a2f97` | Neither |
| OBSERVATION | Controlled | geometry__width-1.pub | 95232 | `1fbe3169b7145be8952708929d767e607fb530d7e4a48dd7f6104d8c2f3b4f41` | Neither |
| OBSERVATION | Controlled | geometry__width-150.pub | 95232 | `5a1d8d731ef00e6861242764cd836f656242bcfd3022ae39f76786504f0628a0` | Neither |
| OBSERVATION | Controlled | geometry__width-612.pub | 95232 | `5a081f6b636c97fa1135aee399ba9376bce552fedd9b2dbdf6e0a53a46f0f1ec` | Neither |
| OBSERVATION | Controlled | group__rotated.pub | 95744 | `aea7f79db246dd5c34269108b1dab8b29c5541c6b1c4523d80b0d3630b160ce6` | Neither |
| OBSERVATION | Controlled | group__two.pub | 95744 | `d7cecf37f5c0325a8dc1187df424c30bfa8525bb3098e0b4a06790f394af5159` | Neither |
| OBSERVATION | Controlled | line__color-red.pub | 95232 | `f1e9fe2a07f69f9b8f777f9c5914716d713a08c9164f6b19f08cfd57e2a45980` | Neither |
| OBSERVATION | Controlled | line__dash-2.pub | 95232 | `52c2be877b0552c81fac34fa81a62f2bf4eff5c9edf2d8d67fb77f801f07f1fb` | Neither |
| OBSERVATION | Controlled | line__dash-4.pub | 95232 | `ddb223c5e4275ae92867468a51fee8a4e6bd41e1812dd2241eba040c761a2a6e` | Neither |
| OBSERVATION | Controlled | line__dash-5.pub | 95232 | `9885f150d7e4eb7955fbdf62543e7e9561a0447cd429328ee8684a2ea0d25129` | Neither |
| OBSERVATION | Controlled | line__no-line.pub | 95232 | `bee2971fe48138d329be877f94dbc7675c6e9aab59cf641b12a88bb567bcd9c7` | Neither |
| OBSERVATION | Controlled | line__weight-0.25.pub | 95232 | `af8ef6244d774c69996635b7947b3482ef4c4a1871c2042e2eb139ac7e03b96b` | Neither |
| OBSERVATION | Controlled | line__weight-1.pub | 95232 | `91aa8bee66da5d13efeb385ec69f15303b0afaf9acfc1a95dce7abcd7d15ba90` | Neither |
| OBSERVATION | Controlled | line__weight-10.pub | 95232 | `dd194825abb2c45cb7ca9b81f00dc6c11c58597d8bc4e139dcd0ed13edd737dd` | Neither |
| OBSERVATION | Controlled | line__weight-4.pub | 95232 | `c3c1a3f1aa72573a783c6b9c8b250e5fb63ab77349ba729b3de90485aa9a2233` | Neither |
| OBSERVATION | Controlled | pages__landscape.pub | 94720 | `22ad856c3b61fc601b1895bb72c8e921ed0330bb553ab6c8a720b599c15aac3d` | Neither |
| OBSERVATION | Controlled | pages__master-ignored.pub | 95744 | `92409f24827f069f1503c82469cabbe911d697b312b0ded4bfd66607dabc67eb` | Neither |
| OBSERVATION | Controlled | pages__master-rect.pub | 95744 | `400c061402e471bdede31c8f4d9db4054693233f098c7ded426988f9ef89c218` | Neither |
| OBSERVATION | Controlled | pages__master-text.pub | 97280 | `8137a8885c89f9f06bf922f532dba66246ca955922b7ccf0c4bdc54733b1479f` | Neither |
| OBSERVATION | Controlled | pages__one.pub | 95232 | `18905060eb45f6aede59991550ce688ce9cf1e5d03beb7f16ea3fadb979551fc` | Neither |
| OBSERVATION | Controlled | pages__size-5.75x4.75.pub | 87040 | `abd024933717e4806054f99b66bb9ddc8d5eac7a7c51803281c916a2ff97ee2c` | Neither |
| OBSERVATION | Controlled | pages__three-rect-p3.pub | 96768 | `3c15cd70034b50a63831a2d248acc9d06621f241c494439d8a07a233e5556459` | Neither |
| OBSERVATION | Controlled | pages__two-rect-p2.pub | 96256 | `bf5d98cb2082e73a99a53a797ffc07353c432ed22a8ea943ff9930d8aba04284` | Neither |
| OBSERVATION | Controlled | pages__two.pub | 95744 | `815fad77c79fa4f88fbde52b4001bc5ef2097d5f4751965518447122ae03ef5f` | Neither |
| OBSERVATION | Controlled | picture__base.pub | 181760 | `39a32b2e6ce8073fbfe9a2d66cb9ab8b4e0d0952bb294a06ac609a3749f071b3` | Neither |
| OBSERVATION | Controlled | picture__border.pub | 181760 | `97907db68b160cd9d9c87eb31f39ad1474cb9d524df6988d51cac82c3a6adb9f` | Neither |
| OBSERVATION | Controlled | picture__crop-all.pub | 181760 | `a8201d751ce462e5946a0813465f5fe61b38eb25227c4afdf600fc1c6f010012` | Neither |
| OBSERVATION | Controlled | picture__crop-left.pub | 181760 | `49ae8c6529cc2a725677f3ec191b2be0e2bcad86d68eaa7887fccac94487aa31` | Neither |
| OBSERVATION | Controlled | picture__crop-top.pub | 181760 | `2db5fd28b05640c1e4ccc8d5826dcd560b166feba4d5d8ab96bfa7dcb25d2ebf` | Neither |
| OBSERVATION | Controlled | picture__flip-h.pub | 181760 | `3637e89b38d233a73f0c4b5e1b28e93ff4aa5dcce96ad2ecaf7f0714340405ca` | Neither |
| OBSERVATION | Controlled | picture__moved.pub | 181760 | `ec1964648b15eb40b6e0cae9ca2118173343eaac091d3232f24cd655768127f8` | Neither |
| OBSERVATION | Controlled | picture__oval-mask.pub | 181760 | `0ef974b5861894d463493aed878f2e36f8147369a1ae10593386f8e77a04a0a8` | Neither |
| OBSERVATION | Controlled | picture__rotated-30.pub | 181760 | `15bdddc73b1b3866207739538fe4c6596e7cbd850aedf7ed3bbad0719738e55f` | Neither |
| OBSERVATION | Controlled | picture__stretched.pub | 181760 | `bb619e30fb9b49619b45ceed21daa9cb9c505fd4fbb791ce44b02428308f077d` | Neither |
| OBSERVATION | Controlled | picture__two.pub | 181760 | `97dcd67cc415bd962589122fe39f95d32304d2733005c54c5fdc711273007d3d` | Neither |
| OBSERVATION | Controlled | shadow__offset.pub | 95744 | `2b8c3cd8478da53e364097b92e2978b8c6f9037a0b576dce84060dd98684eaad` | Neither |
| OBSERVATION | Controlled | shadow__on.pub | 95744 | `b01c8e35d89bcccc1d1e13e85be2c48eb82128c8a88d36adbfd4503f54f8d209` | Neither |
| OBSERVATION | Controlled | shapes__curve.pub | 95744 | `a932e1e445dbf5e088e67bcb6c2344f8ef78f7cb2c140c99d39d13f79adf6501` | Neither |
| OBSERVATION | Controlled | shapes__diamond.pub | 95232 | `b5439878dd52c0a54214a92bb731e415ac91ca3083a9e711e42d92439817d6af` | Neither |
| OBSERVATION | Controlled | shapes__heart.pub | 95232 | `15e55be8782f648de3f48960f97ceac0200a55d80efaa015fa6bdbeac87ae9f5` | Neither |
| OBSERVATION | Controlled | shapes__hexagon.pub | 95232 | `58981e80239daa7a147484d5d05dad17c55e4f51260b19b403a61f335a03c2a8` | Neither |
| OBSERVATION | Controlled | shapes__line-arrow.pub | 95232 | `b727ec743dc1a7c57eb46103673043afa51fea8451d85d9b95d30968488f27a1` | Neither |
| OBSERVATION | Controlled | shapes__line.pub | 95232 | `c02ab6f95f190fb8d1923e7e9965b19f23d77b9e79cfa157944b736047f8bbe0` | Neither |
| OBSERVATION | Controlled | shapes__oval.pub | 95232 | `29cb3020390b751a5c66767688be231a2f7d77a1776d0b2c0060fbdc89f64fdb` | Neither |
| OBSERVATION | Controlled | shapes__polyline.pub | 95744 | `750d1261225acaf73a47d8987118626a39975cc53ce791f5961955757ad601a6` | Neither |
| OBSERVATION | Controlled | shapes__rect.pub | 95232 | `9f533e45948ddc0be2f7be85c97d774766ce36e2d57a1209ef8bb9fbde6c8513` | Neither |
| OBSERVATION | Controlled | shapes__rightarrow.pub | 95232 | `1fd8cdabecd5c9345f9f2a00eeeede7f126c68e20809dea7b3a2423e8a4e5b29` | Neither |
| OBSERVATION | Controlled | shapes__roundrect.pub | 95232 | `6e5b1b4c63a0ee226ca9a8c6c1c863be5481854bb64baf88ed03d2e53c0e1e33` | Neither |
| OBSERVATION | Controlled | shapes__star5.pub | 95232 | `b64f6a7ef259287a8b35ef97c8d40ccb425eb55b54953b3255798cc82a5a95f2` | Neither |
| OBSERVATION | Controlled | shapes__triangle.pub | 95232 | `b6c270e0c709cbfe90b2270c6afc10d17499dfd4d15feca0fcd16dc00d3a8616` | Neither |
| OBSERVATION | Controlled | table__2x2.pub | 97792 | `72d2127f4ffdbe998f4f94414a28584e49af817991b70a564fda885dcb1a6a7d` | Neither |
| OBSERVATION | Controlled | table__3x2.pub | 97792 | `12c45cd9057553f748d0cf23ab5348d901be294b1d6e790eb80c3331d57d2616` | Neither |
| OBSERVATION | Controlled | table__merged.pub | 97792 | `3df51f5e3e29f247b8636627725d43f5973dc7ea887338f32aec8f4b49a23b30` | Neither |
| OBSERVATION | Controlled | text2__base.pub | 97280 | `7d32fbec55d474c67d7a344db4fff17cb1f9216cdda6d736ab2c8424203662b6` | Neither |
| OBSERVATION | Controlled | text2__p2-bold.pub | 97280 | `36baee97f339fcdbde70984d9c4bc851844948375a95601d35e75c9fdca43e50` | Neither |
| OBSERVATION | Controlled | text2__p2-center.pub | 97280 | `6d7a87753090be349d6eec1a1196f9f6b92776073acc2adc6661f4dadf49d1d3` | Neither |
| OBSERVATION | Controlled | text2__text-in-oval.pub | 97280 | `270e0d486423e8a869cb01f72487771ef6fae1e650632ed995d656b29473def2` | Neither |
| OBSERVATION | Controlled | text2__two-boxes.pub | 97280 | `a6405f9b31de04d69a3fdd4179fe97cbfc30e629a8c874e40da80cceaeba0d48` | Neither |
| OBSERVATION | Controlled | text__align-center.pub | 97280 | `226cf7e6918146193e531520b9005e06f434b8f62ceaf700b455bc8ac4fb3713` | Neither |
| OBSERVATION | Controlled | text__align-justify.pub | 97280 | `c5442fe547575f882d51809a315a6916a7233895cafe849e20e09e8e9409b739` | Neither |
| OBSERVATION | Controlled | text__align-right.pub | 97280 | `d1aa1cacbbb272f8b6456c79cf1e024218e15bb40e0f979a4ee53dbf0ea1f969` | Neither |
| OBSERVATION | Controlled | text__allcaps.pub | 97280 | `f95eb0920c19eafd027884bb33ed31389d4aad91094755df1bb9f777a4dc8a25` | Neither |
| OBSERVATION | Controlled | text__base.pub | 97280 | `0c9293e727559fe58ba150e8445b1365b4960883fbe6759c10b2477f128ff9a7` | Neither |
| OBSERVATION | Controlled | text__bold.pub | 97280 | `90a140c2db6c6c8399c848a26a4e7c9ce822d52d219f5589ee026345c2e05635` | Neither |
| OBSERVATION | Controlled | text__bullets.pub | 97280 | `51df14aa94d640932656bc49a3f645b97333ba811467761e31226ec1547011f5` | Neither |
| OBSERVATION | Controlled | text__color-custom.pub | 97280 | `62436393aa82998736c87930793ca8d019162d8fadcfc67700c36c6db5d96c6f` | Neither |
| OBSERVATION | Controlled | text__color-red.pub | 97280 | `8402349638c905016b5d56ee2d4356c906015631b371985378e676d8b6c4ad0a` | Neither |
| OBSERVATION | Controlled | text__font-Arial.pub | 97280 | `9249dd72feeab344a9a5b2d366c4b6973a90843b8090cd8d8831696cdce6295d` | Neither |
| OBSERVATION | Controlled | text__font-ArialBlack.pub | 97280 | `024b15dc2154c4779796e0ea748fdff763bf065f1750cd938eeb07312461e42f` | Neither |
| OBSERVATION | Controlled | text__font-CourierNew.pub | 97280 | `d8f715a6f23a8bf396ccb65da270c62e52a23fc1f26465421d0c413d730010a2` | Neither |
| OBSERVATION | Controlled | text__font-TimesNewRoman.pub | 97280 | `5bd50ace94c87c2962ce97221fffb5ebfcd2c56f46808e3aa275fbcd4b06b77d` | Neither |
| OBSERVATION | Controlled | text__frame-fill-line.pub | 97280 | `59a0bfa1e5642f9164fb7d4b7557e2c080ddc7abfe76276506d21396951500a3` | Neither |
| OBSERVATION | Controlled | text__indent-first-18.pub | 97280 | `2a4a4f3b861c9514d12105312a86e94c6b310545151fe9c238b32e4870035634` | Neither |
| OBSERVATION | Controlled | text__indent-left-18.pub | 97280 | `fe22b3b44baa690da104712645f606a755b9c626bdb8b5b7b6fc5d6ef4abb5c9` | Neither |
| OBSERVATION | Controlled | text__indent-right-18.pub | 97280 | `91fa9d4f838c8909ece6d16ba5c98d5b1bee4f6a39484ffd95bc8b4f17ce332d` | Neither |
| OBSERVATION | Controlled | text__italic.pub | 97280 | `60216ffe5ee57f80c546bd99724086c9693db682e11d3ad60e2ae2ec31eb167e` | Neither |
| OBSERVATION | Controlled | text__line-break.pub | 97280 | `ac822b60ab7365ab82a3d2a8bbd9edc5f9061407d111749c5942b73fc53a125c` | Neither |
| OBSERVATION | Controlled | text__ls-double.pub | 97280 | `9242fc72b4ff5ab82745969006895f6d7a085d190820b2c28a62d04fdb3980a4` | Neither |
| OBSERVATION | Controlled | text__ls-exact-14.pub | 97280 | `22191bec6bd65bbc214760d982e1b44091ec9c11d8ecd3076ac6bf8d5f58d188` | Neither |
| OBSERVATION | Controlled | text__ls-exact-30.pub | 97280 | `69067437491eec0df68a82f812177713207942246444cfbd8db040db8e6a767d` | Neither |
| OBSERVATION | Controlled | text__ls-multiple-1.5.pub | 97280 | `35e0d9d58aab018b259a4a3cd95f05de3629a56e13c5a69109f06a7a6418313f` | Neither |
| OBSERVATION | Controlled | text__ls-single.pub | 97280 | `e8957ec0950137eb616288ba1dd7a882798eef128786c0610aaba88e592907d3` | Neither |
| OBSERVATION | Controlled | text__margins-10.pub | 97280 | `4708cc8ef89dc69507b1ccb0fc1ef13bd837592b366b1d3507a7bc85fe81796a` | Neither |
| OBSERVATION | Controlled | text__rotated-270.pub | 97280 | `ceae546e1c875acf4349a577263e76242ec76805d08211dc6c550f2a0f84427f` | Neither |
| OBSERVATION | Controlled | text__run-bold-word.pub | 97280 | `94d1838e1668d6c0a090e1841944e2295f5229c0a927dcdf5210117f36bc0063` | Neither |
| OBSERVATION | Controlled | text__run-font-word.pub | 97280 | `8bec2c8a3d35ff8a3d3b89cbb48b51f24337e33ae34486186bb4163eff523cdb` | Neither |
| OBSERVATION | Controlled | text__run-red-word.pub | 97280 | `6ab61f70310cb4d2adedfe0d670c11ae8ffadf037874536748efa9e1f15f924e` | Neither |
| OBSERVATION | Controlled | text__run-size-word.pub | 97280 | `089e2a4dd7439549dde6d58e9240c7d5342e201a7edf6ab96ea2e214b0f0ecc0` | Neither |
| OBSERVATION | Controlled | text__size-12.pub | 97280 | `78aba312d290c8fa8c29a93de07e611689000df7924eb8988af35dbb51e25a89` | Neither |
| OBSERVATION | Controlled | text__size-24.pub | 97280 | `80f027a7aca4cf537134bed35eec5ad812794518033541f809b85b24d847df16` | Neither |
| OBSERVATION | Controlled | text__size-36.5.pub | 97280 | `31f4aefa4307098e57a47ff4db35fb8b797d1c71dadf6e7e27ee938c892b03cc` | Neither |
| OBSERVATION | Controlled | text__size-72.pub | 97280 | `7967783a2a96012febe6b9a7c7222c4c24bca1f02170bdc56f9485617ea7ecfe` | Neither |
| OBSERVATION | Controlled | text__size-8.pub | 97280 | `f86bbf9bfac4e67cf34f5ded0cbb5d50fded0fd2634b141f41cbce02cbec7b25` | Neither |
| OBSERVATION | Controlled | text__smallcaps.pub | 97280 | `76ea2fea4a7885054bc1f013971d2fc3acdf7f3c012541520451ddda304eb6c1` | Neither |
| OBSERVATION | Controlled | text__space-after-0.pub | 97280 | `da8a9b5fb487c0acff844a5189fb5362b0da966f330c841f50c8db5007b5c2f8` | Neither |
| OBSERVATION | Controlled | text__space-before-12.pub | 97280 | `260f23e741dc2b29fccd4203c1639c07ef5fbbfc50f75351a239721c93da973a` | Neither |
| OBSERVATION | Controlled | text__superscript.pub | 97280 | `ae1f26ebbc06c7f8904d18ac0e559076256f3f8b9e0a02e11c49e96fca4e6604` | Neither |
| OBSERVATION | Controlled | text__text-changed.pub | 97280 | `21fe1e79b4da37a0cefd87cd494c5bb977073a39be883f38dba068f7a403c200` | Neither |
| OBSERVATION | Controlled | text__text-longer.pub | 97280 | `7fe759e9f7d513e01332717020ca662338bce1251a2d16f28e155c484b96afe9` | Neither |
| OBSERVATION | Controlled | text__tracking-200.pub | 97280 | `f9bc7d51af14bdd8b5eb289225a79738e39b677e74b4f7dcc36411dcc6de2664` | Neither |
| OBSERVATION | Controlled | text__underline.pub | 97280 | `3a4baac97c143a1ae519e4c149fe8df0ce0fec74119d1c1f6109238b116d5f5d` | Neither |
| OBSERVATION | Controlled | text__vanchor-bottom.pub | 97280 | `886be8dbfa7d2c2590ea408f4300a2014823da7585652419badd0ea207d2060e` | Neither |
| OBSERVATION | Controlled | text__vanchor-middle.pub | 97280 | `f92f73a3e92f700bc658a4438a1e30fa692bcc1370bed3570484d808b6299603` | Neither |
| OBSERVATION | Controlled | wordart__arch.pub | 98304 | `dc22fb9983e9119c9969e37dd5ba3d19fb63fa489c8ffeec4e38ea921a41904b` | Neither |
| OBSERVATION | Controlled | wordart__base.pub | 98304 | `270d8fe5d1a2286622b35e40b39e00eac17b878734aedb755059b3fc20050838` | Neither |
| OBSERVATION | Controlled | wordart__circle.pub | 98304 | `bc145b67097b5d9d42f0f5c838d68f2d88130267b51031bdeacd4cb9d0590eeb` | Neither |
| OBSERVATION | Controlled | wordart__fill-red.pub | 98304 | `e6745af79ae3d98bf3b79e3104e291f158934da6a8969dd722e24d0eecd9e6f4` | Neither |
| OBSERVATION | Controlled | wordart__font.pub | 98304 | `1c19cda5f6a40f1cb6f64f3b67a334ff54656bce573be480083c7bdc1bb0fd8a` | Neither |
| OBSERVATION | Controlled | wordart__line-white.pub | 98304 | `5968f890c0b4029dde31c644cb3ff29eb071225e685ea4fc4a527058f675755e` | Neither |
| OBSERVATION | Controlled | wordart__shadow.pub | 98304 | `d24ea44009d4e8c00860a61aec2391cbb3e30fd791deb5f5f480382218cb102d` | Neither |
| OBSERVATION | Controlled | wordart__text.pub | 98304 | `f3d6d544b2e47af20f49c20dbe9151e9c41eb53fbf655a685f8c4cb4cb46d62e` | Neither |
| OBSERVATION | Repository fixture | hello-nifty.pub | 7168 | `5e22460c8bcff08d39362f68177430ca953a136083dc35ffadb95c3375f92c1a` | Neither |
| OBSERVATION | Repository fixture | wmf-picture.pub | 7168 | `7606a8fd9c4f9a5474a29642ec845e636ee2b22d9a7613c3ea5b2e85d435903f` | Neither |

### Container failures

| Basis | File | Exact container error |
| --- | --- | --- |
| OBSERVATION | clusterfuzz-testcase-minimized-POIHPBFFuzzer-4701121678278656.pub | Malformed FAT (sector 129 pointed to twice) |
| OBSERVATION | clusterfuzz-testcase-minimized-POIHPBFFuzzer-4918886059278336.pub | Malformed FAT (FAT has 244 entries, but file has only 64 sectors) |
| OBSERVATION | clusterfuzz-testcase-minimized-POIHPBFFuzzer-6325615354773504.pub | Invalid CFB byte order mark (expected 0xFFFE, found 0xFFFA) |

### Replication checkpoint

| Basis | Rule | Further A records/files tested | Result and limit |
| --- | --- | ---: | --- |
| OBSERVATION | Container/header and directory | 0 further files | Only the known two A files qualify; no new instance tests generality. |
| OBSERVATION | Class-selected record extents and reference graph | 0 further records | Prior bounded checks were rerun unchanged on the known files; the existing 93/93 extent endpoints and reference checks pass. This is a baseline recheck, not replication on new data. |
| OBSERVATION | Root +0x14/+0x18 size at candidate 36000 units/mm | 0 further files | Known A4/Letter numerical fits rechecked; no new physical-size evidence. |
| OBSERVATION | Index-8/index-10 signed32 rectangle window | 0 further positive frames | Six Sample98 candidate rectangles remain positive/distinct/contained under the inferred translation; mspub controls remain zero. No independent placement replication. |
| OBSERVATION | Quill links at index-8 +0x58 / index-10 +0x66 | 0 further positive stories | Existing six Sample98 links and empty mspub control rechecked; no new ID/range evidence. |

OBSERVATION: no new A record can supply a failing record index/class/size or a failing field, because there are no new A records in this inventory. The three failures above are container failures, not extent-rule counterexamples. UNKNOWN: replication of all class rules, references, page dimensions, rectangles and Quill associations outside the known pair remains untested. Neither a rule pass nor a rule failure is claimed for the B, newer-family or rejected-container files.

INFERENCE: the next smallest discriminating step remains obtaining an independently saved, structurally confirmed variant-A file with a known physical page size and one unrotated text frame at documented noncentral positions. First run the unchanged structural and Quill-link checks, then compare the candidate scale/origin with that known rectangle. The available B/newer data cannot replace that missing positive control. UNKNOWN: physical geometry semantics, page identity/order and untested saving versions remain open. This inventory closes availability in the searched locations, not those format questions.
