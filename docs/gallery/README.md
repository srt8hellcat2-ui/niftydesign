<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# NiftyDesign headless sample gallery

Generated on Linux with NiftyDesign CLI `0.9.0-nifty-alpha.1`. These are our
headless renderer's page-1 outputs, not desktop screenshots or Publisher truth
images. No donated or Apache real-file sample was used. The controlled inputs
are `.pub` data only from FuncularLabs/pubsmith at commit `0ccdf27`; hashes and
provenance are recorded in [SOURCES.md](../pub-format/SOURCES.md).

All PNGs are 612 x 792 pixels. CLI page indices start at zero. A successful render
is not a claim of complete visual fidelity to Publisher.

| Image | Source file | What it shows / limitation |
|---|---|---|
| [hello-nifty-page-1.png](hello-nifty-page-1.png) | `crates/pub/tests/data/hello-nifty.pub` | Synthetic text and blue rectangle. Text is `Hello from NiftyDesign`, regenerated through our own existing fixture helper. |
| [wmf-picture-page-1.png](wmf-picture-page-1.png) | `crates/pub/tests/data/wmf-picture.pub` | Same synthetic text plus a WMF picture: red triangle over a blue ellipse. |
| [bullets-page-1.png](bullets-page-1.png) | controlled `text__bullets.pub` | Small bullet and example paragraph text near the top left. |
| [shadow-page-1.png](shadow-page-1.png) | controlled `shadow__on.pub` | Blue rectangle with a dark outline/offset shadow; subtle at this scale. |
| [table-2x2-page-1.png](table-2x2-page-1.png) | controlled `table__2x2.pub` | **Blank output.** CLI exited successfully and wrote a valid nonempty PNG, but every RGB pixel is white. No visible grid or cell text. Follow-up diagnosis below: imported host story has no frame linkage. This is not a successful table visual demonstration. |
| [hello-nifty-pdf-page-1.png](hello-nifty-pdf-page-1.png) | NiftyDesign's [hello-nifty.pdf](hello-nifty.pdf), exported from `hello-nifty.pub` | Page 1 of our exported PDF, rasterized by `pdftoppm`; the after image below. |

## Before and after PDF export

These compare two output paths through NiftyDesign, **not** an original Publisher
page with an imported page. Both show the text and blue rectangle in the same
overall layout; rasterization/antialiasing can differ. No pixel-exact claim is made.

| Before: imported page rendered directly to PNG | After: exported PDF page rasterized to PNG |
|---|---|
| ![Imported hello-nifty page](hello-nifty-page-1.png) | ![NiftyDesign exported PDF page](hello-nifty-pdf-page-1.png) |

[Download the exported PDF](hello-nifty.pdf).

## Reproduce

With a built CLI (use `designcraft-cli.exe` on Windows):

```sh
designcraft-cli run --in crates/pub/tests/data/hello-nifty.pub --page 0 \
  --export docs/gallery/hello-nifty-page-1.png --export docs/gallery/hello-nifty.pdf
designcraft-cli run --in crates/pub/tests/data/wmf-picture.pub --page 0 \
  --export docs/gallery/wmf-picture-page-1.png
# For the controlled set, replace INPUT with the external pinned fixture path:
designcraft-cli run --in INPUT/text__bullets.pub --page 0 --export docs/gallery/bullets-page-1.png
designcraft-cli run --in INPUT/shadow__on.pub --page 0 --export docs/gallery/shadow-page-1.png
designcraft-cli run --in INPUT/table__2x2.pub --page 0 --export docs/gallery/table-2x2-page-1.png
pdftoppm -f 1 -singlefile -scale-to 792 -png docs/gallery/hello-nifty.pdf \
  docs/gallery/hello-nifty-pdf-page-1
```

Every selected CLI command and the PDF rasterizer exited successfully. The blank
table result above is retained rather than hidden. All six PNGs were inspected.
No reader or rendering code was changed to produce these images.

## Blank table diagnosis — 2026-10-20

The own CLI `document.inspect` and saved native document show a genuine 2x2 table
with cell texts `A1`, `B1`, `A2`, `B2`, row heights 36 pt and column widths 144 pt.
Text frame 7 points to host story 8, but that story's `frames` is empty. In the
own importer `tables.rs::table_frame`, the assignable-cell branch replaces the
linked source story with `Story::new(sid)` without retaining its frame list.
`designcraft_compose::frame_specs` gathers frames from `st.frames`; this story consequently has
no composed frames/lines. Its table text cannot reach `Renderer::draw_tables`.
The imported table deliberately also has no cell fills and zero-weight edges,
as documented in the format notes; those unsupported decorations cannot paint
a grid independently. The 612x792 PNG is confirmed uniformly white.

This is an importer/model linkage defect, not evidence that the sample has no
text or a newly discovered rasterizer defect. Per the task, it is reported
without changing the importer/readers or inventing a grid. The blank image is
retained as a visible failed demonstration. A later scoped fix should preserve
the existing host frame linkage and add a rendering regression test.
