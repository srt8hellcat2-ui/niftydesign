# Experimental legacy reader review

OBSERVATION: this review compares the [contract](legacy-minimal-reader.md) version 2 in this branch with this repository's experimental crate at `fbab857f` (the pinned revision for this re-review). The inspected files are `crates/pub-legacy/src/{lib,contents,text,cmap,tests}.rs`. Implementation access is expressly authorized for this review. Evidence about the format comes only from the existing [A](legacy-97-2000-contents-a.md), [B](legacy-97-2000-contents-b.md) and [replication](legacy-97-2000-replication.md) notes and their retained local census. No external reader or format description was used.

OBSERVATION: this is a source review and manual real-file trace, not an executed crate test. The restored corpus directories are unavailable in this session. The requested existing-census route is used; retained C Contents extracts allow checking ASCII eligibility without quoting private content. No new format finding or binary is committed.

## Gate review

INFERENCE: MATCH means the inspected branch implements the acceptance and failure policy; STRICTER means it rejects additional inputs; LOOSER means at least one required guard or diagnostic is missing. A LOOSER row can also contain a stricter subcase. These classifications concern implementation, not independent confirmation of the format.

| Basis | Gate | Result | Reason and implementation location |
| --- | --- | --- | --- |
| OBSERVATION | G01 | LOOSER | `open_cf` now explicitly requires CFB V3; `read_stream` checks declared size before reading and delivered length afterwards. Unique root-Contents resolution and complete chain validation remain delegated/unverified. Unchecked intermediates such as Quill `at + 2` also lack overflow guards on a 32-bit host. The required linear parent-work bound is not achieved: `path.contains(&at)` scans the growing path, so one long chain still takes quadratic work. |
| OBSERVATION | G02 | MATCH | `Layout::parse` checks the 34-byte header, discriminator, overlapping constants, actual length and table pointer; subsequent region checks bound the referenced structures. |
| OBSERVATION | G03 | MATCH | `walk` now enforces the four B terminators, both masked walks and their equality, cardinality, index base and exact end. The parsed root UUID is converted to canonical lowercase text before comparison; this is value-equivalent for these constants, not comparison of arbitrary presentation strings. |
| OBSERVATION | G04 | MATCH | Control marker, length, directory pointers, family-specific adjacency and directory gap are checked; C's remaining interval checks occur in `read_map`. |
| OBSERVATION | G05 | MATCH | Count-derived directory bound, nonempty count, unique IDs, maximum ID and serialized ascending in-range starts are checked; endpoints are next starts or stream end. |
| OBSERVATION | G06 | MATCH | Every prefix fits its span, H is safe and matches a known descriptor; an unknown index is retained opaque with a warning. |
| OBSERVATION | G07 | MATCH | One external-parent root, resolving acyclic parent chains, header root ID and root index 21 are checked. |
| OBSERVATION | G08 | LOOSER | `check_profile` now enforces exact consumed class/H/width and extent, including root and C targets. Missing check/report: bounded unconsumed records with known table indices but unvalidated logical extents are not counted and warned as opaque. |
| OBSERVATION | G09 | MATCH | `size_candidate` now enforces the exact root profile as well as extent, child IDs, bounded positive dimensions and bounded conversion; failure withholds size only. |
| OBSERVATION | AB01 | MATCH | `sections` starts at 0x18, bounds entries, follows the sentinel with a seen set and 64-block cap; missing or invalid Quill becomes text-path refusal. |
| OBSERVATION | AB02 | MATCH | Section bounds and unique required tags are checked. The 100000-section limit is explicitly adopted as an operational budget in contract v2. |
| OBSERVATION | AB03 | MATCH | SYID header/list/count budget, positive IDs and uniqueness are checked before catalog use. |
| OBSERVATION | AB04 | MATCH | STRS count, displacement 8 and complete length list are checked with no clamping. |
| OBSERVATION | AB05 | MATCH | Positive cumulative lengths exactly consume even-length TEXT; each slice ends in CR. |
| OBSERVATION | AB06 | MATCH | Each candidate passes its exact profile, extent and four-byte probe; unsupported width or dangling local ID withholds that segment. Zero links create no story. |
| OBSERVATION | AB07 | MATCH | Strict UTF-16 decoding and once-per-ID output remain; `Source::Story.records` now retains every validated linking record ID in storage order, with a duplicate warning. |
| OBSERVATION | AB08 | MATCH | Any present TEXT/STRS section, even zero-length, rejects the empty profile. Failed candidates yield Partial; a validated positive probe contradicting zero SYID refuses text. Only fully checked zero probes yield EmptyKnownStore. |
| OBSERVATION | C01 | MATCH | Absolute buffer bounds, terminal FF/CRLF and zero padding to the next 512-byte boundary are checked. |
| OBSERVATION | C02 | MATCH | Exactly one index-26/class-10/H=4 map is required and its extent is independently checked. |
| OBSERVATION | C03 | MATCH | Positive even repeated count, width 10, exact equation and extra boundary M bound both arrays. |
| OBSERVATION | C04 | MATCH | Positive adjacent ranges cover M exactly, each has one terminal FF preceded by CRLF, and the external trailer is excluded. |
| OBSERVATION | C05 | MATCH | The map now requires exact index-specific target class/H/width through `check_profile`, in addition to distinct resolving IDs, kind agreement and zero odd IDs. |
| OBSERVATION | C06 | MATCH | ASCII-only slices preserve controls; high-byte slices retain range/ID with text absent, encoding-unknown and Partial state. |
| OBSERVATION | O01 | LOOSER | Missing structured diagnostic fields/categories and explicit unknown-family/field/scope on every failure; C provenance also omits the source map ID required by v2. Missing no-read check for an alternate store: C calls `quill_stream`, which actually reads it before warning that it was not read; errors are silently ignored. Presence must be tested without reading its payload, with a presence warning independent of readability. |

OBSERVATION: no named gate is wholly absent. The missing subchecks are substantial: matching the gate numbering in test names does not establish complete contract coverage. The tests include positive A/B/C models and named negative cases, but do not demonstrate the omitted checks above. The only real-file test in the inspected source is ignored by default and covers Sample98/Sample2000, not the other seven identities.

UNKNOWN: dependency-level validation of malformed FAT chains, duplicate container entries and truncated declared streams is not established here; the container dependency's source was not inspected. Explicit version-3 admission and declared/delivered stream-length checks are now present. Unique container-name resolution remains a dependency guarantee not verified here.

## Real-file manual replication

OBSERVATION: shared G02–G07 relationships pass in the retained census for all eight requested files. All consumed root/text dependency extents have the contract's sampled shapes, so G08/G09 checks also pass on these particular inputs. The earlier transferred generic extent rules fail on unrelated C records; the implementation does not consume those records and therefore does not refuse the maps. These facts do not establish the remaining diagnostics, opaque-record reporting or resource-work guarantees.

INFERENCE: in the following table, “pass” is a manual acceptance prediction from the census, conditional on successful container extraction. G01 is recorded separately as a documented valid-container observation, not a fresh execution or a demonstration that all malformed-container checks exist. Nonapplicable A/B or C gates are N/A. C06 withholding is a successful application of its policy, not a damaged-file failure.

| Basis | File/family | Root raw width / height | Gate trace and expected scope | Segments / decoded / encoding-unknown |
| --- | --- | --- | --- | --- |
| OBSERVATION / INFERENCE | Sample98.pub / A | 7560000 / 10692000 | Shared and AB gates pass; C N/A; Complete | 6 / 6 / 0; 488 UTF-16 units |
| OBSERVATION / INFERENCE | mspub.pub / A | 7772400 / 10058400 | Shared and AB empty-profile gates pass; C N/A; EmptyKnownStore | 0 / 0 / 0 |
| OBSERVATION / INFERENCE | zmspub.pub / A | 7560000 / 10692000 | Shared and AB empty-profile gates pass; C N/A; EmptyKnownStore | 0 / 0 / 0 |
| OBSERVATION / INFERENCE | CATALOG.PUB / C | 5029200 / 7772400 | Shared and C gates pass; AB N/A; Complete | 28 / 28 / 0 |
| OBSERVATION / INFERENCE | JAZZY.PUB / C | 18288000 / 18288000 | Shared/C structural gates pass; C06 withholds eight decodes; Partial | 72 / 64 / 8 |
| OBSERVATION / INFERENCE | MODERN.PUB / C | 18288000 / 18288000 | Shared/C structural gates pass; C06 withholds nine decodes; Partial | 74 / 65 / 9 |
| OBSERVATION / INFERENCE | PLAIN.PUB / C | 20116800 / 20116800 | Shared/C structural gates pass; C06 withholds ten decodes; Partial | 88 / 78 / 10 |
| OBSERVATION / INFERENCE | SCHOOL.PUB / C | 7772400 / 10058400 | Shared/C structural gates pass; C06 withholds one decode; Partial | 30 / 29 / 1 |

OBSERVATION: the C counts were cross-checked by taking each previously verified AP21 byte range from its retained Contents extract, starting at S=686 in these five files, and counting ranges containing a byte at least 0x80. Only aggregate counts are recorded here. The counts sum to 292 validated ranges, 264 ASCII-decodable ranges and 28 undecoded ranges. The mapped cores contain respectively 5262, 6845, 6768, 7108 and 10562 byte units; the separate final two-byte CRLF trailer is not part of any segment.

OBSERVATION: Sample98's six ranges are [0,86), [86,184), [184,246), [246,326), [326,413), [413,488), with story IDs 1,3,4,6,7,8. Its two remaining candidate link probes are zero. Each empty A attachment has two zero candidate probes and the validated empty Quill profile. Attachment hashes and identities are already recorded in SOURCES and the replication note; they are not treated as three independent populated A text examples.

INFERENCE: no contradiction with the reader's inspected expected Sample98 output was found: six stories, the stated lengths and raw size pair match its ignored real-file assertion. There is no inspected real-file assertion for the other seven requested files. The C segment totals must not be mistaken for decoded-string totals: an expectation of 72/74/88/30 decoded strings would differ by 8/9/10/1 respectively. Conversely the reader retains all range objects, including undecoded ones, so expecting those objects to disappear would also differ.

UNKNOWN: this checkpoint does not establish that running the compiled reader on the original eight containers returns exactly these results, warnings or segment order. The raw corpus was not restored and the reader was not executed. Full output equality, container-path behavior and ignored-test execution remain unverified. The documented B sample is outside this eight-file trace; its table behavior remains single-file evidence.

## Decisions folded into contract v2

INFERENCE: the following policy wording is now incorporated into contract v2. The table records the reason for each clarification; this docs revision does not implement fixes. Remaining deviations are identified in the current gate table above.

| Basis | Decision or gap | V2 policy wording |
| --- | --- | --- |
| INFERENCE | CLSID notation | “Compare parsed UUID values, not presentation strings. Braces and hexadecimal letter case in this document are notation only. Do not reorder serialized UUID fields manually.” |
| INFERENCE | Zero-length stories versus an empty store | “A positive SYID catalog with any zero STRS length is an unsupported text partition and refuses the text path. Only validated SYID count zero can select EmptyKnownStore. No empty slice may stand in for a failed story.” This makes the existing positive-length rule explicit rather than adding zero-length support. |
| INFERENCE | Empty optional sections | “For AB08, TEXT and STRS must be absent. Present zero-length sections are an unsupported empty-store variant pending evidence; their presence is not equivalent to absence.” The pinned implementation now enforces this rule. |
| INFERENCE | Failed links in an empty store | “EmptyKnownStore requires every sampled candidate record to have its required shape, checked extent and bounded four-byte probe, with every probe zero. A failed candidate prevents a known-empty assertion; report Partial/unrecovered unless a validated positive probe contradicts SYID zero, in which case refuse the text path.” |
| INFERENCE | B tail word with a zero low byte | “Starting after each six-byte entry prefix, read complete u16 words. Perform low-eight-zero and low-nine-zero walks independently. Both must succeed with identical boundaries and fixed words, end exactly at C, and terminate only on 0x0000, 0x0600, 0x0800 or 0xA600. A divergent walk, including 0x0100, or a new agreeing terminator such as 0x0200 is unsupported; never choose a mask from which one fits.” |
| INFERENCE | Record-shape admission | “Check exact descriptor class, instance H and stored element width for each consumed profile before applying its equation: AB 8=(9,118,2), 10=(1,116,8); C 0=(9,92,2), 1=(1,96,14), 26=(10,4,10); root 21=(25,178,2) in AB and (25,136,2) in C. Equation agreement alone is insufficient.” These constants already appear in the contract; this states their enforcement order. |
| INFERENCE | Unused known records | “Report the number of bounded unconsumed records whose independent extent/meaning was not validated as opaque. Do not claim their equations passed, and do not fail independent text because their optional interpretation is unknown.” |
| INFERENCE | Positive but unlinked catalog entries | “Complete means all supported Contents-linked candidates were recovered, not every possible text location in the publication. Count and warn about positive SYID IDs without a validated link; retain them as unclaimed catalog entries, never linked output stories. Distinguish this coverage limitation from a failed candidate.” The pinned implementation counts and warns about unreported catalog IDs; Complete refers to linked candidates only. |
| INFERENCE | Duplicate-link provenance and ordering | “Emit one decoded A/B story per validated ID; retain all validated source record IDs, its Quill unit range and the catalog identity. Preserve a deterministic storage order and label it storage order only. Duplicate references establish neither reading order nor threading.” The pinned implementation now retains the linking record IDs. |
| INFERENCE | Resource budgets and validation cost | “The 32 MiB budget applies before reading each stream, using declared size when available, and again to delivered bytes. Require count arrays to fit before reserving storage; bound aggregate section count explicitly if an additional limit is adopted. Bound graph-validation work as well as record count; use already validated paths rather than repeatedly traversing an adversarial long chain.” The pinned implementation memoizes settled nodes, but its linear search of the current path still permits quadratic work. |
| INFERENCE | Unneeded alternate store | “After family admission, read only the supported text path. An unexpected alternate store is warned and not decoded. Failure to read an unused alternate store must not invalidate independently validated text.” The pinned C path still reads Quill after admission to detect its presence; this violates the proposed no-payload-read policy. |
| INFERENCE | Diagnostic and confidence representation | “Every failure includes a stable failure category, family or explicit legacy-family-unknown, failed field/range or record ID, and recovery scope. Output states that classification is an experimental structural profile, with B and populated A evidence still FRAGILE. No error contains private text.” Generic malformed strings in early shared checks do not supply every component. |

UNKNOWN: no additional real file currently demonstrates zero-length positive stories, a zero-length-section empty profile, a new B terminator, or a wider A/B story reference. Synthetic cases can test refusal policy but cannot decide those format meanings. The C code page, physical page-size unit and exact saving versions also remain unknown; this review supplies no new evidence for them.

## Next independent evidence

| Basis / rank | Requested independent file | UNKNOWN it would address |
| --- | --- | --- |
| INFERENCE / 1 | A native Publisher 2000 save that first classifies as B, with known custom page dimensions and independently authored expected text in several stories; preferably exercises varied descriptor tails. | B framing, link offsets and populated text are single-file findings. A tail word that separates the two walks would discriminate masks; an ordinary new B file alone may not settle that question. Known dimensions would also test the raw-pair scale. |
| INFERENCE / 2 | A populated Publisher 97 or 98 save that classifies as A, independent of Sample98, with known nonstandard dimensions, distinct short stories and an intentionally empty text object. | A's populated links/partition are single-file evidence despite three A containers. Expected empty-object behavior would test whether zero links or zero-length catalog stories represent it; independently recorded dimensions test physical scale. |
| INFERENCE / 3 | A matching C-profile save with owner-confirmed expected non-ASCII text and creation language/code-page settings, known dimensions and several distinct text objects. | Five C files confirm structural ranges but do not identify the code page or natural-language story semantics. Distinguishable accented characters/punctuation and confirmed settings could validate a labelled decoder without guessing from printability. |

INFERENCE: the next smallest verification step is to run the unchanged reader against the original eight verified binaries when available, checking family, raw pairs, source ranges, scopes and the 264/28 C decoded/unknown split against this manual trace. That run would confirm execution, not close the missing guard requirements. A later code-lane review should test each missing guard with byte-built fixtures before treating the reader as contract-complete.

UNKNOWN: this checkpoint does not establish page identity/order, frame placement, fonts, formatting, version-wide compatibility or physical dimensions. No geometry or layout decoding is proposed. The outstanding resource, alternate-store and diagnostic gaps above remain open even though the documented valid files fit the implemented paths.
