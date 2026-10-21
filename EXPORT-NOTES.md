<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# NiftyDesign source export — 2026-10-21

Source version: **0.9.0-nifty-alpha.1**. Prepared from merged `pub-import`
45378356 on `codex/queue`, then switched to a new orphan branch
`niftydesign-export`. The export has one parentless commit, without the old
repository's commit history. This is a **private source transfer**, not a release,
deployment, visibility change or workflow dispatch. The original `codex/queue`
and `pub-import` branches are not rewritten.

## Every file removed from the starting tree

The original root README was replaced by the prepared NiftyDesign README.
The public-repo files below were moved to their final destinations, not lost.
The removed newsletter experiment contains Microsoft template text/clip art;
none of its images or scripts is part of this export. Additional old autopilot,
release-planning and upstream-proposal notes were excluded as owner/process
material, rather than shipping obsolete instructions and draft release claims.

- `docs/ALPHA-RELEASE.md` — removed (process / owner notes).
- `docs/AUTOPILOT.md` — removed (process / owner notes).
- `docs/DEBRAND.md` — removed (process / owner notes).
- `docs/FORK-OPTIONS.md` — removed (process / owner notes).
- `docs/PUB-AGENTS.md` — removed (process / owner notes).
- `docs/REPORTS.md` — removed (process / owner notes).
- `docs/ROADMAP.md` — removed (process / owner notes).
- `docs/SHIP.md` — removed (process / owner notes).
- `docs/TASKS.md` — removed (process / owner notes).
- `docs/UPSTREAM-ISSUE.md` — removed (process / owner notes).
- `docs/brand/artcraft-logo-white.png` — removed (upstream trademark artwork).
- `docs/brand/artcraft-logo-white.svg` — removed (upstream trademark artwork).
- `docs/brand/artcraft-logo.png` — removed (upstream trademark artwork).
- `docs/brand/artcraft-logo.svg` — removed (upstream trademark artwork).
- `docs/brand/artcraft-mark-black.png` — removed (upstream trademark artwork).
- `docs/brand/artcraft-mark-black.svg` — removed (upstream trademark artwork).
- `docs/brand/artcraft-mark.png` — removed (upstream trademark artwork).
- `docs/brand/artcraft-mark.svg` — removed (upstream trademark artwork).
- `docs/public-repo/.github/ISSUE_TEMPLATE/bug_report.yml` — moved to `.github/ISSUE_TEMPLATE/bug_report.yml`.
- `docs/public-repo/.github/ISSUE_TEMPLATE/config.yml` — moved to `.github/ISSUE_TEMPLATE/config.yml`.
- `docs/public-repo/.github/ISSUE_TEMPLATE/file_failed_to_open.yml` — moved to `.github/ISSUE_TEMPLATE/file_failed_to_open.yml`.
- `docs/public-repo/CONTRIBUTING.md` — moved to `CONTRIBUTING.md`.
- `docs/public-repo/KNOWN-LIMITATIONS.md` — moved to `docs/KNOWN-LIMITATIONS.md`.
- `docs/public-repo/README.md` — moved to `README.md`.
- `docs/public-repo/RELEASE-CHECKLIST.md` — removed (owner-only release checklist).
- `docs/screenshots/real-file/01-open-page-2.png` — removed (Microsoft-template newsletter experiment).
- `docs/screenshots/real-file/02-moved.png` — removed (Microsoft-template newsletter experiment).
- `docs/screenshots/real-file/03-resized.png` — removed (Microsoft-template newsletter experiment).
- `docs/screenshots/real-file/04-edited-text.png` — removed (Microsoft-template newsletter experiment).
- `docs/screenshots/real-file/05-saved-reopened.png` — removed (Microsoft-template newsletter experiment).
- `docs/screenshots/real-file/06-export-pdf-dialog.png` — removed (Microsoft-template newsletter experiment).
- `docs/screenshots/real-file/07-exported-pdf-page-2.png` — removed (Microsoft-template newsletter experiment).
- `docs/screenshots/real-file/08-fresh-session-reopen.png` — removed (Microsoft-template newsletter experiment).
- `docs/screenshots/real-file/README.md` — removed (Microsoft-template newsletter experiment).
- `docs/screenshots/real-file/editing.jsonl` — removed (Microsoft-template newsletter experiment).
- `docs/screenshots/real-file/fresh-reopen-probe.jsonl` — removed (Microsoft-template newsletter experiment).
- `docs/screenshots/real-file/verify.py` — removed (Microsoft-template newsletter experiment).

Directories removed once empty: `docs/public-repo/` (including its nested
`.github/ISSUE_TEMPLATE/`) and `docs/screenshots/real-file/`.

Partial removals / replacements:

- `README.md`: removed the old upstream product README; replaced it with the
  prepared NiftyDesign README from `docs/public-repo/README.md`.
- `docs/SAMPLE-REQUEST.md`: removed the introductory owner/process instructions,
  the complete “Intake checklist (for the owner, per file)” and “Open points for
  the owner” sections. Retained only the request text and its unresolved contact
  placeholder; replaced its old product name with NiftyDesign.
- Removed draft-only editorial boxes/headings from the promoted README,
  CONTRIBUTING, KNOWN-LIMITATIONS and issue forms, plus the retained parity and
  tester/web guides. Unverified facts and pending contact/contribution decisions
  remain visibly unresolved; they are not silently marked verified.
- `ASSETS.md`: removed ledger rows for removed mark/newsletter files; retained
  the code, font, synthetic gallery, Sample2, owner-branding and demo-image credits.
- Root `AGENTS.md` and `CLAUDE.md`: replaced only their fork-note references to
  the removed queues with links to CLEANROOM.md and SOURCES.md. The upstream
  guidance and other existing source-file headers remain intact.

## What was kept and checked visually

All retained images are complete original outputs; no image was retouched.
“Microsoft content” here means template text/clip art, not the necessary plain
text product name in the application's description or font-family names.

| Checked files | Content seen | Decision |
|---|---|---|
| `docs/gallery/hello-nifty-page-1.png`, `hello-nifty-pdf-page-1.png` | “Hello from NiftyDesign” and blue rectangle | Keep: own synthetic input |
| `docs/gallery/wmf-picture-page-1.png` | Same text; red triangle and blue ellipse | Keep: own synthetic WMF, not clip art |
| `docs/gallery/bullets-page-1.png` | “Hello Example Text world” and bullet | Keep: controlled fixture |
| `docs/gallery/shadow-page-1.png` | Blue rectangle and dark outline/shadow | Keep: controlled fixture |
| `docs/gallery/table-2x2-page-1.png` | Uniformly blank page | Keep: openly documented failed table demonstration |
| `docs/screenshots/01-start.png` through `07-edited-pdf-page.png` | Own Nifty mark; own hello/WMF sample, “Sunday service at 10:00”, “Welcome to NiftyDesign.”, File menu and PDF options | Keep: own synthetic samples |
| All seven PNGs in `docs/screenshots/real-file-2/Sample2.pub/` | Generic formatting-test text: “This is some text on the first page”, “This is in bold and italic”, “NiftyDesign clean-file editing check.”; no pictures or template text | Keep: Sample2 only |
| `docs/images/page-cover.png`, `page-2.png`, `page-3.png`, `page-4.png`, `ui-spread.png` | Original synthetic magazine: “The Quiet Art of Layout”, “Notes on the Grid”, “Color, Ink & Paper”; code-generated sunset/hills and layout discussion | Keep: inherited project sample; historical UI title is not a claim about this build |
| `docs/demo/coffee-shop/` captures and PDF rasters | Generic Sample2 original; fictional coffee shop, example contacts and owner-generated art | Keep: original fictional demo; source/provenance already recorded |
| `docs/screenshots/real-file/` | Owner identifies this as Microsoft-template newsletter content; existing provenance also identifies clip art | Remove the whole experiment; do not repackage it |

The gallery contains **six PNGs**; real-file-2 contains **seven Sample2 PNGs** and
no other sample's image. The old 24-file scan text is retained as a record, not
as permission to include images of the other real files. No Apache `.pub` is
included. The only checked-in `.pub` inputs remain the two project synthetic
fixtures under `crates/pub/tests/data/`.

All Markdown image references in README files were checked to resolve inside
the exported tree. The root README's six image references point only to the
synthetic samples and Sample2. The gallery PDF and screenshots PDF originate
from those same own samples. The coffee-shop native/PDF artifacts retain only
fictional added content; no original Sample2 objects/stories remain hidden.

## Licence and header changes

- LICENSE-MIT and LICENSE-APACHE are byte-identical to the starting tree.
- NOTICE now starts “NiftyDesign”, contains the requested copyright,
  MIT/Apache option, unofficial/non-affiliation and trademark/owner-mark text,
  preserves the complete existing third-party list, and adds the demo images.
- Every Rust source under crates/pub and every newly added NiftyDesign code,
  test, script/workflow and documentation file has the requested owner/SPDX
  notice. Rust uses the exact `// Copyright ...` line; Python/PowerShell/YAML/TOML
  use `#`, Markdown uses an HTML comment. Binary assets, JSON/JSONL data, licence
  texts and the owner's verbatim NOTES.txt were not given invalid code comments
  or relicensed. Their terms and provenance remain in ASSETS.md / NOTICE.
- contents.rs, quill.rs and read.rs differ **only by the added copyright line**;
  removing that first line reproduces the starting files byte for byte.
- Existing upstream source-file headers were left unchanged. A new-file list
  was determined relative to the parent of the original clean-room import.
- The branding checker retains all eight removed mark filenames and SHA-256
  fingerprints, so renamed copies still fail without shipping the artwork.
  Its regression test uses synthetic bytes instead of loading a removed logo.
- `/.builder/`, `__pycache__/` and Python bytecode patterns were added to .gitignore, so generated caches, reproduction output, score reports
  and externally sourced evidence cannot be accidentally included by a fresh
  clone's default add operation.

## Remaining placeholders and UNVERIFIED items

No pending decision below is treated as an approval or as a completed test.
This inventory includes all literal owner-placeholder and UNVERIFIED occurrences
in retained text files, excluding this inventory itself. Line references refer
to this export. Format-note hypotheses remain hypotheses, not encoding rules.
The known-limitations UNVERIFIED section also explicitly includes desktop GUI,
in-app printing/system dialog, Windows system fonts, hosted package builds,
web picking/dropping, SmartScreen wording, untested no-network claims and
untested Windows versions.

### `CONTRIBUTING.md`

- Line 21: - [ ] You send it only after the maintainer asks, by the private channel they give you. \`[owner fills in: contact method]\`
- Line 26: The source is in this repository. Pull requests may be welcome. \`[owner fills in: how you want contributions handled]\` If a fix belongs in shared DesignCraft code, it goes upstream to ArtCraft **only if the owner chooses** to offer it. Do not open issues or pull requests with ArtCraft on the maintainer's behalf.

### `README.md`

- Line 26: - **The build is not code-signed.** Windows SmartScreen may warn you when you first run it. *UNVERIFIED: the exact wording and clicks have not been recorded.* Run it only if you are comfortable with an unsigned test build, and only if you downloaded it from the Releases page above.
- Line 67: | **UNVERIFIED** | The desktop window and editing in it on a real Windows PC. In-app printing. Whether Windows system fonts are picked up. Any hosted build. The web file picker. |
- Line 94: - **In-app printing is UNVERIFIED.** It has not been tried on a real Windows desktop with a printer.
- Line 104: - Whether Windows system fonts are picked up on a real Windows PC is **UNVERIFIED**.

### `docs/KNOWN-LIMITATIONS.md`

- Line 28: - Whether Windows system fonts are picked up on a real Windows PC is **UNVERIFIED**.
- Line 33: - The File menu has **Print...** (Ctrl+P). In-app printing is **UNVERIFIED**.
- Line 59: ## UNVERIFIED (no evidence in the project docs)
- Line 68: - Behaviour on Windows versions other than the one the owner tests. \`[owner fills in]\`

### `docs/SAMPLE-REQUEST.md`

- Line 19: **To send one:** reply here or contact [OWNER TO FILL IN: contact method]. Please tell us which Publisher version made it, if you know, and what the file is for. Saving a copy in Publisher 2002 or later is fine for old files, but we would like the original too if you have it.

### `docs/WEB-TEST.md`

- Line 11: - **Not checked** (REPORTS.md says so): the browser's file picker, drag-and-drop, and on-screen rendering. Everything below about what you will see in the browser is **UNVERIFIED** until someone does it.

### `docs/demo/coffee-shop/README.md`

- Line 155: and color/print production quality are **UNVERIFIED**.

### `docs/pub-format/anchor-coordinates.md`

- Line 4: Written 2026-10-19 by the data-only describer. Every statement is labelled **OBSERVATION** (read from the files, with the file names that show it), **INFERENCE** (follows from observations; reasoning given), **UNKNOWN**, or **UNVERIFIED HYPOTHESIS**. Inputs: this repository's \`CLEANROOM.md\` and \`docs/pub-format/\` notes, and the \`.pub\` data files listed in \`SOURCES.md\` under "Describer (data-only)". No reader code, no \`truth/*.json\`, no golden images. The MS-ODRAW page could not be fetched (host unreachable), so nothing below relies on specification text; record names are used only as already referenced in \`README.md\`.
- Line 67: - **Sign / direction — UNKNOWN.** The stored number equals the name's number, so there is no sign flip between name and file, but the files contain no rendering. Whether positive is clockwise on screen is not settled. (UNVERIFIED HYPOTHESIS: clockwise, as that is the usual UI convention; nothing here shows it.)

### `docs/pub-format/unknown-fields.md`

- Line 4: Written 2026-10-19 by the data-only describer. Labels: **OBSERVATION**, **INFERENCE**, **UNKNOWN**, **UNVERIFIED HYPOTHESIS**. Inputs: the documented property-list and Quill formats (\`property-lists.md\`, \`text-store.md\` §§7–10) applied to the 142 pubsmith and 24 Apache files listed in \`SOURCES.md\`; nothing else. "Default" = the second \`STSH\` section's entry 0 (character) or entry 1 (paragraph); "run" = a record from \`FDPC\` (character) or \`FDPP\` (paragraph).
- Line 49: - **0x39 — meaning UNKNOWN.** The only association is "copies the size value in the same record" (240/240), which is correlation. UNVERIFIED HYPOTHESIS: a size for a separate text class (for example a second script), written alongside 0x0C by some writers/settings. No pair isolates it; confidence: none.
- Line 50: - **0x48 — meaning UNKNOWN.** Same pattern with the colour key 0x44. UNVERIFIED HYPOTHESIS: a colour for a separate text class. Confidence: none.
- Line 52: - **0x3C — meaning UNKNOWN; association established.** The pair \`text__base\` / \`text__align-justify\` adds 0x3C together with alignment 6, so the pair isolates "justified paragraph" as the difference but cannot separate 0x04 from 0x3C. Centre and right alignment add no 0x3C. Combined with the Apache runs (5/5 justified, 0/5 others) this is solid evidence that **0x3C is written whenever alignment is justified**; what its value encodes is unknown (UNVERIFIED HYPOTHESIS: a justification parameter; the different values 0x4832 versus 0x0032 are unexplained). Confidence that it is tied to justification: high; that it has any particular meaning: none.

### `docs/publisher-parity-checklist.md`

- Line 57: | Files and versions | Opening a file through the web file picker | P1 | unscored | UNVERIFIED: only a headless test exists. |
- Line 70: | Character formatting | Missing-font substitution | P1 | P | Microsoft fonts cannot be included. Arial and Times New Roman go to Liberation Sans and Liberation Serif where available. Arial Black, Franklin Gothic, Rockwell, Elephant, Sylfaen and others go to a default font, so text can wrap differently. A Find/Replace Font dialog lists missing fonts. Windows system fonts on a real PC: UNVERIFIED. |
- Line 114: | Editing in the app | Select, move and resize imported items | P0 | D | Engine workflow test, and two real files (newsletter, Sample2.pub) with engine-checked bounds; the desktop GUI itself is UNVERIFIED. |
- Line 115: | Editing in the app | Edit imported text | P0 | D | Engine test, screenshot 04, and the same two real files (replace text, save, reopen, export PDF); desktop GUI UNVERIFIED. |
- Line 117: | Editing in the app | Using all of the above in a real desktop window | P0 | unscored | UNVERIFIED: screenshots come from a test renderer, not a desktop. |
- Line 120: | Printing | In-app File > Print | P1 | P | The File menu now has Print... (Ctrl+P), an existing engine command exposed in the menu. Automated tests check the engine side. Whether it works on a real Windows desktop and printer is UNVERIFIED. Export to PDF works (automated checks). |
- Line 121: | Printing | Physical printing and the native print dialog | P1 | unscored | UNVERIFIED: never observed on a desktop. |
- Line 125: These rows have no supporting statement in the documents, so no status was guessed. UNVERIFIED means the docs say it has never been observed.
- Line 128: - **Files and versions: Opening a file through the web file picker** (P1). UNVERIFIED: only a headless test exists.
- Line 135: - **Editing in the app: Using all of the above in a real desktop window** (P0). UNVERIFIED: screenshots come from a test renderer, not a desktop.
- Line 138: - **Printing: Physical printing and the native print dialog** (P1). UNVERIFIED: never observed on a desktop.
- Line 188: - **Printing.** Print... (Ctrl+P) is now in the File menu. Please try it on a real Windows PC with a printer and tell us the result. Until then the row stays P and UNVERIFIED.

### `docs/screenshots/README.md`

- Line 4: Interface renderings from the project's own test renderer; not a real Windows desktop capture; native file dialogs and printing are UNVERIFIED.
- Line 57: Native Save As/format selection and printing remain UNVERIFIED; the custom

### `docs/screenshots/real-file-2/Sample2.pub/README.md`

- Line 113: was dispatched, published or deployed. Native Windows interactions remain UNVERIFIED.

### `packaging/test_alpha_windows.py`

- Line 45:                 for text in [alpha.VERSION, 'REPO_URL_HERE', 'UNVERIFIED', 'SmartScreen',

### `packaging/windows/README-ALPHA.txt`

- Line 10: The exact SmartScreen prompt and launch experience are UNVERIFIED on a real
- Line 36: UNVERIFIED ON A REAL DESKTOP: the desktop GUI, interactive editing, file dialogs,
- Line 38: UNVERIFIED; this Windows package does not include the web app.
- Line 54: REPO_URL_HERE

## Verification results

Results are recorded below after running the checks on this tree. No Windows
binary was built or run for this source export. No desktop or print verification
is implied. The full workspace/app/wasm build was not run.

| Check | Result |
|---|---|
| packaging/check-branding.py on the clean staged source tree | PASS; required legal files and all font licences present; no forbidden mark names/bytes |
| Python packaging tests | PASS: 4 tests, including renamed-byte mark rejection, alpha staging and Windows branding/version contract |
| cargo fmt; cargo fmt --check | PASS |
| cargo check -p designcraft-pub | PASS |
| cargo clippy -p designcraft-pub -p designcraft-cli -p designcraft-ui-egui -- -D warnings | PASS |
| cargo test -p designcraft-pub -p designcraft-cli | PASS: 70 passed, 11 ignored |
| Engine pub_workflow, web_pub, pub_corpus integration targets | PASS: 2 passed, 1 ignored |
| UI library tests | PASS: 545 passed, 1 ignored |
| Explicit ignored pub corpus score test | PASS: 142/142 imports, 152/152 objects, 111/152 positions within 1 pt, 52/52 texts, 111/111 styles; unchanged |
| Real 24-file batch, existing local CLI | Unchanged: 0 OK / 19 warnings / 2 refused / 3 damaged failures; all CSV result rows match the starting-code baseline, excluding output PDF paths |
| Asset audit | PASS: all 94 retained asset files attributed |
| Layering | PASS: 22 crates, no violations |
| README images | PASS: all local Markdown image references in README files exist; root README has six, all reviewed |
| Protected readers / code licences | PASS: readers header-only; LICENSE-MIT and LICENSE-APACHE byte-identical |

The first optional engine/UI rebuild attempt **failed with ENOSPC** (disk full),
not a test assertion. Failed, unlinked compiler objects and that unfinished
incremental session were removed; completed cache artifacts were retained or
preserved as SHA-256-verified compressed archives outside the checkout. No
`cargo clean` was run. Retried engine tests with per-package test debug=0 and
incremental=false, and UI tests with those settings for engine and ui-egui;
both retries passed as reported above. No source fix was needed for that failure.
The full-workspace, desktop app, wasm and Windows package builds were not run.
All corpus truth data was used for scoring only.

## Transfer result

The root commit is pushed only to the requested new private repository main, or
as the fallback `niftydesign-export` branch of designcraft-pub if access is
refused. No old repository branch is force-pushed and no release is published.
The actual destination and root commit hash are reported in the task response;
the new root commit cannot include its own hash in this file.
