# Legacy integration follow-up

OBSERVATION: updated by [round 2](legacy-followup-2.md) with the owner-executed Windows results against b1e4dbec. Those results supersede the earlier test-execution and identity unknowns; the original source comparison remains pinned to that revision.

OBSERVATION: this follow-up reviews the owner's implementation at `b1e4dbec`, superseding the source conclusions pinned to `9b4cf49f` in [the previous integration check](legacy-integration-check.md). Evidence is limited to this repository's notes, the explicitly permitted implementation/tests and the owner's Windows observations in this task. No private Publisher file was requested, downloaded, uploaded or run. Source review and owner execution are distinguished below.

OBSERVATION: the requested branch starts from the documentation branch, which does not contain the experimental reader or its engine dependency. Only this note and `crates/engine/tests/legacy_followup.rs` are added. The test file targets the implementation at b1e4dbec and must be applied there for compilation; no reader/importer logic or manifest was copied into this branch. A separate detached checkout at that revision was prepared for verification. The configured corpus folders are unavailable; no substitute corpus was used.

## Owner Windows observations

OBSERVATION: the owner reports a successful Windows `cargo build --release -p designcraft -p designcraft-cli` of b1e4dbec, exit 0, with version “NiftyDesign CLI 0.18.0-nifty-alpha.5”. Shipped alpha.5 binaries still refuse these files; the version string alone does not identify this implementation. The owner-built batch run converted seven files with warnings and zero failures. The following are owner-reported generated pages/objects, not native publication counts.

| Basis | File | Bytes / reported SHA-256 prefix | Generated pages / objects | Profile / inferred mm | Distinct owner result |
| --- | --- | --- | --- | --- | --- |
| OBSERVATION | CATALOG.PUB | 79360 / bc95ef2569e8 | 9 / 28 | C / 139.7 x 215.9 | Page 5: 61 characters do not fit the story in frame 45. |
| OBSERVATION | JAZZY.PUB | 530944 / 49933de05204 | 3 / 72 | C / 508 x 508 | Eight undecoded segments; 64 segments with omitted controls/paragraph marks. |
| OBSERVATION | MODERN.PUB | 337408 / aa446e31e352 | 3 / 74 | C / 508 x 508 | Nine undecoded; 65 with omitted controls. |
| OBSERVATION | PLAIN.PUB | 343552 / 48384326430f | 3 / 88 | C / 558.8 x 558.8 | Ten undecoded; 78 with omitted controls. |
| OBSERVATION | SCHOOL.PUB | 88064 / e859a1a272e0 | 6 / 30 | C / 215.9 x 279.4 | One undecoded; 29 with omitted controls. |
| OBSERVATION | mspub.pub | 19968 / 0eff04981728 | 1 / 0 | A / 215.9 x 279.4 | Supported-store empty notice; full identity confirmed in round 2. |
| OBSERVATION | zmspub.pub | 17920 / 8a7962fc897a | 1 / 0 | A / 210 x 297 | Supported-store empty notice. |

OBSERVATION: every owner result also includes opaque-record, raw-size uncertainty, estimated-page-size and missing-original-layout/keep-original warnings. The CSV contains only `file,pages,objects,status,warnings,pdf_path,failure_reason,notes,notice`. It has no typed legacy object or columns. On the desktop, JAZZY opened with title “NiftyDesign — JAZZY”, a partial notice, collapsed “Import notes (8)” and plain stacked boxes including an unreadable-text placeholder. The owner transcribed its brackets as parentheses; the pinned source literal uses square brackets. This does not establish a content/encoding discrepancy.

OBSERVATION: the owner corrected a typo in the earlier brief: mspub.pub has full SHA-256 `0eff0498172805bf1e52c74a6334ecc4cdc84fb2e531c0ea25734cfac7923c7d`, exactly the pinned identity. The earlier `0eff04981722` prefix was a brief error, not a second file or an unresolved hash discrepancy. No file transfer was needed.

## Re-check of every earlier comparison row

INFERENCE: MATCH, STRICTER and LOOSER retain their earlier meanings. Each row below corresponds to one earlier comparison row, in the same order. Closed means a particular old gap is closed, not that all text/layout is recovered. Command-result matches apply to admitted results; typed whole-file refusal reports remain unavailable.

| Basis | Earlier row | b1e4dbec command/app | Batch/folder, where applicable | Closed, open or new difference |
| --- | --- | --- | --- | --- |
| OBSERVATION | Routing 1: current importer first, success wins | MATCH | N/A | Unchanged current-importer-first behavior; successful modern import wins. |
| OBSERVATION | Routing 2: explicit older-family refusal, not generic Unsupported | LOOSER | N/A | Still any Unsupported plus looks_legacy; no distinct typed older-refusal cause. Open. |
| OBSERVATION | Routing 3: safe container and exact old prefix | MATCH | N/A | Bounded V3/root-Contents old-prefix probe unchanged; delegated unique-container resolution remains unverified. |
| OBSERVATION | Routing 4: separate gated reader, same bytes | MATCH | N/A | Still invokes the gated separate reader on the original bytes, without relaxing its profile. |
| OBSERVATION | Routing 5: no blank recovery on refused/empty Partial result | MATCH | N/A | Zero retained segments refuse except EmptyKnownStore; size alone never opens a document. |
| OBSERVATION | Routing 6: explicit empty workspace; failed open preserves existing document | MATCH | N/A | Unchanged behavior; a new integration test now compares the prior document after refusal. Closed test gap, not a new implementation rule. |
| OBSERVATION | Available experimental path versus disabled/unavailable path | MATCH | N/A | Compiled-in route unchanged; no optional disable feature is required or provided. |
| OBSERVATION | Failed attempted recovery preserves original refusal and appends reason | LOOSER | N/A | Improved: zero-result failures search non-size/non-scope warnings before generic fallback. Still string-based selection can choose an opaque warning before the useful cause; construction errors retain their separate bad(file.open) path. No complete sanitization/shared-detail guarantee. Partly closed. |
| OBSERVATION | Successful banner plus shared detail | MATCH | N/A | Copy unchanged. Owner observed the partial path, not every successful banner state. |
| OBSERVATION | Partial banner | MATCH | N/A | Copy unchanged; owner observed it on JAZZY. Mixed/all-unknown integration tests now exist in b1e4dbec. |
| OBSERVATION | Partial details/counts | LOOSER | N/A | Counts remain in command JSON, not a complete visible breakdown; withheldCount now explicitly unknown. Batch omits typed counts. Still open. |
| OBSERVATION | Refusal copy | LOOSER | N/A | Appended experimental reason remains, with better fallback selection; construction wrapping and sanitization still open. |
| OBSERVATION | Empty known-store copy | MATCH | N/A | Unchanged wording; owner observed it on the two A opens, with the mspub identity now confirmed. |
| OBSERVATION | Undecoded placeholder | MATCH | N/A | Square-bracket literal unchanged; mixed/all-unknown tests added. Owner observed a placeholder with parentheses in the transcription. |
| OBSERVATION | Estimated-size and fallback-size warnings | MATCH | N/A | Unchanged copy; owner confirms estimated-size uncertainty warnings. A4 fallback not exercised in that run. |
| OBSERVATION | Long-text warning | LOOSER | N/A | Estimator and approximate trigger unchanged. Owner measured CATALOG composition overflow; actual fit is not guaranteed. Still open. |
| OBSERVATION | Persistent, accessible notice/details | MATCH | N/A | Same banner/collapsible notes. Owner observed banner and collapsed Import notes (8); expanded notes/accessibility still need the revised check. |
| OBSERVATION | Unsaved recovery; original never overwritten | MATCH | N/A | Same unsaved DocState and native-extension suggestion; no owner source-hash-after comparison supplied. |
| OBSERVATION | One frame per retained segment in storage order | MATCH | N/A | Unchanged. Owner object counts agree with C segment totals and zero-object empty workspaces, without proving reading order. |
| OBSERVATION | Undecoded-only result | MATCH | N/A | Unchanged Partial/placeholders behavior; new implementation test covers it. No all-unknown private-file run claimed. |
| OBSERVATION | Plain pages and sequential placement | MATCH | N/A | Same generated sequence/default style, now adaptive margin=min(36,w/4,h/4). No original-layout inference. |
| OBSERVATION | Raw candidate conversion and positive usable page | MATCH | N/A | Same raw/12700 points and 72..15552-point admissibility; mm report fields added. Physical field meaning remains UNKNOWN. |
| OBSERVATION | Frames respect generated page margins | MATCH | N/A | Closed: adaptive margins leave positive body and margins on the accepted 72-point minimum. New tiny-page test checks interior bounds. |
| OBSERVATION | Full long text retained; no silent truncation | LOOSER | N/A | Not closed: visible() still removes FF/other low controls and trailing paragraph marks; only a notice was added. It retains ranges, not original text. New trailing-only warning miss explained below; estimator remains approximate. |
| OBSERVATION | No fabricated marker-based pagination | MATCH | N/A | FF still does not become original pagination; its removal is the separate preservation gap. |
| OBSERVATION | Empty workspace versus refusal | MATCH | N/A | Same one-page/no-frame empty state; no document on reader refusal. New empty integration test exists. |
| OBSERVATION | Bounded generated layout | STRICTER | N/A | 2000-generated-page limit unchanged; still additional to the proposal, with explicit refusal rather than truncation. |
| OBSERVATION | reader_revision | LOOSER | LOOSER | Field added to command result, but readerRevision is package/version, not implementation commit. Batch/folder still omit it. |
| OBSERVATION | route | MATCH | LOOSER | Command legacy.route unchanged; batch/folder still drop it. |
| OBSERVATION | family | MATCH | LOOSER | Command family unchanged on success; batch/folder still have only warning text. Refusal remains untyped. |
| OBSERVATION | scope | MATCH | LOOSER | Command success scope unchanged; batch/folder omit it. |
| OBSERVATION | open_result | MATCH | LOOSER | Command openResult unchanged; failed open remains an error, not typed refused data. |
| OBSERVATION | segment_count | MATCH | LOOSER | Command segmentCount unchanged; batch/folder omit it. |
| OBSERVATION | decoded_count | MATCH | LOOSER | Command decodedCount unchanged; batch/folder omit it. |
| OBSERVATION | encoding_unknown_count | MATCH | LOOSER | Command flag count unchanged; batch/folder omit it. |
| OBSERVATION | undecoded_count | MATCH | LOOSER | Command subtraction guarantees N=decoded+undecoded; batch/folder omit all three. |
| OBSERVATION | withheld_count | MATCH | LOOSER | Closed on command success: withheldCount explicitly unknown. Distinct failed-link/unlinked/duplicate counters remain untyped; batch/folder omit even unknown. |
| OBSERVATION | raw_size_pair | MATCH | LOOSER | Command rawSizePair preserved, independent of generated-page default; batch/folder omit it. |
| OBSERVATION | size_status and recovery_page_size | MATCH | LOOSER | Closed success schema gap: point values retain unit=pt and now widthMm/heightMm explicitly carry mm. Batch/folder omit all of them; refusal has no typed unavailable value. |
| OBSERVATION | warnings and refusal diagnostics | LOOSER | LOOSER | Warnings survive, but are free text without every v2 diagnostic field. Improved reason fallback does not close construction wrapping/sanitization. Owner confirms warnings-only batch data. |
| OBSERVATION | provenance and storage-order label | MATCH | LOOSER | Command ranges/IDs plus storageOrderOnly unchanged. No original control sequence added; batch/folder still omit provenance. |

OBSERVATION: the added `trimmed_segments` counter does not preserve source characters. It counts segments containing a non-tab/non-CR/LF low control, or a difference in non-newline/non-tab character counts. Removing trailing CR/LF alone does not satisfy either condition. Thus the new warning covers FF-bearing C segments and the requested combined-control fixture, but can miss a story that loses only its trailing paragraph marks. The immutable source file still contains the original data; the opened document/report does not retain that complete sequence.

OBSERVATION: the overset estimator still assumes about six points per character and 14.4 points per line. The owner reports a measured composition warning in CATALOG despite successful conversion. That is evidence of a generated-frame fit issue, not a native text-range or encoding failure. This review cannot establish whether the adapter's additional approximate warning appeared in that particular file; it must not claim exact overflow counts from the estimator.

## Page-size interpretation

OBSERVATION: the reader takes positive u32 values at root record +0x14/+0x18 only after its root-profile/extent checks. Earlier census values and the current conversion are:

| Basis | Files | Raw width / height | Points, raw/12700 | Inferred mm, raw/36000 |
| --- | --- | --- | --- | --- |
| OBSERVATION | CATALOG | 5029200 / 7772400 | 396 / 612 | 139.7 / 215.9 |
| OBSERVATION | JAZZY, MODERN | 18288000 / 18288000 | 1440 / 1440 | 508 / 508 |
| OBSERVATION | PLAIN | 20116800 / 20116800 | 1584 / 1584 | 558.8 / 558.8 |
| OBSERVATION | SCHOOL; previously pinned mspub | 7772400 / 10058400 | 612 / 792 | 215.9 / 279.4 |
| OBSERVATION | zmspub | 7560000 / 10692000 | about 595.276 / 841.890 | 210 / 297 |

OBSERVATION: `size_candidate` computes inferred_mm=raw/36000. `legacy_open::page_size` converts that to points by multiplying by 72/25.4, algebraically raw/12700. The report now supplies points with explicit `unit=pt` and separate widthMm/heightMm. These conversions are internally consistent. The accepted adapter range remains 72..15552 points per dimension, about 25.4..5486.4 mm; it is a safety bound, not a test of a standard publication size.

OBSERVATION: `NewDocument::default` selects Picas; `Document::new` copies its units to both ruler settings. The adapter does not override that default. In `ui-egui/src/canvas/rulers.rs` labels use `unit.from_pt` while zoom determines tick spacing and screen position. `geom/src/units.rs` defines 12 points per pica. Consequently 1440 points = 120 picas = 508 mm; 1584 points = 132 picas = 558.8 mm. Zoom at 44% changes screen width, not the physical unit or ruler value. The default therefore explains a width near 120 on JAZZY without requiring a conversion error.

OBSERVATION: the owner has confirmed JAZZY at 1440 pt = 508 mm = 120 picas, with the app ruler using picas. There is no unit bug in this observation. UNKNOWN: these arithmetic fits do not prove the root fields are the native physical page rather than another size/canvas candidate. Treat them as intentionally labelled raw candidates with an unverified physical interpretation. The three large-square files are JAZZY, MODERN and PLAIN.

INFERENCE: locally right-click each ruler and record the checked unit. Then select Millimeters, inspect File > Document Setup width/height with their units, record the values and cancel. A 120-pica width should correspond to 508 mm if the displayed document uses this candidate. Owner-recorded page settings in the native application, or an independent save with a known size and a new locally recorded raw pair, would test whether the selected fields and scale track physical size. A ruler change alone cannot settle field semantics. Return only numeric settings/hashes and redacted screenshots, never the private files or their text.

INFERENCE: proposed adapter policy only: never call any raw conversion verified. Always retain the raw pair and uncertainty status. A conservative default workspace could use A4 if either inferred dimension lies outside 50..450 mm, while preserving the rejected candidate and reporting “Original page size is unverified. A standard A4 recovery page is being used.” Permit an explicit owner choice to use a larger candidate as an unverified workspace. This would default the 508/558.8-mm squares to A4 but would also reject legitimate large posters and tiny cards; the range is product policy, not a decoded format law. It is not implemented by this task and does not authorize silently replacing file metadata.

## The two A files with no recovered text

OBSERVATION: prior notes for the pinned mspub/zmspub identities record 4152-byte root Contents, the supported A profile, a 1024-byte `/Quill/QuillSub/CONTENTS` stream with SYID count zero, absent TEXT/STRS/TCD sections and two checked zero story probes. Their Quill streams were identical; their complete files were not. That evidence fits EmptyKnownStore under the narrow contract. The owner reports the corresponding empty-store notice, one generated page and zero objects, consistent with this state. The mspub full hash has now been confirmed against the pinned identity.

UNKNOWN: EmptyKnownStore is not proof the publication contains no text anywhere. An unsupported alternate store or object text could still matter in an unfamiliar file. A no-text result by itself cannot distinguish those possibilities, and the existing empty notice deliberately makes no publication-wide claim.

INFERENCE: the owner can decide the next step locally without sending either binary: compare full file SHA-256/size; obtain a container-only inventory of stream names, lengths and stream hashes using an already permitted local CFB diagnostic; compare root Contents and Quill lengths with the prior facts; and inspect the CLI's family/scope/count fields. If a local diagnostic is available, report SYID count, presence/absence of TEXT/STRS, and counts of valid zero/positive candidate links, not their text. Identical Quill hashes confirm identical stores only. A zero SYID plus absent TEXT/STRS and fully validated zero links supports EmptyKnownStore; a positive count, unexpected text section or positive link contradicts that profile and needs a reported refusal/research step, not a guessed fallback.

INFERENCE: if the owner can open the original locally in its authoring application, report only whether selectable text is visible and the numeric page settings. Visible selectable text together with the validated empty supported store would be evidence of a missed store. A blank template appearance is supporting context, not proof of absence. If no local stream diagnostic/native application is available, that distinction remains UNKNOWN; do not install a third-party Publisher reader for this check.

## New byte-built tests and execution status

OBSERVATION: `crates/engine/tests/legacy_followup.rs` builds only invented test data. The existing permitted synthetic A/C builders are used; the control fixture replaces Quill with a byte-built three-section store, IDs 5/9 and invented short text. No Publisher-made input or private file is used. These checks extend the existing b1e4dbec mixed/all-unknown tests with additional count/provenance and nonempty-notice assertions.

| Basis | Earlier follow-up test | Required result on b1e4dbec | Owner Windows execution, round 2 |
| --- | --- | --- | --- |
| OBSERVATION | all_unknown_c_retains_every_placeholder_and_is_not_empty | Partial, two placeholders, 2=0+2, unknown two, no refusal/empty notice | PASS in owner ordinary run against b1e4dbec |
| OBSERVATION | mixed_c_retains_one_decoded_frame_and_one_placeholder | Partial, placeholder plus Beta, 2=1+1, unknown one | PASS in owner ordinary run against b1e4dbec |
| OBSERVATION | controls_are_kept_by_reader_but_only_tab_is_shown_by_adapter | Reader retains original sequence; displayed tab survives, FF/other control/trailing CRs do not; warning present | PASS in owner ordinary run; current display behavior, not lossless retention |
| OBSERVATION | original_control_sequence_is_retained_after_open | Expected failure: no full original sequence in opened document/report | Owner explicitly ran it: failed intended assertion at legacy_followup.rs:124; confirmed gap |
| OBSERVATION | removed_trailing_paragraphs_alone_are_reported | Expected failure: current counter misses trailing-only removal | Owner explicitly ran it: failed intended assertion at legacy_followup.rs:132; confirmed gap |
| OBSERVATION | cli_run_and_script_keep_mixed_c_report_fields | run --cmd and script - expose all declared legacy fields and count invariant | PASS in owner explicit run with LEGACY_FOLLOWUP_CLI set to release CLI |
| OBSERVATION | batch_report_has_parity_with_mixed_c_command_result | Expected failure: existing CSV columns present but typed fields missing | Owner explicitly ran it: failed intended assertion at legacy_followup.rs:203; confirmed gap |
| OBSERVATION | folder_report_has_parity_with_mixed_c_command_result | Expected failure: per-file JSON warnings exist but legacy object missing | Owner explicitly ran it: failed intended assertion at legacy_followup.rs:222; confirmed gap |

OBSERVATION: the earlier local environment lacked Cargo, but that no longer leaves these eight tests unexecuted: the owner ran them on Windows against b1e4dbec with this test file applied. The ordinary target had 3 passed and 5 ignored; the CLI check subsequently passed explicitly, and each of the four expected-failure cases failed on its intended assertion, not setup/build. These are separate runs, not a single combined green suite. The reported full engine/CLI/importer run has one unrelated Windows batch-test path assertion failure; see round 2. Formatting with rustfmt was not reported by the owner, and remains unverified.

INFERENCE: after applying this test-only file to b1e4dbec in a code checkout, run the ordinary engine/CLI tests. Build the CLI there, set LEGACY_FOLLOWUP_CLI to that executable's absolute path, and run this integration target's CLI test with `--ignored --exact` and its listed test name. Run each expected-failure test separately with `--ignored --exact`; it should assert the missing requirement and fail, not accept missing data as success. A failure before its intended assertion (build, export, missing executable) is a blocked/invalid expected-failure run and must be reported separately. Once a gap is fixed, remove its ignore reason in the code lane and require a pass. Do not count ignored tests as passing parity.

## Revised Windows procedure

INFERENCE: this supersedes the earlier assumed Sample98/Sample2000 workflow and its no-rebuild restriction. Build on the PC from the intended revision; shipped alpha.5 is insufficient even if its version string matches. Use any owner-chosen 97/98/2000 files already on that PC, in a dedicated input folder containing only test copies. No Publisher 2000 sample is available on the PC yet; mark B/native-2000 testing not performed until one is independently supplied locally. Do not request or upload private files.

INFERENCE: in the repository's PowerShell window, use the following local procedure. The PC now has Git and the Rust/MSVC toolchain. The repository is `E:\Projects\NiftyDesign-src`, the built app is `target\release\designcraft.exe`, and the seven local inputs are in `E:\Projects\quarantine\legacy-intake\in`. Choose a new empty output location when prompted. Batch conversion writes PDF/native copies and CSV into its output directory; it does not replace the input files. Each script invocation captures the separate file.open result, rather than relying on warnings in CSV. The command-line files stay private on the PC.

```powershell
# INFERENCE: run locally in the owner's implementation checkout.
Set-Location 'E:\Projects\NiftyDesign-src'
git rev-parse HEAD
cargo build --release -p designcraft -p designcraft-cli
if ($LASTEXITCODE -ne 0) { throw 'Build failed; stop and record the error.' }
$legacyCli = (Resolve-Path '.\target\release\designcraft-cli.exe').Path
$legacyApp = (Resolve-Path '.\target\release\designcraft.exe').Path
& $legacyCli --version
$legacyInput = (Resolve-Path 'E:\Projects\quarantine\legacy-intake\in').Path
$legacyOut = Read-Host 'New local output folder for this run'
if (Test-Path -LiteralPath $legacyOut) { throw 'Choose a new output folder.' }
New-Item -ItemType Directory -Path $legacyOut | Out-Null
$legacyFiles = @(Get-ChildItem -LiteralPath $legacyInput -File | Where-Object { $_.Extension -ieq '.pub' })
if ($legacyFiles.Count -eq 0) { throw 'No .pub files were selected.' }
$legacyIdentity = foreach ($legacyFile in $legacyFiles) {
  [pscustomobject]@{Name=$legacyFile.Name; Bytes=$legacyFile.Length;
    SHA256=(Get-FileHash -LiteralPath $legacyFile.FullName -Algorithm SHA256).Hash}
}
$legacyIdentity | Export-Csv -NoTypeInformation -Path (Join-Path $legacyOut 'identity-before.csv')
& $legacyCli batch-convert $legacyInput --out (Join-Path $legacyOut 'batch') `
  1> (Join-Path $legacyOut 'batch-summary.json') 2> (Join-Path $legacyOut 'batch-errors.txt')
Write-Output ('Batch exit=' + $LASTEXITCODE)
# INFERENCE: UTF-8 applies only to this PowerShell process's native stdin.
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

INFERENCE: retain `batch/report.csv` as one report source and each `*.command.json` as the other. In the latter inspect `results[0].legacy`, plus top-level warnings/notice. Compare family, scope, segmentCount, decodedCount, undecodedCount, encodingUnknownCount, rawSizePair, sizeStatus, recoveryPageSize (points and mm), withheldCount, provenance and storageOrderOnly. Require segmentCount=decodedCount+undecodedCount. Record missing fields as missing, not inferred from warnings. readerRevision currently reports a package/version, so attach `git rev-parse HEAD` independently. For the five known C files, expected segment totals remain 28/72/74/88/30, decoded totals 28/64/65/78/29, unknown totals 0/8/9/10/1, conditional on the full hashes matching the earlier identities. Generated page/object totals are not native-layout or fidelity checks.

INFERENCE: launch the newly built app at the resolved legacyApp path, then File > Open each chosen file without saving it. Record the exact window title and build identifier. Capture the notice with Import notes expanded, cropping/redacting page text and personal paths. Record the zoom and each ruler's checked unit from its right-click menu. Select Millimeters for a local ruler check; open File > Document Setup, record page width/height and displayed units, and cancel. Include a redacted view of the first page outline and the size fields if possible. Do not infer millimetres from a ruler labelled in picas. Report whether the placeholder and plain editing frames appear, not the private story text.

INFERENCE: a successful partial file should have its partial banner and one placeholder per validated unknown segment; an all-unknown file stays Partial with placeholders. A supported empty store should explicitly say it found no text in that store. A refusal should retain the old error plus the experimental reason and preserve the prior document, not replace it with a blank page. Failure includes a crash, old shipped refusal from the wrong build, unexpected blank workspace, missing notice, guessed text, changed input hash, count invariant failure, missing expected report fields, app/report size disagreement after matching units, or hidden/removed text without adequate warning. CATALOG's measured 61-character overflow is an unresolved fit issue even though batch status is converted-with-warnings.

INFERENCE: after the run recompute full SHA-256/size locally and compare with identity-before.csv. Send back only build commit/version, basename/size/full hash, exit codes and sanitized errors, the structural report fields, the window title, page size/units/zoom, and redacted notice/expanded-notes screenshots. No binaries, paragraphs or byte dumps are needed. Do not claim a Publisher 2000 result for this PC until such a file is actually present and checked.

## Ranked next steps

1. INFERENCE: the unit-level check is complete: JAZZY's 120 picas equals 1440 points and 508 mm. The next smallest code step is preserving the typed legacy result in the batch worker outcome; round 2 gives the ordered plan and existing red tests.
2. INFERENCE: keep exact source controls/trailing paragraphs in recovery provenance or an explicit lossless export, fix the trailing-only warning, and replace the approximate fit decision with actual composition evidence. Re-run byte-built control tests and the local CATALOG fit check; do not call a control-removal warning preservation.
3. INFERENCE: preserve the typed legacy result through batch and folder reports, and replace the expected-failure parity tests with required passing tests. Keep a real build commit distinct from the package-version readerRevision.
4. INFERENCE: mspub identity is now settled; retain the narrow EmptyKnownStore interpretation. Keep missed alternate text UNKNOWN unless native visibility/stream metadata contradict the validated store.
5. INFERENCE: when independent known-size/known-text A/B samples are available locally, including a real 2000 sample for this PC, test field semantics/scale and fragile populated-link behavior before widening recovery claims. No private file transfer is requested. C code-page identity remains UNKNOWN; no guessed decoder is proposed here.

OBSERVATION: the owner has now compiled/executed the tests; the four red expected-failure cases confirm the control/warning/report gaps. The mspub identity and pica conversion are settled. UNKNOWN: native intended physical page sizes, C code-page identity, wider A/B replication and the proposed fixes remain unverified. Successful warned conversion is not original-layout fidelity.
