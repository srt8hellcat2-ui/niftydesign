# Text fitting: specification and frame audit

## Decision

OBSERVATION: decision **(b), no behaviour change**. None of the seven affected frames returns a Text Boolean Properties word (`0x00BF`). Their defined `fFitShapeToText` setting therefore uses its false default. None of the 19 readable Apache files returns that property anywhere in its drawing trees. All observed controlled text-boolean words explicitly turn shape fitting off.

SPEC: the permitted current MS-ODRAW ordinary-text properties define `fFitShapeToText`, not a field named `fFitTextToShape`. Property `0x0086` is named `unused134` and must be ignored; it is not defined as `ScaleText`. Undefined bits must not be assigned an older or guessed shrink-to-fit meaning. UNKNOWN: a default, property id or bit position for the requested `fFitTextToShape` name is not established by the inspected pages. That is not evidence that Publisher has no separate host-specific fitting feature.

INFERENCE: the observed clipping is a fit problem in our substituted-font/layout combination. The stored ordinary-text flags inspected here do not justify automatic frame growth or font shrinkage. This does not prove Publisher showed all of the stored text, or that font substitution alone explains any difference from Publisher. The host's exact original layout remains UNKNOWN.

## Access and scope

OBSERVATION: on 2026-10-25 the requested network check returned `301` with exit status 0. Following the redirect stayed inside the MS-ODRAW directory on learn.microsoft.com and returned HTTP 200. Only relevant MS-ODRAW property, enumeration, property-entry and product-note pages were opened. The specification's contents metadata located these pages; no other document or reader was searched or followed. The page URL inventory and access date are in [SOURCES.md](SOURCES.md#s-text-fitting-specification-and-frame-audit--2026-10-25).

SPEC: §3.2.3 says shape text is host-dependent and that applications do not all support the same text properties. Its link to a different document was not followed. The text-boolean fit bit is a SHOULD-level request to adjust the shape's dimensions, not a prescribed font-size search, growth direction, minimum size or line-breaking algorithm. Product note 58 says Word 97/2000 do not use the bit; it does not establish Publisher-specific handling.

## Defined ordinary-text properties

SPEC: the following is a paraphrase of MS-ODRAW §§2.3.21.1–8, 2.3.21.14–15, 2.4.3 and 2.4.4. Insets are signed EMU quantities, valid from zero through `0x0132F540`; the point conversions below use 12,700 EMU per point.

| Property | Id | Default | Meaning |
| --- | --- | --- | --- |
| lTxid | `0x0080` | `0` | Host-assigned text identifier; not a fitting instruction. |
| dxTextLeft | `0x0081` | `0x00016530` = 91,440 EMU = 7.2 pt | Left internal text margin. |
| dyTextTop | `0x0082` | `0x0000B298` = 45,720 EMU = 3.6 pt | Top internal text margin. |
| dxTextRight | `0x0083` | `0x00016530` = 91,440 EMU = 7.2 pt | Right internal text margin. |
| dyTextBottom | `0x0084` | `0x0000B298` = 45,720 EMU = 3.6 pt | Bottom internal text margin. |
| WrapText | `0x0085` | `msowrapSquare`, value `0` | Continue text on later lines rather than beyond a margin. Value `1` (ByPoints) has equivalent wrapping; value `2` (None) extends a line rather than wrapping; values `3` and `4` are undefined and ignored. |
| unused134 | `0x0086` | No meaningful default given | Undefined and must be ignored. No `ScaleText` semantics are defined here. |
| anchorText | `0x0087` | `msoanchorTop`, value `0` | Suggested text placement relative to its orientation, box and margins, with application-dependent exact positioning; not automatic fitting. |
| unused141 | `0x008D` | No meaningful default given | Undefined and must be ignored, even when a numeric value is present. |
| Text Boolean Properties | `0x00BF` | Defaults apply per flag | Three defined value/use pairs below; the remaining marked unused positions are ignored. |

SPEC: anchor values `0/1/2` suggest top/middle/bottom placement; `3/4/5` add horizontal centering to those placements. Values `6/7/8/9` are baseline variants; the host determines the baseline offset, and the standard recommends their corresponding non-baseline alternatives. This enum is not a rule to grow a box or reduce a font. Property-specific application notes identify supporting products, without supplying Publisher layout behaviour.

### Boolean bits and use masks

SPEC: bit numbers below are numeric bit positions in the parsed 32-bit property value, counting bit zero from the least significant bit. A value bit is used only if its corresponding use bit is set. An absent word, or a clear use bit, selects the documented default rather than the raw value bit.

| Name | Value bit / mask | Use bit / mask | Value default | Effect when used |
| --- | --- | --- | ---: | --- |
| fFitShapeToText | 1 / `0x00000002` | 17 / `0x00020000` | 0 | Request adjustment of the containing shape's dimensions to fit its text. |
| fAutoTextMargin | 3 / `0x00000008` | 19 / `0x00080000` | 0 | Use automatic internal margins; when false, use the four margin properties, with defaults for absent margins. |
| fSelectText | 4 / `0x00000010` | 20 / `0x00100000` | 1 | A click on text should enter text editing; when false, one click selects the shape and a later click edits text. |

SPEC: the corresponding use-bit defaults are zero. Numeric bit positions 0 and 2, with positions 16 and 18 in the other half of the word, are marked unused in this property definition. They do not define `fFitTextToShape`. The separate Shape Boolean Properties (`0x033F`, §2.3.2.12) and Group Shape Boolean Properties (`0x03BF`, §2.3.4.44) are different groups and do not supply a text-fit bit under that name. A large raw boolean word must not be interpreted without its own field definition.

OBSERVATION: the existing parser merges repeated boolean words using their high-half use masks, retaining old values only for flags the newer entry does not use. This audit reads that existing merged output, not a guessed interpretation of raw low bits. The effective-value check is `use_set ? value_set : default`.

INFERENCE, hand-computed checks of that rule: `0x00020002` requests shape fitting; `0x00020000` explicitly disables it; `0x00000002` has the value bit alone and therefore still resolves to the false default. `0x00080008` requests automatic margins, not shape fitting. With no select-text use bit the select-text default remains true even if the raw value bit is clear. These examples were checked in the external audit; they are not claims that the sample frames store those enabled settings.

## Seven affected frames

OBSERVATION: the existing Escher parser was rerun on the pinned `.pub` bytes. All seven are shape type 202. In every row, `0x0085` (WrapText), `0x0086` (unused134), `0x0087` (anchorText) and `0x00BF` are **absent**. No fit use/value bits are returned for these shapes. For the defined OfficeArt settings this means WrapText defaults to Square, anchorText to Top, automatic margins to false and fFitShapeToText to false. The host-dependent placement can still come from other existing importer inputs; for example the main title's bottom alignment is not evidence that `0x0087` is stored.

| File | Item / source sequence | lTxid `0x0080` | Margins `0x0081/82/83/84` (left/top/right/bottom), EMU | `0x008D` raw value | fFitShapeToText |
| --- | --- | ---: | --- | --- | --- |
| `SampleNewsletter.pub` | 18/296 | 6 | (36195, 36195, 36195, 36195) | `0x00011AC6` = 72390 | absent; default false |
| `SampleNewsletter.pub` | 20/297 | 7 | (36195, 36195, 36195, 36195) | `0x00011AC6` = 72390 | absent; default false |
| `SampleNewsletter.pub` | 30/307 | 14 | (36195, 36195, 36195, 36195) | `0x00011AC6` = 72390 | absent; default false |
| `SampleNewsletter.pub` | 160/424 | 75 | (36195, 36195, 36195, 36195) | `0x00011AC6` = 72390 | absent; default false |
| `SampleBrochure.pub` | 30/319 | 8 | (36195, 36195, 36195, 36195) | `0x00011AC6` = 72390 | absent; default false |
| `SampleBrochure.pub` | 72/364 | 20 | (36195, 36195, 36195, 36195) | `0x00011AC6` = 72390 | absent; default false |
| `SampleBrochure.pub` | 82/327 | 25 | (36576, 36576, 36576, 36576) | `0x00011940` = 72000 | absent; default false |

OBSERVATION: the first six frames' four margins are each `0x00008D63` (36,195 EMU, 2.85 pt); brochure sequence 327 uses `0x00008EE0` (36,576 EMU, 2.88 pt) on every side. These are explicit values, so the larger default margins are not substituted. `0x008D` is recorded numerically but is undefined in the current specification and provides no fitting instruction.

UNKNOWN: no raw word or defined flag named fFitTextToShape can be reported from these pages. The undefined numeric bits 2 and 18 are clear in every audited text-boolean word, but clearing an undefined bit is not proof of a documented shrink setting. The audit does not infer a host-specific flag elsewhere in the publication.

### Wider scan

OBSERVATION: all source drawing records, including group members and unsequenced drawing containers, were checked. The 19 readable Apache binaries have 318 parsed drawing records, 280 with source sequence numbers, and 146 imported text-frame items; none returns `0x00BF`. The two refused older-format files and three damaged files are not counted as having been inspected successfully.

| Set | Files | Imported text-frame items | `0x00BF` words present | Defined fFitShapeToText true | Explicit false |
| --- | ---: | ---: | ---: | ---: | ---: |
| Controlled names starting text__ or text2__ | 50 | 51 | 50 | 0 | 50 |
| All controlled fixtures | 142 | 63 | 51 | 0 | 51 |
| All readable Apache files | 19 | 146 | 0 | 0 | 0 |
| SampleNewsletter.pub (included above) | 1 | 64 | 0 | 0 | 0 |
| SampleBrochure.pub (included above) | 1 | 20 | 0 | 0 | 0 |

OBSERVATION: every present word is exactly `0x00020000`, not `0x00020002`: use bit 17 is one and value bit 1 is zero. Within the 50 controlled text-named files, 49 files have that word (the two-box fixture supplies two words); the text-in-oval fixture omits it. The additional controlled word is in `pages__master-text.pub`. The all-controlled text-frame count includes eight WordArt frames, whose separate geometry-text fitting flags are not treated as ordinary text flags. All 439 controlled drawing records were checked, so the zero-enabled result is not dependent on identifying text solely from a particular shape type.

UNKNOWN: a positive frame count for the requested fFitTextToShape name cannot be assigned because the inspected ordinary-text specification does not define that field. The observed count of set numeric bits 2 or 18 in `0x00BF` is zero. This is not a license to label those unused positions as a second fitting feature.

## Decision and proposed options

OBSERVATION: decision (b) leaves frame bounds, text sizes, stories and warnings unchanged. There is no flagged frame-growth case here to balance against the position score, and no verified shrink instruction to implement instead. The before/after corpus confirms unchanged shape/text and picture positions. No production or test code was changed; the conditional implementation and fixture-test work in decision (a) was not applicable.

INFERENCE: the four newsletter headlines and three brochure frames overrun their available space with our current resolved fonts and line layout. The earlier height-only experiment recovered their hidden text, and the fresh unchanged render reproduces the cuts. Neither this specification nor these bytes establishes the original Microsoft-font metrics, Publisher's exact line breaking, or which hidden paragraphs Publisher intended to display.

INFERENCE: options for a separate task, not implemented here:

| Option | Benefit | Risk |
| --- | --- | --- |
| Report overset text, naming the frame and preserving its text | Makes clipping visible without changing geometry or typography | More notices, including intentional overflow and linked-story duplicates; severity could change batch categories. |
| Offer an explicit, optional size reduction for selected overset, unthreaded headlines using measured composition | Could recover missing words while preserving frame bounds | Changes type hierarchy and readability; mixed-size paragraphs, tabs and bylines complicate fitting. This would be a new layout policy, not a stored flag's meaning. |
| Make font substitution visible and support manual font/reflow review | Lets the user select available fonts or legally supplied original fonts | Requires user work; original fonts might still leave differences in line breaking, padding or host-specific layout. |

## Before/after verification

OBSERVATION: the full before/after controlled gate is identical:

| Measure | Before | After |
| --- | --- | --- |
| Files imported | 142/142 | 142/142 |
| Legacy object coverage | 152/152 | 152/152 |
| Shape/text positions within 1 pt | 111 of 111 scored objects | 111 of 111 scored objects |
| Picture references within 1 pt | 12 of 41; 29 unmatched; no matched misses | 12 of 41; 29 unmatched; no matched misses |
| Exact texts | 52/52 | 52/52 |
| Styles | 111/111 | 111/111 |
| Native save/reopen/edit/PDF checks | 142 pass | 142 pass |
| Real file status | 9 notes / 10 warnings / 2 refused / 3 damaged | 9 notes / 10 warnings / 2 refused / 3 damaged |
| Real pages / objects | 31 / 280 | 31 / 280 |

OBSERVATION: every real file's status, page/object counts, notes, warnings and failure/refusal reason are unchanged. Formatting, warnings-denied workspace clippy and all seven CI steps pass: 3,094 passed, zero failed, 24 ignored. This is a documentation-only result, not a clipping fix.

### Publisher-reference scores

OBSERVATION: all nine first-page comparisons were rerun in publisher mode before and after. Every CSV row is identical; all sizes match. These scores do not compare the newsletter or brochure with Publisher. Font differences and antialiasing affect scores, and a good score is not proof of correct reading.

| Fixture | SSIM before | SSIM after | Differing fraction before | Differing fraction after |
| --- | ---: | ---: | ---: | ---: |
| `group__rotated` | 0.999980060 | 0.999980060 | 0.001000000 | 0.001000000 |
| `shadow__offset` | 0.999993784 | 0.999993784 | 0.000983957 | 0.000983957 |
| `shadow__on` | 0.999994589 | 0.999994589 | 0.000963636 | 0.000963636 |
| `wordart__arch` | 0.983892067 | 0.983892067 | 0.006321925 | 0.006321925 |
| `wordart__base` | 0.980074652 | 0.980074652 | 0.006571123 | 0.006571123 |
| `wordart__circle` | 0.960595137 | 0.960595137 | 0.009868449 | 0.009868449 |
| `wordart__fill-red` | 0.979558751 | 0.979558751 | 0.012936898 | 0.012936898 |
| `wordart__line-white` | 1.000000000 | 1.000000000 | 0.000000000 | 0.000000000 |
| `wordart__shadow` | 0.979426457 | 0.979426457 | 0.009779679 | 0.009779679 |

OBSERVATION: the new own-output snapshot run `s-baseline` and run `s-after` each contain 142 first pages. The compare mode reports **142 common fixtures, zero changed, zero added/removed**. The list of changed fixtures is empty; all 142 PNG files are also byte-identical. These are self-regression snapshots, not comparisons with Publisher.

### Newsletter before and after

OBSERVATION: first-page images were rendered separately and inspected:

```sh
designcraft-cli run --in /tmp/niftydata/apache-pub/SampleNewsletter.pub --page 0 --scale 1.5 --export /tmp/niftydata/text-fit/newsletter-before.png
designcraft-cli run --in /tmp/niftydata/apache-pub/SampleNewsletter.pub --page 0 --scale 1.5 --export /tmp/niftydata/text-fit/newsletter-after.png
```

OBSERVATION: both are 893 × 1263 pixels and byte-identical. They show `Living and Learning in`, `What it means to be an Amer-`, `Our Excursion to`, and `Strictly`, with the same missing endings. The Liberty picture, sidebar, reminder box, grey rule and decorative drawings remain visible. No visual improvement is claimed.

OBSERVATION: fresh composition still gives each of the four newsletter headings one placed line and an overset story. Franklin Gothic Book at 43.5 pt resolves to Source Sans 3 Regular for the main title; Rockwell Condensed resolves to DejaVu Serif Condensed for the other three. The brochure frames have four, three and two placed lines respectively and remain overset; their fonts and full stored text are recorded in [headline-clipping.md](headline-clipping.md). The stored words are retained, including hidden byline/extra paragraphs; their intended visibility in Publisher remains UNKNOWN.

## Remaining limits

UNKNOWN: Publisher-specific host properties outside these ordinary OfficeArt fields, original Microsoft-font metrics, exact Publisher fitting/line-breaking behaviour, and whether the original samples intentionally contained overset text. The current specification does not authorize assigning a shrink meaning to unused positions. Windows, real-PC fonts, interactive fitting and Publisher render comparisons for the Apache newsletter/brochure were not tested. The two refused and three damaged inputs remain uninspected as usable publications.

OBSERVATION: all 317 required pinned data files were restored outside the checkout as individual raw data downloads and matched their recorded sizes and SHA-256 values. No source code was downloaded from either fixture repository. Raw parser output, decoded counts, specification page copies, validation logs, scores and renders remain under `/tmp/niftydata/text-fit`; self-snapshots remain under `/tmp/niftydata/pixel-snapshots`. No reference image, input data, specification copy or diagnostic code is committed.
