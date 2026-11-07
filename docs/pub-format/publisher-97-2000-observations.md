# Older Publisher files: observations from two samples

## Scope and baseline

OBSERVATION: this notes-only investigation uses main `8148a73`, the seven pinned Apache binaries listed below, the existing container reader, and the unchanged drawing/text parsers. It adds no input source or interpretation from another reader. The source restrictions in PUB-AGENTS.md and CLEANROOM.md remain in force. The earlier research note describes the narrower previous audit; this note adds the specifically requested stream listing, direct parser attempts and plain byte comparisons, without implementing an older record reader.

OBSERVATION: all 317 existing pinned binary, truth and reference-image files matched their recorded sizes and SHA-256 values. There are 142 controlled binaries and 24 Apache binaries. No data was downloaded. Truth and reference images were used only by the existing scoring checks, not for this stream/header/content investigation.

OBSERVATION: the untouched baseline is:

```text
SUMMARY files 142 imported 142 failed 0; truth elements 152 found 152 geometry-within-1pt 111 text exact 52/52 styling 111/111
POSITIONS shapes/text: 111 of 111 scored objects within 1 pt
POSITIONS pictures: 12 of 41 pictures within 1 pt; 29 unmatched (reference image elements; imported pictures only; informational)
```

OBSERVATION: the baseline real batch has 9 conversions with notes, 10 with warnings, 2 refused and 3 damaged, with 31 pages and 280 objects. It has 40 individual notes. Save/reopen/edit/PDF checks pass for all 142 controlled imports.

## Container comparison

OBSERVATION: all seven containers open successfully as CFB version 3. The names below are dataset labels, not independently confirmed saving-product releases. The five current-family comparison files pass the importer's `E8 AC 2C 00` selector. Both older-labelled files enter its `E8 AC 22 00` refusal branch.

OBSERVATION: each container has exactly ten streams and these same six storage entries, including the root: `/`, `/VBA`, `/Quill`, `/Quill/QuillSub`, `/Escher`, `/Objects`. The presence of a storage named VBA or Objects does not establish that it contains executable material or visible objects; no such interpretation or execution was attempted. Control characters in stream names are escaped as `\x01`, `\x03` and `\x05` below. Sizes are directory-reported stream lengths in bytes; every listed stream also opened and read to that length.

| File | File bytes | Contents | Quill text stream | Escher drawing | Escher delay | SummaryInformation |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `Sample98.pub` | 61440 | 5930 | 6144 | 0 | 0 | 44640 |
| `Sample2000.pub` | 61440 | 6042 | 6144 | 0 | 0 | 44640 |
| `Sample.pub` | 72192 | 11490 | 7168 | 3094 | 0 | 44640 |
| `Sample_2010.pub` | 72704 | 11500 | 7168 | 3094 | 0 | 45528 |
| `Sample2.pub` | 72704 | 11502 | 7680 | 3094 | 0 | 44640 |
| `Sample2_2010.pub` | 73216 | 11502 | 7680 | 3094 | 0 | 45528 |
| `Simple.pub` | 65536 | 9806 | 4096 | 1604 | 0 | 44640 |

OBSERVATION: full stream names and sizes follow. The first five columns above abbreviate only the paths, not a different stream. There are no paths present in just an older or just a comparison file in this seven-file set.

| Stream path | Sample98 | Sample2000 | Sample | Sample_2010 | Sample2 | Sample2_2010 | Simple |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `/\x01CompObj` | 94 | 94 | 94 | 94 | 94 | 94 | 94 |
| `/\x03Internal` | 12 | 12 | 12 | 12 | 12 | 12 | 12 |
| `/\x05DocumentSummaryInformation` | 152 | 152 | 152 | 152 | 152 | 152 | 152 |
| `/\x05SummaryInformation` | 44640 | 44640 | 44640 | 45528 | 44640 | 45528 | 44640 |
| `/Contents` | 5930 | 6042 | 11490 | 11500 | 11502 | 11502 | 9806 |
| `/Envelope` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `/Escher/EscherDelayStm` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `/Escher/EscherStm` | 0 | 0 | 3094 | 3094 | 3094 | 3094 | 1604 |
| `/Quill/QuillSub/\x01CompObj` | 86 | 86 | 86 | 86 | 86 | 86 | 86 |
| `/Quill/QuillSub/CONTENTS` | 6144 | 6144 | 7168 | 7168 | 7680 | 7680 | 4096 |

OBSERVATION: comparing the two older files byte-for-byte at stream level, eight streams are identical, including both empty Escher streams. Only `/Contents` and `/Quill/QuillSub/CONTENTS` differ. The equally sized Quill streams differ at 16 byte offsets: `0x05D8–0x05DA`, `0x05F6–0x05F8`, `0x06C4–0x06C6`, `0x0C66–0x0C68`, `0x101E–0x101F`, `0x1146–0x1147`. These are locations only; no field or record meaning is assigned to them. Matching metadata bytes were not interpreted as provenance.

UNKNOWN: identical paths and stream lengths do not establish identical application-specific record layouts, shared content ownership or compatibility with another saving release. The two files do not establish a version-wide stream rule.

## Contents prefix and refusal

OBSERVATION: these are the first 64 bytes of each root `/Contents` stream. Offsets below are relative to that stream, not the compound file. The four rows per file start at `0x00`, `0x10`, `0x20` and `0x30`. Beyond the existing selector, these are raw bytes, not newly named fields.

OBSERVATION: `Sample98.pub`.

```text
0000  E8 AC 22 00 2C 01 05 01 2A 17 00 00 68 02 00 01
0010  00 00 7E 03 00 00 AC 03 00 00 22 00 00 00 F8 00
0020  0C 00 59 00 06 00 68 68 01 00 08 00 00 00 BC 8C
0030  01 00 08 00 72 01 7E 00 88 00 8A 00 00 00 BE 8C
```

OBSERVATION: `Sample2000.pub`.

```text
0000  E8 AC 22 00 2C 01 05 01 9A 17 00 00 CD 02 00 01
0010  00 00 BA 03 00 00 E8 03 00 00 22 00 00 00 FC 00
0020  0C 00 5E 00 06 00 C6 8C 01 00 08 00 72 07 7E 00
0030  88 06 8A A6 20 00 00 08 C8 8C 01 00 08 00 72 07
```

OBSERVATION: `Sample.pub`.

```text
0000  E8 AC 2C 00 E8 03 05 01 E2 2C 00 00 15 00 00 01
0010  00 00 00 00 00 00 00 00 00 00 6A 25 00 00 D6 00
0020  00 00 0C 00 0C 00 00 00 00 00 00 00 AA 00 00 00
0030  01 18 E8 AC 02 68 00 01 00 00 03 20 15 00 E8 03
```

OBSERVATION: `Sample_2010.pub`.

```text
0000  E8 AC 2C 00 E8 03 06 01 EC 2C 00 00 18 00 00 01
0010  00 00 00 00 00 00 00 00 00 00 74 25 00 00 BE 00
0020  00 00 0C 00 0E 00 00 00 00 00 00 00 92 00 00 00
0030  01 18 E8 AC 02 68 00 01 00 00 03 20 18 00 E8 03
```

OBSERVATION: `Sample2.pub`.

```text
0000  E8 AC 2C 00 E8 03 05 01 EE 2C 00 00 15 00 00 01
0010  00 00 00 00 00 00 00 00 00 00 76 25 00 00 D8 00
0020  00 00 0C 00 0C 00 00 00 00 00 00 00 AC 00 00 00
0030  01 18 E8 AC 02 68 00 01 00 00 03 20 15 00 E8 03
```

OBSERVATION: `Sample2_2010.pub`.

```text
0000  E8 AC 2C 00 E8 03 06 01 EE 2C 00 00 18 00 00 01
0010  00 00 00 00 00 00 00 00 00 00 76 25 00 00 BE 00
0020  00 00 0C 00 0E 00 00 00 00 00 00 00 92 00 00 00
0030  01 18 E8 AC 02 68 00 01 00 00 03 20 18 00 E8 03
```

OBSERVATION: `Simple.pub`.

```text
0000  E8 AC 2C 00 E8 03 05 01 4E 26 00 00 15 00 00 01
0010  00 00 00 00 00 00 00 00 00 00 EA 1F 00 00 D8 00
0020  00 00 0C 00 0C 00 00 00 00 00 00 00 AC 00 00 00
0030  01 18 E8 AC 02 68 00 01 00 00 03 20 15 00 E8 03
```

OBSERVATION: grouped raw-byte comparisons within that prefix are:

| Byte range | Sample98 | Sample2000 | Sample / Sample2 / Simple | Sample_2010 / Sample2_2010 |
| --- | --- | --- | --- | --- |
| `0x00–0x03` | `E8 AC 22 00` | `E8 AC 22 00` | `E8 AC 2C 00` | `E8 AC 2C 00` |
| `0x04–0x07` | `2C 01 05 01` | `2C 01 05 01` | `E8 03 05 01` | `E8 03 06 01` |
| `0x0C–0x11` | `68 02 00 01 00 00` | `CD 02 00 01 00 00` | `15 00 00 01 00 00` | `18 00 00 01 00 00` |
| `0x12–0x19` | `7E 03 00 00 AC 03 00 00` | `BA 03 00 00 E8 03 00 00` | all zero | all zero |
| `0x1A–0x1D` | `22 00 00 00` | `22 00 00 00` | varies by file; see dump | varies by file; see dump |

OBSERVATION: `0x08–0x0B` also varies, as shown in every dump. The common `E8 AC` prefix, byte `0x03 = 00`, and `0x0E–0x11 = 00 01 00 00` are identical in all seven files. The older pair has identical first eight bytes. No saving-release number is established for the additional raw byte groups.

OBSERVATION: `import_pub` in `crates/pub/src/lib.rs` first checks the file-size limit and eight-byte CFB signature, opens the container, reads `/Contents`, then compares exactly its first four bytes. `E8 AC 2C 00` proceeds; `E8 AC 22 00` returns the refusal below; anything else returns an unrecognised-header error. It does not consult filenames, metadata, later header bytes, Quill, Escher or an exact release/build before this refusal. `is_pub` separately checks only the first two Contents bytes `E8 AC` after opening the container; it is not an importability or release test.

```text
unsupported Publisher file: Publisher 97/98/2000 files are not supported yet; open and save the file in Publisher 2002 or later
```

OBSERVATION: the existing current-family Contents reader uses bytes at `0x1A` as a trailer location. It was not called on the two older Contents bodies here, and that current-family meaning was not assigned to the older `22 00 00 00` bytes. The older files were not made to pass by replacing their prefix.

UNKNOWN: the meanings of the differing old header bytes, any exact product-release field, and the organization of the older Contents body. No older record layout was decoded, named or guessed. A numeric difference by itself is not a field definition.

## What the existing parsers return

OBSERVATION: the read-only diagnostic called the unchanged `escher::parse` on each actual `/Escher/EscherStm` with its actual delay stream, and `quill::parse` on each actual `/Quill/QuillSub/CONTENTS`. Because these functions are private, an outside-checkout diagnostic used a mechanically identical copy of the current library root with module paths pointing at the original unchanged source files, adding only result collection. No parser branch, property handling, fallback or byte input was altered. No diagnostic code or extracted stream was added to the repository. The ordinary public import result was checked separately.

| File | Container and named streams | Drawing parser | Quill parser | Public importer |
| --- | --- | --- | --- | --- |
| `Sample98.pub` | Open/read succeeds | Success; 0 top-level shapes, 0 pictures | Error: `unknown property kind 0x19` | Refused before body parsers |
| `Sample2000.pub` | Open/read succeeds | Success; 0 top-level shapes, 0 pictures | Error: `unknown property kind 0x19` | Refused before body parsers |
| `Sample.pub` | Open/read succeeds | Success; 9 top-level shapes, 0 pictures | Success; 6 stories | Accepted |
| `Sample_2010.pub` | Open/read succeeds | Success; 9 top-level shapes, 0 pictures | Success; 6 stories | Accepted |
| `Sample2.pub` | Open/read succeeds | Success; 9 top-level shapes, 0 pictures | Success; 6 stories | Accepted |
| `Sample2_2010.pub` | Open/read succeeds | Success; 9 top-level shapes, 0 pictures | Success; 6 stories | Accepted |
| `Simple.pub` | Open/read succeeds | Success; 4 top-level shapes, 0 pictures | Success; 2 stories | Accepted |

OBSERVATION: both direct drawing-parser calls on older files return an empty drawing and no warnings because both drawing inputs have zero bytes. This is not a recovered older drawing, and it does not establish that there are no visible shapes or pictures in the publication. No alternative drawing location was guessed or searched as a record layout.

OBSERVATION: both direct text-parser calls return exactly `damaged Publisher file: unknown property kind 0x19`. No Quill result, story list, fonts or partial text is returned to the caller. That error describes this parser attempt; it does not prove either original file is damaged. No property meaning, offset or recovery rule was invented for `0x19`. The public importer still refuses both files before calling either body parser.

OBSERVATION: in Sample.pub and Sample_2010.pub the drawing parser returns kinds `1, 20, 202, 202, 202, 201, 1, 202, 202`, six Quill stories and no drawing warnings. Sample2.pub and Sample2_2010.pub return the same shape-kind list and six stories with longer initial paragraphs. Simple.pub returns kinds `1, 20, 202, 202` and two stories containing digit/alphabet sequences. These are parser outputs, not an independent description of what a different application displays. No picture-store entries are returned for any of these seven files.

### Readable text bytes, without a story-layout claim

OBSERVATION: plain inspection of the two older Quill byte arrays finds the same contiguous run at stream offset `0x0200`. Interpreting its byte pairs as little-endian text units produces 489 characters, including carriage returns and a final newline. This inspection reads consecutive characters only; it does not identify a record, a story directory, frame association, formatting or displayed visibility. Other readable strings are not a substitute for a successful Quill parse.

OBSERVATION: the run is exactly the concatenation, in ascending returned story-id order, of the six story strings returned by the existing Quill parser for Sample.pub and Sample_2010.pub, plus one final newline. This byte/text equality is why those files are useful content comparisons; it does not prove equal layout. The six comparison stories begin:

| Returned comparison story id | Beginning of text also present in the older contiguous run |
| --- | --- |
| 1 | This is some text on the first page |
| 3 | This is in bold and italic |
| 4 | This is the second page |
| 6 | Table on page 2 |
| 7 | This text is on page two |
| 8 | More text, more hyperlinks |

OBSERVATION: the readable run mentions two pages, text boxes, a table and hyperlinks, and includes the phrases Times New Roman, Arial and sizes 10 and 20 in the text itself. These words are text bytes; they do not establish page count, table geometry, font settings or link targets in the older documents. No private path or incidental address from other strings is reproduced here.

UNKNOWN: the visible content, page count, actual formatting, positions, frame boundaries, table structure and links of either older publication. The present parsers recover no older drawing or complete text-story result, so there is no supported old-file render to inspect. Plain text mentioning a feature does not demonstrate that feature's stored structure or visible appearance.

## Questions and additional samples

UNKNOWN: no finite sample count proves a general format rule. The numbers below are minimum follow-up sets for repeatable observations, not sufficient proof of complete support. Use non-sensitive synthetic files and independently recorded last-saving edition/build. Existing names alone do not verify the exact release; no Publisher 97-specific sample is confirmed here.

| Label | Question left open | Minimum additional files for a useful next check |
| --- | --- | --- |
| UNKNOWN | Exact saving-release mapping of the shared selector | 3 independently labelled files, one each from 97, 98 and 2000, plus provenance for the two existing binaries. Additional files cannot retroactively prove those two files' provenance. |
| UNKNOWN | Whether the observed stream set, empty drawing streams and raw prefix differences recur across contents | 9 labelled older files: blank, text-only and simple shape/bitmap content for each of the three releases; match against 3 current-family controls of the same content kinds. |
| UNKNOWN | Whether the Quill failure recurs in minimal text or depends on formatting | 6 labelled older text-only files: a plain and a formatted version for each release, with the same stated text; compare unchanged parser results. This does not justify assigning meaning to property kind 0x19. |
| UNKNOWN | Whether the text equality corresponds to equal publication content or geometry | 3 labelled older/current pairs with identical stated content, 6 files total, plus intended page/shape descriptions from the contributor. Parser results alone still cannot confirm appearance of unsupported data. |
| UNKNOWN | Older Contents records, alternative drawing organization and text-to-frame links | The 9 older files above can expose more differences but cannot by themselves settle record meanings. A separately scoped evidence-led investigation is needed; no count alone settles undocumented layouts. |

UNVERIFIED HYPOTHESIS: a version-wide claim such as every older file having empty Escher streams, or sharing the same Quill failure, is unsupported by these two samples. This note makes neither claim. The similar readable text and eight matching streams also leave independence of the two samples UNKNOWN; they must not be counted as two independent demonstrations of a format rule.

## Verification and limits

OBSERVATION: the final full corpus gate matches every baseline line above. All 142 save/reopen/edit/PDF checks and the real-batch integration check pass again. The two real CSV reports match field-for-field after excluding output paths: 9 conversions with notes, 10 with warnings, 2 refused, 3 damaged, 31 pages, 280 objects and 40 individual notes. Neither per-file notes nor warning/refusal reasons changed.

OBSERVATION: formatting, strict workspace clippy and all seven CI steps pass, with 3,098 tests passed, zero failed and 25 ignored. Only this new note and the source ledger changed; no importer, renderer, test, protected-reader, workflow or fixture bytes changed.

UNKNOWN: no older publication was rendered or opened in another application. Visible appearance, font/layout fidelity, exact saving release and a general older-version encoding remain unverified. Windows and real-PC fonts were not tested. No web format search or additional specification download was performed for these observations.
