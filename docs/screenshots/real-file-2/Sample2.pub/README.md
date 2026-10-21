<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Sample2.pub editing check — 2026-10-21

Interface renderings from the project's own test renderer using one public Apache test file; not a real Windows desktop capture; synthetic input is not a person using the app.

No images are embedded in any README. Filenames below identify private-test
evidence stored alongside this record; the owner decides whether to show it.
This is not a Windows desktop, native file-dialog, printing or publication check.

## Input and selection

See ../SCAN.md: Sample2.pub contains generic formatting/hyperlink/table test text
and simple geometry. Both original pages and all extracted story/cell text were
examined; no template indicators, personal information, photograph or clip art
was found in the imported output. It ties the richest clean family (two pages /
seven objects) and adds more test paragraphs. Similar variants were not captured
redundantly. No clean candidate with a picture was established.

Reused the existing external pinned Apache POI file, not an upstream reader or
text output. Repository apache/poi, trunk at
bf7c9b6a1490046517cbd8e1504e702a89be4bc9, test-data/publisher/Sample2.pub.
Repository license recorded as Apache-2.0; 72704 bytes, SHA-256
c87ea7cc5606023c5fa99376631bbd45a8e9bdaba34c958057995b0e7ac8cc86.
The .pub stays outside the checkout. All 24 source hashes were checked before
and after the test and match the own tester manifest.

## Captures and actual steps

Same existing ui_shot harness (egui_kittest + wgpu), Linux, 1440x900 logical
points at 2x. UI PNGs are complete 2880x1800 outputs, without edits/crops.
All seven retained images were individually inspected. No Chromium was used.

| File | Step |
|---|---|
| `01-open.png` | (a) Normal file.open; page 1, visible panels; actual missing-font dialog captured once. |
| `02-moved.png` | (b) Dialog cancelled through existing control; select frame 9; synthetic ui.drag (590,280) to (604,286), eight steps plus 20 settling frames. Engine bounds confirm movement. |
| `03-resized.png` | (c) Engine transform.set, frame 9, width 420 / height 90 / reference 0. |
| `04-edited-text.png` | (d) Engine story.replaceRange, story 10, first-line bytes 0..34; story.get confirms replacement and preserves the rest of the text. |
| `05-same-session-reopened.png` | (e) Save as edited.designcraft, reopen in the same harness; read bounds/text back. No pictures exist in this sample, so picture persistence is not applicable. |
| `06-fresh-session-reopened.png` | (e) Separate ui_shot process, empty Session, reopen the saved file; independently verify all three edits against engine state. |
| `07-exported-pdf-page-1.png` | (f) Fresh session exports a two-page PDF; pdftoppm rasterizes page 1 (708x1000). No substitute UI or fake PDF viewer. |

## Engine state (points, spread coordinates)

Frame 9 / story 10, page 1. Bounds are rounded to six decimals for this table;
verification compares full engine output, not pixels.

| State | Bounds [left, top, right, bottom] | Width x height |
|---|---|---|
| Imported | [85.039370,70.866142,484.724409,147.401575] | 399.685039 x 76.535433 |
| Moved | [97.795276,79.464567,497.480315,156.000000] | 399.685039 x 76.535433 |
| Resized | [97.795276,79.464567,517.795276,169.464567] | 420 x 90 |
| Same-session reopen | [97.795276,79.464567,517.795276,169.464567] | 420 x 90 |
| Fresh-session reopen | [97.795276,79.464567,517.795276,169.464567] | 420 x 90 |

Actual move delta: (12.755906,8.598425) pt; screen-pixel deltas are not assumed
to be point deltas. Text's first line changes from `This is some text on the
first page` to `NiftyDesign clean-file editing check.`. The complete story
changes from 163 to 165 UTF-8 bytes. The remaining text is unchanged. Edited,
same-session and fresh-session story.get replies match exactly: frame [9], four
lines/paragraphs, no overset in this edited story. Both move and resize persist.
The story's native paragraph and character formats match before/after (apart
from character run length); these were checked in saved document JSON;
no font substitution is applied to the document. Source SHA remains unchanged.
PDF export: two pages, 33426 bytes, no PDF-export warnings (import warnings remain).

## Exact import warnings

1. `Publisher character formatting is unsupported for fields: defaults [none]; runs [0x00, 0x37, 0x38]. Supported attributes were retained.`
2. `Publisher paragraph formatting is unsupported for fields: defaults [0x30]; runs [none]. Supported attributes were retained.`
3. `Publisher table text across multiple stories/TCD sections is not decoded.`
4. `Publisher table used stored endings for row positions; largest difference from size sums: 9564 EMU; cause unknown.`
5. `Table cell-text association is unconfirmed or missing; the original text was retained after the table.`
6. `Publisher table cell fills, borders, padding and other cell formatting are not decoded; unpainted cells were used.`

## Findings and limits

The font dialog lists three missing faces: Arial Bold Italic, Arial Regular and
Times New Roman Regular. Preflight counts two missing families (Arial and Times
New Roman), plus `Overset text: 81 characters` in table-host frame 15 on page 2:
**three errors before and after these edits**, not the newsletter's twelve.
The two RGB swatch notices are preflight warnings, not errors/import warnings.
Pink missing-font highlights and floating text toolbar are genuine UI output.
No fonts were fetched; fallback metrics may differ on Windows.

Move, resize, text edit and persistence pass in the engine. Chosen 90-pt height
makes this frame's lower bounds overlap the next frame by 7.889764 pt; it is the
requested geometry in this experiment, not a failed resize or an automatic
collision-avoidance promise. Text remains readable. The existing second-page
table story is overset/association-unconfirmed and is not repaired. Same-session
view zoom changes to roughly 75%, fresh session fits around 82%; model geometry
still matches exactly. No new content-loss or picture-loss claim can be made
for pictures because this clean sample has none. The prior newsletter picture
finding remains unresolved and is not tested by selecting a picture-free input.
No reader/engine/UI production changes were made.

## Reproduce

Adjust the external sample path in editing.jsonl if necessary. Outputs stay
private; no .pub, .designcraft, PDF, extracted text or CSV is committed here.

```sh
mkdir -p .builder/clean-sample2
UI_SHOT=target/debug/examples/ui_shot
"$UI_SHOT" docs/screenshots/real-file-2/Sample2.pub/editing.jsonl
"$UI_SHOT" docs/screenshots/real-file-2/Sample2.pub/fresh-reopen.jsonl
python3 docs/screenshots/real-file-2/Sample2.pub/verify.py .builder/clean-sample2 /path/to/Sample2.pub
pdftoppm -f 1 -singlefile -scale-to 1000 -png .builder/clean-sample2/edited.pdf .builder/clean-sample2/07-exported-pdf-page-1
```

The fresh-reopen script must run in a separate process, as it did in this test.
The verifier checks full engine responses and the original source hash. Nothing
was dispatched, published or deployed. Native Windows interactions remain UNVERIFIED.
