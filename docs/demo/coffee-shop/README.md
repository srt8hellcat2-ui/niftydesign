<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# NiftyDesign Coffee Shop demo

2026-10-21. Built by opening the pinned public Apache `Sample2.pub` in the app,
then using real engine commands through `ui_shot` (egui_kittest + wgpu).
No reader logic or other application code changed. No hosted workflow was dispatched.

Interface renderings from the project's own test renderer, starting from one public Apache test file; the added content is original and fictional; AI-generated images are listed in ASSETS.md; not a real Windows desktop capture.

## What was done

1. Opened external `Sample2.pub`; captured the original page and its missing-font
   dialog, then dismissed the dialog and captured the unobscured page. Its three
   missing faces were Arial Regular, Arial Bold Italic and Times New Roman Regular.
   These belong to the original sample, not the finished design.
2. Deleted all seven original objects through `edit.clear`. `document.inspect`
   then contained no original stories. No original text, table or picture remains
   hidden under the new design. Set document defaults and page 1 to US Letter,
   **612 × 792 pt (8.5 × 11 in)**, and placed the owner background at `[0,0,612,792]`.
3. Created the headline, subhead and three original body lines. Source Serif 4 Bold
   is 66 pt for “Fresh Today”; Source Sans 3 Semibold is 27 pt for the shop name;
   body text is Source Sans 3 Regular, 17 pt. No installed fonts are required for
   any final text: `font.list` reports exact bundled matches for all three faces.
4. Placed the two alpha PNGs with `file.place`, moved them with `transform.move`,
   resized their frames with `transform.set`, then fitted their content
   proportionally with `object.fit`. Added a rounded 14 pt corner rectangle,
   warm brown `#9B5933`, with the special, plus an ellipse and 1.5 pt line accent.
   Espresso text is `#543422`; cream text/background is `#FFF4DE`.
5. Resized the existing second page to **252 × 144 pt (exact custom 3.5 × 2 in;
   not an approximate preset)**. Added the fictional name, role, email and phone,
   cream background, brown band and small cup image.
6. Saved `coffee-shop.designcraft`, reopened it in the same session, saved an
   evidence copy, then started a fresh `ui_shot` process and reopened it there.
   **All document.json fields were identical**, including all 19 objects,
   10 stories, transforms, bounds, fills, image links, pixel dimensions and
   graphic sizes. All three embedded PNGs were byte-identical to their inputs
   and to both reopened copies. Absolute source links are recorded by the app;
   the actual image bytes are also embedded in the native file.
   Pictures remained visible in the same-session and fresh-session captures.
   The earlier newsletter disappearance did not reproduce; no speculative fix
   was made and this does not establish its cause.
7. Exported both pages through `file.exportPdf` after same-session reopen:
   2 pages, no export warnings. Rasterized the complete pages with `pdftoppm`
   at 144 dpi: **1224 × 1584** and **504 × 288** pixels.

## Reproduce and verify

From the checkout root, using the project's built binaries and Python Pillow:

```sh
MESA_SHADER_CACHE_DISABLE=true python3 docs/demo/coffee-shop/reproduce.py \
  --harness /path/to/target/debug/examples/ui_shot \
  --cli /path/to/target/debug/designcraft-cli \
  --sample /outside/checkout/Sample2.pub
```

The script checks the pinned SHA-256 before and after the run and runs
`verify.py` automatically. The sample is **not** bundled here. Its provenance:
Apache POI `test-data/publisher/Sample2.pub`, trunk commit
`bf7c9b6a1490046517cbd8e1504e702a89be4bc9` (repository Apache-2.0), SHA-256
`c87ea7cc5606023c5fa99376631bbd45a8e9bdaba34c958057995b0e7ac8cc86`.
No upstream source, `.txt` or other reader output was used.
Full JSONL inputs and command replies are generated in ignored
`.builder/coffee-demo/`; the script checks every command reply and uses IDs from
this pinned import/command sequence. A changed allocation sequence must fail the
checks rather than silently produce evidence. `engine-values.json` retains the
complete per-object inspection values, original warnings and final font/preflight
results. It is evidence from the app, not hand-edited document data.

To repeat only the state/transparency verification after reproduction:

```sh
python3 docs/demo/coffee-shop/verify.py --cli /path/to/target/debug/designcraft-cli
```

## Screenshots and outputs

All screenshots are complete, unretouched outputs of the same test renderer.
The magenta lines are the application's margin guides, not print content.

| File | What it shows |
|---|---|
| `01-original.png` | Original page 1 with the actual missing-font dialog |
| `01b-original-page.png` | Original page 1 after dismissing that dialog |
| `02-background.png` | Original objects removed; full-page owner background |
| `03-headline.png` | Headline, shop name and body copy |
| `04-images-placed.png` | Croissant and cup after placement, movement, resize and fit |
| `05-special-box.png` | Rounded warm-brown special box and its text |
| `06-final-flyer.png` | Finished Letter flyer with accents, hours and fictional address |
| `07-business-card.png` | Exact 3.5 × 2 in business card |
| `08-same-session-reopened.png` | Flyer after native save and same-session reopen |
| `09-fresh-session-reopened.png` | Flyer opened in a fresh process |
| `coffee-shop.designcraft` | Editable two-page document, including embedded images |
| `coffee-shop.pdf` | Both exported pages |
| `pdf-page-1.png`, `pdf-page-2.png` | Complete exported PDF pages, 144 dpi |

## Engine values

Coordinates are page/spread points, `[left, top, right, bottom]`.
All objects are on Layer 1. Every listed value survived both reopens unchanged.
The JSON evidence also includes the native image links, intrinsic pixel sizes,
font resolutions, story text and each placed/moved/resized state.

| Page | ID | Kind | Bounds (pt) | Fill / asset |
|---|---:|---|---|---|
| 1 | 26 | image | 0.000, 0.000, 612.000, 792.000 | [None]; asset 25 |
| 1 | 27 | text frame | 44.000, 55.000, 570.000, 148.000 | [None] |
| 1 | 29 | text frame | 48.000, 151.000, 565.000, 192.000 | [None] |
| 1 | 31 | text frame | 50.000, 207.000, 560.000, 290.000 | [None] |
| 1 | 34 | image | 52.000, 312.000, 332.000, 498.667 | [None]; asset 33 |
| 1 | 36 | image | 320.000, 297.000, 550.000, 488.667 | [None]; asset 35 |
| 1 | 37 | rectangle | 48.000, 513.000, 564.000, 626.000 | Caramel |
| 1 | 38 | text frame | 70.000, 529.000, 535.000, 562.000 | [None] |
| 1 | 40 | text frame | 70.000, 568.000, 535.000, 608.000 | [None] |
| 1 | 42 | ellipse | 50.000, 652.000, 62.000, 664.000 | Caramel |
| 1 | 43 | line | 78.000, 658.000, 560.000, 658.000 | [None] |
| 1 | 44 | text frame | 50.000, 680.000, 560.000, 712.000 | [None] |
| 1 | 46 | text frame | 50.000, 722.000, 565.000, 752.000 | [None] |
| 2 | 48 | rectangle | 0.000, 0.000, 252.000, 144.000 | Cream |
| 2 | 49 | rectangle | 0.000, 0.000, 252.000, 17.000 | Caramel |
| 2 | 50 | text frame | 16.000, 29.000, 184.000, 57.000 | [None] |
| 2 | 52 | text frame | 16.000, 60.000, 180.000, 80.000 | [None] |
| 2 | 54 | text frame | 16.000, 91.000, 180.000, 132.000 | [None] |
| 2 | 57 | image | 181.000, 72.000, 239.000, 120.333 | [None]; asset 35 |

Image movement and resizing, before → moved → resized:

- ID 34: [40.000, 330.000, 280.000, 490.000] → [52.000, 312.000, 292.000, 472.000] → [52.000, 312.000, 332.000, 498.667].
- ID 36: [320.000, 315.000, 530.000, 490.000] → [320.000, 297.000, 530.000, 472.000] → [320.000, 297.000, 550.000, 488.667].
- ID 57: [185.000, 64.000, 238.000, 108.167] → [181.000, 72.000, 234.000, 116.167] → [181.000, 72.000, 239.000, 120.333].

## Transparency and limitations

The croissant is RGBA 1536 × 1024; the cup is RGBA 1374 × 1145. Their zero-alpha
pixels and partial-alpha edge pixels remain in the original PNG bytes embedded
in all three native saves. The PDF has soft masks (`/SMask`). The verifier renders
a background-only reference through the actual exporter in a separate temporary
CLI session, without modifying the saved demo or any screenshot. For each of
the flyer croissant, flyer cup and card cup, eight zero-alpha edge samples in the
PDF raster matched the underlying colored background **exactly (maximum RGB
channel difference 0/255)**: 24 probes total. Visual inspection of the whole PDF
rasters also found no rectangular opaque backdrop. This checks the sampled
transparent edges; it is not a claim that every PDF pixel equals the UI renderer.
No transparency loss was observed after placing, saving, either reopen or export.

Final preflight: **0 errors, 7 warnings**. The background is about 130 effective
ppi, below the app's 150 ppi threshold, and six RGB-swatch warnings remain
(including two unused original sample swatches and the demo palette). The
supplied art was not enlarged or retouched to hide this limitation. The original
import also warns about table/formatting losses below; its content was removed
before designing. No object, text, fill or picture looked missing or changed
between the final and reopened views. The application may use a different zoom
when fitting the page; engine-state comparison does not depend on screenshot
pixel equality. Native file pickers, Windows desktop behavior, physical printing
and color/print production quality are **UNVERIFIED**.

## Exact Sample2.pub import warnings

- Publisher character formatting is unsupported for fields: defaults [none]; runs [0x00, 0x37, 0x38]. Supported attributes were retained.
- Publisher paragraph formatting is unsupported for fields: defaults [0x30]; runs [none]. Supported attributes were retained.
- Publisher table text across multiple stories/TCD sections is not decoded.
- Publisher table used stored endings for row positions; largest difference from size sums: 9564 EMU; cause unknown.
- Table cell-text association is unconfirmed or missing; the original text was retained after the table.
- Publisher table cell fills, borders, padding and other cell formatting are not decoded; unpainted cells were used.
