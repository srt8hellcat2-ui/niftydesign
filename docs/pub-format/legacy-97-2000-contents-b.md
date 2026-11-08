# Legacy Contents variant B: directory, extents and text links

## Scope and evidence

OBSERVATION: AP16 examines only the pinned binary Sample2000.pub, 61440 bytes, SHA-256 `40701ca47b26d04771cdd58467764e9ab39d3da69b59fbc529afd16d263d2f86`, with Sample98.pub as the already documented variant-A comparison. The restored data directories contain 142 controlled and 24 Apache publications. The sample identity is rechecked against its pinned identity; no scoring exports, third-party reader, external format description or specification was consulted. Only derived facts are committed; needed stream copies remain in an ignored research directory. No importer code changed and no full CI was run.

OBSERVATION: A/B are structural sample labels, not independently established saving versions. All B findings in this note are single-sample observations. The AP15 census found no second structurally matching B sample. UNKNOWN: replication of any B rule in another file.

## Container and header

OBSERVATION: the compound container opens as version 3 with root CLSID `{00021201-0000-0000-00C0-000000000046}`. Each stream was opened and its actual length checked. The root CompObj matches A and contains MSPublisher.3; this does not establish a saving version.

| Basis | Stream | Bytes |
| --- | --- | ---: |
| OBSERVATION | `/Contents` | 6042 |
| OBSERVATION | `/Quill/QuillSub/CONTENTS` | 6144 |
| OBSERVATION | `/Quill/QuillSub/\x01CompObj` | 86 |
| OBSERVATION | `/\x01CompObj` | 94 |
| OBSERVATION | `/\x03Internal` | 12 |
| OBSERVATION | `/\x05SummaryInformation` | 44640 |
| OBSERVATION | `/\x05DocumentSummaryInformation` | 152 |
| OBSERVATION | `/Envelope` | 0 |
| OBSERVATION | `/Escher/EscherStm` | 0 |
| OBSERVATION | `/Escher/EscherDelayStm` | 0 |

OBSERVATION: offsets are hexadecimal stream-relative byte offsets unless otherwise stated; integers are little-endian and unaligned reads are necessary. Counts, spans and sizes are decimal.

| Basis | Contents offset / width | B | A comparison / check |
| --- | --- | --- | --- |
| OBSERVATION | +0x00 / 4 | Same documented old-family discriminator | Does not separate A/B. |
| OBSERVATION | +0x08 / u32 | 6042 | Equals actual stream size; A = 5930. |
| UNKNOWN | +0x0C / u16 | 717 | A = 616; no semantic role established. |
| INFERENCE | +0x0E / u16 | 256 | Equals first/root-like directory ID 0x100, as in A; root meaning remains inferred. |
| OBSERVATION | +0x12 / u32 | 0x3BA | Control start; A = 0x37E. |
| OBSERVATION | +0x16 / u32 | 0x3E8 | Directory start; A = 0x3AC. |
| OBSERVATION | +0x1A / u32 | 0x22 | Table-region start, same as A. |
| UNKNOWN | +0x1E / u16 | 252 | A = 248; role unknown. |
| UNKNOWN | +0x20 / u16 | 12 | Same as A; not the measured directory stride. |
| OBSERVATION | +0x22 / u16 | 94 | A = 89; all B record indices are below 94, but descriptor enumeration is not decoded. |
| UNKNOWN | +0x24 / u16 | 6 | Same literal as A; does not establish equivalent table framing. |

OBSERVATION: the control region at 0x3BA starts with u16 0xF0DE and u16 46. Its u32 fields at +0x0A (0x3C4) and +0x0E (0x3C8) both equal 0x3E8; 0x3BA + 46 = 0x3E8. The partition is header [0,0x22), table region [0x22,0x3BA), control [0x3BA,0x3E8), directory [0x3E8,0x608), ten zero gap bytes [0x608,0x612), records [0x612,0x179A). These cover the entire 6042-byte stream. A's pre-record gap is twenty bytes, not ten.

## Directory and index-entry checkpoint

| Basis | Directory field | B evidence |
| --- | --- | --- |
| OBSERVATION | D +0 / u16, D = 0x3E8 | 54 entries; 4 + 54*10 = 544 bytes ending 0x608. |
| OBSERVATION | D +2 / u16 | 0x141, equal to maximum ID among all 54 entries. |
| OBSERVATION | Entry +0 / u16 | 54 distinct IDs. |
| INFERENCE | Entry +2 / u16 | Parent candidate: 53 resolve to a directory ID; only ID 0x100 has external sentinel 0xFF. The resulting directory-parent graph is one rooted tree, 53 edges, no cycle and no missing non-root parent. This is not a complete selector-reference graph. |
| OBSERVATION | Entry +4 / u32 | 54 distinct ascending absolute record starts, all beyond directory end and inside Contents. |
| UNKNOWN | Entry +8 / u16 | Same four attribute values as A: 0x1640 (23), 0x16C0 (20), 0x16E0 (6), 0x1740 (5); semantics not established. |

OBSERVATION: 53 IDs are shared with Sample98 and every shared ID has the same parent candidate and attribute. A-only ID 0x119 has index 30; B-only ID 0x13D has index 92 and parent 0x100. Both spans are 26 bytes and H = 4. Their pairing is a comparison convenience, not proof that they have the same semantic role. Every other shared ID retains its index. The earlier broad statement that both directories have identical ID order is narrowed by this exact difference.

OBSERVATION: copying A's table walk from 0x26 (three u16 words followed by u16 selectors through zero) does not decode B. Its first candidate runs to 0x4E, incorporating what resemble further fixed headers. Continuing consumes 57 candidates and ends at 0x3C4, ten bytes beyond the control start, rather than yielding a count-consistent table. The values at 0x26/0x28/0x2A are 0x8CC6/1/8; the corresponding A first words are 0x6868/1/8. No B descriptor indexing or selector interpretation is inferred from this failed walk.

UNKNOWN: whether B table data has different entry framing, packing, additional fields or an alternative index convention. Record indices are verified u16 values bounded by 1..93, not yet proven lookups into decoded B descriptors. Class numbers below are explicitly A-index comparison labels, not classes independently read from B's table.

## Field-driven extent tests

OBSERVATION: R is a directory record start, E is the next start (or 6042 for the final record), span = E-R. Index is u16 at R; L is byte R+2 and H is byte R+3. Every record has 4 <= H <= span (54/54). T = R+H is a candidate fixed-part boundary. Predictions below use record fields, then compare with E; they never substitute E into a stored length. They establish exact sampled endpoints, not semantic payload meaning or the absence of internal padding.

| Basis | Equation family | Fields and checks | Exact matches when tested across all 54 records |
| --- | --- | --- | ---: |
| OBSERVATION | F: H | No tail | 8/54 |
| OBSERVATION | A: H+10+n*w | u16 n at T, duplicate at T+2, u16 w at T+4; equal counts and positive width; ten-byte header bounded | 27/54 |
| OBSERVATION | B: H+8+q | u32 q at T; eight-byte header bounded | 9/54 |
| OBSERVATION | C: H+22+n*(4+w) | u16 n/duplicate/w at T/T+2/T+4; counts equal; 22-byte minimum bounded | 8/54 |
| OBSERVATION | O: H+18+q | u32 n/duplicate/q at T/T+4/T+8; O=T+18 holds n+1 u32 offsets; first = 4*(n+1), final = q, increasing for nonempty data | 6/54 |

OBSERVATION: taking the union of the tested families accounts for 54/54 endpoints, with zero unmatched records. Fifty records have one family match; four empty tails match both C and O: IDs 0x11E (index 50), 0x105 (31), 0x104 (50), 0x13D (92), each H = 4, span = 26. At their T, O reads n=0, duplicate=0, q=4 and offsets=[4]; C reads n=0, duplicate=0, w=0. Their coincidence does not prove which structure applies. No single equation fits all 54 records.

INFERENCE: retaining A's class-to-equation choice for each of the 53 B records whose index has a sampled A class predicts 53/53 B endpoints, without selecting a family from the B endpoint. This supports transfer of the arithmetic, not a decoded B class table. For index 92 the candidate O rule fits 1/1, but its class and independent class-selected prediction remain UNKNOWN. Thus the result is 54/54 equation-family fits, only 53/54 predictions selected by the prior A-index hypothesis, and 0/54 independently decoded B-descriptor-selected predictions. These counts must not be conflated.

OBSERVATION: L alone equals span in only 4/54 records: 0x135, 0x136, 0x138, 0x12E. It fails in the other fifty rows listed below; it is not a universal record length. H alone succeeds only in the eight F rows and fails in the other 46. H/span differ from the corresponding A comparison in twelve rows, and agree in 42. In particular index-20 H changes from 30 to 32 in all eight instances; hard-coding A's 30-byte prefix would under-predict every one by two bytes. Indices 8/10 retain H = 118/116 and their text anchors.

OBSERVATION: in the table, A-class is only the prior A-index hypothesis; ? means no sampled A index mapping. Each row gives all fitting families, so every non-fitting record for each equation is exactly the complement of that family in this table. A-span and A-H are the same-ID comparison except for the explicitly paired 0x13D/0x119. Fields are relative to T; n/w are u16 for A/C and n/q are u32 for O. All repeated counts were checked equal.

| Basis | ID | Directory entry | R | Span / H / L | Index | A-class | Fitting families | Tail fields | A-span / A-H |
| --- | --- | --- | --- | --- | ---: | --- | --- | --- | --- |
| OBSERVATION | 0x100 | 0x3EC | 0x612 | 202 / 178 / 232 | 21 | 25 | A | n=7; w=2 | 202 / 178 |
| OBSERVATION | 0x11F | 0x3F6 | 0x6DC | 300 / 62 / 68 | 41 | 195 | O | n=2; q=220; offsets=12,116,220 | 294 / 56 |
| OBSERVATION | 0x120 | 0x400 | 0x808 | 144 / 144 / 172 | 54 | 0 | F | none | 112 / 112 |
| OBSERVATION | 0x123 | 0x40A | 0x898 | 194 / 28 / 28 | 40 | 3 | O | n=6; q=148; offsets=28,48,68,88,108,128,148 | 194 / 28 |
| OBSERVATION | 0x12C | 0x414 | 0x95A | 62 / 32 / 126 | 20 | 25 | A | n=10; w=2 | 60 / 30 |
| OBSERVATION | 0x133 | 0x41E | 0x998 | 150 / 140 / 188 | 2 | 1 | A | n=0; w=8 | 150 / 140 |
| OBSERVATION | 0x134 | 0x428 | 0xA2E | 150 / 140 / 190 | 3 | 1 | A | n=0; w=8 | 150 / 140 |
| OBSERVATION | 0x135 | 0x432 | 0xAC4 | 69 / 69 / 69 | 4 | 0 | F | none | 67 / 67 |
| OBSERVATION | 0x136 | 0x43C | 0xB09 | 70 / 70 / 70 | 5 | 0 | F | none | 70 / 70 |
| OBSERVATION | 0x137 | 0x446 | 0xB4F | 61 / 61 / 65 | 6 | 0 | F | none | 61 / 61 |
| OBSERVATION | 0x138 | 0x450 | 0xB8C | 49 / 49 / 49 | 7 | 0 | F | none | 49 / 49 |
| OBSERVATION | 0x139 | 0x45A | 0xBBD | 128 / 118 / 136 | 8 | 9 | A | n=0; w=2 | 128 / 118 |
| OBSERVATION | 0x13A | 0x464 | 0xC3D | 61 / 53 / 53 | 9 | 4 | B | q=0 | 61 / 53 |
| OBSERVATION | 0x13B | 0x46E | 0xC7A | 126 / 116 / 125 | 10 | 1 | A | n=0; w=8 | 126 / 116 |
| OBSERVATION | 0x13C | 0x478 | 0xCF8 | 86 / 78 / 78 | 11 | 4 | B | q=0 | 86 / 78 |
| OBSERVATION | 0x102 | 0x482 | 0xD4E | 34 / 10 / 10 | 67 | 9 | A | n=7; w=2 | 34 / 10 |
| OBSERVATION | 0x103 | 0x48C | 0xD70 | 30 / 8 / 8 | 49 | 2 | C | n=0; w=4 | 30 / 8 |
| OBSERVATION | 0x11E | 0x496 | 0xD8E | 26 / 4 / 4 | 50 | 3 | C/O | n=0; q=4; offsets=4 | 26 / 4 |
| OBSERVATION | 0x105 | 0x4A0 | 0xDA8 | 26 / 4 / 4 | 31 | 3 | C/O | n=0; q=4; offsets=4 | 26 / 4 |
| OBSERVATION | 0x101 | 0x4AA | 0xDC2 | 30 / 8 / 8 | 49 | 2 | C | n=0; w=4 | 30 / 8 |
| OBSERVATION | 0x104 | 0x4B4 | 0xDE0 | 26 / 4 / 4 | 50 | 3 | C/O | n=0; q=4; offsets=4 | 26 / 4 |
| OBSERVATION | 0x131 | 0x4BE | 0xDFA | 14 / 4 / 4 | 55 | 1 | A | n=0; w=6 | 14 / 4 |
| OBSERVATION | 0x132 | 0x4C8 | 0xE08 | 14 / 4 / 4 | 56 | 1 | A | n=0; w=10 | 14 / 4 |
| OBSERVATION | 0x13D | 0x4D2 | 0xE16 | 26 / 4 / 4 | 92 | ? | C/O | n=0; q=4; offsets=4 | 26 / 4 |
| OBSERVATION | 0x124 | 0x4DC | 0xE30 | 56 / 56 / 88 | 57 | 0 | F | none | 56 / 56 |
| OBSERVATION | 0x11D | 0x4E6 | 0xE68 | 192 / 150 / 218 | 71 | 1 | A | n=8; w=4 | 192 / 150 |
| OBSERVATION | 0x11A | 0x4F0 | 0xF28 | 14 / 14 / 70 | 69 | 0 | F | none | 8 / 8 |
| OBSERVATION | 0x11B | 0x4FA | 0xF36 | 70 / 18 / 18 | 76 | 10 | C | n=5; w=2 | 70 / 18 |
| OBSERVATION | 0x11C | 0x504 | 0xF7C | 244 / 18 / 18 | 82 | 2 | C | n=6; w=30 | 244 / 18 |
| OBSERVATION | 0x106 | 0x50E | 0x1070 | 40 / 4 / 4 | 51 | 4 | B | q=28 | 40 / 4 |
| OBSERVATION | 0x117 | 0x518 | 0x1098 | 42 / 32 / 126 | 20 | 25 | A | n=0; w=2 | 40 / 30 |
| OBSERVATION | 0x107 | 0x522 | 0x10C2 | 42 / 32 / 126 | 20 | 25 | A | n=0; w=2 | 40 / 30 |
| OBSERVATION | 0x121 | 0x52C | 0x10EC | 58 / 24 / 24 | 44 | 1 | A | n=4; w=6 | 58 / 24 |
| OBSERVATION | 0x108 | 0x536 | 0x1126 | 22 / 4 / 4 | 51 | 4 | B | q=10 | 22 / 4 |
| OBSERVATION | 0x10A | 0x540 | 0x113C | 46 / 32 / 126 | 20 | 25 | A | n=2; w=2 | 44 / 30 |
| OBSERVATION | 0x10B | 0x54A | 0x116A | 16 / 4 / 4 | 51 | 4 | B | q=4 | 16 / 4 |
| OBSERVATION | 0x125 | 0x554 | 0x117A | 128 / 118 / 136 | 8 | 9 | A | n=0; w=2 | 128 / 118 |
| OBSERVATION | 0x126 | 0x55E | 0x11FA | 128 / 118 / 136 | 8 | 9 | A | n=0; w=2 | 128 / 118 |
| OBSERVATION | 0x127 | 0x568 | 0x127A | 52 / 32 / 126 | 20 | 25 | A | n=5; w=2 | 50 / 30 |
| OBSERVATION | 0x128 | 0x572 | 0x12AE | 22 / 4 / 4 | 51 | 4 | B | q=10 | 22 / 4 |
| OBSERVATION | 0x12A | 0x57C | 0x12C4 | 128 / 118 / 136 | 8 | 9 | A | n=0; w=2 | 128 / 118 |
| OBSERVATION | 0x12B | 0x586 | 0x1344 | 166 / 116 / 125 | 10 | 1 | A | n=5; w=8 | 166 / 116 |
| OBSERVATION | 0x141 | 0x590 | 0x13EA | 290 / 10 / 10 | 78 | 1 | A | n=6; w=45 | 290 / 10 |
| OBSERVATION | 0x140 | 0x59A | 0x150C | 86 / 10 / 10 | 79 | 1 | A | n=6; w=11 | 86 / 10 |
| OBSERVATION | 0x13F | 0x5A4 | 0x1562 | 50 / 10 / 10 | 80 | 1 | A | n=6; w=5 | 50 / 10 |
| OBSERVATION | 0x12E | 0x5AE | 0x1594 | 70 / 70 / 70 | 5 | 0 | F | none | 70 / 70 |
| OBSERVATION | 0x12F | 0x5B8 | 0x15DA | 128 / 118 / 136 | 8 | 9 | A | n=0; w=2 | 128 / 118 |
| OBSERVATION | 0x130 | 0x5C2 | 0x165A | 128 / 118 / 136 | 8 | 9 | A | n=0; w=2 | 128 / 118 |
| OBSERVATION | 0x10D | 0x5CC | 0x16DA | 42 / 32 / 126 | 20 | 25 | A | n=0; w=2 | 40 / 30 |
| OBSERVATION | 0x10E | 0x5D6 | 0x1704 | 22 / 4 / 4 | 51 | 4 | B | q=10 | 22 / 4 |
| OBSERVATION | 0x110 | 0x5E0 | 0x171A | 42 / 32 / 126 | 20 | 25 | A | n=0; w=2 | 40 / 30 |
| OBSERVATION | 0x111 | 0x5EA | 0x1744 | 22 / 4 / 4 | 51 | 4 | B | q=10 | 22 / 4 |
| OBSERVATION | 0x113 | 0x5F4 | 0x175A | 42 / 32 / 126 | 20 | 25 | A | n=0; w=2 | 40 / 30 |
| OBSERVATION | 0x114 | 0x5FE | 0x1784 | 22 / 4 / 4 | 51 | 4 | B | q=10 | 22 / 4 |

OBSERVATION: across the eight index-20 records, the 17 two-byte tail IDs beginning at T+10 all resolve to directory IDs whose parent candidate is the source record (17/17). This corroborates the shifted H-based tail location and the directory-parent hypothesis. UNKNOWN: the other B selector families, complete reference graph, and semantic ownership roles; no A selector convention is automatically promoted to B.

## Quill text anchors and controls

OBSERVATION: a bounded linked-section-directory walk in the 6144-byte Quill stream finds TEXT at 0x200 of length 976, SYID at 0xC00 of length 32, and STRS at 0x11FE of length 82. Their 24-byte directory entries are at 0x20, 0xB0 and 0x188. Each section's offset+length stays within the stream. SYID count at 0xC04 and STRS count at 0x11FE both equal six. IDs are u32 from 0xC08; lengths are u32 from 0x120A, selected by STRS's relative displacement. Six lengths sum to 488 UTF-16 units, exactly the 976 TEXT bytes; each story ends in a CR unit. The TEXT bytes equal Sample98's, without reproducing text content here.

| Basis | Record / index | Contents record start | Relative link / absolute position | Quill ID | ID position in Quill | Length position in Quill | UTF-16 range / length |
| --- | --- | --- | --- | ---: | --- | --- | --- |
| OBSERVATION | 0x125 / 8 | 0x117A | +0x58 / 0x11D2 | 1 | 0xC08 | 0x120A | [0,86) / 86 |
| OBSERVATION | 0x126 / 8 | 0x11FA | +0x58 / 0x1252 | 3 | 0xC0C | 0x120E | [86,184) / 98 |
| OBSERVATION | 0x12A / 8 | 0x12C4 | +0x58 / 0x131C | 4 | 0xC10 | 0x1212 | [184,246) / 62 |
| OBSERVATION | 0x12B / 10 | 0x1344 | +0x66 / 0x13AA | 6 | 0xC14 | 0x1216 | [246,326) / 80 |
| OBSERVATION | 0x12F / 8 | 0x15DA | +0x58 / 0x1632 | 7 | 0xC18 | 0x121A | [326,413) / 87 |
| OBSERVATION | 0x130 / 8 | 0x165A | +0x58 / 0x16B2 | 8 | 0xC1C | 0x121E | [413,488) / 75 |

OBSERVATION: index 8 at +0x58 matches five distinct positive story IDs in five records, and is zero in inactive candidate 0x139. Index 10 at +0x66 matches ID 6 in 0x12B and is zero in 0x13B. All eight bounded fields have zero upper two bytes, so u16 and u32 reads agree; their authoritative stored width remains UNKNOWN. The Quill IDs themselves are u32.

| Basis | Cohort | Anchor hits / positive records | Zero controls | Other aligned u16 fixed-prefix fields hitting any story ID | Same-offset discrimination |
| --- | --- | --- | --- | --- | --- |
| OBSERVATION | Index 8 | 5/5 | 1/1 | 18/336 | Only +0x58 matches each record's assigned ID across all five. Other matches: +0x26, +0x56, +0x62 each match only one of five. |
| OBSERVATION | Index 10 | 1/1 | 1/1 | 6/110 | Only +0x66 matches its positive ID within the tested aligned fixed-prefix window; one positive instance is weaker evidence. |

OBSERVATION: the controls scan aligned u16 fields from +4 through H-2, excluding the anchor; no random independence or statistical p-value is assumed. Small scalar ID coincidences occur in the control pool, so ID equality alone is insufficient. The six distinct IDs, cohort-specific common offsets, zero controls, Quill partition checks and preserved A comparison jointly support the links. Lengths and ranges in the table are derived from Quill; no adjacent Contents-side start/length pair is proven by this step.

INFERENCE: these six B records carry Quill story links in the same positions as the positive A examples. Their directory-parent chains also match: 0x125/0x126 -> 0x10A -> 0x100; 0x12A/0x12B/0x12F/0x130 -> 0x127 -> 0x100. This is a single-sample B association, not proof of frame meaning, pages or reading order. No geometry, formatting, pictures or import-failure repair is attempted.

## Ranked UNKNOWNs and next smallest step

1. UNKNOWN: B's exact terminator/tag encoding, selector payload width and leading-prefix/index-1 convention. OBSERVATION: AP17 bounds 92 ordinary entries and corroborates 54/54 instantiated H/class lookups; AP18's descriptor-8/10 projections are bounded in 14/14 probes. AP19 tests descriptor-2/3 word 0x0772: +114 is bounded in 2/2 records but zero in both, while +370 is outside both independently predicted 150-byte spans (0/2 bounds passes). Controls contain 110/134 zeros and no directory-parent matches. INFERENCE: reject +370 as an unconditional live field in these records; do not choose a mask merely because +114 fits. The exact encoding and applicability remain UNKNOWN. The next decisive reader-placement step is an independently documented physical page/frame measurement, not another zero-only selector test.
2. UNKNOWN: nonempty class-67 framing, the correct interpretation of the four ambiguous empty tails, count-versus-capacity roles, internal padding and general extent semantics. OBSERVATION: AP17 identifies index 92's sampled class word as 67; its empty tail fits the O equation but cannot discriminate alternatives. No global single rule or cross-file B replication is established.
3. UNKNOWN: general B selector references, complete reference graph, parent semantics and attribute-bit meanings. OBSERVATION: AP18 adds three parent-corroborated projected descriptor-10 child links from 0x12B at +106/+108/+110 to the 17 index-20 tail IDs. Each family has only one positive instance; +32 remains zero-only. Control pools give 3/432 directory-ID coincidences and 0/432 source-parent matches.
4. UNKNOWN: authoritative Contents-side story-ID width, stored text ranges, page identity/order and visual reading order. OBSERVATION: AP18 joins all six story links to distinct candidate rectangles in the same records and verifies their collection-tail memberships, with no positive failures and two inactive controls. UNKNOWN: physical scale/origin and actual displayed placement; a known physical page and unrotated frame rectangle is needed to separate scale from translation. Alternative origins/sizes and other-class numerical positives prevent promotion to faithful geometry semantics.
5. UNKNOWN: saving version and wider coverage. Sample2000 is the sole B file currently available; A geometry inferences and other untested class semantics are not generalized to B.

INFERENCE: a bounded experimental walker can enumerate the 54 directory spans and preserve unknown records. A production reader still needs the B descriptor checkpoint before treating the transferred class mapping as a decoded format rule. OBSERVATION: this is the first checkpoint with a complete directory census, 54/54 arithmetic fits and six controlled text links; research stops here.

## Descriptor framing and page/frame candidates

OBSERVATION: AP17 is single-sample B research on the same pinned Sample2000.pub identity listed above, rechecked before these experiments. Only its root Contents and existing own A/B notes supply evidence; no new input, specification, scoring export or third-party format source is used. All B results in this section, including every table row, are single-sample. They do not establish a general B encoding or independent visual placement. No importer or environment setting changed; no full CI was run.

### Index-8 and index-10 entry bounds

OBSERVATION: a reproducible framing candidate starts ordinary entries at 0x26 with index label 2. Each ordinary entry has three little-endian u16 words (six bytes), followed by two-byte words through the first word whose low nine bits are zero. For an entry at P with k nonterminating tail words, its measured width is 6+2*(k+1), and its end is P plus that width. All reads are bounded by the header control pointer 0x3BA, not by a guessed table length. The next entry starts at that end. Starting labels at 2 is tested independently against the 54 record indices and their H bytes below, rather than copied from A.

| Basis | Entry | Start | Fixed fields (offset: value) | Nonterminating tail words (offset: raw value) | Terminating word | k / width / end |
| --- | ---: | --- | --- | --- | --- | --- |
| OBSERVATION | 8 | 0x72 | 0x72:0x7692; 0x74:9; 0x76:2 | 0x78:0x0620 | u16 0 at 0x7A | 1 / 10 / 0x7C |
| OBSERVATION | 10 | 0x86 | 0x86:0x7487; 0x88:1; 0x8A:8 | 0x8C:0x066A; 0x8E:0x006C; 0x90:0x066E; 0x92:0xA620 | u16 0 at 0x94 | 4 / 16 / 0x96 |

OBSERVATION: for entry 8, word-0 high byte 118 equals H in all six index-8 records; for entry 10, high byte 116 equals H in both index-10 records. All field widths here are two bytes. Entry width is found by the bounded tail scan; neither word-0 low byte 146/135 nor high byte 118/116 is the serialized table-entry length.

UNKNOWN: the meaning of tail-word high bits and the low-bit payload is not decoded by framing. Applying mask 0x00FF instead of 0x01FF produces exactly the same boundaries and tails for all 92 ordinary entries. The file therefore cannot discriminate those two termination predicates. This is an observed boundary rule, not proof that every selector is a nine-bit offset. Masking and then interpreting values as references is a separate unperformed semantic test.

### Complete table test and exceptions

OBSERVATION: the scan consumes 92/92 ordinary entries without an out-of-bounds read or leftover byte, ending exactly at 0x3BA. Labels run 2..93, compatible with the leading u16 94 at 0x22 as a one-past bound. Every one of the 54 instantiated record indices resolves, and descriptor word-0 high byte equals record H in 54/54. No ordinary entry fails this candidate framing. Terminators are raw 0x0000 in 19 entries, 0x0600 in 70, 0x0800 in two and 0xA600 in one; treating only literal zero as termination gives the failed AP16 walk instead. Thus literal-zero termination is refuted for this whole-table walk.

UNKNOWN: the leading u16 6 at 0x24 is not a six-byte-header ordinary entry. It remains a separate two-byte prefix between count and first ordinary entry. A putative index 1 has no instantiated record here, and whether that word represents a short/omitted slot, metadata or a different convention cannot be determined. A uniform ordinary-entry rule covers 92 entries, not 93 ordinary entries; index 1 is the sole unresolved slot if 94 means one-past indices 1..93. No contents or length is invented for it, and index 0's convention is also not proven. The complete measured region is two-byte count + two-byte prefix + 916 ordinary-entry bytes = 920 bytes, from 0x22 to 0x3BA.

OBSERVATION: the following census lists every ordinary entry's start, end and total width, plus its three fixed words. Bounds are reproduced by the scan, not supplied as an offset list to it. k is the number of nonterminating tail words; fixed word meanings other than sampled H/class correlations remain limited. All rows are single-sample B observations.

| Basis | Index | Start | End | Bytes | k | Word 0 | Word 1 | Word 2 | Terminator |
| --- | ---: | --- | --- | ---: | ---: | --- | ---: | ---: | --- |
| OBSERVATION | 2 | 0x26 | 0x38 | 18 | 5 | 0x8CC6 | 1 | 8 | 0x800 |
| OBSERVATION | 3 | 0x38 | 0x4A | 18 | 5 | 0x8CC8 | 1 | 8 | 0x800 |
| OBSERVATION | 4 | 0x4A | 0x54 | 10 | 1 | 0x454F | 0 | 0 | 0x0 |
| OBSERVATION | 5 | 0x54 | 0x5E | 10 | 1 | 0x4650 | 0 | 0 | 0x0 |
| OBSERVATION | 6 | 0x5E | 0x68 | 10 | 1 | 0x3D4B | 0 | 0 | 0x0 |
| OBSERVATION | 7 | 0x68 | 0x72 | 10 | 1 | 0x313B | 0 | 0 | 0x0 |
| OBSERVATION | 8 | 0x72 | 0x7C | 10 | 1 | 0x7692 | 9 | 2 | 0x0 |
| OBSERVATION | 9 | 0x7C | 0x86 | 10 | 1 | 0x353F | 4 | 0 | 0x0 |
| OBSERVATION | 10 | 0x86 | 0x96 | 16 | 4 | 0x7487 | 1 | 8 | 0x0 |
| OBSERVATION | 11 | 0x96 | 0xA0 | 10 | 1 | 0x4E58 | 4 | 0 | 0x0 |
| OBSERVATION | 12 | 0xA0 | 0xA8 | 8 | 0 | 0x0 | 0 | 0 | 0x600 |
| OBSERVATION | 13 | 0xA8 | 0xB0 | 8 | 0 | 0x0 | 0 | 0 | 0x600 |
| OBSERVATION | 14 | 0xB0 | 0xBC | 12 | 2 | 0x242E | 25 | 2 | 0x600 |
| OBSERVATION | 15 | 0xBC | 0xC6 | 10 | 1 | 0x222C | 25 | 2 | 0x0 |
| OBSERVATION | 16 | 0xC6 | 0xCE | 8 | 0 | 0x0 | 0 | 0 | 0x600 |
| OBSERVATION | 17 | 0xCE | 0xD6 | 8 | 0 | 0x404 | 1 | 0 | 0x600 |
| OBSERVATION | 18 | 0xD6 | 0xDE | 8 | 0 | 0x404 | 2 | 0 | 0x600 |
| OBSERVATION | 19 | 0xDE | 0xE6 | 8 | 0 | 0x404 | 3 | 0 | 0x600 |
| OBSERVATION | 20 | 0xE6 | 0xF6 | 16 | 4 | 0x207E | 25 | 2 | 0x0 |
| OBSERVATION | 21 | 0xF6 | 0x13E | 72 | 32 | 0xB2E8 | 25 | 2 | 0x0 |
| OBSERVATION | 22 | 0x13E | 0x156 | 24 | 8 | 0x121C | 0 | 0 | 0x600 |
| OBSERVATION | 23 | 0x156 | 0x15E | 8 | 0 | 0x0 | 0 | 0 | 0x600 |
| OBSERVATION | 24 | 0x15E | 0x166 | 8 | 0 | 0x404 | 2 | 8 | 0x600 |
| OBSERVATION | 25 | 0x166 | 0x16E | 8 | 0 | 0x404 | 2 | 16 | 0x600 |
| OBSERVATION | 26 | 0x16E | 0x176 | 8 | 0 | 0x404 | 10 | 10 | 0x600 |
| OBSERVATION | 27 | 0x176 | 0x17E | 8 | 0 | 0x404 | 26 | 2 | 0x600 |
| OBSERVATION | 28 | 0x17E | 0x186 | 8 | 0 | 0x404 | 2 | 4 | 0x600 |
| OBSERVATION | 29 | 0x186 | 0x18E | 8 | 0 | 0x404 | 2 | 2 | 0x600 |
| OBSERVATION | 30 | 0x18E | 0x196 | 8 | 0 | 0x408 | 3 | 198 | 0x600 |
| OBSERVATION | 31 | 0x196 | 0x19E | 8 | 0 | 0x404 | 3 | 0 | 0x600 |
| OBSERVATION | 32 | 0x19E | 0x1A6 | 8 | 0 | 0x404 | 3 | 0 | 0x600 |
| OBSERVATION | 33 | 0x1A6 | 0x1AE | 8 | 0 | 0x800 | 0 | 0 | 0x600 |
| OBSERVATION | 34 | 0x1AE | 0x1B6 | 8 | 0 | 0x800 | 0 | 0 | 0x600 |
| OBSERVATION | 35 | 0x1B6 | 0x1BE | 8 | 0 | 0x404 | 3 | 0 | 0x600 |
| OBSERVATION | 36 | 0x1BE | 0x1C6 | 8 | 0 | 0x76 | 1 | 128 | 0x600 |
| OBSERVATION | 37 | 0x1C6 | 0x1CE | 8 | 0 | 0x36 | 1 | 64 | 0x600 |
| OBSERVATION | 38 | 0x1CE | 0x1D6 | 8 | 0 | 0x404 | 67 | 50 | 0x600 |
| OBSERVATION | 39 | 0x1D6 | 0x1E0 | 10 | 1 | 0x1616 | 4 | 0 | 0x0 |
| OBSERVATION | 40 | 0x1E0 | 0x1E8 | 8 | 0 | 0x1C1C | 3 | 0 | 0x600 |
| OBSERVATION | 41 | 0x1E8 | 0x1F2 | 10 | 1 | 0x3E44 | 195 | 0 | 0x0 |
| OBSERVATION | 42 | 0x1F2 | 0x1FA | 8 | 0 | 0x404 | 3 | 0 | 0x600 |
| OBSERVATION | 43 | 0x1FA | 0x202 | 8 | 0 | 0x404 | 3 | 0 | 0x600 |
| OBSERVATION | 44 | 0x202 | 0x20A | 8 | 0 | 0x1818 | 1 | 6 | 0x600 |
| OBSERVATION | 45 | 0x20A | 0x212 | 8 | 0 | 0x404 | 2 | 4 | 0x600 |
| OBSERVATION | 46 | 0x212 | 0x21A | 8 | 0 | 0x404 | 2 | 6 | 0x600 |
| OBSERVATION | 47 | 0x21A | 0x224 | 10 | 1 | 0x808 | 2 | 4 | 0x0 |
| OBSERVATION | 48 | 0x224 | 0x22C | 8 | 0 | 0x404 | 3 | 0 | 0x600 |
| OBSERVATION | 49 | 0x22C | 0x236 | 10 | 1 | 0x808 | 2 | 4 | 0x0 |
| OBSERVATION | 50 | 0x236 | 0x23E | 8 | 0 | 0x404 | 3 | 0 | 0x600 |
| OBSERVATION | 51 | 0x23E | 0x246 | 8 | 0 | 0x404 | 4 | 0 | 0x600 |
| OBSERVATION | 52 | 0x246 | 0x24E | 8 | 0 | 0x2424 | 1 | 78 | 0x600 |
| OBSERVATION | 53 | 0x24E | 0x256 | 8 | 0 | 0xC8C8 | 8 | 0 | 0x600 |
| OBSERVATION | 54 | 0x256 | 0x25E | 8 | 0 | 0x90AC | 0 | 14 | 0x600 |
| OBSERVATION | 55 | 0x25E | 0x266 | 8 | 0 | 0x404 | 1 | 6 | 0x600 |
| OBSERVATION | 56 | 0x266 | 0x26E | 8 | 0 | 0x404 | 1 | 10 | 0x600 |
| OBSERVATION | 57 | 0x26E | 0x276 | 8 | 0 | 0x3858 | 0 | 0 | 0x600 |
| OBSERVATION | 58 | 0x276 | 0x27E | 8 | 0 | 0x404 | 3 | 0 | 0x600 |
| OBSERVATION | 59 | 0x27E | 0x286 | 8 | 0 | 0x404 | 3 | 0 | 0x600 |
| OBSERVATION | 60 | 0x286 | 0x292 | 12 | 2 | 0x193F | 35 | 0 | 0x600 |
| OBSERVATION | 61 | 0x292 | 0x29C | 10 | 1 | 0x1A1A | 4 | 0 | 0x0 |
| OBSERVATION | 62 | 0x29C | 0x2A4 | 8 | 0 | 0x8 | 3 | 0 | 0x600 |
| OBSERVATION | 63 | 0x2A4 | 0x2AC | 8 | 0 | 0x404 | 4 | 306 | 0x600 |
| OBSERVATION | 64 | 0x2AC | 0x2BC | 16 | 4 | 0x1C1D | 0 | 0 | 0x0 |
| OBSERVATION | 65 | 0x2BC | 0x2CC | 16 | 4 | 0x207E | 25 | 8 | 0x0 |
| OBSERVATION | 66 | 0x2CC | 0x2D4 | 8 | 0 | 0x1212 | 35 | 0 | 0x600 |
| OBSERVATION | 67 | 0x2D4 | 0x2DC | 8 | 0 | 0xA0A | 9 | 8 | 0x600 |
| OBSERVATION | 68 | 0x2DC | 0x2E4 | 8 | 0 | 0xA0A | 9 | 8 | 0x600 |
| OBSERVATION | 69 | 0x2E4 | 0x2F4 | 16 | 4 | 0xE46 | 0 | 0 | 0x0 |
| OBSERVATION | 70 | 0x2F4 | 0x2FC | 8 | 0 | 0x808 | 3 | 0 | 0x600 |
| OBSERVATION | 71 | 0x2FC | 0x30A | 14 | 3 | 0x96DA | 1 | 4 | 0xA600 |
| OBSERVATION | 72 | 0x30A | 0x312 | 8 | 0 | 0x1212 | 3 | 0 | 0x600 |
| OBSERVATION | 73 | 0x312 | 0x31A | 8 | 0 | 0x1212 | 3 | 0 | 0x600 |
| OBSERVATION | 74 | 0x31A | 0x322 | 8 | 0 | 0x1212 | 3 | 0 | 0x600 |
| OBSERVATION | 75 | 0x322 | 0x32A | 8 | 0 | 0x404 | 4 | 0 | 0x600 |
| OBSERVATION | 76 | 0x32A | 0x332 | 8 | 0 | 0x1212 | 10 | 2 | 0x600 |
| OBSERVATION | 77 | 0x332 | 0x33A | 8 | 0 | 0x1212 | 2 | 22 | 0x600 |
| OBSERVATION | 78 | 0x33A | 0x342 | 8 | 0 | 0xA0A | 1 | 45 | 0x600 |
| OBSERVATION | 79 | 0x342 | 0x34A | 8 | 0 | 0xA0A | 1 | 11 | 0x600 |
| OBSERVATION | 80 | 0x34A | 0x352 | 8 | 0 | 0xA0A | 1 | 5 | 0x600 |
| OBSERVATION | 81 | 0x352 | 0x35A | 8 | 0 | 0x404 | 4 | 0 | 0x600 |
| OBSERVATION | 82 | 0x35A | 0x362 | 8 | 0 | 0x1212 | 2 | 30 | 0x600 |
| OBSERVATION | 83 | 0x362 | 0x36A | 8 | 0 | 0xA0A | 9 | 8 | 0x600 |
| OBSERVATION | 84 | 0x36A | 0x372 | 8 | 0 | 0x1616 | 4 | 0 | 0x600 |
| OBSERVATION | 85 | 0x372 | 0x37A | 8 | 0 | 0x808 | 4 | 0 | 0x600 |
| OBSERVATION | 86 | 0x37A | 0x382 | 8 | 0 | 0x808 | 4 | 0 | 0x600 |
| OBSERVATION | 87 | 0x382 | 0x38A | 8 | 0 | 0x1010 | 0 | 0 | 0x600 |
| OBSERVATION | 88 | 0x38A | 0x392 | 8 | 0 | 0x404 | 4 | 0 | 0x600 |
| OBSERVATION | 89 | 0x392 | 0x39A | 8 | 0 | 0x404 | 4 | 0 | 0x600 |
| OBSERVATION | 90 | 0x39A | 0x3A2 | 8 | 0 | 0x404 | 3 | 0 | 0x600 |
| OBSERVATION | 91 | 0x3A2 | 0x3AA | 8 | 0 | 0x404 | 3 | 0 | 0x600 |
| OBSERVATION | 92 | 0x3AA | 0x3B2 | 8 | 0 | 0x404 | 67 | 0 | 0x600 |
| OBSERVATION | 93 | 0x3B2 | 0x3BA | 8 | 0 | 0x1414 | 3 | 0 | 0x600 |

OBSERVATION: word 1 agrees with the previous A-class hypothesis for all 53 previously mapped B records (53/53). Index 92 at [0x3AA,0x3B2) instead has fixed words 0x0404, 67, 0; its sole record 0x13D has H=4 and span=26. Thus the previously unknown sampled class code is 67, not silently class 3. Index 92's class-67 offset-table candidate uses n=0, repeated n=0, q=4 and one offset 4 at T+18, predicting 4+18+4=26. UNKNOWN: nonempty class-67 behavior; its empty record cannot discriminate C versus O arithmetic. The class label and empty-tail equation are separate conclusions.

| Basis | Descriptor word-1 class | Records | Equation checked | Exact endpoints |
| --- | --- | ---: | --- | ---: |
| OBSERVATION | 0 | 8 | H | 8/8 |
| OBSERVATION | 1 / 9 / 25 | 11 / 7 / 9 | H+10+n*w | 27/27 |
| OBSERVATION | 4 | 9 | H+8+q | 9/9 |
| OBSERVATION | 2 / 10 | 3 / 1 | H+22+n*(4+w) | 4/4 |
| OBSERVATION | 3 / 195 | 4 / 1 | H+18+q with stored offset table | 5/5 |
| INFERENCE | 67 | 1 | Candidate H+18+q; ambiguous empty tail | 1/1 |

OBSERVATION: the table lookup now corroborates H/class for all 54 instantiated records; equations selected by those class codes yield 54/54 endpoints, including the explicitly single-instance class-67 inference. This narrows AP16's 0/54 B-descriptor-selected count: 53 have class codes matching previously tested class rules; one has a newly observed class code with an empty-tail candidate. It does not resolve the three other empty-tail arithmetic coincidences or prove all uninstantiated descriptor semantics.

### Page-size and rectangle candidates

| Basis | Candidate | Contents evidence | Result |
| --- | --- | --- | --- |
| OBSERVATION | Root ID 0x100, index 21, R=0x612 | LE u32 at R+0x14 = 0x626: 7560000; R+0x18 = 0x62A: 10692000; both within fixed prefix H=178 | Same size pair as Sample98. |
| INFERENCE | Physical unit | Dividing the two stored values by 36000 yields exactly 210 and 297 | A4 at candidate 36000 units/mm (12700/pt, 914400/in). Exact numerical fit, not independent physical-size proof. |
| OBSERVATION | Duplicate pair | Index-44 ID 0x121 at R=0x10EC, u32 R+4/R+8 at 0x10F0/0x10F4 | Same two values. |
| OBSERVATION | Standard-pair false-positive control | Full byte-aligned search for the Letter pair 7772400/10058400 finds it at 0x752 and 0x7BA, inside index-41/class-195 record 0x11F | A standard-looking pair elsewhere does not prove page dimensions. |

OBSERVATION: the A rectangle probe exists at exactly the same record-relative offsets in B: signed LE i32 at +0x06/+0x0A/+0x0E/+0x12, candidate left/top/right/bottom in a centered coordinate system. All four fields fit within each index-8/index-10 fixed prefix. Candidate page coordinates add W/2 to horizontal endpoints and H/2 to vertical endpoints. Checked using doubled wide integers, containment is 0 <= 2*l+W < 2*r+W <= 2*W and 0 <= 2*t+H < 2*b+H <= 2*H. This is a mathematical containment test, not independent validation of that origin.

| Basis | ID / index | Absolute field offsets | Raw signed endpoints | Candidate translated endpoints |
| --- | --- | --- | --- | --- |
| OBSERVATION | 0x125 / 8 | 0x1180, 0x1184, 0x1188, 0x118C | -2700000, -4446000, 2376000, -3474000 | 1080000, 900000, 6156000, 1872000 |
| OBSERVATION | 0x126 / 8 | 0x1200, 0x1204, 0x1208, 0x120C | -2700000, -3294000, 2484000, -2214000 | 1080000, 2052000, 6264000, 3132000 |
| OBSERVATION | 0x12A / 8 | 0x12CA, 0x12CE, 0x12D2, 0x12D6 | -2952000, -4410000, 2412000, -2862000 | 828000, 936000, 6192000, 2484000 |
| OBSERVATION | 0x12B / 10 | 0x134A, 0x134E, 0x1352, 0x1356 | -2412000, -2538000, 1152000, -1880436 | 1368000, 2808000, 4932000, 3465564 |
| OBSERVATION | 0x12F / 8 | 0x15E0, 0x15E4, 0x15E8, 0x15EC | -2376000, -90000, 2016000, 594000 | 1404000, 5256000, 5796000, 5940000 |
| OBSERVATION | 0x130 / 8 | 0x1660, 0x1664, 0x1668, 0x166C | -2340000, 882000, 2196000, 2610000 | 1440000, 6228000, 5976000, 7956000 |

OBSERVATION: all six positive story-linked records have positive width/height, lie inside the candidate page after that translation and have distinct rectangles (6/6 each). No one of these six candidate frames lies outside it. Raw endpoint values equal the six Sample98 values. Inactive index-8 ID 0x139 and index-10 ID 0x13B both yield four zeros (2/2 negative controls), not live frame rectangles. There is no need to move the candidate fields elsewhere in this sample; physical field meanings remain inferred.

| Basis | Geometry control, single-sample B | Result |
| --- | --- | --- |
| OBSERVATION | All common byte-aligned four-i32 windows wholly inside the six fixed prefixes, offsets 0..100 | Only start +0x06 gives six positive, contained rectangles; 1/101 common window. |
| OBSERVATION | Same four offsets with 16-bit unsigned/signed, 32-bit unsigned/signed, 64-bit unsigned/signed reads | Containment passes 2/6, 2/6, 0/6, 6/6, 0/6, 0/6 respectively. |
| OBSERVATION | Treat raw coordinates as page-top-left endpoints without translation | 0/6 contained. |
| OBSERVATION | Treat the last two raw values as width/height | Only 2/6 have positive width and height. |
| OBSERVATION | Same offsets in other indices, requiring all 16 bytes inside candidate fixed prefix | 24 eligible records, 2/24 pass: ID 0x123/index 40/class 3 and ID 0x12E/index 5/class 0. These are false-positive controls, not newly decoded frames. |
| OBSERVATION | Permit the probe to cross the candidate fixed boundary but remain within the record span | 42 eligible other records; 3/42 pass, adding ID 0x141/index 78/class 1. This is an unsafe semantic relaxation despite bounded bytes. |
| OBSERVATION | Shift the candidate origin by another +36000 units on both axes | Still 6/6 contained; containment does not uniquely fix origin. |
| OBSERVATION | Use the Letter size instead of the root A4 pair, retaining half-page translation | Still 6/6 contained; frame containment does not independently prove page size. |

OBSERVATION: fixed-prefix controls by class are class 0: 1/7, class 1: 0/4, class 3: 1/1, class 4: 0/2, class 25: 0/9, class 195: 0/1; other instantiated classes have zero eligible other-index controls. The counts exclude the eight index-8/index-10 candidates. The feasible horizontal translation interval is [2952000,5076000], vertical [4446000,8082000], so multiple origins fit. UNKNOWN: physical scale, centered-origin semantics, rotations and actual displayed placement remain unverified. Numerical positives in other classes prohibit recognizing frames from this window alone.

### Replication requirements and checkpoint

INFERENCE: a second B file should first meet the independently bounded header/directory checks, then reproduce ordinary-entry framing and index/H/class correlations while varying its descriptor tails. To distinguish termination masks, it must contain a tail word with low byte zero and bit 8 set, or other evidence that separates zero-low-eight from zero-low-nine. To resolve the leading prefix/index-1 convention it should instantiate record index 1 or vary the leading word/table population. A nonempty class-67 record is needed to distinguish its candidate extent rule from the empty-tail coincidences.

INFERENCE: positive geometry replication needs a second B file with an independently known physical page size and one unrotated text frame at documented, noncentral positions, ideally a different size and layout. The exact root size, signed rectangle probes and translation must predict that known placement before scale/origin semantics are promoted. Matching Sample2000's copied numerical values or merely obtaining contained boxes is insufficient.

OBSERVATION: this checkpoint proves a reproducible whole-region framing candidate for 92 ordinary entries, 54/54 instantiated H correlations, sampled class codes including 67, and the unchanged geometry probes' numerical results. UNKNOWN: the prefix/unused slot, exact termination mask and tail tags, selector interpretation, general class-67 framing and physical geometry meaning. Research stops here; no importer change or full CI is made.

## Selector projections, story-to-rectangle joins and placement limits

OBSERVATION: AP18 reuses Sample2000.pub and the own A/B notes already listed in SOURCES.md; its pinned identity and the preceding bounded checks were rerun. All B findings and table rows in this section are single-sample. No new source, specification, scoring export or other format reader is used. Only derived facts are committed. No importer, environment setting or existing script is changed and no full CI is run.

### Descriptor-8/10 projection tests

OBSERVATION: descriptor 8 has raw tail word 0x0620; descriptor 10 has 0x066A, 0x006C, 0x066E, 0xA620. Low-byte projection yields record-relative offsets 32 and 106/108/110/32 respectively. Low-seven and low-nine projections give exactly the same values here, so this test does not identify a universal payload width or tag mask. For each projected field, require offset+2 <= H and R+H <= directory-derived end before a LE u16 read. All 14/14 instantiated projected reads satisfy both bounds.

OBSERVATION: using the literal full raw word or low-ten-bit projection instead leaves only 2/14 reads bounded (descriptor 10's unchanged 0x006C in two records); twelve are outside the fixed prefix. For the three positive projected matches below, moving the base to R+H or R+H+10 puts all three reads outside their source record. These controls support record-start-relative small offsets for those matches, not an unconditional relocation rule for every descriptor.

| Basis | Descriptor / raw word | Source record | Projected relative / absolute offset | u16 value | Directory target / parent check |
| --- | --- | --- | --- | --- | --- |
| OBSERVATION | 8 / 0x620 | 0x139 | +0x20 / 0xBDD | 0x0 | not a directory ID; role UNKNOWN |
| OBSERVATION | 8 / 0x620 | 0x125 | +0x20 / 0x119A | 0x0 | not a directory ID; role UNKNOWN |
| OBSERVATION | 8 / 0x620 | 0x126 | +0x20 / 0x121A | 0x0 | not a directory ID; role UNKNOWN |
| OBSERVATION | 8 / 0x620 | 0x12A | +0x20 / 0x12E4 | 0x0 | not a directory ID; role UNKNOWN |
| OBSERVATION | 8 / 0x620 | 0x12F | +0x20 / 0x15FA | 0x0 | not a directory ID; role UNKNOWN |
| OBSERVATION | 8 / 0x620 | 0x130 | +0x20 / 0x167A | 0x0 | not a directory ID; role UNKNOWN |
| OBSERVATION | 10 / 0x66A | 0x13B | +0x6A / 0xCE4 | 0x0 | not a directory ID; role UNKNOWN |
| OBSERVATION | 10 / 0x66A | 0x12B | +0x6A / 0x13AE | 0x141 | index 78; parent = source |
| OBSERVATION | 10 / 0x6C | 0x13B | +0x6C / 0xCE6 | 0x0 | not a directory ID; role UNKNOWN |
| OBSERVATION | 10 / 0x6C | 0x12B | +0x6C / 0x13B0 | 0x140 | index 79; parent = source |
| OBSERVATION | 10 / 0x66E | 0x13B | +0x6E / 0xCE8 | 0x0 | not a directory ID; role UNKNOWN |
| OBSERVATION | 10 / 0x66E | 0x12B | +0x6E / 0x13B2 | 0x13F | index 80; parent = source |
| OBSERVATION | 10 / 0xA620 | 0x13B | +0x20 / 0xC9A | 0x0 | not a directory ID; role UNKNOWN |
| OBSERVATION | 10 / 0xA620 | 0x12B | +0x20 / 0x1364 | 0x0 | not a directory ID; role UNKNOWN |

OBSERVATION: there are three nonzero ID hits, all in 0x12B: +0x6A -> 0x141/index 78, +0x6C -> 0x140/index 79, +0x6E -> 0x13F/index 80. Each target has the source as its independent directory-parent candidate (3/3), and each selector family has one positive instance plus a zero in 0x13B. The corresponding A selectors use these same offsets and child targets. The new B word at +0x20 is zero in all eight index-8/index-10 records; its role stays UNKNOWN, rather than being called a reference or conclusively not a reference.

OBSERVATION: each match has the following explicit false-positive controls. Control pools contain every other aligned u16 fixed-prefix field from +4 through H-2, excluding all projected offsets and the story anchor (+0x58 or +0x66). A pool is counted once per record, not once per selector. The 0x12B pool has 51 fields, zero directory-ID hits, zero parent-corroborated hits and no other occurrence of any of its three targets. Thus each of the three target matches has a zero-hit local control, while its paired 0x13B selector is zero. Reading the positive fields as u8 gives 65/64/63, none directory IDs; reading as u32 gives 20971841/20906304/65855, none directory IDs. Only the tested u16 width resolves all three targets.

| Basis | Source record | Nonselector control fields | Directory-ID hits | Parent-corroborated hits | Zero control fields |
| --- | --- | ---: | ---: | ---: | ---: |
| OBSERVATION | 0x139 | 55 | 0 | 0 | 38 |
| OBSERVATION | 0x125 | 55 | 0 | 0 | 31 |
| OBSERVATION | 0x126 | 55 | 1 | 0 | 31 |
| OBSERVATION | 0x12A | 55 | 1 | 0 | 31 |
| OBSERVATION | 0x12F | 55 | 0 | 0 | 31 |
| OBSERVATION | 0x130 | 55 | 1 | 0 | 31 |
| OBSERVATION | 0x13B | 51 | 0 | 0 | 35 |
| OBSERVATION | 0x12B | 51 | 0 | 0 | 22 |

OBSERVATION: across the eight unique control pools, 3/432 fields hit a directory ID and 0/432 corroborate source-parent relationships. Zeros occur in 250/432 controls, so the eight zero +0x20 fields and three zero child-selector controls do not positively establish reference semantics. The three exact positive target values do not occur elsewhere in their source's control pool. The controls are empirical fields, not independent random draws, so these counts are not p-values.

INFERENCE: descriptor-10 projected +0x6A/+0x6C/+0x6E are sample-corroborated child references, probable as general selector families because each has only one nonzero source instance. The projections preserve A's three tested child links. UNKNOWN: general tag meanings, selector payload width, conditions of applicability and the zero-only +0x20 field; no full B reference graph follows from these five families.

### What scale and origin the evidence supports

OBSERVATION: the root size values are 7560000/10692000; their ratio equals 210/297. Dividing by the candidate 36000 units/mm gives A4 exactly. Signed32 probes at +0x06/+0x0A/+0x0E/+0x12 give the same six raw rectangles as Sample98. A half-root-size translation makes six distinct, positive, contained boxes, while interpreting raw coordinates directly as top-left-page coordinates contains none under the inside-page requirement. Two of 24 other-index fixed-prefix controls also pass the translated test.

INFERENCE: B is numerically consistent with A's candidate units and centered-page origin. It does not independently prove either. The standard-size match supports a 36000-units/mm hypothesis, and containment supports one possible origin; neither supplies a physical measurement. Alternative translations over [2952000,5076000] horizontally and [4446000,8082000] vertically work, including a further 36000-unit shift. Even substituting Letter dimensions still contains all six. Other-class numerical positives do not establish rectangle semantics, and failure of raw containment alone does not rule out intentionally off-page objects.

INFERENCE: one additional discriminating observation is an independently documented creation measurement for one unrotated B text frame: its physical left/top/right/bottom in millimetres on a page of known physical size, tied to a known story/record. From raw endpoints l,t,r,b and measured endpoints L,T,R,B, compute s_x=(r-l)/(R-L), s_y=(b-t)/(B-T), and translations t_x=s_x*L-l, t_y=s_y*T-t. Equality of s_x/s_y tests a shared scale; s=36000 and t_x=W/2, t_y=H/2 would test the proposed unit and origin, and the known physical page dimensions independently test root W/H. Different results reject or narrow the candidate. A single measured point without an independently known extent cannot separate scale from translation. UNKNOWN: this joint page/frame observation is unavailable here; no physical unit/origin conclusion is promoted to PROVEN.

### Text-to-rectangle join

OBSERVATION: a story link and the four rectangle probes are co-located in the same Contents record. No traversal through descriptor 10's three child records is required to join them. Index 8 reads its story candidate at +0x58, index 10 at +0x66; geometry reads +0x06/+0x0A/+0x0E/+0x12 in that same bounded fixed prefix. Quill SYID/STRS/TEXT supplies the story range, not the projected descriptor tails. Joining by record ID produces the following complete positive census.

| Basis | Record / index | Story link absolute offset / ID | UTF-16 range | Collection -> root | Collection child-tail ID position | Candidate translated rectangle |
| --- | --- | --- | --- | --- | --- | --- |
| OBSERVATION | 0x125 / 8 | 0x11D2 / 1 | [0,86) | 0x10A -> 0x100 | 0x1166 | 1080000, 900000, 6156000, 1872000 |
| OBSERVATION | 0x126 / 8 | 0x1252 / 3 | [86,184) | 0x10A -> 0x100 | 0x1168 | 1080000, 2052000, 6264000, 3132000 |
| OBSERVATION | 0x12A / 8 | 0x131C / 4 | [184,246) | 0x127 -> 0x100 | 0x12A4 | 828000, 936000, 6192000, 2484000 |
| OBSERVATION | 0x12B / 10 | 0x13AA / 6 | [246,326) | 0x127 -> 0x100 | 0x12A6 | 1368000, 2808000, 4932000, 3465564 |
| OBSERVATION | 0x12F / 8 | 0x1632 / 7 | [326,413) | 0x127 -> 0x100 | 0x12AA | 1404000, 5256000, 5796000, 5940000 |
| OBSERVATION | 0x130 / 8 | 0x16B2 / 8 | [413,488) | 0x127 -> 0x100 | 0x12AC | 1440000, 6228000, 5976000, 7956000 |

OBSERVATION: all six positive story links have a distinct nondegenerate candidate rectangle in the same record (6/6); every story ID resolves to the already bounded Quill partition (6/6), and every text record appears in its directory parent's index-20 tail (6/6). There are no failing positive records. Index-8 0x139 and index-10 0x13B have zero story links and all-zero rectangles: these are the two excluded inactive controls, not silently placed text frames. All eight index-8/index-10 records are accounted for. The parent chains and same-record join reproduce A's pattern, with B index-20 H=32 rather than A's 30.

OBSERVATION: the story-ID controls remain five distinct IDs at the sole common +0x58 cohort offset, one positive +0x66 field, and two zero controls, as tested in AP16. The other-class geometry hits 0x123 and 0x12E do not qualify through these tested index-8/index-10 story anchors; their numerical rectangles do not create extra text records. These are controls on selecting candidates, not proof that other classes can never contain frames. UNKNOWN: actual displayed frame identity, physical placement, pages/order and any multi-frame/story sharing beyond this sample. The measured same-record association is stronger than geometry plausibility alone, but still cannot prove visual semantics.

### Bounds-checked builder checklist and A differences

OBSERVATION: the requested PROVEN status below means checked byte relationships in this single sample; it never means a universal B encoding. INFERENCE labels describe implementation requirements or candidate semantic interpretation; UNKNOWN labels prohibit silently presenting an unverified placement as faithful. The Basis column retains the research evidence labels separately from the requested builder status.

| Basis | Builder status | Data and bounds checks for one candidate text frame | Difference from A |
| --- | --- | --- | --- |
| OBSERVATION | PROVEN (sampled bytes) | Validate actual Contents length at +0x08, bounded pointers +0x12/+0x16/+0x1A, directory count at D, and 10-byte entries D+4+10*i. Require distinct IDs and bounded ascending starts; E is next start or stream end. | Same directory fields; B control/directory at 0x3BA/0x3E8 in this sample. |
| INFERENCE | INFERRED (safe reader policy) | Use checked wide arithmetic for counts, offsets and multiplications; cap counts by remaining bytes, reject overlaps/overflow, bound traversal with visited IDs, preserve unsupported cases. Do not use a numerical match to bypass a failed bound. | Same safety policy as A. |
| OBSERVATION | PROVEN (sampled bytes) | Resolve index 8 or 10; verify H from record byte +3, fixed prefix inside E, and matching descriptor H. Require +0x16 <= H for four signed32 reads, and anchor+4 <= H before the sampled story read. | Same offsets/H=118 or116; B table has its separate candidate scan, not A's literal-zero walk. Leading slot/tag semantics remain UNKNOWN. |
| OBSERVATION | PROVEN (sampled join) | Read sampled story candidate at +0x58 or +0x66 and require exact membership in Quill SYID. Bound Quill section directory, count arrays and UTF-16 ranges; paired SYID/STRS counts and cumulative lengths must fit TEXT. All six sampled u32 reads equal their low u16 values. | Same story offsets and Quill partition method. Authoritative Contents-side story-ID width still UNKNOWN. |
| OBSERVATION | PROVEN (sampled containment relationships) | Resolve the record's directory-parent candidate and verify its ID in the index-20 child tail: T=R+H, ten-byte array header, checked n*w span, w=2 here. Continue a bounded parent chain to root-like 0x100. | B H=32, tail IDs at record+42; A H=30, IDs at +40. Do not hard-code the A value. This chain does not prove page assignment. |
| INFERENCE | INFERRED (geometry candidate) | Read root index-21 positive u32 W/H at +0x14/+0x18 only if +0x1C <= fixed prefix; read signed32 l/t/r/b at the four frame offsets. Require r>l and b>t; treat all-zero controls as unplaced. | Same numerical fields and candidate meanings as A. |
| INFERENCE | INFERRED (experimental placement only) | Compute L2=2*l+W, R2=2*r+W, T2=2*t+H, B2=2*b+H in checked wide integers. For an inside-page candidate require 0<=L2<R2<=2*W and 0<=T2<B2<=2*H; preserve half-unit precision and report failure instead of silent clipping. Convert by candidate 36000 units/mm or 12700 units/point. | Same unconfirmed half-page origin/scale as A; B provides matching numerical data, not known-position validation. |
| UNKNOWN | UNKNOWN | Faithful physical origin/scale, whether the candidate rectangle is the actual displayed frame, which collection is an actual page, page/reading order and other frame classes. Obtain the independent physical page/frame observation before claiming faithful placement. | Same essential limit as A. |
| UNKNOWN | UNKNOWN / out of scope | Rotation, anchors, margins, columns, fonts, colours, pictures and the existing Quill import failure; no derived repair or default is claimed. | Same excluded work as A. |

INFERENCE: the Quill bounds in that checklist mean: start directory blocks at 0x18; bound u16 entry count at block+2, u32 next-block pointer at +4, and 24-byte entries at +8; bound each entry's section offset/length u32 at +16/+20. For SYID, bound count u32 at section+4 and IDs from +8. For STRS, bound count at +0, displacement at +4 and lengths from section+4+displacement. Convert cumulative UTF-16 ranges to TEXT bytes with checked multiplication by two; reject unsupported count mismatches or ranges beyond TEXT rather than trimming them into a false match. For the sampled Contents-side u32 story read, a future nonzero upper half is unsupported by this cohort; do not silently truncate it to a known ID. These are conservative experimental-reader checks, not newly decoded semantics.

INFERENCE: an experimental builder can attach each of the six verified story ranges to its same-record candidate box while surfacing the geometry/page assumptions. The evidence does not yet justify a universally faithful B page renderer. A descriptor child reference is not needed to connect text to these candidate boxes, and should not be confused with the story link.

OBSERVATION: this first AP18 checkpoint contains 14 bounded projected reads, three controlled child-ID matches, eleven zero controls and six complete story-to-candidate-rectangle joins with no positive failures. UNKNOWN: physical placement and general selector semantics remain open; no new source entry is needed because all evidence was already listed. Research stops here.

## Descriptor-2/3 projections and B reader readiness

OBSERVATION: AP19 is single-sample B research on the already listed Sample2000.pub. Its pinned SHA-256 and preceding bounded experiments were rechecked; only the same binary and own notes supply evidence. No new source, scoring data, external reader or specification was used. All results in this section, including table rows, describe this one file. No importer, environment setting or existing script changed; no full CI or pull request was run.

### Exact candidates and record census

OBSERVATION: descriptor 2 occupies [0x26,0x38), descriptor 3 [0x38,0x4A). Their first raw tail words are both LE u16 0x0772, at Contents 0x2C and 0x3E respectively. Projection with 0x00FF yields 0x72 = 114; with 0x01FF yields 0x172 = 370. This tests the same raw word, not two independent stored fields. The census has exactly one index-2 record, ID 0x133, and one index-3 record, ID 0x134; no other instantiated descriptor-2/3 record exists in the directory.

| Basis | Descriptor / record | Directory entry | Record [R,E) | H / span | +114 candidate u16 | +370 candidate u16 |
| --- | --- | --- | --- | --- | --- | --- |
| OBSERVATION | 2 / 0x133 | 0x41E | [0x998,0xA2E) | 140 / 150 | [0xA0A,0xA0C), fixed-prefix bounded; value 0 | [0xB0A,0xB0C), outside record; not read |
| OBSERVATION | 3 / 0x134 | 0x428 | [0xA2E,0xAC4) | 140 / 150 | [0xAA0,0xAA2), fixed-prefix bounded; value 0 | [0xBA0,0xBA2), outside record; not read |

OBSERVATION: both records use the stored array-tail equation H+10+n*w: H=140, n=0, duplicate n=0, w=8, predicting 150 bytes. Thus neither candidate's bounds are judged against a length guessed from that candidate. +114+2=116 fits the 140-byte fixed prefix and the independent 150-byte span in 2/2 records. +370+2=372 exceeds both in 2/2 records. Relative to the end of either source span, the latter read would end 222 bytes too late. Both sources have directory-parent candidate 0x12C.

### False-positive and directory-parent controls

| Basis | Candidate | Prefix / record bounds passes | Values actually read | Directory-ID / source-child / actual-parent matches | Control and conclusion |
| --- | --- | --- | --- | --- | --- |
| OBSERVATION | R+114, u16 | 2/2; 2/2 | Two zeros | 0/2; 0/2; 0/2 | Other aligned fixed-prefix fields are often zero; no positive semantic match. |
| OBSERVATION | R+370, u16 | 0/2; 0/2 | None | Not tested: zero eligible reads | Whole-stream bounds alone pass 2/2, despite both fields belonging to other records; stream bounds are an insufficient control. |

OBSERVATION: for each source, the false-positive pool is every other aligned LE u16 field at relative +4,+6,...,+138, excluding +114. This is 67 fields per record. In each pool 55/67 fields are zero, 0/67 equal any directory ID, 0/67 name a target whose directory parent is the source, and 0/67 equal the source's actual parent candidate 0x12C. Combined, zeros occur in 110/134 fields, with 0/134 directory-ID, child-parent or actual-parent hits. These two local pools explicitly control both bounded zero matches; the pools are not independent random draws and no statistical significance is claimed.

OBSERVATION: a misleading whole-stream bound would accept both +370 candidates, but directory ownership places [0xB0A,0xB0C) at record 0x136 relative +1, and [0xBA0,0xBA2) at record 0x138 relative +0x14. These byte positions are determined from offsets only; their values were not used to manufacture a selector match. A field in a different record is not a valid field of the source merely because it stays inside Contents.

INFERENCE: +114 is supported only as a bounded, zero-valued candidate in the current two records. It is not supported as a reference, and it does not prove an eight-bit selector payload. +370 is refuted as an unconditional live record-relative u16 field in both current records; a nine-bit interpretation with an optional, stale or otherwise inapplicable selector is still compatible with the observed bounds failures. Choosing the low-byte mask solely because its reads fit would be unjustified. UNKNOWN: the actual payload mask, tag meanings, applicability rule, zero semantics and whether either word identifies a live reference in another B file. Neither projection obtains positive reference evidence in this test.

INFERENCE: a bounds-checked reader can preserve this raw word and skip an unsupported or out-of-record interpretation. It must not silently replace an out-of-bounds interpretation with +114 and call the resulting zero a decoded reference. This selector uncertainty does not change the six already checked story-to-candidate-box joins, which use co-located fields rather than descriptor-2/3 traversal.

### Short Sample2000 status table

OBSERVATION: PROVEN below means directly checked sampled byte relationships, not universal B semantics. INFERRED identifies supported interpretations or experimental reader choices. UNKNOWN identifies evidence that the current file does not supply.

| Basis | Status | Sample2000 finding and reader implication |
| --- | --- | --- |
| OBSERVATION | PROVEN (single sample) | Actual stream length/header pointers, 54/54 bounded directory spans, 54 distinct IDs; directory-parent values form one rooted tree with 53 internal edges. These permit bounded enumeration, not page identification. |
| OBSERVATION | PROVEN (single sample) | 92/92 ordinary descriptor candidates end exactly at 0x3BA; all 54 instantiated indices agree with descriptor H and expose class words, including class 67. Low-eight/low-nine predicates give identical framing. |
| INFERENCE | INFERRED | Class-selected equations predict 54/54 endpoints: 53 use independently corroborated previous class rules, one class-67 empty instance uses a candidate rule. Uniform framing semantics, nonempty class 67 and internal padding remain unproven. |
| OBSERVATION | PROVEN (single-sample joins) | Six distinct Quill IDs/ranges sum to 488 UTF-16 units; 6/6 positive links co-locate with distinct, positive candidate boxes and collection-tail memberships; two inactive controls are zero. Byte joining is available without descriptor-2/3 references. |
| INFERENCE | INFERRED | Story-link/frame interpretation and root/collection roles beyond measured joins; candidate physical placement uses root +0x14/+0x18, signed32 endpoints +0x06/+0x0A/+0x0E/+0x12, 36000 units/mm and half-page translation. A bounds-checked experimental placement is possible with explicit assumptions. |
| UNKNOWN | UNKNOWN | Exact selector mask/tags, leading slot, +114/+370 reference meaning, general graph and flags; authoritative Contents story-ID width; complete page identity/order and actual visible frame semantics. |
| UNKNOWN | UNKNOWN — largest faithful-placement gap | Physical scale and origin have no independent position measurement; containment, standard-size fits and other-class controls cannot prove them. No second B file validates generality. |

INFERENCE: the single additional observation with the greatest placement value is one independently documented B creation measurement linking a known story to an unrotated frame's physical left/top/right/bottom on a page of known physical size. It could be supplied for Sample2000 or in one new confirmed-B sample. Positive measured width/height determine the raw-to-physical scale; measured left/top then determine translations, permitting direct tests of 36000 units/mm, root dimensions and half-page origin. The known story joins that measurement to the tested record, avoiding arbitrary box matching. This closes or falsifies the largest one-frame placement hypothesis, not general page ordering, all descriptor semantics or wider-file coverage. A known page size alone, an isolated point or another merely plausible contained rectangle would not close it.

INFERENCE: the next smallest decisive step is that joint physical page/frame observation. Repeating the current zero probes cannot establish a selector mask. A future selector-specific positive control would need a B record in which the competing fields are both legally addressable and their nonzero values can be independently tied to directory relationships, or independent evidence of when the larger offset applies. UNKNOWN: neither such positive selector evidence nor the physical placement measurement is available here. Research stops at this checkpoint with both limits explicit.
