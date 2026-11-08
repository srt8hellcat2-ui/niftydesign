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

1. UNKNOWN: B's descriptor-table framing and index-to-class/selector convention. The A walk fails; class-selected transfer is a hypothesis, not 54 independently decoded B classes. INFERENCE: the next smallest step is to bound B entries for indices 8 and 10, using their stable record H and three fixed words as checks, then test the resulting entry-framing rule over the complete table against count 94 and control boundary 0x3BA. Do not assign semantic selector roles first.
2. UNKNOWN: index 92's class, the correct interpretation of the four ambiguous empty tails, count-versus-capacity roles, internal padding and general extent semantics. All sampled endpoints fit known arithmetic, but no global single rule or cross-file B replication is established.
3. UNKNOWN: B selector references beyond the 17 corroborated index-20 tail IDs, complete reference graph, parent semantics and attribute-bit meanings. The directory-parent tree alone does not decode all references.
4. UNKNOWN: authoritative Contents-side story-ID width, any stored text range, page identity/order and visual reading order. Six story links and Quill-derived ranges are corroborated, not all record semantics.
5. UNKNOWN: saving version and wider coverage. Sample2000 is the sole B file currently available; A geometry inferences and other untested class semantics are not generalized to B.

INFERENCE: a bounded experimental walker can enumerate the 54 directory spans and preserve unknown records. A production reader still needs the B descriptor checkpoint before treating the transferred class mapping as a decoded format rule. OBSERVATION: this is the first checkpoint with a complete directory census, 54/54 arithmetic fits and six controlled text links; research stops here.
