# Preset shape and WordArt geometry (ECMA-376)

Source: the public ECMA-376 standard, 5th edition (2016), Part 1, downloaded from ecma-international.org. Two files from it were read on 2026-10-24: presetShapeDefinitions.xml (187 preset shapes) and presetTextWarpDefinitions.xml (40 text warps). It is allowed under docs/PUB-AGENTS.md section 2 on the owner words recorded there. It is a public standard, not a .pub reader, and says nothing about how .pub files store anything. This note is a paraphrase in our own words. Labels: SPEC (stated in the files), INFERENCE (our reasoning), UNKNOWN.

## How a preset shape is defined
SPEC: each shape has (a) a list of named formulas (guides) computed from the shape width w, height h and helper values such as hc (half the width, the horizontal centre), vc (vertical centre), hd2, hd3, hd4 (half, third and quarter of the height), and the box edges l, t, r, b; (b) one or more paths built from moveTo, lineTo, cubicBezTo, arcTo and close; (c) a text rectangle and connection points. Formula operators seen so far: "*/ a b c" means a times b divided by c, and "+- a b c" means a plus b minus c. Others (pin, sin, cos, cat2, sat2, and a conditional) appear in the text warps; read their meaning from the standard before using them.

## Heart (MS-ODRAW shape number 74)
SPEC, from the definition named heart:
- dx1 = w x 49 / 48, dx2 = w x 10 / 48.
- x1 = hc - dx1, x2 = hc - dx2, x3 = hc + dx2, x4 = hc + dx1.
- y1 = t - h/3 (above the top edge of the box).
- Path: move to (hc, h/4). Cubic curve with control points (x3, y1) and (x4, h/4), ending at (hc, b). Second cubic curve with control points (x1, h/4) and (x2, y1), ending at (hc, h/4). Close.
- Text rectangle: left w/6, top h/4, right 5w/6, bottom 2h/3.
INFERENCE: named formulas and path pieces provide a shared way to describe presets. This does not establish support for the other 186 definitions: their operators, paths and shape-number mappings need separate checks. The current interpreter supports only the heart.

## WordArt text warps (relates to ROADMAP C9)
SPEC: 40 text-warp shapes are defined: textArchDown, textArchDownPour, textArchUp, textArchUpPour, textButton, textButtonPour, textCanDown, textCanUp, textCascadeDown, textCascadeUp, textChevron, textChevronInverted, textCircle, textCirclePour, textCurveDown, textCurveUp, textDeflate, textDeflateBottom, textDeflateInflate, textDeflateInflateDeflate, textDeflateTop, textDoubleWave1, textFadeDown, textFadeLeft, textFadeRight, textFadeUp, textInflate, textInflateBottom, textInflateTop, textPlain, textRingInside, textRingOutside, textSlantDown, textSlantUp, textStop, textTriangle, textTriangleInverted, textWave1, textWave2, textWave4.
SPEC, textArchUp as the example: one adjust value (default 180 degrees, written as 10800000 in 60000ths of a degree) sets where the arc starts; the formulas turn that into a start point on the shape ellipse; the path is a move to that point followed by an arc using half the width and half the height as radii. So the shape gives the PATH that the text follows.
UNKNOWN: how each letter is stretched, turned and spaced along that path is not spelled out in these files. Decide it by comparing renders with the arch and circle fixtures and the scoring-only reference images, and record what stays UNKNOWN.
INFERENCE for the name mapping: TextArchUpCurve (144) is textArchUp, TextArchDownCurve (145) is textArchDown, TextCircleCurve (146) is textCircle, TextButtonCurve (147) is textButton. Confirm on the fixtures before relying on it.

OBSERVATION (C9): the pinned `wordart__arch.pub` stores 144 and `wordart__circle.pub` stores 146, with no stored adjustments. Their mapping to the corresponding ECMA default paths remains an INFERENCE from the names and measured renders; the two numbers are confirmed by the binary audit. UNKNOWN: fixture confirmation of 145 or 147. Default upward arch now uses existing editable path text fitted from measured font advances. Circle did not meet its reference baseline in the measured trials and retains straight text and a warning, as do other warps. See [wordart.md](wordart.md) for the audit, geometry checks, scores and remaining limits.

## C8 input audit — 2026-10-24

OBSERVATION: read the seven pinned .pub fixture files with the existing `escher::parse` and `import_pub` functions on main `550357e3`. Each contains a linked shape with Contents sequence 293. None has an adjust property in 0x0147..0x0150. This measures stored values and current output; it does not confirm an ECMA preset match or the conversion of adjust units.

| Fixture | Stored shape number | Current drawing |
| --- | ---: | --- |
| shapes__heart.pub | 74 | Rectangle; unsupported-shape warning |
| shapes__star5.pub | 12 | Ten-vertex polygon forming a five-point star; fixed inner-radius ratio 0.381966 |
| shapes__diamond.pub | 4 | Four-vertex diamond polygon |
| shapes__hexagon.pub | 9 | Six-vertex polygon; default horizontal inset one quarter of the frame width |
| shapes__rightarrow.pub | 13 | Seven-vertex arrow polygon; default head base at three quarters of the width, shaft top at one quarter of the height |
| shapes__triangle.pub | 5 | Three-vertex polygon; default apex at the horizontal centre |
| shapes__roundrect.pub | 2 | Rounded rectangle; default corner radius one sixth of the smaller frame dimension |

OBSERVATION about current code: except for the unsupported heart, these are the existing hand-coded `Builder::geometry` branches, not results of an ECMA formula interpreter. The rounded-rectangle item is classified as Rectangle but has rounded path geometry. The existing code reads adjusts for triangle, hexagon, right arrow and rounded rectangle; no sample in this group exercises a non-default adjust. No ECMA adjust conversion is confirmed here. CLI PNG exports of heart, star5 and roundrect were also inspected outside the checkout: the heart is a plain rectangle, the star has five points, and the rounded rectangle has rounded corners. These are baseline renders, not validation against the standard or Publisher.

Earlier access attempt (resolved by the direct archive download below): this environment could not retrieve `https://ecma-international.org/publications-and-standards/standards/ecma-376/` on 2026-10-24: the proxy rejected the connection with HTTP 403 Forbidden. No new standard text or XML was obtained, so no download SHA-256 is available. At that attempt, no alternate source was consulted, no new formulas or paths were implemented, and the text-warp definitions were not opened. The successful direct archive download and heart verification are recorded below.


## C8 heart outline — 2026-10-24

OBSERVATION: downloaded the official fifth-edition Part 1 archive at the direct URL recorded in SOURCES.md. Opened only `presetShapeDefinitions.xml` inside its geometry archive, not the text-warp file, PDF or schemas. The file contains 187 preset definitions. The `heart` entry confirms every outline formula and control point in the heart section above; no numeric correction was needed. Its three additional guides (`il`, `ir`, `ib`) describe the text rectangle, not the outline. Text insets and connector positions are not implemented by this change.

The small interpreter in `crates/pub/src/presets.rs` evaluates named guides in order, then emits a move, cubic curves and a close. For the confirmed arithmetic subset, `*/ a b c` multiplies a and b and divides by c; `+- a b c` adds a and b and subtracts c. These are the two operators already described above and used by the official heart entry. No other operator is implemented or claimed as checked. Width and height are the unrotated frame dimensions; coordinates are translated by the frame origin after evaluation. Negative control coordinates and controls beyond the frame are retained, because they determine the lobes. Existing item transforms still apply rotation and flips.

OBSERVATION: at width 96 and height 120, the guides give dx1=98, dx2=20, x1=-50, x2=28, x3=68, x4=146 and y1=-40. At frame origin (10,20), the outline starts at (58,50), curves through (78,-20) and (156,50) to (58,140), then through (-40,50) and (38,-20) back to (58,50). Unit tests check these hand-computed values, doubled dimensions, invalid arithmetic and invalid frames. Unknown guides, unsupported operators, division by zero and non-finite results return errors. No XML is bundled or read at runtime.

The only newly mapped preset is Heart, MS-ODRAW number 74, as recorded in odraw-geometry.md and confirmed by the pinned `shapes__heart.pub` audit. It now becomes an editable closed path with two cubic curves. If its outline cannot be evaluated, the rectangle fallback gives a plain warning. Other unmatched shape numbers keep their previous behavior and warnings. Star5, diamond, hexagon, right arrow, triangle and rounded rectangle remain hand-coded and unchanged; no measured difference from the standard was established for them.

UNKNOWN: matching Publisher's own drawing has not been tested. The standard outline and the fixture's stored shape number establish the chosen outline, not pixel agreement with Publisher. Windows and non-default preset adjustments were not tested. WordArt is outside C8 and its definition file was not opened in this task.

## AE fallback census — 2026-10-31

OBSERVATION: audited main `ce679520` using the existing compound-file reader, Escher parser and public importer on all 142 controlled and 24 real binaries. The restore check verified all 317 pinned input sizes and hashes. There are 161 successful imports: 142 controlled and 19 real. Two older files are refused and three damaged containers cannot be inventoried. No other reader or new format source was used. The source list and hashes are the existing publisher-fixtures.csv manifest; this census reads no private reference files.

OBSERVATION: there are **zero ordinary preset rectangle fallbacks**, zero unsupported-preset warnings and zero failed-freeform rectangle warnings in these successful imports. Every stored ordinary preset number encountered has an existing drawing branch. This is a count of fallback behavior, not proof of outline fidelity. In particular, the rounded rectangle has Rectangle as its item category but has a rounded path; that is not a fallback.

INFERENCE: no shape meets the requested combination of an unimplemented ordinary preset and an existing fixture. Therefore no geometry, adjustment conversion, stored metadata, positions, fills, outlines, fonts or WordArt was changed. No new code tests were added because there is no implementation to validate; existing tests were not weakened or deleted. The generic unsupported-shape warning remains necessary for other inputs. README and known-limit wording remain true and are unchanged.

### Fallback priority table

OBSERVATION: the ordinary-preset table is empty. The separate WordArt exception is listed to make the scope explicit.

| Shape number | Name from existing MS-ODRAW notes | Files | Occurrences | Default ECMA match without adjustment guesswork | Action |
| --- | --- | ---: | ---: | --- | --- |
| — | Ordinary preset fallback | 0 | 0 | No candidate to check | No implementation |
| 146 | TextCircleCurve | 1 | 1 | INFERENCE: textCircle baseline, not glyph fitting | WordArt excluded; stays straight |

OBSERVATION: `wordart__circle.pub`, sequence 293, stores 146 without adjust properties. It gives exactly: “WordArt shapes (arches, circles, waves) were imported as straight text.” It is the only encountered non-plain warp still drawn straight. `wordart__arch.pub` stores 144 and already has an editable curved baseline with the warning “Curved WordArt uses measured font advances; its appearance can differ from Publisher.” Seven type-136 WordArt records are plain text, not missing ordinary outlines. See wordart.md for the measured circle trial and UNKNOWN letter fitting. Tables (201) are intentional table frames, not ordinary preset fallback candidates.

### All stored drawing types

OBSERVATION: counts below include every parsed drawing record in successful files, including records not linked to a rendered page. Group containers are counted separately. Raw record counts must not be confused with imported object counts or scored positions. File counts mean distinct input files, including page and parent records. Names for ordinary supported types describe the existing geometry branches; no new ECMA equivalence or adjustment-unit conversion is asserted.

| Number | Existing name or role | Files | Records | Handling today |
| ---: | --- | ---: | ---: | --- |
| 0 | NotPrimitive | 8 | 23 | Two editable freeform paths; 21 group containers |
| 1 | Rectangle | 161 | 311 | Rectangle |
| 2 | RoundRectangle | 1 | 1 | Rounded path; Rectangle category |
| 3 | Ellipse | 3 | 3 | Oval |
| 4 | Diamond | 1 | 1 | Four-point polygon |
| 5 | IsoscelesTriangle | 1 | 1 | Triangle polygon |
| 6 | RightTriangle | 3 | 13 | Right triangle polygon |
| 9 | Hexagon | 1 | 1 | Six-point polygon |
| 12 | Star | 1 | 1 | Five-point star polygon |
| 13 | Arrow | 1 | 1 | Right arrow polygon |
| 20 | Line | 161 | 192 | Line |
| 74 | Heart | 1 | 1 | Two cubic curves; editable path |
| 136 | TextPlainText | 7 | 7 | Plain editable WordArt text |
| 144 | TextArchUpCurve | 1 | 1 | Editable curved text |
| 146 | TextCircleCurve | 1 | 1 | Straight text; warp warning |
| 201 | HostControl / table records | 11 | 11 | Table frame, not ordinary outline |
| 202 | TextBox | 69 | 188 | Rectangular text frame |

UNKNOWN: no newly encountered fallback exists to compare with presetShapeDefinitions.xml. The earlier heart definition remains the only interpreter mapping; other existing polygon branches were not rewritten. A default ECMA outline cannot establish Publisher adjustment units or guarantee the appearance of an unrepresented preset.

### Per-file audit

OBSERVATION: each successful file below has zero ordinary-preset fallbacks and no corresponding warning. The type census shows number × record count, including group containers and unused drawing records. These binary facts were collected before reading scoring results. “Unavailable” means no claim about hidden shape records.

| Set | File | Stored types (number × count) | Ordinary fallback count |
| --- | --- | --- | ---: |
| controlled | `fill__fill-green.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `fill__fill-grey.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `fill__fill-orange.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `fill__fill-red.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `fill__fill-white.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `fill__gradient.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `fill__no-fill.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `fill__scheme-accent1.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `fill__transparency-50.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `geometry__base.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `geometry__flip-h.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `geometry__flip-v.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `geometry__height-10.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `geometry__height-300.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `geometry__left-0.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `geometry__left-1.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `geometry__left-100.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `geometry__left-306.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `geometry__left-500.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `geometry__left-73.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `geometry__rot-1.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `geometry__rot-180.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `geometry__rot-270.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `geometry__rot-30.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `geometry__rot-359.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `geometry__rot-90.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `geometry__top-0.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `geometry__top-396.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `geometry__top-700.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `geometry__top-74.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `geometry__two-rects-swapped.pub` | 1 × 3, 20 × 1 | 0 |
| controlled | `geometry__two-rects.pub` | 1 × 3, 20 × 1 | 0 |
| controlled | `geometry__width-1.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `geometry__width-150.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `geometry__width-612.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `group__rotated.pub` | 0 × 1, 1 × 3, 20 × 1 | 0 |
| controlled | `group__two.pub` | 0 × 1, 1 × 3, 20 × 1 | 0 |
| controlled | `line__color-red.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `line__dash-2.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `line__dash-4.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `line__dash-5.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `line__no-line.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `line__weight-0.25.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `line__weight-1.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `line__weight-10.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `line__weight-4.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `pages__landscape.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `pages__master-ignored.pub` | 1 × 3, 20 × 1 | 0 |
| controlled | `pages__master-rect.pub` | 1 × 3, 20 × 1 | 0 |
| controlled | `pages__master-text.pub` | 1 × 2, 20 × 1, 202 × 1 | 0 |
| controlled | `pages__one.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `pages__size-5.75x4.75.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `pages__three-rect-p3.pub` | 1 × 3, 20 × 1 | 0 |
| controlled | `pages__two-rect-p2.pub` | 1 × 3, 20 × 1 | 0 |
| controlled | `pages__two.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `picture__base.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `picture__border.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `picture__crop-all.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `picture__crop-left.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `picture__crop-top.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `picture__flip-h.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `picture__moved.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `picture__oval-mask.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `picture__rotated-30.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `picture__stretched.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `picture__two.pub` | 1 × 3, 20 × 1 | 0 |
| controlled | `shadow__offset.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `shadow__on.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `shapes__curve.pub` | 0 × 1, 1 × 1, 20 × 1 | 0 |
| controlled | `shapes__diamond.pub` | 1 × 1, 4 × 1, 20 × 1 | 0 |
| controlled | `shapes__heart.pub` | 1 × 1, 20 × 1, 74 × 1 | 0 |
| controlled | `shapes__hexagon.pub` | 1 × 1, 9 × 1, 20 × 1 | 0 |
| controlled | `shapes__line-arrow.pub` | 1 × 1, 20 × 2 | 0 |
| controlled | `shapes__line.pub` | 1 × 1, 20 × 2 | 0 |
| controlled | `shapes__oval.pub` | 1 × 1, 3 × 1, 20 × 1 | 0 |
| controlled | `shapes__polyline.pub` | 0 × 1, 1 × 1, 20 × 1 | 0 |
| controlled | `shapes__rect.pub` | 1 × 2, 20 × 1 | 0 |
| controlled | `shapes__rightarrow.pub` | 1 × 1, 13 × 1, 20 × 1 | 0 |
| controlled | `shapes__roundrect.pub` | 1 × 1, 2 × 1, 20 × 1 | 0 |
| controlled | `shapes__star5.pub` | 1 × 1, 12 × 1, 20 × 1 | 0 |
| controlled | `shapes__triangle.pub` | 1 × 1, 5 × 1, 20 × 1 | 0 |
| controlled | `table__2x2.pub` | 1 × 1, 20 × 1, 201 × 1 | 0 |
| controlled | `table__3x2.pub` | 1 × 1, 20 × 1, 201 × 1 | 0 |
| controlled | `table__merged.pub` | 1 × 1, 20 × 1, 201 × 1 | 0 |
| controlled | `text2__base.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text2__p2-bold.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text2__p2-center.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text2__text-in-oval.pub` | 1 × 1, 3 × 1, 20 × 1 | 0 |
| controlled | `text2__two-boxes.pub` | 1 × 1, 20 × 1, 202 × 2 | 0 |
| controlled | `text__align-center.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__align-justify.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__align-right.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__allcaps.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__base.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__bold.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__bullets.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__color-custom.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__color-red.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__font-Arial.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__font-ArialBlack.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__font-CourierNew.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__font-TimesNewRoman.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__frame-fill-line.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__indent-first-18.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__indent-left-18.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__indent-right-18.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__italic.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__line-break.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__ls-double.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__ls-exact-14.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__ls-exact-30.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__ls-multiple-1.5.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__ls-single.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__margins-10.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__rotated-270.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__run-bold-word.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__run-font-word.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__run-red-word.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__run-size-word.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__size-12.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__size-24.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__size-36.5.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__size-72.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__size-8.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__smallcaps.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__space-after-0.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__space-before-12.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__superscript.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__text-changed.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__text-longer.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__tracking-200.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__underline.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__vanchor-bottom.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `text__vanchor-middle.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| controlled | `wordart__arch.pub` | 1 × 1, 20 × 1, 144 × 1 | 0 |
| controlled | `wordart__base.pub` | 1 × 1, 20 × 1, 136 × 1 | 0 |
| controlled | `wordart__circle.pub` | 1 × 1, 20 × 1, 146 × 1 | 0 |
| controlled | `wordart__fill-red.pub` | 1 × 1, 20 × 1, 136 × 1 | 0 |
| controlled | `wordart__font.pub` | 1 × 1, 20 × 1, 136 × 1 | 0 |
| controlled | `wordart__line-white.pub` | 1 × 1, 20 × 1, 136 × 1 | 0 |
| controlled | `wordart__shadow.pub` | 1 × 1, 20 × 1, 136 × 1 | 0 |
| controlled | `wordart__text.pub` | 1 × 1, 20 × 1, 136 × 1 | 0 |
| real | `51318.pub` | 0 × 3, 1 × 7, 6 × 2, 20 × 5, 202 × 2 | 0 |
| real | `60685.pub` | 0 × 1, 1 × 27, 20 × 13, 202 × 4 | 0 |
| real | `LinkAt0And10.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| real | `LinkAt10.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| real | `LinkAt10And20And30.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| real | `LinkAt10And20And30And40.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| real | `LinkAt10Longer.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| real | `LinkAt20.pub` | 1 × 1, 20 × 1, 202 × 1 | 0 |
| real | `Sample.pub` | 1 × 2, 20 × 1, 201 × 1, 202 × 5 | 0 |
| real | `Sample2.pub` | 1 × 2, 20 × 1, 201 × 1, 202 × 5 | 0 |
| real | `Sample2000.pub` | Unavailable | UNKNOWN |
| real | `Sample2_2010.pub` | 1 × 2, 20 × 1, 201 × 1, 202 × 5 | 0 |
| real | `Sample3.pub` | 1 × 2, 20 × 1, 201 × 1, 202 × 5 | 0 |
| real | `Sample3_2010.pub` | 1 × 2, 20 × 1, 201 × 1, 202 × 5 | 0 |
| real | `Sample4.pub` | 1 × 2, 20 × 1, 201 × 1, 202 × 5 | 0 |
| real | `Sample4_2010.pub` | 1 × 2, 20 × 1, 201 × 1, 202 × 5 | 0 |
| real | `Sample98.pub` | Unavailable | UNKNOWN |
| real | `SampleBrochure.pub` | 0 × 6, 1 × 19, 6 × 4, 20 × 10, 202 × 20 | 0 |
| real | `SampleNewsletter.pub` | 0 × 9, 1 × 15, 3 × 1, 6 × 7, 20 × 5, 136 × 1, 202 × 63 | 0 |
| real | `Sample_2010.pub` | 1 × 2, 20 × 1, 201 × 1, 202 × 5 | 0 |
| real | `Simple.pub` | 1 × 1, 20 × 1, 202 × 2 | 0 |
| real | `clusterfuzz-testcase-minimized-POIHPBFFuzzer-4701121678278656.pub` | Unavailable | UNKNOWN |
| real | `clusterfuzz-testcase-minimized-POIHPBFFuzzer-4918886059278336.pub` | Unavailable | UNKNOWN |
| real | `clusterfuzz-testcase-minimized-POIHPBFFuzzer-6325615354773504.pub` | Unavailable | UNKNOWN |

OBSERVATION: unavailable-file diagnostics:

- `Sample2000.pub`: unsupported Publisher file: Publisher 97/98/2000 files are not supported yet; open and save the file in Publisher 2002 or later.
- `Sample98.pub`: unsupported Publisher file: Publisher 97/98/2000 files are not supported yet; open and save the file in Publisher 2002 or later.
- `clusterfuzz-testcase-minimized-POIHPBFFuzzer-4701121678278656.pub`: damaged Publisher file: compound file: Malformed FAT (sector 129 pointed to twice).
- `clusterfuzz-testcase-minimized-POIHPBFFuzzer-4918886059278336.pub`: damaged Publisher file: compound file: Malformed FAT (FAT has 244 entries, but file has only 64 sectors).
- `clusterfuzz-testcase-minimized-POIHPBFFuzzer-6325615354773504.pub`: damaged Publisher file: compound file: Invalid CFB byte order mark (expected 0xFFFE, found 0xFFFA).

### Scoring and limits

OBSERVATION: untouched-main gate: 142/142 imports, 152/152 objects, 111/111 scored positions within 1 pt, 52/52 exact texts, 111/111 styles; picture positions 12/41 matched passes, zero measured misses, 29 unmatched. All 142 native save/reopen/edit/PDF checks passed. The real batch passed its exact expected classification: 9 notes, 10 warnings, 2 refused, 3 damaged. Snapshot ae-baseline contains all 142 first pages.

OBSERVATION: the nine scoring references below have matching 850 × 1100 comparison dimensions. Their untouched-main values equal the last merged WordArt audit; the older arch number in pixel-diff.md predates the improved arch. No shape fixture has a Publisher pixel reference in this nine-image set.

| Reference | Baseline SSIM | Baseline differing fraction |
| --- | ---: | ---: |
| `group__rotated.pub` | 0.999980060 | 0.001000000 |
| `shadow__offset.pub` | 0.999993784 | 0.000983957 |
| `shadow__on.pub` | 0.999994589 | 0.000963636 |
| `wordart__arch.pub` | 0.983892067 | 0.006321925 |
| `wordart__base.pub` | 0.980074652 | 0.006571123 |
| `wordart__circle.pub` | 0.960595137 | 0.009868449 |
| `wordart__fill-red.pub` | 0.979558751 | 0.012936898 |
| `wordart__line-white.pub` | 1.000000000 | 0.000000000 |
| `wordart__shadow.pub` | 0.979426457 | 0.009779679 |

OBSERVATION: no fixture was changed by an implementation, so there is no changed-fixture corpus or by-eye comparison to report. No new visual match is claimed. Snapshot ae-after versus ae-baseline compares 142 common fixtures: zero changed, zero added or removed. The nine final pixel scores are exactly the baseline values in the table above (both SSIM and differing fraction); every corpus score and real-file classification is unchanged.

UNKNOWN: which unsupported ordinary presets are common in user documents; this small corpus contains none. To establish a next implementation, supply one default-only file per currently unsupported ordinary preset, with its original rendering for scoring, followed by separate files changing one adjustment at a time. Separate trapezoid, pentagon, left-arrow and up-arrow samples would be useful requests, not frequency claims or verified ECMA mappings.

UNKNOWN: Windows execution, real-PC fonts, Publisher’s own fonts, non-default outline fidelity, and contents of refused or damaged files were not tested by this census. Zero fallback warnings does not prove a perfect copy.

OBSERVATION: the existing hand-computed heart arithmetic/control-point tests and warned unknown-type/invalid-heart regression remain unchanged. The separate pinned seven-preset fixture test was run explicitly and passed, including the heart, star, diamond, hexagon, right arrow, triangle and rounded rectangle. This is regression coverage of existing behavior, not evidence for any unimplemented outline.

OBSERVATION: final verification passed fmt, workspace clippy with warnings denied, and all seven cargo xtask ci steps (fmt, pdf-deps, clippy, test, assets, layers, wasm). No test binary failed to launch and no temporary-file failure required a sequential rerun. The full corpus, 142 round trips, exact real-batch classification, nine-reference pixel run and 142-page snapshot comparison all passed with unchanged numbers. The branch was rebased on origin/main before the final push. No product code, protected reader, asset or test was changed.
