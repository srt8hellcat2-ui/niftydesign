# Legacy recovery: roadmap and release text

OBSERVATION: this note records the owner's Windows results for the local implementation based on b1e4dbec. It supersedes the open implementation items in [round 3](legacy-followup-3.md) where the owner reports them fixed below. No new implementation head, detailed test counts or local execution are supplied for this state. Evidence is the owner's report and our existing notes; no private files or external format descriptions were obtained. This task changes documentation only and runs no tests.

## Maintainer summary: changed behavior and limits

OBSERVATION: nine existing files are available on the PC: five family C templates, three family A files (Sample98, mspub, zmspub), and one family B file (Sample2000). Sample98 and Sample2000 each open Complete with six segments, all six decoded, and an inferred A4 candidate from raw [7560000,10692000]. The two other A files open as EmptyKnownStore. Four C files retain undecoded segments because their non-ASCII encoding is not established. This is evidence for these identities, not every file saved by those releases.

OBSERVATION: stored candidates exceeding 450 mm on either side now produce an A4 recovery workspace with the notice “Original page size is unverified. A standard A4 recovery page is being used.” This applies to JAZZY/MODERN's 508-mm squares and PLAIN's 558.8-mm square. The stored pair remains separate metadata; choosing A4 does not establish an original physical size. The 1440-point/120-pica/508-mm conversion is settled, not a unit bug.

OBSERVATION: original control/trailing-paragraph text is now retained as sourceText in the command result, bounded to 4096 characters per segment and 1000 segments. It is excluded from batch/folder reports. The warning now counts segments with two or more trailing paragraph marks. This closes the earlier command-result/warning checks, but does not demonstrate lossless source-text persistence in a saved native document or complete retention beyond those limits.

OBSERVATION: batch report.csv adds exactly one last column, legacy. Its header is now `file,pages,objects,status,warnings,pdf_path,failure_reason,notes,notice,legacy`. The new cell contains JSON without sourceText and with provenance capped at 500 entries. Folder report.json carries a per-file legacy object. readerRevision identifies the package/version; the adjacent buildCommit is unknown unless the release build sets NIFTY_BUILD_COMMIT. Aggregate segment counts must not be inferred from a capped provenance array.

INFERENCE: one JSON column keeps the existing nine CSV columns in their original positions and carries nested provenance/page-size values in the same shape as the command and folder result. It avoids fifteen flattened columns and repeated schema decisions. Consumers must use a CSV reader for quoting/newlines and then parse the legacy cell as JSON; strict nine-column consumers need updating. This is a design rationale, not evidence that old consumers automatically accept the new header.

OBSERVATION: the owner reports all previously expected-failure tests now pass and an invented-file mutation check cuts/corrupts input and asserts no panic. No new pass-count total is inferred. Those checks do not prove safety for every malformed input. Historical test files on this documentation branch are not edited or reclassified by this docs-only task.

UNKNOWN: native page semantics, C's code page, original reading order and broad B coverage remain unresolved. The exact new implementation commit and cap/omission indicators were not supplied. A successful command-result retention test can pass without native save/reload preserving sourceText; it must not be described as that stronger guarantee.

## User guide draft for README

### Opening old Publisher files (97, 98, 2000)

INFERENCE: **Experimental.** Supported old Publisher files can open as recovered text in a new editable document. This does not recreate the original publication. Opening the file does not change the original. Review the recovered text and Import notes, and keep the original file.

OBSERVATION: the successful-opening notice reads:

> Experimental text recovery. Recovered text is ready to edit. This is a new document, not the original layout.

OBSERVATION: its shared detail reads:

> Original frames, fonts, formatting, page order, tables and pictures are not recovered. The order of recovered text may not be the reading order. Keep the original file. If possible, open it in Publisher 2002 or later and save a new copy.

OBSERVATION: when recovery is partial, the notice reads:

> Experimental text recovery. Some text could not be recovered. Missing text is reported below; text we could not read is marked in the document. This is a new document, not the original layout.

OBSERVATION: unreadable text is marked “[Text could not be read. No replacement text was guessed.]”. An empty supported store has this separate notice:

> Experimental text recovery found no text in the supported text store. This does not mean the original file is blank. The empty page is a recovery workspace, not an original page.

OBSERVATION: page size is estimated. The estimated-size notice reads “Page size is estimated from stored numbers. It has not been verified.” A rejected large candidate instead says:

> Original page size is unverified. A standard A4 recovery page is being used.

INFERENCE: the plain recovery pages, text placement and A4 fallback are editing choices, not proof of the original layout. Some controls or paragraph endings may not appear in the editable text, and text can extend beyond a generated frame. Read the warnings. If possible, open the original in Publisher 2002 or later, save a separate copy and open that copy. No recovery result is a promise of fidelity.

## Release notes draft

INFERENCE: **DRAFT ONLY — not published.** Proposed release wording:

- INFERENCE: Experimental text-only opening for supported Publisher 97/98/2000 layouts creates a new document and leaves the original unchanged. Unreadable ranges have labelled placeholders; supported empty stores have their own notice.
- INFERENCE: Large unverified page-size candidates now use an A4 recovery workspace with an explicit warning. Source control/trailing-paragraph text is available in bounded command-result sourceText; it is not a complete saved-document recovery guarantee.
- INFERENCE: Batch report.csv adds one final legacy JSON column; convert-folder report.json adds a per-file legacy object. These structural reports omit sourceText and cap provenance. buildCommit accompanies readerRevision when supplied at build time, otherwise it is unknown.
- INFERENCE: Original frames, fonts, formatting, page order, tables and pictures are not recovered. C non-ASCII decoding, native physical page sizes, reading order and wider B coverage remain unconfirmed. Keep the original file and review recovery warnings.

## Owner Windows release checklist

INFERENCE: run this once against the intended implementation build, not this documentation branch. Use the existing Sample98, Sample2000 and JAZZY bytes at the paths below. Do not substitute invented/tester files for a missing case. These commands create local copies/reports only; they do not publish a release. Record the commit and any uncommitted changes: a version string or buildCommit alone cannot identify uncommitted source changes.

1. INFERENCE: build the app/CLI, explicitly supplying the actual build commit. Stage identical copies of the three existing files in a fresh temporary input directory so both folder commands process exactly those three cases. Record original sizes/hashes before opening.

```powershell
# INFERENCE: run locally on the owner's PC; no private file is uploaded.
Set-Location 'E:\Projects\NiftyDesign-src'
$legacyCommit = git rev-parse HEAD
if ($LASTEXITCODE -ne 0) { throw 'Cannot identify the build commit.' }
git status --short
$env:NIFTY_BUILD_COMMIT = $legacyCommit.Trim()
cargo build --release -p designcraft -p designcraft-cli
if ($LASTEXITCODE -ne 0) { throw 'Build failed; do not use stale executables.' }
$legacyCli = (Resolve-Path '.\target\release\designcraft-cli.exe').Path
$legacyApp = (Resolve-Path '.\target\release\designcraft.exe').Path
& $legacyCli --version
$legacySources = @(
  'E:\Projects\NiftyDesign\apache-pub-files\Sample98.pub',
  'E:\Projects\NiftyDesign\apache-pub-files\Sample2000.pub',
  'E:\Projects\quarantine\legacy-intake\in\JAZZY.PUB'
)
$legacyRoot = Join-Path $env:TEMP ('nifty-legacy-check-' + [guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $legacyRoot | Out-Null
$legacyInput = Join-Path $legacyRoot 'input'
New-Item -ItemType Directory -Path $legacyInput | Out-Null
$legacyBefore = foreach ($legacySource in $legacySources) {
  $legacyFile = Get-Item -LiteralPath $legacySource -ErrorAction Stop
  Copy-Item -LiteralPath $legacyFile.FullName -Destination $legacyInput -ErrorAction Stop
  [pscustomobject]@{Name=$legacyFile.Name; Bytes=$legacyFile.Length;
    SHA256=(Get-FileHash -LiteralPath $legacyFile.FullName -Algorithm SHA256).Hash}
}
$legacyBefore | Export-Csv -NoTypeInformation -Path (Join-Path $legacyRoot 'identity-before.csv')
```

2. INFERENCE: capture each file.open command result with script stdin. The raw command JSON can now contain private sourceText: keep these files local and do not share them unredacted. Compare only the selected structural fields with the expected table below. A buildCommit of unknown or a mismatched commit is a release-provenance failure when this build supplied NIFTY_BUILD_COMMIT.

```powershell
# INFERENCE: UTF-8 is set for this process's native stdin, not a system setting.
$OutputEncoding = [System.Text.UTF8Encoding]::new($false)
foreach ($legacyFile in Get-ChildItem -LiteralPath $legacyInput -File) {
  $legacyCommand = @{command='file.open'; params=@{path=$legacyFile.FullName}} | ConvertTo-Json -Compress
  $legacyRaw = $legacyCommand | & $legacyCli script - 2> (Join-Path $legacyRoot ($legacyFile.Name + '.stderr.txt'))
  $legacyExit = $LASTEXITCODE
  $legacyRaw | Set-Content -LiteralPath (Join-Path $legacyRoot ($legacyFile.Name + '.private-command.json')) -Encoding UTF8
  if ($legacyExit -ne 0) { throw ('Open failed: ' + $legacyFile.Name) }
  $legacyReply = ($legacyRaw -join "`n") | ConvertFrom-Json
  $legacyResult = $legacyReply.results[0].legacy
  if ($null -eq $legacyResult) { throw 'Missing typed legacy command result.' }
  if ($legacyResult.buildCommit -ne $legacyCommit.Trim()) { throw 'Missing or stale build commit.' }
  if ($legacyResult.segmentCount -ne ($legacyResult.decodedCount + $legacyResult.undecodedCount)) { throw 'Segment-count mismatch.' }
  $legacyResult | Select-Object family,scope,segmentCount,decodedCount,undecodedCount,encodingUnknownCount,
    rawSizePair,sizeStatus,recoveryPageSize,readerRevision,buildCommit | ConvertTo-Json -Depth 8
}
```

| Basis | Existing file | Expected family / scope | Segments / decoded / unknown | Raw pair and generated size |
| --- | --- | --- | --- | --- |
| OBSERVATION / INFERENCE | Sample98.pub | A / Complete | 6 / 6 / 0 | [7560000,10692000]; inferred-from-raw; about 595.276 x 841.890 pt, 210 x 297 mm. |
| OBSERVATION / INFERENCE | Sample2000.pub | B / Complete | 6 / 6 / 0 | Same raw pair and inferred A4 size; this is the existing single B case, not independent broad coverage. |
| OBSERVATION / INFERENCE | JAZZY.PUB | C / Partial | 72 / 64 / 8 | [18288000,18288000] retained; default-A4; generated 210 x 297 mm, not the rejected 508-mm square. |

INFERENCE: these expected values combine the owner's current state with the earlier identity-specific census. Require route=experimental-legacy, openResult=recovery-document and storageOrderOnly=true in every case; unknown here means encodingUnknownCount, not an entirely omitted segment count. Do not assert an old generated page count after the A4 fallback. The six Sample stories total 488 UTF-16 units; that does not measure visible paragraph presentation or original page order.

3. INFERENCE: run both conversion commands on the same staged inputs. Each must process three files without failure, with recovery warnings. The CSV must have ten columns with legacy last; parse that cell as JSON. Each folder file entry must have legacy. Compare their structural fields with the command results/table, including buildCommit and the raw-versus-generated size distinction. Aggregate counts must remain unchanged despite capped provenance. Neither structural report may contain sourceText.

```powershell
& $legacyCli batch-convert $legacyInput --out (Join-Path $legacyRoot 'batch') `
  1> (Join-Path $legacyRoot 'batch-summary.json') 2> (Join-Path $legacyRoot 'batch-errors.txt')
if ($LASTEXITCODE -ne 0) { throw 'Batch conversion failed; inspect its errors/report.' }
& $legacyCli convert-folder $legacyInput (Join-Path $legacyRoot 'folder') `
  1> (Join-Path $legacyRoot 'folder-summary.txt') 2> (Join-Path $legacyRoot 'folder-errors.txt')
if ($LASTEXITCODE -ne 0) { throw 'Folder conversion failed; inspect its errors/report.' }
$legacyRows = @(Import-Csv -LiteralPath (Join-Path $legacyRoot 'batch\report.csv'))
if ($legacyRows.Count -ne 3) { throw 'Expected three batch rows.' }
foreach ($legacyRow in $legacyRows) {
  if ($legacyRow.status -ne 'converted with warnings') { throw 'Unexpected batch status.' }
  if ([string]::IsNullOrWhiteSpace($legacyRow.legacy)) { throw 'Missing legacy JSON column.' }
  if ($legacyRow.legacy -match '"sourceText"\s*:') { throw 'Private sourceText leaked into CSV.' }
  $legacyRow.legacy | ConvertFrom-Json | Select-Object family,scope,segmentCount,decodedCount,
    undecodedCount,encodingUnknownCount,rawSizePair,sizeStatus,recoveryPageSize,buildCommit
}
$legacyFolder = Get-Content -LiteralPath (Join-Path $legacyRoot 'folder\report.json') -Raw | ConvertFrom-Json
if ($legacyFolder.files.Count -ne 3) { throw 'Expected three folder entries.' }
foreach ($legacyEntry in $legacyFolder.files) {
  if ($null -eq $legacyEntry.legacy) { throw 'Missing folder legacy object.' }
  if (($legacyEntry.legacy | ConvertTo-Json -Depth 20) -match '"sourceText"\s*:') { throw 'Private sourceText leaked into folder report.' }
  $legacyEntry.legacy | Select-Object family,scope,segmentCount,decodedCount,undecodedCount,
    encodingUnknownCount,rawSizePair,sizeStatus,recoveryPageSize,buildCommit
}
```

4. INFERENCE: start the freshly built app with `Start-Process -FilePath $legacyApp`. Use File > Open for the three original paths in legacySources, one at a time. Sample98/Sample2000 must show the complete text-recovery notice and plain editable recovery text. JAZZY must show the partial notice, unreadable-text placeholders and the exact unverified-size/A4 warning. Expand Import notes; record the window title, zoom and ruler unit. Inspect File > Document Setup and cancel: generated dimensions must agree with the report in matched units. Default picas are not millimetres. Do not expect original pictures, tables, frame positions or page order.

5. INFERENCE: after opening, recompute original sizes/hashes and compare them with the before values. A refusal from these known cases, blank replacement, crash, changed original, missing notice/legacy data, guessed unreadable text, wrong counts, wrong A4 policy, stale/unknown supplied build commit, sourceText in a structural report, or an app/report size mismatch is a failed release check. Retain private command captures locally; only redacted screenshots and structural summaries are suitable for sharing. A recoverable overflow warning still needs review and is not proof of faithful layout.

```powershell
$legacyAfter = foreach ($legacySource in $legacySources) {
  $legacyFile = Get-Item -LiteralPath $legacySource -ErrorAction Stop
  [pscustomobject]@{Name=$legacyFile.Name; Bytes=$legacyFile.Length;
    SHA256=(Get-FileHash -LiteralPath $legacyFile.FullName -Algorithm SHA256).Hash}
}
if (Compare-Object $legacyBefore $legacyAfter -Property Name,Bytes,SHA256) { throw 'Original identity changed.' }
Write-Output ('Local check records: ' + $legacyRoot)
```

UNKNOWN: this checklist is a proposed recurring owner procedure, not a run performed in this docs task. Saved-document control retention, behavior beyond retention/report caps and new release-build provenance are not certified by the three small cases alone.

## Ordered remaining roadmap

| Basis / order | Work and why it matters | Evidence/acceptance needed | New files or blockage |
| --- | --- | --- | --- |
| INFERENCE / 1 | Preserve original controls/trailing paragraphs in the saved native document. Command-only sourceText can disappear when the process closes; bounded retention must not be called universally lossless. | An invented segment with tab, FF, another control and trailing CR/CR must survive native save/reload as original source metadata, separate from edited text. Check source provenance, editing isolation, cap/omission indicators and privacy; exercise over-4096-character, over-1000-segment and over-500-provenance cases separately. | No new real files needed. Byte-built data can establish persistence/limits; no tests are added this round. |
| INFERENCE / 2 | Replace the character-count fit estimate with measured composition. A converted page can still hide text; fallback A4 changes wrapping. | Compare actual composition/overset against invented long and wide-glyph paragraphs, plus a local recheck of the earlier CATALOG overflow case. Never use the same estimator as its own oracle or assume the old 61-character count persists. | No new real files needed; existing local files and invented cases suffice. |
| INFERENCE / 3 | Set NIFTY_BUILD_COMMIT in the release workflow and keep buildCommit next to readerRevision. A package version does not identify a binary's source. | Built artifacts must report the intended full commit consistently through command, batch and folder results; deliberately unset metadata must say unknown. Record dirty source separately and verify packaging has the same provenance. | No new files needed; workflow/build verification only. |
| INFERENCE / 4 | Print friendly Windows report paths without exposing ordinary verbatim prefixes. Preserve canonical paths for actual I/O. | Drive and UNC display cases must produce usable friendly paths; keep long-path/device namespace handling explicit and do not globally replace slashes. Compare CSV/folder path output with real output existence. | No new Publisher files needed; temporary paths suffice. |
| INFERENCE / 5 | Establish the physical meaning of raw size fields. Correct pica/mm arithmetic does not establish original page size. | Independent native width/height/orientation for the same files, or independently saved known-size data, must track the stored pair. A4 fallback is not evidence of a standard original page. | **Blocked on independent native-size evidence.** New files are unnecessary if existing files can supply it locally; numeric fits alone cannot close it. |
| INFERENCE / 6 | Establish C's code page before decoding the remaining non-ASCII segments. Readability is not an encoding oracle. | Owner-known characters/language or native comparison must discriminate candidate encodings; preserve strict partial recovery until then. Byte-built tests can check a chosen decoder, not identify a native code page. | **Blocked on independent character/encoding evidence.** Existing templates may supply local ground truth; otherwise an independently known-text sample is needed and is not currently available. No sample request is made. |
| INFERENCE / 7 | Replicate family B beyond Sample2000. Its presence on the PC is settled, but populated B framing/link evidence is still single-file FRAGILE. | Another independent B save with varied IDs/story lengths and recorded native size must pass the profile, extents, Quill links/ranges and size checks. Shared Sample98/Sample2000 text does not prove independent text variation. | **Blocked without another independent B sample.** None of the nine existing identities replaces that missing replication; invented fixtures do not increase real-file coverage. |
| INFERENCE / 8 | Determine whether any original reading order can be justified. Generated storage-order pages must not imply it. | Compare intended/native story sequence with preserved IDs/ranges on independently labelled examples. An ascending ID, byte order or one successful layout is insufficient. Retain storageOrderOnly until a supported rule is independently demonstrated. | **Blocked on independent intended-order evidence.** A local native view of current files could help; otherwise known-order samples are needed. No such evidence is claimed from the current nine-file corpus. |

## Single next step

INFERENCE: the next code-lane step is one byte-built native-save/reload check for original sourceText and provenance, using invented controls and two trailing paragraph marks below the retention caps. Check the reloaded document, not the transient command JSON. That establishes the precise persistence gap before implementing saved-document source retention; it needs no new private or tester file. This is a future code-lane task, not a test added or run here.
