# Upstream merge trial — 2026-10-31

OBSERVATION: this is an incomplete trial checkpoint, not an adopted upstream release. The branch proposes version 0.15.2-nifty-alpha.1 and adds Publisher to the browser document picker. Its production code otherwise remains the existing 0.12-derived fork. A real three-way tree application was attempted in an ignored detached worktree; it is not promoted while unresolved. Nothing is merged into main or pushed to upstream.

INFERENCE: verdict **not yet**. The successful checks on this checkpoint cannot establish that the merged 0.15.2 code preserves Publisher behavior. Resolve and test the complete trial before adoption.

## Versions and scope

OBSERVATION: starting fork main is `ce679520ae636d1a5756485cac95982121ad2f69`, version 0.12.0-nifty-alpha.4. Fetched upstream main is `ea214dd44dbfbfed23ffd79e5efaba375510afb4`, manifest version 0.15.2. The selected target is tag v0.15.2, commit `7e70203206531f0fde1dd5feb2b7d94768842ebc`, also version 0.15.2. Histories have no common ancestor; the comparison base is the previously adopted upstream 0.12.0 commit `5e8ef678321cfdd2d18927e2af94d58e5f9d8836`.

OBSERVATION: base to selected tag has 1,578 reachable commits, 909 non-merge commits and 201 first-parent steps. Across the whole tree the rename-aware diff changes 562 files, with 129,536 additions and 80,758 deletions. The selected code/manifests/test delta has 513 rename-aware paths, or 570 paths with renames disabled. These are alternative counting methods, not counts to add. Fetched upstream main is 1,651 reachable commits beyond the adopted base and 73 beyond the selected tag.

OBSERVATION: no release-notes or changelog file was found by those names in the tag. The following user-visible changes summarize repository commit subjects; these are upstream descriptions, not behavior independently verified here.

- OBSERVATION: Text: discretionary hyphens and French/Italian elisions receive composition fixes.
- OBSERVATION: Text: drop-cap placement follows vertical justification; tables clear footnote space.
- OBSERVATION: Tables: text-to-table conversion rejects excess rows and insertion respects limits.
- OBSERVATION: Paths: cut, delete, erase and smooth handle selected anchor points more consistently.
- OBSERVATION: Colour picker: eyedropper dragging, cancellation and sampling receive fixes.
- OBSERVATION: Effects: outer glows and blended shadows isolate the spread from the paper.
- OBSERVATION: PDF: anchored text frames preserve the host frame text area.
- OBSERVATION: IDML: paragraph overprint attributes and default story preferences are exported.
- OBSERVATION: Accessibility: table-of-contents and printing/package fields gain labels.
- OBSERVATION: Application: update checks honor pretend versions and the update prompt uses app artwork.
- OBSERVATION: Flatpak: launcher version pins and CUPS printing receive fixes; release 0.15.2 adjusts debug stripping.

## Baseline

OBSERVATION: restore verified all 317 pinned input sizes and hashes: skipped 317, restored 0, failed 0. Untouched-main gate: 142/142 imports, 152/152 objects, 111/111 scored positions within 1 pt, 52/52 texts, 111/111 styles. Picture positions: 12/41 matched passes, zero measured misses, 29 unmatched. All 142 save/reopen/edit/PDF cases passed. The exact real batch classification test passed: 9 notes, 10 warnings, 2 refused, 3 damaged. All nine folder-convert tests, including external controlled and real mixed-folder cases, passed.

OBSERVATION: the nine untouched-main pixel scores are below. The final checkpoint comparison is recorded after validation.

| Fixture | Baseline SSIM | Baseline differing fraction |
| --- | ---: | ---: |
| `group__rotated.pub` | 0.999980060 | 0.001000000 |
| `shadow__offset.pub` | 0.999993784 | 0.000983957 |
| `shadow__on.pub` | 0.999994589 | 0.000963636 |
| `wordart__arch.pub` | 0.983892067 | 0.006321925 |
| `wordart__base.pub` | 0.980074652 | 0.006571123 |
| `wordart__circle.pub` | 0.960595137 | 0.009868449 |
| `wordart__fill-red.pub` | 0.979558751 | 0.012936898 |
| `wordart__line-white.pub` | 1.000000000 | 0.000000000 |
| `wordart__shadow.pub` | 0.979426457 | 0.009779679 |

## Three-way application and unresolved files

OBSERVATION: generated binary-capable full-index patches from the adopted base to the selected tag, with renames disabled, and applied them with three-way/index handling to a detached copy of our starting main. This is a tree merge despite unrelated histories. It covers crates/, apps/, xtask/, Cargo.toml and Cargo.lock. There is no upstream crates/pub tree. No protected reader was changed. No external Publisher input or scoring reference, workflow, font binary, icon or environment setting was replaced on the branch. Documentation was excluded because this task permits only this note and a source-use line.

OBSERVATION: 546 of 570 individual path applications succeeded textually. Twenty-two paths contain conflicts and two modified old test files could not be deleted by the upstream patch. Clean application is not compilation or semantic compatibility. No conflicted result was committed, no conflict was declared resolved and no test was removed to bypass it. The table states the remaining resolution work, not changes already completed.

| Path | Result | Remaining work (INFERENCE from overlapping edits) |
| --- | --- | --- |
| `Cargo.lock` | Conflict | Dependency pins and workspace package versions diverged. Regenerate after the manifest and code ports; do not retain conflict markers. |
| `Cargo.toml` | Conflict | Upstream version and dependency/patch revisions overlap the fork version. Keep the proposed fork version and pub/cfb entries with upstream dependencies. |
| `apps/designcraft-cli/src/main.rs` | Conflict | Dispatch, usage and validation overlap batch/folder conversion and identity. Port worker dispatch and argument handling into the new CLI. |
| `apps/designcraft-cli/tests/cli.rs` | Failed deletion | Upstream deletes the modified old test file and adds tests/it/cli.rs. Port every fork assertion into the new harness before removal. |
| `apps/designcraft-web/index.html` | Conflict | Browser shell changes overlap fork title/branding. Preserve fork identity with upstream shell behavior. |
| `apps/designcraft/src/main.rs` | Conflict | Asynchronous native dialogs overlap Publisher filters and save suggestions. Port those changes without reverting asynchronous dialogs. |
| `crates/engine/src/cmd/book.rs` | Conflict | Upstream command changes overlap fork naming. Keep upstream behavior and fork-visible names. |
| `crates/engine/src/cmd/edit.rs` | Conflict | The local delta only renames a clipboard-test variable from marker to placeholder. Upstream rewrites/moves the surrounding tests; preserve assertions while accepting the current test layout. |
| `crates/engine/src/cmd/export.rs` | Conflict | Upstream export changes overlap fork-specific output wording. Preserve upstream export logic and fork identity. |
| `crates/engine/src/cmd/file.rs` | Conflict | Upstream replaces the monolith with file/*.rs. Move pub sniffing to bytes.rs, import routing to open.rs, command descriptions to registry.rs; move all local tests. |
| `crates/engine/src/cmd/package.rs` | Conflict | Upstream package changes overlap fork naming. Keep upstream logic and fork output names. |
| `crates/engine/src/cmd/print.rs` | Conflict | Upstream printing replaces local printing code. Review preserved platform behavior and tests; do not silently discard either test set. |
| `crates/engine/src/tests_linked_paths.rs` | Failed deletion | The local delta adds a copyright/SPDX header; upstream deletes/moves the old file. Locate its replacement, preserve attribution and verify the tests moved before removing the old path. |
| `crates/epub/src/lib.rs` | Conflict | Export metadata/naming overlaps upstream export changes. Keep upstream code and fork identity. |
| `crates/fonts/src/fontdb.rs` | Conflict | Upstream splits font handling while local code adds width/type fallback and embedded condensed families. Port fallback logic and all four font includes into current modules; pixel parity is unproven. |
| `crates/pdf/src/export.rs` | Conflict | Upstream export changes overlap fork producer metadata. Keep upstream drawing behavior and fork identity; measure pixels. |
| `crates/ui-egui/src/about.rs` | Conflict | Upstream About changes overlap fork identity/artwork. Preserve fork branding and attribution. |
| `crates/ui-egui/src/chrome.rs` | Conflict | Upstream chrome changes overlap fork title/menu behavior. Review event handling and keep fork identity. |
| `crates/ui-egui/src/dialogs.rs` | Conflict | Upstream dialog extraction overlaps the local native-print dispatch and cancellation/status handling. Reconcile actual printing behavior and tests, not just wording. |
| `crates/ui-egui/src/i18n.rs` | Conflict | Upstream translation registry overlaps fork/local wording. Preserve relevant entries in the current registry. |
| `crates/ui-egui/src/i18n/cs.rs` | Conflict | Fork identity translations overlap upstream catalog changes. Keep fork names in the current catalog and preserve applicable translations. |
| `crates/ui-egui/src/i18n/ru.rs` | Conflict | Fork identity translations overlap upstream catalog changes. Keep fork names in the current catalog and preserve applicable translations. |
| `crates/ui-egui/src/lib.rs` | Conflict | Upstream extracts app/*.rs. Port import notice, font-dialog suppression, screenshot/status handling and all tests into the new modules. |
| `crates/ui-egui/src/menus.rs` | Conflict | Upstream extracts menus/*.rs. Port save suggestions, fork help/quit entries and all tests without reverting upstream asynchronous save flow. |

OBSERVATION: the previous audit eight-path list was not exhaustive for this newer tag. The full remaining selected path list is in the appendix. All 570 path changes await a resolved, validated integration; the 546 textual successes are not advertised as adopted. The generated per-path patches, application results and unresolved diff/status are retained under ignored target/ai-review for comparison. Detached scratch worktrees were removed after their checks; no broken source was published.

## Build and behavior checks

OBSERVATION: clean upstream v0.15.2 cargo check for the desktop application passed in 2m 25s. This is upstream alone, without our importer. The dependency fetch succeeded; no network workaround or dependency override was used.

OBSERVATION: a second isolated worktree copied crates/pub unchanged into v0.15.2 and added only workspace pub/cfb entries. Its first offline check could not resolve clipboard-win (^5.3.1): the cached index listed 5.4.1 as yanked. The normal online retry resolved dependencies and reached Rust compilation, which failed with three E0063 errors: GradientStop constructors at crates/pub/src/lib.rs:725 and :779 lack swatch, and the Graphic constructor at :881 lacks layer_options. Neither attempt launched a test binary. No protected reader edit is indicated by these errors, and no importer behavior change was made to bypass them.

INFERENCE: the complete merge additionally needs an explicit API-compatibility port of those three constructors in unprotected lib.rs, with new fields chosen from upstream semantics and tested for unchanged gradients and pictures. A textual application of crates/pub is insufficient. This probe used upstream fonts and libraries, not the fully identity-preserving merge, so even a later successful probe would not establish pixel parity.

UNKNOWN: the full merge still has 24 unresolved path operations and the importer API incompatibilities above. No successful merged build, merged corpus, merged pixel parity or merged desktop/browser runtime result is claimed.

OBSERVATION: a real checkpoint CLI script completed four session steps successfully: file.open of the synthetic hello-nifty.pub, document.inspect, file.openBytes of the same bytes, and document.inspect. Both imported copies have one page and no saved source path. This tests command dispatch in the checkpoint, not the unresolved upstream port. Checkpoint-only gate results follow below.

UNKNOWN: Windows desktop, native dialogs, printing on a real printer, real-PC fonts and Publisher’s own fonts were not tested. Browser picker operation is untested by a person; adding its extension only changes the allowed picker list.

## Checkpoint verification

OBSERVATION: this table describes the retained checkpoint, not the unresolved upstream merge.

| Check | Untouched-main evidence | Retained checkpoint |
| --- | --- | --- |
| Imports / objects / positions / texts / styles | 142/142; 152/152; 111/111; 52/52; 111/111 | Identical, including every per-file scored fact, warning and note |
| Picture positions | 12 matched passes; zero measured misses; 29 unmatched of 41 | Identical |
| Save/reopen/edit/PDF | 142 passed | 142 passed |
| Real batch | 9 notes / 10 warnings / 2 refused / 3 damaged | Identical |
| Nine pixel references | Values in baseline table | Every SSIM and differing fraction identical; no worse score |
| Folder conversion | All 9 tests passed, including the external mixed cases | All 9 passed |
| Headless session commands | Existing corpus/workflow coverage | Explicit CLI file.open + file.openBytes script passed; pub_workflow and web_pub passed |
| Desktop cargo check | Not separately run on untouched main | Passed for the retained checkpoint; separately passed for clean upstream 0.15.2 |
| Web application cargo check | Not separately run on untouched main | Explicit wasm32-unknown-unknown designcraft-web check passed |

OBSERVATION: checkpoint fmt and strict workspace/all-target clippy passed. The first checkpoint CI test step failed only alpha_version_identifies_unofficial_build: actual NiftyDesign CLI 0.15.2-nifty-alpha.1, expected NiftyDesign CLI 0.12.0-nifty-alpha.4. Updated that single expected literal to the requested version, retaining exact equality and all other assertions. The sequential full-workspace rerun passed 3,112 tests, zero failures, 27 ignored. No test-binary launch or temporary-file failure occurred. The final full CI run passed all seven steps: fmt, pdf-deps, clippy, test, assets, layers and wasm; 3,112 passed, zero failed, 27 ignored.

INFERENCE: these checks make this limited checkpoint reviewable. They do not establish upstream integration compatibility: the branch has not adopted the 570 selected path operations, and the isolated importer fails against new APIs. Do not merge or release this checkpoint as an upstream upgrade. Complete the conflict/API ports and repeat every gate on the merged candidate, including exact pixel comparison and both application builds.

## Remaining selected path inventory

OBSERVATION: each entry below is a base-to-v0.15.2 path still awaiting complete validated integration on the trial branch. Clean textual application alone does not remove a path from this list.

- `Cargo.lock`
- `Cargo.toml`
- `apps/designcraft-cli/examples/pdf_export.rs`
- `apps/designcraft-cli/src/automation_args.rs`
- `apps/designcraft-cli/src/lib.rs`
- `apps/designcraft-cli/src/main.rs`
- `apps/designcraft-cli/src/perf.rs`
- `apps/designcraft-cli/src/stress.rs`
- `apps/designcraft-cli/tests/cli.rs`
- `apps/designcraft-cli/tests/it/cli.rs`
- `apps/designcraft-cli/tests/it/main.rs`
- `apps/designcraft-web/index.html`
- `apps/designcraft-web/src/web.rs`
- `apps/designcraft/Cargo.toml`
- `apps/designcraft/src/gpu.rs`
- `apps/designcraft/src/gpu/backend.rs`
- `apps/designcraft/src/gpu/tests.rs`
- `apps/designcraft/src/mac_window.rs`
- `apps/designcraft/src/main.rs`
- `apps/designcraft/src/native_menu.rs`
- `crates/color/src/ase.rs`
- `crates/color/src/cms.rs`
- `crates/color/src/cms/mod.rs`
- `crates/color/src/gradient.rs`
- `crates/color/src/swatch.rs`
- `crates/compose/Cargo.toml`
- `crates/compose/src/breaker.rs`
- `crates/compose/src/cache.rs`
- `crates/compose/src/grep.rs`
- `crates/compose/src/hyphen.rs`
- `crates/compose/src/hyphen/mod.rs`
- `crates/compose/src/lib.rs`
- `crates/compose/src/notes.rs`
- `crates/compose/src/overlay.rs`
- `crates/compose/src/parabox.rs`
- `crates/compose/src/placed.rs`
- `crates/compose/src/ruby.rs`
- `crates/compose/src/shape.rs`
- `crates/compose/src/table.rs`
- `crates/compose/src/table/border_tests.rs`
- `crates/compose/src/tests.rs`
- `crates/compose/src/vars.rs`
- `crates/compose/src/warichu.rs`
- `crates/compose/src/wrap.rs`
- `crates/doc/src/anchored.rs`
- `crates/doc/src/assets.rs`
- `crates/doc/src/attrs.rs`
- `crates/doc/src/attrs/char_attrs.rs`
- `crates/doc/src/attrs/enums.rs`
- `crates/doc/src/attrs/language.rs`
- `crates/doc/src/attrs/para_attrs.rs`
- `crates/doc/src/attrs/rules_lists.rs`
- `crates/doc/src/attrs/tabs.rs`
- `crates/doc/src/attrs/tests.rs`
- `crates/doc/src/build.rs`
- `crates/doc/src/edit.rs`
- `crates/doc/src/endnotes.rs`
- `crates/doc/src/file_info.rs`
- `crates/doc/src/index.rs`
- `crates/doc/src/item.rs`
- `crates/doc/src/item/content.rs`
- `crates/doc/src/item/effects.rs`
- `crates/doc/src/item/graphic.rs`
- `crates/doc/src/item/paint.rs`
- `crates/doc/src/item/text_frame.rs`
- `crates/doc/src/lib.rs`
- `crates/doc/src/notes.rs`
- `crates/doc/src/otf.rs`
- `crates/doc/src/story.rs`
- `crates/doc/src/styles.rs`
- `crates/doc/src/swatch_refs.rs`
- `crates/doc/src/table.rs`
- `crates/doc/src/table/cell.rs`
- `crates/doc/src/table/edit.rs`
- `crates/doc/src/table/grid.rs`
- `crates/doc/src/table/merge.rs`
- `crates/doc/src/table/story.rs`
- `crates/engine/Cargo.toml`
- `crates/engine/src/cmd.rs`
- `crates/engine/src/cmd/anchored.rs`
- `crates/engine/src/cmd/anchors.rs`
- `crates/engine/src/cmd/book.rs`
- `crates/engine/src/cmd/datamerge.rs`
- `crates/engine/src/cmd/datamerge/fill.rs`
- `crates/engine/src/cmd/datamerge/mod.rs`
- `crates/engine/src/cmd/datamerge/parse.rs`
- `crates/engine/src/cmd/datamerge/tests.rs`
- `crates/engine/src/cmd/edit.rs`
- `crates/engine/src/cmd/edit/text_clipboard_tests.rs`
- `crates/engine/src/cmd/editorial.rs`
- `crates/engine/src/cmd/endnotes.rs`
- `crates/engine/src/cmd/export.rs`
- `crates/engine/src/cmd/export/flatten.rs`
- `crates/engine/src/cmd/export/formats.rs`
- `crates/engine/src/cmd/export/pdf.rs`
- `crates/engine/src/cmd/export/registry.rs`
- `crates/engine/src/cmd/export/text_tests.rs`
- `crates/engine/src/cmd/file.rs`
- `crates/engine/src/cmd/file/bytes.rs`
- `crates/engine/src/cmd/file/eps_place_tests.rs`
- `crates/engine/src/cmd/file/new.rs`
- `crates/engine/src/cmd/file/open.rs`
- `crates/engine/src/cmd/file/place.rs`
- `crates/engine/src/cmd/file/registry.rs`
- `crates/engine/src/cmd/file/save.rs`
- `crates/engine/src/cmd/file/snippets.rs`
- `crates/engine/src/cmd/file_info.rs`
- `crates/engine/src/cmd/find.rs`
- `crates/engine/src/cmd/find/tests.rs`
- `crates/engine/src/cmd/flex.rs`
- `crates/engine/src/cmd/fonts.rs`
- `crates/engine/src/cmd/index.rs`
- `crates/engine/src/cmd/interactive.rs`
- `crates/engine/src/cmd/interchange.rs`
- `crates/engine/src/cmd/layout.rs`
- `crates/engine/src/cmd/layout/layers.rs`
- `crates/engine/src/cmd/layout/margins.rs`
- `crates/engine/src/cmd/layout/pages.rs`
- `crates/engine/src/cmd/layout/parents.rs`
- `crates/engine/src/cmd/layout/registry.rs`
- `crates/engine/src/cmd/layout/sections.rs`
- `crates/engine/src/cmd/links.rs`
- `crates/engine/src/cmd/links/tests.rs`
- `crates/engine/src/cmd/lists.rs`
- `crates/engine/src/cmd/load_styles.rs`
- `crates/engine/src/cmd/load_styles/tests.rs`
- `crates/engine/src/cmd/mathexpr.rs`
- `crates/engine/src/cmd/media.rs`
- `crates/engine/src/cmd/mod.rs`
- `crates/engine/src/cmd/notes.rs`
- `crates/engine/src/cmd/object.rs`
- `crates/engine/src/cmd/package.rs`
- `crates/engine/src/cmd/package/tests.rs`
- `crates/engine/src/cmd/paste_svg.rs`
- `crates/engine/src/cmd/pdflayers.rs`
- `crates/engine/src/cmd/pdflayers/tests.rs`
- `crates/engine/src/cmd/place_text.rs`
- `crates/engine/src/cmd/place_text/tests.rs`
- `crates/engine/src/cmd/preflight.rs`
- `crates/engine/src/cmd/prefs.rs`
- `crates/engine/src/cmd/prepress.rs`
- `crates/engine/src/cmd/print.rs`
- `crates/engine/src/cmd/spelling.rs`
- `crates/engine/src/cmd/style.rs`
- `crates/engine/src/cmd/style/color.rs`
- `crates/engine/src/cmd/style/color_tests.rs`
- `crates/engine/src/cmd/style/object.rs`
- `crates/engine/src/cmd/style/paragraph.rs`
- `crates/engine/src/cmd/style/swatches.rs`
- `crates/engine/src/cmd/table.rs`
- `crates/engine/src/cmd/table/cell.rs`
- `crates/engine/src/cmd/table/content.rs`
- `crates/engine/src/cmd/table/options.rs`
- `crates/engine/src/cmd/table/registry.rs`
- `crates/engine/src/cmd/table/selection.rs`
- `crates/engine/src/cmd/table/sizing.rs`
- `crates/engine/src/cmd/table/structure.rs`
- `crates/engine/src/cmd/table/styles.rs`
- `crates/engine/src/cmd/table/target.rs`
- `crates/engine/src/cmd/text.rs`
- `crates/engine/src/cmd/text/asian.rs`
- `crates/engine/src/cmd/text/caret.rs`
- `crates/engine/src/cmd/text/edit.rs`
- `crates/engine/src/cmd/text/format.rs`
- `crates/engine/src/cmd/text/opentype.rs`
- `crates/engine/src/cmd/text/story.rs`
- `crates/engine/src/cmd/thread.rs`
- `crates/engine/src/cmd/toc.rs`
- `crates/engine/src/cmd/variables.rs`
- `crates/engine/src/cmd/variables/tests.rs`
- `crates/engine/src/cmd/xml.rs`
- `crates/engine/src/cmd/xref.rs`
- `crates/engine/src/file_access.rs`
- `crates/engine/src/guard.rs`
- `crates/engine/src/io.rs`
- `crates/engine/src/lib.rs`
- `crates/engine/src/math.rs`
- `crates/engine/src/math/tests.rs`
- `crates/engine/src/recovery.rs`
- `crates/engine/src/recovery/tests.rs`
- `crates/engine/src/sample.rs`
- `crates/engine/src/separations.rs`
- `crates/engine/src/tests.rs`
- `crates/engine/src/tests/autosize.rs`
- `crates/engine/src/tests/file_guard.rs`
- `crates/engine/src/tests/idml.rs`
- `crates/engine/src/tests/ime.rs`
- `crates/engine/src/tests/linked_paths.rs`
- `crates/engine/src/tests/params.rs`
- `crates/engine/src/tests/roots.rs`
- `crates/engine/src/tests/table.rs`
- `crates/engine/src/tests_autosize.rs`
- `crates/engine/src/tests_file_guard.rs`
- `crates/engine/src/tests_idml.rs`
- `crates/engine/src/tests_ime.rs`
- `crates/engine/src/tests_linked_paths.rs`
- `crates/engine/src/tests_params.rs`
- `crates/engine/src/tests_roots.rs`
- `crates/engine/src/tests_table.rs`
- `crates/engine/src/tooling.rs`
- `crates/engine/tests/book_safety.rs`
- `crates/engine/tests/browser_print.rs`
- `crates/engine/tests/character_style_from_selection.rs`
- `crates/engine/tests/close_retention.rs`
- `crates/engine/tests/collection_state_atomicity.rs`
- `crates/engine/tests/combined_close_glyph.rs`
- `crates/engine/tests/distribute_locks.rs`
- `crates/engine/tests/export_page_input.rs`
- `crates/engine/tests/graphic_layer_options.rs`
- `crates/engine/tests/insert_pages_bounds.rs`
- `crates/engine/tests/it/book_safety.rs`
- `crates/engine/tests/it/browser_print.rs`
- `crates/engine/tests/it/character_style_from_selection.rs`
- `crates/engine/tests/it/close_retention.rs`
- `crates/engine/tests/it/collection_state_atomicity.rs`
- `crates/engine/tests/it/combined_close_glyph.rs`
- `crates/engine/tests/it/distribute_locks.rs`
- `crates/engine/tests/it/export_page_input.rs`
- `crates/engine/tests/it/insert_pages_bounds.rs`
- `crates/engine/tests/it/main.rs`
- `crates/engine/tests/it/param_keys.rs`
- `crates/engine/tests/it/pdf_backend.rs`
- `crates/engine/tests/it/preflight_resolution.rs`
- `crates/engine/tests/it/primary_frame_columns.rs`
- `crates/engine/tests/it/save_state_atomicity.rs`
- `crates/engine/tests/it/stroke_tint_range.rs`
- `crates/engine/tests/it/text_decorations.rs`
- `crates/engine/tests/it/text_insert_memory.rs`
- `crates/engine/tests/it/text_thread_regressions.rs`
- `crates/engine/tests/it/thread_color_safety.rs`
- `crates/engine/tests/param_keys.rs`
- `crates/engine/tests/pdf_backend.rs`
- `crates/engine/tests/pdf_optional_content.rs`
- `crates/engine/tests/preflight_resolution.rs`
- `crates/engine/tests/primary_frame_columns.rs`
- `crates/engine/tests/save_state_atomicity.rs`
- `crates/engine/tests/stroke_tint_range.rs`
- `crates/engine/tests/support/optional_content.rs`
- `crates/engine/tests/support/optional_content_fallback.rs`
- `crates/engine/tests/support/optional_content_xref.rs`
- `crates/engine/tests/text_decorations.rs`
- `crates/engine/tests/text_insert_memory.rs`
- `crates/engine/tests/text_thread_regressions.rs`
- `crates/engine/tests/thread_color_safety.rs`
- `crates/epub/src/chapters.rs`
- `crates/epub/src/css.rs`
- `crates/epub/src/fonts.rs`
- `crates/epub/src/html.rs`
- `crates/epub/src/lib.rs`
- `crates/epub/src/markup.rs`
- `crates/epub/src/package.rs`
- `crates/epub/src/story_writer.rs`
- `crates/epub/src/tests.rs`
- `crates/epub/tests/export_tag_styles.rs`
- `crates/epub/tests/grouped_reading_order.rs`
- `crates/epub/tests/it/export_tag_styles.rs`
- `crates/epub/tests/it/grouped_reading_order.rs`
- `crates/epub/tests/it/main.rs`
- `crates/fonts/build.rs`
- `crates/fonts/src/fontdb.rs`
- `crates/fonts/src/fontdb/db.rs`
- `crates/fonts/src/fontdb/face.rs`
- `crates/fonts/src/fontdb/fallback.rs`
- `crates/fonts/src/fontdb/names.rs`
- `crates/fonts/src/fontdb/outline.rs`
- `crates/fonts/src/fontdb/scoped.rs`
- `crates/fonts/src/fontdb/system.rs`
- `crates/fonts/src/fontdb/tests.rs`
- `crates/fonts/src/fontdb/tests/docfonts.rs`
- `crates/fonts/src/fontdb/tests/fallback.rs`
- `crates/fonts/src/fontdb/tests/sysfonts.rs`
- `crates/fonts/src/fontdb/tests/vertical.rs`
- `crates/fonts/src/tests_docfonts.rs`
- `crates/fonts/src/tests_fallback.rs`
- `crates/fonts/src/tests_sysfonts.rs`
- `crates/fonts/src/tests_vertical.rs`
- `crates/fonts/tests/it/main.rs`
- `crates/fonts/tests/it/system_fonts.rs`
- `crates/fonts/tests/system_fonts.rs`
- `crates/format/src/lib.rs`
- `crates/format/src/tests.rs`
- `crates/geom/src/path.rs`
- `crates/geom/src/trace.rs`
- `crates/geom/src/units.rs`
- `crates/idml/src/export.rs`
- `crates/idml/src/export/common.rs`
- `crates/idml/src/export/designmap.rs`
- `crates/idml/src/export/fonts_prefs.rs`
- `crates/idml/src/export/graphics.rs`
- `crates/idml/src/export/item.rs`
- `crates/idml/src/export/spread.rs`
- `crates/idml/src/export/story.rs`
- `crates/idml/src/export/styles.rs`
- `crates/idml/src/export/table.rs`
- `crates/idml/src/export/text_attrs.rs`
- `crates/idml/src/import.rs`
- `crates/idml/src/import/document.rs`
- `crates/idml/src/import/frame.rs`
- `crates/idml/src/import/item.rs`
- `crates/idml/src/import/object_style.rs`
- `crates/idml/src/import/package.rs`
- `crates/idml/src/import/spread.rs`
- `crates/idml/src/import/story.rs`
- `crates/idml/src/import/styles.rs`
- `crates/idml/src/import/swatches.rs`
- `crates/idml/src/import/table.rs`
- `crates/idml/src/import/text_attrs.rs`
- `crates/idml/src/import/values.rs`
- `crates/idml/src/names.rs`
- `crates/idml/src/ruby.rs`
- `crates/idml/src/tests.rs`
- `crates/idml/src/tests/assets.rs`
- `crates/idml/src/tests/facing_ltr.rs`
- `crates/idml/src/tests/frame_grid.rs`
- `crates/idml/src/xml.rs`
- `crates/images/Cargo.toml`
- `crates/images/src/lib.rs`
- `crates/images/src/psd.rs`
- `crates/images/src/psd/tests.rs`
- `crates/images/src/tests.rs`
- `crates/images/tests/it/main.rs`
- `crates/images/tests/it/rgba_conversion_contract.rs`
- `crates/images/tests/rgba_conversion_contract.rs`
- `crates/mcp/src/server.rs`
- `crates/mcp/src/tests.rs`
- `crates/mcp/tests/automation_roots.rs`
- `crates/mcp/tests/it/automation_roots.rs`
- `crates/mcp/tests/it/main.rs`
- `crates/pdf/Cargo.toml`
- `crates/pdf/src/export.rs`
- `crates/pdf/src/export/booklet.rs`
- `crates/pdf/src/export/common.rs`
- `crates/pdf/src/export/graphics.rs`
- `crates/pdf/src/export/images.rs`
- `crates/pdf/src/export/metadata.rs`
- `crates/pdf/src/export/paint.rs`
- `crates/pdf/src/export/pipeline.rs`
- `crates/pdf/src/export/walk.rs`
- `crates/pdf/src/forms.rs`
- `crates/pdf/src/gradient_inks.rs`
- `crates/pdf/src/inks.rs`
- `crates/pdf/src/lib.rs`
- `crates/pdf/src/optional_content.rs`
- `crates/pdf/src/pdfx.rs`
- `crates/pdf/src/pdfx/fonts.rs`
- `crates/pdf/src/pdfx/fonts/tests.rs`
- `crates/pdf/src/pdfx/tests.rs`
- `crates/pdf/src/pdfx1a.rs`
- `crates/pdf/src/print_tests.rs`
- `crates/pdf/src/registration_tests.rs`
- `crates/pdf/src/rtl_tests.rs`
- `crates/pdf/src/tests.rs`
- `crates/pdf/src/text.rs`
- `crates/pdf/src/transitions.rs`
- `crates/pdf/src/x1a_tests.rs`
- `crates/pdf/tests/gradient_color_spaces.rs`
- `crates/pdf/tests/image_content_cache.rs`
- `crates/pdf/tests/it/gradient_color_spaces.rs`
- `crates/pdf/tests/it/image_content_cache.rs`
- `crates/pdf/tests/it/main.rs`
- `crates/pdf/tests/it/page_entry_search_contract.rs`
- `crates/pdf/tests/it/spot_gradients.rs`
- `crates/pdf/tests/page_entry_search_contract.rs`
- `crates/pdf/tests/support/image_content_cache_contract.rs`
- `crates/pptx/src/tests.rs`
- `crates/render/src/blendspace.rs`
- `crates/render/src/color.rs`
- `crates/render/src/damage.rs`
- `crates/render/src/decode.rs`
- `crates/render/src/draw.rs`
- `crates/render/src/driver.rs`
- `crates/render/src/fx.rs`
- `crates/render/src/images.rs`
- `crates/render/src/lib.rs`
- `crates/render/src/pdf_bounds.rs`
- `crates/render/src/pdf_layers.rs`
- `crates/render/src/pdf_raster.rs`
- `crates/render/src/stroke.rs`
- `crates/render/src/tests.rs`
- `crates/render/src/text.rs`
- `crates/render/src/types.rs`
- `crates/render/tests/blend_over_paper.rs`
- `crates/render/tests/colorize.rs`
- `crates/render/tests/effects.rs`
- `crates/render/tests/glyph_retention.rs`
- `crates/render/tests/golden/inner-shadow.png`
- `crates/render/tests/gradient_inks.rs`
- `crates/render/tests/gradients.rs`
- `crates/render/tests/it/blend_over_paper.rs`
- `crates/render/tests/it/colorize.rs`
- `crates/render/tests/it/effects.rs`
- `crates/render/tests/it/glyph_retention.rs`
- `crates/render/tests/it/gradient_inks.rs`
- `crates/render/tests/it/gradients.rs`
- `crates/render/tests/it/main.rs`
- `crates/render/tests/it/pdf_geometry.rs`
- `crates/render/tests/it/perf.rs`
- `crates/render/tests/it/placed_pdf.rs`
- `crates/render/tests/it/stroke_alignment.rs`
- `crates/render/tests/it/text_frame_bottom.rs`
- `crates/render/tests/it/text_stroke_behind.rs`
- `crates/render/tests/pdf_geometry.rs`
- `crates/render/tests/perf.rs`
- `crates/render/tests/placed_pdf.rs`
- `crates/render/tests/stroke_alignment.rs`
- `crates/render/tests/text_frame_bottom.rs`
- `crates/render/tests/text_stroke_behind.rs`
- `crates/textimport/src/docx.rs`
- `crates/textimport/src/docx/tests.rs`
- `crates/textimport/src/docx_numbering.rs`
- `crates/textimport/src/lib.rs`
- `crates/textimport/src/rtf.rs`
- `crates/textimport/src/tagged.rs`
- `crates/textimport/src/xlsx.rs`
- `crates/tools/src/catalog.rs`
- `crates/tools/src/frame.rs`
- `crates/tools/src/gradient.rs`
- `crates/tools/src/lib.rs`
- `crates/tools/src/nav.rs`
- `crates/tools/src/select.rs`
- `crates/tools/src/snap.rs`
- `crates/tools/src/snap/tests.rs`
- `crates/tools/src/tests.rs`
- `crates/tools/src/text.rs`
- `crates/ui-egui/Cargo.toml`
- `crates/ui-egui/examples/ui_shot.rs`
- `crates/ui-egui/src/about.rs`
- `crates/ui-egui/src/app.rs`
- `crates/ui-egui/src/app/clipboard.rs`
- `crates/ui-egui/src/app/commands.rs`
- `crates/ui-egui/src/app/control.rs`
- `crates/ui-egui/src/app/file_dialogs.rs`
- `crates/ui-egui/src/app/files.rs`
- `crates/ui-egui/src/app/frame.rs`
- `crates/ui-egui/src/app/ime.rs`
- `crates/ui-egui/src/app/input.rs`
- `crates/ui-egui/src/app/keys.rs`
- `crates/ui-egui/src/app/quit.rs`
- `crates/ui-egui/src/app/screenshot.rs`
- `crates/ui-egui/src/app/view.rs`
- `crates/ui-egui/src/canvas.rs`
- `crates/ui-egui/src/canvas/column_guides.rs`
- `crates/ui-egui/src/canvas/frames.rs`
- `crates/ui-egui/src/canvas/guides.rs`
- `crates/ui-egui/src/canvas/ime.rs`
- `crates/ui-egui/src/canvas/input.rs`
- `crates/ui-egui/src/canvas/page_decor.rs`
- `crates/ui-egui/src/canvas/render.rs`
- `crates/ui-egui/src/canvas/rulers.rs`
- `crates/ui-egui/src/canvas/scroll.rs`
- `crates/ui-egui/src/canvas/tests.rs`
- `crates/ui-egui/src/canvas/text_overlay.rs`
- `crates/ui-egui/src/canvas/tool_overlays.rs`
- `crates/ui-egui/src/canvas/view.rs`
- `crates/ui-egui/src/chrome.rs`
- `crates/ui-egui/src/chrome/app_bar.rs`
- `crates/ui-egui/src/chrome/control_bar.rs`
- `crates/ui-egui/src/chrome/start_screen.rs`
- `crates/ui-egui/src/chrome/status_bar.rs`
- `crates/ui-egui/src/chrome/tests.rs`
- `crates/ui-egui/src/clock.rs`
- `crates/ui-egui/src/control.rs`
- `crates/ui-egui/src/credits.rs`
- `crates/ui-egui/src/cursors.rs`
- `crates/ui-egui/src/dialogs.rs`
- `crates/ui-egui/src/dialogs/options.rs`
- `crates/ui-egui/src/dialogs/style_cjk.rs`
- `crates/ui-egui/src/dialogs/tests.rs`
- `crates/ui-egui/src/dock.rs`
- `crates/ui-egui/src/effects_dialog.rs`
- `crates/ui-egui/src/guide_selection_tests.rs`
- `crates/ui-egui/src/i18n.rs`
- `crates/ui-egui/src/i18n/ar.rs`
- `crates/ui-egui/src/i18n/cs.rs`
- `crates/ui-egui/src/i18n/de.rs`
- `crates/ui-egui/src/i18n/fr.rs`
- `crates/ui-egui/src/i18n/it.rs`
- `crates/ui-egui/src/i18n/ja.rs`
- `crates/ui-egui/src/i18n/ka.rs`
- `crates/ui-egui/src/i18n/pl.rs`
- `crates/ui-egui/src/i18n/pt_br.rs`
- `crates/ui-egui/src/i18n/ru.rs`
- `crates/ui-egui/src/i18n/table.rs`
- `crates/ui-egui/src/i18n/tests.rs`
- `crates/ui-egui/src/i18n/uk.rs`
- `crates/ui-egui/src/icons.rs`
- `crates/ui-egui/src/key_names.rs`
- `crates/ui-egui/src/keys.rs`
- `crates/ui-egui/src/lib.rs`
- `crates/ui-egui/src/menus.rs`
- `crates/ui-egui/src/menus/bar.rs`
- `crates/ui-egui/src/menus/context.rs`
- `crates/ui-egui/src/menus/export.rs`
- `crates/ui-egui/src/menus/invoke.rs`
- `crates/ui-egui/src/menus/keys.rs`
- `crates/ui-egui/src/menus/quick_apply.rs`
- `crates/ui-egui/src/menus/run.rs`
- `crates/ui-egui/src/menus/state.rs`
- `crates/ui-egui/src/menus/tables.rs`
- `crates/ui-egui/src/menus/tests.rs`
- `crates/ui-egui/src/menus/tree.rs`
- `crates/ui-egui/src/menus/ui_commands.rs`
- `crates/ui-egui/src/new_document.rs`
- `crates/ui-egui/src/new_document/tests.rs`
- `crates/ui-egui/src/object_control.rs`
- `crates/ui-egui/src/package_dialog.rs`
- `crates/ui-egui/src/page_history.rs`
- `crates/ui-egui/src/panel_docking.rs`
- `crates/ui-egui/src/panel_docking/tests.rs`
- `crates/ui-egui/src/panel_docking_tests.rs`
- `crates/ui-egui/src/panels.rs`
- `crates/ui-egui/src/panels/cell_stroke.rs`
- `crates/ui-egui/src/panels/color_panel_tests.rs`
- `crates/ui-egui/src/panels/conditions.rs`
- `crates/ui-egui/src/panels/datamerge.rs`
- `crates/ui-egui/src/panels/font_menu.rs`
- `crates/ui-egui/src/panels/font_menu_ui_tests.rs`
- `crates/ui-egui/src/panels/glyphs.rs`
- `crates/ui-egui/src/panels/interactive.rs`
- `crates/ui-egui/src/panels/layers.rs`
- `crates/ui-egui/src/panels/library.rs`
- `crates/ui-egui/src/panels/mod.rs`
- `crates/ui-egui/src/panels/notes.rs`
- `crates/ui-egui/src/panels/pages.rs`
- `crates/ui-egui/src/panels/pages/tests.rs`
- `crates/ui-egui/src/panels/properties.rs`
- `crates/ui-egui/src/panels/properties/document.rs`
- `crates/ui-egui/src/panels/properties/flex.rs`
- `crates/ui-egui/src/panels/properties/links.rs`
- `crates/ui-egui/src/panels/properties/object.rs`
- `crates/ui-egui/src/panels/properties/object_panels.rs`
- `crates/ui-egui/src/panels/properties/tests.rs`
- `crates/ui-egui/src/panels/properties/text.rs`
- `crates/ui-egui/src/panels/properties/typography.rs`
- `crates/ui-egui/src/panels/separations.rs`
- `crates/ui-egui/src/panels/story.rs`
- `crates/ui-egui/src/panels/styles.rs`
- `crates/ui-egui/src/panels/swatches.rs`
- `crates/ui-egui/src/panels/table.rs`
- `crates/ui-egui/src/panels/tabs.rs`
- `crates/ui-egui/src/panels/typography.rs`
- `crates/ui-egui/src/parent_canvas_tests.rs`
- `crates/ui-egui/src/prefs.rs`
- `crates/ui-egui/src/prefs/tests.rs`
- `crates/ui-egui/src/render_worker.rs`
- `crates/ui-egui/src/rtl.rs`
- `crates/ui-egui/src/section_options.rs`
- `crates/ui-egui/src/separations.rs`
- `crates/ui-egui/src/services.rs`
- `crates/ui-egui/src/shortcut_import.rs`
- `crates/ui-egui/src/shortcuts_dialog.rs`
- `crates/ui-egui/src/state.rs`
- `crates/ui-egui/src/story_editor.rs`
- `crates/ui-egui/src/taskbar.rs`
- `crates/ui-egui/src/tests.rs`
- `crates/ui-egui/src/theme.rs`
- `crates/ui-egui/src/title_bar.rs`
- `crates/ui-egui/src/toolbar.rs`
- `crates/ui-egui/src/toolbar/tests.rs`
- `crates/ui-egui/src/updates_ui.rs`
- `crates/ui-egui/src/widgets.rs`
- `crates/ui-egui/tests/it/color_profiles_restart.rs`
- `crates/ui-egui/tests/it/main.rs`
- `crates/ui-egui/tests/it/native_job_lifetime_control.rs`
- `crates/ui-egui/tests/native_job_lifetime_control.rs`
- `xtask/Cargo.toml`
- `xtask/src/assets.rs`
- `xtask/src/layers.rs`
- `xtask/src/main.rs`
- `xtask/src/pdf_deps.rs`

## Final branch handling

OBSERVATION: origin/main advanced during validation to b90d876c0 with the preset census documentation. Rebased the checkpoint onto that main; the only rebase conflict was the appended SOURCES.md entries, and both entries were retained. Preserved the already published checkpoint as an additional parent so the final push can be a normal fast-forward without rewriting published history. This joins only our own branch history, not upstream history. The temporary upstream remote and detached trial worktrees were removed. No upstream action or release was made.
