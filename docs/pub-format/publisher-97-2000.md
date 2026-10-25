# Publisher 97, 98 and 2000: research boundary

## Result

OBSERVATION: this is notes-only research on main `71adf6a`. The requested MS-PUB path returned HTTP 404, while Microsoft's Office File Formats catalog and technical-document list returned HTTP 200. Neither the catalog's contents metadata nor its technical-document list contains a Publisher/MS-PUB entry. This is a missing-resource result, not a proxy access denial. No other reader, commercial application, external format description or repository source was consulted.

UNKNOWN: no MS-PUB specification was obtained. There is therefore no verified Publisher-version statement or applicability note to paraphrase. It would be wrong to invent a header definition or say a missing specification explicitly excludes a release. The result is **no verified MS-PUB coverage**, not proof that no Publisher specification exists anywhere.

| Publisher release | Verified coverage from MS-PUB |
| --- | --- |
| 97 | UNKNOWN: no applicable specification obtained; neither full nor partial coverage established. |
| 98 | UNKNOWN: no applicable specification obtained; neither full nor partial coverage established. |
| 2000 | UNKNOWN: no applicable specification obtained; neither full nor partial coverage established. |
| 2002 or newer | UNKNOWN as a Microsoft Publisher specification claim; the existing importer and local format notes describe observed current-family files. |

OBSERVATION: the permitted MS-CFB specification was reachable, published revision 12.0 dated 2024-04-23. It defines a general compound-file container, not Publisher's application-specific streams. Its applicability statement is about storage of objects and does not list Publisher 97/98/2000. Its structure versions 3 and 4 are **container versions**, not Publisher product releases; the specification's revision 12.0 is a documentation revision, not either kind of file version.

INFERENCE: **no-go for implementing older Publisher import now**, with a **large** provisional size estimate. The main risk is inventing the publication directory, text-store and cross-stream associations without an authorized older-layout description. More labelled samples would improve coverage, but samples alone do not authorize decoding an undocumented layout in this task. An authoritative specification URL, or a separate explicit research instruction within the clean-room rules, is needed before implementation.

## What the unchanged importer can establish

OBSERVATION: the audit used the same `cfb::CompoundFile::open`, `/Contents` stream open/read and `import_pub` paths as the current importer. It recorded only the validated container version, the first four Contents bytes already checked by the importer, and the import result/diagnostics. The diagnostic wrapper remains outside the checkout. No older Contents record directory, Quill body, Escher body or other undocumented older structure was decoded. The old files are refused before those readers run.

OBSERVATION: `import_pub` recognizes `E8 AC 2C 00` as its 2002+ family and `E8 AC 22 00` as its shared 97/98/2000 refusal family. These are **existing implementation selectors**, not verified MS-PUB version-field definitions. `Imported` does not report a specific creating/saving Publisher release or build. `is_pub` checks the container and a shorter Contents prefix, not an exact Publisher release.

OBSERVATION: MS-CFB §2.2 documents Major Version and Minor Version in the outer file header. The existing container interface reports version 3 for all 163 readable containers, corresponding to Major Version `0x0003`. It does not expose the minor field in this audit; the specification recommends `0x003E`, but that recommendation is not an observed value for these files. No raw offset reader was added to infer further Publisher fields. There are **no verified Publisher-specific header version fields for any table row**.

| Existing-path classification | Controlled | Apache | Total |
| --- | ---: | ---: | ---: |
| Accepted, 2002+ selector | 142 | 19 | 161 |
| Refused, shared 97/98/2000 selector | 0 | 2 | 2 |
| Damaged container, Publisher selector not reached | 0 | 3 | 3 |
| Total | 142 | 24 | 166 |

OBSERVATION: **two files enter the importer's older-than-2002 refusal branch**: Sample98.pub and Sample2000.pub. Both have the same selector and the same refusal reason. UNKNOWN: their exact release/build, which one was last saved by which Publisher version, whether any 97-specific file is present, and the original application version of any damaged file. The names suggest 98 and 2000 but are not proof. Thus two is the importer-classified older-format count, not an independently verified product-version census.

### Every file, grouped by the importer's version family

OBSERVATION: the following tables list all 166 binaries, including the two refused files. In the version-fields column, `CFB major 3` is documented container information exposed by the existing path; `Publisher UNKNOWN` means no MS-PUB field definition or release value could be verified. CFB Minor Version remains unreported in every row. Accepted means conversion succeeded, not that it matches Publisher.

#### 2002+ selector: E8 AC 2C 00

OBSERVATION: all rows in this group return the current-family selector and import successfully. The exact Publisher product version is not reported.

| File | Documented version fields available | Importer verdict |
| --- | --- | --- |
| `pubsmith-fixtures/fill__fill-green.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/fill__fill-grey.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/fill__fill-orange.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/fill__fill-red.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/fill__fill-white.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/fill__gradient.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/fill__no-fill.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/fill__scheme-accent1.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/fill__transparency-50.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/geometry__base.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/geometry__flip-h.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/geometry__flip-v.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/geometry__height-10.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/geometry__height-300.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/geometry__left-0.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/geometry__left-1.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/geometry__left-100.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/geometry__left-306.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/geometry__left-500.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/geometry__left-73.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/geometry__rot-1.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/geometry__rot-180.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/geometry__rot-270.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/geometry__rot-30.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/geometry__rot-359.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/geometry__rot-90.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/geometry__top-0.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/geometry__top-396.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/geometry__top-700.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/geometry__top-74.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/geometry__two-rects-swapped.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/geometry__two-rects.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/geometry__width-1.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/geometry__width-150.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/geometry__width-612.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/group__rotated.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/group__two.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/line__color-red.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/line__dash-2.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/line__dash-4.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/line__dash-5.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/line__no-line.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/line__weight-0.25.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/line__weight-1.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/line__weight-10.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/line__weight-4.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/pages__landscape.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/pages__master-ignored.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/pages__master-rect.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/pages__master-text.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/pages__one.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/pages__size-5.75x4.75.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/pages__three-rect-p3.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/pages__two-rect-p2.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/pages__two.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/picture__base.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/picture__border.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/picture__crop-all.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/picture__crop-left.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/picture__crop-top.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/picture__flip-h.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/picture__moved.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/picture__oval-mask.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/picture__rotated-30.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/picture__stretched.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/picture__two.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/shadow__offset.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/shadow__on.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/shapes__curve.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/shapes__diamond.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/shapes__heart.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/shapes__hexagon.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/shapes__line-arrow.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/shapes__line.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/shapes__oval.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/shapes__polyline.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/shapes__rect.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/shapes__rightarrow.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/shapes__roundrect.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/shapes__star5.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/shapes__triangle.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/table__2x2.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/table__3x2.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/table__merged.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text2__base.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text2__p2-bold.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text2__p2-center.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text2__text-in-oval.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text2__two-boxes.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__align-center.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__align-justify.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__align-right.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__allcaps.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__base.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__bold.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__bullets.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__color-custom.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__color-red.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__font-Arial.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__font-ArialBlack.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__font-CourierNew.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__font-TimesNewRoman.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__frame-fill-line.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__indent-first-18.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__indent-left-18.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__indent-right-18.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__italic.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__line-break.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__ls-double.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__ls-exact-14.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__ls-exact-30.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__ls-multiple-1.5.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__ls-single.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__margins-10.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__rotated-270.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__run-bold-word.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__run-font-word.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__run-red-word.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__run-size-word.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__size-12.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__size-24.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__size-36.5.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__size-72.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__size-8.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__smallcaps.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__space-after-0.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__space-before-12.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__superscript.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__text-changed.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__text-longer.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__tracking-200.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__underline.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__vanchor-bottom.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/text__vanchor-middle.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/wordart__arch.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/wordart__base.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/wordart__circle.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/wordart__fill-red.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/wordart__font.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/wordart__line-white.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/wordart__shadow.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `pubsmith-fixtures/wordart__text.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `apache-pub/51318.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `apache-pub/60685.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `apache-pub/LinkAt0And10.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `apache-pub/LinkAt10.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `apache-pub/LinkAt10And20And30.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `apache-pub/LinkAt10And20And30And40.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `apache-pub/LinkAt10Longer.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `apache-pub/LinkAt20.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `apache-pub/Sample.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `apache-pub/Sample2.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `apache-pub/Sample2_2010.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `apache-pub/Sample3.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `apache-pub/Sample3_2010.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `apache-pub/Sample4.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `apache-pub/Sample4_2010.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `apache-pub/SampleBrochure.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `apache-pub/SampleNewsletter.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `apache-pub/Sample_2010.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |
| `apache-pub/Simple.pub` | CFB major 3; Publisher UNKNOWN | Accepted, 2002+ selector |

#### Shared 97/98/2000 selector: E8 AC 22 00

OBSERVATION: R below is the exact unchanged refusal reason:

```text
unsupported Publisher file: Publisher 97/98/2000 files are not supported yet; open and save the file in Publisher 2002 or later
```

| File | Documented version fields available | Importer verdict |
| --- | --- | --- |
| `apache-pub/Sample2000.pub` | CFB major 3; Publisher UNKNOWN | Refused, shared 97/98/2000 selector; R |
| `apache-pub/Sample98.pub` | CFB major 3; Publisher UNKNOWN | Refused, shared 97/98/2000 selector; R |

OBSERVATION: the two allowed data files match the existing manifest:

| File | Bytes | SHA-256 |
| --- | ---: | --- |
| `Sample2000.pub` | 61440 | `40701ca47b26d04771cdd58467764e9ab39d3da69b59fbc529afd16d263d2f86` |
| `Sample98.pub` | 61440 | `8912f295ff0cd1f55b873b3b98a182218f33c420877e025d2f0fd795f12dd0c0` |

#### Unclassified: damaged before the Contents selector

OBSERVATION: no validated container version or Publisher-version conclusion is assigned to these files. They are damaged cases, not version refusals.

| File | Documented version fields available | Importer verdict |
| --- | --- | --- |
| `apache-pub/clusterfuzz-testcase-minimized-POIHPBFFuzzer-4701121678278656.pub` | Not reached; Publisher UNKNOWN | `damaged Publisher file: compound file: Malformed FAT (sector 129 pointed to twice)` |
| `apache-pub/clusterfuzz-testcase-minimized-POIHPBFFuzzer-4918886059278336.pub` | Not reached; Publisher UNKNOWN | `damaged Publisher file: compound file: Malformed FAT (FAT has 244 entries, but file has only 64 sectors)` |
| `apache-pub/clusterfuzz-testcase-minimized-POIHPBFFuzzer-6325615354773504.pub` | Not reached; Publisher UNKNOWN | `damaged Publisher file: compound file: Invalid CFB byte order mark (expected 0xFFFE, found 0xFFFA)` |

## Dependency audit: what remains unknown for older Publisher

OBSERVATION: this inventory comes from the current importer and its local format notes, not from decoding the refused files. It groups the structural dependencies of `lib.rs`, `read.rs`, `contents.rs`, `quill.rs`, `tables.rs`, `escher.rs`, `paths.rs`, `presets.rs`, `wordart.rs` and `metafile.rs`. The protected readers were read as existing implementation evidence and were not edited.

OBSERVATION: the classification column asks about documented applicability to the older Publisher files. DOCUMENTED FOR OLDER VERSIONS below is deliberately limited to the common container that the unchanged path successfully opens in both old-family files. It does not document their publication payload. UNKNOWN means no permitted older-Publisher definition was verified, even if a generic standard or current-family local note describes a component. DOCUMENTED ONLY FOR 2002+ is not assigned merely because our local observations came from current-family samples; that would invent an official version boundary.

| Current dependency | Evidence used by the 2002+ importer | Older-version classification |
| --- | --- | --- |
| Compound-file envelope: header, sectors, FAT/DIFAT, mini-stream, directory/storage and stream access | MS-CFB and the existing container-open path; both old-family containers validate | DOCUMENTED FOR OLDER VERSIONS — common container only |
| Named stream selection | Contents, Quill/QuillSub/CONTENTS, Escher/EscherStm and optional EscherDelayStm paths | UNKNOWN |
| Publisher family selector and further publication header meaning | Existing four-byte check; current local header notes | UNKNOWN |
| Shared property-list framing | Key/kind pairs, fixed and variable sizes, nested lists, self-counting lengths and bounded numeric reads | UNKNOWN |
| Contents trailer and record framing | Trailer offset, length-prefixed records and nested record-directory fields | UNKNOWN |
| Record directory and sequence links | Entry types, record offsets, sequence-number identity and cross-record references | UNKNOWN |
| Document page ordering and dimensions | Document record, page list and EMU dimensions | UNKNOWN |
| Page/master ownership and scratch-area filtering | Page shape lists, applied master, master labels and placeholder/pasteboard handling | UNKNOWN |
| Shape-to-story association | Shape records, story id, linked-frame index and vertical alignment | UNKNOWN |
| Publication palette and colour references | Palette records, direct RGB and indexed/scheme colour resolution | UNKNOWN |
| Table dimension records | Grid sizes, cumulative endings, row heights, column widths and cell-record references | UNKNOWN |
| Table cell topology | Cell row/column bounds, spans/merges, occupancy and host-frame association | UNKNOWN |
| Quill section directory | Section tags, lengths, locations, chained directory blocks and guards | UNKNOWN |
| Quill text payload | UTF-16 units, retained controls, paragraph endings and omitted field characters | UNKNOWN |
| Story-id/length mapping | SYID and STRS lists, text slicing and run-to-story association | UNKNOWN |
| Font and text-colour tables | FONT and PL lists and their indices | UNKNOWN |
| Default text styles | STSH style-sheet selection and default character/paragraph records | UNKNOWN |
| Formatting-run pages | FDPC/FDPP pages, byte/unit run endpoints and format-record offsets | UNKNOWN |
| Character formatting properties | Family, size, weight, italic, underline, colour, capitalization, position and tracking keys | UNKNOWN |
| Paragraph formatting properties | Alignment, indents, before/after spacing, exact/multiple line spacing and bullet properties | UNKNOWN |
| Table text ranges and recovery | TCD cell-text boundaries, table-story association and the bounded single-table recovery path | UNKNOWN |
| OfficeArt containers and shape properties | Record headers, drawing/group/shape containers, FSP, primary/tertiary FOPT and use-mask merging | UNKNOWN |
| Publisher-specific drawing framing | Trailing words, client anchor/data length and key/value pairs, page-centre coordinate conventions | UNKNOWN |
| Drawing-to-Contents join | Client sequence reference and page/object ownership mapping | UNKNOWN |
| Group and rotation geometry | Group coordinate systems, child anchors, nested transforms, flips and rotated-frame convention | UNKNOWN |
| Freeform path arrays | Geometry bounds, vertex/segment arrays and the supported path-record subset | UNKNOWN |
| Preset identifiers and shape outlines | Preset shape numbers and existing geometry/formula implementation; an outline standard does not define old Publisher storage | UNKNOWN |
| WordArt text and warp selection | Geometry-text properties, font strings and arch path association; native editable text is an output choice | UNKNOWN |
| Drawing appearance properties | Fills/gradients, strokes, dashes, arrowheads, opacity and shadow conversion | UNKNOWN |
| Picture store and references | BStore/FBSE, one-based picture ids, delayed blips and payload association | UNKNOWN |
| Bitmap/metafile payload wrappers and cropping | DIB/bitmap wrapping, metafile headers and decompression, embedded vector records and crop fractions | UNKNOWN |

OBSERVATION: this grouped inventory has **31 entries: one documented common-container dependency, zero verified DOCUMENTED ONLY FOR 2002+ dependencies and 30 UNKNOWN older-Publisher dependencies**. Generic drawing/image standards do not, by themselves, establish that the old Publisher files use the same stream names, wrappers, references or text store. Current local notes remain useful evidence for the current-family importer, not an older-version implementation plan.

UNKNOWN: the older publication header beyond the already-used discriminator, every older directory/record/property layout, text-store organization, drawing association and exact product-version mapping. Those are stopping questions, not invitations to interpret neighbouring bytes or try a newer reader on old bodies. No field was inferred from truth JSON, reference PNGs, filenames or trial decoding of an older layout.

OBSERVATION: native document construction, font substitution, text composition, overset notes, rendering and exports are downstream implementation behaviour, not stored Publisher structures. Standard metadata streams currently ignored by the importer were not mined for creator/version hints. Adding a second header check alone would not provide any of the missing publication payload semantics.

## Starter samples to request from testers

INFERENCE: start with **three non-sensitive, independently labelled files per release**: Publisher 97, 98 and 2000, nine files total. This is a proposed request, not a post or a test of another application:

- A blank one-page publication, with the intended paper size stated.
- A small one-page text sample with a short paragraph and two text frames; state the intended text and basic formatting in plain words.
- A small one-page sample with one simple shape and one freely shareable bitmap; state the intended page contents.

INFERENCE: for each file, ask for the exact product edition/build known to have last saved it, how that provenance is known, and whether it was subsequently saved by another version. Do not infer the release from its filename or CFB Major Version. If the contributor cannot confirm the saving version, label it UNKNOWN. Keep the original file bytes and a SHA-256 value; there is no need to rename or resave it for this research.

INFERENCE: ask contributors not to send personal, employer/client, medical, financial or other sensitive material; use disposable samples with invented text and shareable pictures. Avoid confidential attachments, credentials, embedded private objects and private links. The two existing old-family samples are useful refusal controls, but their exact saving releases are not established here. These starter samples would not be enough to establish broad support for long stories, multi-page publications, tables or complex drawings.

## Recommendation and limits

INFERENCE: **no-go on an implementation task now; size large, with low confidence in the estimate**. Keep the present refusal. First obtain an authoritative Publisher-specific description whose older-version scope can be checked, or a separately authorized bounded research task; then obtain version-provenanced samples. Do not treat a request for more samples as permission to decode undocumented layouts. The main risk is apparently successful imports that silently lose or misassociate text and drawing objects.

UNKNOWN: whether a Publisher-specific specification is available at a different supplied URL, which exact older product releases the two samples represent, and whether some structures are shared across the old/current families. This investigation establishes neither old-version rendering nor fidelity. No commercial application or another reader was run or described. No importer, renderer, protected-reader, fixture-test or workflow code was changed. The existing README already has a Publisher 97/98/2000 entry under Still needs research, so no duplicate row was added.

## Baseline and verification

OBSERVATION: before and after this notes-only change, the controlled gate reports 142/142 imports, 152/152 legacy overlap coverage, 111 of 111 scored shape/text positions within 1 pt, 12 of 41 picture positions within 1 pt with 29 unmatched, 52/52 exact texts and 111/111 styles. Save/reopen/edit/PDF checks pass on all 142 imports.

OBSERVATION: both Apache batches report 9 conversions with notes, 10 with warnings, 2 refused and 3 damaged, with 31 pages and 280 objects. The reports match field-for-field after excluding output paths, including individual notes, warnings and refusal reasons. There are 40 Apache notes, including 10 overset notes; the controlled set has 55 notes, including one overset note. These are the current baseline, not new notes from this investigation.

OBSERVATION: formatting, strict workspace clippy and all seven CI steps pass; the CI test totals are 3,098 passed, zero failed and 25 ignored. The full controlled gate, save/reopen/edit/PDF check and real-batch integration check pass again. No product or test code changed.

UNKNOWN: Windows execution, real-PC fonts, older-version import/rendering and comparison with Publisher itself were not tested. No exact saving release was independently established for any corpus file. The absence of a verified Publisher specification prevents a documented older-layout compatibility conclusion.
