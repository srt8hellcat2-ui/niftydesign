# WordArt paths

## Audit before implementation

OBSERVATION: on main `515f0925910698cb05386934003d44ea0b33395b`, the existing `escher::parse` and `import_pub` read these seven pinned binary fixtures. No truth JSON or reference image was used for this storage audit.

| Fixture suffix | Shape number | Geometry text | Stored font | Current drawing |
| --- | ---: | --- | --- | --- |
| arch | 144 | Sample Text! | Arial Black | Straight editable text, warning |
| circle | 146 | Sample Text! | Arial Black | Straight editable text, warning |
| base | 136 | Sample Text! | Arial Black | Straight editable text |
| fill-red | 136 | Sample Text! | Arial Black | Straight editable red text |
| line-white | 136 | Sample Text! | Arial Black | Straight editable text, white outline |
| shadow | 136 | Sample Text! | Arial Black | Straight editable text with shadow |
| font | 136 | Sample Text! | Times New Roman | Straight editable text |

OBSERVATION: each linked shape has Contents sequence 293, an unrotated frame `(72,72)..(332.25,123)` points and no flips. Properties 0x00C0 and 0x00C5 contain the UTF-16 text and font; the parser reports merged boolean property 0x00FF as 0xFFFF5700. No 0x00C2 alignment, 0x00C3 size, 0x00C4 spacing, 0x00C6 CSS font, 0x0147..0x0150 adjustment, 0x0145 vertices or 0x0146 segment data is present. These are observations of these files, not a claim about every Publisher file. The importer currently estimates size from frame dimensions and character count, centers paragraphs, retains names and styles, and creates rectangular frames with no `TextFrameOptions.path`.

OBSERVATION from the permitted MS-ODRAW notes in odraw-geometry.md: 144 is TextArchUpCurve and 146 TextCircleCurve. The binary fixtures confirm these two stored numbers. INFERENCE: mapping them to ECMA textArchUp and textCircle is consistent with the standard names and the observed files; the binary files alone do not contain those paths. UNKNOWN: fixture confirmation of TextArchDownCurve 145 or TextButtonCurve 147; neither occurs in this audit.

## Existing editable path text

OBSERVATION about the implementation: `crates/doc/src/item.rs` has `PathType` (start/end, flip and alignment), `crates/engine/src/cmd/path_type.rs` exposes `type.onPath` and `type.pathOptions`, and `crates/compose/src/lib.rs` composes a line by path length and places each glyph rigidly at its midpoint with the path tangent. `PathTextMap` maps editing positions. `crates/render/src/text.rs` paints those glyph outlines with the existing text fill and stroke. No new document feature is needed. UNKNOWN: whether this rigid placement reproduces Publisher's stretching and spacing.

## Default paths and fitting

OBSERVATION from the downloaded `presetTextWarpDefinitions.xml`: `textArchUp` defaults `adj` to 10800000 (180 degrees). At that default its named guides reduce to a start at `(left, vertical centre)`, radii `(width/2,height/2)`, start angle 180 degrees and positive sweep 180 degrees. `textCircle` has the same default start and radii; its conditional guides select `d3=10799999` and `swAng=21599998`, just two angle units short of a complete turn. One angle unit is 1/60000 degree. The paths are open, not closed polygons. `textArchDown` was read too: its default starts at the left midpoint and sweeps the lower half in the opposite direction. It is not mapped because there is no confirming fixture in this task.

OBSERVATION about the implementation: the geometry helper retains the two checked XML default arcs, but the importer enables only upward arch 144. Circle 146, other warps, any stored adjustment, multiline text and unsupported measurements keep their original straight editable frame and warning. The circle arc remains unit-tested geometry, not enabled import behavior. No XML is bundled or read at runtime.

## Measured fitting

OBSERVATION about the implementation: the importer now calls the existing composer on a temporary document clone with a straight baseline. The text is shaped at one point with normal horizontal scale and no baseline shift, using the same font resolution, substitute face, kerning and glyph shaping as rendering. It sums the real composed glyph advances, including spaces. The final size is the curved path's measured arc length divided by that one-point advance sum. No character count or guessed average advance enters the curved fit, and no horizontal scale override is added.

INFERENCE, chosen by visual comparison: arch lettering sits slightly inside the ellipse. Its inward amount is now the actual shaped text's ink descent, measured from the composed glyph outlines at one point and scaled to the fitted size. This replaces the fixed size/4 shift. It is a placement choice, not a Publisher storage rule. Original text and requested font names remain normal editable stories; the six presets, heart, fills, outlines and shadows are unchanged.

OBSERVATION: the one-point reference is unit normalization, not a tuned visible size. A million-point temporary straight path avoids wrapping while measuring, and a 16,384-byte text limit bounds the probe; oversized or overset probes decline. The final size takes one representable floating-point step downward to avoid endpoint rounding beyond the path, a numerical guard rather than a visible margin. Fits outside the existing 0.1–1296 point size range decline. None of these bounds was fitted to an image. The existing workspace composer is a new importer dependency; no external package was added.

OBSERVATION: in this Linux environment the arch's measured advances total 272.966213280 points, matching the path length. The resulting font size is 41.715902156 points and baseline shift is -10.021593682 points. The resolved face is Open Sans Extrabold; the original Arial Black name remains stored. The external test reports the actual resolved face and checks advance sum against path length, a single non-overset line and no horizontal scale override.

## Circle trials and decision

OBSERVATION: the rejected first version used path length and character count, horizontal scale 1.1 and size/4 baseline shift. It scored arch 0.983278719 / 0.006359358 and circle 0.953396313 / 0.013665241 (SSIM / differing fraction). Those visual constants have been removed from curved fitting.

OBSERVATION: measured-advance circle at size 83.431792742 with measured ink-descent placement scored 0.953676350 / 0.013636364. Letters still overlapped at the join. A second measured trial used the font's natural baseline with no shift: overlap was removed by eye, but the letters sat farther outside the reference and scored 0.942521982 / 0.014551872. Neither met the straight-text baseline of 0.960595137 / 0.009868449. Circle therefore remains straight with the existing warning. This does not establish that every possible circle-fitting design would fail; only the measured trials are claimed.

## Reference scoring

OBSERVATION: the final comparison uses the original c9-baseline production source and the reviewed arch-only implementation with the same fonts and settings. All nine sizes match at 850×1100. The table is generated from the recorded measurements; values are fractions, not percentages.

| Fixture | SSIM before | SSIM after | Differing fraction before | Differing fraction after |
| --- | ---: | ---: | ---: | ---: |
| group__rotated.pub | 0.999980060 | 0.999980060 | 0.001000000 | 0.001000000 |
| shadow__offset.pub | 0.999993784 | 0.999993784 | 0.000983957 | 0.000983957 |
| shadow__on.pub | 0.999994589 | 0.999994589 | 0.000963636 | 0.000963636 |
| wordart__arch.pub | 0.976310406 | 0.983892067 | 0.006426738 | 0.006321925 |
| wordart__base.pub | 0.980074652 | 0.980074652 | 0.006571123 | 0.006571123 |
| wordart__circle.pub | 0.960595137 | 0.960595137 | 0.009868449 | 0.009868449 |
| wordart__fill-red.pub | 0.979558751 | 0.979558751 | 0.012936898 | 0.012936898 |
| wordart__line-white.pub | 1.000000000 | 1.000000000 | 0.000000000 | 0.000000000 |
| wordart__shadow.pub | 0.979426457 | 0.979426457 | 0.009779679 | 0.009779679 |

OBSERVATION by eye: arch lettering follows the upward curve continuously, with no apparent overlapping letters. It fills the baseline without the old added horizontal stretch. Letter forms and some spacing still differ from the reference. Circle is straight again and visibly differs from its curved reference; it is explicitly unsupported. The other seven reference images and scores are unchanged. Both metrics meet the requested baseline for all nine, but this is not a fidelity match.

INFERENCE: the remaining arch differences are consistent with substitute glyph shapes and the unconfirmed placement rule. Their separate contributions remain UNKNOWN. Sparse white margins, substituted fonts and antialiasing affect the scores; a good score does not establish correct reading. Only nine Publisher references exist, six WordArt.

## Own-output regression

OBSERVATION: `c9-review-final` compared with `c9-baseline` reports one changed fixture, `wordart__arch.pub`: SSIM 0.971664035, differing fraction 0.007701608. There are no size mismatches or added/removed fixtures. Against the rejected `c9-after` run, only arch and circle differ (circle reverted to the baseline). No non-WordArt fixture changed. These are comparisons of our own output, not Publisher references.

## Reproduce

```sh
export PUB_CORPUS=/tmp/niftydata/pubsmith-fixtures
export PUB_REAL_CORPUS=/tmp/niftydata/apache-pub
cargo test --locked -p designcraft-pub --test wordart -- --ignored --nocapture
PUB_PIXEL_MODE=publisher cargo test --locked -p designcraft-cli --test pub_pixel_diff pub_pixel_diff -- --ignored --exact --nocapture
# Existing runs are immutable; choose a new name when repeating the snapshot.
PUB_PIXEL_MODE=snapshot PUB_PIXEL_RUN=my-wordart-change cargo test --locked -p designcraft-cli --test pub_pixel_diff pub_pixel_diff -- --ignored --exact --nocapture
PUB_PIXEL_MODE=compare PUB_PIXEL_BEFORE=c9-baseline PUB_PIXEL_AFTER=my-wordart-change cargo test --locked -p designcraft-cli --test pub_pixel_diff pub_pixel_diff -- --ignored --exact --nocapture
```

Reference CSV, markdown and all nine side-by-side images remain outside the repository in `/tmp/niftydata/pixel-report/`. Review trials, logs and rejected images are under `/tmp/niftydata/c9-review/`; snapshots remain under `/tmp/niftydata/pixel-snapshots/`. The original audit's input hashes remain recorded in SOURCES.md, and all 317 data files were reverified for the review.

OBSERVATION: the final 142-page snapshot took 1.236 seconds, excluding compilation/startup. The other 141 PNGs are byte-identical to c9-baseline. All 142 final PNGs are byte-identical to the pre-guard measured-fit preview.

## Checks and limits

OBSERVATION: hand-computed path endpoints, tangents, glyph transforms and ellipse-angle tests are retained. A synthetic four-point measured advance on a 200-point path requires a 50-point font and a measured 0.2-point descent gives a -10-point shift. A separate shaping test confirms that equal character counts (`iiii` and `WWWW`) produce different fitted sizes. The seven-fixture test checks actual advance sum, absence of overset, original names/text, editable stories, no horizontal scale override and circle's straight-text warning. Initial audit and fixture harness errors from the first revision were corrected; the review tests pass.

OBSERVATION: final CI passed all seven steps, including fmt, clippy and WebAssembly: 3,088 tests passed, zero failed, 24 ignored. The full controlled gate remains 142/142 imports, 152/152 objects, 111/152 positions, 52/52 exact texts and 111/111 styles. All 142 native save/reopen/edit/PDF cases pass. The real batch remains 9 notes, 10 warnings, 2 refused and 3 damaged, 31 pages and 280 objects; every real file's status/page/object counts are unchanged. The numeric guard declines non-finite measurements; its regression case passes, and it changes none of the controlled renders.

UNKNOWN: Publisher's exact stretching, kerning, tracking, vertical/reversed and multiline rules, non-default adjustments and combinations with decorations or shadows. Font substitution remains active, and a later font replacement may change line fit; continuous refitting after edits or font installation is not implemented. Circle and other warps remain unsupported. Windows, real-PC fonts, interactive desktop editing and Publisher drawing beyond the nine references were not tested.
