# The Office 97-2003 "Standard" colour palette, read from a public screenshot (2026-11-28)

Source and method (observation only; nothing copied): the Standard tab of the Colors dialog in Microsoft Office and Publisher 2002 shows 127 hexagonal swatches in 13 rows (7, 8, 9, 10, 11, 12, 13, 12, 11, 10, 9, 8, 7) and 17 greys below them. A public course page (flylib.com, "Quick Course in Microsoft Publisher 2002", "Creating Custom Color Schemes") shows that dialog as a small JPEG. The centre of each swatch was sampled (a 3 x 3 pixel mean) and rounded to the nearest of the six levels 00, 33, 66, 99, CC, FF per channel. The mean error before rounding was 7.5 of 765, so the rounding is reliable for the swatches that are on the six-level grid. The image is not stored in the repository.

Hexagon, top to bottom, left to right (hex RRGGBB). Cells whose raw sample was far from a grid colour are marked `?`; a swatch such as a pure tint may not be on the six-level grid at all:

```
  row 0 (7): 003366 336699 3366CC 003399 000099 0000CC 000066
  row 1 (8): 006666 006699 0099CC 0066CC 0033CC 0000FF 3333FF 333399
  row 2 (9): 009999 009999 33CCCC 00CCFF 0099FF 0066FF 3366FF 3333CC 666699
  row 3 (10): 339966 00CC99 00FFCC 00FFFF 33CCFF 3399FF 6699FF 6666FF 6600FF 6600CC
  row 4 (11): 339933 00CC66 00FF99 66FFCC 66FFFF 66CCFF 99CCFF 9999FF 9966FF 9933FF 9900FF
  row 5 (12): 006600 00CC00 00FF00 66FF99 99FFCC CCFFFF CCFFFF CCCCFF CC99FF CC66FF CC00FF 9900CC
  row 6 (13): 003300 009900 33CC33 66FF66 99FF99 CCFFCC FFFFFF FFCCFF FF99FF FF66FF FF00FF CC00CC 660066
  row 7 (12): 336600 009900 66FF33 99FF66 CCFF99 FFFFCC FFCCCC FF99CC FF66CC FF33CC CC0099 990099
  row 8 (11): 333300 669900 99FF33 CCFF66 FFFF99 FFCC99 FF9999 FF6699 FF3399 CC3399 990099
  row 9 (10): 666633 99CC00 CCFF33 FFFF66 FFCC66 FF9966 FF6699 FF0066 CC0099 993366
  row 10 (9): 999900 CCCC00 FFFF00 FFCC00 FF9933 FF6600 FF6666 CC0066 660033
  row 11 (8): 996633 CC9900 FF9900 CC6600 FF3300 FF0000 CC0000 990033
  row 12 (7): 663300 996600 CC3300 993300 990000 990000 990033

```

(The two rows next to the middle one were read with a coarse row spacing, and the sampled row of the bottom three rows is the least reliable; a few cells repeat their neighbour. Treat the table as a first reading, not a verified copy, until each swatch is checked against a second screenshot.)

## Evidence that the numbers in the scheme record index this palette

OBSERVATION: the scheme record (layout index 71, see legacy-minimal-reader.md) has eight entries at +0xa0, +0xa4, ..., +0xbc, each a literal colour (`RR GG BB 20`) or a small number. If the number counts the 17 greys first and then the hexagon swatches row by row from the top (index 17 is the first swatch of the first row), the entries of every scheme are coherent:

| Scheme | Number | Resolves to | Reads as |
|---|---|---|---|
| Bluebird | 31, 37, 30 | `333399`, `0066FF`, `3333FF` | blues (a blue scheme) |
| Dark Blue | 50 | `6600CC` | purple-blue, next to its literal `9999CC` and `CCCCE6` |
| Waterfall | 18 | `336699` | blue-teal, next to its literal `669999` |
| Black & Gray | 4 | a grey (one of the 17) | a grey, next to its literal greys |
| Island | 14 | a grey | a grey, next to its literal blue and lime |

Entry 0 is 0 (black) in nearly every file; entry 7 is always 1. This ordering does not hold for the other orders tried (top to bottom without the greys, or bottom to top).

INFERENCE: the numbers in the scheme record are positions in this palette (greys 0 to 16, hexagon 17 to 143). UNKNOWN: the order of the 17 greys; and what numbers in frame and shape fill references mean. A frame fill reference of 26 (in `IHEC Postcards`, whose top strips are plain yellow on the stored preview) resolves to `0099CC` under the same numbering, so frame fill numbers do not follow it. Nothing in the opener uses this table yet.

## Direct fill numbers: the 40-colour grid (2026-11-28)

OBSERVATION: the fill reference of a frame (u32 at +0x22) is a bare number below 40 in four donated files: 26 (`IHEC Postcards`, the headline strips) and 5 and 27 (`Hotels`, `Super Bowl`, the direction cards on page 2). The Office 97-2003 colour drop-down offers a grid of 40 colours in five rows of eight (names and order in the public Microsoft Office colour chart): black, brown, olive green, dark green, dark teal, dark blue, indigo, grey 80 %; dark red, orange, dark yellow, green, teal, blue, blue-grey, grey 50 %; red, light orange, lime, sea green, aqua, light blue, violet, grey 40 %; pink, gold, yellow, bright green, turquoise, sky blue, plum, grey 25 %; rose, tan, light yellow, light green, light turquoise, pale blue, lavender, white.

INFERENCE: a direct number is a position in that grid, counted in reading order from 0. Number 26 is Yellow, and the `IHEC Postcards` strips are yellow on the stored preview (with the red WordArt headline on top, whose stored drawing uses exact red). Numbers 27 and 5 give Bright Green and Dark Blue, which suit the green direction cards with a navy header of the `Super Bowl` page; they are not checked against a preview (page 2 of those files is not on the stored preview). The scheme numbers (see above) use the 144-swatch hexagon instead: the two pickers of the program, the drop-down and "More Colors", give two numberings. The opener reads numbers 1 to 39 as this grid; the one confirmed value is 26.

OBSERVATION (text on a dark fill): black text on a navy or dark-green fill would be unreadable, and the "automatic" text colour of the program is light on a dark fill (the black frames of `Hotels` carry white text). The opener gives text whose colour is not decoded (automatic, or a small number) the colour white on a frame whose fill is dark (luminance below 100 of 255), whatever the source of the fill.
