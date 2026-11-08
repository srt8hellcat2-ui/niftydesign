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

1. UNKNOWN: B's exact terminator/tag encoding and leading-prefix/index-1 convention. OBSERVATION: AP17 bounds 92 ordinary entries exactly to 0x3BA and corroborates 54/54 instantiated H/class lookups, but low-eight and low-nine termination cannot be separated in this sample; the word at 0x24 remains special. INFERENCE: the next smallest step is to test the low-bit projections of the raw tail words in descriptors 8 and 10 against bounded record fields, directory IDs and false-positive controls. Keep high-bit/tag meanings and uninstantiated slot 1 UNKNOWN unless the bytes discriminate them.
2. UNKNOWN: nonempty class-67 framing, the correct interpretation of the four ambiguous empty tails, count-versus-capacity roles, internal padding and general extent semantics. OBSERVATION: AP17 identifies index 92's sampled class word as 67; its empty tail fits the O equation but cannot discriminate alternatives. No global single rule or cross-file B replication is established.
3. UNKNOWN: B selector references beyond the 17 corroborated index-20 tail IDs, complete reference graph, parent semantics and attribute-bit meanings. The directory-parent tree alone does not decode all references.
4. UNKNOWN: authoritative Contents-side story-ID width, stored text ranges, page identity/order and visual reading order. OBSERVATION: AP17 finds the same A4 root-size pair and six distinct contained signed32 rectangle candidates at the A offsets. UNKNOWN: physical scale/origin and actual placement; alternative origins/sizes also contain all six, and two other-class fixed-prefix controls pass. Numerical plausibility does not prove geometry semantics.
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
