<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Drawing-object position, size, rotation, flip and groups (describer, data-only)

Written 2026-10-19 by the data-only describer. Every statement is labelled **OBSERVATION** (read from the files, with the file names that show it), **INFERENCE** (follows from observations; reasoning given), **UNKNOWN**, or **UNVERIFIED HYPOTHESIS**. Inputs: this repository's `CLEANROOM.md` and `docs/pub-format/` notes, and the `.pub` data files listed in `SOURCES.md` under "Describer (data-only)". No reader code, no `truth/*.json`, no golden images. The MS-ODRAW page could not be fetched (host unreachable), so nothing below relies on specification text; record names are used only as already referenced in `README.md`.

All numbers are little-endian. "EMU" = 12 700 per point. "Stored" numbers below are signed 32-bit values as read.

## 1. How the drawing stream is laid out (observation)

**OBSERVATION** (all 142 pubsmith files, e.g. `geometry__base.pub`): `Escher/EscherStm` is one drawing-group container (record type 0xF000, length-prefixed with the usual 8-byte Escher header: version/instance 16 bits, type 16 bits, length 32 bits) followed by several blocks, each being **a 4-byte little-endian number followed by a drawing container (type 0xF002)**. In `geometry__base.pub` the numbers are 2, 1, 0 (drawing-header instance 3, 2, 1). Only the drawing with number 0 contains shapes that carry a `Contents` link (below); the others hold scratch/placeholder shapes without links. In `Sample2000.pub` and `Sample98.pub` (Contents signature `E8 AC 22 00`) no Escher data could be read at all (empty stream), so none of this applies to them.

Inside a drawing: a group container (0xF003) whose first shape container (0xF004) is a patriarch shape (flags 0x5, empty rectangle), then one shape container per object. Per object shape container, in this order: shape record 0xF00A (spid and flags), property table 0xF00B, tertiary property table 0xF122, then **0xF010** and **0xF011**.

**OBSERVATION — 0xF010 is where the position is stored.** Its data is not fixed binary fields: it is a *length-prefixed property list* in the encoding of `property-lists.md` (the 32-bit length counts itself, then key/kind/data). In every file, keys 0x01, 0x02, 0x03, 0x04 occur, all kind 0x20 (4-byte), read as **signed** 32-bit. No other key occurs in any of the 166 files (counts over all shapes: key 1 ×500, 2 ×339, 3 ×340, 4 ×340).

**OBSERVATION — a key whose value is zero is omitted.** `geometry__left-306.pub` has no key 0x01 (its left is 0); `geometry__top-396.pub` has no key 0x02. Inference: an absent key means 0.

**OBSERVATION — 0xF011 holds the link.** Property list with key 0x01, kind 0x68 = the object's `Contents` sequence number (293 = 0x125 for the first user object in the pubsmith geometry files). This is the already documented 0x6801 pairing. In all 166 files, no sequence number appears on two shapes.

## 2. Anchor fields: left, top, right, bottom (controlled variants)

**OBSERVATION / INFERENCE.** Key 1 = left, 2 = top, 3 = right, 4 = bottom. Reasoning: `left-*` files change keys 1 and 3 together; `top-*` change keys 2 and 4 together; `width-*` change only key 3; `height-*` change only key 4 (table below).

**Unit — INFERENCE (strong).** Between `left-0` and `left-100`, `left-306`, `left-500` key 1 changes by 1 270 000, 3 886 200, 6 350 000 EMU, i.e. exactly 12 700 × the number in the file name (100, 306, 500). `width-150` and `width-612` give right − left = 1 905 000 and 7 772 400 = 12 700 × 150 and × 612; `height-10`, `height-300` give 127 000 and 3 810 000. So **the unit is EMU and the file-name numbers are points** (assumption stated: the names mean points; the exact 12 700 factor and the fact that `width-612` equals the document width 7 772 400 EMU from the `Contents` document record support it).

**Origin — OBSERVATION then INFERENCE.** In the Letter-sized (612 × 792 pt) files, stored left = (intended left − 306) pt and stored top = (intended top − 396) pt: `left-306` and `top-396` store 0. 306 and 396 are half the page width and height. **INFERENCE: the origin is the centre of the page, x to the right, y downward** (positive y goes down, because larger `top-N` files store larger numbers). Independent support from Publisher-authored files: `SampleBrochure.pub` (document 841.89 × 595.28 pt, landscape) has ordinary objects whose left/right extremes are −373.42 and +373.42 pt and top/bottom extremes −261.64 and +261.64 pt (e.g. sequence 305: left −373.42, top −261.64, bottom +261.64; sequence 357: right +373.42, top −261.64), symmetric about 0 with margins of 47.52 pt and 36.00 pt on both sides; `SampleNewsletter.pub` (595.28 × 841.89, portrait) group sequence 309: −261.64…+261.64 horizontally. Symmetry about 0 on both a landscape and portrait page is what a page-centre origin predicts.

**Page-relative, not spread-relative — INFERENCE.** In `SampleBrochure.pub`, objects on the page record 266 and on page record 334 (taken from the page records' object lists in `Contents`) use the same range (both pages contain left edges at −373.42 and right edges at +373.42); page 2 is not offset by a page width. In the pubsmith files, `pages__two-rect-p2.pub` (rectangle on page 2) and `pages__three-rect-p3.pub` (rectangle on page 3) store identical numbers (−76 200, −1 219 200, 1 193 800, 50 800). Master-page objects (`pages__master-rect.pub`, `pages__master-text.pub`, sequence 294 on page record 263) use the same kind of numbers. Whether the origin is the centre of *each page* in a facing-pages spread is not separately shown: **UNKNOWN** whether any file is a facing-pages spread.

**Rounding — OBSERVATION.** Two files deviate from the exact 12 700 × name value by 4 EMU: `left-1` stores −3 873 496 (expected −3 873 500) and `left-73` stores −2 959 104 (expected −2 959 100). All other variant values are exact. Cause **UNKNOWN**.

### Stored numbers, Letter page 612 × 792 pt (centre 306, 396 pt)

Stored order: left, top, right, bottom, in EMU. "—" = key absent (value 0). Intended value comes from the file name (points).

| File | Intended (name) | Left | Top | Right | Bottom |
|---|---|---:|---:|---:|---:|
| `geometry__base` | (none; implied left 72, top 72, 144 × 72 pt) | −2971800 | −4114800 | −1143000 | −3200400 |
| `geometry__left-0` | left 0 | −3886200 | −4114800 | −2057400 | −3200400 |
| `geometry__left-1` | left 1 | −3873496 | −4114800 | −2044696 | −3200400 |
| `geometry__left-73` | left 73 | −2959104 | −4114800 | −1130304 | −3200400 |
| `geometry__left-100` | left 100 | −2616200 | −4114800 | −787400 | −3200400 |
| `geometry__left-306` | left 306 | — | −4114800 | 1828800 | −3200400 |
| `geometry__left-500` | left 500 | 2463800 | −4114800 | 4292600 | −3200400 |
| `geometry__top-0` | top 0 | −2971800 | −5029200 | −1143000 | −4114800 |
| `geometry__top-74` | top 74 | −2971800 | −4089400 | −1143000 | −3175000 |
| `geometry__top-396` | top 396 | −2971800 | — | −1143000 | 914400 |
| `geometry__top-700` | top 700 | −2971800 | 3860800 | −1143000 | 4775200 |
| `geometry__width-1` | width 1 | −2971800 | −4114800 | −2959100 | −3200400 |
| `geometry__width-150` | width 150 | −2971800 | −4114800 | −1066800 | −3200400 |
| `geometry__width-612` | width 612 | −2971800 | −4114800 | 4800600 | −3200400 |
| `geometry__height-10` | height 10 | −2971800 | −4114800 | −1143000 | −3987800 |
| `geometry__height-300` | height 300 | −2971800 | −4114800 | −1143000 | −304800 |
| `geometry__rot-1`, `-30`, `-180`, `-359` | rotation 1, 30, 180, 359 | −2971800 | −4114800 | −1143000 | −3200400 |
| `geometry__rot-90`, `-270` | rotation 90, 270 | −2514600 | −4572000 | −1600200 | −2743200 |
| `geometry__flip-h`, `geometry__flip-v` | flip | −2971800 | −4114800 | −1143000 | −3200400 |
| `geometry__two-rects` (seq 293 / 294) | two rectangles | −2971800 / −2616200 | −4114800 / −3759200 | −1143000 / −787400 | −3200400 / −2844800 |
| `geometry__two-rects-swapped` | same two, order swapped | seq 294 listed first; same numbers per sequence | | | |
| `pages__landscape` (page 792 × 612) | — | −2971800 | −4114800 | −1143000 | −3200400 |
| `pages__size-5.75x4.75` (page 414 × 342) | — | −342900 | −1943100 | 1485900 | −1028700 |

Base rectangle: width 1 828 800 EMU = 144 pt, height 914 400 = 72 pt, left 72 pt, top 72 pt if the origin is (306, 396) — **INFERENCE**, base has no number in its name.

### Rotation (OBSERVATION / INFERENCE)

- **Where:** shape property table (0xF00B), property id 0x0004, a 32-bit value. **Value = degrees × 65 536**: rot-1 → 0x10000, rot-30 → 0x1E0000, rot-90 → 0x5A0000, rot-180 → 0xB40000, rot-270 → 0x10E0000, rot-359 → 0x1670000. Always non-negative, same number as the file name; the property is absent at 0. Same encoding in `picture__rotated-30.pub`, `text__rotated-270.pub`, `group__rotated.pub` (30°, on the group shape).
- **Sign / direction — UNKNOWN.** The stored number equals the name's number, so there is no sign flip between name and file, but the files contain no rendering. Whether positive is clockwise on screen is not settled. (UNVERIFIED HYPOTHESIS: clockwise, as that is the usual UI convention; nothing here shows it.)
- **Anchor under rotation — OBSERVATION.** For 1°, 30°, 180°, 359° the four anchor numbers are identical to base. For **90° and 270°** the anchor box has **width and height swapped** (72 × 144 pt instead of 144 × 72) around the **same centre** (−162, −288 pt). `text__rotated-270.pub` shows the same thing against `text__base.pub` (288 × 144 pt box at (72, 72) becomes 144 × 288 pt with unchanged centre (−90, −252)).
- **INFERENCE:** the anchor is not simply the unrotated frame at all angles (it would not change at 90°), and not the visible bounding box at all angles (it would grow at 30°). **UNKNOWN** where the switch happens (only 1, 30, 90, 180, 270, 359 sampled) and **UNKNOWN** whether a reader should apply the stored rotation to the swapped box or treat the swapped box as the already-rotated result; the two readings give visible shapes of 144 × 72 pt versus 72 × 144 pt for `rot-90`. No file settles it without a rendered reference.

### Flip (OBSERVATION)

The anchor is unchanged. The shape record's flag word (0xF00A, second 32-bit value) differs from base (0xA00): `flip-h` = 0xA40 (+0x40), `flip-v` = 0xA80 (+0x80). INFERENCE: bit 0x40 = flip horizontal, bit 0x80 = flip vertical (names agree; both flips together unsampled; `picture__flip-h.pub` also shows 0x40 in the flags). Flip does not move or resize the box.

## 3. Groups (group__two, group__rotated, plus 19 more groups in other files)

**OBSERVATION.** A group is a nested group container (0xF003) whose first shape container is the group's own shape (flags 0x201; `Contents` type 0x30 or 0x31 object), then the children. Three different coordinate records appear:

1. **The group's own 0xF010 anchor** (keys 1–4 as above) — in page coordinates. `group__two.pub`: −2971800, −4114800, 203200, −3200400 (width 3 175 000 = 250 pt, height 914 400 = 72 pt; same left/top as base, i.e. left 72, top 72).
2. **The group's 0xF009 record** (four signed 32-bit values: left, top, right, bottom): the group's *child coordinate space*. `group__two.pub`: 107213400, 106070400, 110388400, 106984800. Children have **no 0xF010 anchor**; instead:
3. **Each child's 0xF00F record** (four signed 32-bit values, same order): child rectangle in that child space: `group__two.pub` child 293: 107213400, 106070400, 109042200, 106984800 (144 × 72 pt); child 294: 109474000, 106070400, 110388400, 106984800 (72 × 72 pt).

**OBSERVATION:** in all 21 groups in the 166 files, the group's 0xF009 rectangle equals the exact union (min of lefts and tops, max of rights and bottoms) of its children's 0xF00F rectangles. In 20 of 21 groups the width and height of the 0xF010 anchor equal those of the 0xF009 rectangle (ratio 1.0); the exception is a `SampleNewsletter.pub` group (anchor 1316992 × 2387825, child space 1299583 × 2935888; ratios 1.0134 and 0.8133).

**INFERENCE:** the child space has the same unit (EMU), a far-away arbitrary origin, and the group's anchor is where that rectangle lands on the page: a child's page position = anchor origin + (child coordinate − child-space origin) × (anchor size ÷ child-space size). Reasoning: sizes match 1:1 in 20 of 21 groups and children's relative offsets keep their sizes in EMU (`group__two`: child 2 starts 2 260 600 EMU = 178 pt = 144 pt + 34 pt right of child 1's left edge). The scale term is supported only by the one non-1 group; the translation by the 20 equal-size groups. **UNKNOWN / not verifiable from the files:** the rendered position of any child (no reference rendering allowed), nested groups (no group contains a subgroup in any of the 166 files), per-child rotation or flip inside groups.

**Group rotation — OBSERVATION:** `group__rotated.pub` equals `group__two.pub` except the group shape carries rotation property 0x4 = 0x1E0000 (30°); the group anchor, child-space rectangle and both child rectangles are unchanged. INFERENCE: group rotation is stored once on the group and the children stay in the unrotated child space. Whether the rotation centre is the centre of the group's anchor box is **UNKNOWN**.

## 4. What the data settles and what it does not

| Fact | Status |
|---|---|
| Anchor = 0xF010 property list, keys 1–4 = left/top/right/bottom, signed 32-bit EMU, absent = 0 | Settled by `geometry__*` |
| File-name values are points; stored unit EMU (12 700 per pt) | Settled (exact except two ±4 EMU deviations, cause unknown) |
| Origin = page centre (Letter portrait) | Settled for 612 × 792 by the controls; supported for A4 portrait and landscape by symmetry in Publisher-authored files |
| Page-relative (each page its own origin) | Supported (`SampleBrochure`, `pages__two-rect-p2`/`three-rect-p3`); facing-page spreads unknown |
| Origin for landscape / 5.75 × 4.75 pt pages from the controls | **UNKNOWN**: `pages__landscape` stores the same numbers as portrait (so the object, if it stayed at 72, 72 intended, would lie partly above a 612-pt-tall page, top = −18 pt); the intended position in these two files is not given by the names. In `pages__size-5.75x4.75` the object's link is on the *third* page-list entry (record 269), not the second (record 266) as in every other geometry file, so it may be in the scratch area (the drawing note already warns of scratch objects in this file) and its origin cannot be compared |
| Rotation = property 0x4, degrees × 65 536 | Settled; direction unknown |
| Anchor box swaps width/height at 90° and 270° (centre fixed) | Observed; rule between 30° and 90° unknown; interpretation unknown |
| Flip bits 0x40 / 0x80 of the shape flags | Settled by `flip-*` (anchor unchanged) |
| Group child space, translation | Inferred (see §3); scale term inferred from one file; nested groups unknown |

**Additional controlled samples that would settle the unknowns:** rotation 45° and 60° and 135° (find the swap threshold); a non-square asymmetric shape (arrow, right triangle) at 90° beside its 0° version, with a rendered reference, to settle whether to apply rotation to the swapped box; the same shape at the same intended position in portrait, landscape and 5.75 × 4.75 pages (made by creating the shape after the page setup); a facing-pages document with one shape on each side of the spread; a group with a subgroup; a group stretched to a non-1 scale next to its unstretched twin.

## 5. Table frames (the eleven table files)

Method: the table is the type-0x10 record in `Contents` (record directory, as documented); the drawing shape whose 0xF011 key 0x01 equals its sequence number is its frame; exactly one shape matched in each file. Row sizes and endings are the dimension-list entries (key 0x02 size, key 0x01 cumulative ending) after the column entries, as in `tables.md`.

| File | Table seq | Frame left, top, right, bottom (EMU) | Frame height EMU | Frame height pt | Row sizes (EMU) | Sum of row sizes | Last stored ending | Matches (≤ 1 pt) |
|---|---:|---|---:|---:|---|---:|---:|---|
| `Sample.pub`, `Sample2.pub`, `Sample3.pub`, `Sample4.pub`, `Sample2_2010.pub`, `Sample3_2010.pub`, `Sample4_2010.pub`, `Sample_2010.pub` (identical numbers in all eight) | 299 | −2412000, −2538000, 1152000, −1880436 | 657564 | 51.7767 | 216000 ×3 | 648000 | 657564 | **Both** are within 1 pt (sum is 9564 EMU = 0.753 pt below); **the last stored ending matches exactly (difference 0)**; the row-size sum does not match exactly |
| `table__2x2.pub` | 293 | −2971800, −4114800, 685800, −3200400 | 914400 | 72.0 | 457200 ×2 | 914400 | 914400 | both exactly |
| `table__3x2.pub` | 293 | −2971800, −4114800, 685800, −2743200 | 1371600 | 108.0 | 457200 ×3 | 1371600 | 1371600 | both exactly |
| `table__merged.pub` | 293 | −2971800, −4114800, 685800, −3200400 | 914400 | 72.0 | 457200 ×2 | 914400 | 914400 | both exactly |

**OBSERVATION:** for the eight Apache files, the stored endings are 219188, 438376, 657564 (each step = row size 216000 + 3188), and the frame height is exactly 657564 EMU = the last ending. Frame width is 3 564 000 EMU in those eight = the sum of the column sizes (1 782 000 × 2) and the declared width; in the controls frame width 3 657 600 = column sum. The frame left of the Apache tables is −2412000 EMU (−189.92 pt) on an A4 page of 7 560 000 × 10 692 000 EMU.

**Answer: the frame height matches the last stored row ending exactly (657 564 EMU), and sits 9 564 EMU (0.753 pt) above the row-size sum 648 000, which is also inside the 1 pt tolerance. For the controls, both agree.** This agrees with the previous retained-reader measurement recorded in `tables.md` (51.776692913 pt). **UNKNOWN:** why the ending steps are 3188 EMU larger than the row sizes (no cause is shown by the files); whether the frame is derived from the endings or the endings from the frame.
