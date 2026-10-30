# Picture gaps — AC inventory, 2026-10-30

## Result and limits

OBSERVATION: untouched main `c6c9ee0` gives 142/142 imports, 152/152 objects, 111/111 scored shape/text positions within 1 pt, 52/52 exact texts and 111/111 styles. The separate picture check gives 12/41 matched passes, zero matched misses and 29 unmatched image-labelled references. All 142 save/reopen/edit/PDF cases pass. The real batch remains 9 with notes, 10 with warnings, 2 refused and 3 damaged.

OBSERVATION: all 317 pinned inputs verified against their existing sizes and SHA-256 values. The inventory covers all 142 controlled files and 19 successfully imported real files. The two refused old-format files and three damaged files are recorded separately, without claiming that their picture content is empty. No product source, parser, scorer, test, importer or renderer was changed.

OBSERVATION: the current drawing reader exposes 37 stored picture records across 22 files: 12 PNG, 10 JPEG, 2 GIF and 13 WMF. There are 29 imported picture assets and 31 placed picture items. Eight stored JPEG records have no positive parsed placement reference and no imported asset. Two placed assets are reused: picture 1 in picture__two.pub and picture 1 in SampleNewsletter.pub.

INFERENCE: the 29 unmatched references are a comparison-category mismatch, not evidence of 29 lost pictures. Every one has no imported picture candidate and no positive parsed picture-placement reference. They cover other object types in the importer. No picture-format correction is shown to turn those references into native-picture matches.

UNKNOWN: visual fidelity of the actual PNG, JPEG, GIF and WMF pictures against Publisher. None has a golden image here. The 12 position passes check placed-frame bounds, not pixels or complete picture content. Successful decoding and a lack of warnings do not prove a faithful copy.

## Counting and evidence

OBSERVATION: picture index means the one-based key returned by the existing drawing parser's picture map. A stored record, an imported asset, a shape reference and a placed item are counted separately. Reuse does not duplicate a stored record. Source sequences identify parsed drawing shapes; output item ids identify current document items. Sets on the same row are not a newly decoded sequence-to-item mapping.

OBSERVATION: a read-only diagnostic under the ignored build directory called the existing compound-file reader, drawing parser, public importer, raster decoder and vector-to-SVG conversion. Parser source was read unchanged. The storage pass did not open scoring JSON or golden images. A separate scoring pass counted image-labelled references and located their page/element identifiers using the unchanged scorer's rules. Those exports supplied no storage interpretation.

OBSERVATION: the 34-byte metafile wrapper and its compression selector were handled exactly as the existing conversion code does, only to inspect the payload signature and existing conversion result. No new header rule is proposed. No new external specification, other reader or web format search was used. Golden images were hash-checked only; none was used to infer encoding or judge native-picture fidelity.

## Counts by format

OBSERVATION: the following counts are for successfully imported files only. The stored record count includes the eight unused JPEG records; the asset and item columns do not.

| Format | Stored records | Imported assets | Placed items |
| --- | --- | --- | --- |
| PNG | 12 | 12 | 13 |
| JPEG | 10 | 2 | 2 |
| GIF | 2 | 2 | 2 |
| WMF | 13 | 13 | 14 |
| Total | 37 | 29 | 31 |

OBSERVATION: the 13 WMF records all become SVG assets. All 16 imported raster assets fully decode with the existing raster decoder; all 13 generated SVG assets parse with the existing SVG reader. No parsed placement reference points to a missing stored picture record. No picture-related import or metafile-conversion warning occurs in this inventory. The source files can still have unrelated text/table warnings.

## Every exposed picture record

OBSERVATION: all rows in this table come from the binary/parser pass and public import output. PNG/JPEG/GIF sizes are raster pixels. WMF sizes marked “SVG” are the generated SVG's integer viewport dimensions reported by the importer, not a stored raster resolution. A dash in source sequence/item ids means none was found, not a decoded failure.

| File | Picture index | Record / payload / signature key | Size | Source sequence(s) | Output item(s) | Current handling | Reference availability |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `picture__base.pub` | 1 | 0xF01E / PNG / P | 1024 × 1024 | 293 | 7 | Placed raster; no picture warning | Picture-frame scoring JSON; no golden |
| `picture__border.pub` | 1 | 0xF01E / PNG / P | 1024 × 1024 | 293 | 7 | Placed raster; no picture warning | Picture-frame scoring JSON; no golden |
| `picture__crop-all.pub` | 1 | 0xF01E / PNG / P | 1024 × 1024 | 293 | 7 | Placed raster; no picture warning | Picture-frame scoring JSON; no golden |
| `picture__crop-left.pub` | 1 | 0xF01E / PNG / P | 1024 × 1024 | 293 | 7 | Placed raster; no picture warning | Picture-frame scoring JSON; no golden |
| `picture__crop-top.pub` | 1 | 0xF01E / PNG / P | 1024 × 1024 | 293 | 7 | Placed raster; no picture warning | Picture-frame scoring JSON; no golden |
| `picture__flip-h.pub` | 1 | 0xF01E / PNG / P | 1024 × 1024 | 293 | 7 | Placed raster; no picture warning | Picture-frame scoring JSON; no golden |
| `picture__moved.pub` | 1 | 0xF01E / PNG / P | 1024 × 1024 | 293 | 7 | Placed raster; no picture warning | Picture-frame scoring JSON; no golden |
| `picture__oval-mask.pub` | 1 | 0xF01E / PNG / P | 1024 × 1024 | 293 | 7 | Placed raster; no picture warning | Picture-frame scoring JSON; no golden |
| `picture__rotated-30.pub` | 1 | 0xF01E / PNG / P | 1024 × 1024 | 293 | 7 | Placed raster; no picture warning | Picture-frame scoring JSON; no golden |
| `picture__stretched.pub` | 1 | 0xF01E / PNG / P | 1024 × 1024 | 293 | 7 | Placed raster; no picture warning | Picture-frame scoring JSON; no golden |
| `picture__two.pub` | 1 | 0xF01E / PNG / P | 1024 × 1024 | 293, 295 | 7, 9 | Placed raster; no picture warning | Picture-frame scoring JSON; no golden |
| `wordart__arch.pub` | 1 | 0xF01D / JPEG / J | 128 × 128 | — | — | Unused by importer; no picture warning; role UNKNOWN | WordArt object scoring JSON; golden at file level; JPEG role UNKNOWN |
| `wordart__base.pub` | 1 | 0xF01D / JPEG / J | 128 × 128 | — | — | Unused by importer; no picture warning; role UNKNOWN | WordArt object scoring JSON; golden at file level; JPEG role UNKNOWN |
| `wordart__circle.pub` | 1 | 0xF01D / JPEG / J | 128 × 128 | — | — | Unused by importer; no picture warning; role UNKNOWN | WordArt object scoring JSON; golden at file level; JPEG role UNKNOWN |
| `wordart__fill-red.pub` | 1 | 0xF01D / JPEG / J | 128 × 128 | — | — | Unused by importer; no picture warning; role UNKNOWN | WordArt object scoring JSON; golden at file level; JPEG role UNKNOWN |
| `wordart__font.pub` | 1 | 0xF01D / JPEG / J | 128 × 128 | — | — | Unused by importer; no picture warning; role UNKNOWN | WordArt object scoring JSON; no golden; JPEG role UNKNOWN |
| `wordart__line-white.pub` | 1 | 0xF01D / JPEG / J | 128 × 128 | — | — | Unused by importer; no picture warning; role UNKNOWN | WordArt object scoring JSON; golden at file level; JPEG role UNKNOWN |
| `wordart__shadow.pub` | 1 | 0xF01D / JPEG / J | 128 × 128 | — | — | Unused by importer; no picture warning; role UNKNOWN | WordArt object scoring JSON; golden at file level; JPEG role UNKNOWN |
| `wordart__text.pub` | 1 | 0xF01D / JPEG / J | 128 × 128 | — | — | Unused by importer; no picture warning; role UNKNOWN | WordArt object scoring JSON; no golden; JPEG role UNKNOWN |
| `51318.pub` | 1 | 0xF01E / PNG / P | 360 × 216 | 311 | 25 | Placed raster; no picture warning | No scoring or golden reference |
| `SampleBrochure.pub` | 1 | 0xF01B / WMF / W | SVG 1963 × 1981 | 362 | 70 | Placed as SVG; no picture warning | No scoring or golden reference |
| `SampleBrochure.pub` | 2 | 0xF01E / GIF / G | 148 × 184 | 367 | 76 | Placed raster; no picture warning | No scoring or golden reference |
| `SampleBrochure.pub` | 3 | 0xF01B / WMF / W | SVG 1614 × 1988 | 363 | 80 | Placed as SVG; no picture warning | No scoring or golden reference |
| `SampleBrochure.pub` | 4 | 0xF01E / GIF / G | 150 × 208 | 375 | 84 | Placed raster; no picture warning | No scoring or golden reference |
| `SampleBrochure.pub` | 5 | 0xF01B / WMF / W | SVG 1939 × 1529 | 377 | 86 | Placed as SVG; no picture warning | No scoring or golden reference |
| `SampleBrochure.pub` | 6 | 0xF01B / WMF / W | SVG 2001 × 1809 | 371 | 88 | Placed as SVG; no picture warning | No scoring or golden reference |
| `SampleBrochure.pub` | 7 | 0xF01D / JPEG / J | 185 × 230 | 365 | 90 | Placed raster; no picture warning | No scoring or golden reference |
| `SampleBrochure.pub` | 8 | 0xF01B / WMF / W | SVG 1000 × 967 | 382 | 92 | Placed as SVG; no picture warning | No scoring or golden reference |
| `SampleNewsletter.pub` | 1 | 0xF01B / WMF / W | SVG 16446 × 16298 | 338, 403 | 60, 138 | Placed as SVG; no picture warning | No scoring or golden reference |
| `SampleNewsletter.pub` | 2 | 0xF01B / WMF / W | SVG 1344 × 751 | 346 | 71 | Placed as SVG; no picture warning | No scoring or golden reference |
| `SampleNewsletter.pub` | 3 | 0xF01B / WMF / W | SVG 3191 × 3159 | 368 | 98 | Placed as SVG; no picture warning | No scoring or golden reference |
| `SampleNewsletter.pub` | 4 | 0xF01B / WMF / W | SVG 16446 × 9663 | 376 | 109 | Placed as SVG; no picture warning | No scoring or golden reference |
| `SampleNewsletter.pub` | 5 | 0xF01B / WMF / W | SVG 1000 × 500 | 389 | 117 | Placed as SVG; no picture warning | No scoring or golden reference |
| `SampleNewsletter.pub` | 6 | 0xF01D / JPEG / J | 281 × 425 | 317 | 150 | Placed raster; no picture warning | No scoring or golden reference |
| `SampleNewsletter.pub` | 7 | 0xF01B / WMF / W | SVG 2806 × 1023 | 303 | 154 | Placed as SVG; no picture warning | No scoring or golden reference |
| `SampleNewsletter.pub` | 8 | 0xF01B / WMF / W | SVG 1995 × 452 | 419 | 162 | Placed as SVG; no picture warning | No scoring or golden reference |
| `SampleNewsletter.pub` | 9 | 0xF01B / WMF / W | SVG 1998 × 632 | 420 | 164 | Placed as SVG; no picture warning | No scoring or golden reference |

OBSERVATION: signature keys summarize exact observed bytes below. The drawing record type alone is insufficient: SampleBrochure.pub indices 2 and 4 have type 0xF01E but contain GIF89a bytes, and are imported as GIF, not PNG. The importer passes ordinary bitmap payloads to its existing format detector.

| Key | Observed signature bytes | Meaning |
| --- | --- | --- |
| P | `89 50 4E 47 0D 0A 1A 0A` | PNG signature; full observed heads and dimensions remain in the local inventory. |
| J | `FF D8 FF E0` | JPEG signature; full observed heads and dimensions remain in the local inventory. |
| G | `47 49 46 38 39 61` | GIF signature; full observed heads and dimensions remain in the local inventory. |
| W | Each record includes the existing metafile wrapper; its first bytes vary. Every extracted payload starts `01 00 09 00 00 03`. | WMF header accepted by the existing WMF reader; wrapper bytes are not a raster signature. |

OBSERVATION: eight 128 × 128 JPEG records occur one each in the eight WordArt fixtures. No parsed drawing shape uses their index through the current picture-placement property. The importer creates editable text, not a picture, for those objects.

UNKNOWN: the purpose of those eight JPEG records. Their size and co-occurrence with WordArt do not establish that they are previews, textures, fallback artwork or hidden pictures. No scoring reference was used to assign them a role.

## Why each of the 29 references is unmatched

OBSERVATION: every row below is scoring page 1, element 1. Each has zero imported picture candidates and zero positive parsed picture-placement references. The reference labels the object as an image; the importer represents it as the object type shown. “Unassigned” picture content on a normal painted shape is not by itself a placeholder: these shapes have their own geometry and paint.

| Basis | File | Page / element | Current importer output | Primary unmatched cause |
| --- | --- | --- | --- | --- |
| OBSERVATION | `fill__gradient.pub` | 1 / 1 | Painted shape/path/line | Reference covers another object category; no picture candidate |
| OBSERVATION | `group__rotated.pub` | 1 / 1 | Group with two painted shape leaves | Reference covers another object category; no picture candidate |
| OBSERVATION | `line__dash-2.pub` | 1 / 1 | Painted shape/path/line | Reference covers another object category; no picture candidate |
| OBSERVATION | `line__dash-4.pub` | 1 / 1 | Painted shape/path/line | Reference covers another object category; no picture candidate |
| OBSERVATION | `line__dash-5.pub` | 1 / 1 | Painted shape/path/line | Reference covers another object category; no picture candidate |
| OBSERVATION | `shadow__offset.pub` | 1 / 1 | Painted shape/path/line | Reference covers another object category; no picture candidate |
| OBSERVATION | `shadow__on.pub` | 1 / 1 | Painted shape/path/line | Reference covers another object category; no picture candidate |
| OBSERVATION | `shapes__curve.pub` | 1 / 1 | Painted shape/path/line | Reference covers another object category; no picture candidate |
| OBSERVATION | `shapes__diamond.pub` | 1 / 1 | Painted shape/path/line | Reference covers another object category; no picture candidate |
| OBSERVATION | `shapes__heart.pub` | 1 / 1 | Painted shape/path/line | Reference covers another object category; no picture candidate |
| OBSERVATION | `shapes__hexagon.pub` | 1 / 1 | Painted shape/path/line | Reference covers another object category; no picture candidate |
| OBSERVATION | `shapes__line-arrow.pub` | 1 / 1 | Painted shape/path/line | Reference covers another object category; no picture candidate |
| OBSERVATION | `shapes__line.pub` | 1 / 1 | Painted shape/path/line | Reference covers another object category; no picture candidate |
| OBSERVATION | `shapes__polyline.pub` | 1 / 1 | Painted shape/path/line | Reference covers another object category; no picture candidate |
| OBSERVATION | `shapes__rightarrow.pub` | 1 / 1 | Painted shape/path/line | Reference covers another object category; no picture candidate |
| OBSERVATION | `shapes__roundrect.pub` | 1 / 1 | Painted shape/path/line | Reference covers another object category; no picture candidate |
| OBSERVATION | `shapes__star5.pub` | 1 / 1 | Painted shape/path/line | Reference covers another object category; no picture candidate |
| OBSERVATION | `shapes__triangle.pub` | 1 / 1 | Painted shape/path/line | Reference covers another object category; no picture candidate |
| OBSERVATION | `table__2x2.pub` | 1 / 1 | Table/text frame | Reference covers another object category; no picture candidate |
| OBSERVATION | `table__3x2.pub` | 1 / 1 | Table/text frame | Reference covers another object category; no picture candidate |
| OBSERVATION | `table__merged.pub` | 1 / 1 | Table/text frame | Reference covers another object category; no picture candidate |
| OBSERVATION | `wordart__arch.pub` | 1 / 1 | Editable WordArt text | Reference covers another object category; no picture candidate |
| OBSERVATION | `wordart__base.pub` | 1 / 1 | Editable WordArt text | Reference covers another object category; no picture candidate |
| OBSERVATION | `wordart__circle.pub` | 1 / 1 | Editable WordArt text | Reference covers another object category; no picture candidate |
| OBSERVATION | `wordart__fill-red.pub` | 1 / 1 | Editable WordArt text | Reference covers another object category; no picture candidate |
| OBSERVATION | `wordart__font.pub` | 1 / 1 | Editable WordArt text | Reference covers another object category; no picture candidate |
| OBSERVATION | `wordart__line-white.pub` | 1 / 1 | Editable WordArt text | Reference covers another object category; no picture candidate |
| OBSERVATION | `wordart__shadow.pub` | 1 / 1 | Editable WordArt text | Reference covers another object category; no picture candidate |
| OBSERVATION | `wordart__text.pub` | 1 / 1 | Editable WordArt text | Reference covers another object category; no picture candidate |

OBSERVATION: primary causes are mutually exclusive in the following table; object-type subcounts split the same 29 cases and must not be added again.

| Cause | Count | Interpretation |
| --- | --- | --- |
| OBSERVATION: reference covers another object category | 29 | 17 painted shapes/paths/lines; 8 WordArt texts; 3 tables; 1 group |
| OBSERVATION: matched picture has wrong position or size | 0 | All 12 matched pictures pass within 1 pt |
| OBSERVATION: unmatched reference linked to a failed picture decode | 0 | No positive parsed placement reference in these 29 files |
| OBSERVATION: unmatched reference linked to an empty picture placeholder | 0 | No positive parsed placement reference in these 29 files |
| OBSERVATION: unmatched reference linked to an unsupported picture format | 0 | No such linkage shown; the unused WordArt JPEG roles remain UNKNOWN |
| UNKNOWN: visible image-content mismatch in native pictures | Unmeasured | No native-picture golden images; not an extra count of unmatched references |

OBSERVATION: files in the 17 painted-object subcount are fill__gradient.pub; line__dash-2.pub, line__dash-4.pub, line__dash-5.pub; shadow__offset.pub, shadow__on.pub; and the eleven shapes__ files listed individually above. All eight wordart__ files, all three table__ files and group__rotated.pub complete the 29. The list agrees with picture-positions.md and the fresh unchanged corpus result.

UNKNOWN: the exact reason the scoring exporter rasterized these other objects, or its precise painted-bound rounding. Their image label is not a statement about stored picture encoding. This inventory does not turn their unmeasured picture-position status into a pass, or claim that their appearance matches Publisher.

## Format coverage and present behavior

OBSERVATION: the table distinguishes observed inputs from branches present in the existing code. Zero-count rows are not Publisher-format validation. General support in the shared image library does not prove that Publisher stores or preserves that format when inserting it.

| Format | Observed count | Current handling | Remaining limit |
| --- | --- | --- | --- |
| PNG | 12 records / 13 items | OBSERVATION: detected by signature, retained as PNG raster assets. | UNKNOWN: alpha, color profiles and visual fidelity against Publisher. |
| JPEG | 10 records / 2 placed items; 8 unused records | OBSERVATION: two real JPEGs retained and decoded; eight unreferenced WordArt records are not placed. | UNKNOWN: role of unused records; JPEG color/metadata fidelity. |
| GIF | 2 records / 2 items | OBSERVATION: GIF89a inside 0xF01E records is detected and retained as GIF. | UNKNOWN: animation, indexed transparency and alternate GIF inputs in Publisher. |
| BMP / DIB | 0 | OBSERVATION about code: 0xF01F data goes through the existing DIB-to-BMP helper, then image detection and size reading. | UNKNOWN: controlled Publisher evidence, bit depths and malformed-header behavior; not established by these files. |
| TIFF | 0 | OBSERVATION about code: raster detection/size/decoding exists; the drawing parser recognizes 0xF029 headers. | UNKNOWN: a Publisher fixture exercising the path, page selection and color/alpha variants. |
| WebP | 0 | OBSERVATION about code: shared raster detection and decoding exists. | UNKNOWN: whether a Publisher-authored file preserves a WebP payload and exposes it through this picture path. |
| PSD | 0 | OBSERVATION about code: shared size/decoder support reads a merged composite. | UNKNOWN: whether a Publisher-authored file preserves a PSD payload through this path; no layered-picture promise. |
| WMF | 13 records / 14 items | OBSERVATION: existing limited vector conversion produces SVG, including selected geometry, pens/brushes and transforms; all observed conversions return no picture warnings. | UNKNOWN: visual fidelity and unhandled records; successful conversion is not proof of completeness. |
| EMF | 0 | OBSERVATION about code: same conversion entry point selects an EMF reader and creates SVG on success. | UNKNOWN: Publisher fixture confirmation and fidelity; not demonstrated by the WMF files. |
| PICT | 0 | OBSERVATION about code: explicitly unsupported; a referenced PICT produces an empty frame and the warning “PICT pictures are not supported yet; their frames were kept empty.” | UNKNOWN: a pinned PICT example and its desired visual result. |
| SVG (derived output) | 13 generated assets / 14 items; 0 stored SVG pictures | OBSERVATION: WMF conversions yield SVG assets which all parse. | UNKNOWN: direct embedded SVG in a Publisher file; the generic raster-size path is not proof of direct SVG import. |

OBSERVATION about WMF/EMF code: the existing drawing parser preserves the metafile wrapper. The importer calls its existing vector conversion and stores a successful result as an SVG asset. A conversion returning no drawable result leaves the frame empty with “A WMF or EMF picture could not be read; its frame was kept empty.” There is no fallback bitmap in that branch.

OBSERVATION about WMF/EMF code: recognized text records, embedded-bitmap records, and selected arcs/pattern fills are skipped with the warnings “Text inside WMF/EMF pictures was left out.”, “Bitmaps inside WMF/EMF pictures were left out.” and “Arcs and pattern fills inside WMF/EMF pictures were left out.” Both record loops also have an unhandled-record branch that does nothing without a new warning. Limits can stop a partial conversion. These are limitations of present code, not new claims about the formats.

OBSERVATION about PICT code: referenced record type 0xF01C returns no asset and no Graphic content; the frame is retained with the explicit unsupported warning. This was not exercised by a pinned data file. For other raster payloads, failure to read a pixel size normally keeps an empty frame with a decode warning. The DIB helper can return no data before that warning branch; a warning on every invalid DIB is therefore not established here.

UNKNOWN: whether any real WMF picture is missing an unhandled record's visible effect. The thirteen SVG outputs parse, but neither absence of warnings nor those parses compare their content with Publisher.

## Files that would settle remaining questions

INFERENCE: request 28 small Publisher-authored files, each with its corresponding Publisher PDF or PNG for scoring only, all using original or clearly redistributable artwork. Keep page, frame, artwork, version and other settings fixed except for the stated change:

- INFERENCE: 10 format controls, one each for PNG, JPEG, BMP, GIF, TIFF, WebP, PSD, WMF, EMF and PICT. Use the same asymmetric colored-corner artwork and record the inserted format. UNKNOWN: which formats the chosen Publisher version accepts or transcodes; the saved binary must establish what was retained, not the filename.
- INFERENCE: 8 metafile variations: four WMF and four EMF, each adding only text, an embedded bitmap, an arc, or a pattern fill to its format control. These isolate the omissions named by the existing warnings.
- INFERENCE: 6 PNG variations, one each changing only cropping, 30-degree rotation, a border, partial transparency, alt text, or an oval mask. Existing controls measure several frame positions, but not corresponding Publisher pixels for all these features.
- INFERENCE: 2 reuse/layout variations, one putting the PNG in a group and one placing the same PNG twice. These test real grouped-picture behavior and asset reuse without treating a group as one picture.
- INFERENCE: 2 otherwise identical plain WordArt files differing only in their text, to narrow the role of the unused square JPEG records. UNKNOWN: whether that pair alone can settle their role; a changed or unchanged record is evidence to record, not automatic proof of a preview or texture.

## Recommendation

OBSERVATION: all 29 unmatched references fail to find a picture candidate because their current output is another object category. There is no demonstrated misplaced, undecodable or unsupported native picture among these 29. The largest subcategory is painted geometry, with 17 references.

INFERENCE: the largest useful coverage improvement would be a separate comparison for rasterized non-picture objects, while retaining the existing picture-only metric. It could account for up to all 29 references; a first painted-geometry slice would address 17. It must not relabel these as native-picture successes or weaken any gate. No image-decoder change is shown to increase the current twelve native-picture matches for this set.

OBSERVATION: the scorer is in crates/pub/tests/corpus.rs, an unprotected file. The current image placement and vector conversion are in crates/pub/src/lib.rs and crates/pub/src/metafile.rs, also outside the three protected readers. No edit to any of them is proposed as part of this notes-only task.

INFERENCE: a separate non-picture comparison is medium-sized diagnostic/test work. Its main risk is comparing rasterized painted extents with text frames, table frames or a single group child and reporting false matches. Publisher's rounding and missing pixel references remain UNKNOWN; bounds alone cannot establish fidelity.

INFERENCE: for actual picture fidelity, collect the controlled WMF/EMF files before changing conversion. WMF is the largest placed format here (14 items), but a code change based only on counts would be unjustified. A small warning-disclosure audit in metafile.rs may be feasible; adding full missing metafile text/bitmap/arc/pattern support is likely large and risks coordinate, font and clipping errors. UNKNOWN: the number of real-file visual defects it would fix, and whether it would fix any present defect at all. It would not resolve the 29 non-picture unmatched references.

## Complete file census and source hashes

OBSERVATION: every pinned publication is listed below. “No exposed records” means the current parser returned none and the importer produced no picture asset; it is not a claim about all possible picture storage. Refused/damaged entries are unavailable, not zero-picture findings. All readable files without a detailed record row above have no exposed pictures, placement references or output picture items.

| Set | File | Stored records | Placed items | Finding | SHA-256 |
| --- | --- | --- | --- | --- | --- |
| controlled | `fill__fill-green.pub` | 0 | 0 | OBSERVATION: No exposed records | `140bc6d105976866a894c5ad869cae4c3d4e53792182cd8b95e72ca2356ef03e` |
| controlled | `fill__fill-grey.pub` | 0 | 0 | OBSERVATION: No exposed records | `f653a0279aed2777a32ecc4f0c07178116655d158a7c13ef8558e171fb1e72dd` |
| controlled | `fill__fill-orange.pub` | 0 | 0 | OBSERVATION: No exposed records | `679755392fd23b3e78abc496678c2a28af91fcb8b1fdd1fb05c53bcf07126432` |
| controlled | `fill__fill-red.pub` | 0 | 0 | OBSERVATION: No exposed records | `dd52ac497bedbe04d6b7d304fc27ef7769dbfb7c1a060c6f41b215857519f1b6` |
| controlled | `fill__fill-white.pub` | 0 | 0 | OBSERVATION: No exposed records | `b42326eeee9eb0a3213510b67f4795cdcabd49d75c090910f345e19b40f0fe61` |
| controlled | `fill__gradient.pub` | 0 | 0 | OBSERVATION: No exposed records | `3de0b9e1abc3100335cd6d021b6c15d80f4a04731db883adde2148ae39a7af29` |
| controlled | `fill__no-fill.pub` | 0 | 0 | OBSERVATION: No exposed records | `29a1a6f2b432b288bf98cd2b00eae4c91ca5a84feb8d6d74bbf78fe02fb23e05` |
| controlled | `fill__scheme-accent1.pub` | 0 | 0 | OBSERVATION: No exposed records | `402ececd0e5ad6cf320eda56cc673bd92d553dc5c5c94110ec91a27b86baa4a2` |
| controlled | `fill__transparency-50.pub` | 0 | 0 | OBSERVATION: No exposed records | `9f39d034e94c082c605e74cde10b32fe5ec6f8a96ceb0d15d96e542686d6d449` |
| controlled | `geometry__base.pub` | 0 | 0 | OBSERVATION: No exposed records | `1e3d6873946014b99d13fc8e47ee9f3c9c8ef368bfa0b754a725e023e4d4c34d` |
| controlled | `geometry__flip-h.pub` | 0 | 0 | OBSERVATION: No exposed records | `f7dd8434332adac8a67fdf4e253cd104b4d8b3ad16f3448abd501a59503a9f9f` |
| controlled | `geometry__flip-v.pub` | 0 | 0 | OBSERVATION: No exposed records | `d2d4da9aa9ba00615f42c1a27e0639fc59af0793b5bbaf7442e48b442b7192fe` |
| controlled | `geometry__height-10.pub` | 0 | 0 | OBSERVATION: No exposed records | `da1a0124946404d017ff51e4ed21d2bab0614af75b01403036cd4489e15b057a` |
| controlled | `geometry__height-300.pub` | 0 | 0 | OBSERVATION: No exposed records | `d5c4203342c1538cf38009a85691542439844b3cbdb137a8c314ccaa911ed73d` |
| controlled | `geometry__left-0.pub` | 0 | 0 | OBSERVATION: No exposed records | `2d003f7cb246a601a1224975d1aa94d6ea0e81d29b24f394e3ced2ce0086ca45` |
| controlled | `geometry__left-1.pub` | 0 | 0 | OBSERVATION: No exposed records | `fea0686a4ecfaadc6ceef84037cd356c5a493a20759700b344c0d6e9df7be186` |
| controlled | `geometry__left-100.pub` | 0 | 0 | OBSERVATION: No exposed records | `7eca9f50be551171e99e6cd7314e5927f41092db5d676504da87131f7937ed76` |
| controlled | `geometry__left-306.pub` | 0 | 0 | OBSERVATION: No exposed records | `fc55f9528ee597e14e9b90831443e7bb2975c7d7fd52b5bc150d50b122f21a4b` |
| controlled | `geometry__left-500.pub` | 0 | 0 | OBSERVATION: No exposed records | `97179df47ce218e0be341a88668445958bf4f11439a13caa1dc3bc20e3a2dbcb` |
| controlled | `geometry__left-73.pub` | 0 | 0 | OBSERVATION: No exposed records | `c9dce9f6ded024cdeb5dbba9d6497c63274f28d69f4f236cce7faed4cc98eda3` |
| controlled | `geometry__rot-1.pub` | 0 | 0 | OBSERVATION: No exposed records | `6ae4fcd65f3a40e0a860f2fc946a862df2f939e47ba516bb972471c15ead0310` |
| controlled | `geometry__rot-180.pub` | 0 | 0 | OBSERVATION: No exposed records | `517e9c739f56672e6486f71a4e53768d0ff15ab5ea13cdf7a2cde40f46b471e4` |
| controlled | `geometry__rot-270.pub` | 0 | 0 | OBSERVATION: No exposed records | `d719acb601ad60c9eb3eaacadc3a9fc562e9b96be3a77d25a45a2397b7657c09` |
| controlled | `geometry__rot-30.pub` | 0 | 0 | OBSERVATION: No exposed records | `d4b4fa2c4db0d6533d9fddc722af0024fc39c30b3c2d7b76e7dbad67d30e2168` |
| controlled | `geometry__rot-359.pub` | 0 | 0 | OBSERVATION: No exposed records | `772bb25fbfd595bb7829f798d3caf52cb53ec6a8645ee4026c830cdd982e44bb` |
| controlled | `geometry__rot-90.pub` | 0 | 0 | OBSERVATION: No exposed records | `e360634ee3df510142dffb699c4a60a8bfe50c1cbd3a27b01682b5f3a0799a87` |
| controlled | `geometry__top-0.pub` | 0 | 0 | OBSERVATION: No exposed records | `c846a7e17cd34d20ddfc9b6e95e11d6eff1d4472d6a80cdc6e76def8375ceb86` |
| controlled | `geometry__top-396.pub` | 0 | 0 | OBSERVATION: No exposed records | `f2dec8a67177808512b34d4c908fd6f1e83b3a4fd75cbeb290163d6db44b5b68` |
| controlled | `geometry__top-700.pub` | 0 | 0 | OBSERVATION: No exposed records | `91e4f802c8cf3de112add7e5b3bae2506edb5d511b81a0c5701fd044226cabf5` |
| controlled | `geometry__top-74.pub` | 0 | 0 | OBSERVATION: No exposed records | `850dd82b85c9382843ba2037348ca416ae1d773035e09ab4f55eb1c719519f4e` |
| controlled | `geometry__two-rects-swapped.pub` | 0 | 0 | OBSERVATION: No exposed records | `a952da95e108fa3697ecceb4ee0936f7b3a2b780f4eff751eebf18b88e17231d` |
| controlled | `geometry__two-rects.pub` | 0 | 0 | OBSERVATION: No exposed records | `834cb91a54880211143b29a63ee61297a05bdc506dedd316b211195f5a0a2f97` |
| controlled | `geometry__width-1.pub` | 0 | 0 | OBSERVATION: No exposed records | `1fbe3169b7145be8952708929d767e607fb530d7e4a48dd7f6104d8c2f3b4f41` |
| controlled | `geometry__width-150.pub` | 0 | 0 | OBSERVATION: No exposed records | `5a1d8d731ef00e6861242764cd836f656242bcfd3022ae39f76786504f0628a0` |
| controlled | `geometry__width-612.pub` | 0 | 0 | OBSERVATION: No exposed records | `5a081f6b636c97fa1135aee399ba9376bce552fedd9b2dbdf6e0a53a46f0f1ec` |
| controlled | `group__rotated.pub` | 0 | 0 | OBSERVATION: No exposed records | `aea7f79db246dd5c34269108b1dab8b29c5541c6b1c4523d80b0d3630b160ce6` |
| controlled | `group__two.pub` | 0 | 0 | OBSERVATION: No exposed records | `d7cecf37f5c0325a8dc1187df424c30bfa8525bb3098e0b4a06790f394af5159` |
| controlled | `line__color-red.pub` | 0 | 0 | OBSERVATION: No exposed records | `f1e9fe2a07f69f9b8f777f9c5914716d713a08c9164f6b19f08cfd57e2a45980` |
| controlled | `line__dash-2.pub` | 0 | 0 | OBSERVATION: No exposed records | `52c2be877b0552c81fac34fa81a62f2bf4eff5c9edf2d8d67fb77f801f07f1fb` |
| controlled | `line__dash-4.pub` | 0 | 0 | OBSERVATION: No exposed records | `ddb223c5e4275ae92867468a51fee8a4e6bd41e1812dd2241eba040c761a2a6e` |
| controlled | `line__dash-5.pub` | 0 | 0 | OBSERVATION: No exposed records | `9885f150d7e4eb7955fbdf62543e7e9561a0447cd429328ee8684a2ea0d25129` |
| controlled | `line__no-line.pub` | 0 | 0 | OBSERVATION: No exposed records | `bee2971fe48138d329be877f94dbc7675c6e9aab59cf641b12a88bb567bcd9c7` |
| controlled | `line__weight-0.25.pub` | 0 | 0 | OBSERVATION: No exposed records | `af8ef6244d774c69996635b7947b3482ef4c4a1871c2042e2eb139ac7e03b96b` |
| controlled | `line__weight-1.pub` | 0 | 0 | OBSERVATION: No exposed records | `91aa8bee66da5d13efeb385ec69f15303b0afaf9acfc1a95dce7abcd7d15ba90` |
| controlled | `line__weight-10.pub` | 0 | 0 | OBSERVATION: No exposed records | `dd194825abb2c45cb7ca9b81f00dc6c11c58597d8bc4e139dcd0ed13edd737dd` |
| controlled | `line__weight-4.pub` | 0 | 0 | OBSERVATION: No exposed records | `c3c1a3f1aa72573a783c6b9c8b250e5fb63ab77349ba729b3de90485aa9a2233` |
| controlled | `pages__landscape.pub` | 0 | 0 | OBSERVATION: No exposed records | `22ad856c3b61fc601b1895bb72c8e921ed0330bb553ab6c8a720b599c15aac3d` |
| controlled | `pages__master-ignored.pub` | 0 | 0 | OBSERVATION: No exposed records | `92409f24827f069f1503c82469cabbe911d697b312b0ded4bfd66607dabc67eb` |
| controlled | `pages__master-rect.pub` | 0 | 0 | OBSERVATION: No exposed records | `400c061402e471bdede31c8f4d9db4054693233f098c7ded426988f9ef89c218` |
| controlled | `pages__master-text.pub` | 0 | 0 | OBSERVATION: No exposed records | `8137a8885c89f9f06bf922f532dba66246ca955922b7ccf0c4bdc54733b1479f` |
| controlled | `pages__one.pub` | 0 | 0 | OBSERVATION: No exposed records | `18905060eb45f6aede59991550ce688ce9cf1e5d03beb7f16ea3fadb979551fc` |
| controlled | `pages__size-5.75x4.75.pub` | 0 | 0 | OBSERVATION: No exposed records | `abd024933717e4806054f99b66bb9ddc8d5eac7a7c51803281c916a2ff97ee2c` |
| controlled | `pages__three-rect-p3.pub` | 0 | 0 | OBSERVATION: No exposed records | `3c15cd70034b50a63831a2d248acc9d06621f241c494439d8a07a233e5556459` |
| controlled | `pages__two-rect-p2.pub` | 0 | 0 | OBSERVATION: No exposed records | `bf5d98cb2082e73a99a53a797ffc07353c432ed22a8ea943ff9930d8aba04284` |
| controlled | `pages__two.pub` | 0 | 0 | OBSERVATION: No exposed records | `815fad77c79fa4f88fbde52b4001bc5ef2097d5f4751965518447122ae03ef5f` |
| controlled | `picture__base.pub` | 1 | 1 | OBSERVATION: 1 PNG | `39a32b2e6ce8073fbfe9a2d66cb9ab8b4e0d0952bb294a06ac609a3749f071b3` |
| controlled | `picture__border.pub` | 1 | 1 | OBSERVATION: 1 PNG | `97907db68b160cd9d9c87eb31f39ad1474cb9d524df6988d51cac82c3a6adb9f` |
| controlled | `picture__crop-all.pub` | 1 | 1 | OBSERVATION: 1 PNG | `a8201d751ce462e5946a0813465f5fe61b38eb25227c4afdf600fc1c6f010012` |
| controlled | `picture__crop-left.pub` | 1 | 1 | OBSERVATION: 1 PNG | `49ae8c6529cc2a725677f3ec191b2be0e2bcad86d68eaa7887fccac94487aa31` |
| controlled | `picture__crop-top.pub` | 1 | 1 | OBSERVATION: 1 PNG | `2db5fd28b05640c1e4ccc8d5826dcd560b166feba4d5d8ab96bfa7dcb25d2ebf` |
| controlled | `picture__flip-h.pub` | 1 | 1 | OBSERVATION: 1 PNG | `3637e89b38d233a73f0c4b5e1b28e93ff4aa5dcce96ad2ecaf7f0714340405ca` |
| controlled | `picture__moved.pub` | 1 | 1 | OBSERVATION: 1 PNG | `ec1964648b15eb40b6e0cae9ca2118173343eaac091d3232f24cd655768127f8` |
| controlled | `picture__oval-mask.pub` | 1 | 1 | OBSERVATION: 1 PNG | `0ef974b5861894d463493aed878f2e36f8147369a1ae10593386f8e77a04a0a8` |
| controlled | `picture__rotated-30.pub` | 1 | 1 | OBSERVATION: 1 PNG | `15bdddc73b1b3866207739538fe4c6596e7cbd850aedf7ed3bbad0719738e55f` |
| controlled | `picture__stretched.pub` | 1 | 1 | OBSERVATION: 1 PNG | `bb619e30fb9b49619b45ceed21daa9cb9c505fd4fbb791ce44b02428308f077d` |
| controlled | `picture__two.pub` | 1 | 2 | OBSERVATION: 1 PNG | `97dcd67cc415bd962589122fe39f95d32304d2733005c54c5fdc711273007d3d` |
| controlled | `shadow__offset.pub` | 0 | 0 | OBSERVATION: No exposed records | `2b8c3cd8478da53e364097b92e2978b8c6f9037a0b576dce84060dd98684eaad` |
| controlled | `shadow__on.pub` | 0 | 0 | OBSERVATION: No exposed records | `b01c8e35d89bcccc1d1e13e85be2c48eb82128c8a88d36adbfd4503f54f8d209` |
| controlled | `shapes__curve.pub` | 0 | 0 | OBSERVATION: No exposed records | `a932e1e445dbf5e088e67bcb6c2344f8ef78f7cb2c140c99d39d13f79adf6501` |
| controlled | `shapes__diamond.pub` | 0 | 0 | OBSERVATION: No exposed records | `b5439878dd52c0a54214a92bb731e415ac91ca3083a9e711e42d92439817d6af` |
| controlled | `shapes__heart.pub` | 0 | 0 | OBSERVATION: No exposed records | `15e55be8782f648de3f48960f97ceac0200a55d80efaa015fa6bdbeac87ae9f5` |
| controlled | `shapes__hexagon.pub` | 0 | 0 | OBSERVATION: No exposed records | `58981e80239daa7a147484d5d05dad17c55e4f51260b19b403a61f335a03c2a8` |
| controlled | `shapes__line-arrow.pub` | 0 | 0 | OBSERVATION: No exposed records | `b727ec743dc1a7c57eb46103673043afa51fea8451d85d9b95d30968488f27a1` |
| controlled | `shapes__line.pub` | 0 | 0 | OBSERVATION: No exposed records | `c02ab6f95f190fb8d1923e7e9965b19f23d77b9e79cfa157944b736047f8bbe0` |
| controlled | `shapes__oval.pub` | 0 | 0 | OBSERVATION: No exposed records | `29cb3020390b751a5c66767688be231a2f7d77a1776d0b2c0060fbdc89f64fdb` |
| controlled | `shapes__polyline.pub` | 0 | 0 | OBSERVATION: No exposed records | `750d1261225acaf73a47d8987118626a39975cc53ce791f5961955757ad601a6` |
| controlled | `shapes__rect.pub` | 0 | 0 | OBSERVATION: No exposed records | `9f533e45948ddc0be2f7be85c97d774766ce36e2d57a1209ef8bb9fbde6c8513` |
| controlled | `shapes__rightarrow.pub` | 0 | 0 | OBSERVATION: No exposed records | `1fd8cdabecd5c9345f9f2a00eeeede7f126c68e20809dea7b3a2423e8a4e5b29` |
| controlled | `shapes__roundrect.pub` | 0 | 0 | OBSERVATION: No exposed records | `6e5b1b4c63a0ee226ca9a8c6c1c863be5481854bb64baf88ed03d2e53c0e1e33` |
| controlled | `shapes__star5.pub` | 0 | 0 | OBSERVATION: No exposed records | `b64f6a7ef259287a8b35ef97c8d40ccb425eb55b54953b3255798cc82a5a95f2` |
| controlled | `shapes__triangle.pub` | 0 | 0 | OBSERVATION: No exposed records | `b6c270e0c709cbfe90b2270c6afc10d17499dfd4d15feca0fcd16dc00d3a8616` |
| controlled | `table__2x2.pub` | 0 | 0 | OBSERVATION: No exposed records | `72d2127f4ffdbe998f4f94414a28584e49af817991b70a564fda885dcb1a6a7d` |
| controlled | `table__3x2.pub` | 0 | 0 | OBSERVATION: No exposed records | `12c45cd9057553f748d0cf23ab5348d901be294b1d6e790eb80c3331d57d2616` |
| controlled | `table__merged.pub` | 0 | 0 | OBSERVATION: No exposed records | `3df51f5e3e29f247b8636627725d43f5973dc7ea887338f32aec8f4b49a23b30` |
| controlled | `text2__base.pub` | 0 | 0 | OBSERVATION: No exposed records | `7d32fbec55d474c67d7a344db4fff17cb1f9216cdda6d736ab2c8424203662b6` |
| controlled | `text2__p2-bold.pub` | 0 | 0 | OBSERVATION: No exposed records | `36baee97f339fcdbde70984d9c4bc851844948375a95601d35e75c9fdca43e50` |
| controlled | `text2__p2-center.pub` | 0 | 0 | OBSERVATION: No exposed records | `6d7a87753090be349d6eec1a1196f9f6b92776073acc2adc6661f4dadf49d1d3` |
| controlled | `text2__text-in-oval.pub` | 0 | 0 | OBSERVATION: No exposed records | `270e0d486423e8a869cb01f72487771ef6fae1e650632ed995d656b29473def2` |
| controlled | `text2__two-boxes.pub` | 0 | 0 | OBSERVATION: No exposed records | `a6405f9b31de04d69a3fdd4179fe97cbfc30e629a8c874e40da80cceaeba0d48` |
| controlled | `text__align-center.pub` | 0 | 0 | OBSERVATION: No exposed records | `226cf7e6918146193e531520b9005e06f434b8f62ceaf700b455bc8ac4fb3713` |
| controlled | `text__align-justify.pub` | 0 | 0 | OBSERVATION: No exposed records | `c5442fe547575f882d51809a315a6916a7233895cafe849e20e09e8e9409b739` |
| controlled | `text__align-right.pub` | 0 | 0 | OBSERVATION: No exposed records | `d1aa1cacbbb272f8b6456c79cf1e024218e15bb40e0f979a4ee53dbf0ea1f969` |
| controlled | `text__allcaps.pub` | 0 | 0 | OBSERVATION: No exposed records | `f95eb0920c19eafd027884bb33ed31389d4aad91094755df1bb9f777a4dc8a25` |
| controlled | `text__base.pub` | 0 | 0 | OBSERVATION: No exposed records | `0c9293e727559fe58ba150e8445b1365b4960883fbe6759c10b2477f128ff9a7` |
| controlled | `text__bold.pub` | 0 | 0 | OBSERVATION: No exposed records | `90a140c2db6c6c8399c848a26a4e7c9ce822d52d219f5589ee026345c2e05635` |
| controlled | `text__bullets.pub` | 0 | 0 | OBSERVATION: No exposed records | `51df14aa94d640932656bc49a3f645b97333ba811467761e31226ec1547011f5` |
| controlled | `text__color-custom.pub` | 0 | 0 | OBSERVATION: No exposed records | `62436393aa82998736c87930793ca8d019162d8fadcfc67700c36c6db5d96c6f` |
| controlled | `text__color-red.pub` | 0 | 0 | OBSERVATION: No exposed records | `8402349638c905016b5d56ee2d4356c906015631b371985378e676d8b6c4ad0a` |
| controlled | `text__font-Arial.pub` | 0 | 0 | OBSERVATION: No exposed records | `9249dd72feeab344a9a5b2d366c4b6973a90843b8090cd8d8831696cdce6295d` |
| controlled | `text__font-ArialBlack.pub` | 0 | 0 | OBSERVATION: No exposed records | `024b15dc2154c4779796e0ea748fdff763bf065f1750cd938eeb07312461e42f` |
| controlled | `text__font-CourierNew.pub` | 0 | 0 | OBSERVATION: No exposed records | `d8f715a6f23a8bf396ccb65da270c62e52a23fc1f26465421d0c413d730010a2` |
| controlled | `text__font-TimesNewRoman.pub` | 0 | 0 | OBSERVATION: No exposed records | `5bd50ace94c87c2962ce97221fffb5ebfcd2c56f46808e3aa275fbcd4b06b77d` |
| controlled | `text__frame-fill-line.pub` | 0 | 0 | OBSERVATION: No exposed records | `59a0bfa1e5642f9164fb7d4b7557e2c080ddc7abfe76276506d21396951500a3` |
| controlled | `text__indent-first-18.pub` | 0 | 0 | OBSERVATION: No exposed records | `2a4a4f3b861c9514d12105312a86e94c6b310545151fe9c238b32e4870035634` |
| controlled | `text__indent-left-18.pub` | 0 | 0 | OBSERVATION: No exposed records | `fe22b3b44baa690da104712645f606a755b9c626bdb8b5b7b6fc5d6ef4abb5c9` |
| controlled | `text__indent-right-18.pub` | 0 | 0 | OBSERVATION: No exposed records | `91fa9d4f838c8909ece6d16ba5c98d5b1bee4f6a39484ffd95bc8b4f17ce332d` |
| controlled | `text__italic.pub` | 0 | 0 | OBSERVATION: No exposed records | `60216ffe5ee57f80c546bd99724086c9693db682e11d3ad60e2ae2ec31eb167e` |
| controlled | `text__line-break.pub` | 0 | 0 | OBSERVATION: No exposed records | `ac822b60ab7365ab82a3d2a8bbd9edc5f9061407d111749c5942b73fc53a125c` |
| controlled | `text__ls-double.pub` | 0 | 0 | OBSERVATION: No exposed records | `9242fc72b4ff5ab82745969006895f6d7a085d190820b2c28a62d04fdb3980a4` |
| controlled | `text__ls-exact-14.pub` | 0 | 0 | OBSERVATION: No exposed records | `22191bec6bd65bbc214760d982e1b44091ec9c11d8ecd3076ac6bf8d5f58d188` |
| controlled | `text__ls-exact-30.pub` | 0 | 0 | OBSERVATION: No exposed records | `69067437491eec0df68a82f812177713207942246444cfbd8db040db8e6a767d` |
| controlled | `text__ls-multiple-1.5.pub` | 0 | 0 | OBSERVATION: No exposed records | `35e0d9d58aab018b259a4a3cd95f05de3629a56e13c5a69109f06a7a6418313f` |
| controlled | `text__ls-single.pub` | 0 | 0 | OBSERVATION: No exposed records | `e8957ec0950137eb616288ba1dd7a882798eef128786c0610aaba88e592907d3` |
| controlled | `text__margins-10.pub` | 0 | 0 | OBSERVATION: No exposed records | `4708cc8ef89dc69507b1ccb0fc1ef13bd837592b366b1d3507a7bc85fe81796a` |
| controlled | `text__rotated-270.pub` | 0 | 0 | OBSERVATION: No exposed records | `ceae546e1c875acf4349a577263e76242ec76805d08211dc6c550f2a0f84427f` |
| controlled | `text__run-bold-word.pub` | 0 | 0 | OBSERVATION: No exposed records | `94d1838e1668d6c0a090e1841944e2295f5229c0a927dcdf5210117f36bc0063` |
| controlled | `text__run-font-word.pub` | 0 | 0 | OBSERVATION: No exposed records | `8bec2c8a3d35ff8a3d3b89cbb48b51f24337e33ae34486186bb4163eff523cdb` |
| controlled | `text__run-red-word.pub` | 0 | 0 | OBSERVATION: No exposed records | `6ab61f70310cb4d2adedfe0d670c11ae8ffadf037874536748efa9e1f15f924e` |
| controlled | `text__run-size-word.pub` | 0 | 0 | OBSERVATION: No exposed records | `089e2a4dd7439549dde6d58e9240c7d5342e201a7edf6ab96ea2e214b0f0ecc0` |
| controlled | `text__size-12.pub` | 0 | 0 | OBSERVATION: No exposed records | `78aba312d290c8fa8c29a93de07e611689000df7924eb8988af35dbb51e25a89` |
| controlled | `text__size-24.pub` | 0 | 0 | OBSERVATION: No exposed records | `80f027a7aca4cf537134bed35eec5ad812794518033541f809b85b24d847df16` |
| controlled | `text__size-36.5.pub` | 0 | 0 | OBSERVATION: No exposed records | `31f4aefa4307098e57a47ff4db35fb8b797d1c71dadf6e7e27ee938c892b03cc` |
| controlled | `text__size-72.pub` | 0 | 0 | OBSERVATION: No exposed records | `7967783a2a96012febe6b9a7c7222c4c24bca1f02170bdc56f9485617ea7ecfe` |
| controlled | `text__size-8.pub` | 0 | 0 | OBSERVATION: No exposed records | `f86bbf9bfac4e67cf34f5ded0cbb5d50fded0fd2634b141f41cbce02cbec7b25` |
| controlled | `text__smallcaps.pub` | 0 | 0 | OBSERVATION: No exposed records | `76ea2fea4a7885054bc1f013971d2fc3acdf7f3c012541520451ddda304eb6c1` |
| controlled | `text__space-after-0.pub` | 0 | 0 | OBSERVATION: No exposed records | `da8a9b5fb487c0acff844a5189fb5362b0da966f330c841f50c8db5007b5c2f8` |
| controlled | `text__space-before-12.pub` | 0 | 0 | OBSERVATION: No exposed records | `260f23e741dc2b29fccd4203c1639c07ef5fbbfc50f75351a239721c93da973a` |
| controlled | `text__superscript.pub` | 0 | 0 | OBSERVATION: No exposed records | `ae1f26ebbc06c7f8904d18ac0e559076256f3f8b9e0a02e11c49e96fca4e6604` |
| controlled | `text__text-changed.pub` | 0 | 0 | OBSERVATION: No exposed records | `21fe1e79b4da37a0cefd87cd494c5bb977073a39be883f38dba068f7a403c200` |
| controlled | `text__text-longer.pub` | 0 | 0 | OBSERVATION: No exposed records | `7fe759e9f7d513e01332717020ca662338bce1251a2d16f28e155c484b96afe9` |
| controlled | `text__tracking-200.pub` | 0 | 0 | OBSERVATION: No exposed records | `f9bc7d51af14bdd8b5eb289225a79738e39b677e74b4f7dcc36411dcc6de2664` |
| controlled | `text__underline.pub` | 0 | 0 | OBSERVATION: No exposed records | `3a4baac97c143a1ae519e4c149fe8df0ce0fec74119d1c1f6109238b116d5f5d` |
| controlled | `text__vanchor-bottom.pub` | 0 | 0 | OBSERVATION: No exposed records | `886be8dbfa7d2c2590ea408f4300a2014823da7585652419badd0ea207d2060e` |
| controlled | `text__vanchor-middle.pub` | 0 | 0 | OBSERVATION: No exposed records | `f92f73a3e92f700bc658a4438a1e30fa692bcc1370bed3570484d808b6299603` |
| controlled | `wordart__arch.pub` | 1 | 0 | OBSERVATION: 1 JPEG | `dc22fb9983e9119c9969e37dd5ba3d19fb63fa489c8ffeec4e38ea921a41904b` |
| controlled | `wordart__base.pub` | 1 | 0 | OBSERVATION: 1 JPEG | `270d8fe5d1a2286622b35e40b39e00eac17b878734aedb755059b3fc20050838` |
| controlled | `wordart__circle.pub` | 1 | 0 | OBSERVATION: 1 JPEG | `bc145b67097b5d9d42f0f5c838d68f2d88130267b51031bdeacd4cb9d0590eeb` |
| controlled | `wordart__fill-red.pub` | 1 | 0 | OBSERVATION: 1 JPEG | `e6745af79ae3d98bf3b79e3104e291f158934da6a8969dd722e24d0eecd9e6f4` |
| controlled | `wordart__font.pub` | 1 | 0 | OBSERVATION: 1 JPEG | `1c19cda5f6a40f1cb6f64f3b67a334ff54656bce573be480083c7bdc1bb0fd8a` |
| controlled | `wordart__line-white.pub` | 1 | 0 | OBSERVATION: 1 JPEG | `5968f890c0b4029dde31c644cb3ff29eb071225e685ea4fc4a527058f675755e` |
| controlled | `wordart__shadow.pub` | 1 | 0 | OBSERVATION: 1 JPEG | `d24ea44009d4e8c00860a61aec2391cbb3e30fd791deb5f5f480382218cb102d` |
| controlled | `wordart__text.pub` | 1 | 0 | OBSERVATION: 1 JPEG | `f3d6d544b2e47af20f49c20dbe9151e9c41eb53fbf655a685f8c4cb4cb46d62e` |
| real | `51318.pub` | 1 | 1 | OBSERVATION: 1 PNG | `3ab75a6a9196e0a51fc9b0aa759459501c71d313030c06652aabffbae0a2ab09` |
| real | `60685.pub` | 0 | 0 | OBSERVATION: No exposed records | `0d3cecf0ff000a20931d0de3ebc48040bea10e03833ade3f9902f996c20e7184` |
| real | `LinkAt0And10.pub` | 0 | 0 | OBSERVATION: No exposed records | `c59b7160c21a531e5872e76141c4cd43a4d74f30d78401f64175467fa40c44fc` |
| real | `LinkAt10.pub` | 0 | 0 | OBSERVATION: No exposed records | `f1b7f65cb9646fddc2cfc77a795109bb958f629e4fbf7c9fc7a4d1b045074c35` |
| real | `LinkAt10And20And30.pub` | 0 | 0 | OBSERVATION: No exposed records | `202ac6f82e2a33ef0181158728b5c4df6781ccb105e012f520d331acafbcebf1` |
| real | `LinkAt10And20And30And40.pub` | 0 | 0 | OBSERVATION: No exposed records | `b54068d4f82dd0ee533f2a655a7edd2acbdc02619621b32fb23cb9ce842ed669` |
| real | `LinkAt10Longer.pub` | 0 | 0 | OBSERVATION: No exposed records | `a917a9e575f5cfc1ba8e78226981b662b06f96da2d7f2eb3e2b00a4b4f3dada5` |
| real | `LinkAt20.pub` | 0 | 0 | OBSERVATION: No exposed records | `cec79a2bd2f6bc71dd276bd6584c34d66fa6f710d9adbed951bd9cb32c5b2fed` |
| real | `Sample.pub` | 0 | 0 | OBSERVATION: No exposed records | `6fefdef46b87c767150878dc384549cb2d2ec2ac54de25f8ddb3a5628301107e` |
| real | `Sample2.pub` | 0 | 0 | OBSERVATION: No exposed records | `c87ea7cc5606023c5fa99376631bbd45a8e9bdaba34c958057995b0e7ac8cc86` |
| real | `Sample2000.pub` | Unavailable | Unavailable | UNKNOWN: refused older format; no supported drawing stream | `40701ca47b26d04771cdd58467764e9ab39d3da69b59fbc529afd16d263d2f86` |
| real | `Sample2_2010.pub` | 0 | 0 | OBSERVATION: No exposed records | `931ff20980b8f72aab227fa3cbe61b017be8d17c337fecb9aca3c97075940777` |
| real | `Sample3.pub` | 0 | 0 | OBSERVATION: No exposed records | `424c69173ff08948c2529c8084b4ac2403f1ff1057146f4edd02fc29b44481fc` |
| real | `Sample3_2010.pub` | 0 | 0 | OBSERVATION: No exposed records | `baa9555a72d9e4264e3fd722896e2e43b960d20b4afdb96f75cb721a6133ff91` |
| real | `Sample4.pub` | 0 | 0 | OBSERVATION: No exposed records | `42195f7ad23d911219fea3ec88e66e867e9b9a6821a16dd1b535e2aa9d57a11b` |
| real | `Sample4_2010.pub` | 0 | 0 | OBSERVATION: No exposed records | `544e2043f91aed664c0a09f9ae45da42291b6730f107f99f3cdf9462ed38fb02` |
| real | `Sample98.pub` | Unavailable | Unavailable | UNKNOWN: refused older format; no supported drawing stream | `8912f295ff0cd1f55b873b3b98a182218f33c420877e025d2f0fd795f12dd0c0` |
| real | `SampleBrochure.pub` | 8 | 8 | OBSERVATION: 2 GIF, 1 JPEG, 5 WMF | `ffed034ac87e679f0bd08ff9cf74ad11c0e0e510a42b1bc1a7502415f6c29c87` |
| real | `SampleNewsletter.pub` | 9 | 10 | OBSERVATION: 1 JPEG, 8 WMF | `6a825ba26ba35d6e885acdc62e859591ed37cb0ff7480b554b9cb362b644dfcf` |
| real | `Sample_2010.pub` | 0 | 0 | OBSERVATION: No exposed records | `f0a90aa566d72eb822eb48b4f378c33007610e35f3f1b999cdb88f2c9683858b` |
| real | `Simple.pub` | 0 | 0 | OBSERVATION: No exposed records | `2606f530052a818c7949b88c9bf245dbc0a74ddc6b5a60004f37b387fe4b955d` |
| real | `clusterfuzz-testcase-minimized-POIHPBFFuzzer-4701121678278656.pub` | Unavailable | Unavailable | UNKNOWN: damaged compound file; no inventory | `07b48fcd8a3b85a83b054ea9722352d5269591d2482fb710002794da49f48b26` |
| real | `clusterfuzz-testcase-minimized-POIHPBFFuzzer-4918886059278336.pub` | Unavailable | Unavailable | UNKNOWN: damaged compound file; no inventory | `2641d2d48a345f0eb52b4dd0151330142e3037505fea614c272ec7eba4b3e9b0` |
| real | `clusterfuzz-testcase-minimized-POIHPBFFuzzer-6325615354773504.pub` | Unavailable | Unavailable | UNKNOWN: damaged compound file; no inventory | `e06b81d1a11a592d890e8d77c7f243c0529a36a0ec9bc4f18e8b1afb9e4f7bd2` |

OBSERVATION: the three compound-file failures are:

- OBSERVATION: `clusterfuzz-testcase-minimized-POIHPBFFuzzer-4701121678278656.pub`: `Malformed FAT (sector 129 pointed to twice)`.
- OBSERVATION: `clusterfuzz-testcase-minimized-POIHPBFFuzzer-4918886059278336.pub`: `Malformed FAT (FAT has 244 entries, but file has only 64 sectors)`.
- OBSERVATION: `clusterfuzz-testcase-minimized-POIHPBFFuzzer-6325615354773504.pub`: `Invalid CFB byte order mark (expected 0xFFFE, found 0xFFFA)`.

UNKNOWN: pictures in the two refused old-format files or the three damaged files. Their records are not counted in format totals. Visual completeness of real-file clip art, exact color/transparency behavior, direct EMF/PICT/DIB/TIFF/WebP/PSD Publisher input coverage, and the unused WordArt JPEG role remain unresolved. Windows and Publisher execution were not tested.

## Validation

OBSERVATION: the baseline full corpus gate, save/reopen/edit/PDF check, real batch, formatting, explicit warnings-denied clippy and all seven workspace CI steps pass. The first complete draft records observations only; final repeat results will be added after review. Local raw inventory, scoring-only classification and validation logs are under target/ac-review and are not committed. No markdown table in this note contains a blank line.
