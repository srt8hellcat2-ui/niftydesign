<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Scripted Windows desktop demo

Once the workflow is registered on the repository's default branch, the owner
can run **NiftyDesign Windows desktop demo** in Actions → Run workflow, selecting
a branch containing `.github/workflows/windows-desktop-demo.yml`.
It is manual-only, read-only, unsigned, and neither publishes nor deploys.
No run has been dispatched by the builder. Windows execution is UNVERIFIED until
an owner-run artifact exists; successful local helper tests are not a Windows pass.

The job builds the real x64 desktop app with Rust 1.95.0, `--locked --release`, on
windows-2022. It downloads only pinned Apache `Sample2.pub`, verifying its size
and SHA-256 using the same manifest/helper as tester-cli. The sample stays in
RUNNER_TEMP outside the checkout. This is the previously scanned generic sample;
no newsletter, Microsoft template, donated file, or other Apache source is used.

The driver reuses the Linux Coffee Shop command plan, using the documented TCP
control channel and actual engine commands. Like the Linux baseline, it resizes
Sample2's existing second page for the card rather than appending a third page.
It opens the generic sample, replaces
its objects, builds the Letter flyer and 3.5 × 2 inch business card with bundled
fonts and the three owner demo images, moves/resizes pictures, saves, reopens in
the same session, and exports both PDF pages. Each major step has two complete,
unretouched images: the app's `ui.screenshot`, and a desktop capture made by
PowerShell `System.Drawing.Graphics.CopyFromScreen`. Uniform captures fail.
The original and any missing-font dialog are captured before dismissal.

Download the single **windows-desktop-demo** artifact. It contains:

- Both screenshot kinds, per-capture window title/bounds/size, control transcript,
  command plan and individual engine replies.
- `coffee-shop.designcraft`, `coffee-shop.pdf`, and two PDFium rasters of the
  actual exported PDF (not screenshots of the editor).
- `comparison.csv`: every object and its bounds (0.02 pt tolerance), object IDs
  and counts, story previews/composition, page sizes, full story text read back
  through `story.get` and from saves, links/pixel dimensions and embedded-byte
  preservation. Paths are compared as checkout-local files, not Linux strings.
  Expected values come from `docs/demo/coffee-shop/engine-values.json` and the
  existing demo's native file for full text. A mismatch fails the run.
- `REPORT.md`, exact JSON responses/transport errors, font availability for Arial
  and Times New Roman, initial dialog state, final fonts and preflight results.
- Per-attempt app output and isolated settings `logs/designcraft*.log`, even if
  initialization fails. Exact selected-adapter log lines state backend/device
  type. CPU means software; otherwise software status stays unknown unless the
  log explicitly identifies integrated/discrete hardware. Retry history is
  recorded separately; no fallback cause is inferred.

If default startup fails, the driver tries documented `WGPU_BACKEND=dx12`, then
`gl`, with a fresh settings directory per attempt and bounded startup/readback
waits. If no window/control/readback works it stops with failure evidence. It
does not install a guessed driver, pretend the runner has a GPU, or silently use
an offscreen renderer instead of the real app. A Windows runner may have no
usable interactive desktop/adapter, or CopyFromScreen may return blank output.
Compilation failures also fail the workflow; uploads use `always()`. The build
and driver steps have 100/15-minute limits within the 120-minute job budget,
leaving upload time. An interrupted driver leaves an INCOMPLETE report rather
than a completion claim.

Printing first records PowerShell `Get-Printer` and `file.printers {}`, then
`file.print` with `dryRun: true`. In this revision `file.printers` documents no
parameters; only `file.print` documents `dryRun`. If the installed allowlisted
**Microsoft Print to PDF** (preferred) or **Microsoft XPS Document Writer** queue is listed and dry-run succeeds, it attempts just
one real `file.print`, copies 1, pages 1–2, `systemDialog: false`. A fixed local
printer port points at the artifact's `virtual-printer.pdf` (or `.xps`) to avoid a Save Print
Output dialog; the exact setup result is retained. Administrative permissions,
spooler availability, PDF-driver behavior or a native output prompt may still
prevent output. A real submission has a 90-second response timeout. A timeout is
reported as no response, not a successful print; a successful response without a
file is reported without claiming a PDF appeared. There is no retry or physical
printer selection. Any output file is retained. No physical printing is proven.

This run proves only the commands, engine comparisons, rendered output and
virtual-spooler responses actually recorded. It is scripted, with no mouse, no
physical printer, no SmartScreen verification, no native Open/Save/Print dialog
verification, and no human desktop usability pass. It does not establish correct
layout for every Publisher file or validate the Windows alpha package/installer.

Local helper checks: `python -m unittest discover -s packaging -p test_windows_desktop_demo.py`.
The driver requires Pillow 11.3.0 and pypdfium2 4.30.0 (permissive PDFium wrapper;
installed only in the job, not bundled in a release).

Set `NIFTY_SAMPLE2` to the pinned external file to enable the optional local plan
fixture test; without it that one helper test skips and never downloads data.
