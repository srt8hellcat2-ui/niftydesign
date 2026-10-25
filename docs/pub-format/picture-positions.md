# Picture position baseline

## Result and meaning

OBSERVATION: on untouched main `15feae6`, all 142 controlled files import, all 152 exported elements have legacy overlap coverage, all 111 scored shape/text positions pass, all 52 texts are exact and all 111 checked styles match. The old log never measured positions for 41 image-labelled exports.

OBSERVATION: with the picture-only one-to-one matcher, **12 of 41 image-labelled references are within 1 pt, zero matched pictures miss, and 29 references are unmatched**. There are only 12 imported picture items among these references. The 29 unmatched entries are exports of other kinds of object under the current importer, not 29 measured picture-position failures. The old diagnostic count of 23 differences beyond 1 pt is not a picture baseline. Two were actual-picture comparisons using inappropriate bounds; the remaining 21 compared other kinds of item. No importer or renderer change was needed to obtain this result.

OBSERVATION: the retained compatibility line is identical before and after:

```text
SUMMARY files 142 imported 142 failed 0; truth elements 152 found 152 geometry-within-1pt 111 text exact 52/52 styling 111/111
```

OBSERVATION: the following lines are new:

```text
POSITIONS shapes/text: 111 of 111 scored objects within 1 pt
POSITIONS pictures: 12 of 41 pictures within 1 pt; 29 unmatched (reference image elements; imported pictures only; informational)
COVERAGE legacy found totals above measure overlap, not one-to-one picture matches.
```

OBSERVATION: searching the repository for `SUMMARY files`, `geometry-within-1pt` and the corpus report's function name found only the producer in `crates/pub/tests/corpus.rs`, not a parser of that summary. The old line is nevertheless retained for external users. Its `found 152` remains the historical overlap-coverage count, including rasterized text/shapes/tables. The separate picture count never uses that permissive coverage result. No picture count or tolerance result is asserted as a CI quality threshold.

## Matcher and bounds

OBSERVATION: only `Content::Graphic` leaves are picture candidates. Text frames, shapes, groups as a whole and tables are never picture candidates. Candidates belong to the current document page, including applicable non-overridden parent items. Nested group transforms are composed before measuring each picture. A parent item displayed on two different pages is two displayed instances; an instance is used at most once per page.

OBSERVATION: the matcher repeatedly chooses the globally nearest unused reference/picture pair. Distance is the sum of absolute errors in x, y, width and height; ties use reference order and item id. There is no overlap requirement and no distance cutoff that would hide a large positional miss as an unmatched reference. Each reference and picture instance can be used once. Shape/text matching, tolerances, style checking and text comparison are unchanged.

OBSERVATION: picture bounds are **the axis-aligned placed-frame outline after the full item/group transform, expanded for its visible stroke**, using `Item::visible_bounds`. These are painted frame bounds, not the full underlying image rectangle and not an alpha-pixel bounding box. Border handling follows the existing center/inside/outside stroke alignment. The cropped image's larger underlying rectangle is deliberately excluded; the imported frame already clips the image. This investigation does not change cropping or drawing.

INFERENCE: these bounds best describe the scoring export for the available simple cases. This is an inference about what to compare in a scoring oracle, not a statement about how Publisher stores bounds:

| Scoring case | Export bounds (x,y,w,h), pt | Diagnostic alternative | Chosen placed-frame bounds | Largest component error, pt |
| --- | --- | --- | --- | ---: |
| `picture__base` | (72.000000, 72.000000, 216.000000, 216.000000) | unstroked/untransformed frame (72.000000, 72.000000, 216.000000, 216.000000) | (72.000000, 72.000000, 216.000000, 216.000000) | 0.000000 |
| `picture__crop-all` | (87.120000, 87.120000, 185.760000, 185.760000) | full underlying image (71.999977, 71.999919, 216.000046, 216.000870) | (87.187480, 87.187480, 185.625039, 185.625748) | 0.134961 |
| `picture__border` | (69.600000, 69.600000, 220.800000, 220.800000) | unstroked/untransformed frame (72.000000, 72.000000, 216.000000, 216.000000) | (69.500000, 69.500000, 221.000000, 221.000000) | 0.200000 |
| `picture__rotated-30` | (32.400000, 32.400000, 295.200000, 295.200000) | unstroked/untransformed frame (72.000000, 72.000000, 216.000000, 216.000000) | (32.469256, 32.469256, 295.061487, 295.061487) | 0.138513 |

OBSERVATION: the base case matches exactly. The crop-all frame differs by at most 0.135 pt, whereas the full underlying image is about 216 × 216 pt rather than the export's 185.76 × 185.76 pt. The 5 pt border's unstroked frame has a 4.8 pt size difference; expanding for the centered border reduces the maximum component error to 0.2 pt. Applying the 30° transform reduces that picture's maximum difference from 79.2 to 0.139 pt. Crop-left, crop-top, moved, flipped and resized controls also pass without modifying product code.

UNKNOWN: the exporter's precise raster rounding, transparency, effect and irregular-outline bounds rules are not established by these few cases. The 0.08–0.2 pt residuals are consistent with raster quantization, but that is an INFERENCE, not a confirmed cause. The metric does not include pixel content, image quality or a shadow envelope. Visible-stroke bounds use the document helper's existing approximations; scaled group strokes, unusual joins and clipping inside non-group containers need separate evidence.

## Every matched picture

OBSERVATION: tuples are `(x, y, width, height)` in points; errors are ours minus expected, not a single combined error. Each row is within the unchanged 1 pt maximum-component tolerance. No fixture was selected or excluded by its score.

| Fixture; item/sequence | Expected | Ours | Signed component errors | Traits |
| --- | --- | --- | --- | --- |
| `picture__base`; 7/293 | (72.000000, 72.000000, 216.000000, 216.000000) | (72.000000, 72.000000, 216.000000, 216.000000) | (0.000000, 0.000000, 0.000000, 0.000000) | ordinary frame |
| `picture__border`; 7/293 | (69.600000, 69.600000, 220.800000, 220.800000) | (69.500000, 69.500000, 221.000000, 221.000000) | (-0.100000, -0.100000, 0.200000, 0.200000) | 5 pt border |
| `picture__crop-all`; 7/293 | (87.120000, 87.120000, 185.760000, 185.760000) | (87.187480, 87.187480, 185.625039, 185.625748) | (0.067480, 0.067480, -0.134961, -0.134252) | cropped |
| `picture__crop-left`; 7/293 | (87.120000, 72.000000, 200.880000, 216.000000) | (87.187480, 72.000000, 200.812520, 216.000000) | (0.067480, 0.000000, -0.067480, 0.000000) | cropped |
| `picture__crop-top`; 7/293 | (72.000000, 87.120000, 216.000000, 200.880000) | (72.000000, 87.187480, 216.000000, 200.812520) | (0.000000, 0.067480, 0.000000, -0.067480) | cropped |
| `picture__flip-h`; 7/293 | (72.000000, 72.000000, 216.000000, 216.000000) | (72.000000, 72.000000, 216.000000, 216.000000) | (0.000000, 0.000000, 0.000000, 0.000000) | horizontal flip |
| `picture__moved`; 7/293 | (100.080000, 120.000000, 216.000000, 216.000000) | (100.000000, 120.000000, 216.000000, 216.000000) | (-0.080000, 0.000000, 0.000000, 0.000000) | ordinary frame |
| `picture__oval-mask`; 7/293 | (72.000000, 72.000000, 216.000000, 216.000000) | (72.000000, 72.000000, 216.000000, 216.000000) | (0.000000, 0.000000, 0.000000, 0.000000) | ordinary frame |
| `picture__rotated-30`; 7/293 | (32.400000, 32.400000, 295.200000, 295.200000) | (32.469256, 32.469256, 295.061487, 295.061487) | (0.069256, 0.069256, -0.138513, -0.138513) | 30° rotation |
| `picture__stretched`; 7/293 | (72.000000, 72.000000, 130.080000, 130.080000) | (72.000000, 72.000000, 130.000000, 130.000000) | (0.000000, 0.000000, -0.080000, -0.080000) | resized |
| `picture__two`; 7/293 | (72.000000, 72.000000, 216.000000, 216.000000) | (72.000000, 72.000000, 216.000000, 216.000000) | (0.000000, 0.000000, 0.000000, 0.000000) | ordinary frame |
| `picture__two`; 9/295 | (319.920000, 72.000000, 100.080000, 100.080000) | (320.000000, 72.000000, 100.000000, 100.000000) | (0.080000, 0.000000, -0.080000, -0.080000) | resized |

OBSERVATION: **the list of matched pictures beyond 1 pt is empty**. Its grouped counts are therefore zero for every trait. For context, the measured set contains:

| Trait (groups overlap) | Matched pictures | Beyond 1 pt |
| --- | ---: | ---: |
| Cropped | 3 | 0 |
| Flipped | 1 | 0 |
| Rotated | 1 | 0 |
| Inside a group | 0 | 0 |
| Uncropped resized frame | 2 | 0 |
| Frame dimensions differ from 216 × 216 pt, including crops | 5 | 0 |
| 5 pt border | 1 | 0 |

OBSERVATION: “resized” here means an uncropped frame other than the 216 × 216 pt base: 130 × 130 pt in `picture__stretched` and 100 × 100 pt for the second `picture__two` item. It does not imply a suspicious source size. No pinned controlled native picture is inside a group; the grouped rotation/flip/stroke calculation is covered by a hand-computed synthetic test instead.

## The 29 unmatched references

OBSERVATION: every reference below is image-labelled for scoring but its fixture imports no picture candidate. The existing drawing parser returns no positive `0x0104` picture reference anywhere in these fixtures' drawing trees. The importer recognizes text, shapes, groups or tables instead. These are unmatched, not assigned fictitious picture coordinates or component errors.

| Export fixture family | Count | Unmatched fixture names |
| --- | ---: | --- |
| fill | 1 | `fill__gradient` |
| group | 1 | `group__rotated` |
| line | 3 | `line__dash-2`, `line__dash-4`, `line__dash-5` |
| shadow | 2 | `shadow__offset`, `shadow__on` |
| shapes | 11 | `shapes__curve`, `shapes__diamond`, `shapes__heart`, `shapes__hexagon`, `shapes__line`, `shapes__line-arrow`, `shapes__polyline`, `shapes__rightarrow`, `shapes__roundrect`, `shapes__star5`, `shapes__triangle` |
| table | 3 | `table__2x2`, `table__3x2`, `table__merged` |
| wordart | 8 | `wordart__arch`, `wordart__base`, `wordart__circle`, `wordart__fill-red`, `wordart__font`, `wordart__line-white`, `wordart__shadow`, `wordart__text` |

UNKNOWN: a picture-only metric does not measure these 29 objects' position accuracy. Their visual reference comparison remains separate. There is no new claim here that the existing parser understands every possible picture representation in every Publisher file.

## Candidate explanations and cheap checks

1. **INFERENCE: the earlier rotated-picture discrepancy compared coordinate spaces.** Confirm or kill by checking the existing parser's rotation and the imported item's transform, then compare its transformed frame. OBSERVATION: `picture__rotated-30.pub`, sequence 293, has property `0x0004 = 1966080`; the existing importer reads this as 30°. Its imported frame is `(72,72,216,216)` and transformed bounds are `(32.469256,32.469256,295.061487,295.061487)`. The reference error is at most 0.139 pt, so this check confirms the coordinate-space explanation for that old outlier, not a decoding defect.
2. **INFERENCE: the earlier border discrepancy compared an unpainted frame with painted extents.** Confirm or kill by checking the imported stroke and source width property, then expanding the frame using the document's stroke-bounds helper. OBSERVATION: `picture__border.pub`, sequence 293, returns `0x01CB = 63500`; the existing importer converts it to a centered 5 pt stroke. The chosen bounds are `(69.5,69.5,221,221)`, at most 0.2 pt from the export. This confirms the local bounds explanation. UNKNOWN: the exact rounding behind the remaining fraction of a point.
3. **INFERENCE: the rest of the old apparent misses came from treating rasterized non-picture exports as imported pictures.** Confirm or kill by inspecting the importer content types and all parsed drawing trees for a positive `0x0104` reference. OBSERVATION: the 29 unmatched elements have neither an imported picture candidate nor a positive parsed picture reference. Their previous 21 beyond-1-pt comparisons therefore do not demonstrate picture-position misses. This check confirms the classification explanation for this controlled set; it does not validate the other objects' geometry.

OBSERVATION: these checks used pinned binary bytes through the existing `escher::parse` and importer, plus the permitted local MS-ODRAW notes. Truth JSON was read only in the scoring comparison. Golden images were hash-checked but not used for storage inference or metric tuning. No other reader, source repository code or new format description was consulted.

## Run and validation

```sh
source /workspace/.setup/activate.sh
export PUB_CORPUS=/tmp/niftydata/pubsmith-fixtures
export PUB_REAL_CORPUS=/tmp/niftydata/apache-pub
cargo test --locked -p designcraft-pub --test corpus -- --ignored --nocapture
```

OBSERVATION: all 317 pinned input sizes and hashes matched existing manifests; no download was needed. Raw source audit, all 12 numeric comparisons and before/after logs remain outside the repository in `/tmp/niftydata/picture-scoring/`.

OBSERVATION: formatting, explicit warnings-denied clippy and all seven CI steps pass. Workspace tests: 3,094 passed, 0 failed, 24 ignored. Six new ordinary scorer tests cover picture-only filtering, one-to-one use, nearest bounds including size, global pair ordering, non-overlapping misses, frame rather than full-image bounds, grouped transforms/borders, page scope and applied/overridden parents. All 142 native save/reopen/edit/PDF cases pass. The legacy corpus summary is unchanged; every real file's status, pages, objects, notes, warnings and refusal/failure reasons match before/after. Real totals remain nine with notes, ten with warnings, two refused, three damaged, 31 pages and 280 objects.

UNKNOWN: Windows, real-PC fonts, Publisher's own drawing beyond the existing references, picture alpha/clip envelopes, picture shadows and real grouped-picture files were not tested by this baseline. No picture-quality threshold was introduced.

## Wording inventory

OBSERVATION: the entries below list every existing documentation location reworded in this task. Historical figures now name the 111 scored shape/text elements; no old test is presented as having run the new picture matcher. Explicit explanations quoting the old `111/152` ratio remain in `position-misses.md`, the historical scoring audit in `real-file-triage.md` and the historical check explanation in `pub-import-status.md`. The old summary output is also retained unchanged. README.md and ROADMAP.md had no ambiguous ratio to replace.

| Existing file | Locations reworded (original line numbers) |
| --- | --- |
| `docs/KNOWN-LIMITATIONS.md` | Checked only by automated tests, never by a person (line 69) |
| `docs/PUB-AGENTS.md` | 5. Quality gates (code changes) (line 23); 9. Data folders for agent environments (new paragraph after original line 41) |
| `docs/RELEASE-NOTES.md` | What changed since 0.12.0-nifty-alpha.3 (line 12) |
| `docs/file-format-parity.md` | Documents and interchange (line 33) |
| `docs/pub-format/SOURCES.md` | Builder: test maintenance check (2026-10-23) (line 899–900); Builder: alpha.2 preparation (2026-10-23) (line 933); Builder: 2026-10-23 — DesignCraft 0.12.0 base (line 960); Builder — 2026-10-24 C7 path observations and validation (line 1133); Builder — 2026-10-24 C8 baseline and blocked official download (line 1141); Builder — 2026-10-24 C8 baseline and blocked official download (line 1143); C8 heart outline — 2026-10-24 (line 1153) |
| `docs/pub-format/font-fallback.md` | Validation (line 141) |
| `docs/pub-format/pixel-diff.md` | Own baseline and checks (line 73) |
| `docs/pub-format/tables.md` | Why case (c), not (a) or (b) (line 227); Needs you — missing drawing notes (line 293) |
| `docs/pub-format/wordart.md` | Checks and limits (line 91) |
| `docs/pub-import-status.md` | Publisher import: what works (line 9); Publisher import: what works (line 11); Publisher import: what works (line 13); Publisher import: what works (line 15); Publisher import: what works (line 17); Score (task 3.4, unchanged on `pub-import` since) (line 102); Score (task 3.4, unchanged on `pub-import` since) (line 106); How this was checked (line 130) |
| `docs/publisher-parity-checklist.md` | Measured results (line 154); Caveat (line 175) |
| `docs/real-file-triage.md` | Per-item before/after checks (line 34–37); T4 converted-copy notice — 2026-10-23 (line 47); T3 diagnostic-severity update — 2026-10-23 (line 51); T1 validation update — 2026-10-23 (line 71); Validation (line 254); 2026-10-19 · C4 scoring audit: 41 unscored image exports (line 270); 2026-10-19 · queue final regression counts (C6) (line 362) |
| `docs/warning-triage.md` | Sources and validation (line 88) |
