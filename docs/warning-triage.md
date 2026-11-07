<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Warning triage — 2026-10-19, queue continuation final

Fresh normal-import reports (local, not committed):
`.builder/queue-C6again-controlled/report.csv` and
`.builder/queue-C6again-real/report.csv`. Exact texts are deduplicated per file;
counts overlap. No encoding is inferred from warnings or scoring truth.

| Dataset | Before OK / warnings / refused / failed | Final | Exact warning texts before → final | File-warning pairs before → final |
|---|---|---|---:|---:|
| 142 allowed pubsmith fixtures | 79 / 63 / 0 / 0 | 79 / 63 / 0 / 0 | 9 → 7 | 120 → 66 |
| 24 pinned Apache .pub files | 0 / 19 / 2 / 3 | 0 / 19 / 2 / 3 | 17 → 14 | 72 → 64 |

Controlled outputs have 146 PDF pages; real successful outputs have 31 pages /
280 objects. The converter combines the two refusals with failures (five).

C2 removes only observed same-record matching 0x39/0x0C and 0x48/0x44 notices;
mismatches and absent partners still warn numerically. C3 removes 0x3C from a
warning only when alignment 6 is imported. Neither rule claims a field meaning.
Default paragraph 0x30 still warns, so affected-file counts do not fall.
See [unknown-fields.md](pub-format/unknown-fields.md) for the observation-based
rules and [real-file-triage.md](real-file-triage.md) for all 24 current rows.

## Ranked controlled-fixture inventory

Three examples are shown where available; otherwise every available example is
listed. Decisions retain the triage categories; C7–C9 may use only the public
specification, and were blocked because its text was unreachable.

| Rank | Exact warning text | Files | Fixture examples | Decision |
|---|---|---:|---|---|
| 1 | Publisher paragraph formatting is unsupported for fields: defaults [0x30]; runs [none]. Supported attributes were retained. | 54 | `pages__master-text.pub`, `table__2x2.pub`, `table__3x2.pub` | needs describer notes |
| 2 | 1 object(s) with soft effects (shadows, glows, feathers, bevels) or knockout groups exported as images | 3 | `shadow__offset.pub`, `shadow__on.pub`, `wordart__shadow.pub` | cannot fix |
| 3 | Publisher table cell fills, borders, padding and other cell formatting are not decoded; unpainted cells were used. | 3 | `table__2x2.pub`, `table__3x2.pub`, `table__merged.pub` | needs describer notes |
| 4 | Freeform shapes with unsupported segments were imported as rectangles. | 2 | `shapes__curve.pub`, `shapes__polyline.pub` | needs describer notes |
| 5 | WordArt shapes (arches, circles, waves) were imported as straight text. | 2 | `wordart__arch.pub`, `wordart__circle.pub` | needs describer notes |
| 6 | Scratch-area objects were not imported. | 1 | `pages__size-5.75x4.75.pub` | fixable from existing docs/pub-format notes |
| 7 | Shape type 74 is not supported yet; it was imported as a rectangle of the same size. | 1 | `shapes__heart.pub` | needs describer notes |

### 1. needs describer notes — 54 file(s)

**Warning:** Publisher paragraph formatting is unsupported for fields: defaults [0x30]; runs [none]. Supported attributes were retained.

**Bulletin/flyer effect and next evidence:** Supported alignment, spacing and indents remain. The effect of unidentified 0x30 on bulletin paragraphs is unknown; this warning does not prove lost numbering or tabs. Notes need to establish its value, inheritance and visible effect from controlled samples.

### 2. cannot fix — 3 file(s)

**Warning:** 1 object(s) with soft effects (shadows, glows, feathers, bevels) or knockout groups exported as images

**Bulletin/flyer effect and next evidence:** A shadowed heading or logo is rasterized in the PDF. Native shadows remain editable. This is the existing PDF exporter strategy, not a missing Publisher shadow; eliminating it needs renderer work rather than a decoding change.

### 3. needs describer notes — 3 file(s)

**Warning:** Publisher table cell fills, borders, padding and other cell formatting are not decoded; unpainted cells were used.

**Bulletin/flyer effect and next evidence:** Cell shading, rules and padding may differ in a bulletin timetable. Notes need controlled per-cell fill/border/padding variants and their associations; no colours or border meanings are inferred here.

### 4. needs describer notes — 2 file(s)

**Warning:** Freeform shapes with unsupported segments were imported as rectangles.

**Bulletin/flyer effect and next evidence:** A curve or polyline decoration becomes a rectangle. C7 is blocked on public MS-ODRAW access; the existing notes do not settle the missing segments.

### 5. needs describer notes — 2 file(s)

**Warning:** WordArt shapes (arches, circles, waves) were imported as straight text.

**Bulletin/flyer effect and next evidence:** An arched or circular heading becomes straight text. C9 is blocked on public MS-ODRAW access; no text-shape geometry was inferred.

### 6. fixable from existing docs/pub-format notes — 1 file(s)

**Warning:** Scratch-area objects were not imported.

**Bulletin/flyer effect and next evidence:** Off-page reusable material is omitted. Existing contents-stream notes identify the skipped objects, but retaining them still needs a native pasteboard placement design; suppressing the warning alone would conceal the omission.

### 7. needs describer notes — 1 file(s)

**Warning:** Shape type 74 is not supported yet; it was imported as a rectangle of the same size.

**Bulletin/flyer effect and next evidence:** The heart icon becomes a rectangle. C8 is blocked on public MS-ODRAW access; the geometry remains unconfirmed here.

## Sources and validation

Used only existing pub-format notes and our converter's fresh output. No other
reader/output or web format search was used. C7, C8 and C9 each attempted the
public MS-ODRAW landing page; the proxy rejected CONNECT with 403, so no spec
text or new geometry was obtained. Final corpus score remains **142/142 imports,
152/152 objects, 111/111 scored shape/text positions, 52/52 texts, 111/111 styles**. Truth exports
were used for scoring only. Fmt, clippy with warnings denied and ordinary tests
passed (69 passed, 10 ignored); the opt-in fixture warning test also passed.

## Own synthetic WMF fixture — 2026-10-20 follow-up

`wmf-picture.pub` (own regenerated test fixture, not one of the 142 controlled
inputs) reports exactly `Text inside WMF/EMF pictures was left out.` This text
was absent from the corpus-based inventory above. It is an existing picture-text
limitation, already observed in the C15 screenshot record; text drawn inside a
picture can disappear from a flyer even while its shapes remain visible.
The Windows alpha README now lists it as a known gap. No decoding fix or new
encoding claim is made, and the controlled/real warning counts above are not
changed by adding this separate own-sample observation.
