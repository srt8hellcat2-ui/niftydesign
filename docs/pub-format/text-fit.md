# Text fitting: specification and frame audit

## Decision

OBSERVATION: decision **(b), no automatic fitting change**; the import-time note described below now reports overflow without changing layout. None of the seven affected frames returns a Text Boolean Properties word (`0x00BF`). Their defined `fFitShapeToText` setting therefore uses its false default. None of the 19 readable Apache files returns that property anywhere in its drawing trees. All observed controlled text-boolean words explicitly turn shape fitting off.

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

OBSERVATION (original specification audit): decision (b) left frame bounds, text sizes, stories and warnings unchanged. There is no flagged frame-growth case here to balance against the position score, and no verified shrink instruction to implement instead. The before/after corpus confirms unchanged shape/text and picture positions. No production or test code was changed; the conditional implementation and fixture-test work in decision (a) was not applicable.

INFERENCE: the four newsletter headlines and three brochure frames overrun their available space with our current resolved fonts and line layout. The earlier height-only experiment recovered their hidden text, and the fresh unchanged render reproduces the cuts. Neither this specification nor these bytes establishes the original Microsoft-font metrics, Publisher's exact line breaking, or which hidden paragraphs Publisher intended to display.

INFERENCE: the original audit proposed the options below. Option 1 is now implemented as the lower-severity note described below; options 2 and 3 remain proposals:

| Option | Benefit | Risk |
| --- | --- | --- |
| Report overset text, naming the frame and preserving its text | Makes clipping visible without changing geometry or typography | More notices, including intentional overflow and linked-story duplicates; severity could change batch categories. |
| Offer an explicit, optional size reduction for selected overset, unthreaded headlines using measured composition | Could recover missing words while preserving frame bounds | Changes type hierarchy and readability; mixed-size paragraphs, tabs and bylines complicate fitting. This would be a new layout policy, not a stored flag's meaning. |
| Make font substitution visible and support manual font/reflow review | Lets the user select available fonts or legally supplied original fonts | Requires user work; original fonts might still leave differences in line breaking, padding or host-specific layout. |

## Before/after verification

OBSERVATION: this section records the earlier specification-only investigation. The later import-note implementation and its checks are recorded below.

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

## Import-time overset notes — 2026-10-25

OBSERVATION: option 1 is implemented as a NOTE. `import_pub` in `crates/pub/src/lib.rs` calls `overset::notes` after `Builder::build` and after existing diagnostic classification. `crates/pub/src/overset.rs` composes each nonempty framed story with the existing `compose_story` and default composition options. It reads the document immutably and does not change text, styles, fonts, paths, frames or sizes. Unframed cell/auxiliary stories are handled through their containing story rather than composed as standalone frames.

OBSERVATION: each story id is visited once; a threaded chain produces one note. The last composed frame locates the overflow page, using the document's page name and group transform. Parent frames are named by their parent-page prefix. The preview is the first eight whitespace-separated words, capped at 80 Unicode characters without control characters or line breaks. The exact note template is:

```text
Page {page}, text beginning "{first words}": Some text in this frame does not fit and is hidden (it is still in the file). Publisher may have used a different font, so check this frame against your original.
```

OBSERVATION: these messages are appended directly to `Imported.notes`, never to `Imported.warnings`. Previously, `import_pub` converted generic unsupported-format-field messages into notes, retaining known-loss details as warnings. `open_pub` in `crates/engine/src/cmd/interchange.rs` forwards both arrays separately and stores notes in the document state. The UI's import-notes list and the batch converter's `Outcome::add_warnings` consume the separate notes array. `Outcome::status` uses only failures/warnings to select its category: the CSV's literal `converted OK` can have a populated notes column. The existing “nine with notes” category remains that lower-severity case; no status string or severity policy was changed.

### Note inventory

OBSERVATION: only three files gain overset notes in this environment. No existing note or warning is removed. There are 11 new notes in total, each for a different story. All other 141 controlled fixtures and 17 readable Apache files gain none; two refused and three damaged files retain their outcomes.

| File | Existing notes | Notes after | New overset notes | Category after |
| --- | ---: | ---: | ---: | --- |
| `text__size-72.pub` | 1 | 2 | 1 | with notes, no warnings |
| `SampleBrochure.pub` | 2 | 5 | 3 | with warnings |
| `SampleNewsletter.pub` | 2 | 9 | 7 | with warnings |

OBSERVATION: the new note prefixes below identify every affected story. Each prefix is followed by the exact common message above.

| File | Page and first words |
| --- | --- |
| `text__size-72.pub` | Page 1, text beginning "Hello Example Text world" |
| `SampleBrochure.pub` | Page 1, text beginning "The Great Portraits Exhibition May 23 – June" |
| `SampleBrochure.pub` | Page 2, text beginning "Hindsight Lisa Baker uses mixed media to create" |
| `SampleBrochure.pub` | Page 2, text beginning "The eyes of an angel May 2003 A" |
| `SampleNewsletter.pub` | Page 1, text beginning "This week in class, we have been discussing" |
| `SampleNewsletter.pub` | Page 1, text beginning "What it means to be an American By" |
| `SampleNewsletter.pub` | Page 1, text beginning "Our Excursion to the Zoo" |
| `SampleNewsletter.pub` | Page 1, text beginning "On Monday, we visited the zoo to learn" |
| `SampleNewsletter.pub` | Page 1, text beginning "Living and Learning in Class X" |
| `SampleNewsletter.pub` | Page 1, text beginning "This week’s reminders: Bring in your favorite book" |
| `SampleNewsletter.pub` | Page 1, text beginning "Strictly Jazz" |

OBSERVATION: the newsletter's seven stories include its four headlines, the American-class body chain, the excursion body chain and the reminders. The three brochure stories are the exhibition title/date and the two captions. Counting per story avoids the previous ten newsletter frame-level overset reports becoming ten notes.

### Added import time

OBSERVATION: original-main and changed imports were measured on the same Linux environment using the same pinned bytes, read into memory before timing. Each run used an initial pass followed by 12 warmed passes with alternating traversal order. The timed interval includes only `import_pub`; output serialization and disk reads are outside it. The table summarizes differences between each file's before/after median over those 12 warmed imports. Refused/damaged cases were attempted but excluded from successful-import statistics.

| Set | Successful files | Median added ms | Mean added ms | Largest per-file median increase, ms |
| --- | ---: | ---: | ---: | ---: |
| All controlled | 142 | 0.003 | 0.048 | 0.276 |
| Controlled text__ / text2__ | 50 | 0.078 | 0.091 | 0.248 |
| Readable Apache | 19 | 1.144 | 3.320 | 41.430 |

OBSERVATION: the largest warmed case is SampleNewsletter: median import time rises from 27.523 ms to 68.953 ms, an additional 41.430 ms. Its first measured pass adds 53.818 ms. Initial font loading also produces a 43.260 ms increase for the first controlled text/master fixture. These outliers are reported separately from warmed typical files; the typical added time is below the requested roughly 50 ms limit. This is not a claim that every cold import adds less than 50 ms. First-pass measurements are not one fresh process per file, and Windows timings remain UNKNOWN.

### Checks and limits

OBSERVATION: four ordinary tests cover a zero-height overset frame, a fitting short story, one note for a two-page threaded chain, document immutability and a bounded Unicode preview. The ignored fixture test checks all seven newsletter stories, no overset note for text__base, and the notes-only severity of text__size-72. The explicit fixture run passed. All seven CI steps passed, including formatting and warnings-denied clippy: **3,098 passed, zero failed, 25 ignored**.

OBSERVATION: before/after imports, coverage, shape/text positions, picture positions, texts and styles are identical: **142/142 imports, 152/152 legacy overlap coverage, 111 of 111 scored shape/text positions within 1 pt, 12 of 41 picture references within 1 pt with 29 unmatched, 52/52 exact texts and 111/111 styles**. All 142 native save/reopen/edit/PDF checks pass. All 161 successful imported documents were structurally identical after excluding only creation/modification timestamps, including all text, font, frame and asset data. The added notes are outside the imported Document value.

OBSERVATION: the real batch remains **9 with notes / 10 with warnings / 2 refused / 3 damaged**, **31 pages and 280 objects**. Every file's category, warnings, object/page counts and failure/refusal reason is unchanged. Only SampleBrochure's note count changes from 2 to 5 and SampleNewsletter's from 2 to 9; all other real note arrays are unchanged. Both affected files already had warnings, and neither changes category. The controlled size-72 fixture remains notes-only, with its count increasing from 1 to 2.

OBSERVATION: all nine Publisher-reference CSV rows, including both metrics and image sizes, are identical before/after; the scores equal the nine rows recorded above. Snapshot runs `u-baseline` and `u-after` compare **142 common fixtures, zero changed, zero added/removed**. All 142 PNGs are byte-identical, so the changed-fixture list is empty. This is an own-output regression check, not an additional Publisher comparison. Inputs and all reports, raw before/after timing samples and normalized document comparisons remain outside the repository under `/tmp/niftydata/overset-note` and `/tmp/niftydata/pixel-snapshots`.

UNKNOWN: whether Publisher intended the overset text to be hidden, whether it used different fonts, and the exact layout it originally displayed. This check reports the existing composer's overset flag, not every possible clipped glyph or table-cell clipping. The note is an import-time observation and does not update after later edits. Manual desktop note presentation, Windows, real-PC fonts and fresh-process-per-file timing were not tested. No automatic fit, font replacement change, frame growth or text-size reduction was added.
