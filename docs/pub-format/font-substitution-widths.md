# Font substitution by width

## Before-change audit

OBSERVATION: this audit uses main `4b2bd7a`, all 142 controlled and 24 Apache binaries at their recorded pinned hashes, the existing Quill parser's font list and imported text formatting. Refused/damaged files yield no successful current-family font list. No original Microsoft font file or metrics were read. Candidate class/width/weight below comes from an available substitute's OS/2 metadata and existing classifier, not from the missing original. Uses counts count formatting entries, not unique files. Original declarations without an imported use are included separately from actual text requests.

UNKNOWN: every missing original's numeric width class, measured advances and exact classification are unverified. Name hints below are INFERENCE; candidate metadata and literal name words are OBSERVATION. Existing special-case family classifications are substitution policy, not verified original metrics.

| Missing declaration | Imported formatting uses | Before substitute / style | Candidate class / width | Original name evidence | After substitute / style / width |
| --- | ---: | --- | --- | --- | --- |
| Ahamedey | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| Angsana New | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| Arial | 44 | Liberation Sans / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Liberation Sans / Regular / 5 |
| Arial Black | 9 | Open Sans / Extrabold | SansSerif / 5 | OBSERVATION: name contains black; INFERENCE: type/width/weight hint only | Open Sans / Extrabold / 5 |
| Arial Narrow | 1 | DejaVu Sans / Condensed | SansSerif / 4 | OBSERVATION: name contains narrow; INFERENCE: type/width/weight hint only | Barlow Condensed / Regular / 3 |
| Batang | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| BrowalliaUPC | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| Calibri | 69 | Liberation Sans / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Liberation Sans / Regular / 5 |
| Cambria | 0 | Liberation Serif / Regular | Serif / 5 | UNKNOWN: type/width not established from name alone | Liberation Serif / Regular / 5 |
| Courier New | 1 | Liberation Mono / Regular | Monospaced / 5 | OBSERVATION: name contains courier; INFERENCE: type/width/weight hint only | Liberation Mono / Regular / 5 |
| DaunPenh | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| DokChampa | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| Elephant | 1 | Source Serif 4 / Regular | Serif / 5 | UNKNOWN: type/width not established from name alone | Source Serif 4 / Regular / 5 |
| Estrangelo Edessa | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| EucrosiaUPC | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| Euphemia Regular CAS | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| Franklin Gothic Book | 27 | Source Sans 3 / Regular | SansSerif / 5 | OBSERVATION: name contains gothic; INFERENCE: type/width/weight hint only | Source Sans 3 / Regular / 5 |
| Franklin Gothic Demi Cond | 0 | Source Sans 3 / Regular | SansSerif / 5 | OBSERVATION: name contains gothic, cond; INFERENCE: type/width/weight hint only | Barlow Condensed / Regular / 3 |
| Franklin Gothic Heavy | 0 | Source Sans 3 / Regular | SansSerif / 5 | OBSERVATION: name contains gothic, heavy; INFERENCE: type/width/weight hint only | Source Sans 3 / Regular / 5 |
| Franklin Gothic Medium Cond | 0 | Source Sans 3 / Regular | SansSerif / 5 | OBSERVATION: name contains gothic, cond; INFERENCE: type/width/weight hint only | Barlow Condensed / Regular / 3 |
| Garamond | 28 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| Gautami | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| Georgia | 1 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| Gill Sans MT | 5 | Source Sans 3 / Regular | SansSerif / 5 | OBSERVATION: name contains sans; INFERENCE: type/width/weight hint only | Source Sans 3 / Regular / 5 |
| Iskoola Pota | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| Kalinga | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| Kartika | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| Latha | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| MS Mincho | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| MV Boli | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| Malgun Gothic | 0 | Source Sans 3 / Regular | SansSerif / 5 | OBSERVATION: name contains gothic; INFERENCE: type/width/weight hint only | Source Sans 3 / Regular / 5 |
| Mangal | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| Microsoft Himalaya | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| Microsoft Yi Baiti | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| Mongolian Baiti | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| Nyala | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| PMingLiU | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| Plantagenet Cherokee | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| Raavi | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| Rockwell Condensed | 19 | DejaVu Serif / Condensed | SlabSerif / 4 | OBSERVATION: name contains condensed, cond; INFERENCE: type/width/weight hint only | Nifty Narrow Serif / Regular / 2 |
| Sandnya | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| Sendnya | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| Shruti | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| SimSun | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| Sylfaen | 2 | Source Serif 4 / Regular | Serif / 5 | UNKNOWN: type/width not established from name alone | Source Serif 4 / Regular / 5 |
| Symbol | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| Tahoma | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| Times New Roman | 127 | Liberation Serif / Regular | Serif / 5 | UNKNOWN: type/width not established from name alone | Liberation Serif / Regular / 5 |
| Tunga | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |
| Vrinda | 0 | Source Sans 3 / Regular | SansSerif / 5 | UNKNOWN: type/width not established from name alone | Source Sans 3 / Regular / 5 |

OBSERVATION: the current code also has explicit type hints for Elephant, Sylfaen, Times, Cambria, Calibri and Rockwell. INFERENCE: those are useful existing policy choices, but their classifications are not independently established from an absent original font in this task.

## Bundled faces and size

OBSERVATION: two open-licence families are bundled, four faces in total. Barlow Condensed Regular/Bold are unmodified upstream files with OS/2 width class 3 and weights 400/700. Their family metadata declares sans serif; their OS/2 family/serif PANOSE fields and IBM class are zero, so the classifier has a narrow family-name fallback for this bundle. This is stated family metadata, not an invented original-font metric.

OBSERVATION: Nifty Narrow Serif Regular/Bold are renamed static derivatives of Roboto Serif, made only from the supplied variation axes: width 62.5, optical size 144, grade 0 and weight 400/700. The generated OS/2 width class is 2. All original glyphs are retained; no glyph outlines were stretched by our renderer or importer. The full upstream variable file is 3,944,924 bytes and remains outside the repository. Full OFL licence texts and original copyright attribution are bundled; no Microsoft or additional Adobe collection font was downloaded or changed.

INFERENCE: the display optical design and extra-condensed width are useful for the observed headline fit. Optical size 144 is a fixed supplied design coordinate, not the text's point size and not automatic optical sizing. The earlier width-75/default-optical-20 trial left all seven newsletter notes; width 62.5 at optical 20 also left seven. The final supplied display design reduces the count to five. This is selection against our measured fit and readability, not proof of the absent original's proportions. No original Microsoft advance, width class or outline was used.

OBSERVATION: font preparation is repeatable with `python packaging/prepare_condensed_serif.py ORIGINAL_VARIABLE_FONT OUTPUT_DIRECTORY`, using the pinned source hash in that script. Repeating the preparation produced byte-identical static files. The preparation environment used version 4.61.1 of the optional font-processing library; normal builds, packaging and runtime do not depend on it. No Rust dependency was added. The retained metadata name ids 1/2/3/4/6/16/17/21/22 are renamed consistently; source copyright/licence entries remain intact.

| Bundled family | Faces | Font bytes | Licence bytes | Total, decimal KB |
| --- | --- | ---: | ---: | ---: |
| Nifty Narrow Serif | Regular, Bold | 538204 | 4489 | 542.693 |
| Barlow Condensed | Regular, Bold | 212392 | 4377 | 216.769 |

OBSERVATION: total bundle growth is 759,462 bytes (759.462 KB), below 1.5 MB. Both families use SIL OFL 1.1. Licence collection already includes the new texts in portable packages; the Windows installer list is extended too. Its packaging test now checks the exact six licence paths instead of only counting four. No release workflow was run.

OBSERVATION: Stint Ultra Condensed was inspected as another OFL candidate: its own width class is 1 and PANOSE identifies slab serifs. It was not bundled; it is farther from the condensed-name hint than the chosen extra-condensed serif, and no need for a third family was demonstrated. Bebas Neue family metadata was inspected but no face was downloaded or bundled. Every inspected font/metadata/licence URL and hash is listed in SOURCES.md.

## Ranking rule

OBSERVATION: exact available family requests still resolve normally. Existing specific substitution chains for Arial, Times, Courier, Calibri, Cambria and script fallbacks retain priority. For other classified missing families, the existing type distance ranks candidates first: matching type, including the existing related serif/slab group, before other types. This change does not assert that a serif substitute reproduces Rockwell's slab design. Missing unclassified families retain the previous default fallback.

OBSERVATION: name/style width hints map Ultra Condensed to 1, Extra Condensed to 2, Condensed/Narrow or the separate word Cond to 3, Semi Condensed to 4, Semi Expanded to 6, Expanded/Extended to 7, Extra Expanded to 8 and Ultra Expanded to 9. These are INFERENCE for substitution targeting, not recovered OS/2 values for an absent font. No stated width means no invented numeric width preference. Candidate numeric widths come only from valid OS/2 classes 1–9. Ranking uses absolute class distance; unknown/invalid candidate width ranks behind any known width when a target is stated.

OBSERVATION: remaining order is italic agreement, the requested width direction on an equal-distance tie, weight distance, the existing preferred bundled family, an installed/document/memory face over a bundled face on an otherwise equal tie, then stable family/style spelling. Thus an extra-condensed class 2 and semi-condensed class 4 are both distance 1 from target 3, and the narrower one wins unless the other has the requested italic style. This preserves the existing installed condensed-style assertions without weakening them. Weight uses the existing style-name hints; exact absent-original weights remain UNKNOWN.

OBSERVATION: the original name stays in the imported document. Substitution does not register the missing family as installed. All 161 successful import document serializations compare equal before/after after removing only timestamps. The existing explicit Find/Replace Font command remains available, and the imported-file path that suppresses its automatic opening is unchanged. No importer, frame fitting, stored size, margin, position, fill, outline, preset or protected reader was edited.

INFERENCE: the replacement is a closer width candidate, not a metric-compatible copy. Width classes are broad categories rather than measured text lengths; different designs at the same class can still produce different line breaks. The copied family hint for Rockwell remains an existing policy inference; its actual original classification is not established from its missing file.

## Overset and rendered observations

OBSERVATION: the existing composer and importer notes were used on all successful files. Notes are per story, including a threaded chain once. Counts below refer only to notes containing the existing hidden-text message.

| File | Before | After |
| --- | ---: | ---: |
| `SampleNewsletter.pub` | 7 | 5 |
| `SampleBrochure.pub` | 3 | 3 |
| `text__font-Arial.pub` | 0 | 0 |
| `text__font-ArialBlack.pub` | 0 | 0 |
| `text__font-CourierNew.pub` | 0 | 0 |
| `text__font-TimesNewRoman.pub` | 0 | 0 |

OBSERVATION: SampleNewsletter loses exactly the Our Excursion to the Zoo and Strictly Jazz notes, as those two complete stories now fit. What it means to be an American retains its note because trailing text in the story remains hidden even though the headline and byline now appear. Living and Learning in Class X remains clipped; its Franklin Gothic Book request has no condensed-name hint and still uses Source Sans 3. Body-story overflow also remains. No other file gains or loses an overset note. Real-file notes total 40 → 38 and overset notes 10 → 8; controlled notes stay 55 including one overset note in text__size-72.

OBSERVATION: first-page before/after renders of SampleNewsletter and SampleBrochure were made at scale 1.5 and inspected. The newsletter now shows the complete Strictly Jazz, Our Excursion to the Zoo and American headline/byline, in visibly narrower, lighter serif letters. The main title still omits Class X. The zoo headline still overlaps the giraffe artwork; no object was moved to hide that problem. The brochure first-page PNG is byte-identical. Its one Arial Narrow formatting request is the bold 13 pt date in the hidden end of the first-page title story (story 31, frame 30). The preceding Georgia-formatted lines already fill that frame; changing the hidden date face does not make it visible.

UNKNOWN: there is no Publisher golden image for SampleNewsletter or SampleBrochure in these inputs. The nine available references are six WordArt, two shadows and one group. Newsletter appearance cannot be compared with Publisher here; whether the new serif is too narrow or light relative to the original remains unknown. No matching appearance claim is made from overset reduction.

## Pixel comparisons and controlled snapshots

OBSERVATION: all nine reference scores are unchanged and every reduced-size comparison remains 850 × 1100 without a size mismatch. These fixtures use none of the changed visible requests. The controlled snapshot comparison x-after against x-baseline reports **142 compared, zero changed, zero added/removed**. Thus the changed-fixture list for the 142 controlled first pages is empty; these own-output snapshots are not a Publisher comparison.

| Fixture | SSIM before | SSIM after | Differing fraction before | Differing fraction after |
| --- | ---: | ---: | ---: | ---: |
| `group__rotated.pub` | 0.999980060 | 0.999980060 | 0.001000000 | 0.001000000 |
| `shadow__offset.pub` | 0.999993784 | 0.999993784 | 0.000983957 | 0.000983957 |
| `shadow__on.pub` | 0.999994589 | 0.999994589 | 0.000963636 | 0.000963636 |
| `wordart__arch.pub` | 0.983892067 | 0.983892067 | 0.006321925 | 0.006321925 |
| `wordart__base.pub` | 0.980074652 | 0.980074652 | 0.006571123 | 0.006571123 |
| `wordart__circle.pub` | 0.960595137 | 0.960595137 | 0.009868449 | 0.009868449 |
| `wordart__fill-red.pub` | 0.979558751 | 0.979558751 | 0.012936898 | 0.012936898 |
| `wordart__line-white.pub` | 1.000000000 | 1.000000000 | 0.000000000 | 0.000000000 |
| `wordart__shadow.pub` | 0.979426457 | 0.979426457 | 0.009779679 | 0.009779679 |

OBSERVATION: x-baseline and x-after use the same machine, fonts outside this change, inputs, 1 px/pt scale and first-page policy. The nine Publisher references cover no ordinary headline or font-only fixture. White margins, font substitution, antialiasing and the limited reference set restrict what these scores say. A good score is not proof of correct reading, or of matching Publisher typography.

## Verification and remaining limits

OBSERVATION: baseline and final gates both report 142/142 imports, 152/152 overlap coverage, 111/111 scored shape/text positions within 1 pt, 12/41 pictures within 1 pt with 29 unmatched, 52/52 exact texts and 111/111 styles. All 142 save/reopen/edit/PDF checks pass. The real batch remains 9 with notes / 10 with warnings / 2 refused / 3 damaged, 31 pages and 280 objects. Only SampleNewsletter loses two notes (9 total notes → 7); its warning category and two warnings remain. SampleBrochure stays at five total notes and one warning. No warning or refusal category changes.

OBSERVATION: five additional unit tests use hand-specified classes, widths and weights without network or system fonts. They cover exact and nearest width, invalid/unknown widths, family priority, width-direction ties, italic preservation, installed/bundled ties and bundled availability. Existing condensed-style tests remain intact; the bundled-only Rockwell expectation now names the new substitute. Packaging tests pass, and static font regeneration matches both bundled hashes. The existing README statement that Microsoft fonts may be substituted is still true, so no new limits line was added.

UNKNOWN: Windows execution, real-PC font inventories, Microsoft's original font files/metrics and Publisher's own rendering of these headlines were not tested. Only first pages were rendered for the regression snapshots and brochure/newsletter visual check; later-page visual fidelity is unmeasured. No browser visual check or physical printing was performed. Existing ordinary-title/body overflow, substituted original classification and exact width compatibility remain unresolved; no automatic text fitting was introduced.

OBSERVATION: formatting, strict workspace linting and all seven CI steps pass: 3,103 tests passed, zero failed and 25 ignored. Packaging checks run 34 tests successfully, with three platform-dependent skips. Original per-file warning and refusal messages compare unchanged.
