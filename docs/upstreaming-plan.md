DRAFT ONLY

# Upstream readiness and pull request plan — 2026-10-30

## Scope and conclusion

OBSERVATION: this is a local audit and a plan, not an upstream proposal. No upstream push, issue, pull request or release was made. The inventory freezes this fork at `ca8986ee9bac68c79aa5f91702647057e4f1b6fd` and upstream main at `ea08e4c2ba290f092063c99d70c4bdd5afc45887`; the audit note itself is excluded from the inventory. Only this note and a short data-use source entry are changed by the audit.

INFERENCE: do not send the fork's complete diff upstream. The importer has a separable core, but integration must be ported to the current module layout; branding, packaging, workflow changes, bundled fonts and most demonstration material must stay out. The current combined patch is not ready to merge. Small review units are possible only after deliberate extraction; they are not existing commits that can all be cherry-picked unchanged.

OBSERVATION: the untouched fork baseline is 142/142 imports, 152/152 objects, 111/111 scored positions within 1 pt, 52/52 texts and 111/111 styles. The separate picture result is 12/41 matched passes, zero measured misses and 29 unmatched references. All 142 save/reopen/edit/PDF cases pass. Real batch: 9 with notes, 10 with warnings, 2 refused and 3 damaged.

## Relation to upstream

OBSERVATION: the repository is not shallow. `git merge-base origin/main upstream/main` returns no common ancestor (exit 1). This fork's root is `d82d52562f6e5b4bb039998ea4d45060038b85b7`, whose manifest says 0.9.0-nifty-alpha.1. The source record identifies the original upstream version as 0.9.0, commit `7cb8b12507def48fb9346e5004340b5e8e4cfcb7`.

OBSERVATION: the source record and local upgrade commit `263e780d` say the 0.9.0-to-0.12.0 upstream delta was applied as a three-way patch, preserving importer integration and fork-specific changes. The adopted upstream 0.12.0 tree is `5e8ef678321cfdd2d18927e2af94d58e5f9d8836` (release commit dated 2026-10-22). This adopted tree is the comparison base used below; it is not a merge base. This avoids counting the already adopted 0.9-to-0.12 changes as new work.

OBSERVATION: this fork's current manifest says 0.12.0-nifty-alpha.4. Upstream's current manifest says 0.14.0. From the adopted 0.12.0 commit to the fetched upstream main, there are 782 reachable commits, including 499 non-merge commits, or 193 first-parent steps. These are different distance measures, not three counts to add together.

OBSERVATION: the crates touched most often below are counted by distinct non-merge commits in that range; one commit may touch several crates. File counts or churn would be different measures.

| Upstream crate | Non-merge commits touching it |
| --- | --- |
| crates/ui-egui | 186 |
| crates/engine | 152 |
| crates/pdf | 55 |
| crates/compose | 42 |
| crates/doc | 39 |
| crates/idml | 33 |
| crates/render | 25 |
| crates/textimport | 21 |

OBSERVATION: upstream now separates engine file commands into file/bytes.rs, file/open.rs, file/registry.rs and related modules; UI application behavior into app/*.rs; menus into menus/*.rs; and CLI tests into tests/it/*.rs. Its rules also now require calendar conversion through chrono and forbid CI changes in feature/fix pull requests.

## Grouped inventory

OBSERVATION: comparison is adopted upstream 0.12.0 tree → frozen fork main. Every changed path appears once in the appendix. Renames are disabled for reproducibility, so a move can appear as a deletion and addition. Line counts are textual additions/deletions, not current file length. Binary files have no textual line count and are counted separately. There are 311 changed paths, +23,419 / -1,549 text lines and 79 binary paths.

OBSERVATION: whole-file classification is a census convenience, not permission to send every hunk in a group. Mixed files in (b), (d), (e) and (g) still contain identity or unrelated edits. Group (f) identifies identity-only paths; the mixed identity hunks listed below must also be excluded. The shared font substitution implementation is in (g), not (a), because it changes general font resolution.

| Group | Meaning | Paths | Text + | Text - | Binary paths |
| --- | --- | --- | --- | --- | --- |
| a | Importer and importer-only measurement | 23 | 6907 | 0 | 2 |
| b | Open/import integration and CLI | 21 | 2115 | 94 | 0 |
| c | Added fonts and demo assets | 10 | 196 | 0 | 7 |
| d | Documentation and notes | 128 | 9615 | 501 | 52 |
| e | Packaging, release and test-data infrastructure | 49 | 3421 | 433 | 0 |
| f | Identity-only changes: exclude upstream | 59 | 186 | 473 | 18 |
| g | Other shared changes and examples | 21 | 979 | 48 | 0 |

OBSERVATION: (a) is all of crates/pub plus the importer-only pixel comparison test in apps/designcraft-cli/tests/pub_pixel_diff.rs. It includes two original synthetic 7,168-byte publications, not the external controlled or Apache corpora. (b) includes workspace manifests/lockfile, importer dependency registration, command routing, import state/notices, save suggestions, desktop recognition, workflow tests and the two batch CLI commands. (c) contains four font binaries, two font licences and four demonstration-art files. (e) includes both added and removed workflows; none belongs in the importer feature proposal.

OBSERVATION: mixed identity changes occur in Cargo.toml's version, the CLI manifest description and main/help/version strings, desktop main/title/filter strings, engine file/interchange wording, and UI menus. Engine Cargo.toml additionally contains unrelated Linux printing dependencies. UI lib.rs additionally carries screenshot/status changes. Shared fontdb.rs embeds the added fonts. These hunks need selective extraction or rewriting. Copyright and licence notices are authorship, not branding, and must be preserved.

## Scratch application trial

OBSERVATION: a detached scratch worktree was created under target/ag-review from the fetched upstream main. A binary-capable, full-index delta for all 44 paths in (a)+(b) was tried with three-way/index application. The first all-at-once attempt failed on the missing CLI test path and left the worktree unchanged. Its log also reported potential conflicts; those were not treated as applied changes.

OBSERVATION: applying the same path deltas individually exposed 36 clean applications, seven conflicted files and one unavailable path. All 23 paths in (a) applied textually. In (b), thirteen applied cleanly, seven conflicted and one could not apply. No conflict was resolved. An EOF-whitespace warning also appeared in the initial patch log; no source was reformatted or repaired for this trial.

| Path | Result | Observed overlap and proposed treatment |
| --- | --- | --- |
| Cargo.lock | Conflict | Upstream has newer shared-library/dependency revisions and versions; the fork patch comes from the 0.12 lockfile. Regenerate a minimal lock update on the current upstream base. |
| Cargo.toml | Conflict | Upstream 0.14.0 version conflicts with the fork-specific 0.12.0-nifty-alpha.4 version. Preserve upstream version and add only required dependency entries. |
| apps/designcraft-cli/src/main.rs | Conflict | Usage, dispatch and validation/automation handling evolved while the fork added batch commands and identity text. Extract command registration into current code. |
| apps/designcraft-cli/tests/cli.rs | Missing path | Upstream now uses apps/designcraft-cli/tests/it/cli.rs and tests/it/main.rs. Do not restore the old monolithic path. |
| apps/designcraft/src/main.rs | Conflict | Native dialog handling has changed, including asynchronous dialogs, while the fork changed filters and save suggestions. Preserve upstream flow. |
| crates/engine/src/cmd/file.rs | Conflict | Upstream replaced the monolithic implementation with modules and re-exports. Port registration, sniffing, open and save changes to those modules. |
| crates/ui-egui/src/lib.rs | Conflict | Application state/behavior was extracted to app/*.rs and tests; new asynchronous-save/quit state overlaps the fork changes. |
| crates/ui-egui/src/menus.rs | Conflict | Menus were split into modules/re-exports; the fork changes monolithic menu definitions, save behavior and branding. |

OBSERVATION: an offline cargo check after the failed atomic attempt ran against unchanged upstream and stopped on an uncached dependency: `failed to load source for dependency ecolor`; `can't checkout from https://github.com/storytold/egui.git: you are in the offline mode (--offline)`. No environment or dependency setup was changed to bypass that limitation.

OBSERVATION: after the per-path application, cargo check --workspace --locked --offline fails before compiling code: `key with no value, expected =`, at Cargo.toml line 7 containing the unresolved `<<<<<<< ours` marker. This is a failed check, not a successful integration build.

UNKNOWN: Rust API compatibility, tests, layering and WebAssembly behavior of a conflict-resolved upstream port. No conflict-resolved port was created, and the upstream dependency cache is incomplete. A clean textual application of the importer does not answer those questions.

OBSERVATION: the current fork's browser picker DOC_EXTS in apps/designcraft-web/src/web.rs still omits pub; that file's actual delta is identity-only and is classified (f). The engine and drag/drop/byte paths accept publications, but the upstream UI proposal must explicitly add the browser picker entry too. A successful byte-path test does not test that picker.

## Rules audit

OBSERVATION: upstream AGENTS.md was read at the fetched revision. The following status applies to the existing candidate and its boundaries; a pass on the fork is not a pass on the unresolved upstream worktree.

| Rule | Status | Evidence / required action |
| --- | --- | --- |
| No forbidden panic/unsafe constructs in production | PASS for reviewed (a)/(b) | Importer, engine/UI roots and CLI use strict Clippy denies; workspace forbids unsafe. A scoped source scan finds no forbidden production calls in reviewed candidate paths. Apparent interchange unwrap hits are inside cfg(all(test,...)). This is not a proof that every hostile input can never exhaust resources. |
| Deny attributes on new clean crates | PASS | crates/pub/src/lib.rs carries all six Clippy denies and forbids unsafe; Cargo.toml inherits workspace lints. Existing CLI/engine/UI roots already carry denies. Preserve these upstream. |
| User-visible actions are commands | PASS for open path; FAIL for literal whole-CLI proposal | The importer is reached by file.open/file.openBytes and exportPdf/saveAs. batch.rs/folder.rs add CLI-only orchestration with no engine CommandSpec for the whole folder operation. An engine command design or an explicit upstream exception is required; neither is assumed. |
| Layering | PASS on fork; UNKNOWN upstream port | CI layers passes; pub is L3 with only lower-layer workspace dependencies. xtask/src/layers.rs needs the pub entry. The unresolved port cannot be checked. |
| WebAssembly | PASS on fork; UNKNOWN upstream port | CI wasm checks include the pub crate and engine. The CLI worker process is a native binary concern, not a library dependency. Current-upstream compatibility and browser picker behavior are not established by that build. |
| Thin UI and command dispatch | PASS for intended open feature | file.open/file.openBytes own conversion; UI presents notice/notes and dispatches commands. Rewrite to upstream app/files.rs, file_dialogs.rs and menu modules; omit unrelated screenshot/status/printing changes. |
| Asset ledger | PASS full fork; FAIL bare (a)+(b) slice | ASSETS.md contains both synthetic publication rows. The trial excludes (d), so it lacks those ledger additions; a real PR must bring those two rows or regenerate fixtures outside the repository. Do not copy the complete fork ledger. |
| Fonts remain in separate repository | FAIL for bundled-font proposal | assets/fonts adds BarlowCondensed-Regular.ttf, BarlowCondensed-Bold.ttf, NiftyNarrowSerif-Regular.ttf and NiftyNarrowSerif-Bold.ttf. They conflict with the upstream no-new-font-binaries rule. Move to the designated optional font repository and remove hard-coded includes before proposing shared substitution. |
| No large binary corpus fixtures | PASS for (a)/(b) | Only the two original synthetic 7,168-byte files are in crates/pub/tests/data. The 142 controlled and 24 Apache inputs remain external. Demos and screenshots in (d) are outside the candidate and must not be swept in. |
| No CI changes in feature PR | PASS only for selected feature slice; FAIL whole fork | Group (e) adds/removes/modifies workflows and release automation. Exclude them all. Layer registration is source/test infrastructure, not permission to edit workflows. |
| Calendar conversion through chrono | PASS on reviewed additions | No new hand-written calendar conversion is introduced by the reviewed importer/glue. Worker timeouts use elapsed durations. Preserve current upstream time APIs. |
| Licence and clean-room provenance | UNKNOWN for blanket whole-tree claim | The code notices and declared dependency licences are compatible in the inspected subset. Demo-content rights and exhaustive historical provenance cannot be certified; details below. |

## Proposed small pull request sequence

INFERENCE: the table below is a staged plan, not a set of already prepared patches. Sizes are estimated changed lines including tests; they are not additive whole-file counts. Existing module coupling means extraction is real work: contents.rs uses table types, and tables.rs uses Builder. Each intermediate PR must compile and perform its advertised subset without placeholder panics or silently discarding unsupported data. Keep all existing regression expectations for the completed port; do not weaken them to make a partial slice appear complete.

| Review unit | Files / extraction | Estimated size | Dependencies | Headless verification | Rewrite / main risk |
| --- | --- | --- | --- | --- | --- |
| 1. Evidence and scope notes | Selected content from CLEANROOM.md, pub-format/README.md, contents-stream.md, property-lists.md and SOURCES.md; a short supported/unsupported summary | Small: 300–600 lines | None | Review provenance and links; do not include images, history dumps, corpus contents or private references | Rewrite concise upstream wording; repository-specific instructions and historical identity references are not ready as-is. |
| 2. Bounded parser foundation | crates/pub/Cargo.toml; read.rs, contents.rs, escher.rs, quill.rs; a small public parser entry point and tests; selected Cargo.toml/lock entries; xtask layer entry | Medium: 1,400–1,800 lines; existing four parser files total 1,188 lines including inline tests | 1, if maintainers want documentation first | cargo test -p designcraft-pub; strict clippy; layers; cargo check --target wasm32-unknown-unknown -p designcraft-pub | Requires extraction of table data types and a deliberate parser API; not a straight cherry-pick. Maintain bounds/unknown-field reporting. |
| 3. Basic document conversion | Selected Builder/lib.rs logic plus import.rs/paragraphs.rs tests and generated hello-nifty fixture/ledger row | Medium: 1,200–1,800 lines | 2 | Headless import, text/style/position checks for supported controls, invalid-input tests; native save/reopen | Rewrite lib.rs wiring around the staged parser API. Explicitly report advanced unsupported objects until their later slices; do not advertise completed parity yet. |
| 4. Geometry and composition diagnostics | paths.rs, presets.rs, wordart.rs, overset.rs; related curves/presets/WordArt/overset tests and lib.rs hooks | Medium: 900–1,400 lines | 3 | Hand-computed geometry/fitting tests; affected external controls; nine available pixel references; wasm | Existing isolated helpers/tests are reusable evidence, not proven current-upstream builds. Keep circle unsupported and preserve approximation warnings. |
| 5. Vector picture conversion | metafile.rs; picture_asset hooks; relevant import tests; generated wmf-picture fixture and ledger row | Medium: 1,000–1,300 lines | 3 | Synthetic WMF/EMF tests, image bounds/crop tests, empty/invalid vector input; wasm | Document skipped records and incomplete fidelity. Keep original fixtures/licence notices; no template clip art in the PR. |
| 6. Table conversion | tables.rs; table-related contents/quill/lib hooks; tests/tables.rs and engine-independent table checks | Medium: 900–1,300 lines | 3 | Synthetic cells/merges, three controlled tables and eight real table files; text and geometry checks | Preserve unknown fills/borders/padding and uncertain merged-cell text; split parser/table types without changing interpretations. |
| 7. Open commands and workflow | Upstream file/registry.rs, bytes.rs and open.rs; interchange.rs; DocState/import metadata; engine dependency; selected workflow/corpus/table/web tests | Small–medium: 300–600 lines | 3–6 for completed importer | Session file.open and file.openBytes; original-byte preservation; unsaved copy/save/reopen/PDF; full 142/24 gates | Rewrite old file.rs hunks into current modules; preserve upstream file-access, font-loading and atomic save behavior. No fork version/links/printing changes. |
| 8. Desktop/web presentation | Upstream app/files.rs, app/file_dialogs.rs, menu run modules; desktop open filters; browser DOC_EXTS; headless UI tests | Small–medium: 200–400 lines | 7 | Headless notice/notes/save tests; browser byte-path tests; wasm; separately request native picker testing | Adapt asynchronous dialogs instead of replacing them. Add missing browser extension explicitly. Keep upstream branding. |
| 9. Optional font substitution | Selected generic logic from fonts/src/fontdb.rs, kind.rs, substitute.rs and their tests; optional font-repository contribution handled separately | Medium: 300–600 lines, excluding font binaries | Independent review; optional after 7 | Font type/width/weight fallback tests without required external fonts; repeat corpus and pixels | Remove the four unconditional include_bytes entries. Neutralize fork-specific derived-family naming only with proper font licence/renaming review. No TTF files in the application PR. |
| 10. Optional folder conversion | Selected CLI batch.rs/folder.rs/main.rs and tests, adapted to upstream tests/it; folder-convert.md | Large as one patch: about 1,300–1,600 lines; split worker/execution and reporting into two reviews | 7 plus command-architecture decision | Mixed good/refused/damaged folder, overwrite/link/name protections, empty/oversized inputs, both reports and exit codes | Not ready as-is under the literal command rule. Design engine command coverage or obtain a maintainer exception; then split into roughly 500–800-line execution and report/test reviews. No workflow changes. |


INFERENCE: add a separate optional regression-data review after the applicable code units. It would select crates/pub/tests/corpus.rs, crates/engine/tests/pub_corpus.rs, apps/designcraft-cli/tests/pub_pixel_diff.rs and public fixture metadata from group (e), plus a host-neutral corpus recipe. The current restore script hard-codes this fork's workspace/data layout and is not ready upstream. Metadata/recipe and pixel reporting should be separate roughly 500–800-line reviews rather than one large infrastructure PR. Test through explicitly configured external corpora, verify hashes, retain scoring-only separation and leave all workflows untouched. Main risks are redistribution assumptions, environment-specific fonts, and treating sparse pixel scores as fidelity guarantees.

OBSERVATION: reusable source evidence includes the 109-line bounded read helper, 370-line drawing reader, 527-line text reader, 182-line publication-record reader and their tests. Reuse of their implementation does not require changing their format interpretation. The combined crate still requires workspace registration, coupling extraction if staged, and verification against the new upstream APIs. No complete candidate has been shown to build there as-is.

INFERENCE: no UI, font substitution or batch-conversion change is needed just to review the bounded importer. Conversely, the full standalone crates/pub candidate is already 22 paths and +6,366 text lines, including two binary fixtures; adding the pixel harness brings (a) to 23 paths and +6,907 lines. If maintainers reject a staged parser API, that single larger crate review is the honest alternative. Calling it a small PR would be misleading.

UNKNOWN: maintainer preference for staged decoder APIs, acceptable review sizes, CLI command exceptions, font-repository changes or new dependency acceptance. None was requested or approved upstream during this audit.

## Licence and provenance findings

OBSERVATION: both current project manifests declare MIT OR Apache-2.0 and both trees contain LICENSE-MIT and LICENSE-APACHE. Apache-2.0 is an offered licence, not either project's sole declared licence. The new importer manifest inherits the workspace licence; its source files carry matching SPDX notices. Preserve upstream and contributor notices rather than replacing authorship with a project brand.

OBSERVATION: local dependency metadata declares cfb 0.15.0 as MIT, miniz_oxide 0.8.9 as MIT OR Zlib OR Apache-2.0, uuid as Apache-2.0 OR MIT, and web-time as MIT OR Apache-2.0. The latter two are in the newly relevant dependency graph, not independent Publisher decoders. No conflicting licence declaration was identified in the reviewed importer/glue subset. This is a declaration check, not a legal opinion or exhaustive supply-chain audit.

OBSERVATION: the four added font binaries use the OFL licence recorded in ASSETS.md and the two bundled licence files. Their code and font licences must remain distinct. The policy objection is their inclusion in the application repository, not a finding that OFL necessarily conflicts with Apache-2.0. The derived serif's naming/redistribution conditions need review before moving it.

OBSERVATION: groups (c)/(d) contain more than source code: demo artwork, converted publications, PDFs, screenshots and branding. ASSETS.md describes owner-reserved project-use marks, project-specific artwork rights, and controlled-fixture content whose rights were not independently verified. A statement that all of (a)–(d) is cleared for upstream redistribution cannot be made. Exclude these materials unless their particular rights and upstream suitability are established.

OBSERVATION: the clean-room note says the original three protected readers were removed and rewritten. The source record names the permitted public specifications, controlled/Apache data and scoring-only references and says other Publisher readers were not used. Geometry is documented as a local paraphrase/implementation of permitted standard definitions; no specification XML or vendor artwork is bundled in the importer crate. The two committed binary fixtures are documented as original synthetic data.

INFERENCE: the selected code and provenance are consistent with an Apache-compatible clean-room contribution. Existing upstream Latin font assets and third-party backend resources are pre-existing upstream material, not new assets to claim as original importer work. Nothing in the reviewed code identifies copied code from another Publisher reader.

UNKNOWN: an exhaustive negative claim that nothing copied from Microsoft, Adobe or another reader exists anywhere in all historical work cannot be proved by this audit. Microsoft-template-derived or sample content can appear in fork demonstration artifacts, so do not include them. Publicly accessible test data is not automatically cleared for redistribution. Ask for provenance review of the selected source/docs and generated fixtures before an upstream proposal; no upstream action is taken here.

## Validation, boundaries and remaining unknowns


OBSERVATION: origin/main advanced during the audit to include only the private-reference rule, clean-room clarification and seven-row metadata list (three paths, eighteen added lines). The inventory was refreshed to that main revision; importer/glue counts and the 44-path application trial are unchanged. No private publication data was obtained. The notes branch was rebased onto the refreshed main. Previously pushed draft/final notes commits were then retained as a merge parent so the final push can be a normal fast-forward without rewriting published history; this is local notes-branch history, not an upstream merge.


OBSERVATION: the restored pinned corpus is used only for the unchanged baseline/regression gates, never to infer storage in this audit. The seven private reference files are neither fetched nor used. Baseline and final fork corpus results match: 142/142 imports, 152/152 objects, 111/111 scored positions, 52/52 texts and 111/111 styles; pictures remain 12/41 matched passes and 29 unmatched. Baseline, final and post-rebase runs pass all 142 save/reopen/edit/PDF cases and retain real-batch counts 9/10/2/3. Formatting, explicit strict clippy and all three seven-step CI runs pass: 3,112 tests passed, zero failed and 27 ignored. Neither CI run failed to launch a test binary or failed on a temporary file, so no sequential retry was needed. These are fork checks, not checks of a resolved upstream port.

UNKNOWN: a conflict-resolved current-upstream build, current-upstream corpus numbers, Windows/native picker behavior, font equivalence after moving optional faces, legal clearance for the complete historical/demo tree, and maintainer acceptance. The offline dependency failure is a local cache limitation, not evidence that upstream itself fails to build.

OBSERVATION: all scratch evidence is under target/ag-review. No project code was changed, no conflicts were repaired, and no upstream workflow or public endpoint was invoked beyond fetching the repository. The temporary upstream remote and scratch worktree have been removed after preserving the patch, unresolved diff and logs. Only the origin remote remains; no upstream push was made. The path appendix below is the complete frozen inventory; no markdown table contains a blank line.

## Complete path inventory

OBSERVATION: A/M/D mean added, modified and deleted relative to the adopted base. “binary” replaces textual counts. The byte column is current fork file size, or zero for a deleted path; it is not an added-byte measurement.


### (a) Importer and importer-only measurement

| Path | Status | Text + | Text - | Current bytes |
| --- | --- | --- | --- | --- |
| `apps/designcraft-cli/tests/pub_pixel_diff.rs` | A | 541 | 0 | 24411 |
| `crates/pub/Cargo.toml` | A | 23 | 0 | 642 |
| `crates/pub/src/contents.rs` | A | 182 | 0 | 7267 |
| `crates/pub/src/escher.rs` | A | 370 | 0 | 13219 |
| `crates/pub/src/lib.rs` | A | 1441 | 0 | 71341 |
| `crates/pub/src/metafile.rs` | A | 962 | 0 | 36707 |
| `crates/pub/src/overset.rs` | A | 112 | 0 | 4724 |
| `crates/pub/src/paths.rs` | A | 221 | 0 | 9247 |
| `crates/pub/src/presets.rs` | A | 146 | 0 | 5713 |
| `crates/pub/src/quill.rs` | A | 527 | 0 | 22336 |
| `crates/pub/src/read.rs` | A | 109 | 0 | 3979 |
| `crates/pub/src/tables.rs` | A | 609 | 0 | 28478 |
| `crates/pub/src/wordart.rs` | A | 219 | 0 | 9398 |
| `crates/pub/tests/corpus.rs` | A | 533 | 0 | 24534 |
| `crates/pub/tests/curves.rs` | A | 33 | 0 | 1992 |
| `crates/pub/tests/data/hello-nifty.pub` | A | binary | binary | 7168 |
| `crates/pub/tests/data/wmf-picture.pub` | A | binary | binary | 7168 |
| `crates/pub/tests/import.rs` | A | 271 | 0 | 12404 |
| `crates/pub/tests/overset.rs` | A | 41 | 0 | 2196 |
| `crates/pub/tests/paragraphs.rs` | A | 385 | 0 | 19166 |
| `crates/pub/tests/presets.rs` | A | 42 | 0 | 2235 |
| `crates/pub/tests/tables.rs` | A | 64 | 0 | 3509 |
| `crates/pub/tests/wordart.rs` | A | 76 | 0 | 4767 |

### (b) Open/import integration and CLI

| Path | Status | Text + | Text - | Current bytes |
| --- | --- | --- | --- | --- |
| `Cargo.lock` | M | 105 | 21 | 150642 |
| `Cargo.toml` | M | 4 | 1 | 5723 |
| `apps/designcraft-cli/Cargo.toml` | M | 2 | 0 | 770 |
| `apps/designcraft-cli/src/batch.rs` | A | 498 | 0 | 22160 |
| `apps/designcraft-cli/src/folder.rs` | A | 486 | 0 | 22234 |
| `apps/designcraft-cli/src/main.rs` | M | 28 | 11 | 28087 |
| `apps/designcraft-cli/tests/batch.rs` | A | 138 | 0 | 7343 |
| `apps/designcraft-cli/tests/cli.rs` | M | 7 | 0 | 23208 |
| `apps/designcraft-cli/tests/folder.rs` | A | 273 | 0 | 13255 |
| `apps/designcraft/src/main.rs` | M | 37 | 10 | 39034 |
| `crates/engine/Cargo.toml` | M | 5 | 0 | 1914 |
| `crates/engine/src/cmd/file.rs` | M | 68 | 9 | 111770 |
| `crates/engine/src/cmd/interchange.rs` | M | 48 | 3 | 30450 |
| `crates/engine/src/lib.rs` | M | 14 | 0 | 32293 |
| `crates/engine/tests/pub_corpus.rs` | A | 64 | 0 | 3183 |
| `crates/engine/tests/pub_tables.rs` | A | 91 | 0 | 5211 |
| `crates/engine/tests/pub_workflow.rs` | A | 80 | 0 | 5063 |
| `crates/engine/tests/web_pub.rs` | A | 24 | 0 | 1382 |
| `crates/ui-egui/src/lib.rs` | M | 107 | 3 | 159935 |
| `crates/ui-egui/src/menus.rs` | M | 35 | 36 | 281927 |
| `xtask/src/layers.rs` | M | 1 | 0 | 12999 |

### (c) Added fonts and demo assets

| Path | Status | Text + | Text - | Current bytes |
| --- | --- | --- | --- | --- |
| `assets/demo/NOTES.txt` | A | 10 | 0 | 1800 |
| `assets/demo/bakery-background.png` | A | binary | binary | 1432767 |
| `assets/demo/coffee-cup.png` | A | binary | binary | 1140797 |
| `assets/demo/croissant.png` | A | binary | binary | 2069069 |
| `assets/fonts/BarlowCondensed-Bold.ttf` | A | binary | binary | 109912 |
| `assets/fonts/BarlowCondensed-Regular.ttf` | A | binary | binary | 102480 |
| `assets/fonts/NiftyNarrowSerif-Bold.ttf` | A | binary | binary | 270352 |
| `assets/fonts/NiftyNarrowSerif-Regular.ttf` | A | binary | binary | 267852 |
| `assets/fonts/OFL-BarlowCondensed.txt` | A | 93 | 0 | 4377 |
| `assets/fonts/OFL-RobotoSerif.txt` | A | 93 | 0 | 4489 |

### (d) Documentation and notes

| Path | Status | Text + | Text - | Current bytes |
| --- | --- | --- | --- | --- |
| `AGENTS.md` | M | 2 | 1 | 8964 |
| `ASSETS.md` | M | 142 | 20 | 50076 |
| `CLAUDE.md` | M | 43 | 0 | 15139 |
| `CONTRIBUTING.md` | A | 30 | 0 | 2707 |
| `README.md` | M | 134 | 334 | 15286 |
| `ROADMAP.md` | M | 11 | 31 | 129311 |
| `SECURITY.md` | A | 31 | 0 | 2025 |
| `docs/ALPHA-NOTES.md` | A | 61 | 0 | 6518 |
| `docs/CLEANROOM.md` | A | 12 | 0 | 1308 |
| `docs/CONTROLLED-SAMPLES.md` | A | 121 | 0 | 7619 |
| `docs/KNOWN-LIMITATIONS.md` | A | 95 | 0 | 10244 |
| `docs/PUB-AGENTS.md` | A | 66 | 0 | 9611 |
| `docs/RELEASE-NOTES.md` | A | 56 | 0 | 5836 |
| `docs/SAMPLE-REQUEST.md` | A | 21 | 0 | 2086 |
| `docs/TESTER-INSTALL.md` | A | 30 | 0 | 2370 |
| `docs/TRIAGE.md` | A | 28 | 0 | 2496 |
| `docs/WEB-TEST.md` | A | 40 | 0 | 4326 |
| `docs/agents.md` | M | 1 | 10 | 5571 |
| `docs/architecture.md` | M | 0 | 2 | 8797 |
| `docs/control-protocol.md` | M | 1 | 1 | 9006 |
| `docs/demo/alpha4/alpha4-flyer.designcraft` | A | binary | binary | 804073 |
| `docs/demo/alpha4/alpha4-flyer.pdf` | A | binary | binary | 932181 |
| `docs/demo/alpha4/alpha4-flyer.png` | A | binary | binary | 378717 |
| `docs/demo/alpha4/alpha4-sample2-copy.designcraft` | A | binary | binary | 19153 |
| `docs/demo/alpha4/app-arch-wordart.png` | A | binary | binary | 457451 |
| `docs/demo/alpha4/app-curve.png` | A | binary | binary | 429400 |
| `docs/demo/alpha4/app-flyer.png` | A | binary | binary | 840310 |
| `docs/demo/alpha4/app-heart.png` | A | binary | binary | 431864 |
| `docs/demo/alpha4/app-polyline.png` | A | binary | binary | 433884 |
| `docs/demo/alpha4/app-sample2.png` | A | binary | binary | 520943 |
| `docs/demo/coffee-shop/01-original.png` | A | binary | binary | 390068 |
| `docs/demo/coffee-shop/01b-original-page.png` | A | binary | binary | 375302 |
| `docs/demo/coffee-shop/02-background.png` | A | binary | binary | 1234856 |
| `docs/demo/coffee-shop/03-headline.png` | A | binary | binary | 1264896 |
| `docs/demo/coffee-shop/04-images-placed.png` | A | binary | binary | 1425415 |
| `docs/demo/coffee-shop/05-special-box.png` | A | binary | binary | 1333930 |
| `docs/demo/coffee-shop/06-final-flyer.png` | A | binary | binary | 1347976 |
| `docs/demo/coffee-shop/07-business-card.png` | A | binary | binary | 522494 |
| `docs/demo/coffee-shop/08-same-session-reopened.png` | A | binary | binary | 1363699 |
| `docs/demo/coffee-shop/09-fresh-session-reopened.png` | A | binary | binary | 1355190 |
| `docs/demo/coffee-shop/README.md` | A | 164 | 0 | 10198 |
| `docs/demo/coffee-shop/coffee-shop.designcraft` | A | binary | binary | 4753702 |
| `docs/demo/coffee-shop/coffee-shop.pdf` | A | binary | binary | 5998491 |
| `docs/demo/coffee-shop/engine-values.json` | A | 754 | 0 | 14627 |
| `docs/demo/coffee-shop/pdf-page-1.png` | A | binary | binary | 1043308 |
| `docs/demo/coffee-shop/pdf-page-2.png` | A | binary | binary | 27991 |
| `docs/demo/coffee-shop/reproduce.py` | A | 181 | 0 | 9050 |
| `docs/demo/coffee-shop/verify.py` | A | 133 | 0 | 6942 |
| `docs/file-format-parity.md` | M | 3 | 2 | 11457 |
| `docs/folder-convert.md` | A | 40 | 0 | 4470 |
| `docs/gallery/README.md` | A | 71 | 0 | 4585 |
| `docs/gallery/bullets-page-1.png` | A | binary | binary | 13957 |
| `docs/gallery/hello-nifty-page-1.png` | A | binary | binary | 14684 |
| `docs/gallery/hello-nifty-pdf-page-1.png` | A | binary | binary | 5104 |
| `docs/gallery/hello-nifty.pdf` | A | binary | binary | 6025 |
| `docs/gallery/shadow-page-1.png` | A | binary | binary | 11531 |
| `docs/gallery/table-2x2-page-1.png` | A | binary | binary | 12795 |
| `docs/gallery/wmf-picture-page-1.png` | A | binary | binary | 22960 |
| `docs/gaps.md` | M | 4 | 4 | 16550 |
| `docs/mcp.md` | M | 6 | 51 | 13823 |
| `docs/open-publisher-files.md` | A | 83 | 0 | 5152 |
| `docs/pdf-backend.md` | M | 5 | 35 | 2367 |
| `docs/pub-format/README.md` | A | 100 | 0 | 7908 |
| `docs/pub-format/SOURCES.md` | A | 1373 | 0 | 147619 |
| `docs/pub-format/anchor-coordinates.md` | A | 118 | 0 | 18246 |
| `docs/pub-format/contents-stream.md` | A | 141 | 0 | 10641 |
| `docs/pub-format/drawing-details.md` | A | 78 | 0 | 5731 |
| `docs/pub-format/font-fallback.md` | A | 192 | 0 | 10645 |
| `docs/pub-format/font-substitution-widths.md` | A | 140 | 0 | 20846 |
| `docs/pub-format/headline-clipping.md` | A | 455 | 0 | 25611 |
| `docs/pub-format/lists-and-tabs.md` | A | 137 | 0 | 18611 |
| `docs/pub-format/odraw-geometry.md` | A | 53 | 0 | 11158 |
| `docs/pub-format/picture-gaps.md` | A | 378 | 0 | 54039 |
| `docs/pub-format/picture-positions.md` | A | 136 | 0 | 16972 |
| `docs/pub-format/pixel-diff.md` | A | 75 | 0 | 6723 |
| `docs/pub-format/position-misses.md` | A | 115 | 0 | 15146 |
| `docs/pub-format/preset-geometry.md` | A | 54 | 0 | 9287 |
| `docs/pub-format/property-lists.md` | A | 42 | 0 | 3228 |
| `docs/pub-format/publisher-97-2000-observations.md` | A | 202 | 0 | 17525 |
| `docs/pub-format/publisher-97-2000.md` | A | 311 | 0 | 34530 |
| `docs/pub-format/spot-checks.md` | A | 120 | 0 | 7040 |
| `docs/pub-format/table-formatting.md` | A | 146 | 0 | 18567 |
| `docs/pub-format/table-review.md` | A | 98 | 0 | 8914 |
| `docs/pub-format/tables.md` | A | 422 | 0 | 25850 |
| `docs/pub-format/text-fit.md` | A | 219 | 0 | 26234 |
| `docs/pub-format/text-store.md` | A | 333 | 0 | 21771 |
| `docs/pub-format/unknown-fields.md` | A | 259 | 0 | 21147 |
| `docs/pub-format/wordart.md` | A | 139 | 0 | 19612 |
| `docs/pub-import-status.md` | A | 133 | 0 | 13860 |
| `docs/publisher-parity-checklist.md` | A | 199 | 0 | 18766 |
| `docs/real-file-triage.md` | A | 376 | 0 | 32477 |
| `docs/screenshots/01-start.png` | A | binary | binary | 324470 |
| `docs/screenshots/02-hello-editor.png` | A | binary | binary | 283239 |
| `docs/screenshots/03-wmf-editor.png` | A | binary | binary | 302987 |
| `docs/screenshots/04-edited-text.png` | A | binary | binary | 292263 |
| `docs/screenshots/05-file-menu.png` | A | binary | binary | 378833 |
| `docs/screenshots/06-export-pdf-dialog.png` | A | binary | binary | 342330 |
| `docs/screenshots/07-edited-pdf-page.png` | A | binary | binary | 10988 |
| `docs/screenshots/README.md` | A | 83 | 0 | 6685 |
| `docs/screenshots/edited-hello.pdf` | A | binary | binary | 6965 |
| `docs/screenshots/real-file-2/SCAN.md` | A | 132 | 0 | 14443 |
| `docs/screenshots/real-file-2/Sample2.pub/01-open.png` | A | binary | binary | 390314 |
| `docs/screenshots/real-file-2/Sample2.pub/02-moved.png` | A | binary | binary | 417820 |
| `docs/screenshots/real-file-2/Sample2.pub/03-resized.png` | A | binary | binary | 398577 |
| `docs/screenshots/real-file-2/Sample2.pub/04-edited-text.png` | A | binary | binary | 399533 |
| `docs/screenshots/real-file-2/Sample2.pub/05-same-session-reopened.png` | A | binary | binary | 390065 |
| `docs/screenshots/real-file-2/Sample2.pub/06-fresh-session-reopened.png` | A | binary | binary | 382448 |
| `docs/screenshots/real-file-2/Sample2.pub/07-exported-pdf-page-1.png` | A | binary | binary | 35929 |
| `docs/screenshots/real-file-2/Sample2.pub/README.md` | A | 113 | 0 | 7203 |
| `docs/screenshots/real-file-2/Sample2.pub/editing.jsonl` | A | 27 | 0 | 2856 |
| `docs/screenshots/real-file-2/Sample2.pub/fresh-reopen.jsonl` | A | 10 | 0 | 1066 |
| `docs/screenshots/real-file-2/Sample2.pub/verify.py` | A | 49 | 0 | 2202 |
| `docs/screenshots/scripts/editor.jsonl` | A | 14 | 0 | 1118 |
| `docs/screenshots/scripts/save-reopen.jsonl` | A | 7 | 0 | 747 |
| `docs/screenshots/scripts/start.jsonl` | A | 2 | 0 | 56 |
| `docs/screenshots/scripts/wmf.jsonl` | A | 4 | 0 | 315 |
| `docs/screenshots/windows-card-editor.png` | A | binary | binary | 460522 |
| `docs/screenshots/windows-card-page.png` | A | binary | binary | 37326 |
| `docs/screenshots/windows-flyer-editor.png` | A | binary | binary | 673160 |
| `docs/screenshots/windows-flyer-page.png` | A | binary | binary | 670889 |
| `docs/screenshots/windows-reopened-after-restart.png` | A | binary | binary | 766754 |
| `docs/screenshots/windows-sample2-opened.png` | A | binary | binary | 228665 |
| `docs/target-app-parity.md` | M | 6 | 7 | 90291 |
| `docs/test-data.md` | A | 68 | 0 | 8021 |
| `docs/typography-parity.md` | M | 2 | 2 | 8762 |
| `docs/ui-parity.md` | M | 1 | 1 | 10968 |
| `docs/warning-triage.md` | A | 101 | 0 | 6850 |
| `docs/windows-desktop-demo.md` | A | 83 | 0 | 5617 |

### (e) Packaging, release and test-data infrastructure

| Path | Status | Text + | Text - | Current bytes |
| --- | --- | --- | --- | --- |
| `.coderabbit.yaml` | D | 0 | 6 | 0 |
| `.gitattributes` | A | 6 | 0 | 346 |
| `.github/ISSUE_TEMPLATE/bug_report.yml` | A | 39 | 0 | 1428 |
| `.github/ISSUE_TEMPLATE/config.yml` | A | 6 | 0 | 344 |
| `.github/ISSUE_TEMPLATE/file_failed_to_open.yml` | A | 43 | 0 | 1628 |
| `.github/dependabot.yml` | D | 0 | 30 | 0 |
| `.github/scripts/test-pub-cli.py` | A | 132 | 0 | 6888 |
| `.github/test-data/private-reference-files.csv` | A | 8 | 0 | 1945 |
| `.github/test-data/publisher-fixtures.csv` | A | 167 | 0 | 17018 |
| `.github/workflows/alpha-windows.yml` | A | 76 | 0 | 3409 |
| `.github/workflows/ci.yml` | D | 0 | 67 | 0 |
| `.github/workflows/freebsd.yml` | D | 0 | 29 | 0 |
| `.github/workflows/nix.yml` | M | 13 | 3 | 1063 |
| `.github/workflows/sonarcloud.yml` | D | 0 | 20 | 0 |
| `.github/workflows/tester-cli.yml` | A | 68 | 0 | 2352 |
| `.github/workflows/tester-web.yml` | A | 51 | 0 | 2001 |
| `.github/workflows/windows-alpha-release.yml` | A | 131 | 0 | 5866 |
| `.github/workflows/windows-arm64.yml` | D | 0 | 35 | 0 |
| `.github/workflows/windows-desktop-demo.yml` | A | 130 | 0 | 5456 |
| `.gitignore` | M | 7 | 0 | 250 |
| `packaging/alpha_windows.py` | A | 123 | 0 | 5382 |
| `packaging/check-branding.py` | A | 76 | 0 | 3636 |
| `packaging/env.sh` | M | 1 | 0 | 2703 |
| `packaging/freebsd/package.sh` | D | 0 | 103 | 0 |
| `packaging/icons.sh` | M | 3 | 56 | 244 |
| `packaging/linux/ai.storyteller.designcraft.desktop` | M | 1 | 1 | 472 |
| `packaging/linux/ai.storyteller.designcraft.metainfo.xml.in` | M | 7 | 13 | 2020 |
| `packaging/linux/ai.storyteller.designcraft.mime.xml` | M | 1 | 1 | 773 |
| `packaging/linux/nfpm.yaml` | M | 2 | 2 | 3438 |
| `packaging/linux/package.sh` | M | 5 | 5 | 9667 |
| `packaging/macos/Info.plist.in` | M | 11 | 11 | 4407 |
| `packaging/macos/package.sh` | M | 16 | 16 | 8416 |
| `packaging/nifty-icons.py` | A | 22 | 0 | 1375 |
| `packaging/prepare_condensed_serif.py` | A | 42 | 0 | 1985 |
| `packaging/stage_windows_desktop_evidence.ps1` | A | 68 | 0 | 3552 |
| `packaging/test_alpha_windows.py` | A | 79 | 0 | 4735 |
| `packaging/test_branding.py` | A | 53 | 0 | 2415 |
| `packaging/test_windows_alpha_release.py` | A | 100 | 0 | 6388 |
| `packaging/test_windows_contract.py` | A | 41 | 0 | 2279 |
| `packaging/test_windows_desktop_demo.py` | A | 466 | 0 | 28487 |
| `packaging/web/README.md` | M | 4 | 4 | 4271 |
| `packaging/windows/README-ALPHA.txt` | A | 89 | 0 | 5479 |
| `packaging/windows/designcraft.wxs` | M | 35 | 17 | 7005 |
| `packaging/windows/package.ps1` | M | 14 | 12 | 7771 |
| `packaging/windows_alpha_release.py` | A | 103 | 0 | 4274 |
| `packaging/windows_desktop_demo.py` | A | 712 | 0 | 38766 |
| `scripts/niftydata.manifest` | A | 318 | 0 | 79871 |
| `scripts/restore-niftydata.sh` | A | 152 | 0 | 8878 |
| `sonar-project.properties` | D | 0 | 2 | 0 |

### (f) Identity-only changes: exclude upstream

| Path | Status | Text + | Text - | Current bytes |
| --- | --- | --- | --- | --- |
| `NOTICE` | M | 10 | 7 | 9504 |
| `apps/designcraft-cli/src/perf.rs` | M | 1 | 1 | 23113 |
| `apps/designcraft-web/index.html` | M | 4 | 2 | 1009 |
| `apps/designcraft-web/src/web.rs` | M | 4 | 4 | 21485 |
| `apps/designcraft/Cargo.toml` | M | 1 | 0 | 2471 |
| `apps/designcraft/build.rs` | M | 4 | 3 | 1605 |
| `apps/designcraft/src/native_menu.rs` | M | 2 | 2 | 11537 |
| `assets/app-icon/README.md` | M | 5 | 49 | 338 |
| `assets/app-icon/designcraft-1024.png` | M | binary | binary | 605267 |
| `assets/app-icon/designcraft-macos-512.png` | M | binary | binary | 168221 |
| `assets/app-icon/designcraft-small.svg` | M | 1 | 1 | 807160 |
| `assets/app-icon/designcraft.icns` | M | binary | binary | 1036345 |
| `assets/app-icon/designcraft.ico` | M | binary | binary | 80503 |
| `assets/app-icon/designcraft.svg` | M | 1 | 1 | 807160 |
| `assets/app-icon/hicolor/128x128/apps/ai.storyteller.designcraft.png` | M | binary | binary | 16742 |
| `assets/app-icon/hicolor/16x16/apps/ai.storyteller.designcraft.png` | M | binary | binary | 742 |
| `assets/app-icon/hicolor/24x24/apps/ai.storyteller.designcraft.png` | M | binary | binary | 1341 |
| `assets/app-icon/hicolor/256x256/apps/ai.storyteller.designcraft.png` | M | binary | binary | 49823 |
| `assets/app-icon/hicolor/32x32/apps/ai.storyteller.designcraft.png` | M | binary | binary | 2058 |
| `assets/app-icon/hicolor/48x48/apps/ai.storyteller.designcraft.png` | M | binary | binary | 3810 |
| `assets/app-icon/hicolor/512x512/apps/ai.storyteller.designcraft.png` | M | binary | binary | 168221 |
| `assets/app-icon/hicolor/64x64/apps/ai.storyteller.designcraft.png` | M | binary | binary | 5869 |
| `assets/app-icon/hicolor/scalable/apps/ai.storyteller.designcraft.svg` | M | 1 | 1 | 807160 |
| `assets/nifty/nifty-mark.png` | A | binary | binary | 1150174 |
| `assets/nifty/nifty-wordmark.png` | A | binary | binary | 752031 |
| `contributors/people.toml` | M | 0 | 3 | 1995 |
| `crates/engine/src/cmd/book.rs` | M | 1 | 1 | 20365 |
| `crates/engine/src/cmd/inspect.rs` | M | 1 | 1 | 6437 |
| `crates/engine/src/links.rs` | M | 8 | 23 | 1060 |
| `crates/engine/src/recovery.rs` | M | 1 | 1 | 79501 |
| `crates/epub/src/lib.rs` | M | 2 | 2 | 108042 |
| `crates/mcp/src/backend.rs` | M | 4 | 4 | 12163 |
| `crates/mcp/src/lib.rs` | M | 2 | 2 | 1577 |
| `crates/mcp/src/server.rs` | M | 3 | 5 | 10088 |
| `crates/mcp/src/tools.rs` | M | 2 | 2 | 31474 |
| `crates/pdf/src/export.rs` | M | 5 | 5 | 94403 |
| `crates/pptx/src/package.rs` | M | 5 | 5 | 17596 |
| `crates/ui-egui/src/about.rs` | M | 23 | 177 | 4415 |
| `crates/ui-egui/src/chrome.rs` | M | 7 | 48 | 85526 |
| `crates/ui-egui/src/i18n.rs` | M | 10 | 11 | 305235 |
| `crates/ui-egui/src/i18n/ar.rs` | M | 8 | 8 | 111561 |
| `crates/ui-egui/src/i18n/cs.rs` | M | 9 | 15 | 90897 |
| `crates/ui-egui/src/i18n/de.rs` | M | 8 | 8 | 100088 |
| `crates/ui-egui/src/i18n/fr.rs` | M | 3 | 9 | 22955 |
| `crates/ui-egui/src/i18n/it.rs` | M | 6 | 6 | 87700 |
| `crates/ui-egui/src/i18n/ja.rs` | M | 8 | 8 | 110664 |
| `crates/ui-egui/src/i18n/ka.rs` | M | 8 | 8 | 150288 |
| `crates/ui-egui/src/i18n/pl.rs` | M | 8 | 8 | 81398 |
| `crates/ui-egui/src/i18n/pt_br.rs` | M | 6 | 6 | 84940 |
| `crates/ui-egui/src/i18n/ru.rs` | M | 8 | 8 | 109751 |
| `crates/ui-egui/src/i18n/uk.rs` | M | 6 | 6 | 114063 |
| `docs/brand/artcraft-logo-white.png` | D | binary | binary | 0 |
| `docs/brand/artcraft-logo-white.svg` | D | 0 | 7 | 0 |
| `docs/brand/artcraft-logo.png` | D | binary | binary | 0 |
| `docs/brand/artcraft-logo.svg` | D | 0 | 7 | 0 |
| `docs/brand/artcraft-mark-black.png` | D | binary | binary | 0 |
| `docs/brand/artcraft-mark-black.svg` | D | 0 | 4 | 0 |
| `docs/brand/artcraft-mark.png` | D | binary | binary | 0 |
| `docs/brand/artcraft-mark.svg` | D | 0 | 4 | 0 |

### (g) Other shared changes and examples

| Path | Status | Text + | Text - | Current bytes |
| --- | --- | --- | --- | --- |
| `crates/engine/src/cmd/edit.rs` | M | 4 | 4 | 65359 |
| `crates/engine/src/cmd/export.rs` | M | 8 | 2 | 69767 |
| `crates/engine/src/cmd/file/eps_place_tests.rs` | M | 1 | 0 | 11252 |
| `crates/engine/src/cmd/package.rs` | M | 4 | 4 | 48411 |
| `crates/engine/src/cmd/print.rs` | M | 129 | 16 | 10715 |
| `crates/engine/src/cmd/print/native.rs` | A | 99 | 0 | 4750 |
| `crates/engine/src/tests_linked_paths.rs` | M | 1 | 0 | 4761 |
| `crates/fonts/src/fontdb.rs` | M | 89 | 11 | 104640 |
| `crates/fonts/src/kind.rs` | M | 3 | 1 | 7756 |
| `crates/fonts/src/substitute.rs` | A | 352 | 0 | 16864 |
| `crates/ui-egui/examples/ui_shot.rs` | M | 33 | 3 | 6643 |
| `crates/ui-egui/src/control.rs` | M | 1 | 0 | 42427 |
| `crates/ui-egui/src/dialogs.rs` | M | 6 | 7 | 527478 |
| `crates/ui-egui/src/parent_canvas_tests.rs` | M | 35 | 0 | 7399 |
| `examples/csv-merge/README.md` | A | 59 | 0 | 4759 |
| `examples/csv-merge/csv-to-labels.ps1` | A | 56 | 0 | 2507 |
| `examples/csv-merge/interventions.csv` | A | 6 | 0 | 200 |
| `examples/csv-merge/people.csv` | A | 4 | 0 | 56 |
| `examples/csv-merge/streets-and-interventions.ps1` | A | 85 | 0 | 4510 |
| `examples/csv-merge/streets.csv` | A | 3 | 0 | 64 |
| `xtask/src/assets.rs` | M | 1 | 0 | 2689 |
