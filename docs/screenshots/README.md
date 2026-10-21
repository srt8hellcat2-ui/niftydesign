<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# NiftyDesign interface renderings (C15)

Interface renderings from the project's own test renderer; not a real Windows desktop capture; native file dialogs and printing are UNVERIFIED.

Produced on Linux with `crates/ui-egui/examples/ui_shot.rs`: egui_kittest + wgpu,
1440 x 900 logical points at 2x, yielding unmodified 2880 x 1800 PNGs.
No headless Chromium, image retouching, cropping or compositing was used.
Only `hello-nifty.pub` and `wmf-picture.pub` from `crates/pub/tests/data` were opened.
No donated, Apache or other real-file samples were used for these C15 captures.
The earlier Microsoft-template newsletter captures are excluded from this source export.

## Images and limits

| Requested shot | Image / output | Sample | How it was produced / limit |
|---|---|---|---|
| 1. Start screen | [01-start.png](01-start.png) | None; empty session | Own UI test renderer, [start script](scripts/start.jsonl). NiftyDesign name, owner mark and attribution card. |
| 2. Hello in editor, panels visible | [02-hello-editor.png](02-hello-editor.png) | `hello-nifty.pub` | Own UI test renderer; normal `file.open`, expanded Properties/Pages/Layers dock, rulers and frame edges. Unedited fixture text is literal sample content, not the app name. |
| 3. WMF picture in editor | [03-wmf-editor.png](03-wmf-editor.png) | `wmf-picture.pub` | Own UI test renderer, [WMF script](scripts/wmf.jsonl). Synthetic triangle/ellipse picture and text. Import returned the warning `Text inside WMF/EMF pictures was left out.`; no fidelity claim is made. |
| 4. Text box edited | [04-edited-text.png](04-edited-text.png) | `hello-nifty.pub`, edited in memory | Own UI test renderer. Engine `story.replaceRange` replaced story 8, byte range 0..22 with `Sunday service at 10:00` and `Welcome to NiftyDesign.` on two lines. `story.get` verified the edit before capture. The unsaved marker is visible. |
| 5. Save As / format choice | [05-file-menu.png](05-file-menu.png), **partial** | Edited hello document | Own UI test renderer; actual File menu opened by synthetic pointer input. It shows Save and Save As, but **does not display a `.designcraft` format option**. The native Save As dialog/format selector was **not produced**: this harness supplies file read/write services but no `pick_save` provider; invoking Save As returned no dialog. No substitute picker was drawn. |
| 6. Export PDF dialog | [06-export-pdf-dialog.png](06-export-pdf-dialog.png) | Edited hello document | Own UI test renderer. `ui.menu.invoke` opened the real app `app.exportPdf` options dialog; it was captured and cancelled. This does not verify the later native destination picker or dialog confirmation on Windows. |
| 7. Exported PDF page beside editor | [07-edited-pdf-page.png](07-edited-pdf-page.png), paired below with image 04 | Edited hello document | Engine `file.exportPdf` generated [edited-hello.pdf](edited-hello.pdf); installed `pdftoppm` rasterized its first page (773 x 1000). This PNG is a PDF page rendering, not a UI screenshot. No single combined desktop/PDF-viewer capture was produced; the table below presents two original images side by side without editing them. |

## Editor and its exported PDF

| Edited document in the UI renderer | Page 1 of the PDF exported from that state |
|---|---|
| ![Edited hello document in the editor](04-edited-text.png) | ![Page of the edited document's exported PDF](07-edited-pdf-page.png) |

The PDF was exported through the engine before opening/cancelling the options
dialog. This pair is not a Publisher reference comparison, a real desktop capture
or a pixel-exact match test. The source .pub files were not modified.

## Reproduce and cache provenance

From the repository root, with a built renderer:

```sh
cargo build -p designcraft-ui-egui --example ui_shot
# Use the resulting target/debug/examples/ui_shot (ui_shot.exe on Windows):
UI_SHOT=target/debug/examples/ui_shot
"$UI_SHOT" docs/screenshots/scripts/start.jsonl
"$UI_SHOT" docs/screenshots/scripts/editor.jsonl
"$UI_SHOT" docs/screenshots/scripts/wmf.jsonl
pdftoppm -f 1 -singlefile -scale-to 1000 -png docs/screenshots/edited-hello.pdf \
  docs/screenshots/07-edited-pdf-page
```

The checked-in captures were rebuilt from this branch with workspace version
`0.9.0-nifty-alpha.1`, using the same egui_kittest + wgpu renderer as the About
screen. The File menu includes the existing Print command and Ctrl+P. Both
synthetic input fixtures were regenerated through the repository's test helpers
with `Hello from NiftyDesign`; no reader logic changed. This Linux rendering
is not evidence of a compiled Windows alpha or real desktop behavior.

Native Save As/format selection and printing remain UNVERIFIED; the custom
Export PDF options dialog is all that was rendered for shot 6. No hosted workflow
was dispatched and nothing was published or deployed.

## Save As follow-up — 2026-10-20

Desktop Save As is native-only: `menus::run_ui(app.saveDialog)` passes the
`.designcraft` suggestion to `Services.pick_save`; desktop `main::pick_save`
uses `rfd::FileDialog` with the suggested extension filter. There is no app-drawn
format chooser to capture. No substitute dialog or picker provider was added.

The [save/reopen script](scripts/save-reopen.jsonl), run with the same fresh
`ui_shot` harness, instead edits story 8 to `Sunday service at 10:00` and
`Welcome to NiftyDesign.` (two lines), saves `.builder/hello-edited.designcraft`
through `file.saveAs`, reopens it through `file.open`, and reads `story.get`
before and after. Both results match exactly: 47 UTF-8 bytes, two lines,
frame `[7]`, no overset. The native document ZIP's story text also matches, and
the source .pub SHA-256 is unchanged. The save/open commands returned success;
this proves engine persistence, not the native dialog. Run `mkdir -p .builder`
before `ui_shot docs/screenshots/scripts/save-reopen.jsonl` to reproduce.
