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
3. UNKNOWN: root/page ordering and master/scratch classification. OBSERVATION:
   AP2 accounts for all 24 direct directory children of the eight index-20
   records: 17 tail IDs and seven fixed-field IDs. OBSERVATION: AP3 accounts
   for another 12 matching directory edges, eleven tail IDs and one fixed ID.
   OBSERVATION: the conservative AP10 selector graph, augmented only with
   prior proven index-20 tails, has 38/27 edges and 16/12 indegree-zero
   vertices in Sample98/mspub; both are partial forests, with no cycles or
   dangling included IDs and every edge agreeing with the parent candidate.
   UNKNOWN: reference roles and child-list rules for other classes or beyond
   these two samples; the partial graph does not identify publication roots.
4. UNKNOWN: general shape-to-story mapping, field widths, geometry, formatting,
   table-cell ownership and displayed visibility. Existing Quill import failure
   on kind `0x19` was not repaired or bypassed into a complete import.
5. UNKNOWN: independent saving versions and wider coverage, including any
   confirmed Publisher 97-specific sample. OBSERVATION: AP3 verifies the
   supplied mspub.pub size/hash and structural checks; its earlier unavailability
   is resolved. UNKNOWN: zmspub.pub and the claimed equality of the owner pair
   remain unverified; independent manufacture is owner-reported provenance.

INFERENCE: after AP11, the next smallest byte-only test is descriptors 2/3's
+370 selection in their four instances. Compare the literal selection with
the bounded candidate +114 (370 modulo 256), using directory-ID and matched
controls before accepting any high-byte tagging hypothesis. UNKNOWN: this
is not an established mapping, and it does not resolve +208. Probable/zero-only
roles, duplicate counts, nonempty index-49 behavior and B remain separate.

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
