# Legacy integration follow-up, round 3

OBSERVATION: this note supersedes the execution-status and portable-path conclusions in [round 2](legacy-followup-2.md). The owner executed the reported checks on Windows against `b1e4dbec22581f7a03249911960d249bf0e171da` with the round-2 test files applied. Evidence is limited to the supplied results, our earlier notes and permitted repository code/tests. No private file was requested or accessed, and no external Publisher reader, third-party format description or prohibited repository was consulted.

OBSERVATION: this branch is cut from the documentation/test branch, which does not contain the experimental production implementation. The engine tests target the b1e4dbec code checkout with its existing synthetic fixture dependency. Only docs and test files change here; no reader, importer, adapter or report-writer logic changes. The configured corpus folders are unavailable; no replacement data was used.

## Executed owner results

| Basis | Check | Owner-observed result |
| --- | --- | --- |
| OBSERVATION | legacy_followup with round-2 tests | 4 passed, 6 ignored. The new standard_positive_raw_pair_keeps_its_own_recovery_size case passed. |
| OBSERVATION | Explicit oversized_square_uses_a4_workspace_and_preserves_raw_pair | Failed its intended assertion at legacy_followup.rs:260: left inferred-from-raw, right default-A4. The proposed size policy is confirmed absent, not an execution unknown. |
| OBSERVATION | Explicit cli_run_and_script_keep_mixed_c_report_fields | Passed with LEGACY_FOLLOWUP_CLI set to the release CLI. |
| OBSERVATION | Earlier component-only batch path fix | Still failed batch.rs:64 on Windows: actual report path has the canonical verbatim prefix, expected path did not. |
| OBSERVATION | Owner's canonicalize-based batch fix | 3 passed, 1 ignored. The canonicalized output root plus native component joins matches the report. |
| OBSERVATION | Other checks in designcraft-pub, designcraft-cli and designcraft-engine | Owner reports all other checks passed: engine library 1107, engine integration 101, CLI integration 28, legacy_open 10. These are the reported group counts, not disjoint counts to sum. |

OBSERVATION: the intended assertion failure proves that b1e4dbec still uses the large inferred workspace; it does not prove the raw field is physically wrong. The four earlier expected-failure requirements for typed batch/folder reports and original-controls/trailing-only warnings remain confirmed open as recorded in round 2. No production fixes for those requirements are claimed here.

OBSERVATION: the owner's JAZZY conversion remains settled at the unit level: raw [18288000,18288000] converts to 1440 points, 508 mm and 120 picas. The mspub full hash is also settled. Neither is an unresolved discrepancy. New execution evidence removes the prior unknowns about compilation and execution of the round-2 tests.

UNKNOWN: native physical page semantics, independent B replication/a real 2000 sample on the PC, the C code page and original reading order remain open. No release-wide compatibility or layout-fidelity claim follows from these counts. There is no verified 2000/B sample on this PC; the earlier repository Sample2000 evidence is still single-file FRAGILE.

## Canonical-path test correction

OBSERVATION: `apps/designcraft-cli/tests/batch.rs:63` now constructs the expected PDF path as `fs::canonicalize(&output).unwrap().join(directory).join(format!("{name}.pdf"))`. The loop and assertion at line 64 remain unchanged from round 2. This applies the exact form the owner reports passing.

OBSERVATION: the CLI canonicalizes its output directory. On Windows the CSV path consequently uses a form such as `\\?\C:\...\output\nested,folder\same.PUB.pdf`. Native component joins fixed the earlier forward-slash mismatch but did not add the verbatim root prefix. Canonicalizing the existing output directory in the test aligns both root representation and separators. On Unix there is no Windows verbatim prefix; the owner identifies this form as portable. No filesystem/report behavior is altered by this assertion change.

INFERENCE: keep this test comparing the current canonical report representation until the product deliberately changes its display format. The passing owner result is evidence for this fix on Windows; no new local run or separate Unix execution is claimed here.

## Product observation: Windows path presentation

OBSERVATION: the CSV's pdf_path currently exposes the Windows verbatim `\\?\` prefix. That is a user-visible report presentation issue, separate from the test's obligation to match the existing implementation. A canonical path is not evidence of a legacy-reader defect.

INFERENCE: the smallest code-lane proposal is a Windows report-display formatter at the point pdf_path is printed: strip the leading `\\?\` from ordinary drive-letter paths only, leaving canonical PathBuf values untouched for filesystem operations. Handle `\\?\UNC\server\share` as `\\server\share`, not `UNC\server\share`; do not blindly strip device/volume namespace forms. Add drive/UNC/ordinary-path display tests and update the CSV assertion for the intentional presentation change. Do not apply a global backslash replacement or change reader routing. This proposal is not implemented here. If reports must preserve verbatim long-path round trips, retain a separate canonical-path value rather than silently losing that capability.

## Positive A-family notice test

OBSERVATION: the new test is `a_family_file_open_and_batch_keep_legacy_notice_and_warning_status` in `crates/engine/tests/legacy_followup.rs`. It writes the permitted byte-built A fixture containing invented A/B text into a temporary directory, opens it through engine file.open, then runs batch-convert on that same temporary input with the explicitly supplied built CLI. No Publisher-made file is used.

INFERENCE: the required passing behavior is family A/Complete and editable A/B text on open, then one batch conversion with warnings and no failures. The test parses CSV quote escaping and embedded newlines, locates columns by header name, and requires the actual notice field to contain the engine's legacy notice, status to equal converted with warnings, and failure_reason to be empty. A whole-report substring match could confuse warnings with the notice column and is deliberately avoided. This checks existing notice propagation, not the still-missing typed legacy fields.

OBSERVATION: the new positive test is ignored only because it requires LEGACY_FOLLOWUP_CLI to name a separately built executable, exactly like the existing positive CLI check. It is not an EXPECTED FAILURE test and must be run explicitly when that prerequisite is available. Missing CLI, export/setup failure or zero selected tests is not a pass. No new Cargo dependency or manifest change is needed in the implementation checkout.

UNKNOWN: this new test has not run in this environment or in the supplied owner results. Both local requested commands, `cargo test -p designcraft-engine` and `cargo test -p designcraft-cli`, stop with exit 127: `/bin/bash: line 1: cargo: command not found`. This local tool limitation does not undo the owner's executed round-2 evidence. Compilation/runtime of the newly added test and local formatting remain unverified; whitespace checking is not a substitute.

INFERENCE: in the owner's implementation checkout with these test patches applied, build the matching CLI, set its absolute path, and explicitly run the positive notice case. Keep the expected-failure cases ignored in ordinary runs until their requirements are implemented.

```powershell
# INFERENCE: owner-local verification; the new test creates only invented temporary data.
Set-Location 'E:\Projects\NiftyDesign-src'
git rev-parse HEAD
cargo build --release -p designcraft-cli
if ($LASTEXITCODE -ne 0) { throw 'Build failed; do not use a stale CLI.' }
$env:LEGACY_FOLLOWUP_CLI = (Resolve-Path '.\target\release\designcraft-cli.exe').Path
cargo test -p designcraft-cli --test batch recursive_batch_continues_after_bad_input_and_writes_both_valid_formats -- --exact
cargo test -p designcraft-engine --test legacy_followup a_family_file_open_and_batch_keep_legacy_notice_and_warning_status -- --ignored --exact
```

## Self-contained task for the owner's code lane

INFERENCE: apply the following in the experimental implementation checkout, starting from b1e4dbec plus the test patches. Keep the current importer-first routing and strict reader gates unchanged. Use only synthetic test fixtures for these changes; no private-file transfer or decoding research is needed. Make small commits and treat each test's intended assertion as the acceptance condition, not an ignore count or a warning substring elsewhere.

1. INFERENCE: pass the typed legacy object through the batch worker result, serialization/deserialization, CSV writer and each folder report entry. CSV must expose the fields required by the existing test; encode structured values such as provenance and recoveryPageSize as properly quoted JSON cells. Folder entries must retain a nested legacy object. Preserve these exact names: readerRevision, route, family, scope, openResult, segmentCount, decodedCount, encodingUnknownCount, undecodedCount, rawSizePair, sizeStatus, recoveryPageSize, withheldCount, provenance and storageOrderOnly. Preserve existing warnings, notes and notice columns separately; never put private decoded text into these structural reports. Un-ignore `batch_report_has_parity_with_mixed_c_command_result` and `folder_report_has_parity_with_mixed_c_command_result` when the built CLI is supplied; both must turn green. Keep `cli_run_and_script_keep_mixed_c_report_fields` and the new A notice test green. Strengthen parity checks to compare values as well as field presence before claiming full parity.
2. INFERENCE: count removal of trailing CR/LF paragraph marks even when there is no FF or other control. Un-ignore `removed_trailing_paragraphs_alone_are_reported`; it must turn green while `controls_are_kept_by_reader_but_only_tab_is_shown_by_adapter` remains green. Preserve current notice fields and the warning about omitted controls/trailing paragraph marks in top-level warnings/import notes. A warning is not lossless storage: `original_control_sequence_is_retained_after_open` must remain an explicit expected failure until the separate source-text metadata design from round 2 is implemented.
3. INFERENCE: implement the proposed default plausibility policy in the adapter, not the reader: a positive raw candidate outside the inclusive 50..450-mm range on either inferred dimension defaults to an A4 workspace. Keep rawSizePair unchanged, report sizeStatus=default-A4, set recoveryPageSize to the generated approximately 210 x 297 mm page, and include exactly “Original page size is unverified. A standard A4 recovery page is being used.” Preserve the separate unknown-size handling when no validated raw candidate exists. Un-ignore `oversized_square_uses_a4_workspace_and_preserves_raw_pair`; it must turn green for its 500-mm square. Keep `standard_positive_raw_pair_keeps_its_own_recovery_size` green for 215.9 x 279.4 mm. Add inclusive-boundary, lower-bound and single-axis cases before claiming full policy coverage; a large original may be legitimate, so no verified-size claim is introduced.
4. INFERENCE: include the real build commit next to the package version **inside readerRevision**, for example `designcraft-pub-legacy <version>; commit=<full-build-SHA>; dirty=<status>`. Use build-time provenance, not the app's runtime working directory or a hard-coded b1e4dbec. Preserve explicit unknown for builds without commit provenance. Propagate the same readerRevision through command, batch and folder outputs. The existing ignored `cli_run_and_script_keep_mixed_c_report_fields` can be un-ignored only with its built-CLI prerequisite, but currently passes on version/field presence alone; strengthen its shared assertion with an independently supplied expected build SHA and add revision-equality checks to both parity tests. `report_fields_follow_the_shared_schema` is already active and also needs the stronger assertion; there is no existing ignored test that independently proves a real build SHA. Do not claim an unmodified old test verifies the new provenance requirement.

INFERENCE: the single next smallest production step is retaining an optional legacy object in the batch worker Outcome and its serialized round trip. A worker round-trip assertion should preserve family and the segment counts before expanding CSV/folder output; the full parity tests will remain red until their writer paths are complete. This is the first bounded part of item 1, not a reason to claim the whole item green.

UNKNOWN: physical page meaning, C encoding and reading order still need independent evidence, and the PC still lacks a verified B/2000 sample. Missing typed reports, lossless controls, trailing-only warning and page policy are confirmed implementation gaps, not mysterious format unknowns. This round fixes only the portable assertion and adds a positive test; none of those production gaps is silently marked fixed.
