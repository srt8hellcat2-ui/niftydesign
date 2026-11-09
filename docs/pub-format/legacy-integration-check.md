# Legacy open integration check

OBSERVATION: this review compares [the open-notice proposal](legacy-open-notice.md) and [contract v2](legacy-minimal-reader.md) with the owner's integration at `f2d8ef9f`, including the subsequent notes change at `9b4cf49f`. Source conclusions below are pinned to `9b4cf49fb111c6e7ceb8eaf988a49f0954839ed8`. Inspected sources include `crates/engine/src/cmd/legacy_open.rs`, its `interchange.rs` caller, `crates/engine/tests/legacy_open.rs`, the reader's routing probe, the CLI command/report paths and the UI notice renderer. No external reader, third-party format description or prohibited repository was consulted.

OBSERVATION: this is a source inspection, not an executed integration, GUI or corpus run. The configured corpus folders are unavailable. Existing notes supply the manual expectations; no new sample facts, binaries or private text are committed. No code under crates or apps is changed.

INFERENCE: MATCH means the specified behavior is present in the inspected path; STRICTER means additional rejection; LOOSER means a missing guarantee or weaker behavior. A MATCH does not establish runtime correctness. UI presentation and report serialization are considered separately from the engine's return value.

## Routing comparison

| Basis | Proposal rule | Result | Exact behavior or difference |
| --- | --- | --- | --- |
| OBSERVATION | Routing 1: current importer first, success wins | MATCH | `interchange::open_pub` calls `import_pub_with` first; a successful import bypasses legacy recovery. The current-family integration test checks absence of `legacy` and the existing conversion notice. |
| OBSERVATION | Routing 2: explicit older-family refusal, not generic Unsupported | LOOSER | The guard accepts any `PubError::Unsupported(why)` when `looks_legacy(bytes)` is true. It does not distinguish the older-family unsupported cause from another Unsupported cause. The prefix narrows the path, but does not implement the proposed stable refusal discriminator. No substring heuristic is used. |
| OBSERVATION / UNKNOWN | Routing 3: safe container and exact old prefix | MATCH | `looks_legacy` opens bounded V3 root Contents and checks all four old discriminator bytes; failures stay on the existing error path. The proposal's unique-name guarantee still depends on container resolution, as noted in the earlier G01 review; this source inspection does not independently establish that dependency guarantee. |
| OBSERVATION | Routing 4: separate gated reader, same bytes | MATCH | `open_legacy` calls `read_legacy(bytes)` with the original bytes. It does not choose a family by filename or change a gate. Earlier reader-contract assurance gaps are not closed by this adapter. |
| OBSERVATION | Routing 5: no blank recovery on refused/empty Partial result | MATCH | A reader error returns an error. Zero retained segments refuses unless scope is EmptyKnownStore; TextRefused normally has zero segments and therefore refuses. A surviving size alone cannot create a document. |
| OBSERVATION | Routing 6: explicit empty workspace; failed open preserves existing document | MATCH | EmptyKnownStore creates one page and no frames with its distinct notice. `s.add_document` occurs only after successful document construction; refusal does not replace the session document. Existing tests cover no-document refusal, not preservation of a pre-existing edited document. |
| OBSERVATION | Unavailable/disabled experimental path keeps refusal | LOOSER | No separate integration enable/disable control is present in this path. It always attempts recovery when the guard matches. An explicitly labelled result exists, but there is no tested disabled-path behavior. |
| OBSERVATION | Failed attempted recovery preserves original refusal and appends reason | LOOSER | Common errors contain the same original wrapper and Unsupported reason plus the proposed experimental line. Reader errors use their string; zero-result failure selects a warning containing `refused` or falls back to “no text was recovered”, potentially dropping a more specific non-refusal warning. Document-construction errors use `bad(file.open,...)` instead of the original refusal wrapper. No sanitization boundary for dependency error strings is demonstrated. |

INFERENCE: the session is not mutated before successful construction, but an app-level error dialog still needs native verification. A safe old-prefix probe is not evidence that the whole file passed legacy admission. A modern-prefix parse failure remains on the current importer path, without legacy repair.

## Notice comparison

| Basis | Proposal copy or behavior | Result | Exact difference |
| --- | --- | --- | --- |
| OBSERVATION | Successful banner plus shared detail | MATCH | `NOTICE_OK` contains the proposed successful copy and the shared missing-layout/original-file advice. It is retained in `DocState.import_notice`. |
| OBSERVATION | Partial banner | MATCH | `NOTICE_PARTIAL` contains the proposed text. Any non-Complete/non-empty admitted scope with retained segments selects it. |
| OBSERVATION | Partial details/counts | LOOSER | Returned JSON has decoded/unknown counts, but Import notes do not display those counts together or count entirely withheld stories. The generic notice says missing text is reported below even when no distinct omission count exists. |
| OBSERVATION | Refusal copy | LOOSER | The reader/text-path failure case uses the requested appended sentence. Specific-reason preservation, document-construction error wrapping and sanitization have the routing gaps above. |
| OBSERVATION | Empty known-store copy | MATCH | `NOTICE_EMPTY` explicitly distinguishes a supported empty store from a blank original file. |
| OBSERVATION | Undecoded placeholder | MATCH | Every returned segment with `text=None` gets exactly “[Text could not be read. No replacement text was guessed.]”. Its provenance is in the response, not decoded from its bytes. |
| OBSERVATION | Estimated-size and fallback-size warnings | MATCH | The two proposed sentences are added to warnings and retained in import notes, according to `size_status`. |
| OBSERVATION | Long-text warning | LOOSER | The text matches, but its trigger is the same approximate character/line estimator used for placement, not measured composition. Real overflow can be missed or falsely reported. |
| OBSERVATION | Persistent, accessible notice/details | MATCH | UI `app/frame.rs` displays `import_notice` above the document and warnings in collapsible Import notes in ordinary editing mode. Presentation mode suppresses this chrome; native accessibility, clipping and visibility have not been tested here. |

## Document-shape comparison

| Basis | Proposal rule | Result | Exact behavior or difference |
| --- | --- | --- | --- |
| OBSERVATION | Unsaved recovery; original never overwritten | MATCH | `DocState::new(d,None)` leaves the document unsaved; file-path opens suggest a `.designcraft` sibling save path. Base64 opens have no suggested source path. The integration test asserts unsaved state only for base64. |
| OBSERVATION | One frame per retained segment in storage order | MATCH | The adapter iterates `r.segments` once and creates one frame per item. Duplicate A/B provenance does not create another frame. Frames use default paragraph formatting, not original styling. |
| OBSERVATION | Undecoded-only result | MATCH | With retained segments, no decoded-text minimum is imposed. All-unknown C input therefore opens Partial, with one placeholder per segment, not EmptyKnownStore or refusal. This path has no engine integration test. |
| OBSERVATION | Plain pages and sequential placement | MATCH | Frames are stacked with 36-point margins and 8-point gaps; further pages are generated as needed. Pages are non-facing and have no automatic primary frame. This is generated layout, not recovered page order. |
| OBSERVATION | Raw candidate conversion and positive usable page | MATCH | Only the reader's checked candidate is used. The adapter converts inferred mm to points, equivalent to raw/12700, and admits each dimension from 72 through 15552 points. Otherwise it uses approximately A4 portrait, 595.276 by 841.89 points, with a warning. The actual numerical limits are now explicit; this review does not establish that they are shared application limits. |
| OBSERVATION | Frames fit generated page margins | LOOSER | Body width/height are each forced to at least 36 points. A permitted 72-point dimension leaves zero space between the two 36-point margins, yet a 36-point frame body is used. Frames can therefore extend past an accepted recovery page. |
| OBSERVATION | Full long text retained; no silent truncation | LOOSER | The visible ordinary text is stored even when estimated height is capped, but `visible()` drops controls below U+0020 except tab/CR/LF, converts paragraph ends and trims all trailing LF. The original segment/control sequence is not retained in the document metadata. This loses data the proposal required preserving; provenance alone cannot reconstruct it. Overflow detection is approximate. |
| OBSERVATION | No fabricated marker-based pagination | MATCH | CR/LF become editable paragraphs; FF does not cause a page break. Removing FF is nevertheless the preservation gap above. |
| OBSERVATION | Empty workspace versus refusal | MATCH | EmptyKnownStore yields one page/no frames. Errors and no-recoverable-content outcomes never reach session insertion. |
| OBSERVATION | Bounded generated layout | STRICTER | The adapter refuses when generated page index reaches 2000, adding a workspace limit absent from the notice proposal. It does not silently drop the remaining segments. |

UNKNOWN: the exact rendered font metrics, editable visibility of overflowing text, destination save dialog and notices on a native Windows build are not established by the approximate placement code or source-level tests.

## Report-field comparison

OBSERVATION: `open_legacy` returns a nested `legacy` object and top-level `warnings`, `notice` and empty `notes`. A CLI `run --cmd file.open=...` prints the command result, whereas `run --in ...` discards the open result. Batch `Outcome::add_warnings/value` retains warnings/notice/notes but drops the entire legacy object; folder conversion builds on that reduced outcome. A legacy-family name in a warning is not a substitute for the requested typed fields.

| Basis | Required field | Engine command result | Batch/folder report | Exact difference |
| --- | --- | --- | --- | --- |
| OBSERVATION | reader_revision | LOOSER | LOOSER | Missing in both; record the tested build externally. |
| OBSERVATION | route | MATCH | LOOSER | `legacy.route` is experimental-legacy; batch/folder discard it. |
| OBSERVATION | family | MATCH | LOOSER | Exact family label exists on success; a refused result has only an error string, not typed unknown-family data. Batch/folder omit the field. |
| OBSERVATION | scope | MATCH | LOOSER | `legacy.scope` is the debug-form scope on success; no typed refusal report exists. |
| OBSERVATION | open_result | MATCH | LOOSER | `legacy.openResult` distinguishes recovery-document and empty-recovery-workspace; refusal returns an error rather than a structured refused object. |
| OBSERVATION | segment_count | MATCH | LOOSER | `segmentCount=r.segments.len()`; unavailable on refusal. |
| OBSERVATION | decoded_count | MATCH | LOOSER | `decodedCount` counts `text.is_some()`. |
| OBSERVATION | encoding_unknown_count | MATCH | LOOSER | `encodingUnknownCount` counts explicit flags, independently from text absence. |
| OBSERVATION | undecoded_count | MATCH | LOOSER | `undecodedCount=segmentCount-decodedCount` guarantees the requested sum invariant on every successful response. |
| OBSERVATION | withheld_count | LOOSER | LOOSER | No value, including no explicit unknown; failed links/unlinked IDs/duplicate counts are not separate typed counters. |
| OBSERVATION | raw_size_pair | MATCH | LOOSER | `rawSizePair` retains the reader's pair or null; no default is substituted into it. |
| OBSERVATION | size_status and recovery_page_size | LOOSER | LOOSER | `sizeStatus` is present, but `recoveryPageSize` reports points, while the proposal requests explicit mm. Values are convertible; this is a schema-unit difference. No unavailable/refused size object is emitted. |
| OBSERVATION | warnings and refusal diagnostics | LOOSER | LOOSER | Warnings survive in both paths, but remain strings without every v2 diagnostic field. Refusal wrapping/reason gaps are listed above. The notes-once fix returns empty notes so batch does not duplicate the warnings there. |
| OBSERVATION | provenance and storage-order label | MATCH | LOOSER | Per-segment story/range/records or target/map/range plus decoded status are returned, and `storageOrderOnly=true`; batch/folder omit them. |

INFERENCE: MATCH report cells refer only to admitted engine results. The proposal's typed `Refused`/`TextRefused` report and unknown-valued unavailable counters are LOOSER for every relevant field on a failed open. In particular a JSON command failure must not be described as a validated zero-story publication. Per-segment encoding-unknown flags would also make auditability stronger than the current aggregate plus decoded flag.
