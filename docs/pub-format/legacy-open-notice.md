# Experimental legacy File > Open notice and routing

OBSERVATION: this integration proposal uses the [v2 reader contract](legacy-minimal-reader.md), [reader review and manual trace](legacy-reader-review.md), [legacy-family census](legacy-97-2000-replication.md), and the repository's experimental reader at `88315cb0`. The inspected implementation is the owner's `crates/pub-legacy`, not an external reader. This document changes no code and claims no executed app or real-file test. The configured corpus folders are unavailable; no replacement data or external format descriptions were consulted.

OBSERVATION: [the earlier refusal audit](publisher-97-2000.md) records the existing message as `unsupported Publisher file: Publisher 97/98/2000 files are not supported yet; open and save the file in Publisher 2002 or later`. It records the current-family selector as `E8 AC 2C 00` and the older refusal selector as `E8 AC 22 00`. These distinguish observed layouts, not independently verified saving releases. Exact current UI wrapping/localization is UNKNOWN; preserve the actual refusal supplied by the unchanged importer rather than reconstructing it from this historical quotation.

## Routing rules

INFERENCE: the following is the required order. The application adapter creates any recovery pages/frames; the experimental reader itself continues to return only segments, provenance, diagnostics and a size candidate. The 2002+ importer and its success/error decisions remain unchanged.

1. INFERENCE: use the normal File > Open dispatch and run the existing 2002+ Publisher importer first for a Publisher candidate. If it succeeds, keep its document and warnings and stop. Never run the legacy path to replace, supplement or retry that success.
2. INFERENCE: consider the experimental path only after the importer has returned its explicit older-family unsupported outcome. Do not route on the `.pub` extension, filename, a generic parse failure or the words “unsupported” alone. If the application has only a displayed error string, the adapter needs a stable refusal discriminator; do not introduce a localized substring heuristic.
3. INFERENCE: additionally require a safely opened compound container and a uniquely resolved root Contents whose first four bytes are exactly `E8 AC 22 00`. Container failure, a short stream, an unknown prefix or `E8 AC 2C 00` keeps the original failure. Never send a damaged current-family file to the legacy reader as a repair attempt. The short `looks_legacy` probe is a routing hint, not admission or evidence of recoverable text.
4. INFERENCE: invoke the separate, explicitly labelled experimental text-recovery path on the same original bytes, within its budgets. The legacy reader must perform all contract gates itself: version-3 container, header and matching CLSID/table profile, bounded directory/records/graph, and the chosen text path. A/B require the parsed 00021201 UUID profile and table word 89/94; C requires parsed 00021200 and word 64, with the complete relationships in the contract. No filename, hash, plausible size or missing Quill stream selects a family. Keep unsupported/ambiguous profiles refused; never relax a gate to make a document.
5. INFERENCE: only an admitted result with independently validated decoded segments, retained encoding-unknown segments, or the explicit EmptyKnownStore state may produce a recovery document. Complete and Partial use the notices below. A TextRefused result, whole-file error, or Partial result containing neither decoded text nor validated undecoded ranges is a failed open: show the existing refusal plus the sanitized legacy reason and do not create a blank document. A size candidate alone is not a successful open.
6. INFERENCE: present EmptyKnownStore as an explicitly identified recovery state, not an error-free empty publication. A user may open the labelled empty recovery page with the empty-store notice. No failed or missing text catalog may be converted into that state. Preserve the currently open document on any refused open; do not replace it with a new empty page.

INFERENCE: keep the current refusal unchanged when the experimental path is unavailable/disabled, the failure was not the explicit older-family outcome, the safe prefix check cannot complete, or no supported legacy profile is admitted. If the legacy reader was actually attempted and refuses, append its reason even when its family remains unknown. Route failure must never become a string-carving fallback, a guessed encoding or another attempt through a different family.

## Notice drafts

INFERENCE: the quoted text in the following table is proposed user-facing copy, not a statement that these screens exist. Do not show the evidence labels as part of the notice. Use the short banner plus the shared detail text; keep both accessible after opening.

| Basis | State | Proposed text |
| --- | --- | --- |
| INFERENCE | Successful text-only recovery | “Experimental text recovery. Recovered text is ready to edit. This is a new document, not the original layout.” |
| INFERENCE | Partial recovery | “Experimental text recovery. Some text could not be recovered. Missing text is reported below; text we could not read is marked in the document. This is a new document, not the original layout.” |
| INFERENCE | Refusal | Keep the existing refusal verbatim, followed on a separate line by “Experimental text recovery could not open this file. Reason: {sanitized legacy reason}”. Show no recovery document. |
| INFERENCE | Empty known store | “Experimental text recovery found no text in the supported text store. This does not mean the original file is blank. The empty page is a recovery workspace, not an original page.” |
| INFERENCE | Shared detail for every state | “Original frames, fonts, formatting, page order, tables and pictures are not recovered. The order of recovered text may not be the reading order. Keep the original file. If possible, open it in Publisher 2002 or later and save a new copy.” |
| INFERENCE | Unreadable-segment placeholder | “[Text could not be read. No replacement text was guessed.]” |
| INFERENCE | Validated raw-size candidate used | “Page size is estimated from stored numbers. It has not been verified.” |
| INFERENCE | No usable size candidate | “Original page size is unknown. A standard A4 recovery page is being used.” |
| INFERENCE | Text cannot fit its generated frame | “Some recovered text extends beyond its recovery page. The full text is retained for editing.” |

INFERENCE: the partial-recovery details show decoded, undecoded and withheld counts separately when known, along with sanitized reasons. Do not invent a missing-character count from a failed range. Unknown-size or opaque-record warnings alone do not mean decoded text was withheld. Complete is limited to supported linked candidates, not a promise that all text in the publication was recovered.

UNKNOWN: exact original page count, reading order, physical size units, saving release, omitted embedded-object text and C's non-ASCII code page remain unverified. No notice may describe this as full Publisher import or imply those uncertainties were resolved by opening the file.

## Shape of the recovery document

INFERENCE: create an unsaved recovery document with a visible “Experimental text recovery” label. Never overwrite or automatically save the original. Keep the recovery report associated with the document and explain that generated frames/pages are editing containers, not recovered objects.

INFERENCE: create one editable text frame per decoded segment, preserving the reader's serialized output order. An A/B story reported once remains one frame even if several source records link to it. A C validated range with undecoded bytes receives one separate frame containing only the labelled placeholder above and source provenance, never guessed text. A segment withheld entirely has no invented range/frame; list the omission and reason in the report. If every retained segment is encoding-unknown, show Partial and placeholders, never EmptyKnownStore.

INFERENCE: place the generated frames sequentially on plain recovery pages using application-default text styling and margins. Start a further plain recovery page when needed, preserving segment order and one frame per segment. Generated page count, positions and line wrapping make no claim about original pages or layout. Retain a long segment's complete text even if its single frame cannot fit a page; expose the extension/overset with the notice above rather than truncate or silently hide characters. Do not split a segment merely to imply original pagination.

INFERENCE: use the validated raw root-size pair only when both values are positive and the reader supplied it after its metadata gates. Retain the original integer pair in provenance. Rendering requires an explicitly inferred conversion, raw/36000 millimetres (equivalently raw/12700 points); it does not turn raw units into a verified physical size. Apply the application's existing finite-number and page-dimension limits after conversion. If the candidate is absent, zero, invalid or outside those limits, use A4 portrait, 210 by 297 mm, as a generated recovery default with the unknown-size warning. Never claim that default came from the file. Do not clamp an invalid candidate to a seemingly plausible original size.

INFERENCE: EmptyKnownStore may produce one plain recovery workspace page, no content frames, and its persistent notice. A refusal produces no document even if a positive size candidate survives. Keep source ranges/IDs and undecoded status in metadata, not as private byte dumps. Preserve recoverable control data in the underlying segment; visible CR/LF paragraph presentation is editing behavior. FF or other stored controls must not be interpreted as evidence of an original page break or table boundary.

## Shared command-line and app report

INFERENCE: both consumers must report the fields below from the same reader result before document creation. Report counts refer to source segments, never generated page/frame totals. No paragraph, original binary data or guessed decoded text belongs in a batch report. This is a required report schema, not a claim that a command-line runner already exists.

| Basis | Field | Required meaning |
| --- | --- | --- |
| INFERENCE | reader_revision; route | Pinned implementation revision; `current-importer`, `experimental-legacy`, or `current-refusal`. Never label current-importer output as legacy. |
| INFERENCE | family | `experimental legacy A`, `experimental legacy B`, `experimental legacy C`, or `legacy-family-unknown` on refusal before admission. |
| INFERENCE | scope | `Complete`, `Partial`, `EmptyKnownStore`, `TextRefused`, or `Refused`; preserve the reader scope, except an adapter refusing a no-recoverable-content open records its refusal separately. |
| INFERENCE | open_result | `recovery-document`, `empty-recovery-workspace`, or `refused`, distinct from reader scope. A Partial result with no retained segments can have open_result refused. |
| INFERENCE | segment_count | Number of returned validated segment objects, including retained undecoded ranges. |
| INFERENCE | decoded_count | Number with decoded text present, including its stored controls. |
| INFERENCE | encoding_unknown_count | Number explicitly marked encoding-unknown; do not equate every missing string with unknown encoding. |
| INFERENCE | undecoded_count | Number with decoded text absent. For the documented C trace it equals encoding_unknown_count. |
| INFERENCE | withheld_count | Number of entirely omitted segments if independently countable, otherwise `unknown`. Failed link attempts, unlinked catalog IDs and duplicate references are separate counters, not assumed distinct missing stories. |
| INFERENCE | raw_size_pair | Checked positive integer width/height candidate, or `none`; no default substituted here. |
| INFERENCE | size_status; recovery_page_size | `inferred-from-raw`, `default-A4`, or `unavailable` when no document is created; generated page width/height and explicit mm units when applicable. Preserve raw candidate even if rendering limits force a default. |
| INFERENCE | warnings; refusal | Ordered diagnostics carrying v2 category, family, gate, field, record_id-or-none, recovery_scope, result_scope and sanitized reason. Preserve original importer refusal separately and append legacy reason on a failed experimental attempt. |
| INFERENCE | provenance | Per-segment ID type, source ranges and units; A/B story ID plus every linking record ID, or C map ID/target ID/byte range. Record storage order explicitly, never reading order. |

INFERENCE: require segment_count = decoded_count + undecoded_count and encoding_unknown_count <= undecoded_count. Zero decoded text with retained unknown ranges is Partial, not empty. Refused results must not pretend to have validated zero counts: use `unknown` for unavailable segment/count/size fields. TextRefused returns zero segments but says text was not recovered, not that no text exists. EmptyKnownStore has zero validated segments and zero decoded/unknown counts.

OBSERVATION: the existing manual trace predicts C segment counts 28/72/74/88/30, decoded counts 28/64/65/78/29 and unknown counts 0/8/9/10/1. A predicts six decoded Sample98 stories and EmptyKnownStore for mspub/zmspub. These remain manual expectations; they are not fresh execution results for this revision or the forthcoming app path.

## Re-review of the remaining gates at 88315cb0

INFERENCE: these classifications compare the pinned source with contract v2, not with test names. MATCH means the required behavior is present, STRICTER means additional rejection, and LOOSER means at least one requirement remains unimplemented or its delegated guarantee remains unverified. Only the three requested gates are re-reviewed; the earlier review remains pinned to its earlier revision.

| Basis | Gate | Result | Source evidence; exact remaining check |
| --- | --- | --- | --- |
| OBSERVATION / UNKNOWN | G01 | LOOSER | `open_cf/read_stream` explicitly enforce V3, file/declared-stream budgets and delivered-length equality. The Quill walker now rejects a block pointer outside its bounded stream before adding field offsets; the parent walk uses active/settled state with constant-time membership instead of a growing-path search. Remaining unestablished check: unique root Contents resolution and rejection of ambiguous container directory entries. The crate directly accepts `cf.is_stream/entry/open_stream` resolution without an independent ambiguity check or a reviewed dependency guarantee. Complete FAT/directory-chain validation is also delegated, not independently verified here. This is an assurance gap, not an observed accepting exploit. |
| OBSERVATION | G08 | LOOSER | Exact consumed profiles and equations remain enforced. A new opaque count covers known indices outside `[0,1,8,10,21,26]`, but this union is not the actual consumed-record set. For example an A record at index 0 or a C record at index 8 is excluded from the opaque count although its logical extent is never validated; an unlinked C index-0/1 target is also excluded. Missing check/report: track records actually validated by the active family and dependency walk, and count/warn about every other bounded known-index record whose logical extent was not validated. The index-33 test covers only the outside-union case. |
| OBSERVATION | O01 | LOOSER | C provenance now contains the source map ID and target ID/range. Alternate Quill presence now uses `quill_present` with `is_stream` and reads no stream payload; this closes the earlier alternate-store issue. Remaining missing output: every diagnostic's explicit v2 category, gate, family-or-unknown, structural field/record ID, recovery_scope and result_scope. `LegacyError` variants and warning strings do not supply that complete schema, and dependency error text is forwarded without a demonstrated sanitization boundary. A document adapter must not invent missing gate/record facts by parsing free text. |

OBSERVATION: source inspection finds genuine improvements in all three areas; it does not establish that every requested contract obligation is complete. The reviewed tests exercise an outside-union opaque record and an out-of-stream Quill pointer, but they were not executed in this docs task. No synthetic or real-file success is claimed from a test's name or presence.

INFERENCE: the standard recovery-page fallback above belongs solely to the app's editing workspace. It does not weaken G09: the reader still withholds invalid size metadata and never fabricates an original A4 size. Likewise generated frames do not add geometry recovery to the text-only reader contract. Routing and notices require their own integration verification before being described as implemented.

UNKNOWN: the exact current refusal wrapper, app page-dimension limits, eight-file runtime outputs, malformed-container ambiguity behavior and forthcoming File > Open integration have not been verified here. Keep those limitations visible in the integration report. B remains single-file FRAGILE, A populated-text links remain single-file FRAGILE, and C's code page remains UNKNOWN; these UI decisions do not improve that evidence coverage.
