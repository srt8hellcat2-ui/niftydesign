<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Upgrading NiftyDesign from DesignCraft 0.18.0 to 0.20.x

Checked 2026-11-22. Nothing in this repository was upgraded; the check ran in a scratch copy.

## Where the fork stands

- The fork is on `0.18.0-nifty-alpha.5`. Its base commit `58ac6d29` ("Release 0.18.0") has exactly the same tree as upstream `v0.18.0` (same tree hash), so the fork's own work is a clean overlay of 62 commits: 341 files, +30,626 / -1,391 lines.
- Upstream has `v0.20.0` and `v0.20.1`. `v0.20.1` is the current patch release; this document targets it. (There is no `v0.19.x` fork step to take: `v0.19.0` is included in the range.)
- Size of the upstream change from `v0.18.0` to `v0.20.1`: 146 files (7 added, 139 modified), mostly `crates/engine` (51), `crates/ui-egui` (29) and `crates/doc` (16). From `v0.18.0` to `v0.20.0` alone it is 111 files, +6,738 / -484.
- 25 files were changed on both sides: `Cargo.toml`, `Cargo.lock`, `ROADMAP.md`, `apps/designcraft/src/main.rs`, six engine files (`cmd.rs`, `cmd/file/bytes.rs`, `cmd/interchange.rs`, `cmd/table.rs`, `lib.rs`, `recovery.rs`), `ui-egui/src/app/files.rs` and the translation files in `ui-egui/src/i18n/`.

## Trial result

Method: clone this fork, take upstream's change set `v0.18.0 -> v0.20.1` as a binary patch, apply it three-way onto the fork's current branch.

- All files applied without conflict, including all 25 that both sides changed, except `Cargo.toml` and `Cargo.lock`. Their conflicts are only the version line (`0.18.0-nifty-alpha.5` against `0.20.1`, 22 places). Resolved as `0.20.1-nifty-alpha.6`.
- Upstream also moves the pinned `craft-update` revision in `craft-libs` (`c5a1cbfb...` to `6e03f65b...`); the lock file changes with it and fetches that revision.
- Built and tested in the scratch copy: `cargo test -p designcraft-pub-legacy -p designcraft-pub -p designcraft-engine -p designcraft-ui-egui`. 31 test binaries passed, none failed (engine 1,159 tests, ui-egui 930, pub-legacy 58).

Not run: the full workspace, the 142-file corpus gate and the 24-file real-file batch, `cargo xtask ci` (fmt, clippy, assets, layers, wasm), and the desktop app on the upgraded code. The upgrade is not verified until those pass.

## What to do

1. Branch from the fork's working branch as `nifty-update-0.20.1`, and protect it before pushing (see below).
2. Apply the upstream patch three-way: `git diff --binary v0.18.0 v0.20.1` in an upstream clone, then `git apply --3way` here. Resolve `Cargo.toml` and `Cargo.lock` to the new fork version.
3. Change the version string everywhere it is hard-coded: `Cargo.toml`, `Cargo.lock`, `.github/workflows/alpha-windows.yml`, `.github/workflows/windows-alpha-release.yml`, `packaging/alpha_windows.py`, `packaging/test_windows_alpha_release.py`, `packaging/test_windows_contract.py`, `packaging/windows/README-ALPHA.txt`, `packaging/windows_alpha_release.py` (its error text), `README.md` ("Based on DesignCraft 0.18.0") and `ROADMAP.md`.
4. Run the gates that were not run: `cargo xtask ci`, the corpus (142/142 imports, 152/152 objects, 111/111 positions, 52/52 texts, 111/111 styles) and the real-file batch before and after, the wasm build, and the desktop test pass in [pub-format/desktop-test-log.md](pub-format/desktop-test-log.md).
5. Re-check the fork's integration points that upstream also touched: `cmd/file/bytes.rs`, `cmd/interchange.rs` (the legacy route), `recovery.rs`, `ui-egui/src/app/files.rs` and the Publisher entries in the translation files. They merged without conflicts, but merging cleanly is not the same as behaving the same.
6. Tag `v0.20.1-nifty-alpha.6` and release as usual. Releases are the owner's decision.

## Branches and the GitHub notice

The previous upgrade lived on the branch `nifty-update-0.18.0`, which has a different history from the fork's `main` (see ROADMAP). The upgrade branch will therefore be the working branch, and GitHub will show its "branch isn't protected" suggestion for it. A ruleset on the pattern `nifty-update-*` covers this and the next upgrade branch.
