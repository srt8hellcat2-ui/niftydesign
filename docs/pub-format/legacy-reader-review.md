# Experimental legacy reader review

OBSERVATION: this review compares the [contract](legacy-minimal-reader.md) at `af76694113ae646534f9db6534cc0bece2152dc5` with this repository's experimental crate at `00f2b83dda403171f703b05ae22e8054244179f3`. The inspected files are `crates/pub-legacy/src/{lib,contents,text,cmap,tests}.rs`. Implementation access is expressly authorized for this review. Evidence about the format comes only from the existing [A](legacy-97-2000-contents-a.md), [B](legacy-97-2000-contents-b.md) and [replication](legacy-97-2000-replication.md) notes and their retained local census. No external reader or format description was used.

OBSERVATION: this is a source review and manual real-file trace, not an executed crate test. The restored corpus directories are unavailable in this session. The requested existing-census route is used; retained C Contents extracts allow checking ASCII eligibility without quoting private content. No new format finding or binary is committed.

## Gate review

INFERENCE: MATCH means the inspected branch implements the acceptance and failure policy; STRICTER means it rejects additional inputs; LOOSER means at least one required guard or diagnostic is missing. A LOOSER row can also contain a stricter subcase. These classifications concern implementation, not independent confirmation of the format.

| Basis | Gate | Result | Reason and implementation location |
| --- | --- | --- | --- |
| OBSERVATION | G01 | LOOSER | `lib::open/read_stream` limits file/extracted bytes and delegates container validation to `cfb`, but never explicitly checks version 3, declared versus delivered stream length, or unique Contents resolution. Those guarantees are not established by this review. |
| OBSERVATION | G02 | MATCH | `Layout::parse` checks the 34-byte header, discriminator, overlapping constants, actual length and table pointer; subsequent region checks bound the referenced structures. |
| OBSERVATION | G03 | LOOSER | CLSID/cardinality/index base/exact walk and B low-eight/low-nine agreement are checked, but the observed B terminator allowlist is not checked. For example 0x0200 terminates both walks and is not one of the contract's four terminators. |
| OBSERVATION | G04 | MATCH | Control marker, length, directory pointers, family-specific adjacency and directory gap are checked; C's remaining interval checks occur in `read_map`. |
| OBSERVATION | G05 | MATCH | Count-derived directory bound, nonempty count, unique IDs, maximum ID and serialized ascending in-range starts are checked; endpoints are next starts or stream end. |
| OBSERVATION | G06 | MATCH | Every prefix fits its span, H is safe and matches a known descriptor; an unknown index is retained opaque with a warning. |
| OBSERVATION | G07 | MATCH | One external-parent root, resolving acyclic parent chains, header root ID and root index 21 are checked. |
| OBSERVATION | G08 | LOOSER | `check_extent` checks the class equation and duplicate counts, but consumers omit some required exact class/H/width profiles. Root H/class, A/B index-8/10 class/H/width and C target class/H/width are not explicitly enforced. Unconsumed known descriptors with unexplained extents receive no specific opaque warning. |
| OBSERVATION | G09 | MATCH | `size_candidate` requires independent root extent/root-tail checks, bounded fixed-prefix fields and positive dimensions; failure withholds size only. Conversion of u32 values to f64 is bounded. G08's missing root-shape check still applies. |
| OBSERVATION | AB01 | MATCH | `sections` starts at 0x18, bounds entries, follows the sentinel with a seen set and 64-block cap; missing or invalid Quill becomes text-path refusal. |
| OBSERVATION | AB02 | MATCH | All declared sections are bounded and needed tags must be unique; other bounded bodies are ignored. |
| OBSERVATION | AB03 | MATCH | SYID header/list/count budget, positive IDs and uniqueness are checked before catalog use. |
| OBSERVATION | AB04 | MATCH | STRS count, displacement 8 and complete length list are checked with no clamping. |
| OBSERVATION | AB05 | MATCH | Positive cumulative lengths exactly consume even-length TEXT; each slice ends in CR. |
| OBSERVATION | AB06 | LOOSER | Four-byte probes, upper-half rejection, local-ID resolution and segment withholding are present; G08's exact known-shape guards are missing. |
| OBSERVATION | AB07 | LOOSER | Strict UTF-16 and once-per-ID output with duplicate warnings are present, but `Source::Story` retains ID/range without the linking record's ID, losing required record provenance. |
| OBSERVATION | AB08 | LOOSER | Zero-length TEXT/STRS sections are accepted although the contract says absent; failed/unvalidated candidate links can still lead to `EmptyKnownStore`, including when every candidate fails before `probes` increments. |
| OBSERVATION | C01 | MATCH | Absolute buffer bounds, terminal FF/CRLF and zero padding to the next 512-byte boundary are checked. |
| OBSERVATION | C02 | MATCH | Exactly one index-26/class-10/H=4 map is required and its extent is independently checked. |
| OBSERVATION | C03 | MATCH | Positive even repeated count, width 10, exact equation and extra boundary M bound both arrays. |
| OBSERVATION | C04 | MATCH | Positive adjacent ranges cover M exactly, each has one terminal FF preceded by CRLF, and the external trailer is excluded. |
| OBSERVATION | C05 | LOOSER | IDs resolve uniquely, target indices agree with kinds, odd IDs are zero and generic target extents pass; the exact target class/H/width profiles remain unchecked. |
| OBSERVATION | C06 | MATCH | ASCII-only slices preserve controls; high-byte slices retain range/ID with text absent, encoding-unknown and Partial state. |
| OBSERVATION | O01 | LOOSER | Family, scope, ranges and size uncertainty are exposed without layout output, but A/B record provenance is absent and early errors do not consistently provide the required family-or-unknown, field and corruption/unsupported/limit classification. |

OBSERVATION: no named gate is wholly absent. The missing subchecks are substantial: matching the gate numbering in test names does not establish complete contract coverage. The tests include positive A/B/C models and named negative cases, but do not demonstrate the omitted checks above. The only real-file test in the inspected source is ignored by default and covers Sample98/Sample2000, not the other seven identities.

UNKNOWN: dependency-level validation of malformed FAT chains, duplicate container entries and truncated declared streams is not established here; the container dependency's source was not inspected. Explicit version-3 admission is absent in the crate regardless of dependency behavior.

## Real-file manual replication

OBSERVATION: shared G02–G07 relationships pass in the retained census for all eight requested files. All consumed root/text dependency extents have the contract's sampled shapes, so G08/G09 checks also pass on these particular inputs. The earlier transferred generic extent rules fail on unrelated C records; the implementation does not consume those records and therefore does not refuse the maps. These facts do not excuse the missing guards on unfamiliar inputs.

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
