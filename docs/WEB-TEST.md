<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# testing .pub import in a browser

**Status: draft. The owner decides whether and when to share any web build.** Facts come from [REPORTS.md](REPORTS.md) (C12, 2026-10-20), [ROADMAP.md](ROADMAP.md), [DEBRAND.md](DEBRAND.md) and [open-publisher-files.md](open-publisher-files.md).

## What exists today (and what does not)

- DesignCraft has a browser app (`apps/designcraft-web`) built to WebAssembly.
- A manual-only workflow, `tester-web.yml`, is written to build it and upload the result as an artifact named `designcraft-web-test` (kept 14 days). **It has not been run on GitHub.** REPORTS.md says no hosted run, deployment or publication is claimed.
- Checked locally: the web app builds (Trunk release build, exit 0), and a headless test in Node opened the allowed sample file `hello-nifty.pub` through the engine (1 test passed: one page, two objects, expected text).
- **Not checked** (REPORTS.md says so): the browser's file picker, drag-and-drop, and on-screen rendering. Everything below about what you will see in the browser is **UNVERIFIED** until someone does it.

## How a tester would try it (unverified until the owner's first pass)

1. The owner gives you the web test build as a folder of files (`dist/web`) or a web address. It is a plain set of static files; a browser cannot open `index.html` straight from disk in every case, so the owner should say how to serve it (unverified).
2. Open the page in a current desktop browser.
3. Open a `.pub` file with the page's Open command, or drag the file onto the page. Which of these works is **unverified**: only the engine's open-from-bytes step has been tested.
4. Expect the document to open as a new, unsaved document with its pages, text, pictures and shapes, plus a list of anything not brought over fully (same as the desktop build; see [ALPHA-NOTES.md](ALPHA-NOTES.md)). Publisher 97/98/2000 files are refused with a message. Saving back to `.pub` is not offered.
5. What saving and PDF export do in the browser (a download? a file dialog?) is **unverified**. Printing in the browser is **unverified** and has not been checked.

## Your files stay in your browser (unverified claim to confirm)

The design is that the file is read by the page in your own browser and not sent to a server; the web build has no deploy, no secrets and no upload step in its workflow. **The owner should confirm this by watching the browser's network activity during a test before telling anyone it is true.** Until then, tell testers to use only non-private files.

## What to report

- Browser name and version, and operating system.
- What you did (opened by picker or by drop) and what you expected.
- The exact warning text or error message (errors start with "import:").
- Whether the page froze, went blank or crashed, and what you did just before.
- A screenshot of the result beside the original in Publisher, if you have it.
- The `.pub` file only if it is not private. Do not send files with personal information.

## Before a web build is shared publicly (owner steps)

1. **Remove the ArtCraft marks from the web build** following [DEBRAND.md](DEBRAND.md): new name without "ArtCraft"; no files from `docs/brand/`; replace the About screen mark and ArtCraft links (a code change, roadmap item C13, not yet done); add the plain-text line "Based on DesignCraft by the ArtCraft team. Unofficial build; not supported by ArtCraft."; keep NOTICE, LICENSE-MIT, LICENSE-APACHE, ASSETS.md and the font licences. Check the page title, favicon, icons and page text too (not covered by the repo's docs; unverified).
2. **Host it as static files.** The build output is `dist/web` (HTML, JavaScript, wasm). The workflow does not deploy anything; the owner would upload the folder to a static host. Which host is the owner's decision.
3. **Decide where it lives.** Address, name of the host account, and who is the contact. Not answered by any file in the repo. Keep the ArtCraft name out of the address and account (DEBRAND.md, section 1).
4. **Do the browser pass yourself first** (open, edit, save/export, check network activity) so the unverified items above become verified or are removed from the text.
5. **Owner-only:** posting the link anywhere public is the owner's action.
