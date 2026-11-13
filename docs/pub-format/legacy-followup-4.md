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
