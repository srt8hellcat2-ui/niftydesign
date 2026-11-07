# Headline clipping investigation

## Result

OBSERVATION: all four cut-off first-page headlines in `SampleNewsletter.pub` retain their full text. The existing composer places one line in each frame and reports overset. The title requests Franklin Gothic Book and uses Source Sans 3; the other three request Rockwell Condensed and use DejaVu Serif Condensed in this environment. Increasing only frame height in a temporary in-memory copy, at the same width and with the same text and fonts, makes their missing text appear. This demonstrates a fit problem in our current layout, not lost words in the import.

INFERENCE: answer **(a)** applies to the measured substitute-font layout, and **(b)** applies in the literal sense that the stored story is longer than the visible output. Neither establishes that all of that story was meant to be visible in Publisher. In particular one headline story also contains a byline and `hjhjhlh`. UNKNOWN: whether Publisher originally showed all this text, clipped it too, used different line-breaking metrics, or used fit-to-text/best-fit behaviour (**c**). The necessary standard flags could not be verified, and there is no Publisher reference render for these Apache samples. Do not attribute the whole difference exclusively to font substitution.

## Scope and evidence

OBSERVATION: all 24 pinned Apache binaries were attempted; 19 import, two older-format files are refused and three damaged files fail. All imported text frames were composed with the existing composer. Thirteen frames in two files belong to overset stories: ten newsletter frames in seven stories and three brochure frames in three stories. Six of the newsletter frames are linked body/reminder text rather than headlines; a story's overset status applies to the whole chain, not independently to every earlier frame. The seven short overset frames below cover the four newsletter headlines plus the brochure exhibition title and two captions. No other imported Apache file reported an overset story. This is a composer-led search, not proof that no other visible clipping exists.

OBSERVATION: source evidence comes from the existing `escher::parse`, `quill::parse` and `Builder` output on pinned bytes. A temporary diagnostic wrapper outside the checkout captured each placed item id and its source Contents sequence; no product file or parser was edited. Its full imported documents match the unmodified public importer after excluding only creation/modified timestamps, for all 161 successful imports (142 controlled and 19 Apache). The extra-height experiment changes only an in-memory frame path to a 10,000 pt height. It does not save a changed document or propose that height as a fix. No truth JSON or golden image was used for this storage/text audit. Local raw reports, the diagnostic wrapper and validation logs remain in `/tmp/niftydata/layout-investigation/`.

| File | SHA-256 |
| --- | --- |
| `SampleNewsletter.pub` | `6a825ba26ba35d6e885acdc62e859591ed37cb0ff7480b554b9cb362b644dfcf` |
| `SampleBrochure.pub` | `ffed034ac87e679f0bd08ff9cf74ad11c0e0e510a42b1bc1a7502415f6c29c87` |

OBSERVATION: both are the data files at Apache commit `bf7c9b6a1490046517cbd8e1504e702a89be4bc9`. All 317 existing controlled/Apache/scoring/reference inputs matched their existing size and hash manifests. Nothing was re-downloaded.

## Frame measurements

OBSERVATION: dimensions are untransformed frame width and height in points, not glyph bounds. All seven source shapes have type 202, no rotation, no horizontal or vertical flip and no group parent. Each of these stories has exactly one frame, so these particular cuts are not text flowing to another frame. Lines means actual placed lines, not hypothetical wrapped lines that did not fit. Overset offsets below are UTF-8 byte offsets in the imported story. Strings in the detailed entries use JSON escaping.

| File; page; item/Contents sequence | Width × height (pt) | Placed lines | Overset at byte | Lines at same width with extra height | Requested family/style/size | Actual faces, including recovered text |
| --- | --- | ---: | ---: | ---: | --- | --- |
| `SampleBrochure.pub`; 1; 30/319 | 165.109291 × 80.778898 | 4 | 70 | 5 | Georgia / Regular / 13 pt; Arial Narrow / Bold / 13 pt | DejaVu Sans / Condensed Bold; Source Sans 3 / Regular |
| `SampleBrochure.pub`; 2; 72/364 | 109.413701 × 40.793622 | 3 | 53 | 4 | Garamond / Regular / 10 pt; Garamond / Bold / 10 pt | Source Sans 3 / Bold; Source Sans 3 / Regular |
| `SampleBrochure.pub`; 2; 82/327 | 113.385827 × 39.685039 | 2 | 30 | 3 | Garamond / Bold / 10 pt; Garamond / Regular / 10 pt | Source Sans 3 / Bold; Source Sans 3 / Regular |
| `SampleNewsletter.pub`; 1; 18/296 | 336.078110 × 51.954724 | 1 | 27 | 5 | Rockwell Condensed / Regular / 24 pt; Rockwell Condensed / Regular / 14 pt | DejaVu Serif / Condensed |
| `SampleNewsletter.pub`; 1; 20/297 | 221.102362 × 38.535039 | 1 | 17 | 2 | Rockwell Condensed / Regular / 24 pt | DejaVu Serif / Condensed |
| `SampleNewsletter.pub`; 1; 30/307 | 451.179764 × 77.872913 | 1 | 23 | 2 | Franklin Gothic Book / Regular / 43.5 pt | Source Sans 3 / Regular |
| `SampleNewsletter.pub`; 1; 160/424 | 119.055118 × 38.535039 | 1 | 9 | 2 | Rockwell Condensed / Regular / 24 pt | DejaVu Serif / Condensed |

OBSERVATION: the newsletter American headline's five extra-height lines include a final empty paragraph; four contain text. The font list includes the hidden brochure date line: Arial Narrow Bold resolves to DejaVu Sans Condensed Bold when it is actually composed. This avoids claiming that the face of the visible first line was used for every run.

OBSERVATION: the existing importer reads text insets from properties `0x0082, 0x0081, 0x0084, 0x0083` into top/left/bottom/right document options, dividing by 12,700. Six frames have 36,195 in each field, producing 2.85 pt per side. Brochure item 82 has 36,576, producing 2.88 pt per side. Thus the two 221.102362/119.055118 pt newsletter frames offer only 215.402362/113.355118 pt text width. Their 38.535039 pt heights offer 32.835039 pt after those insets. The document options show one column and auto-size off in all seven; the main title uses bottom vertical justification, the others top. These are observations of importer behaviour, not a claim that an absent source property means Publisher auto-size was off.

### Full text and placed lines

OBSERVATION: the following source strings are the existing Quill parser's complete UTF-16 story decoded to text, including carriage returns. Imported text is also shown in full. The importer changes internal carriage returns to newlines and removes one final carriage return; none of the missing headline words is removed. Line strings show source ranges and therefore omit a displayed discretionary hyphen.

#### SampleBrochure.pub — item 30, sequence 319

OBSERVATION:

```text
Source story: "\t\tThe \r\t      Great    \r            Portraits \r           Exhibition \rMay 23 – June 1, 2003\r"
Imported text: "\t\tThe \n\t      Great    \n            Portraits \n           Exhibition \nMay 23 – June 1, 2003"
Placed lines: ["\t\tThe ", "\t      Great    ", "            Portraits ", "           Exhibition "]
Lines with extra height: ["\t\tThe ", "\t      Great    ", "            Portraits ", "           Exhibition ", "May 23 – June 1, 2003"]
```

OBSERVATION: full source Escher anchor `(left, top, right, bottom)` in the parser's EMU coordinate space is `[2574000.0, -2484000.0, 4670888.0, -1458108.0]`. Imported frame `(x, y, width, height)` is `[623.622047244, 102.047244094, 165.109291339, 80.778897638]` pt.

OBSERVATION: all 36 scalar properties returned by the existing parser for this shape follow, sorted by id. These are the parser's merged values, not an inventory of each duplicate primary/tertiary record. Its complex-property map is empty. No unknown field is assigned a meaning here.

```text
0x0080 = 0x00000008 (8)
0x0081 = 0x00008D63 (36195)
0x0082 = 0x00008D63 (36195)
0x0083 = 0x00008D63 (36195)
0x0084 = 0x00008D63 (36195)
0x008D = 0x00011AC6 (72390)
0x017F = 0x007F007B (8323195)
0x0181 = 0x08000004 (134217732)
0x01BF = 0x007F001C (8323100)
0x01C0 = 0x08000000 (134217728)
0x01CB = 0x00000000 (0)
0x01FF = 0x01EF0066 (32440422)
0x0201 = 0x08000004 (134217732)
0x023F = 0x00030000 (196608)
0x033F = 0x00080008 (524296)
0x037F = 0x007F0010 (8323088)
0x0384 = 0x00008EE0 (36576)
0x0385 = 0x00008EE0 (36576)
0x0386 = 0x00008EE0 (36576)
0x0387 = 0x00008EE0 (36576)
0x03BF = 0xFFFB8201 (4294672897)
0x0540 = 0x08000000 (134217728)
0x0542 = 0x08000007 (134217735)
0x057F = 0x00080000 (524288)
0x0580 = 0x08000000 (134217728)
0x0582 = 0x08000007 (134217735)
0x05BF = 0x00080000 (524288)
0x05C0 = 0x08000000 (134217728)
0x05C2 = 0x08000007 (134217735)
0x05FF = 0x00080000 (524288)
0x0600 = 0x08000000 (134217728)
0x0602 = 0x08000007 (134217735)
0x063F = 0x00080000 (524288)
0x0640 = 0x08000000 (134217728)
0x0642 = 0x08000007 (134217735)
0x06FF = 0x00020002 (131074)
```

#### SampleBrochure.pub — item 72, sequence 364

OBSERVATION:

```text
Source story: "Hindsight \rLisa Baker uses mixed media to create her portraits.\r"
Imported text: "Hindsight \nLisa Baker uses mixed media to create her portraits."
Placed lines: ["Hindsight ", "Lisa Baker uses mixed ", "media to create her "]
Lines with extra height: ["Hindsight ", "Lisa Baker uses mixed ", "media to create her ", "portraits."]
```

OBSERVATION: full source Escher anchor `(left, top, right, bottom)` in the parser's EMU coordinate space is `[-4742496.0, -404618.0, -3352942.0, 113461.0]`. Imported frame `(x, y, width, height)` is `[47.52, 265.778110236, 109.413700787, 40.793622047]` pt.

OBSERVATION: all 36 scalar properties returned by the existing parser for this shape follow, sorted by id. These are the parser's merged values, not an inventory of each duplicate primary/tertiary record. Its complex-property map is empty. No unknown field is assigned a meaning here.

```text
0x0080 = 0x00000014 (20)
0x0081 = 0x00008D63 (36195)
0x0082 = 0x00008D63 (36195)
0x0083 = 0x00008D63 (36195)
0x0084 = 0x00008D63 (36195)
0x008D = 0x00011AC6 (72390)
0x017F = 0x007F007B (8323195)
0x0181 = 0x08000002 (134217730)
0x01BF = 0x007F001C (8323100)
0x01C0 = 0x08000000 (134217728)
0x01CB = 0x00000000 (0)
0x01FF = 0x01EF0066 (32440422)
0x0201 = 0x08000004 (134217732)
0x023F = 0x00030000 (196608)
0x033F = 0x00080008 (524296)
0x037F = 0x007F0010 (8323088)
0x0384 = 0x00008EE0 (36576)
0x0385 = 0x00008EE0 (36576)
0x0386 = 0x00008EE0 (36576)
0x0387 = 0x00008EE0 (36576)
0x03BF = 0xFFFB8201 (4294672897)
0x0540 = 0x08000000 (134217728)
0x0542 = 0x08000007 (134217735)
0x057F = 0x00080000 (524288)
0x0580 = 0x08000000 (134217728)
0x0582 = 0x08000007 (134217735)
0x05BF = 0x00080000 (524288)
0x05C0 = 0x08000000 (134217728)
0x05C2 = 0x08000007 (134217735)
0x05FF = 0x00080000 (524288)
0x0600 = 0x08000000 (134217728)
0x0602 = 0x08000007 (134217735)
0x063F = 0x00080000 (524288)
0x0640 = 0x08000000 (134217728)
0x0642 = 0x08000007 (134217735)
0x06FF = 0x00020002 (131074)
```

#### SampleBrochure.pub — item 82, sequence 327

OBSERVATION:

```text
Source story: "The eyes of an angel\rMay 2003\rA pencil sketch.\r"
Imported text: "The eyes of an angel\nMay 2003\nA pencil sketch."
Placed lines: ["The eyes of an angel", "May 2003"]
Lines with extra height: ["The eyes of an angel", "May 2003", "A pencil sketch."]
```

OBSERVATION: full source Escher anchor `(left, top, right, bottom)` in the parser's EMU coordinate space is `[-3690000.0, 1116000.0, -2250000.0, 1620000.0]`. Imported frame `(x, y, width, height)` is `[130.393700787, 385.511811024, 113.385826772, 39.68503937]` pt.

OBSERVATION: all 32 scalar properties returned by the existing parser for this shape follow, sorted by id. These are the parser's merged values, not an inventory of each duplicate primary/tertiary record. Its complex-property map is empty. No unknown field is assigned a meaning here.

```text
0x0080 = 0x00000019 (25)
0x0081 = 0x00008EE0 (36576)
0x0082 = 0x00008EE0 (36576)
0x0083 = 0x00008EE0 (36576)
0x0084 = 0x00008EE0 (36576)
0x008D = 0x00011940 (72000)
0x017F = 0x00400040 (4194368)
0x0181 = 0x08000002 (134217730)
0x01BF = 0x00100010 (1048592)
0x01C0 = 0x08000000 (134217728)
0x01C2 = 0x08000007 (134217735)
0x01FF = 0x00480040 (4718656)
0x0201 = 0x08000004 (134217732)
0x0384 = 0x00008EE0 (36576)
0x0385 = 0x00008EE0 (36576)
0x0386 = 0x00008EE0 (36576)
0x0387 = 0x00008EE0 (36576)
0x0540 = 0x08000000 (134217728)
0x0542 = 0x08000007 (134217735)
0x057F = 0x00080000 (524288)
0x0580 = 0x08000000 (134217728)
0x0582 = 0x08000007 (134217735)
0x05BF = 0x00080000 (524288)
0x05C0 = 0x08000000 (134217728)
0x05C2 = 0x08000007 (134217735)
0x05FF = 0x00080000 (524288)
0x0600 = 0x08000000 (134217728)
0x0602 = 0x08000007 (134217735)
0x063F = 0x00080000 (524288)
0x0640 = 0x08000000 (134217728)
0x0642 = 0x08000007 (134217735)
0x06FF = 0x00020002 (131074)
```

#### SampleNewsletter.pub — item 18, sequence 296

OBSERVATION:

```text
Source story: "What it means to be an American\r\t\t\t\t\t\tBy Paul Koch\rhjhjhlh\r\r"
Imported text: "What it means to be an American\n\t\t\t\t\t\tBy Paul Koch\nhjhjhlh\n"
Placed lines: ["What it means to be an Amer"]
Lines with extra height: ["What it means to be an Amer", "ican", "\t\t\t\t\t\tBy Paul Koch", "hjhjhlh", ""]
```

OBSERVATION: full source Escher anchor `(left, top, right, bottom)` in the parser's EMU coordinate space is `[-1388192.0, -2441825.0, 2880000.0, -1782000.0]`. Imported frame `(x, y, width, height)` is `[188.331338583, 228.67519685, 336.078110236, 51.954724409]` pt.

OBSERVATION: all 37 scalar properties returned by the existing parser for this shape follow, sorted by id. These are the parser's merged values, not an inventory of each duplicate primary/tertiary record. Its complex-property map is empty. No unknown field is assigned a meaning here.

```text
0x0080 = 0x00000006 (6)
0x0081 = 0x00008D63 (36195)
0x0082 = 0x00008D63 (36195)
0x0083 = 0x00008D63 (36195)
0x0084 = 0x00008D63 (36195)
0x008D = 0x00011AC6 (72390)
0x017F = 0x007F007B (8323195)
0x0181 = 0x08000007 (134217735)
0x0183 = 0x08000007 (134217735)
0x01BF = 0x007F001C (8323100)
0x01C0 = 0x08000000 (134217728)
0x01CB = 0x00000000 (0)
0x01FF = 0x01EF0066 (32440422)
0x0201 = 0x08000004 (134217732)
0x023F = 0x00030000 (196608)
0x033F = 0x00080008 (524296)
0x037F = 0x007F0010 (8323088)
0x0384 = 0x00008EE0 (36576)
0x0385 = 0x00008EE0 (36576)
0x0386 = 0x00008EE0 (36576)
0x0387 = 0x00008EE0 (36576)
0x03BF = 0xFFFB8201 (4294672897)
0x0540 = 0x08000000 (134217728)
0x0542 = 0x08000007 (134217735)
0x057F = 0x00080000 (524288)
0x0580 = 0x08000000 (134217728)
0x0582 = 0x08000007 (134217735)
0x05BF = 0x00080000 (524288)
0x05C0 = 0x08000000 (134217728)
0x05C2 = 0x08000007 (134217735)
0x05FF = 0x00080000 (524288)
0x0600 = 0x08000000 (134217728)
0x0602 = 0x08000007 (134217735)
0x063F = 0x00080000 (524288)
0x0640 = 0x08000000 (134217728)
0x0642 = 0x08000007 (134217735)
0x06FF = 0x00020002 (131074)
```

#### SampleNewsletter.pub — item 20, sequence 297

OBSERVATION:

```text
Source story: "Our Excursion to the Zoo\r"
Imported text: "Our Excursion to the Zoo"
Placed lines: ["Our Excursion to "]
Lines with extra height: ["Our Excursion to ", "the Zoo"]
```

OBSERVATION: full source Escher anchor `(left, top, right, bottom)` in the parser's EMU coordinate space is `[-1332000.0, 1134000.0, 1476000.0, 1623395.0]`. Imported frame `(x, y, width, height)` is `[192.755905512, 510.236220472, 221.102362205, 38.53503937]` pt.

OBSERVATION: all 37 scalar properties returned by the existing parser for this shape follow, sorted by id. These are the parser's merged values, not an inventory of each duplicate primary/tertiary record. Its complex-property map is empty. No unknown field is assigned a meaning here.

```text
0x0080 = 0x00000007 (7)
0x0081 = 0x00008D63 (36195)
0x0082 = 0x00008D63 (36195)
0x0083 = 0x00008D63 (36195)
0x0084 = 0x00008D63 (36195)
0x008D = 0x00011AC6 (72390)
0x017F = 0x007F007B (8323195)
0x0181 = 0x08000007 (134217735)
0x0183 = 0x08000007 (134217735)
0x01BF = 0x007F001C (8323100)
0x01C0 = 0x08000000 (134217728)
0x01CB = 0x00000000 (0)
0x01FF = 0x01EF0066 (32440422)
0x0201 = 0x08000004 (134217732)
0x023F = 0x00030000 (196608)
0x033F = 0x00080008 (524296)
0x037F = 0x007F0010 (8323088)
0x0384 = 0x00008EE0 (36576)
0x0385 = 0x00008EE0 (36576)
0x0386 = 0x00008EE0 (36576)
0x0387 = 0x00008EE0 (36576)
0x03BF = 0xFFFB8201 (4294672897)
0x0540 = 0x08000000 (134217728)
0x0542 = 0x08000007 (134217735)
0x057F = 0x00080000 (524288)
0x0580 = 0x08000000 (134217728)
0x0582 = 0x08000007 (134217735)
0x05BF = 0x00080000 (524288)
0x05C0 = 0x08000000 (134217728)
0x05C2 = 0x08000007 (134217735)
0x05FF = 0x00080000 (524288)
0x0600 = 0x08000000 (134217728)
0x0602 = 0x08000007 (134217735)
0x063F = 0x00080000 (524288)
0x0640 = 0x08000000 (134217728)
0x0642 = 0x08000007 (134217735)
0x06FF = 0x00020002 (131074)
```

#### SampleNewsletter.pub — item 30, sequence 307

OBSERVATION:

```text
Source story: "Living and Learning in Class X\r"
Imported text: "Living and Learning in Class X"
Placed lines: ["Living and Learning in "]
Lines with extra height: ["Living and Learning in ", "Class X"]
```

OBSERVATION: full source Escher anchor `(left, top, right, bottom)` in the parser's EMU coordinate space is `[-2864992.0, -4164904.0, 2864991.0, -3175918.0]`. Imported frame `(x, y, width, height)` is `[72.047874016, 92.999685039, 451.17976378, 77.872913386]` pt.

OBSERVATION: all 37 scalar properties returned by the existing parser for this shape follow, sorted by id. These are the parser's merged values, not an inventory of each duplicate primary/tertiary record. Its complex-property map is empty. No unknown field is assigned a meaning here.

```text
0x0080 = 0x0000000E (14)
0x0081 = 0x00008D63 (36195)
0x0082 = 0x00008D63 (36195)
0x0083 = 0x00008D63 (36195)
0x0084 = 0x00008D63 (36195)
0x008D = 0x00011AC6 (72390)
0x017F = 0x007F007B (8323195)
0x0181 = 0x08000007 (134217735)
0x0183 = 0x08000007 (134217735)
0x01BF = 0x007F001C (8323100)
0x01C0 = 0x08000000 (134217728)
0x01CB = 0x00000000 (0)
0x01FF = 0x01EF0066 (32440422)
0x0201 = 0x08000004 (134217732)
0x023F = 0x00030000 (196608)
0x033F = 0x00080008 (524296)
0x037F = 0x007F0010 (8323088)
0x0384 = 0x00008EE0 (36576)
0x0385 = 0x00008EE0 (36576)
0x0386 = 0x00008EE0 (36576)
0x0387 = 0x00008EE0 (36576)
0x03BF = 0xFFFB8201 (4294672897)
0x0540 = 0x08000000 (134217728)
0x0542 = 0x08000007 (134217735)
0x057F = 0x00080000 (524288)
0x0580 = 0x08000000 (134217728)
0x0582 = 0x08000007 (134217735)
0x05BF = 0x00080000 (524288)
0x05C0 = 0x08000000 (134217728)
0x05C2 = 0x08000007 (134217735)
0x05FF = 0x00080000 (524288)
0x0600 = 0x08000000 (134217728)
0x0602 = 0x08000007 (134217735)
0x063F = 0x00080000 (524288)
0x0640 = 0x08000000 (134217728)
0x0642 = 0x08000007 (134217735)
0x06FF = 0x00020002 (131074)
```

#### SampleNewsletter.pub — item 160, sequence 424

OBSERVATION:

```text
Source story: "Strictly Jazz\r"
Imported text: "Strictly Jazz"
Placed lines: ["Strictly "]
Lines with extra height: ["Strictly ", "Jazz"]
```

OBSERVATION: full source Escher anchor `(left, top, right, bottom)` in the parser's EMU coordinate space is `[-540000.0, 2862000.0, 972000.0, 3351395.0]`. Imported frame `(x, y, width, height)` is `[255.118110236, 646.299212598, 119.05511811, 38.53503937]` pt.

OBSERVATION: all 37 scalar properties returned by the existing parser for this shape follow, sorted by id. These are the parser's merged values, not an inventory of each duplicate primary/tertiary record. Its complex-property map is empty. No unknown field is assigned a meaning here.

```text
0x0080 = 0x0000004B (75)
0x0081 = 0x00008D63 (36195)
0x0082 = 0x00008D63 (36195)
0x0083 = 0x00008D63 (36195)
0x0084 = 0x00008D63 (36195)
0x008D = 0x00011AC6 (72390)
0x017F = 0x007F007B (8323195)
0x0181 = 0x08000007 (134217735)
0x0183 = 0x08000007 (134217735)
0x01BF = 0x007F001C (8323100)
0x01C0 = 0x08000000 (134217728)
0x01CB = 0x00000000 (0)
0x01FF = 0x01EF0066 (32440422)
0x0201 = 0x08000004 (134217732)
0x023F = 0x00030000 (196608)
0x033F = 0x00080008 (524296)
0x037F = 0x007F0010 (8323088)
0x0384 = 0x00008EE0 (36576)
0x0385 = 0x00008EE0 (36576)
0x0386 = 0x00008EE0 (36576)
0x0387 = 0x00008EE0 (36576)
0x03BF = 0xFFFB8201 (4294672897)
0x0540 = 0x08000000 (134217728)
0x0542 = 0x08000007 (134217735)
0x057F = 0x00080000 (524288)
0x0580 = 0x08000000 (134217728)
0x0582 = 0x08000007 (134217735)
0x05BF = 0x00080000 (524288)
0x05C0 = 0x08000000 (134217728)
0x05C2 = 0x08000007 (134217735)
0x05FF = 0x00080000 (524288)
0x0600 = 0x08000000 (134217728)
0x0602 = 0x08000007 (134217735)
0x063F = 0x00080000 (524288)
0x0640 = 0x08000000 (134217728)
0x0642 = 0x08000007 (134217735)
0x06FF = 0x00020002 (131074)
```

## What the properties establish, and what they do not

OBSERVATION: the existing permitted geometry notes cite MS-ODRAW §2.4.24 for shape 202 as a plain text box. The parser returns no `0x00BF` entry for any of these seven shapes, and no complex property. The raw ids/values above are preserved without interpreting unspecified flags. Absence of an explicit fit setting does not by itself establish Publisher's effective defaults or settings stored elsewhere.

OBSERVATION: the public MS-ODRAW landing page at `https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/` was requested through the configured network path. It returned no standard text:

```text
curl: (56) CONNECT tunnel failed, response 403
MS-ODRAW HTTP 000
```

UNKNOWN: this investigation therefore cannot verify or decode additional text-box fit-to-text, best-fit or auto-margin bits from the official specification. No bit meaning was guessed from correlations, export JSON, reference images, another reader or a web format description. Some scalar values are boolean sets which the existing parser merges; their high/low bits must not be treated as independent new flags without their defined use masks and defaults. The `0x0384`–`0x0387` entries are not substituted for the four inset fields that the current importer actually uses.

OBSERVATION: `Builder::place` applies the parsed rectangle, text insets and Contents-derived vertical alignment. `Builder::text_frame` retains the text and requested character formats. These paths do not set document auto-size or implement a source-dependent best-fit font-size adjustment. `compose_story`/`compose` stops assigning glyphs when its available frames are exhausted and sets `overset_at`. The extra-height checks recover all seven stories with no overset. This explains the missing output in our renderer without establishing what Publisher would have done.

INFERENCE: the immediate mechanism is width-dependent wrapping followed by insufficient remaining height, including the date line in the exhibition title and the last lines of both captions. The substitute fonts and our line-breaking/baseline metrics are plausible contributors. Confirming that they alone cause a difference from Publisher would require the original fonts and a permitted Publisher rendering of these same samples. In the American headline, tabs, a smaller byline run and extra stored paragraphs add a separate source of potential layout difference; making only the visible heading fit would not prove that the whole story was faithfully laid out.

OBSERVATION: a useful internal control is newsletter item 74: it holds the same full `Living and Learning in Class X` text, requests Franklin Gothic Book at 8.5 pt, has a 187.85 × 19.267480 pt frame, and composes one complete line without overset. This shows that the family name alone does not determine clipping; size and frame dimensions matter.

## First-page render

OBSERVATION: the existing command was run successfully:

```sh
designcraft-cli run --in /tmp/niftydata/apache-pub/SampleNewsletter.pub --page 0 --scale 1.5 --export /tmp/niftydata/layout-investigation/SampleNewsletter-page1.png
```

OBSERVATION: the inspected output is 893 × 1263 pixels. It visibly reads `Living and Learning in`, `What it means to be an Amer-`, `Our Excursion to`, and `Strictly`. `Class X`, the end of `American`, `the Zoo` and `Jazz` are not visible. The grey rule, purple sidebar, bright magenta reminder box, Liberty picture, giraffe and instruments are present. Body text also ends abruptly in places. This is an observation of our output only; no judgement about Publisher's corresponding appearance is possible from this image alone.

## Validation and remaining limits

OBSERVATION: formatting and all seven CI steps pass: 3,088 tests passed, zero failed, 24 ignored. The before/after controlled gate is unchanged: 142/142 imports, 152/152 objects, 111 position passes under its image-exclusion rule, 52/52 texts and 111/111 styles. All 142 native save/reopen/edit/PDF cases pass. The real batch remains nine with notes, ten with warnings, two refused, three damaged, 31 pages and 280 objects. No importer, composer, renderer, test or scoring implementation was changed; the only committed changes are findings and source notes.

UNKNOWN: no manual Windows test, real-PC font inventory, original Microsoft-font composition, Publisher-render comparison for these Apache samples, or interactive auto-fit test was performed. The two refused and three damaged files could not be inspected as imported documents. The scan does not detect all possible visual cropping, especially text or glyphs clipped without the composer reporting overset. Choosing a font-size adjustment or making frames grow would require a separate product change and evidence about Publisher's intended behaviour.
