<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Sources (clean-room record)

These notes were written by the **describer** (task 3.2) on 2026-10-17, in one claude.ai/code session on branch `task/3.2-pub-format` of `srt8hellcat2-ui/designcraft-pub-development`, based on `feat/pub-import` at commit `93ee76e`.

The describer **was allowed** to read existing parser code. The **builder** (task 3.3) must not read anything in the "existing code" list below, and must not read libmspub or pubsmith's source code.

## Existing code read

| File (at `93ee76e`) | How much |
|---|---|
| `crates/pub/src/contents.rs` | All of it. |
| `crates/pub/src/quill.rs` | All of it. |
| `crates/pub/src/read.rs` | All of it (the property-list encoding; see "Needs a decision" below). |
| `crates/pub/src/lib.rs` | The module notes, the stream-opening code, and the parts that use the results of the two readers (page/master assignment, text frames, colour references, vertical alignment, alignment values). |
| `crates/pub/src/escher.rs` | One search for the client-data record, to describe how drawing records name their `Contents` sequence number. Nothing else. |
| `crates/pub/tests/import.rs`, `crates/pub/tests/corpus.rs` | Searched for the expected values of `hello-nifty.pub` and for how the corpus test is run. |

**Not read:** libmspub (any version), pubsmith's source code (`src/`, `tools/`), any other `.pub` reader.

## Sample files

| File | Where from |
|---|---|
| `crates/pub/tests/data/hello-nifty.pub` | In this repo. |
| 142 `.pub` files in `tests/fixtures/pub/` of `FuncularLabs/pubsmith` | Cloned at commit `0ccdf27` (2026-10-12) to `/home/user/pubsmith`, outside the repo; not committed. Only their bytes were examined. From pubsmith's `README.md` (two lines read) they were made with Publisher 2021. |

The samples were examined with throw-away scripts written in this session (not committed): one extracted streams with the `cfb` Rust crate (MS-CFB), the others printed hex and walked the structures using the rules being written down. Every "confirmed in" statement in these notes comes from that output.

## Public specifications (referred to, not restated)

- **[MS-CFB]** Compound File Binary File Format: the container.
- **[MS-ODRAW]** Office Drawing Binary File Format: the Escher drawing and picture streams, and the colour-reference layout.
- **[MS-OLEPS]** Object Linking and Embedding Property Set Data Structures: the summary-information streams (not used).

These were named from general knowledge; their text was not opened in this session.

## Needs a decision (for the clean-room setup in AUTOPILOT A5)

- `crates/pub/src/read.rs` holds the property-kind size table, which came from the same learning as `contents.rs` and `quill.rs`. A5 plans to give the builder the crate **without** `contents.rs` and `quill.rs` but **with** `read.rs`. For a clean record, leave `read.rs` out too and let the builder rebuild it from `property-lists.md`.
- `crates/pub/src/lib.rs` also contains format knowledge (colour-reference top-byte meanings, control characters in text, how placeholders and masters are used). It is needed by the builder as the interface the two modules must satisfy. It is the owner's call whether that is acceptable or whether its provenance note should say so.

## Clean-room repository setup (2026-10-18)

Created from designcraft-pub-development@6a979401aa23b0d3325e590057385b3c36c66a18 without history by a one-time setup session (Claude Sonnet) that only ran git archive and deleted files, never opened the removed files and wrote no builder code. Removed files: crates/pub/src/contents.rs, quill.rs, read.rs and the autopilot docs (REPORTS, TASKS, NEXT, AUTOPILOT). The builder is a different agent (Codex) with no access to the original repository.

## Builder (Codex)

### Builder (Codex): private source sync (2026-10-22)

Compared the existing private NiftyDesign export at d82d525 with this project's
pub-import changes from 45378356 through a61e5c0d. Used the existing export notes,
public-document drafts, asset ledger and retained images to preserve the export
exclusions and port moved documentation. No other .pub reader was consulted and
no format encoding was inferred. The protected readers were not changed.

Regression checks reuse the pinned controlled .pub corpus and its truth/*.json
only as a scoring oracle, never to infer encoding. The controlled table fixture
is used by the rendering regression; the pinned external Apache set is used only
for batch regression counts. No external .pub or real-file rendering is included
in this sync. The retained generic Sample2 screenshots and fictional Coffee Shop
images keep their existing provenance. The Windows workflow is included but was
not dispatched; no Windows desktop or physical-print result is claimed.


### Builder (Codex): manual Windows desktop demo preparation (2026-10-22)

Reused Apache POI Sample2.pub only as demo input, from test-data/publisher at
bf7c9b6a1490046517cbd8e1504e702a89be4bc9, SHA-256
c87ea7cc5606023c5fa99376631bbd45a8e9bdaba34c958057995b0e7ac8cc86.
The existing external pinned copy was used locally; the manual job downloads only
that .pub through the existing test-pub-cli.py manifest/hash verifier. No Apache
source, README, text output or other upstream files were opened/downloaded.
The 24 pinned real .pub inputs and 142 controlled .pub fixtures were reused only
for the required batch/corpus regression checks; truth/*.json was used for scoring
only, never to infer encoding. No new format rule or reader logic was introduced.

Own-project references: control-protocol.md, registered engine command signatures,
alpha-windows.yml, test-pub-cli.py/publisher-fixtures.csv, README graphics/log
instructions, Coffee Shop reproduce.py/verify.py, engine-values.json and the
existing native demo (full-text reference). The same three assets/demo PNGs and
bundled fonts were reused with their existing ASSETS.md provenance. Only temporary
local renderings were generated; no sample .pub or screenshots were committed.

Job-only helper versions: Pillow 11.3.0 and pypdfium2 4.30.0 from their PyPI
releases (https://pypi.org/project/Pillow/11.3.0/ and
https://pypi.org/project/pypdfium2/4.30.0/). Installed pypdfium2 metadata reports
`(Apache-2.0 OR BSD-3-Clause) AND LicenseRef-PdfiumThirdParty`; used to rasterize
our own exported PDF, not to read .pub files. No helper binary is bundled into
an app release. Windows app, adapter, CopyFromScreen and spooler behavior are
UNVERIFIED until an owner-dispatched run; no workflow was dispatched.


### Builder (Codex): upstream screenshot-readback audit (2026-10-22)

Fresh official storytold/designcraft `main` at `c3382b422c4ba6664a6ccf564ca9c50b49f55f4a`:
UI screenshot handler, canvas/cache and existing tests, relevant git history,
AGENTS.md/CLAUDE.md and CI/xtask conventions. Used only to check a native UI
capture regression and prepare an internal patch, not to infer any file encoding.
Only the synthetic native SVG/PNG regression was copied into the baseline clone.
No .pub samples, truth exports, other reader source or outputs were used.


### Builder (Codex): table host-frame linkage regression (2026-10-21)

Used our own tables.rs / composition / renderer and existing gallery diagnosis;
controlled table__2x2.pub from pubsmith 0ccdf27 (data only), SHA-256
72d2127f4ffdbe998f4f94414a28584e49af817991b70a564fda885dcb1a6a7d;
and the eight previously pinned Apache Sample/2010 table inputs, data only.
All Apache page renders stay in ignored internal storage. No outside source or
other reader was read; no binary encoding was inferred. The unchanged corpus
scorer used the existing 142 fixtures and truth JSON solely for scoring imports.
The fix preserves existing native model frame linkage, without decoding changes.



### Builder (Codex): private NiftyDesign source export (2026-10-21)

Used the merged project tree at 45378356, the existing public-repo drafts,
NOTICE and asset/provenance records. Visually reviewed the retained own-sample
gallery and UI captures, all seven Sample2 captures, and the inherited synthetic
magazine images. Removed the entire newsletter screenshot experiment rather
than redistributing its Microsoft template content. Existing owner-art coffee
shop artifacts are retained unchanged. File lists, removals, remaining
placeholders and verification are recorded in root EXPORT-NOTES.md.

The only edits to contents.rs, quill.rs and read.rs are the requested copyright
comment. No encoding rule or importer logic changed. Existing controlled
fixtures/truth JSON are used only by the unchanged corpus scoring test; no truth
data is used to infer an encoding. The existing pinned 24 Apache data files
were also run through our normal batch converter to check unchanged counts.
No external reader/source was consulted.


### Builder (Codex): NiftyDesign Coffee Shop demo (2026-10-21)

- Started from the already pinned external Apache POI data-only `Sample2.pub`,
  `apache/poi` trunk `bf7c9b6a1490046517cbd8e1504e702a89be4bc9`, repository licence
  Apache-2.0; SHA-256 `c87ea7cc5606023c5fa99376631bbd45a8e9bdaba34c958057995b0e7ac8cc86`.
  No `.pub` sample was copied into the checkout; no Apache source, `.txt`, docs,
  or another reader/output was read. The full pinned 24-file set was used only
  through our batch converter for before/after regression counts.
- Owner-added `assets/demo/bakery-background.png`, `croissant.png`, `coffee-cup.png`
  and their exact provenance `assets/demo/NOTES.txt`. ASSETS.md copies the notes
  and lists SHA-256 for all three images: AI-generated by the project owner via
  ChatGPT, owner-owned output under the OpenAI terms, project demo use. Their
  transparent-background regeneration note is retained; no extra rights claim
  or prompt detail was inferred.
- Our own `ui_shot`, control methods, engine command catalogue, native-file
  serializer and CLI/PDF/render output were used for the demo and exact round-trip
  comparison. Bundled Source Sans 3 / Source Serif 4, already credited in ASSETS.md.
  Pillow was used to inspect alpha/pixel values; `pdftoppm` rasterized our PDF
  output. Neither tool was used as a Publisher reader.
- The controlled 142-fixture corpus and `truth/*.json` were used **for scoring
  imports only**, via the existing corpus test. No encoding was inferred from
  truth files, images or this demo. No reader or format rule was changed.


### Builder (Codex): attempted official Liberation font bundle (2026-10-21)

Task-named official font project only: release
https://github.com/liberationfonts/liberation-fonts/releases/tag/2.1.5;
official linked binary URL
https://github.com/liberationfonts/liberation-fonts/files/7261482/liberation-fonts-ttf-2.1.5.tar.gz
was blocked on its objects.githubusercontent.com redirect. No font/archive was
obtained and no font SHA-256 or internal font version was verified.
Official tag 2.1.5: 4b0192046158094654e865245832c66d2104219e.
Read official tagged README.md and LICENSE (no .pub reader source). LICENSE URL:
https://raw.githubusercontent.com/liberationfonts/liberation-fonts/2.1.5/LICENSE;
SHA-256 93fed46019c38bbe566b479d22148e2e8a1e85ada614accb0211c37b2c61c19b.
The text is SIL OFL 1.1, with Google 2010 / Red Hat 2012 copyright and reserved
font names. This records the license text only, not a shipped/bundled font.

Inspected only our own font resolver and packaging guard/tests for independent
auditing; no mapping logic, fonts, tests or package contents changed. Existing
pinned Sample2.pub/SampleNewsletter.pub were privately rendered as baseline
checks; normal regression reused the same allowed 24 Apache binary inputs and
142 controlled inputs, with their truth JSON used for scoring only. No new
Publisher encoding, third-party .pub reader, unofficial font source or system
font copy was used to implement this blocked task.

### Builder (Codex): Apache content-only public-candidate scan (2026-10-21)

Reused only the 24 existing external `.pub` files from apache/poi trunk at
bf7c9b6a1490046517cbd8e1504e702a89be4bc9, test-data/publisher, as data for this
scan. Repository license remains recorded as Apache-2.0; all 24 SHA-256 values
match the existing own tester manifest and the earlier source table, before and
after. No download or upstream listing, README, text output, code or other reader
was accessed. Own native document JSON provided extracted story/cell text;
our own CLI rendered every page of all 19 successes (31 pages), all inspected.
The derived private text files are our own outputs, not Apache .txt fixtures.
Used only own engine/control/preflight/native-save/PDF functions, own ui_shot
renderer and pdftoppm on our PDF. No format encoding was investigated or changed.

Selected existing Sample2.pub, 72704 bytes, SHA-256
c87ea7cc5606023c5fa99376631bbd45a8e9bdaba34c958057995b0e7ac8cc86, for the interactive
check and seven complete unretouched PNGs. Generic/test content only in observed
text/renders; no Microsoft-template/PII/unclear-photo candidate was promoted.
Classification applies to examined importer output, not unimported hidden data.
No image was embedded in a README, and no original sample/native/PDF/CSV or raw
extracted text was committed. No task owners/ticks or production code changed.

Required regression scoring reused the pinned 142 controlled .pub files solely
through the existing corpus test and its Publisher truth/*.json scoring oracle.
Truth was used for scoring only, never for classification or encoding inference.
Normal native tests exercised their existing synthetic fixtures. No new source
or decoding rule, protected-reader change, hosted workflow, deploy or publication.

### Builder (Codex): follow-up 9, one real-file editing test (2026-10-20)

Downloaded only `apache/poi`, `trunk` at pinned
`bf7c9b6a1490046517cbd8e1504e702a89be4bc9`,
`test-data/publisher/SampleNewsletter.pub`, from the raw-file URL constructed by
our own `.github/scripts/test-pub-cli.py`, using its existing manifest row and
bounded `prepare` download/hash check. Repository license recorded as Apache-2.0.
Size 291840 bytes; SHA-256
`6a825ba26ba35d6e885acdc62e859591ed37cb0ff7480b554b9cb362b644dfcf`.
Only this one .pub was downloaded for the interactive investigation. No Apache
listing, source, text output, README or documentation was fetched/read. Input
remains outside the repo; the separate 24-file regression is recorded below.
Inspected its four own-rendered pages and used only page 2 for retained internal
UI/PDF evidence. Pages with names/contact fields and the complete PDF/native
output remain private/uncommitted. No image was cropped, retouched or embedded
in any README. Own ui_shot, own engine/control/native state and pdftoppm of our
PDF provide the evidence; no other reader/output or truth file was used for it.

Final regression only reused the preexisting pinned 24 Apache .pub data files
and 142 controlled .pub files through the own tester/batch commands. Their input
hashes still match the manifest. Publisher truth/*.json was used only by the
unchanged corpus scoring test, never to derive encodings. No .pub reader logic,
protected source file or new encoding rule changed in any of follow-ups 1–9.

### Builder (Codex): follow-up 8, WMF warning disclosure (2026-10-20)

Used own screenshot/import log for regenerated wmf-picture.pub and existing
warning-triage.md/package README. No new sample, external source, truth use,
encoding inference or reader change.

### Builder (Codex): follow-up 7, provenance clarification (2026-10-20)

Used this SOURCES.md only. Its original 142-fixture entry records pubsmith
0ccdf27 and Publisher 2021, but no operator identity or human-versus-generator
production method. Both are UNKNOWN here; no new pubsmith README/code was read.
This differs from our own hello/WMF synthetic fixtures, whose generator helpers
are recorded above. No new data, encoding inference or reader change.

### Builder (Codex): follow-up 6, performance (2026-10-20)

Used own CLI perf::bench, UI example/command dispatch, frame.create and
transform.move, own regenerated hello-nifty and allowed pinned controlled
picture__oval-mask.pub only. Largest-file selection used file lengths, not any
format inference. Added an original synthetic text probe via engine commands
because that picture sample contains no imported text box. No new encoding,
truth-data use or engine/reader change.

### Builder (Codex): follow-up 5, zip checksum (2026-10-20)

Used only own alpha workflow/staging/packaging tests and Python standard hashlib
and zipfile APIs. Tests use explicit executable stand-ins, not a Windows build.
No new sample, format source, encoding inference or reader change.

### Builder (Codex): follow-up 4, blank table diagnosis (2026-10-20)

Used only controlled table__2x2.pub at the existing pin, tables.md and our own
native document, tables::table_frame, compositor and renderer. No truth file or
new encoding inference. No importer/reader/rendering code was changed.

### Builder (Codex): follow-up 3, Save As (2026-10-20)

Used only own UI save routing, desktop rfd picker integration, engine save/open
commands, native document JSON and the regenerated own hello-nifty.pub. No
external source, new encoding or protected-reader change. Native OS dialogs
were not captured or simulated.

### Builder (Codex): follow-up 2, synthetic sample text (2026-10-20)

Regenerated hello-nifty with the existing `publication` helper in our own
`crates/pub/tests/paragraphs.rs`, exposed as ignored `write_hello_nifty_sample`;
regenerated wmf-picture with the existing `write_wmf_picture_sample` helper.
No binary patching, outside writer, new encoding rule or protected-reader edit.
Compared old/new native documents: only story text/run length changed (and the
input-derived title when temporary paths differed); styles and geometry match.
Current fixture text is `Hello from NiftyDesign` (22 UTF-8 bytes). Current hashes:

- `hello-nifty.pub`: `5e22460c8bcff08d39362f68177430ca953a136083dc35ffadb95c3375f92c1a`.
- `wmf-picture.pub`: `7606a8fd9c4f9a5474a29642ec845e636ee2b22d9a7613c3ea5b2e85d435903f`.

Regenerated only own-sample gallery/UI images with our CLI/UI renderer and
pdftoppm for our exported PDFs. The allowed pinned 142 .pub files remain
unchanged; their truth JSON is used only for regression scoring, never encoding.

### Builder (Codex): follow-up 1, Print (2026-10-20)

Used the own CLI command registry, own UI menu/shortcut/dialog code and own engine
print/native backend. Used only the existing hello-nifty.pub for a non-submitting
print dry-run. No new format source, sample, encoding inference or reader change.

### Builder (Codex): C15 interface renderings (2026-10-20)

Read this repository's PUB-AGENTS, ROADMAP, own ui_shot example, UI control/menu/
dialog code, engine command descriptions and text command implementation. Used
the same cached own C13 renderer built from 6561b0e7, with source-tree equality
verified against the corresponding current trees; it predates the alpha version
tag. README records this limitation. Binary SHA-256:
`3dcec6b3a52c489a8c40bc4a98e895d2f9e264d650ca24dceb1053cbea7c6dec`.

Capture data only:

- `crates/pub/tests/data/hello-nifty.pub`: SHA-256 `acd57ac1aecb534c2d30d13550d07d91d727405c909f249aea1b6d6e88e2893e`.
- `crates/pub/tests/data/wmf-picture.pub`: SHA-256 `8f08dba2e78a7895e7d7ba40bbb7681524836ea2ea7d2229e96896ca2975a05e`.

Both remain unchanged. New text was entered through our engine's story.replaceRange
command; model IDs/ranges came from normal document inspection, not byte encoding
inference. The installed pdftoppm rasterized our own edited-hello PDF only. No
images were retouched, composited or sourced from Publisher truth files. No other
.pub reader or donated/Apache sample was used. The separate regression score used
the existing pinned controlled fixtures and truth JSON for scoring only; no truth
file was used to infer any encoding or generate these captures.

### Builder (Codex): headless gallery (2026-10-20)

Used only the task-named repo fixtures and allowed controlled .pub data below,
plus our own CLI output. No Apache/donated/real-user files were used for gallery
images, and no truth JSON or Publisher reference image was used for this gallery.
Used the installed pdftoppm solely to rasterize our own exported hello-nifty PDF.
No other .pub reader or external source was read. No source fixture was added.
Controlled source fixture licensing was not independently verified or relabelled;
asset provenance records that limitation rather than inventing a licence.

| Source (page 1) | SHA-256 |
|---|---|
| `crates/pub/tests/data/hello-nifty.pub` | `acd57ac1aecb534c2d30d13550d07d91d727405c909f249aea1b6d6e88e2893e` |
| `crates/pub/tests/data/wmf-picture.pub` | `8f08dba2e78a7895e7d7ba40bbb7681524836ea2ea7d2229e96896ca2975a05e` |
| `table__2x2.pub` | `72d2127f4ffdbe998f4f94414a28584e49af817991b70a564fda885dcb1a6a7d` |
| `text__bullets.pub` | `51df14aa94d640932656bc49a3f645b97333ba811467761e31226ec1547011f5` |
| `shadow__on.pub` | `b01c8e35d89bcccc1d1e13e85be2c48eb82128c8a88d36adbfd4503f54f8d209` |

All five CLI renders succeeded. OBSERVATION: table__2x2 produced a uniform white
PNG, despite successful exit; its cause was not investigated and no encoding
rule is inferred. The hello-nifty PDF was rasterized successfully for the paired
comparison. The separate regression corpus still uses truth JSON for scoring
only; it is not an image source for the gallery.

### Builder (Codex): C14 Windows alpha package (2026-10-20)

Used this prompt's package/README requirements and this repository's PUB-AGENTS,
ROADMAP, real-file-triage, existing branding/legal guard, app resource/version code,
font/icon embedding declarations and normal CLI commands. Local smoke input:
crates/pub/tests/data/hello-nifty.pub (hash already recorded below). No new external
source or sample was obtained. Regression inputs are the previously pinned
24 Apache .pub files and 142 pubsmith fixtures. Truth JSON was used only for
corpus scoring, never for encoding inference. No reader code was changed.

### Builder (Codex): manual-only platform packaging triggers (2026-10-20)

Read this repository's PUB-AGENTS, roadmap/reports, ARM64 and FreeBSD workflows,
and the Packaging lint/Nix trigger mappings named by the task. No external build
source or format reference was obtained. Regression inputs were the existing
24 Apache samples and 142 pubsmith fixtures at the pins recorded below. Truth
JSON was used only for the corpus scoring run, never to infer encodings.

### Builder (Codex): ARM64 installer identity and alpha version (2026-10-20)

Read this repository's PUB-AGENTS, ROADMAP, CLEANROOM, reports, Windows workflow,
WiX/package scripts, app resource builder, About/version code and CLI tests.
Inspected the previously referenced shared **build workflow only**,
https://raw.githubusercontent.com/storytold/craft-common-actions/main/.github/workflows/windows-arm64.yml
(retrieved 2026-10-20; SHA-256
`94b72b66422aee879ea1e9cef1823dcb52d9ec41bc39406ec32086a2dabd99f9`),
to check its inputs and install assumptions; no .pub reader or format source.
Checked the installed winresource dependency's documented version-string defaults
for Windows executable metadata. The task changes build/packaging identity only.
The existing 24 Apache inputs and 142 pubsmith fixtures remain at the pins recorded
below. Truth JSON is used for scoring only, never to infer encodings.
Owner logo files remain at their canonical renamed paths, with the hashes already
recorded in the C13 entry. No new artwork or fixture was obtained.

### Builder (Codex): C13 NiftyDesign identity (2026-10-20)

Used the current prompt's product name, exact About text and owner-supplied
original artwork, merged from `origin/pub-import@04d2d259`. Visually inspected
both PNGs before renaming (bytes unchanged):

- `assets/nifty/nifty-wordmark.png` (formerly `1a9e3825-ce81-4fee-b39c-53dcc06a4a9d.png`): SHA-256 `4a0988a4c74c8cb8322ed04fea0137864da5f8e3c9040a5abc15ffd402c4ca65`.
- `assets/nifty/nifty-mark.png` (formerly `df0b4ded-91fd-4b18-b02a-cec1cf0278b2.png`): SHA-256 `4cff0ad7fa05a5904ffc30d229be2e3630362b54851ea72890a5f4b939d287d0`.

Both are the owner's original NiftyDesign branding, rights reserved by the
owner, licensed for use in this project, as stated in this task. Icon outputs
were generated from the mark, without using upstream mark artwork.
Read this repository's PUB-AGENTS, ROADMAP, DEBRAND, brand licence, asset ledger,
UI/native/web identity code and existing packaging code. No format knowledge
was derived, no reader was consulted, and no protected reader was edited.
Regression inputs are the previously recorded 24 Apache `.pub` samples and
142 pubsmith fixtures at their existing pinned commits. Truth JSON is used
only by the corpus scoring run, never to infer encodings.


### Builder (Codex): C12 WebAssembly tester (2026-10-20)

Used our existing web app/Trunk configuration, UI open/drop dispatch and engine
`file.openBytes` implementation, Cargo manifests/lockfile, and package-output
wasm branches. The new runtime test uses only the repository's allowed
`hello-nifty.pub` bytes. No format encoding was investigated or changed; no
other reader, new sample, web format search or specification text was used.
Regression runs reused the 142 pubsmith `.pub` files at `0ccdf27` and the 24
Apache `.pub` files at `bf7c9b6a1490046517cbd8e1504e702a89be4bc9`, with
existing recorded hashes. Truth JSON was used for scoring imports only, never
for encoding inference. External data remains outside the checkout (the local
ignored scoring-cache path is a symlink to that external data).
Build tools: Rust 1.95.0, Trunk 0.21.14, wasm-bindgen CLI 0.2.129 (matching
Cargo.lock), Node 24.19.0 locally; workflow selects Node 24. Tool installation
retained Cargo lock/checksum and TLS verification.


### Builder (Codex): C11 Windows-only workflow (2026-10-19)

Used only our existing workflow, verifier and pinned-data manifest; no format
work or new external source. Regression checks reused the allowed hello-nifty,
142 pubsmith fixtures at `0ccdf27` and 24 Apache `.pub` inputs at
`bf7c9b6a1490046517cbd8e1504e702a89be4bc9`, with existing recorded hashes.
Truth exports were used only by the scoring corpus, never for encoding inference.
No sample download, other reader, web format search or specification access.


### Builder (Codex): C10 tester CLI workflow (2026-10-19)

No format encoding work. Used our CLI/Cargo/workflow configuration and our own
batch outputs. Pinned data inputs: the same 142 FuncularLabs/pubsmith `.pub`
files at `0ccdf27`, and the same 24 apache/poi `.pub` files at
`bf7c9b6a1490046517cbd8e1504e702a89be4bc9` (existing Apache-2.0 source/hash
record below). Added a hash/size/name-only manifest for reproducible CI; data
stays outside the checkout. Tested individual raw `.pub` downloads for
`fill__fill-green.pub` and `51318.pub`; SHA-256 matches the manifest. The workflow
requests only the individually named .pub files, never a repository listing,
source, README, .txt reader output or archive. No other reader was accessed.
Truth JSON was used only by the existing scoring corpus, not by the workflow
verifier and never for encoding inference. Actions API access failed at CONNECT
403; no hosted execution/output is claimed. No new MS-ODRAW access was attempted
for C7–C9, which remain blocked.


### Builder (Codex): queue continuation C6 (2026-10-19)

Source detail: C2/C3 used only `unknown-fields.md` observations and existing
property-list/text-store notes; synthetic tests encode the already documented
kinds and values. Reused the permitted pubsmith `.pub` fixtures at `0ccdf27`
and 24 external Apache `.pub` inputs at
`bf7c9b6a1490046517cbd8e1504e702a89be4bc9`; their hashes/license remain recorded
below. Corpus truth JSON was used for scoring only. C7/C8/C9 each attempted
`https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/`;
all three received proxy CONNECT 403, so no public-spec text was obtained or
used for geometry. No alternate source, web format search, reader code or
reader output was used. Final warning inventories come from our own CSV output;
comparison of our own 19 native document JSON payloads showed creation-time
changes only. No downloaded sample, generated CSV or PDF was committed.

Refreshed final real-file and warning inventories after C2/C3. Controlled batch remains 79 OK / 63 warnings / 0 failed (exact warning texts 9 to 7; file-warning pairs 120 to 66); real batch remains 0 OK / 19 warnings / 2 refused / 3 failed (texts 17 to 14; pairs 72 to 64). C2/C3 are DONE; C7/C8/C9 are BLOCKED by MS-ODRAW proxy access. Final ordinary suites: 69 passed, 10 ignored; the opt-in fixture warning and real-batch harness tests separately passed. Nineteen real document payloads differ from the post-merge baseline only in creation timestamps. Validation reused existing pinned Apache .pub files and allowed pubsmith fixtures. Truth exports were used only by the scoring corpus, never to infer encodings. No other reader or web format search.


### Builder (Codex): queue continuation C9 (2026-10-19)

Public MS-ODRAW text at https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/ could not be obtained: proxy CONNECT 403 Forbidden, curl exit 56. It is unknown whether its text defines the requested WordArt geometry or only names types; no geometry or source substitution was attempted. Needs access to the permitted specification. Full suites: 69 passed, 10 ignored. Validation reused existing pinned Apache .pub files and allowed pubsmith fixtures. Truth exports were used only by the scoring corpus, never to infer encodings. No other reader or web format search.


### Builder (Codex): queue continuation C8 (2026-10-19)

The sole allowed geometry source, public MS-ODRAW at https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/, is inaccessible: proxy CONNECT 403 Forbidden, curl exit 56. No specification text was obtained, so no heart or other preset geometry was inferred or changed. Needs public specification access in this environment. Full suites: 69 passed, 10 ignored. Validation reused existing pinned Apache .pub files and allowed pubsmith fixtures. Truth exports were used only by the scoring corpus, never to infer encodings. No other reader or web format search.


### Builder (Codex): queue continuation C7 (2026-10-19)

Public MS-ODRAW text is unreachable: https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/ returned proxy CONNECT 403 Forbidden (curl exit 56; no origin response or specification text). No other source was used and no freeform code was changed. Needs access to this public specification through the configured environment. Full suites: 69 passed, 10 ignored. Validation reused existing pinned Apache .pub files and allowed pubsmith fixtures. Truth exports were used only by the scoring corpus, never to infer encodings. No other reader or web format search.


### Builder (Codex): queue continuation C3 (2026-10-19)

Implemented the unknown-fields.md 0x3C co-occurrence warning rule in crates/pub/src/quill.rs: suppress only when the same record imports alignment 6; keep the numeric warning otherwise. Tests cover both observed values, property order, absent and other alignments, and separate records. The opt-in real justification fixture passed. Full suites: 69 passed, 10 ignored. The prior scope blocker is resolved. No meaning is claimed and paragraph 0x30 still warns. Used the named pub-format notes, existing pinned Apache .pub files and allowed pubsmith fixtures. Truth exports were used only by the scoring corpus, never to infer encodings. No other reader or web format search.


### Builder (Codex): queue continuation C2 (2026-10-19)

Implemented same-record 0x39/0x0C and 0x48/0x44 exact comparisons in crates/pub/src/quill.rs using unknown-fields.md observations only. Matching unique copies are quiet; differing values, missing partners, different kinds and duplicates retain numeric warnings. Tests cover both orders and separate records; the opt-in fixture check passed. Full suites: 68 passed, 10 ignored. The prior scope blocker is resolved by this prompt. No meaning claimed and no style attributes changed. Used the named pub-format notes, existing pinned Apache .pub files and allowed pubsmith fixtures. Truth exports were used only by the scoring corpus, never to infer encodings. No other reader or web format search.


### Task 5.4 — Publisher workflow (2026-10-18)

No new Publisher encoding was learned. Used only this repository's `hello-nifty.pub` as a synthetic `flyer.pub`, existing import/session/selection/transform/text/placement/native-save/PDF-export commands, UI save/print routing and native file-picker integration. The workflow test creates its own tiny PNG using the existing renderer. Printing work consulted only the pinned craft-print spooler API (`d991bf6`, `crates/print/src/lib.rs`) and ashpd 0.11.1's desktop print portal API/error types, plus standard OS dialog interfaces; these are printing libraries, not Publisher readers. No pubsmith code, unfair_codec_pub, other .pub reader or prohibited repository was opened. The existing allowed 142 binary fixtures and Publisher truth JSON remain the final import regression corpus. Used as a scoring oracle under TASKS.md item 2.1 (Publisher's own layout exports, data not code). The owner reviewed the use on 2026-10-18 and kept it, on the condition below. The owner had not authorized it beforehand.

### Shape shadows in designcraft-pub (2026-10-18)

Compared only the Escher properties in the allowed `shadow__on.pub`, `shadow__offset.pub` and `wordart__shadow.pub` binaries from pubsmith `0ccdf27`. Read the corresponding Publisher-generated `truth/*.json` exports for scoring; these contain flattened image objects and no editable shadow attributes. Used as a scoring oracle under TASKS.md item 2.1 (Publisher's own layout exports, data not code). The owner reviewed the use on 2026-10-18 and kept it, on the condition below. The owner had not authorized it beforehand. No encoding was derived from these exports. No pubsmith code, truth generator, forbidden reader/repository, or other fixture source was accessed. Documented measured flags, colour and 76200-EMU offsets in `drawing-details.md` before implementation. Reused only this repository's retained WordArt shadow property/default mapping and native DropShadow/renderer offset interfaces for the shared mapping; no new specification text or guessed non-default shadow type was used. Synthetic tests exercise the existing opacity/signed-offset interpretation, limits and unsupported cases; real-fixture assertions cover both shape-shadow samples and unchanged WordArt behavior.

### Warnings round 2 (2026-10-18)

Ranked the locally generated cleanup CSV (`.builder/warnings-converted/report.csv`). Rechecked this repository's clean-room/format notes and only allowed pubsmith binary fixtures at `0ccdf27`: `fill__transparency-50.pub`, `fill__fill-grey.pub`, `fill__fill-red.pub`, `shapes__polyline.pub`, `shapes__curve.pub`, `shapes__heart.pub`, `wordart__base.pub`, `wordart__arch.pub`, `wordart__circle.pub`; scanned character/paragraph run properties across the same 142 binary streams. Existing table observations cover all three table-bearing fixtures. Wrote `drawing-details.md` before importing solid-fill opacity. Used this repository's retained gradient-opacity interpretation, native `Fill`/gradient/`Item` interfaces and PDF rasterization behavior to preserve fill-only transparency and assess output diagnostics. No other reader, truth data, template, forbidden repository or new sample was used.

Requested the public MS-ODRAW landing page at `https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/`; the proxy returned 403, so no specification text was obtained or used. The API check also returned 403. Feedback discovery checked the selected repository's main tree and advertised `feedback/*` branch refs; none was present. Output comparisons use only this converter's own native documents and PDFs.

### Task 3.3 (2026-10-18)

Implemented `contents.rs`, `quill.rs`, and `read.rs` from scratch using:

- This repository's `docs/CLEANROOM.md` and every document in `docs/pub-format/`: `README.md`, `property-lists.md`, `contents-stream.md`, `text-store.md`, `spot-checks.md`, and this source record.
- The retained `crates/pub/src/lib.rs` and `escher.rs`, and `crates/pub/tests/import.rs` and `corpus.rs`, for required interfaces, behavior, and test commands.
- `crates/pub/tests/data/hello-nifty.pub`, exercised by the existing integration tests. The existing WMF integration fixture is exercised only by its existing test, not used to derive the new readers.
- Only the 142 `.pub` fixture files under `FuncularLabs/pubsmith/tests/fixtures/pub` at commit `0ccdf27`. Obtained their paths from the GitHub fixture-directory listing and downloaded individual binary files from raw.githubusercontent.com; no source checkout, source files, README, or truth JSON was read. The GitHub API directory request was denied (403); no API data was used.

Repository `AGENTS.md`, Cargo manifests/configuration, and `rustfmt.toml` were consulted for setup and project rules. No external reader implementation or forbidden repository was opened, cloned, or searched. Microsoft's specification texts were not needed or opened. Tests added to the new modules use synthetic bytes and the documented rules.

### Task 4.1 — paragraph-format observations (2026-10-18)

- Rechecked this repository's `docs/CLEANROOM.md`, paragraph-format notes, retained Quill/importer code and document-model attribute interfaces. No `docs/FEEDBACK.md` existed on `main` (`74a0c5f`), and the open-PR query returned no feedback PRs.
- Compared the raw Quill streams of `text__base.pub`, `text__bullets.pub`, all five `text__ls-*.pub`, all three `text__indent-*-18.pub`, `text__space-before-12.pub`, and `text__space-after-0.pub`, from the previously allowed pubsmith fixture files at `0ccdf27`. Scanned FDPP keys and TEXT U+0009 units across all 142 binary files. No numbering/custom-tab sample was present. Read only the current GitHub fixture-directory path listing to establish it has the same 142 files; no source, README or truth JSON was opened.
- Used the existing generic `cfb` dependency to extract streams with a local helper, then a temporary property decoder written from the documented kind table. Synthetic tests transcribe the observed property bytes and construct a Quill/CFB container; they do not invent numbering or custom-tab layouts.
- Consulted this repository's `crates/doc/src/attrs.rs` and the existing composition leading interface to map imported values to native document attributes; these are document-model/rendering interfaces, not Publisher format sources.
- The user additionally authorized only Govdocs1 document 923566, in `https://digitalcorpora.s3.amazonaws.com/corpora/files/govdocs1/zipfiles/923.zip`. Access initially failed at the network proxy (CONNECT HTTP 403); no bytes from that archive have yet been used. No Microsoft templates or samples from other reader repositories were used.


### Task 4.2 — native tables (2026-10-18)

- Read this repo's `docs/CLEANROOM.md`, retained Contents/Quill/Escher/import interfaces and prior format notes. The detailed new observations were written to `docs/pub-format/tables.md` before implementing them.
- Used only pubsmith `.pub` fixture bytes at `0ccdf27`: extracted Contents/Quill/Escher streams from `table__2x2.pub`, `table__3x2.pub`, `table__merged.pub`. A record-directory-only scan across all 142 binary fixtures established that these are the only type-0x10 table samples. Inspected only bounded binary stream structures, never pubsmith code or truth JSON. A local generic CFB extraction helper uses the existing `cfb` dependency; scratch property/drawing decoders were written from the retained format descriptions.
- Consulted `crates/doc/src/table.rs`, `story.rs`, `item.rs`, and the table importer section of `crates/idml/src/import.rs` only for the native DesignCraft model and its API. These are not Publisher format sources.
- No Microsoft specification text, Microsoft template, prohibited reader source or additional external sample repository was used. Feedback discovery checked the selected repository's main tree and advertised Git PR/branch refs; no feedback content was found. GitHub API access initially returned proxy Forbidden.


### Task 4.7 part 2 — character formats (alongside 4.2)

Compared only raw FDPC records in the allowed binary fixtures `text__base.pub`, `text__allcaps.pub`, `text__smallcaps.pub`, `text__superscript.pub`, and `text__tracking-200.pub` at pubsmith `0ccdf27`. Documented the measured keys/kinds/values in `text-store.md` §12 before implementation. Used native `Capitalization`, `Position` and tracking attributes from this repository's `crates/doc/src/attrs.rs`; no other reader, specification text, template, source repository or truth file was consulted. No sample establishes subscript or strikethrough bytes, so those were not guessed; unknown character fields/values warn.


### Builder (Codex): task B / 6.4 batch conversion (2026-10-18)

No new Publisher-format encoding was learned. Consulted this repository's existing CLI command dispatch/run/export APIs, engine Session/DocState and native/PDF export commands, and native-format save/load and PDF page-count interfaces to compose the batch workflow. Used only the existing allowed `hello-nifty.pub` for synthetic folder/error integration tests and the 142 allowed pubsmith `.pub` binaries at `0ccdf27` for batch validation. No pubsmith source/truth files, other Publisher reader, template or forbidden repository was accessed. The direct CLI dependency on this repository's rebuilt `designcraft-pub` provides the bounded import API.

Batch artifact validation reads only the converter's own generated DesignCraft ZIP/JSON files and PDFs (`pdfinfo` page counts), not a Publisher reader or external oracle. All 142 binary inputs produced both output formats; results and 146-page total are recorded in `docs/REPORTS.md`.


### Builder (Codex): warnings cleanup (2026-10-18)

Used the locally generated 6.4 CSV (`.builder/6.4-converted/report.csv`) to count distinct warnings by affected file. Rechecked only the allowed pubsmith binary Quill streams at `0ccdf27`: all 142 section directories and STSH default character records; character/paragraph run fields across the corpus; specific base/size/font/colour/table/shape variants. Confirmed 88 files have no TEXT and 147 exact colour descriptors match their already parsed PL colour indices. Wrote the observations into `text-store.md` §13 before implementation. Retained unconfirmed style/table fields rather than inventing encodings. No reader source, truth JSON, Microsoft template or prohibited repository was opened; no new public specification text was needed. Inspected only our own Quill importer and synthetic tests to implement diagnostics and verify retained colour behavior.

Further validation checked Contents object records in the same 88 no-TEXT binary fixtures: none references a Quill story. Re-ran the batch against all 142 allowed inputs and compared only our own generated native JSON/PDF page counts with the previous 6.4 artifacts. Native snapshots match except creation timestamps; the 142/142 import corpus remains passing. No additional Publisher format source or external oracle was used.


Shape-shadow validation also reuses all 142 Publisher `truth/*.json` layout exports at pubsmith `0ccdf27` with this repository's unchanged corpus scorer. Used as a scoring oracle under TASKS.md item 2.1 (Publisher's own layout exports, data not code). The owner reviewed the use on 2026-10-18 and kept it, on the condition below. The owner had not authorized it beforehand. No encoding was derived from these exports. All five supplied baseline scores remain unchanged.


### Builder (Codex): score check in designcraft-pub (2026-10-18)

Used the existing corpus test unchanged with all 142 already-allowed pubsmith binary fixtures at `0ccdf27` and the corresponding 142 `tests/fixtures/pub/truth/*.json` files downloaded individually from raw.githubusercontent.com at that commit. Used as a scoring oracle under TASKS.md item 2.1 (Publisher's own layout exports, data not code). The owner reviewed the use on 2026-10-18 and kept it, on the condition below. The owner had not authorized it beforehand. No encoding was derived from these exports. No pubsmith source, README, reader implementation, prohibited repository or other truth generator was read. A local SHA-256 manifest records the downloaded JSON data. Consulted only this repository's existing comparison test, manifests/lockfile and clean-room documentation to run the score check. All five scores match the supplied baseline.


### Builder (Codex): task A / 4.2b (2026-10-18)

Reused only the allowed binary fixtures at pubsmith `0ccdf27`: scanned all 142 Contents record directories, then all 13 MCLD blocks, Contents cell records and Escher properties in `table__2x2.pub`, `table__3x2.pub`, `table__merged.pub`. Recorded SHA-256 hashes in `tables.md`. No truth JSON, external reader source, template or additional repository was consulted. No new cell-paint mapping could be confirmed; the existing warning remains.

Migrated the above historical audit and the tables.md audit section from our own unmerged `codex/4.2b-cell-style` branch (tip `1fe1a28bf51c2206087add5658a205c5e09781d6`, substantive audit commit `6f44ccf`) in designcraft-pub-cleanroom. Only our documentation was copied; no implementation, dependency or reader source was brought over. Rechecked the three binary fixture hashes against the recorded values. The Publisher truth JSON for these tables contains flattened image elements, limiting scoring coverage of editable cell fills/borders. Used as a scoring oracle under TASKS.md item 2.1 (Publisher's own layout exports, data not code). The owner reviewed the use on 2026-10-18 and kept it, on the condition below. The owner had not authorized it beforehand. No cell-style encoding was derived from these exports.


Task 5.4 validation also compiled the app's pure Open-filter/Save-path helpers and their retained regression case in a standalone Rust test harness, without building the application. The default document picker now explicitly lists Publisher. Shared PDF preparation in the existing engine exporter preserves shadows/effects and placed-layer visibility for both Print and Export PDF. Validation compared rendered PDF pages (not variable metadata timestamps/IDs); no new Publisher-format source or encoding was used.

### Owner correction: truth-file scoring condition (2026-10-18)

truth/*.json files may be used only to score imports. Never use them to infer how the format is encoded. Record any use in SOURCES.md. See `docs/CLEANROOM.md`.

### Tasks 3.4 / 3.5 closure on merged pub-import (2026-10-18)

Compared only our own clean-room `contents.rs`, `quill.rs` and `read.rs` bytes at `designcraft-pub-cleanroom@9a184ba` against merged `pub-import@9ed40942`; all match. Used the existing corpus scorer once with the 142 previously allowed pubsmith `.pub` fixture binaries and their corresponding Publisher `truth/*.json` layout exports at `0ccdf27`. Truth files were used for scoring imports only; no encoding was derived from them. No new external format source or reader code was consulted. Used the existing source-use and clean-room records to correct the lib.rs provenance comment, without changing reader logic.

### 4.x bookkeeping and dropped-formatting diagnostics (2026-10-18)

Used this repository's clean-room record and `text-store.md` §§11–13, `tables.md`, existing Quill diagnostic code, importer mappings and paragraph/character/table/shadow/workflow tests to audit task completion. No new encoding was derived. Only Quill warning construction changed: feature labels use already documented keys; unidentified fields remain numeric. Synthetic diagnostic cases use existing property encodings with deliberately unsupported kinds/values, not proposed numbering/tab/strikethrough encodings. Reused only the allowed `hello-nifty.pub` and the 142 previously allowed pubsmith `.pub` binary fixtures at `0ccdf27` for test execution. The unchanged corpus scorer reads their corresponding Publisher `truth/*.json` layout exports for scoring imports only; no encoding inference or fixture reverse engineering used truth files. No external reader, prohibited repository, pubsmith source or new specification text was opened. Numbering/custom tabs are referred back to a describer rather than investigated by the builder.

### Task 6.4 — batch half only (2026-10-18)

Inspected our own existing `apps/designcraft-cli/src/batch.rs` and batch tests, engine Open/Open Bytes/save/PDF commands, document item traversal and the previous local 6.4 report/artifact summary from designcraft-pub-cleanroom. That earlier report contains conversion results, not format-encoding evidence. Used this repository's `hello-nifty.pub` plus synthetic corrupt bytes for command/report/source-preservation tests; no reader logic or Publisher encoding was changed. The normal engine `file.openBytes` command receives the same bounded bytes as before. Reused the 142 allowed pubsmith binary fixtures at `0ccdf27` for batch checks. Their Publisher `truth/*.json` exports are used only by the unchanged import scorer to check the 3.4 totals; no truth data is used to infer encodings. No other reader, pubsmith source, prohibited repository or new specification text was consulted.

### Warning triage — docs only (2026-10-19)

Used the earlier normal-import batch report `.builder/batch64-converted/report.csv`, its local artifact summary and saved source-hash manifest, plus Git tree identity to establish that the report covers merged `pub-import@3829e1c9` (same tree as `f797a6aa`). Rechecked hashes of the 142 allowed binary fixtures at pubsmith `0ccdf27`. Counted distinct exact warning texts by affected file; no new sample inspection or encoding inference. Consulted only this repository's `docs/CLEANROOM.md` and format notes `text-store.md` §§11–14, `drawing-details.md`, `tables.md`, `contents-stream.md` §5 and the format coverage checklist for classification and known user-visible omissions. No Microsoft specification text, pubsmith source, forbidden repository or other reader was opened. The unchanged corpus scorer was run separately with the same allowed binaries and Publisher `truth/*.json` layout exports for scoring imports only; no truth data was used for triage, effects or encoding inference. No code or task status changed.

### Real-file robustness triage — Apache POI `.pub` samples (2026-10-19)

Task-scoped sample source: public repository **apache/poi**, branch **trunk**, folder **test-data/publisher/**, commit **bf7c9b6a1490046517cbd8e1504e702a89be4bc9**. License: **Apache-2.0**, as identified in the task/repository license description; no license file, README or documentation was fetched. Obtained the branch hash with `git ls-remote` and only directory path metadata from GitHub's JSON directory view (the API request was blocked). Downloaded individual raw files at the pinned commit: **only the 24 `.pub` files below**, including the three deliberately damaged clusterfuzz cases. No other repository file, `src/` code, `.txt` reader output, other `.pub` reader or its output was opened/read/downloaded; no clone was made.

Binary inputs are outside the checkout at `/workspace/sample-inputs/apache-poi-publisher-bf7c9b6a1490`; the locally generated download manifest is `/workspace/sample-inputs/apache-poi-publisher-manifest.json`. Each hash was calculated from the downloaded bytes and rechecked after conversion. No sample or CSV is committed. Paths below are relative to `apache/poi/test-data/publisher/`; all belong to the pinned `trunk` snapshot above.

| File | Bytes | SHA-256 |
|---|---:|---|
| `51318.pub` | 88576 | `3ab75a6a9196e0a51fc9b0aa759459501c71d313030c06652aabffbae0a2ab09` |
| `60685.pub` | 119296 | `0d3cecf0ff000a20931d0de3ebc48040bea10e03833ade3f9902f996c20e7184` |
| `LinkAt0And10.pub` | 64512 | `c59b7160c21a531e5872e76141c4cd43a4d74f30d78401f64175467fa40c44fc` |
| `LinkAt10.pub` | 64512 | `f1b7f65cb9646fddc2cfc77a795109bb958f629e4fbf7c9fc7a4d1b045074c35` |
| `LinkAt10And20And30.pub` | 64512 | `202ac6f82e2a33ef0181158728b5c4df6781ccb105e012f520d331acafbcebf1` |
| `LinkAt10And20And30And40.pub` | 64512 | `b54068d4f82dd0ee533f2a655a7edd2acbdc02619621b32fb23cb9ce842ed669` |
| `LinkAt10Longer.pub` | 64512 | `a917a9e575f5cfc1ba8e78226981b662b06f96da2d7f2eb3e2b00a4b4f3dada5` |
| `LinkAt20.pub` | 64512 | `cec79a2bd2f6bc71dd276bd6584c34d66fa6f710d9adbed951bd9cb32c5b2fed` |
| `Sample.pub` | 72192 | `6fefdef46b87c767150878dc384549cb2d2ec2ac54de25f8ddb3a5628301107e` |
| `Sample2.pub` | 72704 | `c87ea7cc5606023c5fa99376631bbd45a8e9bdaba34c958057995b0e7ac8cc86` |
| `Sample2000.pub` | 61440 | `40701ca47b26d04771cdd58467764e9ab39d3da69b59fbc529afd16d263d2f86` |
| `Sample2_2010.pub` | 73216 | `931ff20980b8f72aab227fa3cbe61b017be8d17c337fecb9aca3c97075940777` |
| `Sample3.pub` | 72192 | `424c69173ff08948c2529c8084b4ac2403f1ff1057146f4edd02fc29b44481fc` |
| `Sample3_2010.pub` | 72704 | `baa9555a72d9e4264e3fd722896e2e43b960d20b4afdb96f75cb721a6133ff91` |
| `Sample4.pub` | 72192 | `42195f7ad23d911219fea3ec88e66e867e9b9a6821a16dd1b535e2aa9d57a11b` |
| `Sample4_2010.pub` | 72704 | `544e2043f91aed664c0a09f9ae45da42291b6730f107f99f3cdf9462ed38fb02` |
| `Sample98.pub` | 61440 | `8912f295ff0cd1f55b873b3b98a182218f33c420877e025d2f0fd795f12dd0c0` |
| `SampleBrochure.pub` | 161792 | `ffed034ac87e679f0bd08ff9cf74ad11c0e0e510a42b1bc1a7502415f6c29c87` |
| `SampleNewsletter.pub` | 291840 | `6a825ba26ba35d6e885acdc62e859591ed37cb0ff7480b554b9cb362b644dfcf` |
| `Sample_2010.pub` | 72704 | `f0a90aa566d72eb822eb48b4f378c33007610e35f3f1b999cdb88f2c9683858b` |
| `Simple.pub` | 65536 | `2606f530052a818c7949b88c9bf245dbc0a74ddc6b5a60004f37b387fe4b955d` |
| `clusterfuzz-testcase-minimized-POIHPBFFuzzer-4701121678278656.pub` | 331491 | `07b48fcd8a3b85a83b054ea9722352d5269591d2482fb710002794da49f48b26` |
| `clusterfuzz-testcase-minimized-POIHPBFFuzzer-4918886059278336.pub` | 32832 | `2641d2d48a345f0eb52b4dd0151330142e3037505fea614c272ec7eba4b3e9b0` |
| `clusterfuzz-testcase-minimized-POIHPBFFuzzer-6325615354773504.pub` | 2042 | `e06b81d1a11a592d890e8d77c7f243c0529a36a0ec9bc4f18e8b1afb9e4f7bd2` |

Used only our existing DesignCraft batch converter and its CSV/diagnostics to report import/refusal/failure behavior. Compared exact warning strings with `docs/warning-triage.md`, and consulted existing `text-store.md` §§3, 13–14 and the clean-room record to avoid assigning meanings to unknown fields. PDF metadata/rasterization checks used only our generated PDFs; bounded native-archive checks used only our generated `document.json` to confirm reported counts. No sample version was inferred from its name; no encoding was inferred or investigated. No Microsoft specification text was needed. The separate unchanged corpus score reused only the existing 142 allowed pubsmith binaries and their Publisher `truth/*.json` layout exports **for scoring imports only**, not for Apache-sample triage, warning interpretation or format inference. No code, owner or task status changed.

### Table-dimension diagnosis — task-scoped eight samples (2026-10-19)

Reused only the eight named Apache POI binaries for numeric investigation: `Sample.pub`, `Sample2.pub`, `Sample3.pub`, `Sample4.pub`, `Sample2_2010.pub`, `Sample3_2010.pub`, `Sample4_2010.pub`, `Sample_2010.pub`, from `apache/poi` `test-data/publisher/`, `trunk` pinned at `bf7c9b6a1490046517cbd8e1504e702a89be4bc9`. Their SHA-256 values are in the preceding real-file source record and were rechecked against the existing local manifest. No new network download or other Apache repository access occurred. Extracted only their bounded Contents streams using the existing `cfb` container dependency and a local diagnostic helper; read the existing property-list/Contents/table notes and our own `contents.rs`, `read.rs`, `tables.rs` to reproduce the check on already documented numeric fields. No new field meaning was inferred. Compared the same arithmetic against only `table__2x2.pub`, `table__3x2.pub`, `table__merged.pub` at allowed pubsmith `0ccdf27`; all exact sums match.

As separately requested in step 4, reran the existing batch converter on all 24 previously downloaded Apache samples; this is execution-only reuse of the earlier corpus, not format analysis of the other 16 files. Compared the local before/after CSV rows and rechecked all source hashes. Consulted the previous real-file triage for baseline counts. Ran the unchanged 142-fixture corpus scorer using its allowed pubsmith binaries and Publisher `truth/*.json` exports **for scoring imports only**, never to diagnose dimensions or infer encodings. No other reader/output, pubsmith source or new specification text was used. Decision: case (c), no code/test change; the numeric observations do not replace missing describer rules.

### Cautious table rows — prerequisite/input gap (2026-10-19)

Read the existing table diagnosis, newly merged `table-review.md`, `drawing-details.md`, `contents-stream.md` and searched the full `docs/pub-format/` notes for a documented table-frame anchor mapping. Inspected our own retained `escher.rs` comments/anchor reading to locate the gap; its coordinate fields were not treated as documented format facts and no sample drawing bytes were interpreted through them. The required keys/coordinate semantics are absent from the allowed notes, so no drawn-height observation or new encoding rule was produced. Existing Contents observations for the eight Apache files and three pubsmith controls were cited from the diagnosis/review only. No new download, specification text, Apache repository content or other reader was accessed.

Executed the separately requested all-24 batch on the existing local Apache sample folder at pinned `bf7c9b6a1490046517cbd8e1504e702a89be4bc9`; compared the new CSV against the previous run and rechecked source hashes. No other-file format investigation was performed. Ran the existing real-table fixture assertion and full CLI/pub checks. Publisher `truth/*.json` exports at pubsmith `0ccdf27` were used only by the unchanged 142-fixture scoring regression, never to infer anchor or table encodings. No production code, test, task ownership/status or `real-file-triage.md` change was made. The missing-note request is under Needs you in `REPORTS.md` and `tables.md`.

### Drawn table-height observation — retained-code measurement (2026-10-19)

Used the current prompt's read-only retained-reader clarification, existing `tables.md`/`table-review.md`, and the already documented client-data Contents association. An uncommitted throwaway Rust harness compiled our existing `escher.rs` and its `read.rs` dependency read-only, called `escher::parse` directly, selected the returned table shape, and measured its existing rectangle output. Read-only cross-check: `Builder::rect` and existing `pt` conversion; returned rotations were zero in all 11 cases. No code in those files was edited and no new anchor key, unit or group-transform rule was derived or documented.

Inputs were only the three allowed pubsmith table binaries at `0ccdf27` and the eight task-named Apache POI binaries (`Sample.pub`, `Sample2.pub`, `Sample3.pub`, `Sample4.pub`, `Sample2_2010.pub`, `Sample3_2010.pub`, `Sample4_2010.pub`, `Sample_2010.pub`) at pinned `bf7c9b6a1490046517cbd8e1504e702a89be4bc9`, using the existing local copies/provenance. Apache hashes were rechecked against the recorded manifest. No network download, other Apache file, other reader/output or new specification text was accessed. The harness and numeric CSV are ignored local artifacts, not committed. Exact ending matches trigger the requested stop condition, so no code change or new all-24 batch was attempted; the previous CSV/report supplies explicitly labeled baseline counts only.

Ran fmt, CLI/pub clippy/tests and the existing real-table test. The separate unchanged 142-fixture corpus scorer used only its allowed pubsmith binaries and Publisher `truth/*.json` exports **for scoring imports only**, never for these height observations or encoding inference. No code, tests, task owners/status or real-file triage content changed.

### Stored row endings for native table positions (2026-10-19)

Used the current task's specific row-position instruction, `tables.md`'s exact drawn-height observation, `table-review.md`, and the documented table/property-list keys. No new field meaning or anchor/unit/transform rule was inferred. Reused only the eight task-named Apache table samples at pinned `bf7c9b6a1490046517cbd8e1504e702a89be4bc9` and three allowed pubsmith table controls at `0ccdf27` for row geometry checks; prior source records contain their hashes. Inspected only our own table parser/builder, native Table/RowHeightMode and composition interfaces to map stored positions to fixed native row heights. `contents.rs`, `quill.rs`, `read.rs` and `escher.rs` were not edited. Synthetic cases use the documented property encoding and observed 3188-per-row discrepancy, plus deliberately invalid counts/sizes/endings. The fixed native row mode is an implementation choice to preserve the requested positions, not a newly inferred Publisher flag.

As requested, executed the existing converter on all 24 previously downloaded Apache samples; the other 16 were used only for conversion/regression, not encoding investigation. Rechecked all source hashes, compared the existing CSV/native/PDF output baseline from our own converter, inspected generated native table counts/heights/cell text, and rasterized all new PDFs for nonblank checks and visual review. These are our own outputs, never another reader's output. No new download or other Apache repository content was accessed. The unchanged 142-fixture scorer used allowed pubsmith binaries and Publisher `truth/*.json` exports **only for scoring imports**, never for row positions or encoding inference. Updated real-file triage from the newly generated CSV; no task owner or checkbox changed. Text-association limitations were reported without an additional fix.

### Follow-up: retain the previous default row-height mode (2026-10-19)

Used the current task instruction and existing table-row implementation/tests and notes to remove the explicit native `Exactly` assignment, retaining the previous `AtLeast` default. Stored-ending heights, checks and warning are unchanged. Existing frame measurements establish configured geometry, not a Publisher fixed-height flag, so none is inferred. Reused only the existing local 24-file Apache sample corpus for the requested batch regression; no new download or other repository input. The separate 142-fixture scorer uses the same allowed pubsmith binaries and Publisher `truth/*.json` exports **only for scoring imports**, never to infer encoding or row mode. No change to task ownership or checkboxes, and no protected reader edits.

Follow-up validation also compared local before/after CSV rows and our generated native document JSON: only the 24 row modes (and timestamps) differ, with ending-derived heights/warnings unchanged. Per the user's reply, removed only the two fixture-test mode assertions in `crates/pub/tests/tables.rs`; all geometry/warning checks remain. No other reader output was used.
## Describer (data-only)

2026-10-19, branch `describer/anchors-and-fields` from `pub-import@deb7f09`. Task A wrote `pub-format/anchor-coordinates.md`; Task B wrote `pub-format/unknown-fields.md`.

**Inputs used (data only):**
- `docs/CLEANROOM.md` and the notes in `docs/pub-format/`.
- 142 `.pub` files from `FuncularLabs/pubsmith`, `tests/fixtures/pub/`, commit `0ccdf27fc3f381f0897515e226ea485165c699b2`, each downloaded one at a time from raw.githubusercontent.com into a folder outside the repository.
- 24 `.pub` files from `apache/poi`, `test-data/publisher/`, commit `bf7c9b6a1490046517cbd8e1504e702a89be4bc9`, downloaded the same way; all 24 SHA-256 values equal the earlier manifest above.
- The container reading was done with a general OLE/CFB library (MS-CFB) in a throwaway script outside the repo (not committed); all Escher, Contents and Quill walking followed the repository notes and generic record headers.

**Not used:** `truth/*.json`, golden PNGs, any `.txt` from Apache, any .pub reader or its documentation, any pubsmith or POI source, web search. The MS-ODRAW page was tried once with WebFetch and was unreachable (host name did not resolve); no specification text was obtained.

**Deviation to disclose:** the GitHub API and HTML listing were blocked (HTTP 403), so the 142 pubsmith **file names** were taken from a directory listing (`ls`, names only) of the existing local clone `/home/user/pubsmith` (checked out at the same commit `0ccdf27fc3f3`). No file in that clone was opened. The 24 Apache names came from the manifest already in this file.

**Files that could not be fully read:** `Sample2000.pub` and `Sample98.pub` have an empty Escher stream (Task A not applicable; Quill read normally); `clusterfuzz-testcase-minimized-POIHPBFFuzzer-4701121678278656.pub` has no Quill stream and the other two clusterfuzz files were unreadable by the container library or the documented Quill rules (Task B n/a). Task A used the pubsmith files and the readable Apache files.

### SHA-256 of every file used

pubsmith (`0ccdf27`):

| File | SHA-256 |
|---|---|
| `fill__fill-green.pub` | `140bc6d105976866a894c5ad869cae4c3d4e53792182cd8b95e72ca2356ef03e` |
| `fill__fill-grey.pub` | `f653a0279aed2777a32ecc4f0c07178116655d158a7c13ef8558e171fb1e72dd` |
| `fill__fill-orange.pub` | `679755392fd23b3e78abc496678c2a28af91fcb8b1fdd1fb05c53bcf07126432` |
| `fill__fill-red.pub` | `dd52ac497bedbe04d6b7d304fc27ef7769dbfb7c1a060c6f41b215857519f1b6` |
| `fill__fill-white.pub` | `b42326eeee9eb0a3213510b67f4795cdcabd49d75c090910f345e19b40f0fe61` |
| `fill__gradient.pub` | `3de0b9e1abc3100335cd6d021b6c15d80f4a04731db883adde2148ae39a7af29` |
| `fill__no-fill.pub` | `29a1a6f2b432b288bf98cd2b00eae4c91ca5a84feb8d6d74bbf78fe02fb23e05` |
| `fill__scheme-accent1.pub` | `402ececd0e5ad6cf320eda56cc673bd92d553dc5c5c94110ec91a27b86baa4a2` |
| `fill__transparency-50.pub` | `9f39d034e94c082c605e74cde10b32fe5ec6f8a96ceb0d15d96e542686d6d449` |
| `geometry__base.pub` | `1e3d6873946014b99d13fc8e47ee9f3c9c8ef368bfa0b754a725e023e4d4c34d` |
| `geometry__flip-h.pub` | `f7dd8434332adac8a67fdf4e253cd104b4d8b3ad16f3448abd501a59503a9f9f` |
| `geometry__flip-v.pub` | `d2d4da9aa9ba00615f42c1a27e0639fc59af0793b5bbaf7442e48b442b7192fe` |
| `geometry__height-10.pub` | `da1a0124946404d017ff51e4ed21d2bab0614af75b01403036cd4489e15b057a` |
| `geometry__height-300.pub` | `d5c4203342c1538cf38009a85691542439844b3cbdb137a8c314ccaa911ed73d` |
| `geometry__left-0.pub` | `2d003f7cb246a601a1224975d1aa94d6ea0e81d29b24f394e3ced2ce0086ca45` |
| `geometry__left-1.pub` | `fea0686a4ecfaadc6ceef84037cd356c5a493a20759700b344c0d6e9df7be186` |
| `geometry__left-100.pub` | `7eca9f50be551171e99e6cd7314e5927f41092db5d676504da87131f7937ed76` |
| `geometry__left-306.pub` | `fc55f9528ee597e14e9b90831443e7bb2975c7d7fd52b5bc150d50b122f21a4b` |
| `geometry__left-500.pub` | `97179df47ce218e0be341a88668445958bf4f11439a13caa1dc3bc20e3a2dbcb` |
| `geometry__left-73.pub` | `c9dce9f6ded024cdeb5dbba9d6497c63274f28d69f4f236cce7faed4cc98eda3` |
| `geometry__rot-1.pub` | `6ae4fcd65f3a40e0a860f2fc946a862df2f939e47ba516bb972471c15ead0310` |
| `geometry__rot-180.pub` | `517e9c739f56672e6486f71a4e53768d0ff15ab5ea13cdf7a2cde40f46b471e4` |
| `geometry__rot-270.pub` | `d719acb601ad60c9eb3eaacadc3a9fc562e9b96be3a77d25a45a2397b7657c09` |
| `geometry__rot-30.pub` | `d4b4fa2c4db0d6533d9fddc722af0024fc39c30b3c2d7b76e7dbad67d30e2168` |
| `geometry__rot-359.pub` | `772bb25fbfd595bb7829f798d3caf52cb53ec6a8645ee4026c830cdd982e44bb` |
| `geometry__rot-90.pub` | `e360634ee3df510142dffb699c4a60a8bfe50c1cbd3a27b01682b5f3a0799a87` |
| `geometry__top-0.pub` | `c846a7e17cd34d20ddfc9b6e95e11d6eff1d4472d6a80cdc6e76def8375ceb86` |
| `geometry__top-396.pub` | `f2dec8a67177808512b34d4c908fd6f1e83b3a4fd75cbeb290163d6db44b5b68` |
| `geometry__top-700.pub` | `91e4f802c8cf3de112add7e5b3bae2506edb5d511b81a0c5701fd044226cabf5` |
| `geometry__top-74.pub` | `850dd82b85c9382843ba2037348ca416ae1d773035e09ab4f55eb1c719519f4e` |
| `geometry__two-rects-swapped.pub` | `a952da95e108fa3697ecceb4ee0936f7b3a2b780f4eff751eebf18b88e17231d` |
| `geometry__two-rects.pub` | `834cb91a54880211143b29a63ee61297a05bdc506dedd316b211195f5a0a2f97` |
| `geometry__width-1.pub` | `1fbe3169b7145be8952708929d767e607fb530d7e4a48dd7f6104d8c2f3b4f41` |
| `geometry__width-150.pub` | `5a1d8d731ef00e6861242764cd836f656242bcfd3022ae39f76786504f0628a0` |
| `geometry__width-612.pub` | `5a081f6b636c97fa1135aee399ba9376bce552fedd9b2dbdf6e0a53a46f0f1ec` |
| `group__rotated.pub` | `aea7f79db246dd5c34269108b1dab8b29c5541c6b1c4523d80b0d3630b160ce6` |
| `group__two.pub` | `d7cecf37f5c0325a8dc1187df424c30bfa8525bb3098e0b4a06790f394af5159` |
| `line__color-red.pub` | `f1e9fe2a07f69f9b8f777f9c5914716d713a08c9164f6b19f08cfd57e2a45980` |
| `line__dash-2.pub` | `52c2be877b0552c81fac34fa81a62f2bf4eff5c9edf2d8d67fb77f801f07f1fb` |
| `line__dash-4.pub` | `ddb223c5e4275ae92867468a51fee8a4e6bd41e1812dd2241eba040c761a2a6e` |
| `line__dash-5.pub` | `9885f150d7e4eb7955fbdf62543e7e9561a0447cd429328ee8684a2ea0d25129` |
| `line__no-line.pub` | `bee2971fe48138d329be877f94dbc7675c6e9aab59cf641b12a88bb567bcd9c7` |
| `line__weight-0.25.pub` | `af8ef6244d774c69996635b7947b3482ef4c4a1871c2042e2eb139ac7e03b96b` |
| `line__weight-1.pub` | `91aa8bee66da5d13efeb385ec69f15303b0afaf9acfc1a95dce7abcd7d15ba90` |
| `line__weight-10.pub` | `dd194825abb2c45cb7ca9b81f00dc6c11c58597d8bc4e139dcd0ed13edd737dd` |
| `line__weight-4.pub` | `c3c1a3f1aa72573a783c6b9c8b250e5fb63ab77349ba729b3de90485aa9a2233` |
| `pages__landscape.pub` | `22ad856c3b61fc601b1895bb72c8e921ed0330bb553ab6c8a720b599c15aac3d` |
| `pages__master-ignored.pub` | `92409f24827f069f1503c82469cabbe911d697b312b0ded4bfd66607dabc67eb` |
| `pages__master-rect.pub` | `400c061402e471bdede31c8f4d9db4054693233f098c7ded426988f9ef89c218` |
| `pages__master-text.pub` | `8137a8885c89f9f06bf922f532dba66246ca955922b7ccf0c4bdc54733b1479f` |
| `pages__one.pub` | `18905060eb45f6aede59991550ce688ce9cf1e5d03beb7f16ea3fadb979551fc` |
| `pages__size-5.75x4.75.pub` | `abd024933717e4806054f99b66bb9ddc8d5eac7a7c51803281c916a2ff97ee2c` |
| `pages__three-rect-p3.pub` | `3c15cd70034b50a63831a2d248acc9d06621f241c494439d8a07a233e5556459` |
| `pages__two-rect-p2.pub` | `bf5d98cb2082e73a99a53a797ffc07353c432ed22a8ea943ff9930d8aba04284` |
| `pages__two.pub` | `815fad77c79fa4f88fbde52b4001bc5ef2097d5f4751965518447122ae03ef5f` |
| `picture__base.pub` | `39a32b2e6ce8073fbfe9a2d66cb9ab8b4e0d0952bb294a06ac609a3749f071b3` |
| `picture__border.pub` | `97907db68b160cd9d9c87eb31f39ad1474cb9d524df6988d51cac82c3a6adb9f` |
| `picture__crop-all.pub` | `a8201d751ce462e5946a0813465f5fe61b38eb25227c4afdf600fc1c6f010012` |
| `picture__crop-left.pub` | `49ae8c6529cc2a725677f3ec191b2be0e2bcad86d68eaa7887fccac94487aa31` |
| `picture__crop-top.pub` | `2db5fd28b05640c1e4ccc8d5826dcd560b166feba4d5d8ab96bfa7dcb25d2ebf` |
| `picture__flip-h.pub` | `3637e89b38d233a73f0c4b5e1b28e93ff4aa5dcce96ad2ecaf7f0714340405ca` |
| `picture__moved.pub` | `ec1964648b15eb40b6e0cae9ca2118173343eaac091d3232f24cd655768127f8` |
| `picture__oval-mask.pub` | `0ef974b5861894d463493aed878f2e36f8147369a1ae10593386f8e77a04a0a8` |
| `picture__rotated-30.pub` | `15bdddc73b1b3866207739538fe4c6596e7cbd850aedf7ed3bbad0719738e55f` |
| `picture__stretched.pub` | `bb619e30fb9b49619b45ceed21daa9cb9c505fd4fbb791ce44b02428308f077d` |
| `picture__two.pub` | `97dcd67cc415bd962589122fe39f95d32304d2733005c54c5fdc711273007d3d` |
| `shadow__offset.pub` | `2b8c3cd8478da53e364097b92e2978b8c6f9037a0b576dce84060dd98684eaad` |
| `shadow__on.pub` | `b01c8e35d89bcccc1d1e13e85be2c48eb82128c8a88d36adbfd4503f54f8d209` |
| `shapes__curve.pub` | `a932e1e445dbf5e088e67bcb6c2344f8ef78f7cb2c140c99d39d13f79adf6501` |
| `shapes__diamond.pub` | `b5439878dd52c0a54214a92bb731e415ac91ca3083a9e711e42d92439817d6af` |
| `shapes__heart.pub` | `15e55be8782f648de3f48960f97ceac0200a55d80efaa015fa6bdbeac87ae9f5` |
| `shapes__hexagon.pub` | `58981e80239daa7a147484d5d05dad17c55e4f51260b19b403a61f335a03c2a8` |
| `shapes__line-arrow.pub` | `b727ec743dc1a7c57eb46103673043afa51fea8451d85d9b95d30968488f27a1` |
| `shapes__line.pub` | `c02ab6f95f190fb8d1923e7e9965b19f23d77b9e79cfa157944b736047f8bbe0` |
| `shapes__oval.pub` | `29cb3020390b751a5c66767688be231a2f7d77a1776d0b2c0060fbdc89f64fdb` |
| `shapes__polyline.pub` | `750d1261225acaf73a47d8987118626a39975cc53ce791f5961955757ad601a6` |
| `shapes__rect.pub` | `9f533e45948ddc0be2f7be85c97d774766ce36e2d57a1209ef8bb9fbde6c8513` |
| `shapes__rightarrow.pub` | `1fd8cdabecd5c9345f9f2a00eeeede7f126c68e20809dea7b3a2423e8a4e5b29` |
| `shapes__roundrect.pub` | `6e5b1b4c63a0ee226ca9a8c6c1c863be5481854bb64baf88ed03d2e53c0e1e33` |
| `shapes__star5.pub` | `b64f6a7ef259287a8b35ef97c8d40ccb425eb55b54953b3255798cc82a5a95f2` |
| `shapes__triangle.pub` | `b6c270e0c709cbfe90b2270c6afc10d17499dfd4d15feca0fcd16dc00d3a8616` |
| `table__2x2.pub` | `72d2127f4ffdbe998f4f94414a28584e49af817991b70a564fda885dcb1a6a7d` |
| `table__3x2.pub` | `12c45cd9057553f748d0cf23ab5348d901be294b1d6e790eb80c3331d57d2616` |
| `table__merged.pub` | `3df51f5e3e29f247b8636627725d43f5973dc7ea887338f32aec8f4b49a23b30` |
| `text2__base.pub` | `7d32fbec55d474c67d7a344db4fff17cb1f9216cdda6d736ab2c8424203662b6` |
| `text2__p2-bold.pub` | `36baee97f339fcdbde70984d9c4bc851844948375a95601d35e75c9fdca43e50` |
| `text2__p2-center.pub` | `6d7a87753090be349d6eec1a1196f9f6b92776073acc2adc6661f4dadf49d1d3` |
| `text2__text-in-oval.pub` | `270e0d486423e8a869cb01f72487771ef6fae1e650632ed995d656b29473def2` |
| `text2__two-boxes.pub` | `a6405f9b31de04d69a3fdd4179fe97cbfc30e629a8c874e40da80cceaeba0d48` |
| `text__align-center.pub` | `226cf7e6918146193e531520b9005e06f434b8f62ceaf700b455bc8ac4fb3713` |
| `text__align-justify.pub` | `c5442fe547575f882d51809a315a6916a7233895cafe849e20e09e8e9409b739` |
| `text__align-right.pub` | `d1aa1cacbbb272f8b6456c79cf1e024218e15bb40e0f979a4ee53dbf0ea1f969` |
| `text__allcaps.pub` | `f95eb0920c19eafd027884bb33ed31389d4aad91094755df1bb9f777a4dc8a25` |
| `text__base.pub` | `0c9293e727559fe58ba150e8445b1365b4960883fbe6759c10b2477f128ff9a7` |
| `text__bold.pub` | `90a140c2db6c6c8399c848a26a4e7c9ce822d52d219f5589ee026345c2e05635` |
| `text__bullets.pub` | `51df14aa94d640932656bc49a3f645b97333ba811467761e31226ec1547011f5` |
| `text__color-custom.pub` | `62436393aa82998736c87930793ca8d019162d8fadcfc67700c36c6db5d96c6f` |
| `text__color-red.pub` | `8402349638c905016b5d56ee2d4356c906015631b371985378e676d8b6c4ad0a` |
| `text__font-Arial.pub` | `9249dd72feeab344a9a5b2d366c4b6973a90843b8090cd8d8831696cdce6295d` |
| `text__font-ArialBlack.pub` | `024b15dc2154c4779796e0ea748fdff763bf065f1750cd938eeb07312461e42f` |
| `text__font-CourierNew.pub` | `d8f715a6f23a8bf396ccb65da270c62e52a23fc1f26465421d0c413d730010a2` |
| `text__font-TimesNewRoman.pub` | `5bd50ace94c87c2962ce97221fffb5ebfcd2c56f46808e3aa275fbcd4b06b77d` |
| `text__frame-fill-line.pub` | `59a0bfa1e5642f9164fb7d4b7557e2c080ddc7abfe76276506d21396951500a3` |
| `text__indent-first-18.pub` | `2a4a4f3b861c9514d12105312a86e94c6b310545151fe9c238b32e4870035634` |
| `text__indent-left-18.pub` | `fe22b3b44baa690da104712645f606a755b9c626bdb8b5b7b6fc5d6ef4abb5c9` |
| `text__indent-right-18.pub` | `91fa9d4f838c8909ece6d16ba5c98d5b1bee4f6a39484ffd95bc8b4f17ce332d` |
| `text__italic.pub` | `60216ffe5ee57f80c546bd99724086c9693db682e11d3ad60e2ae2ec31eb167e` |
| `text__line-break.pub` | `ac822b60ab7365ab82a3d2a8bbd9edc5f9061407d111749c5942b73fc53a125c` |
| `text__ls-double.pub` | `9242fc72b4ff5ab82745969006895f6d7a085d190820b2c28a62d04fdb3980a4` |
| `text__ls-exact-14.pub` | `22191bec6bd65bbc214760d982e1b44091ec9c11d8ecd3076ac6bf8d5f58d188` |
| `text__ls-exact-30.pub` | `69067437491eec0df68a82f812177713207942246444cfbd8db040db8e6a767d` |
| `text__ls-multiple-1.5.pub` | `35e0d9d58aab018b259a4a3cd95f05de3629a56e13c5a69109f06a7a6418313f` |
| `text__ls-single.pub` | `e8957ec0950137eb616288ba1dd7a882798eef128786c0610aaba88e592907d3` |
| `text__margins-10.pub` | `4708cc8ef89dc69507b1ccb0fc1ef13bd837592b366b1d3507a7bc85fe81796a` |
| `text__rotated-270.pub` | `ceae546e1c875acf4349a577263e76242ec76805d08211dc6c550f2a0f84427f` |
| `text__run-bold-word.pub` | `94d1838e1668d6c0a090e1841944e2295f5229c0a927dcdf5210117f36bc0063` |
| `text__run-font-word.pub` | `8bec2c8a3d35ff8a3d3b89cbb48b51f24337e33ae34486186bb4163eff523cdb` |
| `text__run-red-word.pub` | `6ab61f70310cb4d2adedfe0d670c11ae8ffadf037874536748efa9e1f15f924e` |
| `text__run-size-word.pub` | `089e2a4dd7439549dde6d58e9240c7d5342e201a7edf6ab96ea2e214b0f0ecc0` |
| `text__size-12.pub` | `78aba312d290c8fa8c29a93de07e611689000df7924eb8988af35dbb51e25a89` |
| `text__size-24.pub` | `80f027a7aca4cf537134bed35eec5ad812794518033541f809b85b24d847df16` |
| `text__size-36.5.pub` | `31f4aefa4307098e57a47ff4db35fb8b797d1c71dadf6e7e27ee938c892b03cc` |
| `text__size-72.pub` | `7967783a2a96012febe6b9a7c7222c4c24bca1f02170bdc56f9485617ea7ecfe` |
| `text__size-8.pub` | `f86bbf9bfac4e67cf34f5ded0cbb5d50fded0fd2634b141f41cbce02cbec7b25` |
| `text__smallcaps.pub` | `76ea2fea4a7885054bc1f013971d2fc3acdf7f3c012541520451ddda304eb6c1` |
| `text__space-after-0.pub` | `da8a9b5fb487c0acff844a5189fb5362b0da966f330c841f50c8db5007b5c2f8` |
| `text__space-before-12.pub` | `260f23e741dc2b29fccd4203c1639c07ef5fbbfc50f75351a239721c93da973a` |
| `text__superscript.pub` | `ae1f26ebbc06c7f8904d18ac0e559076256f3f8b9e0a02e11c49e96fca4e6604` |
| `text__text-changed.pub` | `21fe1e79b4da37a0cefd87cd494c5bb977073a39be883f38dba068f7a403c200` |
| `text__text-longer.pub` | `7fe759e9f7d513e01332717020ca662338bce1251a2d16f28e155c484b96afe9` |
| `text__tracking-200.pub` | `f9bc7d51af14bdd8b5eb289225a79738e39b677e74b4f7dcc36411dcc6de2664` |
| `text__underline.pub` | `3a4baac97c143a1ae519e4c149fe8df0ce0fec74119d1c1f6109238b116d5f5d` |
| `text__vanchor-bottom.pub` | `886be8dbfa7d2c2590ea408f4300a2014823da7585652419badd0ea207d2060e` |
| `text__vanchor-middle.pub` | `f92f73a3e92f700bc658a4438a1e30fa692bcc1370bed3570484d808b6299603` |
| `wordart__arch.pub` | `dc22fb9983e9119c9969e37dd5ba3d19fb63fa489c8ffeec4e38ea921a41904b` |
| `wordart__base.pub` | `270d8fe5d1a2286622b35e40b39e00eac17b878734aedb755059b3fc20050838` |
| `wordart__circle.pub` | `bc145b67097b5d9d42f0f5c838d68f2d88130267b51031bdeacd4cb9d0590eeb` |
| `wordart__fill-red.pub` | `e6745af79ae3d98bf3b79e3104e291f158934da6a8969dd722e24d0eecd9e6f4` |
| `wordart__font.pub` | `1c19cda5f6a40f1cb6f64f3b67a334ff54656bce573be480083c7bdc1bb0fd8a` |
| `wordart__line-white.pub` | `5968f890c0b4029dde31c644cb3ff29eb071225e685ea4fc4a527058f675755e` |
| `wordart__shadow.pub` | `d24ea44009d4e8c00860a61aec2391cbb3e30fd791deb5f5f480382218cb102d` |
| `wordart__text.pub` | `f3d6d544b2e47af20f49c20dbe9151e9c41eb53fbf655a685f8c4cb4cb46d62e` |

Apache POI (`bf7c9b6a`):

| File | SHA-256 |
|---|---|
| `51318.pub` | `3ab75a6a9196e0a51fc9b0aa759459501c71d313030c06652aabffbae0a2ab09` |
| `60685.pub` | `0d3cecf0ff000a20931d0de3ebc48040bea10e03833ade3f9902f996c20e7184` |
| `LinkAt0And10.pub` | `c59b7160c21a531e5872e76141c4cd43a4d74f30d78401f64175467fa40c44fc` |
| `LinkAt10.pub` | `f1b7f65cb9646fddc2cfc77a795109bb958f629e4fbf7c9fc7a4d1b045074c35` |
| `LinkAt10And20And30.pub` | `202ac6f82e2a33ef0181158728b5c4df6781ccb105e012f520d331acafbcebf1` |
| `LinkAt10And20And30And40.pub` | `b54068d4f82dd0ee533f2a655a7edd2acbdc02619621b32fb23cb9ce842ed669` |
| `LinkAt10Longer.pub` | `a917a9e575f5cfc1ba8e78226981b662b06f96da2d7f2eb3e2b00a4b4f3dada5` |
| `LinkAt20.pub` | `cec79a2bd2f6bc71dd276bd6584c34d66fa6f710d9adbed951bd9cb32c5b2fed` |
| `Sample.pub` | `6fefdef46b87c767150878dc384549cb2d2ec2ac54de25f8ddb3a5628301107e` |
| `Sample2.pub` | `c87ea7cc5606023c5fa99376631bbd45a8e9bdaba34c958057995b0e7ac8cc86` |
| `Sample2000.pub` | `40701ca47b26d04771cdd58467764e9ab39d3da69b59fbc529afd16d263d2f86` |
| `Sample2_2010.pub` | `931ff20980b8f72aab227fa3cbe61b017be8d17c337fecb9aca3c97075940777` |
| `Sample3.pub` | `424c69173ff08948c2529c8084b4ac2403f1ff1057146f4edd02fc29b44481fc` |
| `Sample3_2010.pub` | `baa9555a72d9e4264e3fd722896e2e43b960d20b4afdb96f75cb721a6133ff91` |
| `Sample4.pub` | `42195f7ad23d911219fea3ec88e66e867e9b9a6821a16dd1b535e2aa9d57a11b` |
| `Sample4_2010.pub` | `544e2043f91aed664c0a09f9ae45da42291b6730f107f99f3cdf9462ed38fb02` |
| `Sample98.pub` | `8912f295ff0cd1f55b873b3b98a182218f33c420877e025d2f0fd795f12dd0c0` |
| `SampleBrochure.pub` | `ffed034ac87e679f0bd08ff9cf74ad11c0e0e510a42b1bc1a7502415f6c29c87` |
| `SampleNewsletter.pub` | `6a825ba26ba35d6e885acdc62e859591ed37cb0ff7480b554b9cb362b644dfcf` |
| `Sample_2010.pub` | `f0a90aa566d72eb822eb48b4f378c33007610e35f3f1b999cdb88f2c9683858b` |
| `Simple.pub` | `2606f530052a818c7949b88c9bf245dbc0a74ddc6b5a60004f37b387fe4b955d` |
| `clusterfuzz-testcase-minimized-POIHPBFFuzzer-4701121678278656.pub` | `07b48fcd8a3b85a83b054ea9722352d5269591d2482fb710002794da49f48b26` |
| `clusterfuzz-testcase-minimized-POIHPBFFuzzer-4918886059278336.pub` | `2641d2d48a345f0eb52b4dd0151330142e3037505fea614c272ec7eba4b3e9b0` |
| `clusterfuzz-testcase-minimized-POIHPBFFuzzer-6325615354773504.pub` | `e06b81d1a11a592d890e8d77c7f243c0529a36a0ec9bc4f18e8b1afb9e4f7bd2` |

### Builder (Codex): queue C1 (2026-10-19)

Verified the existing default AtLeast row mode and absence of Exactly assertions on the fallback base; stored-ending heights, checks and warning remain unchanged. No code change needed. Validation used the existing 24 pinned Apache POI .pub inputs and 142 allowed pubsmith fixtures; truth/*.json was used only by the scoring corpus, never to infer encodings. No additional external inputs.

### Builder (Codex): queue C2 (2026-10-19)

The rule is documented in unknown-fields.md, but implementing same-record comparisons requires quill.rs, which C2 does not name. The standing protected-file rule therefore blocks implementation. Please name quill.rs in this queue item; no code was changed. Validation used the existing 24 pinned Apache POI .pub inputs and 142 allowed pubsmith fixtures; truth/*.json was used only by the scoring corpus, never to infer encodings. No additional external inputs.

### Builder (Codex): queue C3 (2026-10-19)

The alignment-6 observation is documented in unknown-fields.md, but suppressing the same-record 0x3C warning requires quill.rs, which C3 does not name. Please name quill.rs in this queue item to satisfy the standing protected-file rule. No code was changed. Validation used the existing 24 pinned Apache POI .pub inputs and 142 allowed pubsmith fixtures; truth/*.json was used only by the scoring corpus, never to infer encodings. No additional external inputs.

### Builder (Codex): queue C4 (2026-10-19)

Audited all 152 truth scoring elements: all 111 evaluated frames pass; the other 41 are image exports excluded by the scorer, not measured position misses. Listed every excluded element as other / UNKNOWN in real-file-triage.md with the reference needed to settle it. Existing page-centre, EMU and child-space code matches the settled anchor notes; no position change justified. Validation used the existing 24 pinned Apache POI .pub inputs and 142 allowed pubsmith fixtures; truth/*.json was used only by the scoring corpus, never to infer encodings. No additional external inputs.

### Builder (Codex): queue C5 (2026-10-19)

Added an ignored CLI batch integration test enabled by PUB_REAL_CORPUS, requiring an external sample folder. Its explicit run passed for all 24 files and asserted 0 OK, 19 warnings, 2 refusals and 3 failures. Normal suites: 67 passed, 10 ignored; the new ignored test separately passed. No samples committed. Validation used the existing 24 pinned Apache POI .pub inputs and 142 allowed pubsmith fixtures; truth/*.json was used only by the scoring corpus, never to infer encodings. No additional external inputs.

### Builder (Codex): queue C6 (2026-10-19)

Refreshed real-file and warning triage from fresh 24-file and 142-file batches, recorded sources, and retained C2/C3 as BLOCKED. Controlled batch: 79 OK / 63 warnings / 0 failed; real batch and corpus unchanged. No production code changed in this queue. Validation used the existing 24 pinned Apache POI .pub inputs and 142 allowed pubsmith fixtures; truth/*.json was used only by the scoring corpus, never to infer encodings. No additional external inputs.

Queue source detail: used `unknown-fields.md` for C2/C3's documented same-record
observations, `anchor-coordinates.md` for C4's settled/unsettled placement rules,
and existing table/default-mode records for C1. Inspected our own
`corpus.rs::score`, `Builder::rect`, `pt` and `escher::map` read-only for the
scoring/implementation audit; only the CLI batch test was added. C4 additionally
counted truth JSON element types and recorded fixture/page/element indices as a
scoring audit only; no coordinates or encoding rules were derived from them.
Final sources reuse pubsmith fixtures at `0ccdf27` and Apache POI `.pub` inputs
at `bf7c9b6a1490046517cbd8e1504e702a89be4bc9`, with the previously recorded hashes.
No external repository code, reader output, or additional sample was accessed.

### Builder (Codex): alpha package and UI/print audit (2026-10-22)

Used this repository's packaging templates, engine links, UI command/status handling,
DocState display-title accessor, print command and preflight code. Inspected the
already pinned `craft-print` dependency at `storytold/craft-libs@d991bf6792f8ade99c51358bf3c1d902c8cd96b5`
(`crates/print/src/lib.rs` and `windows_print.ps1`) only for print-job naming and
resource/retention handling. The script calls Print and disposes PrintDocument in
finally; no explicit retain/pause/hold operation was found. Actual Windows queue
behavior was not tested here. The RGB-swatch preflight check is unchanged from the
initial source export (`d82d525`) and was left unchanged. No Publisher encoding
or external format source was consulted, and no truth JSON was used. Validation
used the existing repository tests and synthetic command/package regressions.

### Builder (Codex): test maintenance check (2026-10-23)

Used only this repository's original importer/packaging tests and the previously
recorded inputs: the 142 controlled `.pub` fixtures at `0ccdf27`, and the same 24
Apache binaries at `bf7c9b6a1490046517cbd8e1504e702a89be4bc9`, outside the checkout.
All 24 hashes still match `.github/test-data/publisher-fixtures.csv`. Publisher
`truth/*.json` files were used by the existing corpus test **for scoring only**;
no encoding was inferred from them and no new format source was consulted.
Before/after scores: **142/142 imports, 152/152 objects, 111/111 scored shape/text positions within 1 pt, 52/52 exact texts, 111/111 styles**. Both real batches: **0 OK / 19 warnings /
2 refused / 3 damaged**, 31 pages and 280 objects. All 19 native document payloads
and assets, PDF drawing streams/text and all 31 page pixel buffers match;
only normal document/PDF creation/modification metadata was excluded. The
production CLI executable is byte-identical. The full suite reports the same
2,940 passed / 0 failed / 19 ignored; all seven CI steps and the packaging tests
pass. Changes are limited to test-only edits and this
source-use record; no runtime or protected-reader code changed.

### Builder (Codex): alpha.2 preparation (2026-10-23)

Reused the already recorded 142 controlled binary fixtures at `0ccdf27` and the
24 Apache binaries at `bf7c9b6a1490046517cbd8e1504e702a89be4bc9`, outside the
checkout, for regression checks. Publisher `truth/*.json` data is used by the
existing corpus runner for scoring only, never to infer encodings. No new
Publisher format source was consulted; reader logic and importer test text are
unchanged.

Ported only the owner's desktop-demo automation from
`srt8hellcat2-ui/designcraft-pub` PR #26
(`9dcbfdbf7fc49f06b59854f298fa557874dc9b70`) and PR #27
(`a408df8acaf95ca9e49ca35eb677bbf4a2a78680`):
`.github/workflows/windows-desktop-demo.yml`, `packaging/windows_desktop_demo.py`,
`packaging/test_windows_desktop_demo.py` and
`packaging/stage_windows_desktop_evidence.ps1`. The final PR #27 versions include
the earlier evidence-retention changes; the public workflow's copyright line
was retained. These are our existing automation, not a format source. Local
helper tests do not establish Windows desktop, graphics or spooler behavior;
no hosted workflow was dispatched.

Validation: all seven `cargo xtask ci` steps passed, including 2,947 tests and
wasm checks. Packaging: 33 tests, 3 platform-specific skips; actionlint passed
for the three changed workflows. Corpus scores remain **142/142, 152/152,
111/111 scored positions, 52/52, 111/111**. The real batch remains **9 with notes / 10 with
warnings / 2 refused / 3 damaged**, **31 pages / 280 objects**. All 48 expected
cell texts occur in the eight table PDFs. The local CLI reports
`0.9.0-nifty-alpha.2`; Windows execution was not tested locally.

## Builder (Codex): 2026-10-23 — DesignCraft 0.12.0 base

Used the public DesignCraft source delta from `storytold/designcraft`
`v0.9.0` (`7cb8b12507def48fb9346e5004340b5e8e4cfcb7`) to `v0.12.0`
(`5e8ef678321cfdd2d18927e2af94d58e5f9d8836`), applied by a three-way patch
because this export does not share upstream history. Kept our importer integration,
branding and packaging. No Publisher reader logic or importer tests were changed;
no new Publisher encoding was inferred.

Compared the alpha.2 build against the upgraded build using the same 24 pinned
Apache inputs already recorded here (all manifest SHA-256 values checked), plus
`hello-nifty.pub` and `wmf-picture.pub`. Apache inputs and generated real-file outputs
stayed outside the checkout. All 21 successful documents retained their document
content and asset bytes, with identical PDF text and page pixels across 33 pages
in both PDF rasterization and our headless renderer. In these comparisons, only
document creation times, PDF creation/modification times and the native generator
version differed.
All 21 alpha.2 documents loaded, saved and reopened in fresh engine sessions
without content or asset changes; the native format version remained 2.

Used the existing controlled 142-fixture corpus and its Publisher `truth/*.json`
layout exports for scoring only, never for inferring encodings. Scores remained
142/142 imported, 152/152 objects, 111/111 scored shape/text positions within 1 pt, 52/52 exact texts
and 111/111 styles. The Apache batch remained 9 with notes / 10 with warnings /
2 refused / 3 damaged, 31 pages and 280 objects; all 48 expected table cell texts
were found in the eight exported PDFs. Report warnings, notes and refusal reasons
matched the alpha.2 baseline exactly after normalizing output folder paths.

All seven `cargo xtask ci` gates passed: 3,058 tests passed, 19 optional/manual
tests ignored; assets, layers and wasm checks passed. Packaging had 32 passing
tests and two Windows-only skips; actionlint and the built Linux CLI's existing
package string guard passed. The catalogue check initially found six unused
French descriptions for upstream About content absent from our About screen;
removing those entries kept our About text intact and made the check pass.
Windows execution and printing were not tested for this upgrade.

## Builder — 2026-10-24 C7 baseline checks

- Read this repository's binding rules, clean-room record and odraw-geometry.md. No geometry encoding conclusions were derived in this attempt.
- Used only the 142 controlled .pub data files at FuncularLabs/pubsmith commit `0ccdf27fc3f381f0897515e226ea485165c699b2`, `tests/fixtures/pub`, and the 24 listed Apache .pub data files at apache/poi commit `bf7c9b6a1490046517cbd8e1504e702a89be4bc9`, `test-data/publisher`. Local folders: `/tmp/niftydata/pubsmith-fixtures` and `/tmp/niftydata/apache-pub`; all 166 SHA-256 values match .github/test-data/publisher-fixtures.csv. No source code from either repository was read.
- Ran the controlled import report and CLI real-file batch on unchanged main `d0a93ae9`. The controlled folder has no truth exports: no truth files were used, and layout comparisons were not verified. Real batch: 9 converted with notes, 10 with warnings, 2 refused, 3 damaged rejected; 31 pages, 280 objects. Stopped at the task's baseline condition without implementation.

## Builder — 2026-10-24 pinned layout scoring data

Downloaded only JSON scoring data from `https://github.com/FuncularLabs/pubsmith/tree/0ccdf27fc3f381f0897515e226ea485165c699b2/tests/fixtures/pub/truth` via individual raw URLs at that exact commit. The fixture directory listing contains 142 .pub files plus the golden and truth directories; golden images were not downloaded. No reader source was opened. The gate in crates/pub/tests/corpus.rs expects `<PUB_CORPUS folder>/truth/<fixture basename>.json`. Files are stored outside the checkout at `/tmp/niftydata/pubsmith-fixtures/truth/` and used only to score imports, never to infer path storage. The fixture CSV lists .pub hashes only, so the downloaded scoring-data SHA-256 values follow.

| Scoring data file | Bytes | SHA-256 |
| --- | ---: | --- |
| `fill__fill-green.json` | 513 | `abacf3c8dc9741d49e8f683d5803a9a3836b502ce1b94f2c5235a8b98cfe3b98` |
| `fill__fill-grey.json` | 512 | `eb252b667bb9dfd3cfbb67568a87460a3e5b392dfca9ba01b68815ba3347903f` |
| `fill__fill-orange.json` | 514 | `33a4308fc5719447c750679df1e6675236b784a514a66c6e5e5c2a0b4dd220e1` |
| `fill__fill-red.json` | 511 | `5f211a5e908fc155ea86c2a3ee7b7aae76e0422310adbb075ea873b141def652` |
| `fill__fill-white.json` | 513 | `2205c1fd4198af5e344840be4d98b7f56d8de6c2dcf840a6759c779cfed9d85a` |
| `fill__gradient.json` | 505 | `a8744e675a7ca815c87eea20322ac0304a85bc7e66a979e82cb62ead221baff0` |
| `fill__no-fill.json` | 505 | `6196b56edecda700c2dddb3a93f5831eb04f1d03217900ead49331fea0df5773` |
| `fill__scheme-accent1.json` | 517 | `44e990389d6452b9af11f7b2cb0d62f7cf9cd51eb51ecb3fd506d5342a7c305e` |
| `fill__transparency-50.json` | 520 | `2e2dc705a14e886d9033e0e763be6423960aeb3da62da06e403107cfce4e966c` |
| `geometry__base.json` | 507 | `f69d8eee249964c62d3cfc1a07f283186e0ba2dd45823923175ac39d57b6225d` |
| `geometry__flip-h.json` | 509 | `fef8aa1cfad5fe3b7c9192f3b55481797bdd90eff83807b49b2eed7618021cc6` |
| `geometry__flip-v.json` | 509 | `37449040006684b676188a8d1957b59d01c906a5bb2ee73b51c983058eeeafc2` |
| `geometry__height-10.json` | 512 | `46759071c0d526810b99611dad3223a0e11503664a69c214a9cbd468733f5759` |
| `geometry__height-300.json` | 514 | `86da80afcefadc884d944186e7b4099750103337ada615171c5a72edc5efe3be` |
| `geometry__left-0.json` | 508 | `db68d2ad4e4f4276f3a02cd027f509a12b75085979d5306234d07f7f9620327a` |
| `geometry__left-1.json` | 508 | `47ea61b8c40d2d5aaedb5008e5e6115210dd0e76d731db1f4e389d5148316f2b` |
| `geometry__left-100.json` | 512 | `7e72baec0c5752aa2241c167d5b3535fb95c169c313e68601b293d01c2609ae1` |
| `geometry__left-306.json` | 512 | `a34a13065dd1a76b86c6627efd377d2254dd834468d538d7a51d49fd546132b7` |
| `geometry__left-500.json` | 512 | `452190c070eb9237dbd4ac7520bcaa748e63a4f53eba3643c5ff7d70c8da1991` |
| `geometry__left-73.json` | 510 | `19dba829d8affb55ed5ce464877e1652d7fd6405c858baa0180d58357355440b` |
| `geometry__rot-1.json` | 508 | `05df223693a8ce66e92a2c1ea7267fc92c88da22e3af7b7f7eea8897d3269c82` |
| `geometry__rot-180.json` | 512 | `7d008986fdd208897ad3710ded02f99243e89ec76e9f3f1e40afa039d6414e66` |
| `geometry__rot-270.json` | 512 | `5cac30368354166d4d6026d244f17596e933caad71254619618dc1beada3ee46` |
| `geometry__rot-30.json` | 510 | `6f3c69b84562ddc69a18adeb958d2ea008d614903a222a7fd26f6c1ff53f8673` |
| `geometry__rot-359.json` | 512 | `e38b554d0ec816230f9a0f58673e4bb3cda7b40c7b02146fa91fdc880ecc7a30` |
| `geometry__rot-90.json` | 510 | `b335d018cb162ce0332646dcdd538fdb175af50ebbba1d79f38d32f88e195886` |
| `geometry__top-0.json` | 507 | `41f751579445f8f3f49ad0955eaeb5d92427664287c2a74fd747a4583ae842a4` |
| `geometry__top-396.json` | 511 | `dacaa25345ff68532c971a9c2305df1273a79fe2c8b61da9b62e754637fb3c12` |
| `geometry__top-700.json` | 511 | `ca3af72d5a9aa202d6157a109e5b97e31c82275784cd71b3b70ddf67cb7ebf7e` |
| `geometry__top-74.json` | 509 | `cc900033f8e0dbe1130220f3f839f5936376fa2ab437de9dd745c970869a0d1a` |
| `geometry__two-rects-swapped.json` | 884 | `737e026ac242d476afdf75d4611acf29edb872a24801c5470f7f3e9b5cce4c0d` |
| `geometry__two-rects.json` | 876 | `ab9e578b0f5c3857a8845d468727c95517e678bc1f5492b5f95aa5b4f95d16fa` |
| `geometry__width-1.json` | 508 | `3919ccc9903c646ec9abbcf2d69f2ed05733b0cae86099ebe54b9c0b24e93832` |
| `geometry__width-150.json` | 512 | `e05c8d3de76f34ebcb09695f67cdb047720cfc0df1f758a0b06c09db5f69b1ed` |
| `geometry__width-612.json` | 512 | `c44db54a4ed2276a055b4a9d88568dd7b9c05a020a0b4a548de0a999fa52e558` |
| `group__rotated.json` | 505 | `9871b3ce88b3c4e9b56e9eb7320b3e9a9408e9ee6234e58ed19758ad7765772c` |
| `group__two.json` | 868 | `0156f080a45b7ef28c84abea38191af5ed3298d827e82e5d5416df7a69c89d87` |
| `line__color-red.json` | 512 | `36f91c64b0008cfbcf132dfb42e4d3d3c3f6f2942c76b5819fa2e01b1d067518` |
| `line__dash-2.json` | 501 | `8c67a469f58694b309535c37067a8d8550e7f6ded73fcd5116d7df24fd498e04` |
| `line__dash-4.json` | 499 | `c13ae12f07bf601246903bc2b1701cf85fe48ef0df5efed602106d5c9c180ce7` |
| `line__dash-5.json` | 500 | `d050c049736cb2f0ff74fce119b0ddb8e6c68923fe507c3917f31ba5333b94dd` |
| `line__no-line.json` | 444 | `f944392abfb87a7abc2b54d2dfe816d3f8abc998e9617eeda9c3d815eb716dd8` |
| `line__weight-0.25.json` | 515 | `65aaaa7f08fb7e0bb893e8d00bf3d76de354f70dcaee62e680f278aa469f3535` |
| `line__weight-1.json` | 511 | `f4b2de06f85a66ebce8c92441e314f3ba4756e110ceba2494587a7a2077e4587` |
| `line__weight-10.json` | 513 | `f3888f78d0f891830077d56ea00960449a7319222bf10b69937d7acfa37112ef` |
| `line__weight-4.json` | 511 | `a79d9516829787c0bf0efd928bd05fef58dddb661a65bb831fa0e773eee15581` |
| `pages__landscape.json` | 514 | `70658c1d1e2113044478fe43c2b204c10ec36f9fce7754f2285c000247112789` |
| `pages__master-ignored.json` | 517 | `18c38979ead53c003dc0c303047d00032bef209649d215bbd6953cc20a5113ed` |
| `pages__master-rect.json` | 879 | `f50efb123459ad180d540a9d1fca4b75467f7f2bbb76c0e1d19edd4ca8a6a583` |
| `pages__master-text.json` | 1487 | `6dbae81f25eb9030ced57cfffc18451dfbf32a85916999eddc0b56b082e877ca` |
| `pages__one.json` | 506 | `97c8f79a4af219d14ffd3461e5ed4e6e4ad09d9d1df48069233826627b8a4cfd` |
| `pages__size-5.75x4.75.json` | 149 | `5914e77cbc5def6454900eb460116b6d996f226c151c0128b96dd4288cd45e43` |
| `pages__three-rect-p3.json` | 1045 | `6ae699251f122eb9a033bf45d7831196baf5b5c3da6e2fee5e0bda151f43e731` |
| `pages__two-rect-p2.json` | 964 | `875381d804178bdef3a8ec102127adfd74c0aebe9ac2c2adc6570c5da6b3a9db` |
| `pages__two.json` | 585 | `b7ab336cd53c9e1504a6fcfa76c26c3a7164362489a96a41489f3953720eae43` |
| `picture__base.json` | 495 | `a99efa79a00db65ff0d5cfad99380003901afb3018cadea1804594ddbc82913b` |
| `picture__border.json` | 500 | `0f23bb66de5ee1372d2f5b7059407c5b0b6f82929c15c0263a2ff46d98273591` |
| `picture__crop-all.json` | 506 | `00b020af199beb38bb8300f15b081acc8f9b11d23c4d09eb570cc5431df7110e` |
| `picture__crop-left.json` | 506 | `a33ea465c16b102bedd691e785c8d96bc9bc881d566bfe59dd29259211ccfd35` |
| `picture__crop-top.json` | 504 | `72a056355fc4aed5bf71a56810ba7285fabffdd0d93b233a0b184ced68611654` |
| `picture__flip-h.json` | 499 | `93ecd7761cb0c65cb36f27c6afbec672f791df00fd2b2b359a451095de30b7e8` |
| `picture__moved.json` | 513 | `49dbaabb57c5a3b83d7f14db734da5efa61259c2b0d741e86a5876b3885abd37` |
| `picture__oval-mask.json` | 505 | `ee719cba1555cc6fc9a6544f7839c3bab34f290070f4668addabcbd821205ec4` |
| `picture__rotated-30.json` | 481 | `b3139f435d01cf6ae774f70f60731fd95b32a464d9dc2eabf9fb3ad849a16deb` |
| `picture__stretched.json` | 505 | `ed86ffebcdf72949895ca18c01183c83996d6e3e7d921221d015ae2b4fbcd676` |
| `picture__two.json` | 832 | `9331559f78988917d317c496ce1c7f57355f12a006ab8359abefd9d2dfdfcff3` |
| `shadow__offset.json` | 501 | `378f131064e5cd398a752169fe8e3f449954708a8dd04b115221474346e9e980` |
| `shadow__on.json` | 504 | `f96264699705b1d9365b41c2f23427a64ce9c2725fa18d90ce49846bafa8e02b` |
| `shapes__curve.json` | 487 | `6b55e7b39c172d7a9346e7d06466b41e0f8eba52062ccc57ba36d2992ae3185d` |
| `shapes__diamond.json` | 504 | `eca5320bb3887af1fe25f7d9ead5296a72e6d08111d0bb28e9edefc7da19cf7b` |
| `shapes__heart.json` | 487 | `a0825afdf7a51a49610136bbe3e08a0b8016170ede3f18f8f87b81637d6a16a3` |
| `shapes__hexagon.json` | 503 | `298f396b5688756a5d9ab747a0aa1437b913a82ba2dc668ae0eda59c7e3fcb75` |
| `shapes__line-arrow.json` | 496 | `c633b340b400cf4449701fd412dc28fc185f0789aa853dd267a0f4550a2b72f2` |
| `shapes__line.json` | 496 | `50f4e5cb9ff60450ee3ccc0e03f5cceab1466bca2522f9e06c3b1b821273be74` |
| `shapes__oval.json` | 506 | `7c78b32e8731e1be06b67f15284407bef268e8e179470ff7a1d971a7269f572c` |
| `shapes__polyline.json` | 506 | `55a2fa113b7e0977d2d2b9e03b473c180b9cb995f2501fa64f81320caea080c7` |
| `shapes__rect.json` | 508 | `7e576e7fd52eb12ce9f7ce4cc9e38d4e044d5771e9f4c6b6217766385243dfbb` |
| `shapes__rightarrow.json` | 509 | `9a12f6cc35329c522deaf8cdd4be1a15f138385b108f80d52c831c99ed28912b` |
| `shapes__roundrect.json` | 506 | `6d70c8ba417bac0bb64021813514c092cdeae3961030c305d34c36b0d6cc99ca` |
| `shapes__star5.json` | 500 | `0a7ce721f3509ee30852b9a528cfe52f07663a3e4adac78a4b8eede235bbc1a7` |
| `shapes__triangle.json` | 491 | `d2dfcd3e2897f8a6f5d13a749b407c7fb1dc710473b5de55e141ee3a7c606d82` |
| `table__2x2.json` | 507 | `eb61750e7bb5bdfe8426daf226bf17691cd63cdb85ee8f8cc7b3da60f8ddfe7c` |
| `table__3x2.json` | 508 | `6d5df5bb7445293f09da5e61eebefbf03100e9033cc23a00eafdb9abb00d936b` |
| `table__merged.json` | 512 | `1f5921ee2a918a296d46fc4da561b6de72124c8f0c1cb96583ab4eedebb37fcb` |
| `text2__base.json` | 1668 | `b9fb28ad379e47b186820b571df2c5ace507ca32a46c1767ee0227890faa1382` |
| `text2__p2-bold.json` | 1670 | `fd241f145675c014c1b7334237b7bf62468c81941ef2d00e55b146e178ceaeee` |
| `text2__p2-center.json` | 1675 | `33b1e018c8c9d65c6c773a3c81d2928f093cb592d08181cbc078425947161ca2` |
| `text2__text-in-oval.json` | 1484 | `45ac7c36a2c8d39d8b58a61af6c90cd00088f1c5aed199e15ba401d1f69cdbf9` |
| `text2__two-boxes.json` | 2644 | `b95b44ff6a990e745735f0b1dc1e5a59ff5dd3eae7c226197acc1614f46aebd8` |
| `text__align-center.json` | 1139 | `440ab7e67521d172533fc3c0603c86511de232c6d021c852a1871c0dd17e5f1d` |
| `text__align-justify.json` | 1141 | `fbe66fa90560ca1be33f3110299fc292075e20f0e41d4bd3915f160b3e39297a` |
| `text__align-right.json` | 1137 | `2cbb77ca97a05f81de2b4f8b83b2f1e490764be5924ffa4487fedf8434d3928d` |
| `text__allcaps.json` | 1132 | `71d470d7e0f83c69a94dd11154dce8925af2404b94890a29e75ec949e64f8fa1` |
| `text__base.json` | 1129 | `012a1811d13c37a14a5a95d390004355005c29ae297b2cd56ca7501e2ac915e8` |
| `text__bold.json` | 1128 | `8692ae25de7b3e5e35312c1026f1d7f5f5de75258e9b732dc069eb512fbe16c2` |
| `text__bullets.json` | 1132 | `3b5faaa196fcce8d136c76a76de4333bc36b0acc826fb56c6be60ef682716d3f` |
| `text__color-custom.json` | 1137 | `34b81ab388c2592c2b8121c254ae11f0fc98093fb5ec9cd65201bf498cf336fc` |
| `text__color-red.json` | 1134 | `397358e8dfabb8c05149a96eee3a24032f2e3b2dcdc46c221df71b592112757b` |
| `text__font-Arial.json` | 1133 | `b5581711b72b0218de4ba1bac5f720030d109a293929a168e5e067268cd4281e` |
| `text__font-ArialBlack.json` | 1144 | `6687886d1255846037c7374f96d6a725991ad91219cf1a4ae14b33414288b09d` |
| `text__font-CourierNew.json` | 1144 | `03f98c9e71dda65ac34f10a4ec6d763063ca04956a3896b8bd14b35779a9ed98` |
| `text__font-TimesNewRoman.json` | 1151 | `b360279aab663df356b1109d672c1ccd5513e85eab893a929b641fd0f1a63308` |
| `text__frame-fill-line.json` | 1503 | `c0b7848b9c79ee0ff862b25eafd027817d1a6b6c2b53d770d25ff0b31d484eab` |
| `text__indent-first-18.json` | 1140 | `4bf4b74cdecbce53a0cf3275c38d67073375cb7864bebe4eb8d6c1e21f3c2763` |
| `text__indent-left-18.json` | 1139 | `29a68ff6d4faad7cf359f8f6e25e54dcccfb7694eaf23f8b4da73c9c7cc3c579` |
| `text__indent-right-18.json` | 1140 | `13fc434a5a1d7b35e976958ee058caf6d8d4538d5d491072a7fc9db7197d3847` |
| `text__italic.json` | 1130 | `7a7504cf6a50084a05aa7754d976edc7eb75570a6e64e2c09f75a4ed7716bab7` |
| `text__line-break.json` | 1136 | `5fde5764105da1c47e4489e53cbe79507c6d89f696564ee062de8c5c72333a80` |
| `text__ls-double.json` | 1121 | `4d59f630da71c5eb8fc2c2bd9abf8bc3b23c097e3498ec86402dffe6d8d133f7` |
| `text__ls-exact-14.json` | 1123 | `e234067edb3197946abb1cf810e61e11d3507e7e054b03a3539d81fa5490c868` |
| `text__ls-exact-30.json` | 1123 | `59a8b043f844fb5c33c6c72d2cdb18c459f5e0e0623d878fdbf742a918c235c3` |
| `text__ls-multiple-1.5.json` | 1127 | `0e4c0cfffed4fb9d729e5373bfc87dfb2454096c4e8222924abff15946e83372` |
| `text__ls-single.json` | 1121 | `7ae5795f6658b02717b06ae9040f49500c62372e960c646e232fd46a0033dcf4` |
| `text__margins-10.json` | 1083 | `680d09775a86704dc6bbdeca9f13d9e32a9147c54c3a20bfee33d327f5524239` |
| `text__rotated-270.json` | 1138 | `8f52403c695ea7b2689b748248aaf2145fcf534e885460822aa323dd9711b682` |
| `text__run-bold-word.json` | 1741 | `b20176550202523fcb46cfa133b27e371af57e458ea71356fec8307926d9ab8a` |
| `text__run-font-word.json` | 1740 | `de42ca9f021e3b0ea8867b87829e5df1c3e03269a87e1f38b5c2bb0340a5a2d6` |
| `text__run-red-word.json` | 1741 | `12512f99be8d88fd207d9c28e30ced37c4e47546b2a73c1b2ac8f7f18b6e0a3f` |
| `text__run-size-word.json` | 1742 | `f964f16a56e1a2ebce47de575d84d96835508ed11fe53681f8be9cba7b78138e` |
| `text__size-12.json` | 1132 | `3fe403dca3d46ddabcd81d389edd874dcc5f19eec17c3fcafa29c70dbdf66af6` |
| `text__size-24.json` | 1132 | `8161d16f76e1e49bb88b3e4c571878ab3047df59bc3d869943444dc4ed4df613` |
| `text__size-36.5.json` | 1134 | `60c2d79ea0c28acb3f91c8c9a26b5230f8b4df84c520b537cac585f67bc02017` |
| `text__size-72.json` | 1132 | `c150dab99a98e6a93ee3be0439b08706574198d70c1b479072bffd1ad652a4f3` |
| `text__size-8.json` | 1130 | `ef3c6c825f968efea0a321497892ba3d686e8ec104a306974edebc96c84b7371` |
| `text__smallcaps.json` | 1134 | `6274877d4eef42458103bef4d4a9f0a61097ec9314e3a081ba015e56e8458706` |
| `text__space-after-0.json` | 1138 | `1c1762c7449daa0b1d31637e6f74b0d6292fae8aa22ca4c3b329fbc0093892f3` |
| `text__space-before-12.json` | 1141 | `ee8ff0f3d42c9c03b5b016942a1d9459be7d28c893a0b2ade09abb3aee59923a` |
| `text__superscript.json` | 1136 | `e881728ec2384cdf41043a90d78a640816f001c6391b0bece60e994fe8f2a7fc` |
| `text__text-changed.json` | 1137 | `6c9df741ac7fc60ef5055a954f8e97d1be57d3a339403261411f4f9200f1fb8d` |
| `text__text-longer.json` | 1164 | `4ea2932bab798f68c2e295779f9c205ab720eb68445738db7474e21cbee67db5` |
| `text__tracking-200.json` | 1137 | `be2f35c68651b832d4be69d920bb38d6225a5a2a8c163ff8e1751b7f37aa7d78` |
| `text__underline.json` | 1134 | `4a8753398323db5d03bf5c2e7f12d25d8b89baf14995a5d3931cff715c1ef256` |
| `text__vanchor-bottom.json` | 1139 | `7ff0965664124ee17b4e3b5e71eb9cf88e6c169ba3b2d983ea5a3d11d48dc19c` |
| `text__vanchor-middle.json` | 1139 | `6e169450389dbe4f7c19d770e76f35faa3e11e66b73d5c36c6307b47c9f6be7a` |
| `wordart__arch.json` | 497 | `9f0ef4e62604e1ab1f7d911c982814577f86d5bf26f63ac9172f335b239e9728` |
| `wordart__base.json` | 485 | `905bdb30f35e37ac7d8a939e13c228dd10fb04c4528029e27e54cb9c4484ee3e` |
| `wordart__circle.json` | 503 | `29ae8beb795c40859c636cc4af560c66a16b276ba22dae8699ec0501a1b2fe21` |
| `wordart__fill-red.json` | 493 | `ef0504f585524e7dbc8a469dbe2e1d454c622faa80eea8793bb6a7d18639e279` |
| `wordart__font.json` | 485 | `b6f1aebea5b07b2bce3bc72376655f7fe79c6b01b9c069f4f6504cd1b90f50b6` |
| `wordart__line-white.json` | 498 | `86f0a62ccd64dde6c7b2eb40efaf4ebdf492095307248342e034b71a3c9997c9` |
| `wordart__shadow.json` | 488 | `c4caf94c8202a22b8a669ed403ee40fb55e80376e055862facae32f7c6c58780` |
| `wordart__text.json` | 486 | `6563bc7a2872593f622ae892371c6a967cdd7a897f17badd2819224d7b41fd8a` |

## Builder — 2026-10-24 C7 path observations and validation

Used only the .pub bytes of shapes__curve.pub and shapes__polyline.pub from the pinned controlled set to inspect path arrays through this project's existing escher::parse. Section 5 of odraw-geometry.md records the words and coordinates. No scoring exports or images were used to infer path storage. Neither sample settles four-byte vertices or guide indexes, which remain unsupported. Existing Escher property retention suffices; contents.rs, quill.rs and read.rs were not changed.

The scored corpus before/after is identical: 142/142 imported, 152/152 objects, 111/111 scored shape/text positions within 1 pt, 52/52 exact texts, 111/111 styles. The 24-file batch before/after is identical per file, including all diagnostic strings and page/object counts: 9 converted with notes only, 10 with warnings, 2 old-version refusals, 3 damaged-file rejections; 31 pages, 280 objects. CLI PNG renders of both controlled paths were inspected outside the checkout; a cubic and the closed polyline are drawn. No Windows execution or comparison against Publisher's screen output was performed.

Validation: three new unit tests and the external fixture test pass. Workspace formatting, clippy -D warnings and 3,061 tests pass (20 ignored). Full CI stops at the six existing unregistered examples/csv-merge assets; the same asset checker reproduces that failure on clean origin/main. No unrelated asset files or attributions were changed. Layers pass for all 22 checked crates; wasm checks pass for all 17 selected crates. The remaining path limitations are recorded in odraw-geometry.md §5.

## Builder — 2026-10-24 C8 baseline and blocked official download

- Used main `550357e3b25b55e078f2e6d0463c51d879cf8bfd` with merged C7 paths. Read the binding rules, preset-geometry.md and the existing Builder::geometry implementation. Inspected only the pinned .pub data for shapes__heart, shapes__star5, shapes__diamond, shapes__hexagon, shapes__rightarrow, shapes__triangle and shapes__roundrect through this project's existing escher::parse and import_pub. The audit is recorded in preset-geometry.md; no new preset mapping or adjust conversion was inferred.
- Verified all 166 .pub hashes against publisher-fixtures.csv and all 142 scoring JSON hashes against the table above. Local folders: `/tmp/niftydata/pubsmith-fixtures`, its `truth` subfolder, and `/tmp/niftydata/apache-pub`. Pinned commits and data-only restrictions remain as recorded above. The scoring JSONs were used only by the corpus test, never to infer path storage or definitions. No source from fixture repositories was opened.
- Untouched-main baseline: 142/142 imports, 152/152 objects, 111/111 scored shape/text positions within 1 pt, 52/52 exact texts, 111/111 styles. Real batch: 9 notes only, 10 warnings, 2 refused, 3 damaged; 31 pages, 280 objects.
- Attempted official source on 2026-10-24: `https://ecma-international.org/publications-and-standards/standards/ecma-376/`. The environment proxy returned `Tunnel connection failed: 403 Forbidden`. No standard archive or XML was downloaded; its SHA-256 is unavailable. No other source was used and no text-warp file was read. Implementation is blocked on the required official definition file and operator meanings.
- Verification after the audit: cargo xtask ci passes all seven steps (fmt, PDF dependencies, clippy -D warnings, 3,061 tests with 20 ignored, assets, layers and wasm). The repeated corpus score is unchanged at 142/142, 152/152, 111/111 scored positions, 52/52, 111/111. The repeated real batch is unchanged per file, including diagnostic text, pages and objects: 9 notes / 10 warnings / 2 refused / 3 damaged, 31 pages, 280 objects. No product code was changed.

## C8 heart outline — 2026-10-24

Downloaded the official ECMA-376 fifth-edition Part 1 archive from `https://ecma-international.org/wp-content/uploads/ECMA-376-1_5th_edition_december_2016.zip` on 2026-10-24. ZIP SHA-256: `9d0bcad9cf06054785b03762fcfadbf6bab7e54a5f9d69434e34b7fd464d4129`. Opened only `presetShapeDefinitions.xml`, found by name inside `OfficeOpenXML-DrawingMLGeometries.zip`; XML SHA-256: `2f7c868d857c1e3c4b5a6068759fe0e07d77ad58377a6618d1b02ba3507b6939`. Neither XML nor archive is committed. No PDF, schemas, text-warp definition, other reader or fixture-repository source was opened.

OBSERVATION: the official heart entry matches the previously documented seven outline guides, move, two cubic curves and close. The local implementation uses only its `*/` and `+-` arithmetic and outline coordinates. The MS-ODRAW number 74 is retained from odraw-geometry.md and the existing seven-fixture shape-number audit. No new Publisher encoding was inferred; text rectangle, connectors and WordArt were not implemented. Unit tests independently check a 96 by 120 frame using hand-computed coordinates; the external heart fixture checks the closed symmetric cubics. The other six audited presets remain unchanged.

Downloaded only 142 `.pub` files and 142 scoring JSON files from FuncularLabs/pubsmith at `0ccdf27fc3f381f0897515e226ea485165c699b2`, and the 24 listed real `.pub` files from apache/poi at `bf7c9b6a1490046517cbd8e1504e702a89be4bc9`. All 166 `.pub` sizes and SHA-256 values match publisher-fixtures.csv; all 142 JSON sizes and hashes match the scoring-data table above. Files remain under `/tmp/niftydata`, outside the checkout. JSON was used only by the existing scoring regression, never to infer encoding or geometry. No golden images were downloaded. A local `c8-input-sha256.txt` records all 310 input hashes including the standard archive and XML.

Untouched main `bc1b23c` and the heart change both score 142/142 imports, 152/152 objects, 111/111 scored shape/text positions within 1 pt, 52/52 texts and 111/111 styles. The real batch before and after remains 9 notes, 10 warnings, 2 refused and 3 damaged, with 31 pages and 280 objects. The CLI heart PNG was inspected: two rounded lobes and a pointed bottom, retaining fill and stroke. This is our own rendered output, not comparison with Publisher. Windows, Publisher's own drawing and non-default preset adjustments were not tested.

Validation: all seven workspace CI steps passed (formatting, PDF dependencies, warnings-denied lint, tests, assets, layers and WASM). Workspace tests: 3,065 passed, zero failed, 21 ignored. Separate controlled scoring, 142-file save/reopen/edit/PDF, real-file batch regression and seven-preset fixture checks passed.

## Missing-font substitution — 2026-10-24

Reused the pinned 142 controlled `.pub` files, 142 scoring JSON files and 24 Apache
`.pub` files already recorded here; all hashes and sizes matched their ledgers.
Scoring JSON was used only by the unchanged corpus scorer. Consulted this
repository's font-resolution/UI code, bundled font metadata, the available Linux
font catalog and the read-fonts post-header accessor. No Microsoft fonts were
downloaded, no other Publisher reader was read and no protected reader changed.

Missing-name type and width hints are substitution policy; the absent originals'
metrics remain UNKNOWN. Controlled and real corpus numbers stayed unchanged;
all seven workspace CI steps passed, with the imported-font UI test also run
separately. [font-fallback.md](font-fallback.md) records the exact choices,
validation and visible Newsletter clipping/reflow, including 1,809 versus 1,747
rendered glyphs without a claim of improved fidelity.


## Publisher reference PNGs for pixel scoring

OBSERVATION: downloaded only the following nine PNG data files from `FuncularLabs/pubsmith`, commit `0ccdf27fc3f381f0897515e226ea485165c699b2`, directory `tests/fixtures/pub/golden/`, via individual `https://raw.githubusercontent.com/FuncularLabs/pubsmith/0ccdf27fc3f381f0897515e226ea485165c699b2/tests/fixtures/pub/golden/<name>` URLs. The pinned directory lists nine images, totaling 574,351 bytes. Each is 2550 by 3300 pixels and declares 11,811 pixels per meter on both axes (approximately 300 dpi). The user explicitly continued with this nine-image set. Files remain outside the checkout in `/tmp/niftydata/pubsmith-fixtures/golden/`; hashes below were computed after download. No source from that repository was read.

| PNG data file | Bytes | SHA-256 |
| --- | ---: | --- |
| `group__rotated.png` | 74669 | `ef717c25d864ae76261cc67bf7951d2608596caea6372d889fc301f28fcee2d4` |
| `shadow__offset.png` | 45272 | `6b30073ec659c83cf2f15e2233566a324f9560fedc5003bbdb1b432eda3f27ac` |
| `shadow__on.png` | 45222 | `d79c0a144ceb902a9ebe63bfdb056ae8664037a5a36b53c1b80c0ad3c978e408` |
| `wordart__arch.png` | 67029 | `86e134cfb7599a047ea509305ddcaa3d6a228de61749ba6837b5b0242be8b482` |
| `wordart__base.png` | 65178 | `d86a363b2c12d3827d32b4a3bb36a9cf79df82a3f654f914bbd6685a748d149a` |
| `wordart__circle.png` | 94420 | `1edd12d9ef14c09f3bbd512cf95085e21b3f8b9017bb86decfea9111da546d1b` |
| `wordart__fill-red.png` | 67238 | `27df00497ee9f329decd158a20eedf46d32e45131bd82586c2d8813f08b5c282` |
| `wordart__line-white.png` | 40722 | `b6accda793819b4db69f4684683b703b6d630fdabf8858f86aa1f77bc4174543` |
| `wordart__shadow.png` | 74601 | `04f699c60f3136b4b3c63814244ae03dac9040e1af2df22ef8cdc515c5e031bf` |

These Publisher images and the existing truth JSON are scoring oracles only, never inputs for inferring storage. No reference image is committed. Reused the existing 142 controlled `.pub` files, 142 scoring JSON files and 24 Apache `.pub` files after checking their existing manifest hashes and sizes. The new scorer uses this workspace's importer, renderer and PNG decoding/encoding, with no new dependency. [pixel-diff.md](pixel-diff.md) records the visual baseline and limits; own-output snapshots are a separate regression measure, not Publisher comparisons.

## C9 WordArt paths — 2026-10-25

OBSERVATION: downloaded the official ECMA-376 fifth-edition Part 1 archive from `https://ecma-international.org/wp-content/uploads/ECMA-376-1_5th_edition_december_2016.zip`. Listed archive members to locate `OfficeOpenXML-DrawingMLGeometries.zip/presetTextWarpDefinitions.xml`; opened only that XML, not the shape XML, PDF or schemas. The archive and extracted XML remain outside the repository.

| Input | Bytes | SHA-256 |
| --- | ---: | --- |
| ECMA-376-1_5th_edition_december_2016.zip | 43501721 | `9d0bcad9cf06054785b03762fcfadbf6bab7e54a5f9d69434e34b7fd464d4129` |
| presetTextWarpDefinitions.xml | 51599 | `89640bd4aeedfa2c578d1d1acc50d569865ebdef315fd4b468087c25e36acada` |

OBSERVATION: reused the pinned 142 controlled binaries, 142 truth JSON files, 24 Apache binaries and nine reference PNGs; all 317 recorded sizes and hashes matched before work. Storage audit used only the existing parser/importer on seven WordArt binaries and the permitted MS-ODRAW notes. XML supplies only outline paths, not Publisher storage. Truth JSON and PNGs were used only for scoring, and PNGs for visual comparison. INFERENCE and UNKNOWN items, including the 144/146 name mapping and approximate glyph fitting, are recorded in wordart.md. No other reader, repository source or web format search was used.

OBSERVATION, C9 review: reverified all 317 data sizes and hashes, reusing the same XML and scoring references. Upward arch fitting now uses the existing composer's resolved font advances and glyph-ink bounds, with no guessed character width or fixed horizontal scale. Two measured circle trials missed the original reference baseline, so circle remains straight with its warning. The existing workspace composer dependency adds no external package. The trial scores, final nine-reference comparison and remaining INFERENCE/UNKNOWN items are recorded in wordart.md; no new format input was consulted.

## Q: position scoring and headline clipping — 2026-10-25

OBSERVATION: reused the pinned 142 controlled binaries, 142 scoring JSON files, 24 Apache binaries and nine reference PNGs; all 317 existing sizes and SHA-256 values matched. Read-only diagnostics outside the checkout used this repository's existing parser, importer and composer; their 161 successful documents matched the unmodified importer apart from timestamps. Truth JSON supplied only scoring bounds; PNGs were hash-checked, not used for storage inference. [position-misses.md](position-misses.md) records the 41 image exclusions; [headline-clipping.md](headline-clipping.md) records stored text, complete parsed properties, composition and an inspected newsletter render. No product code changed. The permitted MS-ODRAW page returned `curl: (56) CONNECT tunnel failed, response 403`; no new specification text was obtained, and additional fit-flag meanings remain UNKNOWN.

## R: separate picture position scoring — 2026-10-25

OBSERVATION: reused all 317 pinned controlled, Apache, scoring JSON and reference PNG files; their existing sizes and SHA-256 values matched. The existing drawing parser and importer were run on the controlled binary bytes outside the checkout; only truth JSON scoring comparisons selected the comparison bounds, never storage interpretations. Golden PNGs were hash-checked only; no new format source was consulted. The picture-only baseline is 12 within 1 pt, zero measured misses and 29 unmatched image-labelled references; [picture-positions.md](picture-positions.md) records bounds, source observations, INFERENCE/UNKNOWN limits and the historical-denominator wording corrections. Only test code and documentation changed; the old coverage, text and style totals remain unchanged.

## S: text fitting specification and frame audit — 2026-10-25

OBSERVATION: accessed only the following public MS-ODRAW pages on learn.microsoft.com on 2026-10-25; the network check returned 301 and its same-document redirect returned 200. External document links were not followed. Restored 317 pinned data files as individual raw downloads and checked every existing size/hash; binary storage observations use the existing parser only, JSON/PNG references only scoring or hash checks. [text-fit.md](text-fit.md) records SPEC definitions, seven frame observations, use-mask handling and UNKNOWN host behaviour. No active defined shape-fit flag was found, so behaviour was unchanged; gates, nine image scores and all 142 self-snapshots match before/after.

| Scope | Pages read |
| --- | --- |
| Overview and navigation | [MS-ODRAW landing page](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/8560795e-7759-4745-838f-f7f2ef2f1872); [contents metadata](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/toc.json) |
| Flag groups | [2.3.21.15 Text](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/ab9e4283-a47f-429c-8c2b-683a3a7f16d1); [2.3.2.12 Shape](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/aff4b132-455c-40bf-9639-e48a13e1cd5c); [2.3.4.44 Group Shape](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/a0ae6aa5-16e4-4550-87a2-b8ca180c7fd7) |
| Text properties | [2.3.21.1 lTxid](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/6f50606e-9a41-4555-961f-95317c8ed15c); [2.3.21.6 WrapText](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/749ffcd2-b8f2-4311-9f9f-bec3231bdc73); [2.3.21.7 unused134](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/86c644d0-4089-45b8-a17b-1ff865ffc1ff); [2.3.21.8 anchorText](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/908585b2-49d5-47aa-b2e0-5436025a7ad4); [2.3.21.14 unused141](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/dccd7089-cfb0-4c00-a420-d57b0188868c) |
| Internal margins | [2.3.21.2 Left](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/e89d660e-4f08-4786-a159-ee90cc76c9ac); [2.3.21.3 Top](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/7bb231df-17ce-4111-8eba-9a8337e96563); [2.3.21.4 Right](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/4d1d87bd-e76f-4f26-a86f-82b26f2c1b9b); [2.3.21.5 Bottom](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/a1e315f3-bec7-418a-b872-379cf08bbeba) |
| Enumerations | [2.4.3 Wrap](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/893723db-1a02-4790-9418-dbfefc30571d); [2.4.4 Anchor](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/fabbbc36-61b4-45da-9cb6-5aa695f411c9) |
| Property representation | [2.3.1 Property table](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/a7ea6419-bdb8-4faf-844d-fa4acbeb9ecd); [2.2.7 Property entry](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/2841bed9-1ff1-4981-807e-ffb9592c046d) |
| Host scope | [3.2.3 Shape text](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/5e5f0ff0-5a96-4f29-88e4-ad738d4f4bae); [Appendix A, notes 52, 56–58 and 100–102](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/88d4a826-a5d7-490c-88ea-acdd68cbe446) |

## U: import-time overset notes — 2026-10-25

OBSERVATION: reused all 317 pinned controlled/Apache binaries and scoring JSON/reference PNG files after checking existing sizes and SHA-256 values. Only this repository's importer and existing composer supplied the new diagnostics; no new format source was consulted. JSON and PNG inputs remained scoring oracles only. Read-only composition adds one lower-severity note per overset framed story: seven in SampleNewsletter, three in SampleBrochure and one in text__size-72; no warning category changes. [text-fit.md](text-fit.md#import-time-overset-notes--2026-10-25) records exact wording, file counts, warmed and first-pass timing, unchanged corpus/pixel/snapshot results and remaining UNKNOWN items. No layout, text, font or frame change was made.

## Older Publisher research — 2026-10-25

OBSERVATION: notes-only census on main `71adf6a`, using the unchanged importer on all 142 controlled and 24 Apache binaries at the previously recorded pinned commits. All 317 existing binary, truth and golden files matched their recorded sizes and hashes; no data download was needed. Truth/golden data were used only by existing scoring checks, never to infer version fields. The two old-family binary hashes are repeated in [publisher-97-2000.md](publisher-97-2000.md). No older record layouts were decoded.

OBSERVATION: every Microsoft resource read for this investigation is listed below, accessed 2026-10-25. The catalog root redirected to its listed canonical page; the MS-CFB root likewise redirected to its overview. Contents metadata was used only to locate pages. No linked external download or other reader was opened. Page update timestamps identify page revisions where no published protocol revision is stated; metadata and the missing resource have no supplied page version.

| Resource URL | Page version or result |
| --- | --- |
| [MS-PUB requested path](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-pub/) | HTTP 404; no page version available |
| [Office File Formats catalog](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-offfflp/8aea05e3-8c1e-4a9a-9614-31f71e679456) | No protocol revision stated; updated 2025-07-11T20:45:00Z |
| [Catalog contents metadata](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-offfflp/toc.json) | No version label provided |
| [Technical-document list](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-offfflp/6ae2fd93-51fc-4e75-a54a-1b175c627b51) | No protocol revision stated; updated 2025-07-11T20:45:00Z |
| [MS-CFB overview](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-cfb/53989ce4-7b05-4f8d-829b-d08d6148375b) | Revision 12.0, 2024-04-23; page updated 2024-10-30T21:10:00Z |
| [MS-CFB contents metadata](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-cfb/toc.json) | No version label provided; navigation for revision 12.0 |
| [MS-CFB applicability](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-cfb/a35cc9d5-d3d9-488d-8976-d3a121f719db) | Revision 12.0; page updated 2024-04-23T17:55:00Z |
| [MS-CFB versioning](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-cfb/0dfb84e2-4b22-4878-827f-96ef8e4dcdac) | Revision 12.0; page updated 2024-04-23T17:55:00Z |
| [MS-CFB header](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-cfb/05060311-bfce-4b12-874d-71fd4ce63aea) | Revision 12.0; page updated 2024-10-30T21:10:00Z |

UNKNOWN: the requested MS-PUB resource returned HTTP 404, and neither inspected Office File Formats list contained Publisher/MS-PUB. No Publisher applicability statement was obtained; this does not establish that no specification exists elsewhere. MS-CFB documents the common container, not the older Publisher payload. See [the research boundary and full census](publisher-97-2000.md).

## Older Publisher sample observations — 2026-10-25

OBSERVATION: notes-only inspection on main `8148a73`, using the existing container reader and unchanged Escher/Quill parsers, stream listings and plain byte comparisons. All inputs below are Apache binary data at commit `bf7c9b6a1490046517cbd8e1504e702a89be4bc9`, under `test-data/publisher`, checked against `.github/test-data/publisher-fixtures.csv`; no external source or reader was consulted and no data was downloaded. All 317 pinned corpus binary/truth/reference files matched recorded sizes and hashes; truth/reference images were used only by existing regression scoring. No old record layout was decoded or parser behavior changed. Extracted streams and diagnostic results remain outside the repository under `/tmp/niftydata/pub-old-observations`. See [the observations and limits](publisher-97-2000-observations.md).

| File | Bytes | SHA-256 |
| --- | ---: | --- |
| `Sample98.pub` | 61440 | `8912f295ff0cd1f55b873b3b98a182218f33c420877e025d2f0fd795f12dd0c0` |
| `Sample2000.pub` | 61440 | `40701ca47b26d04771cdd58467764e9ab39d3da69b59fbc529afd16d263d2f86` |
| `Sample.pub` | 72192 | `6fefdef46b87c767150878dc384549cb2d2ec2ac54de25f8ddb3a5628301107e` |
| `Sample_2010.pub` | 72704 | `f0a90aa566d72eb822eb48b4f378c33007610e35f3f1b999cdb88f2c9683858b` |
| `Sample2.pub` | 72704 | `c87ea7cc5606023c5fa99376631bbd45a8e9bdaba34c958057995b0e7ac8cc86` |
| `Sample2_2010.pub` | 73216 | `931ff20980b8f72aab227fa3cbe61b017be8d17c337fecb9aca3c97075940777` |
| `Simple.pub` | 65536 | `2606f530052a818c7949b88c9bf245dbc0a74ddc6b5a60004f37b387fe4b955d` |

## Condensed font substitution — 2026-10-29

OBSERVATION: the unchanged baseline used all 142 controlled and 24 Apache binaries at the previously recorded pinned commits; all 317 binary/truth/golden files matched recorded sizes and hashes. Existing parser output and imported formatting supplied font names; truth/golden inputs were used only for scoring. No original Microsoft font bytes/metrics or additional Adobe collection fonts were read or added.

OBSERVATION: font selection inputs below were downloaded on 2026-10-29 from Google Fonts commit `9862abc066c9b83bf84c3e8ed1fc2d8d48f9bbf0`, only binary fonts, full licences and the families' own metadata. Barlow Condensed Regular/Bold and both bundled licence texts are unchanged; the two Nifty Narrow Serif faces are renamed static Roboto Serif derivatives at wdth=62.5, opsz=144, GRAD=0 and wght=400/700, with every glyph retained. Generated SHA-256 values and attribution are in ASSETS.md; exact preparation is recorded in `packaging/prepare_condensed_serif.py` and verified byte-identical on repetition. Stint and Bebas candidates were not bundled. See [font-substitution-widths.md](font-substitution-widths.md) for the audit, choices and limits.

| Download URL | Bytes | SHA-256 |
| --- | ---: | --- |
| https://raw.githubusercontent.com/google/fonts/9862abc066c9b83bf84c3e8ed1fc2d8d48f9bbf0/ofl/robotoserif/RobotoSerif%5BGRAD,opsz,wdth,wght%5D.ttf | 3944924 | `351ced75f3851806aa6d846b669361521eb1925cfc530396df9c1a1b77061ddb` |
| https://raw.githubusercontent.com/google/fonts/9862abc066c9b83bf84c3e8ed1fc2d8d48f9bbf0/ofl/robotoserif/OFL.txt | 4489 | `34dbfbb43e0b4fdeef445d77b9ac0b988e5ad7a9bbf16808c97b66c66d51f553` |
| https://raw.githubusercontent.com/google/fonts/9862abc066c9b83bf84c3e8ed1fc2d8d48f9bbf0/ofl/barlowcondensed/BarlowCondensed-Regular.ttf | 102480 | `583cec5da3b84bc4dc7c9c72e2a565c94d34e431518b19d7e250b7830ad5f996` |
| https://raw.githubusercontent.com/google/fonts/9862abc066c9b83bf84c3e8ed1fc2d8d48f9bbf0/ofl/barlowcondensed/BarlowCondensed-Bold.ttf | 109912 | `e476562ec9c1e16cf16475895b511f08c804f438cc9a9f80a44ea50a0eeb5b65` |
| https://raw.githubusercontent.com/google/fonts/9862abc066c9b83bf84c3e8ed1fc2d8d48f9bbf0/ofl/barlowcondensed/OFL.txt | 4377 | `186d750eb496a4c17a76385f82be6aea2ac1cf2de074a811d63786cf374ea73f` |
| https://raw.githubusercontent.com/google/fonts/9862abc066c9b83bf84c3e8ed1fc2d8d48f9bbf0/ofl/stintultracondensed/StintUltraCondensed-Regular.ttf | 40272 | `1c32b77a50e960f9e88e29e1c65a0a15ffe277681ace0dbf207fafa7be23591e` |
| https://raw.githubusercontent.com/google/fonts/9862abc066c9b83bf84c3e8ed1fc2d8d48f9bbf0/ofl/stintultracondensed/OFL.txt | 4444 | `fc1e501d419d84a8f1a800adae092045002d7141241f498237611dbb8ea763d7` |
| https://raw.githubusercontent.com/google/fonts/9862abc066c9b83bf84c3e8ed1fc2d8d48f9bbf0/ofl/robotoserif/METADATA.pb | 1744 | `5f5893410acc682cefc0eef3c6be921fde33c6e78a052fc763ca949ddb102eee` |
| https://raw.githubusercontent.com/google/fonts/9862abc066c9b83bf84c3e8ed1fc2d8d48f9bbf0/ofl/barlowcondensed/METADATA.pb | 5571 | `fdb9c7b3c0c2c1069bc0193f0b72fb95efef36dc155c281e307da71b7dcfc558` |
| https://raw.githubusercontent.com/google/fonts/9862abc066c9b83bf84c3e8ed1fc2d8d48f9bbf0/ofl/bebasneue/METADATA.pb | 636 | `180da4f2ee7757e589eb5cb82f32c58b64c49f015e504ccab6b21401e7f5d43a` |
| https://raw.githubusercontent.com/google/fonts/9862abc066c9b83bf84c3e8ed1fc2d8d48f9bbf0/ofl/stintultracondensed/METADATA.pb | 715 | `15e32f1cedfb18a9366b4f8cc293201b7ec9e7507b1c3550d7ce7fab1d479470` |


## Table formatting observations — 2026-10-29

OBSERVATION: restored the 142 controlled binary fixtures and 142 truth JSON files from individual raw data URLs at `FuncularLabs/pubsmith` commit `0ccdf27fc3f381f0897515e226ea485165c699b2`, `tests/fixtures/pub/`, plus its nine `golden/` PNGs; restored 24 Apache binaries at `apache/poi` commit `bf7c9b6a1490046517cbd8e1504e702a89be4bc9`, `test-data/publisher/`. Every binary size/hash matches `.github/test-data/publisher-fixtures.csv`; every JSON/PNG size/hash matches the existing tables above (317 files). Files remain under `/tmp/niftydata/pubsmith-fixtures` and `/tmp/niftydata/apache-pub`. No upstream code, README, other reader or web format search was accessed.

OBSERVATION: the table census reads all successfully readable binary Contents directories and Escher drawings. Detailed cell-field observations in [table-formatting.md](table-formatting.md) use the eleven binaries below, the existing bounded property-list/Escher readers and the documented Quill section directory. Scoring JSON was used only by the unchanged corpus checks; golden PNGs only for hash verification and pixel scoring. Neither supplied a storage interpretation. The MS-ODRAW margin definitions are retained from the permitted pages already listed in the text-fit section; no new external specification was used.

| Table binary | SHA-256 |
| --- | --- |
| `table__2x2.pub` | `72d2127f4ffdbe998f4f94414a28584e49af817991b70a564fda885dcb1a6a7d` |
| `table__3x2.pub` | `12c45cd9057553f748d0cf23ab5348d901be294b1d6e790eb80c3331d57d2616` |
| `table__merged.pub` | `3df51f5e3e29f247b8636627725d43f5973dc7ea887338f32aec8f4b49a23b30` |
| `Sample.pub` | `6fefdef46b87c767150878dc384549cb2d2ec2ac54de25f8ddb3a5628301107e` |
| `Sample2.pub` | `c87ea7cc5606023c5fa99376631bbd45a8e9bdaba34c958057995b0e7ac8cc86` |
| `Sample2_2010.pub` | `931ff20980b8f72aab227fa3cbe61b017be8d17c337fecb9aca3c97075940777` |
| `Sample3.pub` | `424c69173ff08948c2529c8084b4ac2403f1ff1057146f4edd02fc29b44481fc` |
| `Sample3_2010.pub` | `baa9555a72d9e4264e3fd722896e2e43b960d20b4afdb96f75cb721a6133ff91` |
| `Sample4.pub` | `42195f7ad23d911219fea3ec88e66e867e9b9a6821a16dd1b535e2aa9d57a11b` |
| `Sample4_2010.pub` | `544e2043f91aed664c0a09f9ae45da42291b6730f107f99f3cdf9462ed38fb02` |
| `Sample_2010.pub` | `f0a90aa566d72eb822eb48b4f378c33007610e35f3f1b999cdb88f2c9683858b` |
