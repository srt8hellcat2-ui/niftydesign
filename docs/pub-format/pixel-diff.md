# Pixel scoring baseline

Publisher scoring and own-output regression are separate modes. Both are optional and have no visual pass/fail criteria. The importer and renderer are unchanged. The runner is an ignored integration test in the existing CLI package, which already has the needed importer, renderer and PNG APIs; no dependency was added. Normal CI runs only its small metric tests.

## Run

From the checkout root, with the Rust environment active:

```sh
export PUB_CORPUS=/tmp/niftydata/pubsmith-fixtures
# Nine Publisher comparisons: CSV, markdown and nine images, ours on the left.
PUB_PIXEL_MODE=publisher cargo test --locked -p designcraft-cli --test pub_pixel_diff pub_pixel_diff -- --ignored --exact --nocapture
# All 142 first pages: choose a new run name; existing runs are never overwritten.
PUB_PIXEL_MODE=snapshot PUB_PIXEL_RUN=my-change cargo test --locked -p designcraft-cli --test pub_pixel_diff pub_pixel_diff -- --ignored --exact --nocapture
# Our output against our output; list changed fixtures, largest change first.
PUB_PIXEL_MODE=compare PUB_PIXEL_BEFORE=main-baseline PUB_PIXEL_AFTER=my-change cargo test --locked -p designcraft-cli --test pub_pixel_diff pub_pixel_diff -- --ignored --exact --nocapture
```

Publisher outputs: `/tmp/niftydata/pixel-report/scores.csv`, `report.md`, and `<fixture>-side-by-side.png`. Rerunning replaces those generated reports. Snapshots: `/tmp/niftydata/pixel-snapshots/<run-name>/<fixture>.png`, plus a manifest recording revision, platform, scale, inventory and elapsed seconds. A partial run without its manifest cannot be compared.

Snapshots use **1 px/pt (72 dpi)**, first page only, white paper and printing content without screen overlays. Compare mode lists decoded RGB changes even below the metric's pixel cutoff, plus dimension and inventory changes. It sorts by lowest SSIM, highest differing fraction, then name. Keep fonts, inputs and environment fixed to attribute differences to code. **These self-regression snapshots are not Publisher references.** Unchanged output can preserve existing mistakes.

## Metrics

The nine PNGs are 2550×3300 and declare 11,811 pixels/meter on both axes, approximately 300 dpi. A 3×3 box reduction makes each 850×1100. Our renderer uses one third the declared physical density, uniformly on both axes, about 100 dpi. Missing density is explicitly recorded as assumed 300 dpi; non-square density is flagged. There is no stretching, registration search or content crop. Size mismatches compare only the top-left overlap; excluded pixels are not penalized, so inspect sizes too.

Images are composited on white. Grayscale is 0.2126R + 0.7152G + 0.0722B on sRGB bytes, without gamma linearization. SSIM uses non-overlapping 8×8 box windows, population variance/covariance, K1=0.01, K2=0.03, L=255; partial windows are weighted by area. Opposite patterns can score negatively. Differing fraction counts pixels whose maximum RGB-channel difference exceeds **8/255**. That cutoff defines a metric, not acceptance. Category means are unweighted across fixtures.

## Nine-reference baseline

OBSERVATION: production source is main `19602639df11f6b0e37e9fa821053ff22e39e841`, with font fallback merged. All nine sizes match at 850×1100. All side-by-side images were inspected for scoring only.

| Category | Count | Mean SSIM | Mean differing fraction |
| --- | ---: | ---: | ---: |
| tables | 0 | — | — |
| text | 0 | — | — |
| wordart | 6 | 0.979328 | 0.007597 |
| fills | 0 | — | — |
| lines | 0 | — | — |
| shadows | 2 | 0.999994 | 0.000974 |
| pictures | 0 | — | — |
| pages | 0 | — | — |
| groups | 1 | 0.999980 | 0.001000 |
| geometry | 0 | — | — |
| shapes | 0 | — | — |

## References, lowest SSIM first

| Fixture | SSIM | Differing fraction | Size mismatch |
| --- | ---: | ---: | --- |
| wordart__circle.pub | 0.960595 | 0.009868 | no |
| wordart__arch.pub | 0.976310 | 0.006427 | no |
| wordart__shadow.pub | 0.979426 | 0.009780 | no |
| wordart__fill-red.pub | 0.979559 | 0.012937 | no |
| wordart__base.pub | 0.980075 | 0.006571 | no |
| group__rotated.pub | 0.999980 | 0.001000 | no |
| shadow__offset.pub | 0.999994 | 0.000984 | no |
| shadow__on.pub | 0.999995 | 0.000964 | no |
| wordart__line-white.pub | 1.000000 | 0.000000 | no |

## Reading and limits

INFERENCE: WordArt is the weakest measured category because our straight lettering differs from the arch/circle reference outlines. Letter size, position and substituted fonts can also contribute; the images do not isolate one cause or explain storage. The group and shadow pairs look close, with edge differences consistent with rasterization and antialiasing.

OBSERVATION: the white-line WordArt pair is blank on white and scores 1.0. White margins dominate these sparse pages, so even obvious mismatches can score highly. **A good score is not proof of correct reading.** Font substitutions and metrics, antialiasing, color handling, reduction and alignment all affect scores. Only nine Publisher references exist here, six WordArt; zero-count categories are unmeasured. References and truth JSON are scoring oracles only, never inputs for inferring storage.

## Own baseline and checks

OBSERVATION: `main-baseline` contains all **142** first pages, rendered in **1.079 seconds**, excluding compilation/startup, using unchanged main production code plus this harness. Comparing `main-repeat` against it found zero changed fixtures. Temporarily replacing only the repeated run's green-fill PNG with its own red-fill PNG reported exactly `fill__fill-green.pub` changed: SSIM 0.988682460, differing fraction 0.020507361. The original was restored; the baseline was never altered.

Hand-computed tests cover identity, black/white SSIM, negative covariance, RGB cutoffs, partial boxes/windows, alpha, physical density and size mismatches. An initial validator rejected decimal fixture names; this was corrected and tested.

Untouched-main gate: 142/142 imports, 152/152 objects, 111/152 positions within 1 pt, 52/52 texts and 111/111 styles; 142 native save/reopen/edit/PDF passes. Real batch: 9 notes, 10 warnings, 2 refused, 3 damaged; 31 pages, 280 objects. Final results match every baseline number and every real file's status/page/object count. All seven workspace CI steps passed: 3,081 tests passed, zero failed, 23 ignored; external corpus, roundtrip, real-batch, reference-scoring and snapshot-compare checks passed separately.

Not tested: Windows, real-PC fonts, other Publisher references, later pages, physical printing or browser execution. Timing describes this Linux environment. No input data or generated images are committed, and no protected importer reader changed.
