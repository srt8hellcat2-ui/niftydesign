# Numbered lists and custom tab stops

## Scope and baseline

OBSERVATION: this notes-only census starts from main `3e5cc22`. It uses the pinned 142 controlled binaries and 24 Apache binaries, the existing container, bounded property-list and Quill readers, and the retained format notes. All 317 recorded sizes and SHA-256 values match: 166 binaries, 142 scoring JSON files and nine reference images. Scoring JSON is used only by the unchanged import score; neither JSON nor images supplies an encoding interpretation. No other reader or external format source was consulted.

OBSERVATION: the untouched-main baseline is 142/142 imports, 152/152 objects, 111/111 scored shape/text positions within 1 pt, 52/52 exact texts and 111/111 styles. The separate picture result is 12/41 matched passes, zero measured misses and 29 unmatched references. All 142 save/reopen/edit/PDF cases pass. The real-file batch passes with 9 notes, 10 warnings, 2 refused and 3 damaged files, 31 pages and 280 objects.

## Inventory and counting method

OBSERVATION: the recorded fixture names contain no `list__*` or `text__tab-*` file. The sole controlled list-named variant is `text__bullets.pub`. No controlled TEXT section contains U+0009. No successfully decoded paragraph has a confirmed numbering descriptor, and the public importer produces no numbered paragraphs. These are counts of confirmed cases, not proof that every opaque field lacks numbering or tab information.

OBSERVATION: source paragraphs are counted separately within each Quill story, including empty CR-terminated paragraphs. Each field count below counts distinct source paragraphs intersected by a field-bearing FDPP run, not just run records. Positions are zero-based UTF-16 offsets in the complete TEXT section; story/paragraph identifiers are separate. Defaults are reported separately from runs. Paragraphs need not equal native paragraph counts after text normalization or frame construction.

| File | Source stories / paragraphs | Confirmed bullet paragraphs | Confirmed numbered paragraphs | Literal tabs / paragraphs with tabs | Confirmed custom stops | Current import |
| --- | --- | ---: | ---: | --- | --- | --- |
| `text__bullets.pub` | 1 / 1 | 1 | 0 | 0 / 0 | UNKNOWN | One native bullet paragraph; no warning; paragraph-default note. |
| `SampleNewsletter.pub` | 44 / 113 | 3 | 0 | 14 / 5 | UNKNOWN | Three native bullet paragraphs; 14 tabs retained; no explicit native stops; warns about one unconfirmed list descriptor and omitted fields/special characters. |
| `SampleBrochure.pub` | 20 / 69 | 0 | 0 | 8 / 5 | UNKNOWN | Eight tabs retained; no list paragraph or explicit native stop; warns about omitted fields/special characters. |

OBSERVATION: these are the only three successfully read files with paragraph-run key 0x57 or literal U+0009. The other 141 controlled binaries and 17 successfully imported Apache binaries have neither. In the three files above the importer sets no list-level override; the native default is level 1. UNKNOWN: original Publisher list levels, the number of separate source lists, and non-default tab-stop counts cannot be established from that native default.

UNKNOWN: the two refused files `Sample98.pub` and `Sample2000.pub` cannot be classified by the current Quill parser: both return unknown property kind 0x19. Their importer result remains the existing older-family refusal. The three damaged clusterfuzz files cannot be read by the container parser. They are not certified list-free or tab-free. No alternative layout or reader was used to fill those gaps.

OBSERVATION: `text__base.pub` is a one-story, one-paragraph no-list control with an empty FDPP record. Comparing its binary properties with the bullet control finds the five added keys listed below. The newsletter supplies three additional paragraphs with the same supported descriptor and a different font-table index and indent pair. No confirmed numbered-list outcome supplies a further comparison.

## Paragraph fields and counts

OBSERVATION: the table covers runs across all successfully decoded inputs. Numeric values are unsigned raw payloads unless a signed interpretation is explicitly identified. Files/paragraphs count a field value once per paragraph even if several runs repeat it. These are observations of field occurrence, not automatic assignments of a new format meaning.

| Field / kind | Values seen | Files / source paragraphs | Interpretation and limit |
| --- | --- | --- | --- |
| 0x57 / 0x8A | children 0x00/0x22 = 23, 0x01/0x22 = 183, 0x02/0x22 = 0 | 2 / 4: control 1, newsletter 3 | OBSERVATION: the existing parser recognizes this descriptor as bullets; two files support the same marker outcome. |
| 0x57 / 0x8A | children 0x00/0x22 = 255, 0x01/0x22 = 0, 0x02/0x22 = 0 | 1 / 1: newsletter | UNKNOWN: unsupported descriptor on an empty paragraph; not evidence of numbering or a verified explicit no-list override. |
| 0x02 / 0x22 | 127000; 0 | 127000 in 2 / 8; 0 in 1 / 4 | OBSERVATION: nonzero on the four bullet paragraphs and four adjacent empty paragraphs; zero on three tab-bearing newsletter paragraphs and the unknown-descriptor empty paragraph. UNVERIFIED HYPOTHESIS: marker-size field. The importer does not apply it as a separate bullet size. |
| 0x03 / 0x1A | 31; 19 | 31 in 1 / 1 control paragraph; 19 in 1 / 7 newsletter paragraphs | OBSERVATION: both indexes name `Symbol` in their own FONT tables. The importer uses this field for marker font. It is not a list level. |
| 0x0C / 0x22 | 4294738696 (-228600); 4294795846 (-171450); 228600; 0 | each in 1 file: respectively 1, 3, 1 and 1 paragraphs | OBSERVATION: existing signed first-line-indent mapping. The positive value is the indent-only control; zero is the newsletter unknown-descriptor paragraph. |
| 0x0D / 0x22 | 228600; 171450; 4294830136 (-137160) | respectively 2 / 2, 1 / 3, 1 / 1 | OBSERVATION: existing signed left-indent mapping; the first two give 18 pt and 13.5 pt alongside the negative bullet first indents. These are not tab-stop coordinates. |
| 0x06 / 0x22 | 0; 171450; 360000 | each in 1 file: respectively 1, 3 and 4 paragraphs | UNKNOWN: newsletter-only field; 171450 accompanies the three bullet paragraphs, 360000 adjacent empty paragraphs, zero the unsupported-descriptor paragraph. Equality with an indent is not a definition. |
| 0x19 / 0x1A | 1–12, 15, 16, 18–23 | 3 files; exact per-file counts below | UNKNOWN: present with and without tabs; not established as a tab count, position, alignment or list level. |
| 0x1D / 0x0A | no payload | 1 / 1: newsletter story 63 paragraph 2, containing five tabs | UNKNOWN: only one observed outcome; no tab semantics established. |
| 0x32 / 0x82 | nested fields described below | 1 / 5: newsletter story 64 paragraphs 5–9 | UNVERIFIED HYPOTHESIS: a tab-stop container; no independent setting-change pair establishes this. None of these five paragraphs contains a literal tab. |

OBSERVATION: keys 0x30 and 0x3C are also present in these real-file paragraph records but are not newly identified as lists or tabs. The retained [unknown-fields.md](unknown-fields.md) census establishes 0x30/0x12 = 5 in 48 run paragraphs across three files, and 0x3C/0x12 = 50 in five brochure run paragraphs; the controlled justified paragraph has 0x3C = 18482. Their presence does not define custom stops. Defaults and justification remain separate from list/tab identification.

## What the importer already does for bullets

OBSERVATION: the current paragraph parser requires parent 0x57 kind 0x8A and child 0x00 equal to 23 before constructing a bullet setting. Child 0x01 is converted to a valid, non-control character; the observed 183 becomes U+00B7. Child 0x02 is not given a meaning: a nonzero value triggers an unsupported-field diagnostic, as does any child key above 2. An invalid marker character also triggers a diagnostic. Those checks do not themselves prevent construction of the recognized bullet setting.

OBSERVATION: marker font comes from paragraph key 0x03, resolved through FONT. The import applies native bullet type, character and font plus the already supported 0x0C/0x0D indents. Key 0x02 is skipped as a recognized key and does not separately set marker size. Missing supported bullet values may inherit paragraph defaults; an unknown descriptor is not decoded as an explicit list-off setting. No original numbering style, start, restart or list-level field is decoded.

OBSERVATION: the recognized descriptor occurs at TEXT ranges [0,25) in the control and [8685,8766), [8767,8846), [8847,8880) in the newsletter. The unsupported `(255,0,0)` descriptor occupies [1637,1638), exactly one CR, in newsletter story 15 paragraph 5. UNVERIFIED HYPOTHESIS: it could clear a list, but neither a no-list interpretation nor a numbering interpretation is confirmed. Do not turn this value into a numbering mode.

## Numbering styles, starts and restarts

| Requested attribute | Finding |
| --- | --- |
| Decimal `1.`, alphabetic `a.`, Roman `i.` or another style | UNKNOWN: no confirmed numbered-list fixture or decoded numbered paragraph. |
| Start number | UNKNOWN: no isolated start-number variants. |
| Separator, prefix and suffix | UNKNOWN: no isolated variants. |
| Restart versus continuation | UNKNOWN: no paired examples across separate lists, stories or ordinary paragraphs. |
| List levels and cross-level numbering | UNKNOWN: native level 1 is a default, not a decoded source level. |
| Same fields varying for numbered lists | UNKNOWN: no confirmed numbering outcome allows comparing its 0x57, 0x02, 0x03, indents or other fields. |

OBSERVATION: child 0x02 of every observed 0x57 descriptor is zero, including the unknown descriptor. UNVERIFIED HYPOTHESIS: assigning it a start, restart or level meaning would invent an encoding. No permitted published paragraph-numbering mapping was used; drawing-property numbers must not be substituted for Quill property keys merely because their numbers resemble one another.

## Literal tabs and candidate stop records

| File; story; paragraph | TEXT range | Literal tabs | Candidate run fields |
| --- | --- | ---: | --- |
| Brochure; 4; 1 | [101,123) | 2 | OBSERVATION: 0x19/0x1A = 5. |
| Brochure; 7; 3 | [238,268) | 1 | OBSERVATION: 0x19/0x1A = 7. |
| Brochure; 7; 4 | [268,296) | 2 | OBSERVATION: 0x19/0x1A = 7. |
| Brochure; 8; 1 | [757,764) | 2 | OBSERVATION: 0x19/0x1A = 6. |
| Brochure; 8; 2 | [764,781) | 1 | OBSERVATION: 0x19/0x1A = 6. |
| Newsletter; 6; 2 | [917,936) | 6 | OBSERVATION: 0x19/0x1A = 3. |
| Newsletter; 15; 2 | [1468,1523) | 1 | OBSERVATION: 0x02/0x22 = 0; 0x19/0x1A = 5. |
| Newsletter; 15; 3 | [1523,1569) | 1 | OBSERVATION: same two fields. |
| Newsletter; 15; 4 | [1569,1637) | 1 | OBSERVATION: same two fields. |
| Newsletter; 63; 2 | [8632,8661) | 5 | OBSERVATION: 0x1D/0x0A present; 0x19 absent. |

OBSERVATION: Brochure means `SampleBrochure.pub` and Newsletter means `SampleNewsletter.pub`. Tab characters are occurrences, not tab-stop definitions. No confirmed stored tab positions, alignment kinds or leaders can be reported for these ten paragraphs. UNKNOWN: whether they use default or custom stops in Publisher.

OBSERVATION: the five identical newsletter 0x32/0x82 payloads contain 0x27/0x1A = 1 and 0x28/0x8A, with one nested 0x00/0x88 record whose 0x00/0x20 value is 171450. Their TEXT ranges are [8767,8846), [8846,8847), [8847,8880), [8880,8881), [8881,8882). Two are bullet paragraphs; three are empty. The first newsletter bullet paragraph lacks 0x32. Count: one file, five source paragraphs, one nested numeric entry per occurrence. UNKNOWN: whether that numeric entry is a position, what unit/origin it would use, how clearing works, and where an alignment or leader would be stored.

UNVERIFIED HYPOTHESIS: interpreting 171450 as EMU would give 13.5 pt, equal to the neighbouring bullet indent. That equality is not a unit or tab-position definition. One file with no isolated setting change cannot establish the parent key's meaning; the outer 1 must not be called a stop count or alignment without evidence.

OBSERVATION: the importer preserves literal U+0009 as a native tab character, rather than replacing it with spaces or deleting it. Neither 0x32 nor 0x19 is decoded into a native tab-stop list. All native paragraphs in the two tab-bearing files have no explicit stop override. The native layout therefore uses its existing default tab policy, not reconstructed Publisher stop settings. UNKNOWN: corresponding Publisher alignments and leaders, and resulting positional accuracy.

## Samples needed

INFERENCE: request 20 small publications, with intended settings recorded and an independent scoring export/reference image for each. Keep the same text, fonts and frame geometry within each comparison family; each variant changes one setting.

- INFERENCE: 1 no-list control with two short paragraph groups separated by an ordinary paragraph.
- INFERENCE: 1 decimal-numbered copy, using the application's default start and continuation settings.
- INFERENCE: 2 copies changing only decimal style to lower-case alphabetic or lower-case Roman.
- INFERENCE: 1 copy changing only the starting number to 5.
- INFERENCE: 2 copies changing only the separator to a closing parenthesis or to no separator.
- INFERENCE: 2 copies changing only restart or continuation for the second group.
- INFERENCE: 1 copy changing only one numbered paragraph to level 2.
- INFERENCE: 1 control containing literal tabs with default stops.
- INFERENCE: 2 copies with one explicit left stop, first at 36 pt and then at 72 pt, changing only position between those two copies.
- INFERENCE: 3 copies changing only that stop's alignment to centre, right or decimal.
- INFERENCE: 2 copies changing only that stop's leader to dots or dashes.
- INFERENCE: 1 copy adding only a second stop at a distinct position.
- INFERENCE: 1 copy clearing only the explicit stop.

## Recommendation and checks

INFERENCE: **wait for samples** for both numbering and custom stops. A later implementation is **medium** once independently differentiated samples establish the mappings: it must preserve list inheritance and restart state and distinguish tab position, alignment and leader settings. UNKNOWN: effort may become large if style references, nested lists or cross-story state require additional mappings.

INFERENCE: the main risk is turning the newsletter's unsupported descriptor 255 into numbering, or treating 0x32's repeated 171450 as a confirmed custom stop. A preserved tab or a native default list level cannot validate either encoding. No importer or renderer change is justified by these observations alone.

OBSERVATION: this first complete draft records the passed untouched-main baseline. UNKNOWN: final formatting, lint, workspace and repeated corpus results are still being collected. Only documentation is changed; no environment configuration, setup script or setting is changed and no release action is run.
