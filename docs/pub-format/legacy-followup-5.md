# Legacy follow-up: larger-set structural evidence

OBSERVATION: this note records the owner's structure-only run over 52 tester inputs. No document text was printed in that run. Only the supplied aggregate numbers are used here; no file identity, document content or personal detail from the set is recorded. No private file was requested, transferred or run here, and no other Publisher reader, external format description or prohibited repository was consulted.

OBSERVATION: this branch starts from the [round-4 documentation](legacy-followup-4.md). This task adds this note and an invented-bytes integration test file only. No reader/importer logic, resource limit or admission rule is changed. The current implementation commit for the larger run was not supplied; these are owner-reported execution results, not a new source audit.

## Complete accounting of the reported set

| Basis | Group | Count | Reported result and limits |
| --- | --- | ---: | --- |
| OBSERVATION | Legacy discriminator e8 ac 22 00 | 32 | The five subgroups below account for all 32. |
| OBSERVATION | Current discriminator e8 ac 2c 00 with Escher streams | 19 | The legacy reader skips these as not a legacy file. They have not yet been run through the current importer. |
| OBSERVATION | Non-compound, described by the owner as pre-OLE | 1 | Separate container category; not an A/B/C profile. |
| OBSERVATION | Supported B layout within the 32 | 18 | Open Complete, 1..52 segments per input, three warnings each except one with four. Complete is the reader's supported-text scope, not native-layout fidelity. |
| OBSERVATION | Header word +0x04=200 within the 32 | 8 | Currently called damaged because the implementation expects 300. Structural pattern below. |
| OBSERVATION | Unknown table word 58 within the 32 | 3 | Root class 00021201, table prefix 6, no Quill store, four streams each. |
| OBSERVATION | Root class 00021200 with table word 94 within the 32 | 2 | Both have the Quill CONTENTS store; one root Contents stream is about 2.2 MB. Refused by the current profile pairing. |
| OBSERVATION | File over MAX_BYTES within the 32 | 1 | Refused at the 32 MiB operational limit. Exact input size was not supplied. |

OBSERVATION: 18+8+3+2+1=32 and 32+19+1=52. There are 18 supported legacy results, 14 refused legacy inputs, 19 legacy-reader skips and one non-compound input. These are assessment counts; they are not a 52-file batch-conversion success/failure score. Warning contents were not supplied and cannot be reconstructed from warning counts.

OBSERVATION: all eight word-200 headers have +0x02=34 and u32 +0x08 equal to the actual Contents length. The 0xf0de marker exists at the control pointer. The directory pointer is below the control pointer by 2242..145772 bytes. The +0x1a pointer is 0x22 in two, 0x31 in one and 0x34 in five. The word at 0x22 is 44 in two and 20552 in six; it is not a universal table cardinality. None has Quill; stream counts range from three to 28. No pointer/word cross-tabulation beyond these aggregates is inferred.

INFERENCE: the repeated word-200 pattern, different pointer order and varying table-start locations support an earlier or distinct generation from the word-300 layouts. Word 200 alone is not demonstrated damage, and is not proof of a particular product release. Recognise the bounded header pattern and refuse it as unsupported before interpreting records or extracting text. This does not certify every such file as undamaged.

INFERENCE: the two root-00021200/table-94 inputs are B candidates, not yet B successes. Admit the additional identity pairing only after the full B path passes; neither a cardinality nor Quill presence is sufficient. The table-58 group remains an undocumented profile; do not assume its entry count/index base or treat missing Quill as an empty supported store.

## What is now known, and what remains unknown

OBSERVATION: B reader execution now has 18 successful inputs in this set, with varied segment counts. The earlier single-input execution limitation is superseded for this set. There is no supplied identity/independence evidence with which to add these counts to an older corpus or claim 18 independent saving histories. Native text correctness was not scored by this structure-only run.

UNKNOWN: exact saving releases, native page semantics, C's code page, reading order, generation-200/table-58 framing and text locations, success of the two B candidates, exact oversize input length and the 19 current-importer outcomes remain unproven. Repeated unsupported profiles are a classification problem, not evidence that lowering or bypassing bounds checks is safe.

OBSERVATION: the single pre-OLE input belongs in a separate non-compound category with stable reason code not-compound. INFERENCE: arbitrary non-compound bytes must get the same container classification without guessing a Publisher release. The code does not acquire pre-OLE text support by recognising that category.

## Ordered task for the owner's code lane

INFERENCE: implement the following in the current experimental implementation checkout. Keep the current-family importer first and unchanged. The tests added here target the existing synthetic-fixture dependencies of that checkout; they do not add those dependencies or production code to this documentation branch. All tester identities/content stay outside the repository.

### (a) Stable reasons, before wider admission

INFERENCE: add a typed reason value to LegacyError with a stable reason_code/as_str interface; keep readable explanations separate. Reports must obtain the code from that type, never by matching English text. Carry reasonCode through the batch worker round trip and folder result. Proposed reporting schema: append a reasonCode column after the existing legacy CSV column and add reasonCode to each folder file object. Keep all existing columns/legacy fields in place. Success uses an empty CSV value/null JSON, not a failure code. Update header expectations for this explicit additive schema change.

| Basis | Code | Category / meaning |
| --- | --- | --- |
| INFERENCE | not-compound | NOT_LEGACY container refusal. Includes the owner-described pre-OLE category; no saving-version guess. |
| INFERENCE | not-legacy | Legacy-assessment skip for a different discriminator. The 19 current-family inputs are routed to the current importer, not counted as failed conversions because of this skip. Their importer results are still UNKNOWN. |
| INFERENCE | header-gen-200 | UNSUPPORTED: safely recognised word-200 header, refused before record/text interpretation. |
| INFERENCE | profile-unknown | UNSUPPORTED: unreviewed profile, including table 58 and an unadmitted identity pairing. Missing Quill is not EmptyKnownStore. |
| INFERENCE | over-size-limit | LIMIT: configured operational budget, not damage or a format diagnosis. |
| INFERENCE | damaged-layout | MALFORMED for actual contradictory/truncated structures in a validated interpretation; do not use it merely for an unfamiliar generation/profile. |

INFERENCE: preserve the v2 diagnostic fields category, family-or-legacy-family-unknown, gate, structural field/record ID, recovery scope and result scope alongside reasonCode. A skipped modern input must not acquire a failed batch status from the legacy assessment; a successful current import has no refusal reasonCode. Retain typed causes even when wrapping an error for the user.

INFERENCE: un-ignore stable_reason_codes_survive_batch_and_folder_reports after typed code/report transport exists and LEGACY_FOLLOWUP_CLI points to the matching built CLI. It uses only invented non-compound and table-58 cases, expects failed conversions and checks the exact code in both report formats. Also un-ignore non_compound_has_its_own_reason_without_a_version_guess and table_58_stays_unknown_and_is_not_an_empty_store only after the probe in (e) exists; they check reasonCode, Refused scope and unknown family rather than prose.

### (b) Review a finite size increase, not an unlimited retry

INFERENCE: the only supplied fact about the oversized input is that it exceeds 32 MiB. Do not pick a new ceiling and claim it admits that input without its exact size. First obtain inputBytes from local metadata, then review root/Quill stream lengths, counted allocations, decoded text, retained sourceText, JSON/document copies and simultaneous worker count. Quantify the worst-case live-memory bound, not only the input buffer. A reviewable bound is workers multiplied by (file buffer + simultaneously live stream buffers + UTF-16 units + decoded/display/document copies + retained sourceText/provenance/JSON + container/index metadata), plus shared process overhead. Assign byte ceilings and copy counts to each term and compare the sum with an explicit memory budget. Character caps also need a byte bound. If needed, separate finite file/stream/decoded-output limits; do not multiply every limit silently.

INFERENCE: allow a reviewed finite raise of MAX_BYTES only when checked arithmetic, count-before-allocation guards, file metadata preflight and explicit stream/output limits still bound memory. Add a counting-reader/allocation test proving an over-budget request is rejected before input-sized reads/reservations, plus boundary cases at the new limit. This task chooses no replacement number and changes no cap.

INFERENCE: the supplied over_limit_probe_refuses_from_metadata_without_reading_input creates only a small invented prefix in memory and extends a file to MAX_BYTES+1 with set_len. It expects over-size-limit, inputBytes/limitBytes and bytesRead=0 before container parsing. No size-sized Vec or unsafe fake slice is built. Physical sparse storage is not guaranteed on every filesystem; the claim is about test memory. The current byte-slice reader alone cannot exercise an allocation-free file-length guard. Un-ignore this test only when the metadata helper is exposed by (e); before then, (b) needs its internal preflight test. A self-reported bytesRead counter alone is not a substitute for instrumented I/O/memory evidence.

### (c) Review exactly the alternate B identity

INFERENCE: consider only the additional pairing of full root UUID 00021200-0000-0000-00c0-000000000046 with table word 94. This is one explicit G03 admission-allowlist extension, not a class mask or a fallback family guess. All remaining G01..G09 and AB01..AB08 checks and failure scopes remain unchanged: header/length/regions, both B table walks and terminator allowlist, directory/prefix/parent/root/extents, exact consumed profiles, Quill catalog/ID/range checks and strict decoding must pass as before. The requested new pairing is the sole identity exception; it cannot honestly be described as an unchanged identity allowlist.

INFERENCE: un-ignore alternate_root_21200_word_94_opens_as_b_after_all_gates and alternate_root_b_keeps_header_and_quill_count_controls together. The positive fixture changes only the root UUID of a complete invented B file. The negative controls keep G02 length rejection and AB04 catalog-count refusal. They are representative controls, not proof that every gate is tested by this one pair; retain the complete existing gate suite. Run the two real candidates locally with unchanged gates and report only aggregate pass/fail counts and structural reasons. Until then their B admission remains UNKNOWN.

### (d) Recognise word 200, refuse extraction

INFERENCE: recognise the common bounded header evidence: old discriminator, +0x02=34, +0x04=200, matching declared Contents length, bounded raw pointer reads, the control marker and the reported directory-before-control shape. Set headerWord04=200, family=legacy-family-unknown, classification=recognised-refused, scope=Refused and reasonCode=header-gen-200. Do this before applying word-300 table/index/directory assumptions or opening a text store. A contradictory common header still needs an appropriate bounds/damage reason; the tag does not certify the whole document.

INFERENCE: un-ignore generation_200_is_recognised_and_refused_before_text once this preflight and (e) exist. The fixture has a 4096-byte Contents, control 3072, directory 256, table pointer 0x34, raw word22=20552 and no Quill. Its body is not a decoded or claimed-valid generation-200 record layout. Do not interpret word22 as a count or use a missing Quill stream to assert emptiness. Table 58 remains profile-unknown with no extraction; no 58-entry walk is proposed.

### (e) Bounded structure-only command

INFERENCE: add file.probeLegacy with a local path parameter, available through the command line's script stdin. It creates no document, performs no text decoding and prints no paths, file/stream names, titles, sourceText, text, raw record regions or private error detail. Use a file-backed bounded reader and metadata preflight rather than reading the entire file to inspect its size. For unknown layouts, expose raw words without inventing semantic counts.

INFERENCE: the supplied tests define this proposed result contract:

| Basis | Fields | Required meaning |
| --- | --- | --- |
| INFERENCE | mode; textDecoded | structure-only; false, even for a supported profile. |
| INFERENCE | inputBytes; limitBytes; bytesRead | Actual file size, configured finite file cap and accounted bytes read. Above the cap return before reading input bytes. |
| INFERENCE | classification; reasonCode; scope; family | not-compound, not-legacy, recognised-refused, over-size-limit or profile-candidate as applicable. Refused inputs have scope Refused; known nonlegacy skips use Skipped; a metadata-only known-profile candidate uses NotDecoded, never Complete text recovery. |
| INFERENCE | isCompound; rootClsid | Validated container presence and UUID metadata, or null if not established. No release inferred. |
| INFERENCE | contentsLength; declaredContentsLength | Actual stream length from container metadata and raw u32 +0x08, when bounded; null if not read. |
| INFERENCE | headerWord02; headerWord04 | Bounded raw u16 values. |
| INFERENCE | controlOffset; directoryOffset; tableOffset | Bounded raw u32 header fields at +0x12/+0x16/+0x1a, with separate range-validity flags where needed. |
| INFERENCE | word22; word24; controlMarker | Raw bounded words at those locations; not universal table semantics. |
| INFERENCE | streamCount; streamLengths; quillPresent; quillLength | Container structural counts/lengths and known store-role presence only; never directory names or store payload. |
| INFERENCE | descriptorCount; recordCount | Only when established by a known validated layout; null for undecoded layouts, never a guessed zero. |

INFERENCE: codes-only data errors should return the structural assessment rather than a blank document. Sanitize I/O errors too; never echo the supplied path. Numeric UUID/header values, fixed schema labels/codes and booleans are structural data; no arbitrary message/content string is needed. Share only this structural response, not ordinary reports from the tester set, which include identities.

INFERENCE: after command registration, un-ignore generation_200_is_recognised_and_refused_before_text, table_58_stays_unknown_and_is_not_an_empty_store, non_compound_has_its_own_reason_without_a_version_guess and over_limit_probe_refuses_from_metadata_without_reading_input. These cases use the dynamic Session command interface so they do not call a nonexistent Rust API at compile time. Before registration an unknown-command failure is a setup block, not evidence about the fixture. Check the no-content-field guard and no-created-document assertion as well as the numbers.

INFERENCE: future owner-local invocation after this command lands, from the implementation checkout, can use the following. No actual input path is recorded here. Do not paste private identities or any ordinary command result containing sourceText.

~~~powershell
# INFERENCE: future command; keep the local path private.
$localProbePath = Read-Host 'Local input path (keep private)'
$probeCommand = @{command='file.probeLegacy'; params=@{path=$localProbePath}} | ConvertTo-Json -Compress
$OutputEncoding = [System.Text.UTF8Encoding]::new($false)
$probeCommand | & '.\target\release\designcraft-cli.exe' script -
~~~

## Test-lane status and smallest next step

OBSERVATION: crates/engine/tests/legacy_profile_shapes.rs adds seven ignored tests using only invented bytes and the existing synthetic A/B fixture helpers: generation 200, table 58, non-compound, alternate-root B acceptance, its rejecting controls, metadata-first oversize refusal, and stable report codes. No binary fixture, tester identity or production change is committed. The report test uses invented temporary names only.

UNKNOWN: compilation, formatting and execution of these new tests are unverified. The execution environment failed before a usable executor was available, so no filesystem corpus count or Cargo command could run. Repository changes were made without that environment. No ignored test is counted as passing; actual over-limit memory behavior and the new command remain pending code.

INFERENCE: the single next smallest step is the typed reason mapping in (a), starting with not-compound and profile-unknown carried through both report paths. That gives useful accurate counters without admitting any new layout. Then proceed through the reviewed budget, exact B alias, recognised word-200 refusal and structure-only probe in the specified order. Probe-dependent tests remain ignored until their prerequisite exists; never turn setup failures into claimed data results.
