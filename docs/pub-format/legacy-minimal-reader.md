# Experimental legacy text reader contract

## Scope and evidence

INFERENCE: a separate experimental crate may recover linked text segments and root-size metadata from the legacy A, B and third-family profiles below. It must remain separate from the 2002+ importer. Its output is text, provenance, size candidates and diagnostics; it creates no frames, pages or layout objects. This document is a proposed reader contract, not an implementation or a claim that every Publisher 97/98/2000 file is decoded.

OBSERVATION: this contract consolidates the [A notes](legacy-97-2000-contents-a.md), [B notes](legacy-97-2000-contents-b.md), [replication and third-family text findings](legacy-97-2000-replication.md), the bounded section-directory definitions in [text-store.md](text-store.md), and identities already recorded in [SOURCES.md](SOURCES.md). No new binary research, other reader, external format description or specification is used for this document. Later replication/AP20/AP21 findings supersede the earlier checkpoints' availability and text-link unknowns.

OBSERVATION: the documented evidence comprises nine distinct binary identities. A reattachment of mspub is not another sample; Sample98 and Sample2000 have identical TEXT payloads, and mspub/zmspub have identical empty Quill streams. File counts therefore do not prove independently varied text content. The following table gives the complete file sets used by every coverage reference in this contract.

| Basis | Evidence set | Files | Documented recovery evidence |
| --- | --- | --- | --- |
| OBSERVATION | ALL, 9 files | The three sets below | 2731 bounded directory records; common header/directory/root probes. |
| OBSERVATION | A, 3 files | Sample98.pub, mspub.pub, zmspub.pub | 132/132 class-selected extents. Only Sample98 has populated text: six links and 488 UTF-16 units. The other two have empty Quill stores. |
| OBSERVATION | B, 1 file, **FRAGILE** | Sample2000.pub | 54/54 extents, six links and 488 UTF-16 units; all B-specific conclusions are single-file. |
| OBSERVATION | C, 5 files | CATALOG.PUB, JAZZY.PUB, MODERN.PUB, PLAIN.PUB, SCHOOL.PUB | Zero-based 64-entry layout tables; central maps link 28/72/74/88/30 byte ranges, 292/292 total. C is a provisional third-family label, not a confirmed saving version. |

UNKNOWN: exact saving releases, physical page identity and independently measured size are not established by these filenames, CLSIDs or numerical fits. **FRAGILE** means a finding rests on one file; a synthetic test does not remove that label. In particular A's populated-text behavior is **FRAGILE**, despite three-file A structural coverage. Two-file positive Quill evidence is explicitly identified below rather than treated as broad version coverage.

## Verified fields, with family boundaries preserved

OBSERVATION: the shared rows below hold in ALL nine documented files. Offsets are hexadecimal; counts are decimal. Integers are little-endian and may be unaligned. L is actual root Contents length, C is the control pointer, D the directory pointer, R a record start, E its next directory start or L for the last record, H the byte at R+3, and T=R+H. A directory-derived span is an upper bound until a separate field equation agrees with E.

| Basis / evidence | Field or rule | Verified relationship; limit |
| --- | --- | --- |
| OBSERVATION / ALL | Compound container; root `/Contents` | Containers open as compound version 3; root Contents exists. A/B root CLSID ends in the 00021201 profile, C in 00021200; the complete values are in the profile table below. Quill is not common to all families. |
| OBSERVATION / ALL | Contents +0x00 / 4; +0x02 / u16; +0x04 / u16 | Discriminator `E8 AC 22 00`; overlapping +0x02 equals 34; +0x04 equals 300. Exact release meanings remain UNKNOWN. |
| OBSERVATION / ALL | +0x08 / u32 | Equals L, the actual extracted stream size. |
| OBSERVATION / ALL | +0x12, +0x16, +0x1A / u32 | C, D and layout-table start respectively; table start is 0x22. All are inside Contents with their full structures bounded. |
| OBSERVATION / ALL | Table +0 / u16; +2 / u16 | First value distinguishes the sampled cardinalities 89/94/64; the second is 6, with UNKNOWN semantic role. Ordinary entries begin at 0x26. Index bases and termination are family-specific. |
| OBSERVATION / ALL | Ordinary layout entry | Three u16 words followed by u16 tail words and a family-specific terminator. With k nonterminating words its width is 6+2*(k+1). No fixed entry stride is established. |
| OBSERVATION / ALL | Entry word 0 high byte; word 1; word 2 | High byte agrees with instance H in 2731/2731 records. Word 1 is the numeric class code used by extent tests. Word 2 is not a universal stride. Low-byte and tail/tag semantics remain UNKNOWN except for separately corroborated uses. |
| OBSERVATION / ALL | Control +0 / u16; +2 / u16; +0x0A / u32 | Marker 0xF0DE, stored length 46, and D. Control +0x0E is **not** a shared directory pointer: it is D in A/B and text start S in C. |
| OBSERVATION / ALL | Directory D / u16; D+2 / u16 | Count N and a value equal to the maximum entry ID. Four-byte header precedes exactly N ten-byte entries. |
| OBSERVATION / ALL | Entry +0 u16; +2 u16; +4 u32; +8 u16 | Distinct record ID, parent-candidate value, absolute R, opaque attribute. Attribute meanings remain UNKNOWN. |
| OBSERVATION / ALL | Directory starts and gap | Starts are distinct, ascending and in range after the directory. The gap to the first record is zero-filled; observed lengths are 10 or 20 bytes, not a universal fixed length. Spans reach L without sorting or invented endpoints. |
| OBSERVATION / ALL | Record +0 / u16; +2 / byte; +3 / byte | Layout index, low prefix byte and H. Every record has its four-byte prefix bounded and 4 <= H <= E-R. Neither the low byte nor H is a universal total length. |
| OBSERVATION / ALL | Parent-candidate graph and header +0x0E / u16 | One record has a parent outside the ID set; all other parents resolve and the graph is acyclic. That root-like record's ID equals header +0x0E and its index is 21. The graph does not establish pages. |
| OBSERVATION / ALL | Root index-21 array tail | Equal u16 counts at T/T+2, stored width 2 at T+4 and IDs from T+10; all sampled IDs resolve to records whose parent candidate is the root. This is not a proven complete list of pages or even all root references. |
| OBSERVATION / ALL | Root R+0x14, R+0x18 / u32 | Positive raw width/height candidates, wholly inside the fixed prefix. Numeric index-44 records repeat the pair at +4/+8. Duplication does not establish physical units or page identity. |

UNKNOWN: +0x06, +0x0C, +0x10, +0x1E and +0x20 are not universally decoded semantic fields. In particular +0x06 is 4 in the attachments and 261 in the two Sample files. The reader must not use these values, directory attributes, filenames, sample hashes or a standard-looking size pair alone to select a release or a text path.

### Family admission and extent evidence

| Basis / evidence | Profile | Required sampled relationships; constants are not byte offsets to hard-code |
| --- | --- | --- |
| OBSERVATION / A | A | CLSID `{00021201-0000-0000-00C0-000000000046}`; table word 89; 88 ordinary entries indexed 1..88; literal-zero tail termination; exact walk ends at C; D=C+46; control +0x0E=D; Quill path `/Quill/QuillSub/CONTENTS`. |
| OBSERVATION / B, **FRAGILE** | B | Same CLSID as A; word 94; 92 ordinary entries indexed 2..93 end at C; leading u16 6 is opaque, slots 0/1 unproved; D=C+46; control +0x0E=D; same Quill path. Low-eight-bit and low-nine-bit zero termination give identical walks. Observed raw terminators are 0, 0x0600, 0x0800, 0xA600. Exact tag/mask semantics remain UNKNOWN. |
| OBSERVATION / C | C | CLSID `{00021200-0000-0000-00C0-000000000046}`; word 64; 64 ordinary entries indexed 0..63; literal-zero termination; exact walk ends at C; S=u32(C+0x0E)=C+46 precedes D; no Quill stream. Text-core end marker P=u32(C+0x12), with S<P and P+2<=D. |

INFERENCE: admission requires all structural relationships, not just the cardinality or CLSID. Recognize the structural family before checking expected text-stream availability under AB01/C01; a missing A/B Quill stream is a failed text path, not proof of family C or an empty publication. Warn about an unexpected extra text store rather than following it as an undocumented fallback. Compute pointers from their fields; do not hard-code C, D, file length, directory IDs, root ID 0x100 or private sample hashes. A relocated synthetic profile is a parser test, not proof that every similarly shaped real file belongs to this family. For B, require the two termination walks to agree on every boundary and fixed word, and retain the opaque prefix. A divergent walk or new terminator profile is unsupported, not an invitation to choose whichever mask happens to fit. Do not interpret descriptor selectors: no selector traversal is needed by the minimal text paths.

OBSERVATION: independent record-length equations verified in the A/B notes are listed here for clarity. They are **not** universal across C: C has 190 failed/unmapped transferred framing cases, including ten matching-length cases with invalid offset-table structure. The minimal reader must distinguish opaque records from validated text dependencies.

| Basis / evidence | Classes | Independent size and extra checks |
| --- | --- | --- |
| OBSERVATION / A; B **FRAGILE** | 0 | H. |
| OBSERVATION / A; B **FRAGILE**; C array subsets | 1,9,25 | H+10+n*w; u16 n, equal duplicate and positive w at T,T+2,T+4. Required C index-0/index-1 targets and index-21 root pass this rule. |
| OBSERVATION / A; B **FRAGILE**; C index 26 | 2,10 | H+22+n*(4+w); u16 n/duplicate/w at T,T+2,T+4; n four-byte slots, extra four-byte boundary and n w-byte elements. Required C index 26 has class 10, H=4 and w=10. |
| OBSERVATION / A; B **FRAGILE** | 4 | H+8+q; u32 q at T. |
| OBSERVATION / A; B **FRAGILE** | 3,195 | H+18+q; u32 n/duplicate/q at T,T+4,T+8; bounded n+1 u32 offsets relative to T+18, first 4*(n+1), last q, strictly increasing. |
| INFERENCE / B only, **FRAGILE** | 67 | Same offset-table candidate fits its sole empty instance; nonempty behavior remains UNKNOWN. This class is not needed for minimal B text recovery. |

INFERENCE: every directory span and prefix must be safe, but a text claim needs independent extent proof only for the records it consumes. Validate A/B text-bearing indices 8/10 (classes 9/1, H=118/116, stored w=2/8), C target indices 0/1 (classes 9/1, H=92/96, w=2/14), C map index 26, and the root index 21 (class 25, H=178 in A/B or 136 in C, w=2). Require the applicable equation to equal E-R exactly; root-tail IDs must also resolve with the sampled parent relationship, without claiming list completeness. An unfamiliar required shape is unsupported. An unconsumed record whose logical extent is unknown stays opaque within its safe directory span, with a format-named warning; it must not block the known text map merely because an A/B equation fails on that C record. No padding or endpoint may be invented to repair a required dependency.

### A/B text fields

| Basis / evidence | Field or rule | Recorded support and limit |
| --- | --- | --- |
| OBSERVATION / A positive Sample98 only, **FRAGILE**; B only, **FRAGILE** | Contents index 8 +0x58, index 10 +0x66 | Six positive links per populated file; ID must resolve in that file's Quill SYID. All sampled upper two bytes of a four-byte read are zero. Authoritative storage width is UNKNOWN. Empty controls exist in mspub/zmspub. |
| OBSERVATION / the two populated Sample files; structural empty-store controls in two owner files | Quill directory | Block at 0x18; u16 count at block+2, u32 next at +4, 24-byte entries from +8; sentinel 0xFFFFFFFF ends chain. Section tag is four bytes at entry+2, section offset/length u32 at +16/+20. Repeated/cyclic chains must be bounded. |
| OBSERVATION / populated behavior **FRAGILE per family** | SYID / STRS / TEXT | SYID count u32 +4 and IDs from +8; STRS count u32 +0, displacement u32 +4, lengths from section+4+displacement (observed displacement 8). Cumulative lengths are UTF-16-unit ranges in TEXT. Sum*2 equals TEXT byte size; all positive slices end with u16 CR. |
| OBSERVATION / Sample98 and Sample2000, two files with identical TEXT | Counts | Six IDs 1,3,4,6,7,8; lengths 86,98,62,80,87,75; 488 units. These are evidence, not constants the reader should impose on future content. |
| OBSERVATION / mspub and zmspub, two files with identical Quill streams | Empty profile | SYID count zero, no TEXT/STRS/TCD, and zero story probes. Zero probes alone never prove an arbitrary publication has no text. |

### C text fields

OBSERVATION / C: S=u32(C+0x0E), P=u32(C+0x12), M=P-S. The core is Contents [S,P); the AP20 candidate buffer includes the CR/LF trailer [P,P+2). Five cores have 5262/6845/6768/7108/10562 bytes. There is a form feed at P-1, CR/LF at P, then zeros to the next 512-byte boundary before D. The rest of the pre-directory region is not decoded.

OBSERVATION / C: exactly one index-26/class-10 record occurs per file. Set T=R+H, n=u16(T), duplicate=u16(T+2), w=u16(T+4). Both counts agree, n is positive/even and w=10. Let A=T+18 and B=T+22+4*n. Require n u32 slots at A, an extra u32 at A+4*n equal to M, and n ten-byte elements at B, all within E. Their independent total H+22+n*(4+10) equals E-R.

OBSERVATION / C: for j from zero through n/2-1, slots 2*j and 2*j+1 are byte start/end relative to S. They form positive, adjacent, ascending spans covering [0,M) without sorting, gaps or overlaps. u16 at B+20*j gives a unique directory ID; u16 at +2 is 1 for target index 0 and 0 for index 1; u16 at B+20*j+10 is zero. There are 292/292 linked ranges, ending in CR/LF then form feed; the final two-byte trailer is unmapped. The target ID is not a Quill story ID. Remaining metadata and rendered marker meanings stay UNKNOWN.

| Basis / evidence | File | M / candidate bytes | Mapped segments |
| --- | --- | --- | ---: |
| OBSERVATION / C | CATALOG.PUB | 5262 / 5264 | 28 |
| OBSERVATION / C | JAZZY.PUB | 6845 / 6847 | 72 |
| OBSERVATION / C | MODERN.PUB | 6768 / 6770 | 74 |
| OBSERVATION / C | PLAIN.PUB | 7108 / 7110 | 88 |
| OBSERVATION / C | SCHOOL.PUB | 10562 / 10564 | 30 |

INFERENCE: a C output may be called a recovered linked text segment, identified by family, source map, target record ID and byte range. It must not invent a Quill ID, a natural-language story boundary, a page or a frame object. Source array order may be preserved as storage order only.

UNKNOWN: C's unique code page is not established. Strict Windows-1252 decoding succeeds in five buffers and its punctuation interpretation is plausible in four; CATALOG's buffer is entirely ASCII. Successful decoding alone is insufficient. The strict default must decode only ASCII-only segments losslessly, preserving control values. A segment with bytes at or above 0x80 must be reported as encoding-unknown and not claimed as verified decoded text. Its validated raw range may be retained as undecoded provenance, never logged as private content. A future explicit owner-confirmed encoding may support a separately labelled interpretation; the reader must not silently select Windows-1252, UTF-16, a replacement character or a different family because it looks readable.

## Required gates and exact failure rules

INFERENCE: the following are conservative reader policies, not newly observed format laws. Refuse means emit no story from the failed unit. File refusal invalidates all results; text-path refusal can retain independently checked raw size candidates; segment refusal permits other independently valid segments with an explicit partial-recovery warning. Metadata failure withholds size only. Every diagnostic names `experimental legacy A`, `B`, `C` or `legacy family unknown`, the failed field/range or record ID, and whether corruption, an unsupported profile or a resource limit was observed. It does not assert an exact release or echo private text.

INFERENCE: no result may be published before the shared tables and that result's complete dependency chain are validated. All offset additions, products, count-derived allocations, range conversions and traversal limits use checked arithmetic. As an initial operational policy, refuse files or individual extracted streams above 32 MiB, more than 65535 recovered segments/IDs, or Quill chains above 64 blocks; these are experimental limits, not format facts. Counts must also fit remaining bytes before allocation. No retry may relax a limit or reinterpret a failed field.

| Basis / gate | Exact acceptance check | Failure rule |
| --- | --- | --- |
| INFERENCE / G01 container and budgets | A bounded container reader validates version-3 sector/FAT/directory chains and stream lengths; root Contents is uniquely resolvable and all operational budgets hold. | Refuse file; distinguish damaged container, unsupported version and reader limit. Never carve strings from a failed container. |
| INFERENCE / G02 header | At least 34 Contents bytes; exact old-family discriminator, +0x02=34, +0x04=300; u32 +0x08=L; checked full control/table/directory fields are inside L. | Refuse file as malformed or unfamiliar legacy header. Do not truncate L or search for another plausible header. |
| INFERENCE / G03 family/table | Admit one complete A/B/C profile above, including the matching CLSID, table cardinality, index convention, prefix 6 and exact table end C. For B both termination walks agree and use only the observed terminator profile. | Refuse unfamiliar/ambiguous profile; B slot 0/1 has no invented descriptor. An unconsumed such slot may stay opaque with warning only if its directory/prefix is safe; a required use is refused. |
| INFERENCE / G04 regions | 0x22 <= C; C+46 <= D; marker/length/control +0x0A agree; directory/table/control do not overlap. A/B require D=C+46 and control +0x0E=D; C requires its separate S/P interval. Bound directory-to-record zero gap rather than hard-code 10/20. | Refuse file for unsafe or contradictory regions. A nonzero/unfamiliar gap is unsupported; do not drop it as padding. |
| INFERENCE / G05 directory | Bound D+4+10*N; N>0; unique IDs, maximum-ID agreement, distinct ascending R after directory and before L; define every E from serialized starts or L. | Refuse file. Never sort starts, merge duplicate IDs or substitute a guessed final endpoint. |
| INFERENCE / G06 prefixes | Every record has R+4<=E and 4<=H<=E-R; each known index resolves under its family's convention and descriptor high byte equals H. | Refuse unsafe/corrupt directory profile. An unsupported but bounded unconsumed descriptor is warned/opaque; a required unknown shape refuses that text dependency. |
| INFERENCE / G07 root/parents | Exactly one root-like parent outside the ID set, all other parents resolve, traversal has no cycle, root ID equals header +0x0E and index is 21. | Refuse structural profile; no guessed root or page assignment. |
| INFERENCE / G08 consumed extents | Root, every text source and every text-link target pass their independently read class equation and known shape above, including count equality and complete tail bounds; predicted endpoint equals E. Root-tail IDs satisfy the sampled resolving-parent relationship. | Refuse dependent segment, or the entire C map if its source/targets fail. Withhold size if root extent or root-tail consistency fails. Opaque unrelated C records produce warnings, not fabricated extents or loss of already independent text. |
| INFERENCE / G09 size candidate | Root +0x1C fits H/E; raw u32 W/H at +0x14/+0x18 are positive and conversions do not overflow. Return raw pair and evidence status. | Withhold size with format-named warning; independent text may remain. Do not scan for a standard-size substitute. |
| INFERENCE / AB01 Quill chain | Open the specified Quill stream; begin at 0x18; bound each block+8+24*count, follow bounded next pointers to 0xFFFFFFFF with a seen-set and 64-block limit. | Refuse A/B text path on missing/invalid chain; do not assume an empty store or fall back to C. |
| INFERENCE / AB02 sections | Bound every declared section offset+size. For positive recovery require one unambiguous TEXT/SYID/STRS group; no multiple-tag pairing is decoded here. | Refuse ambiguous/invalid text path. Ignore other bounded section bodies; do not infer text from TCD, formatting or embedded objects. |
| INFERENCE / AB03 IDs | SYID has at least 8 bytes; count and u32 ID array fit its section and budget; positive IDs are distinct/nonzero. | Refuse text catalog on truncation, duplicates or unsupported null IDs. No ordinal-as-ID substitution. |
| INFERENCE / AB04 lengths | STRS header fits; count equals SYID count; observed displacement 8 selects section+12; all u32 lengths fit. A different displacement/profile is unsupported until documented. | Refuse text catalog. No clamping, ignored excess IDs or inferred lengths; stricter than historical permissive behavior described in text-store.md. |
| INFERENCE / AB05 ranges | Checked cumulative positive lengths define UTF-16-unit slices; TEXT has even byte size; sum*2 equals its full size; each slice fits and ends in u16 CR. | Refuse text catalog for inconsistent partition, odd size or missing boundary evidence. Never resynchronize by paragraph searches. |
| INFERENCE / AB06 links | For index 8/10 require the checked class/H/extent, then anchor+4<=H. Read the four sampled bytes, require upper half zero; a nonzero low u16 must equal an existing u32 SYID value. | Unsupported width or dangling link refuses that segment with warning; never truncate an upper half into a known ID. Zero probes create no story. An unreferenced SYID range is not a claimed linked story. |
| INFERENCE / AB07 text | Only an ID-linked, fully bounded UTF-16LE slice may become a decoded story; use strict decoding and preserve its control units. Distinguish source ID, record provenance and range from layout. | Refuse that story on invalid UTF-16; no replacement characters, field expansion or encoding retry. Conflicting ownership is warned; emit a validated ID once, without inventing threading. |
| INFERENCE / AB08 empty profile | A validated SYID count zero, absent TEXT/STRS and zero sampled link probes produce zero recovered stories. | Positive probes or nonempty unpartitioned TEXT contradict emptiness and refuse text path. Report empty known store, not publication-wide absence of text. |
| INFERENCE / C01 buffer | S=C+46, S<P, P+2<=D; M=P-S; marker at P-1 is FF, [P,P+2) is CR/LF; checked next-512 boundary F<=D with zero gap. | Refuse C text path; never treat P as a length, accept a printable replacement interval or assume missing Quill means empty text. |
| INFERENCE / C02 source | Exactly one index-26/class-10/H=4 map, independently extent-checked; known target profiles are index 0/1, not A's 8/10. | Refuse ambiguous/absent/unfamiliar map. No arbitrary first-record selection or class-wide text guess. |
| INFERENCE / C03 arrays | n=u16(T)>0, even, duplicate matches, w=10; bound n slots at T+18, extra boundary M at T+18+4*n, n elements at B=T+22+4*n, and exact E. | Refuse map on count, width, overflow, extra/missing bytes or contradictory final boundary. |
| INFERENCE / C04 ranges/markers | Pair u32 slots in serialized order; 0<=start<end<=M, first start 0, each end equals next start, last end M. Each range contains one FF at end-1 preceded by CR/LF; no split CR/LF. | Refuse complete map on contradiction; no sorting, gap filling or partial pairing. Preserve [M,M+2) as unmapped trailer, not another story. |
| INFERENCE / C05 record link | Every even metadata element supplies a distinct resolving u16 ID with checked target extent; +2 kind is 1 for index 0, 0 for index 1; odd element ID is zero. | Refuse map on a bad target, repeated ID, kind or phase mismatch. Do not use a whole u32 ID/kind, borrow a neighboring field or reinterpret the payload as Quill IDs. |
| INFERENCE / C06 decode | After map validation, decode ASCII-only slices exactly, including control values; preserve byte ranges and target-ID provenance. | Non-ASCII segment: encoding-unknown warning and no verified decoded-text claim; retain only labelled undecoded range metadata in this mode. A marker is never turned into a page break. |
| INFERENCE / O01 claims | Every output has family/profile confidence, linked ID type, source ranges/units, partial/empty state and diagnostics; private bytes stay out of logs. | Suppress any stronger success label or output field unsupported by these gates; never claim full-fidelity import. |

INFERENCE: unknown selectors, directory attributes, fonts, formatting, pictures and unused opaque sections do not need semantic decoding to pass a text story. They must not be followed or interpreted. Warn that recovery is partial and embedded/object text is outside scope. Required structural dependencies cannot be skipped under that exception. A malformed C map is atomic; A/B malformed catalogs are atomic, while a bad independent link/decoder slice can withhold only its segment after the catalog has passed.

## Page-size output and must-not-claim boundary

OBSERVATION: root raw pairs have nine-file support, while dividing by 36000 gives numerical A4, Letter, half-Letter and custom-square candidates. The existing notes have no independent physical measurement proving this unit or that every root pair is a physical page. Matching pairs in index 44 corroborate storage, not units. Positive geometry once rested on one A and one B file, and third-family half-root containment fails in 58 linked records; none of that is a page-size oracle.

INFERENCE: the output may expose the checked raw root pair as a root-size candidate, and optionally the explicitly **INFERRED** conversion W/36000 and H/36000 millimetres (or W/12700 and H/12700 points) with a named uncertainty warning. It must not label that conversion a verified physical page size. If an API requires a verified physical size, that field is UNKNOWN and must remain absent. Size failure does not authorize a guessed A4/Letter default. A duplicate-pair probe is optional corroboration, not permission to substitute another record's standard-looking values.

INFERENCE: the minimal reader must not claim exact saving release; page count/identity/order; frame creation, positions, dimensions or coordinate transforms; reading order or threading; fonts, glyph coverage, styles, colours, fills, borders, geometry, margins, columns, rotation, tables/cells, pictures, embedded-object text, fields, visible/hidden/overset status or native editability/layout fidelity. Record IDs are opaque provenance, not frames. Storage order is not visual order. It must not expand control codes into page numbers/dates or silently strip markers, normalize paragraph endings, repair files, infer unknown tags or use the current importer as an undocumented fallback.

UNKNOWN: exact C code page and marker rendering; general B tag/slot behavior; authoritative A/B Contents story-ID width; duplicate count authority, unknown class extents and internal padding; root/page semantics and physical scale; wider sample/version coverage. These remain limits even when every mechanical safety check succeeds. A text-only result is not a recovered Publisher document.
