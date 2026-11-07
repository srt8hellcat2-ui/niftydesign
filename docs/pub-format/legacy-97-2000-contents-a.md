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

1. UNKNOWN: how the layout descriptors determine core lengths, field locations,
   variable tails and reference relocation; B defeats the simple A walk.
2. UNKNOWN: general record extent/padding rules beyond adjacent directory offsets,
   directory flag meanings, and header words `0x0C`, `0x1E`, `0x20`, `0x24`.
3. UNKNOWN: complete child-list framing, root/page ordering and master/scratch
   classification; inferred owner links are not sufficient for a reader.
4. UNKNOWN: general shape-to-story mapping, field widths, geometry, formatting,
   table-cell ownership and displayed visibility. Existing Quill import failure
   on kind `0x19` was not repaired or bypassed into a complete import.
5. UNKNOWN: independent saving versions and wider coverage, including the absent
   owner pair and any confirmed Publisher 97-specific sample.

INFERENCE: the next smallest decoding step is to explain only A index-20's
variable child-ID tail, starting with the 44-byte record `0x10A` and checking
its two directory children, then the 50-byte `0x127` and five children. Prove
each count/length/element-width field and how the descriptor participates
before trying another record class or B's descriptor table. This checkpoint
does not perform that follow-up.

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
