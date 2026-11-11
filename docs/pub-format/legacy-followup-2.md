# Legacy integration follow-up, round 2

OBSERVATION: [round 3](legacy-followup-3.md) supersedes this round's execution unknowns for the two size cases and its incomplete Windows path fix. The owner executed the new tests and confirmed a canonical-output-root correction; the historical round-2 results below remain distinct.

OBSERVATION: this note records owner-executed Windows evidence against implementation commit `b1e4dbec22581f7a03249911960d249bf0e171da` with the earlier `legacy_followup.rs` applied. It updates [round 1](legacy-integration-followup.md). Evidence is the owner's supplied numbers, the permitted repository tests and our earlier notes. No private file was requested, downloaded, uploaded or run here; no external Publisher reader, third-party format description or prohibited repository was consulted.

OBSERVATION: this documentation branch starts from the earlier follow-up branch, not the implementation branch. The engine test file targets the b1e4dbec implementation and its existing synthetic fixtures/dependencies. Changes here are confined to docs and tests; no manifest, reader, importer or adapter logic is changed. The configured corpus directories are unavailable and no substitute data was used.

## Executed Windows results

| Basis | Owner command/check | Result and exact limit of the claim |
| --- | --- | --- |
| OBSERVATION | Ordinary legacy_followup integration target | 3 passed, 5 ignored: all_unknown_c_retains_every_placeholder_and_is_not_empty, mixed_c_retains_one_decoded_frame_and_one_placeholder, controls_are_kept_by_reader_but_only_tab_is_shown_by_adapter passed. The file compiled. |
| OBSERVATION | Explicit CLI test with LEGACY_FOLLOWUP_CLI set to release CLI | cli_run_and_script_keep_mixed_c_report_fields passed. Command-result schema and the mixed-C invariant passed for this invented file. |
| OBSERVATION | Explicit original_control_sequence_is_retained_after_open | Failed its intended assertion at legacy_followup.rs:124, confirming original control sequence is not retained in the opened document/report. |
| OBSERVATION | Explicit removed_trailing_paragraphs_alone_are_reported | Failed its intended assertion at legacy_followup.rs:132, confirming the trailing-only warning gap. |
| OBSERVATION | Explicit batch_report_has_parity_with_mixed_c_command_result | Failed its intended assertion at legacy_followup.rs:203, confirming missing typed legacy CSV fields. |
| OBSERVATION | Explicit folder_report_has_parity_with_mixed_c_command_result | Failed its intended assertion at legacy_followup.rs:222, confirming the missing per-file legacy JSON object. |
| OBSERVATION | Full cargo test --no-fail-fast -p designcraft-pub -p designcraft-cli -p designcraft-engine | Owner reports everything passed except recursive_batch_continues_after_bad_input_and_writes_both_valid_formats, apps/designcraft-cli/tests/batch.rs:64. This is a Windows expected-path string mismatch, not a legacy-reader failure. |
| OBSERVATION | legacy_open integration target in that full run | 10 passed. |
| OBSERVATION | Engine library in that full run | 1107 passed, 2 ignored. No unreported CLI/importer aggregate count is inferred. |

OBSERVATION: the explicit expected-failure runs failed on their requirement assertions, not compilation, file creation, export or CLI setup. The earlier three-pass/five-ignored result, later CLI pass and four intentional assertion failures are separate runs. They must not be described as one all-green suite. Their source line numbers refer to the owner's applied version before this round's appended tests.

OBSERVATION: the owner has Git, Rust and the MSVC toolchain on Windows. Earlier “Cargo unavailable” execution unknowns for these eight tests are superseded by the results above and corrected in round 1. This environment still cannot execute Cargo; that affects the two newly added tests and local re-verification, not the already executed Windows evidence.

## Settled corrections and remaining evidence

OBSERVATION: JAZZY's returned rawSizePair is [18288000,18288000]; recoveryPageSize is 1440 points = 508 mm; the default ruler displays 120 picas because a pica is 12 points. The owner confirms that unit interpretation. There is no unit bug in this observed conversion. sizeStatus is inferred-from-raw, readerRevision is “designcraft-pub-legacy 0.18.0-nifty-alpha.5”, withheldCount is unknown, scope is Partial, and the counts are 72 segments = 64 decoded + 8 undecoded.

UNKNOWN: whether that root raw pair describes the native physical page, a larger workspace or another size remains unproven. Correct unit conversion is not proof of field semantics. Independently recorded native page settings and known-size saves, checked locally without transferring files, would distinguish these interpretations. A plausibility fallback is a workspace policy, not a format correction.

OBSERVATION: the owner corrected the earlier brief's mspub prefix typo. Its full SHA-256 is `0eff0498172805bf1e52c74a6334ecc4cdc84fb2e531c0ea25734cfac7923c7d`, exactly the pinned value. The prior `0eff04981722` transcription was incorrect. Identity is settled and removed from UNKNOWN and the next-step queue. The earlier 4152-byte Contents/zero-SYID evidence still supports the narrow EmptyKnownStore result, not a claim that every possible publication text store is decoded.

UNKNOWN: physical page semantics, C code-page identity, text beyond the supported stores and independent populated A/B replication remain open. There is still no Publisher 2000 sample on this PC. Neither test success nor generated page/object counts establishes original page order, fonts, tables, pictures or layout fidelity.

## Portable batch-test fix

OBSERVATION: only the expected-path part of recursive_batch_continues_after_bad_input_and_writes_both_valid_formats changes in `apps/designcraft-cli/tests/batch.rs`, lines 61–63. The PDF expectation at line 63 now uses `output.join(directory).join(format!("{name}.pdf"))`, with directory and filename separate in the loop. The corresponding input check also joins components. The assertion at line 64 remains unchanged.

OBSERVATION: previously the expected string contained `nested,folder/same.PUB.pdf`, whereas Windows serialized the path with `nested,folder\same.PUB.pdf`. Filesystem reads accepted the mixed separators, but the CSV substring assertion did not. Native component joining produces the expected separator on both Windows and Unix. No CSV generation, normalization or reader behavior changes. The owner's round-3 run showed this component-only fix was insufficient: the CLI reports a canonical verbatim Windows root. Round 3 applies fs::canonicalize to the expected output root; the owner reports 3 passed and 1 ignored with that correction.

## Byte-built page-policy tests

INFERENCE: these tests specify the earlier proposed plausibility policy, not an already decoded format rule. Use the permitted synthetic A builder with invented text and metadata; no native Publisher file is embedded. A suffices for this adapter-level check; C is not independently exercised by the new policy cases.

| Basis | Added test | Assertions and expected b1e4dbec behavior | Execution status |
| --- | --- | --- | --- |
| OBSERVATION | standard_positive_raw_pair_keeps_its_own_recovery_size | Raw pair [7772400,10058400] remains in metadata; Complete/two decoded segments; inferred-from-raw; generated 612 x 792 pt = 215.9 x 279.4 mm; uncertainty warning remains. Owner confirms this existing behavior passes. | PASS in owner round-3 run (ordinary target: 4 passed, 6 ignored). |
| OBSERVATION | oversized_square_uses_a4_workspace_and_preserves_raw_pair | Invented [18000000,18000000] means 500 x 500 mm, above 450 mm. Require Complete/two segments, raw pair preserved, default-A4, generated approximately 210 x 297 mm, and the exact warning below. The required fallback is absent. | Owner explicitly ran it: failed its own assertion at line 260, inferred-from-raw versus default-A4; confirmed policy gap. |

INFERENCE: the required fallback warning is “Original page size is unverified. A standard A4 recovery page is being used.” A positive but implausible candidate must stay in rawSizePair; only the generated workspace changes. Legitimate posters can exceed 450 mm, so this is a conservative default, not evidence that their metadata is wrong. The broader proposed 50..450-mm rule and its exact boundaries, one-large-axis cases and absent-size cases need additional tests before a claim of complete policy coverage. No policy is implemented by this test-only task.

OBSERVATION: the earlier local Cargo limitation is superseded for these test outcomes by the owner's round-3 execution: the ordinary target is 4 passed/6 ignored, the CLI field check passes, and the explicit oversized case fails on its intended assertion. The component-only path fix still fails on Windows; the canonical-root form passes there. No local execution or separate Unix run is claimed. `git diff --check` verifies whitespace, not compilation; rustfmt was not run here.

## Smallest lossless-control design

INFERENCE: retain the reader's exact decoded segment string before `visible()` in persistent recovery metadata, alongside its source provenance and generated-frame/story identity. A proposed record is `legacyRecovery.segments[i] = {source, frameId, originalText}`. originalText includes tab, FF, other accepted controls, CR/LF and trailing paragraph marks exactly. It is immutable source evidence, not a replacement for editable frame text; edits must not silently rewrite it or claim it is the latest edited story. Keep visibleText as the current editable representation. No per-character mapping is needed merely to preserve the original sequence.

INFERENCE: the metadata must round-trip through native save/load; a transient local variable or only a range/ID does not preserve source text. If a later command offers the original sequence, make it an explicit owner-local recovery payload. Keep default batch/folder reports structural and redact originalText before report sharing. This source text is private document content, not an error message. For encoding-unknown segments, retain labelled source range/hash provenance and no invented originalText; a lossless unknown-byte payload would require a separate, explicitly private design.

INFERENCE: storing the exact string in serialized document metadata would turn `original_control_sequence_is_retained_after_open` green because that test searches document/report values for the original string. It would not turn the other three existing expected-failure tests green: retaining text does not fix the trailing-only warning, CSV fields or folder JSON schema. It also does not fix the new page-policy test, displayed-control choices, composition-based overflow detection or diagnostics. Add a save/reload and edit-isolation test when the metadata implementation exists; the current test alone cannot prove persistence or edit tracking.

UNKNOWN: the precise persistent metadata API and format-compatibility treatment must be selected by the code lane. This note proposes field semantics only and adds no production storage or report code.

## Ordered code-lane plan

1. INFERENCE: single next smallest step: carry the typed legacy object from file.open through the batch worker Outcome, its serialization/deserialization and report writer. Add stable CSV columns/encoding for every field expected by `batch_report_has_parity_with_mixed_c_command_result`, including counts, raw pair, provenance and revision information; warnings alone are insufficient. Then carry that same object into each folder report entry. Make the batch and folder parity tests green separately and remove their expected-failure ignores. The existing run/script test must remain green. Preserve unavailable values as unknown rather than guessing zero.
2. INFERENCE: compare original text against its displayed transformation, or explicitly detect removed trailing CR/LF, to report trailing-only paragraph removal. Make `removed_trailing_paragraphs_alone_are_reported` green while the existing combined-control display test remains green. In a separate small change add the lossless source metadata above and make `original_control_sequence_is_retained_after_open` green; a warning is not preservation.
3. INFERENCE: implement the chosen page-size plausibility policy in the adapter, retaining the raw pair and using the exact unverified-size/A4 warning. Make `oversized_square_uses_a4_workspace_and_preserves_raw_pair` green, remove its expected-failure ignore and keep `standard_positive_raw_pair_keeps_its_own_recovery_size` green. Add threshold/axis tests before claiming full policy coverage. Do not alter field decoding to force a standard size.
4. INFERENCE: add an actual implementation build commit beside the existing package/version field, for example `readerBuildCommit`, propagated through command, batch and folder reports. Define unknown for builds lacking provenance and include a dirty/test-patch indication rather than pretending an uncommitted patch is a clean commit. Existing report-field tests check only presence/package label, so they cannot prove this improvement. Add a proposed test named `reported_build_commit_matches_build_metadata`, comparing the reported commit against explicitly supplied build metadata, plus parity assertions in all report paths; that new test, not a version-prefix assertion, should go green. It is not implemented by this task. Extend report-parity assertions to compare field values as well as their presence before calling the report paths equivalent.

INFERENCE: first apply the isolated portable-test fix and re-run the Windows failure by name. This clears an unrelated red test before the code-lane implementation sequence; it does not change the next smallest production step, typed legacy state through the batch worker. Preserve all deliberate red expected-failure checks until their corresponding requirements are implemented.

## Updated Windows procedure

OBSERVATION: the PC now has Git and the Rust/MSVC toolchain. The implementation repository is `E:\Projects\NiftyDesign-src`, the app is `target\release\designcraft.exe`, and the seven chosen old files are in `E:\Projects\quarantine\legacy-intake\in`. The [round-1 procedure](legacy-integration-followup.md#revised-windows-procedure) is updated with these paths and remains the complete local capture script. Use the implementation checkout with the intended test patches, not this documentation-only branch. Record the source commit/test-patch state as well as the alpha.5 package version.

INFERENCE: the following command sequence fixes the known PC paths. Build failure stops the procedure; do not silently use an older shipped executable. Choose a new output folder. This produces converted copies locally, not publication of the input files.

```powershell
# INFERENCE: owner-local implementation checkout; no input is uploaded.
Set-Location 'E:\Projects\NiftyDesign-src'
git rev-parse HEAD
git status --short
cargo build --release -p designcraft -p designcraft-cli
if ($LASTEXITCODE -ne 0) { throw 'Build failed; do not use stale executables.' }
$legacyCli = (Resolve-Path '.\target\release\designcraft-cli.exe').Path
$legacyApp = (Resolve-Path '.\target\release\designcraft.exe').Path
$legacyInput = 'E:\Projects\quarantine\legacy-intake\in'
& $legacyCli --version
$legacyOut = Read-Host 'New local output folder for this run'
if (Test-Path -LiteralPath $legacyOut) { throw 'Choose a new output folder.' }
New-Item -ItemType Directory -Path $legacyOut | Out-Null
$legacyFiles = @(Get-ChildItem -LiteralPath $legacyInput -File | Where-Object { $_.Extension -ieq '.pub' })
if ($legacyFiles.Count -eq 0) { throw 'No .pub files were selected.' }
$legacyFiles | ForEach-Object {
  [pscustomobject]@{Name=$_.Name; Bytes=$_.Length;
    SHA256=(Get-FileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash}
} | Export-Csv -NoTypeInformation -Path (Join-Path $legacyOut 'identity-before.csv')
& $legacyCli batch-convert $legacyInput --out (Join-Path $legacyOut 'batch') `
  1> (Join-Path $legacyOut 'batch-summary.json') 2> (Join-Path $legacyOut 'batch-errors.txt')
Write-Output ('Batch exit=' + $LASTEXITCODE)
$OutputEncoding = [System.Text.UTF8Encoding]::new($false)
foreach ($legacyFile in $legacyFiles) {
  $legacyCommand = @{command='file.open'; params=@{path=$legacyFile.FullName}} | ConvertTo-Json -Compress
  $legacyStem = Join-Path $legacyOut $legacyFile.Name
  $legacyResult = $legacyCommand | & $legacyCli script - 2> ($legacyStem + '.stderr.txt')
  $legacyExitCode = $LASTEXITCODE
  $legacyResult | Set-Content -LiteralPath ($legacyStem + '.command.json') -Encoding UTF8
  Write-Output ($legacyFile.Name + ': command exit=' + $legacyExitCode)
}
```

INFERENCE: compare `batch/report.csv` with each command report's `results[0].legacy` and top-level warnings/notice. Record route, family, scope, segment/decoded/undecoded/unknown counts, withheldCount, rawSizePair, recoveryPageSize with points and mm, sizeStatus, package/revision, provenance and storageOrderOnly. Require segmentCount=decodedCount+undecodedCount. Record absent typed batch fields as absent; the owner run confirms that gap. Once the policy change exists, a generated A4 size must not replace the raw candidate in metadata.

INFERENCE: after applying the test patches to the implementation checkout, verify the portability fix and new cases with the following exact target/name selections. The first two should pass; the last is deliberately expected to fail at b1e4dbec until policy code exists. A zero-selected-test run is not a pass.

```powershell
# INFERENCE: run in E:\Projects\NiftyDesign-src after applying the test-only patches.
cargo test -p designcraft-cli --test batch recursive_batch_continues_after_bad_input_and_writes_both_valid_formats -- --exact
cargo test -p designcraft-engine --test legacy_followup standard_positive_raw_pair_keeps_its_own_recovery_size -- --exact
cargo test -p designcraft-engine --test legacy_followup oversized_square_uses_a4_workspace_and_preserves_raw_pair -- --ignored --exact
```

INFERENCE: launch the app from the resolved legacyApp path and use File > Open on the chosen local files. Record the exact window title and build source, expand Import notes and capture a redacted screenshot of the notice/notes. Record page size from File > Document Setup and the selected ruler unit, plus zoom. The JAZZY 120-pica/1440-point/508-mm relationship is already settled; recheck it only to verify the current build or a policy change, not as an unresolved conversion bug. Cancel setup without saving the original. Recompute input hashes afterwards and compare with identity-before.csv.

INFERENCE: a Partial file should show decoded frames/placeholders, an all-unknown file Partial/placeholders, and an empty supported store its explicit empty-workspace notice. Failure includes crash, old refusal from an unintended binary, blank replacement on failed open, missing notice, changed source hash, mismatched counts, lost controls without preservation, schema omissions, or app/report size disagreement after matching units. CATALOG's earlier 61-character generated-frame overflow remains a fit warning to investigate, even with converted-with-warnings status. Return only numeric identity/results and redacted screenshots, never files or paragraphs.

OBSERVATION: there is still no Publisher 2000 sample on this PC. INFERENCE: a local independently saved 2000 file would need to pass the B structural profile before it proves B replication: parsed A/B root UUID, old Contents discriminator/length, table word 94 with 92 entries indexed 2..93, both allowed-terminator walks agreeing through the exact control boundary, bounded directory/record profiles, and valid local Quill links/ranges for populated text. A filename or application-version label alone is insufficient; a different structural family remains a different result. Known physical page settings and independently authored short story lengths would test the remaining semantics without transferring content. Keep B's single-file FRAGILE label until independent relevant checks actually replicate.

OBSERVATION: round 3 confirms execution of the size tests and the canonical-root assertion correction; lossless controls, typed report parity, trailing-only warning and the size policy remain confirmed implementation gaps. UNKNOWN: real physical page identity, independent B replication/a 2000 sample on this PC, C code page and reading order remain open. Unit conversion, mspub identity and the reported test execution are no longer UNKNOWN.
