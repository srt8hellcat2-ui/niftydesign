# Missing fonts in imported Publisher files

## Existing chain

OBSERVATION: `ScopedFonts::face` in `crates/fonts/src/fontdb.rs` first resolves a
canonical typographic, WWS or localized family name. It then tries normalized
alternate names (surrounding whitespace, the `$ID/` prefix and format suffixes).
An exact available family is used before any substitute.

OBSERVATION: `ScopedFonts::substitute_family` had explicit candidate chains for
Calibri, Cambria, Arial/Helvetica, Times, Courier, Arabic and Chinese family-name
hints. It used the first available candidate, otherwise Source Sans 3. Arial
Black, Elephant, Franklin Gothic Book, Rockwell Condensed and Sylfaen had no
matching chain and therefore used Source Sans 3. No type or width ranking was
performed for those names.

The Latin candidate lists, tried in order and skipping absent families, were:

| Missing name | Candidates |
| --- | --- |
| Calibri, Calibri Light | Carlito, Liberation Sans |
| Cambria, Cambria Math | Caladea, Liberation Serif |
| Arial, Helvetica | Liberation Sans, Arial, Helvetica |
| Times New Roman, Times | Liberation Serif, Times New Roman, Times |
| Courier New, Courier | Liberation Mono, Courier New, Courier |

Script-name chains prefer Amiri/Noto Naskh Arabic/Geeza Pro for Arabic or Naskh
names, and installed Kai, FangSong, Song/Mincho or Hei families for the existing
Chinese-character name hints. Their exact lists remain in `preferred_substitute`;
this task does not change those choices.

OBSERVATION: `ScopedFonts::find_or_load` loads only the chosen installed family.
`ScopedFonts::find` prefers an exact style name or alias, otherwise the smallest
weight difference plus an italic-mismatch penalty. If resolution fails, `face`
tries Source Sans 3 in the requested style, Source Sans 3 Regular, the first
loaded face, then a last-resort face.

OBSERVATION: `replacement_face` begins with the resolved face and may choose a
better available face for character coverage. Composition calls `face` through
`StyleTable::intern` in `crates/compose/src/shape.rs`; the document's requested family is
not overwritten. The missing-family flag also enables per-glyph fallback.

OBSERVATION: after every successful `file.open`, `file.openIdml` or
`file.openBytes`, `DesignApp::run` in `crates/ui-egui/src/lib.rs` called
`font.list` and opened Find/Replace Font when a family or style was missing.
Publisher imports were not exempt. `menus::run_ui` handles the explicit
`app.findFontDialog` command separately.

## Selection policy

INFERENCE: names can supply a coarse substitution hint while the actual missing
font's dimensions remain unknown. Existing metric-compatible and script chains
keep priority. Other recognized names use serif, sans serif, monospaced or
display hints. The existing name classifier recognizes words such as Gothic,
Arial, Serif and Mono. Elephant and Sylfaen use a serif policy hint; Rockwell
uses the serif/slab group. The existing Times and Cambria chains already point
to serif families, so those names use the bundled serif when their preferred
candidates are absent; Calibri uses a sans hint. These are substitution policies,
not measurements of missing font files. Unknown names keep the previous Source
Sans 3 fallback.

OBSERVATION about the implementation: candidates come from the document's fonts,
loaded shared fonts, and the installed-font catalog. Their class, weight and
OS/2 width class are read from actual font files. The existing installed-font
scan already reads OS/2; it now retains those traits instead of rereading every
font for a missing-family lookup. The scan also reads the common 32-byte post
header so fixed-pitch-only fonts keep the same class before and after loading;
glyph-name arrays are not needed. Only the selected installed family is loaded.

INFERENCE: the family-name words Condensed/Narrow or Expanded/Extended provide a
width category, not a measured advance. Candidates with a corresponding OS/2
width class rank ahead of unknown or opposite widths within the same broad
type. Without a width hint, no target width is invented. Serif and slab serif
share a broad type for this choice. Italic and requested weight break further
ties; the existing Black weight-name convention is also used for Arial Black
when the requested style is Regular. Bundled families provide stable ties;
family and style names make remaining ties deterministic.

OBSERVATION: a selected condensed style is retained even when its typographic
family also contains normal-width styles. Exact requested families and their
style resolution keep the previous behavior. `has_family` still reports the
original as missing. No family/style name in the document is rewritten, so the
original font can be used if it is installed later.

OBSERVATION: Publisher imports already carry `DocState::import_notice`, set by
`open_pub` in `crates/engine/src/cmd/interchange.rs`. Automatic Find/Replace Font
is suppressed for those imports. The explicit command still opens it, and
native documents with missing fonts retain their existing automatic prompt.

## Limits

UNKNOWN: exact advances, kerning, widths, PANOSE numbers and glyph designs of the
missing Microsoft fonts. Substitution is approximate and can change line breaks
and text overflow. A serif hint for Elephant does not reproduce its display
letterforms. No font metrics were invented, no Microsoft font was downloaded,
and no protected importer reader was changed.

Not tested: Windows, system fonts on a real PC, or comparison with Publisher's
own drawing. The available-font ranking is tested here with bundled fonts,
synthetic test fonts, and the installed Linux font catalog.

## Render observations

OBSERVATION: `font.list` before and after import kept the same requested family,
style and character counts in all 19 successfully imported real files and the
controlled Arial Black fixture. The available fonts on this Linux machine gave:

| Requested family | Before | After |
| --- | --- | --- |
| Arial Black, Regular | Source Sans 3 Regular | Open Sans Extrabold |
| Elephant, Regular | Source Sans 3 Regular | Source Serif 4 Regular |
| Franklin Gothic Book | Source Sans 3, requested Regular/Bold | Source Sans 3, requested Regular/Bold |
| Rockwell Condensed, Regular | Source Sans 3 Regular | DejaVu Serif Condensed |
| Sylfaen, Regular | Source Sans 3 Regular | Source Serif 4 Regular |

OBSERVATION: the first pages of `SampleNewsletter.pub` and
`text__font-ArialBlack.pub` were rendered with the CLI before and after the
change, and both pairs were inspected. The controlled Arial Black text is
visibly heavier. Newsletter text reflows and some headings are clipped; its
visible headline now ends in `Amer-`. The CLI reported 1,809 rendered glyphs
before and 1,747 after on that page, while stored character counts stayed
unchanged. This is a visible layout difference, not a claim of improved
fidelity. The original font metrics and Publisher's drawing were not available
for comparison; no metrics were invented to force a visual match.

The PNGs are outside the repository under `/tmp/nifty-font-results/`, with
`before-` and `after-` prefixes and the input names above followed by `.png`.
Newsletter images are 595×842; the controlled images are 612×792.

## Validation

OBSERVATION: the untouched baseline was current main `5b7a851`, after the preset
shape change had merged. All 166 pinned `.pub` hashes and all 142 scoring JSON
hashes and sizes were rechecked against the fixture CSV and existing source
ledger. Scoring JSON was used only by the unchanged corpus scorer.

| Check | Before | After |
| --- | ---: | ---: |
| Controlled imports | 142/142 | 142/142 |
| Objects found | 152/152 | 152/152 |
| Scored shape/text positions within 1 pt | 111/111 | 111/111 |
| Exact texts | 52/52 | 52/52 |
| Styles | 111/111 | 111/111 |
| Real files converted with notes | 9 | 9 |
| Real files converted with warnings | 10 | 10 |
| Refused old-format files | 2 | 2 |
| Damaged files rejected | 3 | 3 |
| Real output pages | 31 | 31 |
| Real output objects | 280 | 280 |

All 142 controlled files also passed native save/reopen/edit/PDF before and after.
Every real file kept its baseline status, page count and object count. A separate
Newsletter native save/reopen check retained all requested families, styles and
character counts. No protected importer reader, dependency declaration, lockfile
or input fixture was changed.

The font unit tests use hand-stated type/width cases and synthetic fonts. They
check bundled-only choices, known-chain priority, exact-original precedence,
unknown names, loaded and installed condensed styles, and a fixed-pitch font
whose only classification signal is its post header, including a large table.
These are synthetic metadata tests, not measurements of absent Microsoft fonts.

The imported-font UI integration test uses the pinned controlled Arial Black
fixture outside the checkout. It replaces both documented UTF-16 family-name
occurrences in memory with an equal-length name confirmed absent in the current
font database. Real `file.openBytes` import then preserves that requested name
with `missing=true`, suppresses the automatic dialog, and still opens it through
the explicit command. The test passed when run explicitly with `PUB_CORPUS`;
it is ignored by default because its data is external. Existing native-document
automatic-dialog tests are retained.

An initial UI test draft failed because it assumed the minimal `hello-nifty.pub`
fixture contained an Arial name. It did not. That assumption was removed and
the external-fixture test above replaced it; no reader or disk fixture changed.

Final full workspace CI passed all seven steps: formatting, PDF dependencies,
workspace clippy with warnings denied, workspace tests, assets, layers and wasm.
The workspace runner reported 3,076 passed, 0 failed and 22 ignored. The controlled
scorer, 142-file roundtrip, 24-file real integration and imported-font UI test
were also run explicitly and passed. Ignored/manual/platform checks were not
silently treated as executed.

Commands, from the checkout root with the two corpus variables set:

```sh
cargo fmt --all -- --check
cargo xtask ci
cargo test --locked -p designcraft-pub --test corpus -- --ignored --nocapture
cargo test --locked -p designcraft-engine --test pub_corpus -- --ignored --nocapture
cargo test --locked -p designcraft-cli --test batch real_publisher_corpus_keeps_expected_status_counts -- --ignored --nocapture
cargo test --locked -p designcraft-ui-egui --lib publisher_import_keeps_missing_names_and_offers_replacement_only_on_request -- --ignored --nocapture
```
