<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Apache content scan — 2026-10-21

Merged pub-import@bf94c3b1 into codex/queue first. Reused exactly the pinned
24 `.pub` inputs outside the checkout, checked every SHA-256 against our own
`.github/test-data/publisher-fixtures.csv` used by `.github/scripts/test-pub-cli.py`.
No download, upstream source, README, upstream text output or other reader used.
Pin: apache/poi trunk, bf7c9b6a1490046517cbd8e1504e702a89be4bc9; repository license
recorded as Apache-2.0 in SOURCES.md. Source hashes remain unchanged.

## Method and interpretation

Normal CLI batch conversion: 0 OK / 19 with warnings / 2 refused / 3 damaged
failures. For all 19 successes, recursively extracted story/cell text from our
own saved document.json and rendered **every page**, 31 pages total, with our CLI.
Read all extracted text and visually inspected all 31 complete page PNGs. Also
checked imported model strings for address-like email/URI values; none appear
in the generic hyperlink tests beyond their visible labels. Scan renders, text,
models, PDFs and CSV stay private under .builder/public-scan; no source .pub is
committed. These are our importer outputs, not another reader's text output.

(a) means observed Microsoft-template indicators, not proof of the content's
creator. (b) includes named people/contact fields even if apparently placeholders.
(c) means content origin is unclear and fails the conservative screen. (d) means
only generic/test text and simple shapes were observed, with no (a)/(b)/(c)
evidence. Classification concerns the imported output and proposed screenshots;
unsupported or dropped source content cannot be certified by this scan. In
particular, the label `email link` is not a literal email address; original
unimported hyperlink destinations remain unverified. Numeric alphabet/counting
sequences without contact context are test text, not inferred phone numbers.
No classification was inferred from filenames. Refused/damaged inputs are not
classified clean: they yield no inspectable document or page.

## Per-file findings

Warnings refer to the exact inventory below. Looked at: **T** = all own extracted
story/cell text plus imported string values; **P1–N** = every complete page render;
**F** = own preflight result. “None found” always means in that examined output.

| File | Pages / objects | Class | (a) template evidence | (b) personal-information evidence | (c) unclear content evidence | (d) clean evidence / decision | Import warnings or failure | Looked at | Preflight errors |
|---|---|---|---|---|---|---|---|---|---:|
| 51318.pub | 1 / 17 | c | none found; geometric header alone does not prove a template | none found | Sunset photograph: origin/creator or stock-art status unknown | No: photograph prevents generic-text/simple-shape classification | W1 | T, P1–1, F | 1 |
| 60685.pub | 1 / 43 | c | none found | No person name/address/phone/email/photo found; employment history is potentially personal | Résumé narrative: “Caring Registered Nurse with six years experience”; “UF Health Shands Hospital”, “Lake Butler Hospital”; real-vs-fictional origin unknown | No: not demonstrably generic/test content | W2, W3 | T, P1–1, F | 1 |
| LinkAt0And10.pub | 1 / 1 | d | none found | none found | none found | Yes: `LINK123456LINK2`; a single text frame, no pictures | W4 | T, P1–1, F | 1 |
| LinkAt10.pub | 1 / 1 | d | none found | none found | none found | Yes: `1234567890LINK`; a single text frame, no pictures | W4 | T, P1–1, F | 1 |
| LinkAt10And20And30.pub | 1 / 1 | d | none found | none found | none found | Yes: `1234567890LINK123456LINK212345LINK3`; a single text frame, no pictures | W4 | T, P1–1, F | 1 |
| LinkAt10And20And30And40.pub | 1 / 1 | d | none found | none found | none found | Yes: `1234567890LINK123456LINK212345LINK312345LINK4`; a single text frame, no pictures | W4 | T, P1–1, F | 1 |
| LinkAt10Longer.pub | 1 / 1 | d | none found | none found | none found | Yes: `1234567890LINK`; a single text frame, no pictures | W4 | T, P1–1, F | 1 |
| LinkAt20.pub | 1 / 1 | d | none found | none found | none found | Yes: `12345678901234567890LINK`; a single text frame, no pictures | W4 | T, P1–1, F | 1 |
| Sample.pub | 2 / 7 | d | none found | none found | none found | Yes: “This is some text on the first page”, “This is the second page”, “This is a link to Apache POI”, “email link”; formatting-test boxes, plain rectangle/table, no pictures | W5, W4, W6, W7, W8, W9 | T, P1–2, F | 3 |
| Sample2.pub | 2 / 7 | d | none found | none found | none found | Yes: “This is some text on the first page”, “This is the second page”, “This is a link to Apache POI”, “email link”; formatting-test boxes, plain rectangle/table, no pictures; “We’ve added some more text in here, to push all the offsets about a bit.” | W5, W4, W6, W7, W8, W9 | T, P1–2, F | 3 |
| Sample2000.pub | — | unclassified | Not available; no document returned | Not available; no document returned | Not available; no document returned | Not established | import: can't open the Publisher file: unsupported Publisher file: Publisher 97/98/2000 files are not supported yet; open and save the file in Publisher 2002 or later | refusal/failure only | — |
| Sample2_2010.pub | 2 / 7 | d | none found | none found | none found | Yes: “This is some text on the first page”, “This is the second page”, “This is a link to Apache POI”, “email link”; formatting-test boxes, plain rectangle/table, no pictures; “We’ve added some more text in here, to push all the offsets about a bit.” | W5, W4, W6, W7, W8, W9 | T, P1–2, F | 3 |
| Sample3.pub | 2 / 7 | d | none found | none found | none found | Yes: “This is some text on the first page”, “This is the second page12345678”, “This is a link to Apache POI”, “email link”; formatting-test boxes, plain rectangle/table, no pictures | W5, W4, W6, W7, W8, W9 | T, P1–2, F | 3 |
| Sample3_2010.pub | 2 / 7 | d | none found | none found | none found | Yes: “This is some text on the first page”, “This is the second page12345678”, “This is a link to Apache POI”, “email link”; formatting-test boxes, plain rectangle/table, no pictures | W5, W4, W6, W7, W8, W9 | T, P1–2, F | 3 |
| Sample4.pub | 2 / 7 | d | none found | none found | none found | Yes: “This is some text on the first page”, “This is the second page12”, “This is a link to Apache POI”, “email link”; formatting-test boxes, plain rectangle/table, no pictures | W5, W4, W6, W7, W8, W9 | T, P1–2, F | 3 |
| Sample4_2010.pub | 2 / 7 | d | none found | none found | none found | Yes: “This is some text on the first page”, “This is the second page12”, “This is a link to Apache POI”, “email link”; formatting-test boxes, plain rectangle/table, no pictures | W5, W4, W6, W7, W8, W9 | T, P1–2, F | 3 |
| Sample98.pub | — | unclassified | Not available; no document returned | Not available; no document returned | Not available; no document returned | Not established | import: can't open the Publisher file: unsupported Publisher file: Publisher 97/98/2000 files are not supported yet; open and save the file in Publisher 2002 or later | refusal/failure only | — |
| SampleBrochure.pub | 2 / 57 | a + b (also c) | “Contoso Art Gallery”, “Contoso Gallery of Fine Art”; art/coffee/brush clip-art-like illustrations | “Lisa Baker”, “Vincent Van Gogh”, “Leonardo Da Vinci”; “Phone: (425) 555-0187”, “Email: l.baker@contoso.com” | Reproductions of portrait/art works; their inclusion is not cleared by this scan | No | W10, W11, W12 | T, P1–2, F | 8 |
| SampleNewsletter.pub | 4 / 99 | a + b (also c) | “Microsoft Publisher”, “Contoso, Ltd.”, “Inside Story Headline”, “Caption describing picture or graphic”; repeated clip-art-like illustrations | “Jay Adams”, “Peter Houston”, “Tim Kim”, “By Paul Koch”; “Phone: 555-555-0187”, “E-mail: someone@example.com” | Statue/flag photograph and illustrations: origins not established | No; no pages from this file selected | W13, W14, W12 | T, P1–4, F | 12 |
| Sample_2010.pub | 2 / 7 | d | none found | none found | none found | Yes: “This is some text on the first page”, “This is the second page”, “This is a link to Apache POI”, “email link”; formatting-test boxes, plain rectangle/table, no pictures | W5, W4, W6, W7, W8, W9 | T, P1–2, F | 3 |
| Simple.pub | 1 / 2 | d | none found | none found | none found | Yes: “0123456789”, “0123456789abcdef” repeated in two text boxes; no pictures | W4 | T, P1–1, F | 1 |
| clusterfuzz-testcase-minimized-POIHPBFFuzzer-4701121678278656.pub | — | unclassified | Not available; no document returned | Not available; no document returned | Not available; no document returned | Not established | import: can't open the Publisher file: damaged Publisher file: compound file: Malformed FAT (sector 129 pointed to twice) | refusal/failure only | — |
| clusterfuzz-testcase-minimized-POIHPBFFuzzer-4918886059278336.pub | — | unclassified | Not available; no document returned | Not available; no document returned | Not available; no document returned | Not established | import: can't open the Publisher file: damaged Publisher file: compound file: Malformed FAT (FAT has 244 entries, but file has only 64 sectors) | refusal/failure only | — |
| clusterfuzz-testcase-minimized-POIHPBFFuzzer-6325615354773504.pub | — | unclassified | Not available; no document returned | Not available; no document returned | Not available; no document returned | Not established | import: can't open the Publisher file: damaged Publisher file: compound file: Invalid CFB byte order mark (expected 0xFFFE, found 0xFFFA) | refusal/failure only | — |

## Exact import-warning inventory

- **W1**: `Publisher paragraph formatting is unsupported for fields: defaults [0x30]; runs [0x19, 0x30]. Supported attributes were retained.`
- **W2**: `Publisher character formatting is unsupported for fields: defaults [0x58]; runs [none]. Supported attributes were retained.`
- **W3**: `Publisher paragraph formatting is unsupported for fields: defaults [0x30]; runs [0x19]. Supported attributes were retained.`
- **W4**: `Publisher paragraph formatting is unsupported for fields: defaults [0x30]; runs [none]. Supported attributes were retained.`
- **W5**: `Publisher character formatting is unsupported for fields: defaults [none]; runs [0x00, 0x37, 0x38]. Supported attributes were retained.`
- **W6**: `Publisher table text across multiple stories/TCD sections is not decoded.`
- **W7**: `Publisher table used stored endings for row positions; largest difference from size sums: 9564 EMU; cause unknown.`
- **W8**: `Table cell-text association is unconfirmed or missing; the original text was retained after the table.`
- **W9**: `Publisher table cell fills, borders, padding and other cell formatting are not decoded; unpainted cells were used.`
- **W10**: `Publisher character formatting is unsupported for fields: defaults [none]; runs [0x2D, 0x37]. Supported attributes were retained.`
- **W11**: `Publisher paragraph formatting is unsupported for fields: defaults [none]; runs [0x19, 0x30]. Supported attributes were retained.`
- **W12**: `Fields and special characters (page numbers, dates) were left out of the text.`
- **W13**: `Publisher character formatting is unsupported for fields: defaults [none]; runs [0x00, 0x1B, 0x22, 0x37]. Supported attributes were retained.`
- **W14**: `Publisher paragraph formatting is unsupported for fields: defaults [none]; runs [0x06, 0x19, 0x1D, 0x30, 0x32, 0x57]. Supported attributes were retained. Not fully imported: list marker (unconfirmed descriptor).`

## Pick

**15 clean (d) files; four successful imports excluded; five unclassifiable.**
Choose **one: Sample2.pub**, two pages / seven objects. It ties the richest clean
family and has extra test paragraphs relative to Sample.pub. Sample2_2010.pub
has substantially the same content; the 3/4 variants mainly change test digits.
Avoid redundant captures. The smaller hyperlink and Simple files have fewer
warnings but much less distinct content. All clean candidates contain no picture;
no unclear-origin photograph or clip art is admitted to improve visual richness.
The two Microsoft-template/contact documents are excluded as whole files; their
apparently generic pages are not used to evade the selection rule.

Sample2's six warnings remain material: unsupported formatting, ambiguous table
text association, stored-ending differences and unpainted table cells. Content
cleanliness is not visual fidelity. Its existing table story is overset by 81
characters; the task does not repair that import. Captures use its first page.

## What “12 errors” means

Ran the existing `preflight.run` command on SampleNewsletter.pub. It returns
**12 errors = six overset items + six missing font families**, separately from
three import warnings and seven preflight warnings. First messages, verbatim:

- `Overset text: 8 characters` (item 18)
- `Overset text: 3 characters` (item 20)
- `Overset text: 67 characters` (item 25)
- `Overset text: 7 characters` (item 30)
- `Overset text: 1 characters` (item 148)
- `Overset text: 4 characters` (item 160)
- `Missing font: Arial Black`
- `Missing font: Elephant`
- `Missing font: Franklin Gothic Book`
- `Missing font: Rockwell Condensed`
- `Missing font: Sylfaen`
- `Missing font: Times New Roman`

There were seven unavailable faces in the earlier dialog but six families;
Franklin Gothic Book Bold/Regular count as one family-level preflight error.
Preflight warnings include `picture-6.jpeg: effective 150 ppi (< 150)` and RGB
swatch notices; they do not contribute to the 12-error status count.

Every successful file's preflight count is in the table. The selected Sample2
has **three errors**, not twelve: `Overset text: 81 characters` (item 15, page
index 1), `Missing font: Arial`, `Missing font: Times New Roman`. These already
exist before editing. Status bar “errors” does not mean import refused or a
crash. Counts depend on this machine's available fonts; Windows is unverified.

No images are embedded in this document or any README. Nothing is published;
this screen identifies candidates for the owner to review, not a publication.
