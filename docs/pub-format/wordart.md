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

OBSERVATION about the implementation: `crates/pub/src/wordart.rs` reduces only these two default definitions to elliptical arcs, then uses the existing geometry library's cubic approximation at 0.001 point tolerance. Radial angles are converted to ellipse parameter angles, including the circle's two-unit gap on noncircular frames. It is not a general warp-formula interpreter. No XML is bundled or read at runtime. Unknown shape numbers, any stored adjustment and multiline WordArt retain straight editable text with a warning. Invalid or excessively large frames decline path construction. Shapes 145 and 147, waves, pour warps and other variants remain unsupported.

INFERENCE, chosen by render comparison rather than storage: rigid glyphs turned at their advance midpoints are a useful approximation for arch and circle. The first attempt used path length divided by `0.7 × character count` and baseline alignment. Arch SSIM improved to 0.982004, but circle dropped to 0.949788, with visibly incorrect fit. A second attempt used `0.6 × character count` and a downward baseline shift of one quarter of the estimated size; arch scored 0.982285 and circle 0.950472. The final approximation also uses existing horizontal character scale 1.1. These constants are explicit visual heuristics, not Publisher properties, font measurements or standard-defined glyph rules. Font size remains capped at 1296 points. Straight WordArt sizing is unchanged.

OBSERVATION about the implementation: the document keeps normal stories, original text and font names; the path is editable and `TextFrameOptions.path` selects existing path composition. Paragraph alignment and font/style, fill, stroke and shadow import code are retained. The six older shape presets and heart are unchanged. No document schema, composer, renderer, font substitution or protected reader was changed. No additional dependency was added.

UNKNOWN: Publisher's exact glyph stretching, tracking, kerning, best-fit rules, adjustment-unit conversion, vertical/reversed text, multiline warp behavior and combinations with decorations or shadows. This implementation does not infer these from scoring JSON or reference images. Reference images were used only to judge the resulting drawing. The mapping from Office preset names to the corresponding ECMA defaults remains an inference, supported by the two observed numbers and the measured renders.

## Reference scoring

OBSERVATION: before is untouched main, after is this change, using the same fonts, fixed scoring settings and nine pinned reference images described in pixel-diff.md. All nine compared image sizes remain 850×1100 with no mismatch. Values below are fractions, not percentages.

| Fixture | SSIM before | SSIM after | Differing fraction before | Differing fraction after |
| --- | ---: | ---: | ---: | ---: |
| group__rotated.pub | 0.999980060 | 0.999980060 | 0.001000000 | 0.001000000 |
| shadow__offset.pub | 0.999993784 | 0.999993784 | 0.000983957 | 0.000983957 |
| shadow__on.pub | 0.999994589 | 0.999994589 | 0.000963636 | 0.000963636 |
| wordart__arch.pub | 0.976310406 | 0.983278719 | 0.006426738 | 0.006359358 |
| wordart__base.pub | 0.980074652 | 0.980074652 | 0.006571123 | 0.006571123 |
| wordart__circle.pub | 0.960595137 | 0.953396313 | 0.009868449 | 0.013665241 |
| wordart__fill-red.pub | 0.979558751 | 0.979558751 | 0.012936898 | 0.012936898 |
| wordart__line-white.pub | 1.000000000 | 1.000000000 | 0.000000000 | 0.000000000 |
| wordart__shadow.pub | 0.979426457 | 0.979426457 | 0.009779679 | 0.009779679 |

OBSERVATION: arch and circle final side-by-side images were inspected, with our rendering on the left. Arch has the expected upward curve; SSIM improves, but it still differs in letter forms and fit. Circle turns the text around the ellipse; its score is worse than the old straight-text baseline and its differing fraction increases. Some letters remain too close or overlap, and their positions and proportions differ visibly. This is path support, not a claim of improved circle fidelity. The other seven reference scores are unchanged.

INFERENCE: unavailable Arial Black and substituted glyph outlines contribute to the differences, alongside this deliberately approximate fit. The images do not isolate how much each factor contributes. Do not use the score to assert a correct parse: sparse white pages, font differences and antialiasing can conceal or exaggerate errors. The blank white-line pair still scores 1.0. Only nine Publisher references exist, six WordArt; other warps and fonts have no reference validation here.

## Reproduce

From the checkout root with the Rust environment active:

```sh
export PUB_CORPUS=/tmp/niftydata/pubsmith-fixtures
export PUB_REAL_CORPUS=/tmp/niftydata/apache-pub
cargo test --locked -p designcraft-pub --test wordart -- --ignored --nocapture
PUB_PIXEL_MODE=publisher cargo test --locked -p designcraft-cli --test pub_pixel_diff pub_pixel_diff -- --ignored --exact --nocapture
# The two recorded runs already exist; use fresh names for new runs.
PUB_PIXEL_MODE=snapshot PUB_PIXEL_RUN=my-wordart-change cargo test --locked -p designcraft-cli --test pub_pixel_diff pub_pixel_diff -- --ignored --exact --nocapture
PUB_PIXEL_MODE=compare PUB_PIXEL_BEFORE=c9-baseline PUB_PIXEL_AFTER=my-wordart-change cargo test --locked -p designcraft-cli --test pub_pixel_diff pub_pixel_diff -- --ignored --exact --nocapture
```

Publisher CSV, report and side-by-side images remain outside the checkout in `/tmp/niftydata/pixel-report/`. Recorded own-output snapshots are `/tmp/niftydata/pixel-snapshots/c9-baseline/` and `c9-after/`. These snapshots compare our output with our output, not with Publisher. Baseline reports and verification logs are preserved in `/tmp/niftydata/c9/`.

## Validation

OBSERVATION: hand-computed tests use a radius-50 semicircle: endpoints `(10,70)` and `(110,70)`, top `(60,20)`, length `50π`, tangent zero at the top and a glyph centre placed there upright. The circle test places its halfway glyph at the right midpoint with a quarter-turn tangent. A 200×100 ellipse test checks the final radial-angle gap: endpoint approximately `(0,50.00005817764173)`. Further tests decline invalid frames, unsupported shapes, adjustments and multiple rows. The ignored binary-fixture test checks seven cases, original names and text, editable stories and the two expected path midpoints.

OBSERVATION: an initial audit harness had a cursor-type compile error; initial fixture-test checks also needed the string insertion method and the existing stroke-none predicate corrected. These were test-harness errors, not input or importer failures. The corrected checks pass. An endpoint review added the ellipse radial-angle test and correction after the initial render trials; final checks and measurements use that correction.

OBSERVATION: final workspace CI passed all seven steps (fmt, dependency check, clippy, tests, assets, layers and WebAssembly): 3,086 tests passed, zero failed, 24 ignored. The five new hand-computed/fallback tests run in normal CI; the external seven-fixture test passed separately. The untouched-main and final full corpus gates both report 142/142 imports, 152/152 objects, 111/152 positions within 1 point, 52/52 exact texts and 111/111 styles. All 142 native save/reopen/edit/PDF cases pass. The real batch remains 9 notes, 10 warnings, 2 refused and 3 damaged, with 31 pages and 280 objects; every file's status/page/object count matches the preserved baseline report. Baseline statuses were rerun on untouched main; the preserved page/object report comes from the same production sources before the pixel runner was added.

OBSERVATION: `c9-baseline` rendered 142 first pages in 1.233 seconds and `c9-after` in 1.164 seconds, excluding compilation/startup. Own-output comparison reports exactly these changes, with no inventory or size mismatch:

| Fixture | Own-output SSIM | Own-output differing fraction |
| --- | ---: | ---: |
| wordart__circle.pub | 0.954743407 | 0.011322374 |
| wordart__arch.pub | 0.972439561 | 0.007449908 |

OBSERVATION: the other 140 PNGs are byte-identical; only WordArt fixtures changed. No Publisher image was used in that self-comparison. The earlier, pre-endpoint-correction after snapshot is archived separately under `/tmp/niftydata/c9/initial-snapshot-after/`; neither baseline was overwritten.

UNKNOWN / not tested: Windows, real-PC fonts, Publisher's own drawing beyond the nine pinned references, interactive desktop editing, later pages, non-default adjustments, other warps and combined curved-text decorations or shadows. Native roundtrip and story editing are tested; that does not establish every desktop interaction or visual effect.
