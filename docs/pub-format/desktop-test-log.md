<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Desktop test log

Opening real `.pub` files in the running desktop app on Windows, driven through the control channel ([control-protocol.md](../control-protocol.md)), with a screenshot of the window after each open. This is the first run that used the real window, not only the headless export. It checks that files open and what the window shows; it does not compare with Publisher (see level 3 in [parity-roadmap.md](parity-roadmap.md)).

## Run 2026-11-22

- Build: `cargo build -p designcraft` (dev profile), branch `codex/legacy-open-continue`.
- Start: `target/debug/designcraft.exe --control 7979`; each file opened with `engine.execute` `file.open`, then `ui.screenshot`.
- Files: the 52 donated files (data only, not in the repository).

| Result | Files |
|---|---:|
| Opened | 51 |
| Error | 1 (`MYRESUME.PUB`, an early flat file) |
| Legacy family B | 21 |
| Legacy family C (58-entry) | 3 |
| Legacy D, text only | 8 |
| 2002 and newer | 19 |

The slowest open took about 0.5 s. The application log (`designcraft.log`) had no `ERROR`, `WARN` or panic line for the run.

## Findings

| # | Finding | Status |
|---|---|---|
| 1 | After a failed open, the status line kept the old error through later successful opens (the Wireless Flyer window still showed the `MYRESUME.PUB` message). | Fixed: a successful open clears the status line; a test covers it. |
| 2 | `MYRESUME.PUB` was reported as "not a Microsoft Publisher file: not an OLE2 compound file". It is a Publisher file in an early flat format. | Fixed: it is now reported as an early format that is not supported yet; a test covers it. The format itself is still unsupported (roadmap 1.1). |
| 3 | Wireless Flyer (family B): the full-page frame is drawn with its stored purple fill, but the large left-hand text frame is white in the original and has no decoded fill, so the purple shows behind its text. | Open: roadmap 2.5 and 2.6. |
| 4 | Flyer 200201 (family B): large black areas around the columns. The cause was not the fills but a 36-point black border read from the number 0x81 (1 with a flag bit). | Fixed: the flag bit is masked; the page now matches the stored preview by eye. |
| 5 | Family B files: the left column of Flyer 200201 ran below the page onto the pasteboard. The same 36-point borders took the text space. | Fixed with finding 4 for that file. Wireless Flyer still draws its checkmark symbols as the letter `c` (a symbol font is not mapped) and its left list runs past the page; open, roadmap 2.9. |
| 6 | The status bar shows large error counts for imported files, for example 74 for `Coupon - Three Up.PUB` (2002+), although the page renders. They come from text that does not fit its frame (196 such lines across the donated 2002+ files, for example "1990 characters of the story in frame 41 do not fit it"). | Open: decide how much of that is real overflow and how much font substitution; roadmap 3.9 and 3.10. |
| 7 | Family D files open as text on A4 with each form-feed segment in its own frame; they have no positions. A business-card file used to give 13 tiny pages and now gives one A4 page. | Known limit: roadmap 2.4. |

## Repeat

```powershell
cargo build -p designcraft
Start-Process target\debug\designcraft.exe -ArgumentList '--control','7979'
# then send {"id":1,"method":"engine.execute","params":{"command":"file.open","params":{"path":"..."}}}
# and {"id":2,"method":"ui.screenshot","params":{"path":"..."}} as JSON lines to 127.0.0.1:7979
```
