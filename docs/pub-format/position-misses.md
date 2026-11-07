# Position-score investigation

## Result

OBSERVATION: there are **zero measured position misses**, not 41. On main `6906786`, the existing gate reports 152 found elements, 111 position passes, 52 exact texts and 111 correct styles. `crates/pub/tests/corpus.rs::score` increments `geometry_ok` only inside `if !want_image`. Its denominator still includes images. The scoring exports contain 59 `shape`, 52 `text` and 41 `image` elements. All 111 eligible elements pass; all 41 images are excluded, including images whose bounds happen to match. The unchanged 111/152 headline must not be read as 41 failed measurements.

OBSERVATION: the scorer matches overlapping leaf items by kind and nearest inner-frame origin. It allows an image to match any kind of item, flattens groups, and does not enforce one-to-one matching. `Item::inner_bounds` measures the item's path before its transform. A rasterized group in the scoring export can therefore be matched to one child rather than to the whole group. The table below reproduces that selection exactly; it is not a new accuracy gate.

## Evidence and method

OBSERVATION: the 142 pinned controlled binaries were imported with the existing importer. A temporary diagnostic wrapper outside the checkout recorded `Builder::place` item ids together with the existing `escher::parse` output, and composed the resulting documents. No parser or product source was edited. The wrapper's documents were compared with the unmodified public import function, excluding only creation and modification timestamps. Truth JSON was opened separately only to reproduce scoring and locate bound differences; it supplied no storage interpretation. Existing source notes supplied the permitted drawing-property background. Local `positions.json`, the diagnostic wrapper and validation logs remain in `/tmp/niftydata/layout-investigation/`.

OBSERVATION: all 317 existing input files matched their recorded sizes and SHA-256 values: 142 controlled binaries, 142 scoring JSON files, 24 Apache binaries and nine reference PNGs. No new data download was needed. The PNGs were hash-checked but were not needed to diagnose this score.

## The 41 excluded elements

OBSERVATION: tuples are `(x, y, width, height)` in points. Expected means the scoring export's bounds; ours means the scorer-selected item's untransformed inner bounds. Error is **ours minus expected**, separately for all four components; its magnitude is the absolute value. Coordinates include the same page offset as the existing scorer. Values are rounded to 0.001 pt for display. Every row below has scoring type `image` and status **not scored**, regardless of error size. Page/element numbers are one-based; item and Contents sequence numbers identify the imported/source shapes. Picture and table frame describe actual importer content, not the export's `image` label. A table is matched as one frame, not as individual cells.

| Fixture | Page/element; item/sequence | Imported kind; shape number | Expected x,y,w,h | Ours x,y,w,h | Signed errors x,y,w,h | Rotation; flips H/V | Inside group |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `fill__gradient` | 1/1; 7/293 | shape; 1 | (71.040, 71.040, 145.920, 73.920) | (72.000, 72.000, 144.000, 72.000) | (0.960, 0.960, -1.920, -1.920) | 0°; 0/0 | no |
| `group__rotated` | 1/1; 7/293 | shape; 1 | (69.360, 12.960, 255.360, 190.320) | (72.000, 72.000, 144.000, 72.000) | (2.640, 59.040, -111.360, -118.320) | 0°; 0/0 | yes; parent rotation 30.0° |
| `line__dash-2` | 1/1; 7/293 | shape; 1 | (71.040, 71.040, 145.920, 73.920) | (72.000, 72.000, 144.000, 72.000) | (0.960, 0.960, -1.920, -1.920) | 0°; 0/0 | no |
| `line__dash-4` | 1/1; 7/293 | shape; 1 | (71.040, 71.040, 145.920, 73.920) | (72.000, 72.000, 144.000, 72.000) | (0.960, 0.960, -1.920, -1.920) | 0°; 0/0 | no |
| `line__dash-5` | 1/1; 7/293 | shape; 1 | (71.040, 71.040, 145.920, 73.920) | (72.000, 72.000, 144.000, 72.000) | (0.960, 0.960, -1.920, -1.920) | 0°; 0/0 | no |
| `picture__base` | 1/1; 7/293 | picture; 1 | (72.000, 72.000, 216.000, 216.000) | (72.000, 72.000, 216.000, 216.000) | (0.000, 0.000, 0.000, 0.000) | 0°; 0/0 | no |
| `picture__border` | 1/1; 7/293 | picture; 1 | (69.600, 69.600, 220.800, 220.800) | (72.000, 72.000, 216.000, 216.000) | (2.400, 2.400, -4.800, -4.800) | 0°; 0/0 | no |
| `picture__crop-all` | 1/1; 7/293 | picture; 1 | (87.120, 87.120, 185.760, 185.760) | (87.187, 87.187, 185.625, 185.626) | (0.067, 0.067, -0.135, -0.134) | 0°; 0/0 | no |
| `picture__crop-left` | 1/1; 7/293 | picture; 1 | (87.120, 72.000, 200.880, 216.000) | (87.187, 72.000, 200.813, 216.000) | (0.067, 0.000, -0.067, 0.000) | 0°; 0/0 | no |
| `picture__crop-top` | 1/1; 7/293 | picture; 1 | (72.000, 87.120, 216.000, 200.880) | (72.000, 87.187, 216.000, 200.813) | (0.000, 0.067, 0.000, -0.067) | 0°; 0/0 | no |
| `picture__flip-h` | 1/1; 7/293 | picture; 1 | (72.000, 72.000, 216.000, 216.000) | (72.000, 72.000, 216.000, 216.000) | (0.000, 0.000, 0.000, 0.000) | 0°; 1/0 | no |
| `picture__moved` | 1/1; 7/293 | picture; 1 | (100.080, 120.000, 216.000, 216.000) | (100.000, 120.000, 216.000, 216.000) | (-0.080, 0.000, 0.000, 0.000) | 0°; 0/0 | no |
| `picture__oval-mask` | 1/1; 7/293 | picture; 1 | (72.000, 72.000, 216.000, 216.000) | (72.000, 72.000, 216.000, 216.000) | (0.000, 0.000, 0.000, 0.000) | 0°; 0/0 | no |
| `picture__rotated-30` | 1/1; 7/293 | picture; 1 | (32.400, 32.400, 295.200, 295.200) | (72.000, 72.000, 216.000, 216.000) | (39.600, 39.600, -79.200, -79.200) | 30°; 0/0 | no |
| `picture__stretched` | 1/1; 7/293 | picture; 1 | (72.000, 72.000, 130.080, 130.080) | (72.000, 72.000, 130.000, 130.000) | (0.000, 0.000, -0.080, -0.080) | 0°; 0/0 | no |
| `picture__two` | 1/1; 7/293 | picture; 1 | (72.000, 72.000, 216.000, 216.000) | (72.000, 72.000, 216.000, 216.000) | (0.000, 0.000, 0.000, 0.000) | 0°; 0/0 | no |
| `picture__two` | 1/2; 9/295 | picture; 1 | (319.920, 72.000, 100.080, 100.080) | (320.000, 72.000, 100.000, 100.000) | (0.080, 0.000, -0.080, -0.080) | 0°; 0/0 | no |
| `shadow__offset` | 1/1; 7/293 | shape; 1 | (71.040, 71.040, 152.160, 80.160) | (72.000, 72.000, 144.000, 72.000) | (0.960, 0.960, -8.160, -8.160) | 0°; 0/0 | no |
| `shadow__on` | 1/1; 7/293 | shape; 1 | (71.040, 71.040, 148.080, 76.080) | (72.000, 72.000, 144.000, 72.000) | (0.960, 0.960, -4.080, -4.080) | 0°; 0/0 | no |
| `shapes__curve` | 1/1; 7/293 | shape; 0 | (71.040, 162.000, 279.840, 76.080) | (72.000, 163.050, 278.000, 73.901) | (0.960, 1.050, -1.840, -2.179) | 0°; 0/0 | no |
| `shapes__diamond` | 1/1; 7/293 | shape; 4 | (70.560, 70.560, 146.880, 146.880) | (72.000, 72.000, 144.000, 144.000) | (1.440, 1.440, -2.880, -2.880) | 0°; 0/0 | no |
| `shapes__heart` | 1/1; 7/293 | shape; 74 | (72.000, 71.040, 144.720, 146.880) | (71.475, 73.784, 145.050, 142.216) | (-0.525, 2.744, 0.330, -4.664) | 0°; 0/0 | no |
| `shapes__hexagon` | 1/1; 7/293 | shape; 9 | (70.800, 71.040, 146.400, 146.160) | (72.000, 72.000, 144.000, 144.000) | (1.200, 0.960, -2.400, -2.160) | 0°; 0/0 | no |
| `shapes__line-arrow` | 1/1; 7/293 | shape; 20 | (71.520, 71.040, 144.480, 72.960) | (72.000, 72.000, 144.000, 72.000) | (0.480, 0.960, -0.480, -0.960) | 0°; 0/0 | no |
| `shapes__line` | 1/1; 7/293 | shape; 20 | (71.520, 71.040, 144.960, 73.920) | (72.000, 72.000, 144.000, 72.000) | (0.480, 0.960, -0.960, -1.920) | 0°; 0/0 | no |
| `shapes__polyline` | 1/1; 7/293 | shape; 0 | (71.040, 71.040, 146.160, 146.160) | (72.000, 72.000, 144.000, 144.000) | (0.960, 0.960, -2.160, -2.160) | 0°; 0/0 | no |
| `shapes__rightarrow` | 1/1; 7/293 | shape; 13 | (70.800, 67.680, 146.400, 152.640) | (72.000, 72.000, 144.000, 144.000) | (1.200, 4.320, -2.400, -8.640) | 0°; 0/0 | no |
| `shapes__roundrect` | 1/1; 7/293 | shape; 2 | (70.800, 71.040, 146.400, 146.160) | (72.000, 72.000, 144.000, 144.000) | (1.200, 0.960, -2.400, -2.160) | 0°; 0/0 | no |
| `shapes__star5` | 1/1; 7/293 | shape; 12 | (68.880, 68.640, 150.240, 150.240) | (75.524, 72.000, 136.952, 130.249) | (6.644, 3.360, -13.288, -19.991) | 0°; 0/0 | no |
| `shapes__triangle` | 1/1; 7/293 | shape; 5 | (70.320, 69.600, 147.360, 147.600) | (72.000, 72.000, 144.000, 144.000) | (1.680, 2.400, -3.360, -3.600) | 0°; 0/0 | no |
| `table__2x2` | 1/1; 7/293 | table frame; 201 | (71.760, 71.760, 288.240, 72.240) | (72.000, 72.000, 288.000, 72.000) | (0.240, 0.240, -0.240, -0.240) | 0°; 0/0 | no |
| `table__3x2` | 1/1; 7/293 | table frame; 201 | (71.760, 71.760, 288.240, 108.240) | (72.000, 72.000, 288.000, 108.000) | (0.240, 0.240, -0.240, -0.240) | 0°; 0/0 | no |
| `table__merged` | 1/1; 7/293 | table frame; 201 | (71.760, 71.760, 288.240, 72.240) | (72.000, 72.000, 288.000, 72.000) | (0.240, 0.240, -0.240, -0.240) | 0°; 0/0 | no |
| `wordart__arch` | 1/1; 7/293 | WordArt text frame; 144 | (64.800, 51.600, 281.520, 48.960) | (72.000, 72.000, 260.250, 25.500) | (7.200, 20.400, -21.270, -23.460) | 0°; 0/0 | no |
| `wordart__base` | 1/1; 7/293 | WordArt text frame; 136 | (71.520, 71.520, 261.120, 51.840) | (72.000, 72.000, 260.250, 51.000) | (0.480, 0.480, -0.870, -0.840) | 0°; 0/0 | no |
| `wordart__circle` | 1/1; 7/293 | WordArt text frame; 146 | (54.240, 41.520, 296.400, 122.160) | (72.000, 72.000, 260.250, 51.000) | (17.760, 30.480, -36.150, -71.160) | 0°; 0/0 | no |
| `wordart__fill-red` | 1/1; 7/293 | WordArt text frame; 136 | (71.520, 71.520, 261.120, 51.840) | (72.000, 72.000, 260.250, 51.000) | (0.480, 0.480, -0.870, -0.840) | 0°; 0/0 | no |
| `wordart__font` | 1/1; 7/293 | WordArt text frame; 136 | (71.520, 71.520, 261.120, 51.840) | (72.000, 72.000, 260.250, 51.000) | (0.480, 0.480, -0.870, -0.840) | 0°; 0/0 | no |
| `wordart__line-white` | 1/1; 7/293 | WordArt text frame; 136 | (71.280, 71.280, 261.840, 52.560) | (72.000, 72.000, 260.250, 51.000) | (0.720, 0.720, -1.590, -1.560) | 0°; 0/0 | no |
| `wordart__shadow` | 1/1; 7/293 | WordArt text frame; 136 | (71.520, 71.520, 263.280, 54.000) | (72.000, 72.000, 260.250, 51.000) | (0.480, 0.480, -3.030, -3.000) | 0°; 0/0 | no |
| `wordart__text` | 1/1; 7/293 | WordArt text frame; 136 | (71.520, 71.520, 261.120, 51.840) | (72.000, 72.000, 260.250, 51.000) | (0.480, 0.480, -0.870, -0.840) | 0°; 0/0 | no |

OBSERVATION: this is 41 elements in 40 fixtures. A diagnostic comparison of these unscored bounds finds 18 within 1 pt and 23 beyond 1 pt. These are **not** the missing 41 passes: the existing gate skips all 41, including those 18. Applying the same coordinate tolerance to image-export bounds is not necessarily a like-for-like frame comparison.

## Measurable attributes

OBSERVATION: groups below overlap. Counts refer to matched exported elements, not distinct files or source shapes. The middle column is the 41 excluded elements, not 41 failures.

| Attribute | Excluded images (41) | Scored hits (111) |
| --- | ---: | ---: |
| Scoring export type image | 41 | 0 |
| Own rotation nonzero | 1 | 7 |
| In a group | 1 | 2 |
| Parent rotation nonzero | 1 | 0 |
| Horizontal or vertical flip | 1 | 2 |
| Picture content | 12 | 0 |
| WordArt text frame | 8 | 0 |
| Table frame | 3 | 0 |
| Other text frame | 0 | 54 |
| Other shape | 18 | 57 |
| No stroke | 22 | 51 |
| 2 pt stroke | 18 | 56 |
| 5 pt stroke | 1 | 0 |
| Other stroke width | 0 | 4 |
| 612 × 792 pt page | 41 | 110 |
| 792 × 612 pt page | 0 | 1 |

OBSERVATION: the 111 hits have seven rotated items (six `geometry__rot-*` fixtures and `text__rotated-270`), two flipped items and two group members. Rotation, flips and grouping therefore do not distinguish all excluded elements from the hits. All excluded elements use a 612 × 792 pt page, but 110 hits do too. Their common distinguishing attribute is the scorer's `image` classification. The 5 pt border occurs only in the excluded set here; it does not explain the other 40 exclusions. Hits' other stroke widths are 0.25, 1, 4 and 10 pt, once each.

| Fixture family | Excluded elements | Scored hits |
| --- | ---: | ---: |
| fill | 1 | 8 |
| geometry | 0 | 28 |
| group | 1 | 2 |
| line | 3 | 6 |
| pages | 0 | 12 |
| picture | 12 | 0 |
| shadow | 2 | 0 |
| shapes | 11 | 2 |
| table | 3 | 0 |
| text | 0 | 46 |
| text2 | 0 | 7 |
| wordart | 8 | 0 |

## Candidate explanations and cheap checks

1. **INFERENCE: the apparent 41 failures are a denominator/exclusion effect.** Confirm by counting image labels and reading the `want_image` guard; kill this explanation if any non-image element has an error over 1 pt or a missing match. OBSERVATION: the count is exactly 41, every one takes the image branch, all 152 have matches, and all 111 non-image elements pass. This explains the entire reported shortfall without changing code.
2. **INFERENCE: some excluded differences compare painted extents with an inner frame.** Confirm for a simple unrotated example by expanding our frame by half its imported stroke; kill that local explanation if the residual remains large. OBSERVATION: `fill__gradient`, `line__dash-2`, `line__dash-4` and `line__dash-5` each have 2 pt strokes. The maximum four-component residual drops from 1.920 to 0.080 pt after that diagnostic expansion. `picture__border` has a 5 pt stroke; its residual drops from 4.800 to 0.200 pt. This supports that local explanation, not a general frame-decoding defect. UNKNOWN: the scoring exporter’s exact raster-bound/rounding rules. No rule about stored bounds was derived from these scoring values.
3. **INFERENCE: a rotated object's or whole group's exported bounds are not its selected leaf's unrotated bounds.** Confirm by using the existing document transforms and, for a group, unioning its children; kill this local explanation if that gives no improvement. OBSERVATION: `picture__rotated-30` improves from 79.200 pt maximum component difference to 0.139 pt using its transformed bounding box. For `group__rotated`, transforming only the selected first child still leaves 94.652 pt error; unioning both transformed children reduces that to 2.966 pt before stroke expansion. Its two source children have 2 pt strokes and the parent has 30° rotation. The original scorer never makes that union. UNKNOWN: whether stroke, raster rounding or another painted detail accounts for the remaining group difference. Neither operation changes the gate or the importer.

## Validation and limits

OBSERVATION: before and after, the controlled gate remains 142/142 imports, 152/152 found objects, 111 position passes under its existing exclusion rule, 52/52 exact texts and 111/111 styles. All 142 save/reopen/edit/PDF cases pass. The 24-file real batch remains nine with notes, ten with warnings, two refused and three damaged; 31 pages and 280 objects. Formatting passes; all seven CI steps pass, with 3,088 tests passed, zero failed and 24 ignored. No quality threshold, scoring rule or product code was changed.

UNKNOWN: actual Publisher frame coordinates or visual fidelity for image-only exports are not established by this investigation. In particular, the table is not evidence that every shape outline, table cell, glyph or shadow matches Publisher. Windows, real-PC fonts and Publisher's own layout on these files were not tested.
