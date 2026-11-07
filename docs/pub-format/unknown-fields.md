<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Unknown text-formatting fields 0x39, 0x48, 0x30, 0x3C (describer, data-only)

Written 2026-10-19 by the data-only describer. Labels: **OBSERVATION**, **INFERENCE**, **UNKNOWN**, **UNVERIFIED HYPOTHESIS**. Inputs: the documented property-list and Quill formats (`property-lists.md`, `text-store.md` §§7–10) applied to the 142 pubsmith and 24 Apache files listed in `SOURCES.md`; nothing else. "Default" = the second `STSH` section's entry 0 (character) or entry 1 (paragraph); "run" = a record from `FDPC` (character) or `FDPP` (paragraph).

## 1. Summary (what the data shows)

| Field | Where | Kind and values seen | Constant or varying? |
|---|---|---|---|
| Character 0x39 | defaults; Apache runs | kind 0x22; pubsmith: 0x1F018 (= 10 pt in EMU) in the default of **all 142** files, never in a run. Apache: default 0x1F018 in 18 files, 0x1A5AE in `SampleNewsletter.pub`, absent from `Sample98.pub`, `Sample2000.pub`; in runs it takes 16 different values (e.g. 0x37CF8 ×40, 0x3E030 ×16, 0x4A6A0 ×6, 0x284EC ×4) | Constant in pubsmith; varies in Apache runs |
| Character 0x48 | defaults; Apache runs | kind 0x8A, one entry key 0x00. pubsmith: {0:0} in the default of all 142, never in a run. Apache: default {0:0} in 16 files, runs {0:1} (52 runs in 14 files) | Constant in pubsmith; varies in Apache |
| Paragraph 0x30 | defaults; Apache runs | kind 0x12. pubsmith: 0x0005 in the default of all 142, never in a run. Apache: defaults 0x0005 (16 files), 0x8005 (`Sample98.pub`, `Sample2000.pub`), 0x5A05 (`51318.pub`); 48 runs in 3 files, all 0x0005 | Constant in pubsmith; varies by Apache file only in the default |
| Paragraph-run 0x3C | runs only | kind 0x12. pubsmith: only `text__align-justify.pub`, 0x4832, one run. Apache: 5 runs in `SampleBrochure.pub`, 0x0032 | Appears only with justified alignment; value differs between the two sources |

## 2. Observations

**OBSERVATION — pubsmith defaults are byte-identical.** The default character record and default paragraph record are byte-for-byte the same in all 142 pubsmith files (one distinct pair of records). So 0x39 = 0x1F018, 0x48 = {0:0}, 0x30 = 5 are present in every pubsmith file, including the 88 that have no Quill `TEXT` section; the "54 files" Codex count is the files with text. In the default character record the other keys are 0x0C = 0x1F018, 0x24 (font table), 0x44 = {0:0}, 0x58 = {0:1, 1:0, 2:100000}; in the default paragraph record 0x13 = 0x129A8, 0x34 = 0x162362.

**OBSERVATION — 0x39 equals the font-size key 0x0C wherever both occur.** In 240 of 240 character records (defaults and runs, pubsmith defaults and Apache) that contain 0x39, the same record's key 0x0C has the identical value. Every record with 0x39 also has 0x0C. The reverse does not hold: 12 Apache runs have 0x0C and no 0x39 (and the pubsmith size-variant runs, §3, have a changed 0x0C and no 0x39).

**OBSERVATION — 0x48 equals the colour key 0x44 wherever both occur.** In 210 of 210 records containing 0x48, key 0x44 in the same record has the same nested value ({0:0} or {0:1}); every record with 0x48 has 0x44. 0x44 occurs without 0x48 in some records (10 Apache runs; in all pubsmith colour runs).

**OBSERVATION — 0x3C occurs only with alignment 6 (justified).** All six records that have 0x3C (`text__align-justify.pub` and five runs of `SampleBrochure.pub`) have paragraph key 0x04 = 6, and every record in the 166 files with 0x04 = 6 has 0x3C. Records with 0x04 = 0, 1 or 2 (`text__align-center.pub`, `text__align-right.pub`, `text2__p2-center.pub`, five Apache runs) do not. The values differ: 0x4832 (pubsmith) versus 0x0032 (Apache); the high byte 0x48 is the only difference.

**OBSERVATION — 0x30 is not tied to any other paragraph key.** It appears in records with no alignment key, and with alignment 0, 2 and 6; it is absent from the same kinds of records elsewhere (e.g. 397 Apache paragraph runs without it, 41 with it).

**OBSERVATION — producer-linked absence.** `Sample98.pub` and `Sample2000.pub` (Contents signature `E8 AC 22 00`) have the same text as `Sample.pub` (identical 488-unit text) but their default character record has keys 0x0C, 0x24, 0x2E (no 0x39, 0x44, 0x48), and the default paragraph record has 0x30 = 0x8005. `Sample.pub` (`E8 AC 2C 00`) has 0x39, 0x44, 0x48 and 0x30 = 0x0005. This pair differs in the whole writer, not in one setting.

## 3. Controlled pairs (pubsmith) — which pairs differ in these fields

In pubsmith the four fields never appear in a run except 0x3C. Comparing each variant to `text__base.pub` (whose default and runs are as in §1, runs carry only 0x12 and 0x3E language keys):

| Pair | What the pair differs in | Difference in 0x39 / 0x48 / 0x30 / 0x3C |
|---|---|---|
| `text__base` vs `text__align-justify` | paragraph run gains 0x04 = 6 and 0x3C = 0x4832 | **0x3C added**; the other three unchanged |
| `text__base` vs `text__align-center`, `-right` | paragraph run gains 0x04 = 2 / 1 | none |
| `text__base` vs `text__bullets` | paragraph run gains 0x02, 0x03, 0x0C, 0x0D, 0x57 | none |
| `text__base` vs `text__ls-single`, `-double`, `-multiple-1.5`, `-exact-14`, `-exact-30` | paragraph run key 0x34 | none |
| `text__base` vs `text__indent-first-18`, `-left-18`, `-right-18` | paragraph run key 0x0C / 0x0D / 0x0E | none |
| `text__base` vs `text__space-before-12`, `text__space-after-0` | paragraph run 0x12 / 0x13 | none |
| `text__base` vs `text__size-8`, `-12`, `-24`, `-36.5`, `-72`, `text__run-size-word` | character run 0x0C changes (e.g. 24 pt = 0x4A6A0) | none: **0x39 stays at the default 0x1F018 (10 pt) in the default while the run size changes** |
| `text__base` vs `text__color-red`, `-custom`, `text__run-red-word` | character run gains 0x44 = {0:2} and 0x58 | none: **0x48 stays {0:0}** |
| `text__base` vs `text__tracking-200`, `-smallcaps`, `-allcaps`, `-superscript`, `-bold`, `-italic`, `-underline`, `-font-*` | character run keys 0x1F, 0x13, 0x14, 0x0F, 0x02, 0x03, 0x1E, 0x24 | none |

**INFERENCE (from §2 plus these pairs):** no controlled pubsmith pair moves 0x39, 0x48 or 0x30. The Apache files show 0x39 equal to the size and 0x48 equal to the colour, but the pubsmith size and colour variants — which change exactly those quantities — do not change 0x39 or 0x48. So size or colour changes alone do not force these fields; where they appear in Apache runs they copy the neighbouring key.

## 4. Meanings

- **0x39 — meaning UNKNOWN.** The only association is "copies the size value in the same record" (240/240), which is correlation. UNVERIFIED HYPOTHESIS: a size for a separate text class (for example a second script), written alongside 0x0C by some writers/settings. No pair isolates it; confidence: none.
- **0x48 — meaning UNKNOWN.** Same pattern with the colour key 0x44. UNVERIFIED HYPOTHESIS: a colour for a separate text class. Confidence: none.
- **0x30 — meaning UNKNOWN.** Value 0x0005 in all 142 pubsmith defaults and in 64 of the 67 Apache occurrences (206 of 209 overall); other values 0x8005 (2 files, older Contents signature) and 0x5A05 (1 file). Low byte is always 0x05; the high byte is 0x00, 0x80 or 0x5A. No pair isolates it.
- **0x3C — meaning UNKNOWN; association established.** The pair `text__base` / `text__align-justify` adds 0x3C together with alignment 6, so the pair isolates "justified paragraph" as the difference but cannot separate 0x04 from 0x3C. Centre and right alignment add no 0x3C. Combined with the Apache runs (5/5 justified, 0/5 others) this is solid evidence that **0x3C is written whenever alignment is justified**; what its value encodes is unknown (UNVERIFIED HYPOTHESIS: a justification parameter; the different values 0x4832 versus 0x0032 are unexplained). Confidence that it is tied to justification: high; that it has any particular meaning: none.

## 5. Samples that would settle the remaining unknowns

- **0x39 / 0x48:** a pubsmith-style document with East Asian or complex-script text next to Latin text (to test a "second class" size/colour); and the same size change made once through the font-size box and once through "Increase font size" or a character style, to see when 0x39 is written.
- **0x30:** controls changing paragraph options one at a time that are not yet covered (hyphenation, keep-with-next, widow control, tab stops, drop cap, text direction, language), each next to a base file; the high byte (0x80, 0x5A) suggests flag-like data, so toggling each paragraph dialog option separately is the direct test.
- **0x3C:** justified paragraphs with last-line "justify all", with different letter/word spacing settings, and with hyphenation on/off.

## 6. Per-file data

### Apache files (24)

All kinds are as in §1. "—" = absent. Runs list the distinct values.

| File | Quill text | 0x39 default / runs | 0x48 default / runs | 0x30 default / runs | 0x3C |
|---|---|---|---|---|---|
| `51318.pub` | yes | 0x1f018 / 0xd6dee×1 | nested[0:0x0] / — | 0x5a05 / 0x5×2 | — |
| `60685.pub` | yes | 0x1f018 / — | — / — | 0x5 / — | — |
| `LinkAt0And10.pub` | yes | 0x1f018 / — | nested[0:0x0] / nested[0:0x1]×2 | 0x5 / — | — |
| `LinkAt10.pub` | yes | 0x1f018 / — | nested[0:0x0] / nested[0:0x1]×1 | 0x5 / — | — |
| `LinkAt10And20And30.pub` | yes | 0x1f018 / — | nested[0:0x0] / nested[0:0x1]×3 | 0x5 / — | — |
| `LinkAt10And20And30And40.pub` | yes | 0x1f018 / — | nested[0:0x0] / nested[0:0x1]×4 | 0x5 / — | — |
| `LinkAt10Longer.pub` | yes | 0x1f018 / — | nested[0:0x0] / nested[0:0x1]×1 | 0x5 / — | — |
| `LinkAt20.pub` | yes | 0x1f018 / — | nested[0:0x0] / nested[0:0x1]×1 | 0x5 / — | — |
| `Sample.pub` | yes | 0x1f018 / 0x3e030×2, 0x37cf8×5 | nested[0:0x0] / nested[0:0x1]×5 | 0x5 / — | — |
| `Sample2.pub` | yes | 0x1f018 / 0x3e030×2, 0x37cf8×5 | nested[0:0x0] / nested[0:0x1]×5 | 0x5 / — | — |
| `Sample2000.pub` | yes | — / — | — / — | 0x8005 / — | — |
| `Sample2_2010.pub` | yes | 0x1f018 / 0x3e030×2, 0x37cf8×5 | nested[0:0x0] / nested[0:0x1]×5 | 0x5 / — | — |
| `Sample3.pub` | yes | 0x1f018 / 0x3e030×2, 0x37cf8×5 | nested[0:0x0] / nested[0:0x1]×5 | 0x5 / — | — |
| `Sample3_2010.pub` | yes | 0x1f018 / 0x3e030×2, 0x37cf8×5 | nested[0:0x0] / nested[0:0x1]×5 | 0x5 / — | — |
| `Sample4.pub` | yes | 0x1f018 / 0x3e030×2, 0x37cf8×5 | nested[0:0x0] / nested[0:0x1]×5 | 0x5 / — | — |
| `Sample4_2010.pub` | yes | 0x1f018 / 0x3e030×2, 0x37cf8×5 | nested[0:0x0] / nested[0:0x1]×5 | 0x5 / — | — |
| `Sample98.pub` | yes | — / — | — / — | 0x8005 / — | — |
| `SampleBrochure.pub` | yes | 0x1f018 / 0x284ec×4, 0x1e3b1×1, 0x18ce0×1, 0x182f4×1, 0x25350×1, 0x221b4×1 | — / — | — / 0x5×38 | 0x32×5 |
| `SampleNewsletter.pub` | yes | 0x1a5ae / 0x4a6a0×6, 0x2b688×1, 0x86e02×1, 0x1c868×1, 0x2e824×1, 0x1f018×1, 0x13d80×2 | — / — | — / 0x5×8 | — |
| `Sample_2010.pub` | yes | 0x1f018 / 0x3e030×2, 0x37cf8×5 | nested[0:0x0] / nested[0:0x1]×5 | 0x5 / — | — |
| `Simple.pub` | yes | 0x1f018 / — | nested[0:0x0] / — | 0x5 / — | — |
| `clusterfuzz-testcase-minimized-POIHPBFFuzzer-4701121678278656.pub` | no Quill stream | n/a | n/a | n/a | n/a |
| `clusterfuzz-testcase-minimized-POIHPBFFuzzer-4918886059278336.pub` | Quill section directory not readable with the documented rules | n/a | n/a | n/a | n/a |
| `clusterfuzz-testcase-minimized-POIHPBFFuzzer-6325615354773504.pub` | Quill section directory not readable with the documented rules | n/a | n/a | n/a | n/a |

### pubsmith files (142)

"standard" = 0x39 kind 0x22 value 0x1F018, 0x48 kind 0x8A {0:0}, 0x30 kind 0x12 value 5 (byte-identical default records). No pubsmith run carries 0x39, 0x48 or 0x30.

| File | Quill TEXT section | Defaults 0x39 / 0x48 / 0x30 | Runs carrying 0x39 / 0x48 / 0x30 | Run field 0x3C |
|---|---|---|---|---|
| `fill__fill-green.pub` | no | standard | none | none |
| `fill__fill-grey.pub` | no | standard | none | none |
| `fill__fill-orange.pub` | no | standard | none | none |
| `fill__fill-red.pub` | no | standard | none | none |
| `fill__fill-white.pub` | no | standard | none | none |
| `fill__gradient.pub` | no | standard | none | none |
| `fill__no-fill.pub` | no | standard | none | none |
| `fill__scheme-accent1.pub` | no | standard | none | none |
| `fill__transparency-50.pub` | no | standard | none | none |
| `geometry__base.pub` | no | standard | none | none |
| `geometry__flip-h.pub` | no | standard | none | none |
| `geometry__flip-v.pub` | no | standard | none | none |
| `geometry__height-10.pub` | no | standard | none | none |
| `geometry__height-300.pub` | no | standard | none | none |
| `geometry__left-0.pub` | no | standard | none | none |
| `geometry__left-1.pub` | no | standard | none | none |
| `geometry__left-100.pub` | no | standard | none | none |
| `geometry__left-306.pub` | no | standard | none | none |
| `geometry__left-500.pub` | no | standard | none | none |
| `geometry__left-73.pub` | no | standard | none | none |
| `geometry__rot-1.pub` | no | standard | none | none |
| `geometry__rot-180.pub` | no | standard | none | none |
| `geometry__rot-270.pub` | no | standard | none | none |
| `geometry__rot-30.pub` | no | standard | none | none |
| `geometry__rot-359.pub` | no | standard | none | none |
| `geometry__rot-90.pub` | no | standard | none | none |
| `geometry__top-0.pub` | no | standard | none | none |
| `geometry__top-396.pub` | no | standard | none | none |
| `geometry__top-700.pub` | no | standard | none | none |
| `geometry__top-74.pub` | no | standard | none | none |
| `geometry__two-rects-swapped.pub` | no | standard | none | none |
| `geometry__two-rects.pub` | no | standard | none | none |
| `geometry__width-1.pub` | no | standard | none | none |
| `geometry__width-150.pub` | no | standard | none | none |
| `geometry__width-612.pub` | no | standard | none | none |
| `group__rotated.pub` | no | standard | none | none |
| `group__two.pub` | no | standard | none | none |
| `line__color-red.pub` | no | standard | none | none |
| `line__dash-2.pub` | no | standard | none | none |
| `line__dash-4.pub` | no | standard | none | none |
| `line__dash-5.pub` | no | standard | none | none |
| `line__no-line.pub` | no | standard | none | none |
| `line__weight-0.25.pub` | no | standard | none | none |
| `line__weight-1.pub` | no | standard | none | none |
| `line__weight-10.pub` | no | standard | none | none |
| `line__weight-4.pub` | no | standard | none | none |
| `pages__landscape.pub` | no | standard | none | none |
| `pages__master-ignored.pub` | no | standard | none | none |
| `pages__master-rect.pub` | no | standard | none | none |
| `pages__master-text.pub` | yes | standard | none | none |
| `pages__one.pub` | no | standard | none | none |
| `pages__size-5.75x4.75.pub` | no | standard | none | none |
| `pages__three-rect-p3.pub` | no | standard | none | none |
| `pages__two-rect-p2.pub` | no | standard | none | none |
| `pages__two.pub` | no | standard | none | none |
| `picture__base.pub` | no | standard | none | none |
| `picture__border.pub` | no | standard | none | none |
| `picture__crop-all.pub` | no | standard | none | none |
| `picture__crop-left.pub` | no | standard | none | none |
| `picture__crop-top.pub` | no | standard | none | none |
| `picture__flip-h.pub` | no | standard | none | none |
| `picture__moved.pub` | no | standard | none | none |
| `picture__oval-mask.pub` | no | standard | none | none |
| `picture__rotated-30.pub` | no | standard | none | none |
| `picture__stretched.pub` | no | standard | none | none |
| `picture__two.pub` | no | standard | none | none |
| `shadow__offset.pub` | no | standard | none | none |
| `shadow__on.pub` | no | standard | none | none |
| `shapes__curve.pub` | no | standard | none | none |
| `shapes__diamond.pub` | no | standard | none | none |
| `shapes__heart.pub` | no | standard | none | none |
| `shapes__hexagon.pub` | no | standard | none | none |
| `shapes__line-arrow.pub` | no | standard | none | none |
| `shapes__line.pub` | no | standard | none | none |
| `shapes__oval.pub` | no | standard | none | none |
| `shapes__polyline.pub` | no | standard | none | none |
| `shapes__rect.pub` | no | standard | none | none |
| `shapes__rightarrow.pub` | no | standard | none | none |
| `shapes__roundrect.pub` | no | standard | none | none |
| `shapes__star5.pub` | no | standard | none | none |
| `shapes__triangle.pub` | no | standard | none | none |
| `table__2x2.pub` | yes | standard | none | none |
| `table__3x2.pub` | yes | standard | none | none |
| `table__merged.pub` | yes | standard | none | none |
| `text2__base.pub` | yes | standard | none | none |
| `text2__p2-bold.pub` | yes | standard | none | none |
| `text2__p2-center.pub` | yes | standard | none | none |
| `text2__text-in-oval.pub` | yes | standard | none | none |
| `text2__two-boxes.pub` | yes | standard | none | none |
| `text__align-center.pub` | yes | standard | none | none |
| `text__align-justify.pub` | yes | standard | none | 0x12:0x4832 at [0,25) |
| `text__align-right.pub` | yes | standard | none | none |
| `text__allcaps.pub` | yes | standard | none | none |
| `text__base.pub` | yes | standard | none | none |
| `text__bold.pub` | yes | standard | none | none |
| `text__bullets.pub` | yes | standard | none | none |
| `text__color-custom.pub` | yes | standard | none | none |
| `text__color-red.pub` | yes | standard | none | none |
| `text__font-Arial.pub` | yes | standard | none | none |
| `text__font-ArialBlack.pub` | yes | standard | none | none |
| `text__font-CourierNew.pub` | yes | standard | none | none |
| `text__font-TimesNewRoman.pub` | yes | standard | none | none |
| `text__frame-fill-line.pub` | yes | standard | none | none |
| `text__indent-first-18.pub` | yes | standard | none | none |
| `text__indent-left-18.pub` | yes | standard | none | none |
| `text__indent-right-18.pub` | yes | standard | none | none |
| `text__italic.pub` | yes | standard | none | none |
| `text__line-break.pub` | yes | standard | none | none |
| `text__ls-double.pub` | yes | standard | none | none |
| `text__ls-exact-14.pub` | yes | standard | none | none |
| `text__ls-exact-30.pub` | yes | standard | none | none |
| `text__ls-multiple-1.5.pub` | yes | standard | none | none |
| `text__ls-single.pub` | yes | standard | none | none |
| `text__margins-10.pub` | yes | standard | none | none |
| `text__rotated-270.pub` | yes | standard | none | none |
| `text__run-bold-word.pub` | yes | standard | none | none |
| `text__run-font-word.pub` | yes | standard | none | none |
| `text__run-red-word.pub` | yes | standard | none | none |
| `text__run-size-word.pub` | yes | standard | none | none |
| `text__size-12.pub` | yes | standard | none | none |
| `text__size-24.pub` | yes | standard | none | none |
| `text__size-36.5.pub` | yes | standard | none | none |
| `text__size-72.pub` | yes | standard | none | none |
| `text__size-8.pub` | yes | standard | none | none |
| `text__smallcaps.pub` | yes | standard | none | none |
| `text__space-after-0.pub` | yes | standard | none | none |
| `text__space-before-12.pub` | yes | standard | none | none |
| `text__superscript.pub` | yes | standard | none | none |
| `text__text-changed.pub` | yes | standard | none | none |
| `text__text-longer.pub` | yes | standard | none | none |
| `text__tracking-200.pub` | yes | standard | none | none |
| `text__underline.pub` | yes | standard | none | none |
| `text__vanchor-bottom.pub` | yes | standard | none | none |
| `text__vanchor-middle.pub` | yes | standard | none | none |
| `wordart__arch.pub` | no | standard | none | none |
| `wordart__base.pub` | no | standard | none | none |
| `wordart__circle.pub` | no | standard | none | none |
| `wordart__fill-red.pub` | no | standard | none | none |
| `wordart__font.pub` | no | standard | none | none |
| `wordart__line-white.pub` | no | standard | none | none |
| `wordart__shadow.pub` | no | standard | none | none |
| `wordart__text.pub` | no | standard | none | none |

## Builder warning rule C2 (2026-10-19)

OBSERVATION-BASED RULE: in a single character record, omit the 0x39 warning
only when its kind/value exactly equals the unique 0x0C field (kind 0x22), and
omit 0x48 only when its nested kind/value exactly equals the unique 0x44 field
(kind 0x8A). Keep numeric warnings for unequal, missing, differently encoded or
duplicate partners/copies. Field order is irrelevant; inheritance from another
record cannot satisfy the comparison. This uses the observations above and
claims no meaning for either copy field. No imported style attribute changes.

## Builder warning rule C3 (2026-10-19)

OBSERVATION-BASED RULE: omit the paragraph 0x3C warning only when that record's
alignment is imported as 6 (justified). Keep the numeric 0x3C warning when the
condition does not hold, including missing alignment. Both observed values
0x4832 and 0x0032 are covered; this rule claims no meaning for their values.
Default paragraph 0x30 is not covered and still warns. Imported alignment and
all other paragraph attributes are unchanged.
