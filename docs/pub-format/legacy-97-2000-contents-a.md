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
   UNKNOWN: authoritative duplicate fields in index-20, general relocation
   and the meaning of descriptor `0x0019`; descriptor stride is not universal.
2. UNKNOWN: independently length-derived extents/padding for 23 records of
   numeric classes 2, 3, 10 and 195; semantic meanings of all four directory
   attribute values; header words `0x0C`, `0x1E`, `0x20`, `0x24`.
   OBSERVATION: AP4 accounts for 70 spans by three field-driven arithmetic
   rules. AP5 narrows five of the 23 unresolved cases to identical 22-byte
   index-49 tails, but establishes no stored tail-length/count algorithm;
   eighteen other cases remain outside that five-record observation.
   UNKNOWN: a universal length rule, padding in those 23 logical records,
   and flag-bit meanings remain unproved. The five constant-tail fits are
   not promoted to independently decoded extents.
3. UNKNOWN: root/page ordering and master/scratch classification. OBSERVATION:
   AP2 accounts for all 24 direct directory children of the eight index-20
   records: 17 tail IDs and seven fixed-field IDs. OBSERVATION: AP3 accounts
   for another 12 matching directory edges, eleven tail IDs and one fixed ID.
   UNKNOWN: reference roles and child-list rules for other classes or beyond
   these two samples.
4. UNKNOWN: general shape-to-story mapping, field widths, geometry, formatting,
   table-cell ownership and displayed visibility. Existing Quill import failure
   on kind `0x19` was not repaired or bypassed into a complete import.
5. UNKNOWN: independent saving versions and wider coverage, including any
   confirmed Publisher 97-specific sample. OBSERVATION: AP3 verifies the
   supplied mspub.pub size/hash and structural checks; its earlier unavailability
   is resolved. UNKNOWN: zmspub.pub and the claimed equality of the owner pair
   remain unverified; independent manufacture is owner-reported provenance.

INFERENCE: after AP5, the next smallest step needing no new sample is to
compare only the two remaining numeric-class-2/index-82 tails, whose AP4
spans are 244 and 40 with H=18. Their differing residual sizes, 226 and 22,
can test a length/count rule that identical index-49 tails cannot distinguish.
Keep their contents opaque. UNKNOWN: a shared class-2 tail rule is not yet
established; index-20's duplicate roles and B's table remain separate work.

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
