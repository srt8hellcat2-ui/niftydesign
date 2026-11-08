<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Reusing a calendar: Table > Calendar > Set Year

Open last year's calendar (a NiftyDesign file, or a Publisher file you opened), choose
**Table > Calendar > Set Year…**, type the new year (it starts at the current year) and press OK.
Every month is retyped in one step. **Edit > Undo** puts the whole calendar back.

## What it does

For each month it finds a text frame whose whole text is an English month name and a four-digit
year (for example `May 2026`) and a table on the same page or spread whose first row holds
weekday names (`Sun` to `Sat`, or `Mon` to `Sun`; the first three letters count, in any case).
For each pair it:

1. changes only the four year digits of the heading, keeping its formatting;
2. works out the weekday of the 1st in the new year, for a Sunday-first or Monday-first table,
   and retypes every day cell: the day number, or nothing where the month has no day, so days
   left over from last year are cleared (February has 29 days in 2024 and 2028, 28 in 1900 and 2100);
3. keeps each cell's character and paragraph formatting;
4. makes the table one header row plus as many week rows as the month needs, deleting rows from
   the bottom or adding rows below the last one (a new row copies the last row's look);
5. if rows were added and the table frame is now too short, grows the frame downward (at most
   4000 pt) until the table fits, or says that it could not.

## What it will not touch

- Text frames that hold anything besides the month and year (a heading followed by other words is
  skipped, with a warning).
- A table whose first row is not weekday names, that has other than 7 columns or has merged
  cells: it is skipped with a warning naming its page, never half-changed.
- A day cell holding something that is not a day number (a note such as `Dentist`): it is kept as
  it is and listed once in the warnings; the message says how many were kept.
- Any other text, picture or object in the document.

If no month heading with a calendar table is found, nothing changes and the message says so.

## For scripts and agents

The command is `table.setCalendarYear` with `{year: 1900..2200, story?, table?}`; it returns
`{months, changed, warnings, kept, message}`. `story` and `table` limit it to one calendar table.
