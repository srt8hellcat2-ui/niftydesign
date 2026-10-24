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
INFERENCE: the other 186 presets are drawn the same way, so the same small interpreter (formulas plus path pieces) covers all of them. Match each MS-ODRAW shape number to its OOXML name by the English description (for example heart to heart); confirm on the fixtures.

## WordArt text warps (relates to ROADMAP C9)
SPEC: 40 text-warp shapes are defined: textArchDown, textArchDownPour, textArchUp, textArchUpPour, textButton, textButtonPour, textCanDown, textCanUp, textCascadeDown, textCascadeUp, textChevron, textChevronInverted, textCircle, textCirclePour, textCurveDown, textCurveUp, textDeflate, textDeflateBottom, textDeflateInflate, textDeflateInflateDeflate, textDeflateTop, textDoubleWave1, textFadeDown, textFadeLeft, textFadeRight, textFadeUp, textInflate, textInflateBottom, textInflateTop, textPlain, textRingInside, textRingOutside, textSlantDown, textSlantUp, textStop, textTriangle, textTriangleInverted, textWave1, textWave2, textWave4.
SPEC, textArchUp as the example: one adjust value (default 180 degrees, written as 10800000 in 60000ths of a degree) sets where the arc starts; the formulas turn that into a start point on the shape ellipse; the path is a move to that point followed by an arc using half the width and half the height as radii. So the shape gives the PATH that the text follows.
UNKNOWN: how each letter is stretched, turned and spaced along that path is not spelled out in these files. Decide it by comparing renders with the arch and circle fixtures and the scoring-only reference images, and record what stays UNKNOWN.
INFERENCE for the name mapping: TextArchUpCurve (144) is textArchUp, TextArchDownCurve (145) is textArchDown, TextCircleCurve (146) is textCircle, TextButtonCurve (147) is textButton. Confirm on the fixtures before relying on it.
