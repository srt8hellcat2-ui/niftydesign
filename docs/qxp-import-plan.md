<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Opening QuarkXPress files (.qxp): investigation and plan

Status: **investigation. No reader exists yet. Three donated QuarkXPress 7.x documents and their exported PDFs were examined locally on 2026-11-28; the generic findings are in [qxp-format/observations.md](qxp-format/observations.md). The "Unknown" rows below describe what was known when this plan was written.**

## Request

A user wants to open QuarkXPress `.qxp` files. Many archived files date from the years when QuarkXPress was the
industry standard (to about 2005-2006). The workarounds they found are an InDesign script, a paid
converter app, or owning QuarkXPress and exporting IDML, which DesignCraft already opens.

## What the repository has today

- Importers are separate crates: `crates/idml` (IDML in and out), `crates/pub` (Publisher 2002+),
  `crates/pub-legacy` (Publisher 97-2000, text only), plus `epub`, `pdf`, `pptx` and `textimport`.
  `file.open` / `file.openBytes` dispatch by format, so a QXP reader would be a new crate at the same
  layer as `crates/pub` that returns a document plus `warnings` / `notes`, as described in
  [publisher-import.md](publisher-import.md).
- `docs/file-format-parity.md` lists QuarkXPress as not supported (#73, an upstream number).
- No open or closed issue or PR in this repository covers QXP, and nobody has claimed it.

## What can be written today, and what cannot

| Question | Status |
|---|---|
| Magic bytes and version fields of each QXP generation | **Unknown.** I have no public specification I can cite and no sample file. I will not write these from memory. |
| Legacy (3.x-4.x, Mac and Windows) record layout, text and picture storage | **Unknown.** |
| Newer files (5 to 2017+, and the newer container) | **Unknown.** Whether they share the legacy structure or use another container is unverified. |
| Byte order, string encodings, units | **Unknown.** |

Every claim in a reader has to come from a public document that is cited in a sources file, or from
bytes of a sample file we may use (see `docs/CLEANROOM.md` and `docs/pub-format/SOURCES.md` for the
pattern). Right now there are neither, so a "detector" would be invented constants, and any success it
reported would be presenting guesses as fact.

## Clean-room position

- Not read: `benforbrands/QXPConversion` (an InDesign script; the README says MIT, but the repository has no
  LICENSE file). Nothing is copied from it and the code is not needed.
- Not used: Markzware's page (marketing). No other QXP reader is consulted. GPL, LGPL and AGPL source is
  never read.
- Do not run QuarkXPress while writing the parser. If a behaviour specification is wanted, it is a
  separate task done by a different agent.

## Plan

1. **Samples.** Read the private files only through a served folder (see below), never copy them into the
   repository. Record only generic findings.
2. **Format notes** in `docs/qxp-format/`, written only from those bytes and cited public documents, each
   note marked "confirmed in <file>".
3. **Detector crate** (`crates/qxp`): identify the format and version from the header and return a typed
   error, "QuarkXPress version X is not supported", for everything else. It must never return an empty
   document for a file it cannot read. Test it against the real sample files and against truncated and
   corrupt copies.
4. **Content, in order of value:** pages and page size, text boxes and story text, then box geometry,
   colours and pictures. Each step comes with warnings for what is dropped, as the Publisher importer does.
5. **Hook into `file.open`** only once step 4 yields real text.

## Needed from the requester

The user's QuarkXPress samples live on an `E:\` drive. The cloud session's folder list shows only
`GameMachine/Projects` and `GameMachine/NiftyDesign`, both marked "not available", so they cannot be reached.
The samples are donated, private files for the owner's use only. They must not be committed, uploaded,
quoted or put in fixtures, issues, PRs or logs. They may be read only through a folder served to the cloud
session, and anything recorded from them stays generic (structure, byte layout, version markers; no file
names, text or images). Committed tests use tiny synthetic inputs built by us, with provenance recorded.

So the one way forward is to serve the sample folder to cloud sessions through the Claude desktop app
(Settings, Claude Code, Remote Control; the app may need updating).
