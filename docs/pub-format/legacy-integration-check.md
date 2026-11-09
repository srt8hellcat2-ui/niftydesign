# Legacy open integration check

OBSERVATION: this review compares [the open-notice proposal](legacy-open-notice.md) and [contract v2](legacy-minimal-reader.md) with the owner's integration at `f2d8ef9f`, including the subsequent notes change at `9b4cf49f`. Source conclusions below are pinned to `9b4cf49fb111c6e7ceb8eaf988a49f0954839ed8`. Inspected sources include `crates/engine/src/cmd/legacy_open.rs`, its `interchange.rs` caller, `crates/engine/tests/legacy_open.rs`, the reader's routing probe, the CLI command/report paths and the UI notice renderer. No external reader, third-party format description or prohibited repository was consulted.

OBSERVATION: this is a source inspection, not an executed integration, GUI or corpus run. The configured corpus folders are unavailable. Existing notes supply the manual expectations; no new sample facts, binaries or private text are committed. No code under crates or apps is changed.

INFERENCE: MATCH means the specified behavior is present in the inspected path; STRICTER means additional rejection; LOOSER means a missing guarantee or weaker behavior. A MATCH does not establish runtime correctness. UI presentation and report serialization are considered separately from the engine's return value.

## Routing comparison

| Basis | Proposal rule | Result | Exact behavior or difference |
| --- | --- | --- | --- |
| OBSERVATION | Routing 1: current importer first, success wins | MATCH | `interchange::open_pub` calls `import_pub_with` first; a successful import bypasses legacy recovery. The current-family integration test checks absence of `legacy` and the existing conversion notice. |
| OBSERVATION | Routing 2: explicit older-family refusal, not generic Unsupported | LOOSER | The guard accepts any `PubError::Unsupported(why)` when `looks_legacy(bytes)` is true. It does not distinguish the older-family unsupported cause from another Unsupported cause. The prefix narrows the path, but does not implement the proposed stable refusal discriminator. No substring heuristic is used. |
| OBSERVATION / UNKNOWN | Routing 3: safe container and exact old prefix | MATCH | `looks_legacy` opens bounded V3 root Contents and checks all four old discriminator bytes; failures stay on the existing error path. The proposal's unique-name guarantee still depends on container resolution, as noted in the earlier G01 review; this source inspection does not independently establish that dependency guarantee. |
| OBSERVATION | Routing 4: separate gated reader, same bytes | MATCH | `open_legacy` calls `read_legacy(bytes)` with the original bytes. It does not choose a family by filename or change a gate. Earlier reader-contract assurance gaps are not closed by this adapter. |
| OBSERVATION | Routing 5: no blank recovery on refused/empty Partial result | MATCH | A reader error returns an error. Zero retained segments refuses unless scope is EmptyKnownStore; TextRefused normally has zero segments and therefore refuses. A surviving size alone cannot create a document. |
| OBSERVATION | Routing 6: explicit empty workspace; failed open preserves existing document | MATCH | EmptyKnownStore creates one page and no frames with its distinct notice. `s.add_document` occurs only after successful document construction; refusal does not replace the session document. Existing tests cover no-document refusal, not preservation of a pre-existing edited document. |
| OBSERVATION | Available experimental path versus disabled/unavailable path | MATCH | The compiled-in reader is attempted only when the guard matches; other failures retain the current path. No runtime disable control or optional backend exists here. The proposal does not require adding a switch; disabled/unavailable operation is not represented by this build and remains untested. |
| OBSERVATION | Failed attempted recovery preserves original refusal and appends reason | LOOSER | Common errors contain the same original wrapper and Unsupported reason plus the proposed experimental line. Reader errors use their string; zero-result failure selects a warning containing `refused` or falls back to “no text was recovered”, potentially dropping a more specific non-refusal warning. Document-construction errors use `bad(file.open,...)` instead of the original refusal wrapper. No sanitization boundary for dependency error strings is demonstrated. The shared keep-original detail is not appended to refusal errors. |

INFERENCE: the session is not mutated before successful construction, but an app-level error dialog still needs native verification. A safe old-prefix probe is not evidence that the whole file passed legacy admission. A modern-prefix parse failure remains on the current importer path, without legacy repair.

## Notice comparison

| Basis | Proposal copy or behavior | Result | Exact difference |
| --- | --- | --- | --- |
| OBSERVATION | Successful banner plus shared detail | MATCH | `NOTICE_OK` contains the proposed successful copy and the shared missing-layout/original-file advice. It is retained in `DocState.import_notice`. |
| OBSERVATION | Partial banner | MATCH | `NOTICE_PARTIAL` contains the proposed text. Any non-Complete/non-empty admitted scope with retained segments selects it. |
| OBSERVATION | Partial details/counts | LOOSER | Returned JSON has decoded/unknown counts, but Import notes do not display those counts together or count entirely withheld stories. The generic notice says missing text is reported below even when no distinct omission count exists. |
| OBSERVATION | Refusal copy | LOOSER | The reader/text-path failure case uses the requested appended sentence. Specific-reason preservation, document-construction error wrapping and sanitization have the routing gaps above. |
| OBSERVATION | Empty known-store copy | MATCH | `NOTICE_EMPTY` explicitly distinguishes a supported empty store from a blank original file. |
| OBSERVATION | Undecoded placeholder | MATCH | Every returned segment with `text=None` gets exactly “[Text could not be read. No replacement text was guessed.]”. Its provenance is in the response, not decoded from its bytes. |
| OBSERVATION | Estimated-size and fallback-size warnings | MATCH | The two proposed sentences are added to warnings and retained in import notes, according to `size_status`. |
| OBSERVATION | Long-text warning | LOOSER | The text matches, but its trigger is the same approximate character/line estimator used for placement, not measured composition. Real overflow can be missed or falsely reported. |
| OBSERVATION | Persistent, accessible notice/details | MATCH | UI `app/frame.rs` displays `import_notice` above the document and warnings in collapsible Import notes in ordinary editing mode. Presentation mode suppresses this chrome; native accessibility, clipping and visibility have not been tested here. |

## Document-shape comparison

| Basis | Proposal rule | Result | Exact behavior or difference |
| --- | --- | --- | --- |
| OBSERVATION | Unsaved recovery; original never overwritten | MATCH | `DocState::new(d,None)` leaves the document unsaved; file-path opens suggest a `.designcraft` sibling save path. Base64 opens have no suggested source path. The integration test asserts unsaved state only for base64. |
| OBSERVATION | One frame per retained segment in storage order | MATCH | The adapter iterates `r.segments` once and creates one frame per item. Duplicate A/B provenance does not create another frame. Frames use default paragraph formatting, not original styling. |
| OBSERVATION | Undecoded-only result | MATCH | With retained segments, no decoded-text minimum is imposed. All-unknown C input therefore opens Partial, with one placeholder per segment, not EmptyKnownStore or refusal. This path has no engine integration test. |
| OBSERVATION | Plain pages and sequential placement | MATCH | Frames are stacked with 36-point margins and 8-point gaps; further pages are generated as needed. Pages are non-facing and have no automatic primary frame. This is generated layout, not recovered page order. |
| OBSERVATION | Raw candidate conversion and positive usable page | MATCH | Only the reader's checked candidate is used. The adapter converts inferred mm to points, equivalent to raw/12700, and admits each dimension from 72 through 15552 points. Otherwise it uses approximately A4 portrait, 595.276 by 841.89 points, with a warning. The actual numerical limits are now explicit; this review does not establish that they are shared application limits. |
| OBSERVATION | Frames respect generated page margins | LOOSER | Body width/height are each forced to at least 36 points. A permitted 72-point dimension leaves zero space between the two 36-point margins, yet a 36-point frame body is used. Its right/bottom edge can reach the page edge, eliminating those margins. This calculation alone does not prove a frame extends outside the page. |
| OBSERVATION | Full long text retained; no silent truncation | LOOSER | The visible ordinary text is stored even when estimated height is capped, but `visible()` drops controls below U+0020 except tab/CR/LF, converts paragraph ends and trims all trailing LF. The original segment/control sequence is not retained in the document metadata. This loses data the proposal required preserving; provenance alone cannot reconstruct it. Overflow detection is approximate. |
| OBSERVATION | No fabricated marker-based pagination | MATCH | CR/LF become editable paragraphs; FF does not cause a page break. Removing FF is nevertheless the preservation gap above. |
| OBSERVATION | Empty workspace versus refusal | MATCH | EmptyKnownStore yields one page/no frames. Errors and no-recoverable-content outcomes never reach session insertion. |
| OBSERVATION | Bounded generated layout | STRICTER | The adapter refuses when generated page index reaches 2000, adding a workspace limit absent from the notice proposal. It does not silently drop the remaining segments. |

UNKNOWN: the exact rendered font metrics, editable visibility of overflowing text, destination save dialog and notices on a native Windows build are not established by the approximate placement code or source-level tests.

## Report-field comparison

OBSERVATION: `open_legacy` returns a nested `legacy` object and top-level `warnings`, `notice` and empty `notes`. A CLI `run --cmd file.open=...` prints the command result, whereas `run --in ...` discards the open result. Batch `Outcome::add_warnings/value` retains warnings/notice/notes but drops the entire legacy object; folder conversion builds on that reduced outcome. A legacy-family name in a warning is not a substitute for the requested typed fields.

| Basis | Required field | Engine command result | Batch/folder report | Exact difference |
| --- | --- | --- | --- | --- |
| OBSERVATION | reader_revision | LOOSER | LOOSER | Missing in both; record the tested build externally. |
| OBSERVATION | route | MATCH | LOOSER | `legacy.route` is experimental-legacy; batch/folder discard it. |
| OBSERVATION | family | MATCH | LOOSER | Exact family label exists on success; a refused result has only an error string, not typed unknown-family data. Batch/folder omit the field. |
| OBSERVATION | scope | MATCH | LOOSER | `legacy.scope` is the debug-form scope on success; no typed refusal report exists. |
| OBSERVATION | open_result | MATCH | LOOSER | `legacy.openResult` distinguishes recovery-document and empty-recovery-workspace; refusal returns an error rather than a structured refused object. |
| OBSERVATION | segment_count | MATCH | LOOSER | `segmentCount=r.segments.len()`; unavailable on refusal. |
| OBSERVATION | decoded_count | MATCH | LOOSER | `decodedCount` counts `text.is_some()`. |
| OBSERVATION | encoding_unknown_count | MATCH | LOOSER | `encodingUnknownCount` counts explicit flags, independently from text absence. |
| OBSERVATION | undecoded_count | MATCH | LOOSER | `undecodedCount=segmentCount-decodedCount` guarantees the requested sum invariant on every successful response. |
| OBSERVATION | withheld_count | LOOSER | LOOSER | No value, including no explicit unknown; failed links/unlinked IDs/duplicate counts are not separate typed counters. |
| OBSERVATION | raw_size_pair | MATCH | LOOSER | `rawSizePair` retains the reader's pair or null; no default is substituted into it. |
| OBSERVATION | size_status and recovery_page_size | LOOSER | LOOSER | `sizeStatus` is present, but `recoveryPageSize` reports points, while the proposal requests explicit mm. Values are convertible; this is a schema-unit difference. No unavailable/refused size object is emitted. |
| OBSERVATION | warnings and refusal diagnostics | LOOSER | LOOSER | Warnings survive in both paths, but remain strings without every v2 diagnostic field. Refusal wrapping/reason gaps are listed above. The notes-once fix returns empty notes so batch does not duplicate the warnings there. |
| OBSERVATION | provenance and storage-order label | MATCH | LOOSER | Per-segment story/range/records or target/map/range plus decoded status are returned, and `storageOrderOnly=true`; batch/folder omit them. |

INFERENCE: MATCH report cells refer only to admitted engine results. The proposal's typed `Refused`/`TextRefused` report and unknown-valued unavailable counters are LOOSER for every relevant field on a failed open. In particular a JSON command failure must not be described as a validated zero-story publication. Per-segment encoding-unknown flags would also make auditability stronger than the current aggregate plus decoded flag.

## User-visible test gaps and byte-built cases

OBSERVATION: the inspected engine suite has four tests: populated A success, ASCII C success, missing-Quill refusal (plus a truncated-container assertion), and current-family bypass. The CLI corpus tests assert warning/conversion totals, not the legacy report schema. Reader unit tests exercise additional parsing cases, but do not establish their app notices, generated frames or native UI presentation. No assertion covering the following integration behaviors was found in those tests or the targeted legacy-notice/report test search. This is bounded source coverage, not proof that no unrelated test anywhere could exercise a shared component.

INFERENCE: each proposed case below constructs its own compound file and relationship-consistent streams from the contract, using invented short text only. Recompute dependent offsets, stream lengths and extents after mutation. These are future test specifications, not committed fixture code, Publisher-made files or executed results.

| Basis | Untested visible behavior | One proposed byte-built integration test and required assertion |
| --- | --- | --- |
| INFERENCE | B opens through the app route | Build the contract's B-positive profile with an allowed nonzero terminator, two invented stories and exact consumed profiles. Open through file.openBytes; assert family B, experimental notice, two frames, counts and raw pair. |
| INFERENCE | A different Unsupported cause cannot take the older-family route | Build a valid old-prefix candidate and use a controlled current-importer result reporting a distinct Unsupported cause. Assert the routing helper keeps that failure. If no byte pattern reaches that cause today, test the typed outcome seam; do not claim that an invented file demonstrates an existing second cause. |
| INFERENCE | Malformed current-family data never falls back | Build a compound file with the current discriminator and truncated required current structures. Assert failure, no legacy notice/result and no document replacement. The existing current-family test covers success only. |
| INFERENCE | Empty known store notice/workspace | Build A-empty with absent TEXT/STRS and checked zero probes. Assert EmptyKnownStore, one page, zero frames, the full empty notice, zero counts and unsaved state. |
| INFERENCE | Mixed ASCII/unknown text | Build C with two valid mapped segments, replace one content byte with a high byte while preserving its markers. Assert Partial, one exact placeholder, one decoded frame, 2=1+1 counts and unknown count one. |
| INFERENCE | Every segment unknown | Build the same C model with a high byte in both ranges. Assert Partial, two placeholders, 2=0+2, unknown count two, no refusal and no empty-store notice. |
| INFERENCE | Partial A recovery and omitted-segment details | Build two A stories with one unpaired surrogate but valid CR-ended lengths/links. Assert one editable frame, Partial copy and a specific withholding reason; no placeholder with a fabricated range for the missing story. |
| INFERENCE | A4 fallback, limits and positive candidates | Build A with root width zero but valid text, then parameterize a positive out-of-range candidate. Assert raw pair absent versus retained respectively, A4 generated page and fallback warning; never report A4 as file metadata. |
| INFERENCE | Accepted smallest page and margins | Build a valid root pair corresponding to 72 by 72 points with one short story. Assert the specified margins as well as page containment, or an explicit size fallback. The current minimum-body rule predicts a margin failure, not necessarily a containment failure. |
| INFERENCE | Long-story visibility/overflow warning | Build one large CR-ended story of invented wide glyphs and multiple paragraphs. Use actual composition to assert all text remains editable, overflow produces a warning and no characters are silently removed. Do not use the same estimator as the oracle. |
| INFERENCE | Controls and trailing paragraphs retained | Build an A story containing invented letters, tab, FF, another supported control and two trailing paragraph marks. Open it and compare retained source text/metadata exactly, while separately checking displayed paragraphs. The current `visible()` transform predicts a preservation failure. |
| INFERENCE | Storage order, duplicate provenance and page spill | Build A links in a deliberately different order from SYID, with duplicate links, and enough short stories to create a second page. Assert one frame per returned ID, storage sequence preserved, duplicate record provenance retained and no reading-order claim. |
| INFERENCE | Page-budget refusal | Build enough valid long segments to require 2001 recovery pages. Assert refusal with the original message and page-limit reason, and no partially inserted document. This is an integration resource-limit fixture, not evidence about native page count. |
| INFERENCE | Refusal preserves existing work | Start with an edited unsaved synthetic document, then open a byte-built A file missing Quill. Assert identical active document, dirty state and document count, plus both refusal messages. |
| INFERENCE | Construction-error refusal and sanitization | Open a valid byte-built A file while a controlled frame-construction seam returns a structural error containing a sentinel private string. Assert original refusal plus sanitized reason, no private sentinel and no session insertion. A fault seam is needed because no native input is known to trigger this failure. |
| INFERENCE | Filesystem save suggestion and source integrity | Write the byte-built A input to a temporary path, open via file.open, inspect unsaved state and the suggested .designcraft path, then save a new native copy. Assert the original bytes/hash are unchanged and no automatic source overwrite occurred. |
| INFERENCE | UI notice and Import notes really visible | Load the mixed C byte-built file in the native UI harness; check full partial notice, expandable notes, placeholder and size warning at normal and narrow window widths. Engine JSON assertions alone cannot establish this presentation. |
| INFERENCE | CLI/batch/folder report parity | Put one byte-built mixed C file in a temporary directory and open/convert with each CLI path. Assert the full common schema and count invariant in every report, with warnings once. Current batch/folder reports are expected to fail these schema assertions. |

UNKNOWN: Windows accessibility, rendering and dialog behavior cannot be settled by adding reader unit cases alone. The native owner check below remains necessary even after byte-built command tests exist.

## Owner Windows procedure

INFERENCE: use an app and CLI built from the same reviewed revision, or record their exact different revisions. Do not install or rebuild anything merely to make a failed check pass. This procedure opens data and writes local diagnostic reports only; it does not save converted publications or overwrite originals. Keep report files and screenshots local until their private content has been checked.

1. INFERENCE: create `C:\NiftyLegacyCheck` and put the supplied test copies in its `inputs` subfolder. For a reproducible first run use the previously documented Sample98.pub and Sample2000.pub if available. They test observed A/B profiles; filenames do not establish exact native releases. Otherwise choose one owner-confirmed 97/98 save and one owner-confirmed 2000 save, record that fact, and do not assume their story counts match the Sample files.
2. INFERENCE: record the app build identifier from About and CLI `--version`, plus the supplied build's commit hash. Locate `designcraft-cli.exe`; the commands below assume it is in `C:\NiftyLegacyCheck\bin`. If that executable or a compatible build is unavailable, report that step as blocked with its error rather than invent a replacement command.
3. INFERENCE: in PowerShell run the commands below, adjusting only the CLI path and input names. They compute identity, pass a file.open command through stdin and capture its returned JSON. Use `script -` rather than `run --in`, which does not print the open response. Each loop iteration starts a separate CLI process.

```powershell
# INFERENCE: owner-local diagnostic procedure; no publication is saved.
$legacyCli = 'C:\NiftyLegacyCheck\bin\designcraft-cli.exe'
$legacyInputs = @(
  'C:\NiftyLegacyCheck\inputs\Sample98.pub',
  'C:\NiftyLegacyCheck\inputs\Sample2000.pub'
)
& $legacyCli --version
foreach ($legacyPath in $legacyInputs) {
  $legacyFile = Get-Item -LiteralPath $legacyPath -ErrorAction Stop
  $legacyHash = Get-FileHash -LiteralPath $legacyPath -Algorithm SHA256
  [pscustomobject]@{Name=$legacyFile.Name; Bytes=$legacyFile.Length; SHA256=$legacyHash.Hash}
  $legacyCommand = @{command='file.open'; params=@{path=$legacyFile.FullName}} | ConvertTo-Json -Compress
  $legacyStem = Join-Path 'C:\NiftyLegacyCheck' $legacyFile.Name
  $legacyOutput = $legacyCommand | & $legacyCli script - 2> ($legacyStem + '.stderr.txt')
  $legacyExitCode = $LASTEXITCODE
  $legacyOutput | Set-Content -LiteralPath ($legacyStem + '.report.json') -Encoding UTF8
  Write-Output ($legacyFile.Name + ': exit=' + $legacyExitCode)
}
```

OBSERVATION: recorded identities are Sample98.pub, 61440 bytes, SHA-256 `8912f295ff0cd1f55b873b3b98a182218f33c420877e025d2f0fd795f12dd0c0`; Sample2000.pub, 61440 bytes, SHA-256 `40701ca47b26d04771cdd58467764e9ab39d3da69b59fbc529afd16d263d2f86`. Do not compare a different hash against these specific expected results.

4. INFERENCE: inspect the CLI script report's returned command result (inside its results collection). For these exact two samples, expect route experimental-legacy, family A then B, scope Complete, openResult recovery-document, segmentCount six, decodedCount six, undecodedCount zero and encodingUnknownCount zero. RawSizePair must be [7560000,10692000], sizeStatus inferred-from-raw, and recoveryPageSize approximately 595.276 by 841.89 **points**. Check 6=6+0, storageOrderOnly true, provenance and warnings. Reader revision and withheld count are currently missing; record “not provided”, never substitute zero. Any failure/nonzero exit, absent legacy result, incorrect pair, count or family is a mismatch, not a pass.
5. INFERENCE: launch the corresponding desktop app normally. If existing work is open, leave it unsaved only if safe to do so; note which document is active. Choose File > Open (or Ctrl+O), select Sample98.pub, then repeat for Sample2000.pub. Do not use Save or Save As. A successful open should show the banner beginning “Experimental text recovery. Recovered text is ready to edit” and explicitly say this is not the original layout. Expand Import notes and verify the physical-size uncertainty and original-file/resave advice.
6. INFERENCE: on the first page expect plain editable text frames with default styling, approximately A4 portrait, starting near half-inch top/left margins. Expect generated layout, not the original pictures/tables/fonts or original frame positions. The six sample stories total 488 UTF-16 units before adapter paragraph presentation; do not require all six frames to fit the first page. Select the first frame and verify it can be edited without saving. Note any overset indicator, clipping or inaccessible text. The banner/notes must not imply verified page order or fidelity.
7. INFERENCE: for a genuine partial result in other owner files, expect the partial banner and a placeholder for each returned undecoded segment; all-unknown retained segments must produce placeholders and Partial, never a silent empty page. For a true supported empty store expect its explicit empty-workspace notice. For refusal expect the existing refusal followed by the experimental reason, with the prior document still present and no replacement blank document. A crash, source overwrite, missing banner, guessed text, silent character loss, misleading success or report/app disagreement is a failure.
8. INFERENCE: send back build IDs, basename, size, SHA-256, CLI exit/error, the full structural legacy fields and warnings listed above, and a screenshot of the notice with Import notes expanded. Crop or redact document content, personal paths and titles; no original binary, paragraph or private byte dump is requested. If possible capture the first-page outline separately with text redacted. Recompute the source hashes after the test to confirm no original changed. Explain each missing field or mismatch rather than filling it from this expected table.

UNKNOWN: these steps were not executed on Windows here. Whether the owner's installed binaries contain the reviewed commits must be established from their build provenance, not a matching version number alone.

## Three highest-risk assumptions

| Basis / rank | Assumption and risk | Evidence that would settle or bound it |
| --- | --- | --- |
| INFERENCE / 1 | Estimated frame height adequately exposes complete recovered text. Character-count wrapping ignores actual fonts/glyph widths; controls and trailing paragraph marks are already removed by the adapter. Visible success can therefore conceal missing or hidden data. | Byte-built control-preservation tests plus composition-based overflow assertions, followed by a native Windows edit/scroll check with long wide-glyph stories. Preserve source text separately and compare exact units; a rendered first page alone cannot establish completeness. |
| INFERENCE / 2 | Raw/36000 mm corresponds to an original physical page size. Numerical standard sizes and repeated pairs support a candidate, not verified units. A4 fallback is a workspace policy and can look like recovered metadata. | Independent A/B/C saves with owner-recorded native width, height and orientation, including nonstandard unequal sizes, compared against raw pairs and Windows displayed dimensions. Test fallback separately with invalid/absent candidates and verify its warning; no sample can turn a synthetic default into original metadata. |
| INFERENCE / 3 | Storage order is useful enough as an editing sequence without being mistaken for reading order. Plain generated pages can imply ordering the records do not prove. | Independently authored files with owner-supplied story identity and intended reading sequence, deliberately different creation/storage orders, compared to preserved IDs/ranges. Even consistent examples would bound coverage rather than establish universal reading order; keep the warning and storageOrderOnly label. |

UNKNOWN: this checkpoint identifies implementation differences and missing evidence; it does not certify File > Open, CLI conversion fidelity, original page order, physical units or recovery of private text. A/B populated coverage and B framing remain FRAGILE as stated in the contract. The next smallest step is the two-file Windows/CLI check above, followed by the all-unknown C byte-built integration test and batch-schema parity check.
