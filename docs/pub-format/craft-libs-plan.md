# Publisher 2002+ split plan

## Inventory

OBSERVATION — The inventory covers all 13 source modules in `crates/pub/src/` at `000a00af344cc54b391431328f2c932c4abe37c6`. Counts include comments, blank lines and inline tests (5,894 lines total).

| Label | Module | Lines | Role and direct dependencies beyond the standard library and local modules |
|---|---|---:|---|
| OBSERVATION | `lib.rs` | 1,533 | Mixed. Compound-file probe and stream orchestration use `cfb`. Document builder uses DesignCraft document, geometry, color and image APIs, the local metafile converter, `serde_json` and `log`. Error derive uses `thiserror`. |
| OBSERVATION | `read.rs` | 109 | Format reading. Bounded little-endian and property-list helpers. |
| OBSERVATION | `contents.rs` | 232 | Format reading. Contents structure, pages, masters and objects. |
| OBSERVATION | `quill.rs` | 570 | Format reading. Text, fonts and runs. Uses DesignCraft capitalization and position enums. |
| OBSERVATION | `escher.rs` | 400 | Format reading. Drawing records, shapes, groups and picture store. |
| OBSERVATION | `tables.rs` | 922 | Mixed. Table metadata, cell text spans and recovery, then table/story construction and fit. Uses DesignCraft document, geometry and composition APIs. |
| OBSERVATION | `paths.rs` | 221 | Mixed. Decodes freeform drawing data and builds DesignCraft paths. |
| OBSERVATION | `presets.rs` | 146 | Geometry mapping. Local preset definitions produce DesignCraft paths. |
| OBSERVATION | `wordart.rs` | 219 | Geometry mapping. Default WordArt curves produce DesignCraft paths. |
| OBSERVATION | `metafile.rs` | 1,135 | Mixed. Unpacks Escher blips using bounded `miniz_oxide` inflation, then uses local WMF/EMF-to-SVG code. Inline tests use DesignCraft image APIs. |
| OBSERVATION | `overset.rs` | 112 | Document mapping diagnostics. Uses DesignCraft document and composition APIs. |
| OBSERVATION | `preview.rs` | 30 | Older-file adapter from bare metafile to SVG through the local converter. |
| OBSERVATION | `story.rs` | 265 | Older-file adapter from Quill to formatted frames. Uses DesignCraft document and geometry APIs and `serde_json`. |

OBSERVATION — `crates/pub/Cargo.toml` declares five DesignCraft dependencies (geometry, color, document, composition and images), plus `cfb`, `miniz_oxide` 0.8, `thiserror`, `log` and `serde_json`. The 2002+ reader uses UTF-16 decoding and has no local single-byte code-page tables in these modules.

## Dependency map

OBSERVATION — The craft-libs snapshot is `33be35625e252cc4f06aa375b3b428521faea37d`. Its crate directories are `crates/cfb`, `crates/codepage`, `crates/metafile`, `crates/file-formats`, `crates/file-import` and `crates/sniff`. The package manifests and API sources exist for all six. Crate READMEs exist for the first three. [Source tree](https://github.com/storytold/craft-libs/tree/33be35625e252cc4f06aa375b3b428521faea37d/crates)

- OBSERVATION — `craft-cfb` offers borrowed `CompoundFile::open_with`, `has_stream`, `find`, `entry` and `read_path_capped`. Its directory name scan cannot validate stream contents. The public stream reads return a whole `Vec<u8>` and the open limits cover stream length and directory entries.
- INFERENCE — Replace `cfb` for the 2002+ read path. Preserve missing optional streams versus missing required streams, and carry limit errors separately from corruption. Add a bounded root stream prefix read for detection, including mini streams and bounds on container traversal and scratch memory. A small whole-stream cap would reject valid large Contents streams.
- UNKNOWN — Corpus parity for path lookup, chain validation, malformed files and mini streams, plus actual peak container memory.
- OBSERVATION — `craft-codepage` provides `cp1252`, `windows_byte` and `code_page_byte`, including Mac Roman. The 2002+ path has no table to replace.
- INFERENCE — Keep UTF-16 and its source offsets in the parser. Use shared byte decoding only where an established single-byte encoding is needed.
- OBSERVATION — `craft-metafile::parse` takes bare bytes and gives a neutral `Picture` with drawing items and omission counters. `size`, `unit_facts` and `Picture::place` expose sizing and placement. The public reader has fixed internal budgets, no SVG output and no input for Escher blip bounds or size.
- INFERENCE — Keep bounded blip unpacking and `miniz_oxide` in `craft-pub`. Replace the 2002+ local drawing interpreter with `craft-metafile`. Carry Escher bounds and size alongside the picture and preserve their fallback for headerless sizing. Keep SVG serialization or native image mapping in DesignCraft. If measurement requires it, add caller-controlled parse budgets and a sizing hint at the shared metafile boundary.
- UNKNOWN — WMF/EMF placement, crop, diagnostics and image parity, including drawings above the shared 4,096 object-slot cap (local cap 65,536). Measure retained bytes and transient decode cost across multiple pictures.
- OBSERVATION — `craft-file-formats` resolves content and extension hints. `craft-file-import` dispatches vector and raster payloads. `craft-sniff::identify` uses a 4,096-byte prefix.
- INFERENCE — Use the format resolver and import registry for Publisher. Keep Publisher identification out of prefix-only sniffing because the compound-file signature identifies the container and the required stream may sit beyond the prefix.

## Proposed crate

INFERENCE — The owner suggests `craft-pub`. The maintainer decides the name. Move Contents, Quill, Escher, stream reading, table recovery, freeform decoding and blip unpacking into a shared crate. Replace the Quill enums and any table text offsets tied to DesignCraft line-break strings with neutral types. Move source-specific preset and WordArt interpretation behind neutral path commands where needed. Keep raw record structures private.

INFERENCE — Sketch `read(bytes: &[u8], limits: Limits) -> Result<Publication, Error>`. `Publication` carries pages and masters, objects in paint order, grouped shapes and transforms, stories with source UTF-16 run ranges and styles, tables with spans and cell text, and image assets. Use stable source IDs for references, explicit EMU geometry and typed path commands, paint, font and paragraph data. Use structured diagnostics with stable codes and optional source locations. Keep DesignCraft types out of the API.

INFERENCE — Expose `Error::code() -> ErrorCode` with stable codes such as `not_publisher`, `unsupported_version`, `missing_stream`, `malformed`, `truncated` and `limit_exceeded`. Keep detail and stream/offset context separate from the code. Preserve the existing older-format handoff via a typed refusal and the existing DesignCraft discriminator.

OBSERVATION — Existing ceilings include 512 MiB input, 256 MiB per stream, 64 MiB unpacked per metafile, 32 MiB retained SVG/path output per conversion, one million counted records/items per local count check, depth 16 for Escher and groups, 10,000 table cells and 100,000 table-text directory entries.

INFERENCE — Start with those ceilings where applicable. Add a proposed 512 MiB aggregate working-memory ceiling excluding borrowed input, charged before allocation for container metadata, stream copies, model data and decoded assets. Include scratch allocations and work across pictures, free streams when done, and cap diagnostic entries. Preserve the shared import registry's 64 MiB default for its existing callers and pass explicit Publisher limits from DesignCraft. Apply the lower applicable limit before detection and decode. UNKNOWN — The aggregate budget and the shared metafile limits need measurement against the corpus and large-file cases.

INFERENCE — DesignCraft builds `Document`, assigns IDs, maps color and fonts, creates image assets, fits tables and measures overset. It handles document-local font resources, converts structured diagnostics to warnings and notes, and owns the converted-copy notice and save path.

## Registration

- INFERENCE — In craft-libs `crates/file-formats/Cargo.toml` and `src/lib.rs`, add a read-only Publisher capability and a bounded CFB probe. Add `Format::Publisher` to `ALL`, `extensions`, `mime_type` and `capabilities`. `from_extension` then picks up case-insensitive `.pub`. In `detect`, match the modern root Contents discriminator before extension fallback, preserving stronger native and IDML matches. Use the suffix as a hint so damaged or unsupported named files get a typed explanation. Test unnamed, misnamed, generic CFB, short-prefix and damaged inputs in `src/tests.rs`.
- INFERENCE — In craft-libs `crates/file-import/Cargo.toml` and `src/lib.rs`, add a read-only Publisher dependency and entries in `ImportFormat`, `CAPABILITIES` and `format_for`. Have `import` dispatch to the bounded reader and return a publication variant of `Imported`, with the typed error preserved. Check exhaustive consumers, including `src/write.rs::writer_format_for_extension`. Keep detection below dispatch in the dependency graph.
- OBSERVATION — In this fork, `crates/engine/src/cmd/file/bytes.rs::is_document`, `crates/engine/src/cmd/file/open.rs::open_path` and `file_open_bytes` call `designcraft_pub::is_pub`. `crates/engine/src/cmd/interchange.rs::open_pub` calls `import_pub_with` and owns the session conversion state.
- INFERENCE — Change those three detection call sites to shared classification. Keep `bytes.rs::is_document` bounded for prefix callers and let full-byte open handle files whose directory sits later. Update `open_pub` to consume the registry result with the original bytes and name, then call the local mapper with the existing font setup. In `crates/pub/src/lib.rs`, make `import_pub` and `import_pub_with` delegate to shared reading and local mapping for direct callers. Add pinned dependencies in root `Cargo.toml`, `crates/engine/Cargo.toml` and `crates/pub/Cargo.toml`, and update `Cargo.lock`. Keep explicit Publisher commands, CLI behavior and older-file routing covered by their existing tests.

## Two pull requests

1. INFERENCE — First, start from current craft-libs main. Carry only the 2002+ clean-room reader and its provenance into the shared workspace. Add neutral types, budgets, typed errors, bounded detection and registry dispatch. Verify the container swap and metafile output independently with allowed fixtures before merging.
2. INFERENCE — Then start a small branch from a clean checkout of current upstream DesignCraft main. Identify the importer-only diff against the original upstream base, review and trim it, and apply it to that checkout. Drop fork branding, release work, unrelated history and older-reader changes from the patch. Apply only the app mapping and integration once the shared revision exists. Pin the merged craft-libs revision in `Cargo.toml` and update the lockfile. OBSERVATION — The fork's `craft-pdf` entry already uses an exact git revision and version.
3. OBSERVATION — The gates are 142/142 imports, 152/152 objects, 111/111 scored positions within 1 pt, 52/52 texts and 111/111 styles. INFERENCE — Run those gates on the baseline and after each reader or mapping swap. Run format, lint and tests, native and web open checks, and the 24-file real-file batch before and after. Record counts in `docs/REPORTS.md` and revert any change that drops a gate. Use truth data only for scoring.
4. INFERENCE — Smallest next step. On a local craft-libs branch, prototype the bounded root Contents prefix read with synthetic regular- and mini-stream cases, then compare its classification with the current probe on the allowed corpus.

## Out of scope

INFERENCE — Keep Publisher 97/98/2000 recovery outside the split. Keep existing `preview.rs` and `story.rs` adapter behavior in DesignCraft and preserve their call contracts while moving the 2002+ reader.
