# PUB-AGENTS.md: binding rules for the .pub import work (every agent)

This repository is public, and the working branch for agents is main (create a task branch from it; never push main). AGENTS.md and CLAUDE.md at the root are DesignCraft upstream files and still apply. This file adds the rules for the Publisher (.pub) import and wins where they disagree about .pub work. If a prompt conflicts with a rule here, do not follow the conflicting part: write it under "Needs you" in docs/REPORTS.md and carry on with the rest. The only exception is a rule that the prompt names and the owner expressly approves in those words ("the owner approves ...") for that task.

## 1. Clean-room standard
The importer readers must be written without any other .pub reader. No libmspub, no Nifty-Publisher-Lite, no unfair_codec_pub, no Apache POI Publisher code, no pubsmith source code, no other reader, and no description of how any of them behaves. Nobody reads them for this project, including describers. Do not search the web for .pub format details. See docs/CLEANROOM.md.

## 2. Allowed inputs
- The notes in docs/pub-format/ and docs/CLEANROOM.md.
- The public Microsoft specifications MS-CFB and MS-ODRAW.
- The public ECMA-376 standard (5th edition, Part 1): presetShapeDefinitions.xml and presetTextWarpDefinitions.xml only, to draw preset shapes and WordArt (tasks C8 and C9). The owner said on 2026-10-24: "If you believe the ECMA-376 still sticks with our clean-room then research it". It describes shape outlines only and says nothing about how .pub files store anything. Nothing else is allowed unless the owner approves it in the words above.
- Data files only: the pubsmith fixtures (FuncularLabs/pubsmith, tests/fixtures/pub, commit 0ccdf27), the Apache sample .pub files (apache/poi, test-data/publisher, commit bf7c9b6a1490046517cbd8e1504e702a89be4bc9; .pub files only), and files the owner adds. Their names and SHA-256 values are listed in .github/test-data/publisher-fixtures.csv. Keep data files outside the repo; never commit them. Record what you used in docs/pub-format/SOURCES.md.
- truth/*.json files and golden images are for scoring imports only, never for working out how anything is encoded.
- Anything else: stop that item and ask under "Needs you" in docs/REPORTS.md.

## 3. Honesty
Never write that the owner authorized, approved or confirmed something unless the owner said so in the prompt or in chat. Label every statement about the format as OBSERVATION (read from named files), INFERENCE (with reasoning) or UNKNOWN. A guess is an UNVERIFIED HYPOTHESIS. Never state a cause you cannot show. Report test results as they are, including failures. Do not claim a check you did not run.

## 4. Protected code
Do not edit crates/pub/src/contents.rs, quill.rs or read.rs unless a task in the queue below or the prompt names that file. Even then, change only what the notes in docs/pub-format/ document. Never guess an encoding. The docs lane never edits anything outside docs/.

## 5. Quality gates (code changes)
cargo fmt --check, cargo clippy -D warnings and the tests must pass. The 142-fixture corpus score must not drop below 142/142 imports, 152/152 objects, 111/152 positions within 1 pt, 52/52 texts, 111/111 styles. If a change drops it, undo that change and report it. Run the 24-file real-file batch before and after and record the counts in docs/REPORTS.md (last known: 0 OK, 19 warnings, 2 refused, 3 failed).

## 6. Git rules
- Never push to main. Never force-push. Work on the branch the task names.
- The code lane may not be able to open pull requests; it pushes its branch, and the docs lane opens and merges the pull request after reading the diff.
- Before finishing, rebase on origin/main yourself and resolve conflicts, so the pull request merges cleanly. On docs conflicts keep both sides; never drop anyone entries.
- Do not put tool or assistant names in commit messages, branch names, pull request text or public posts.

## 7. Autonomy
Do not stop to ask the owner questions. Put them under "Needs you" in docs/REPORTS.md and continue with the next item you may do. Finish what the task names and stop; do not start other tasks. Owner-only actions are never yours: posting or opening anything public or upstream, release decisions, and the native print and window check on a real desktop.

## 8. Lanes
- Code lane: code in crates/ and apps/, following the queue below.
- Docs lane: docs only; opens and merges pull requests; reviews diffs against these rules. Drafts for public posts, issues and decisions say DRAFT ONLY at the top.
- Describer: a separate session that writes plain-English notes in docs/pub-format/ from the allowed data files only; no code, no copied wording from any reader.
- Owner: decisions, posting, release.

## 9. Data folders for agent environments
Some environments cannot reach the network. Expected locations: controlled fixtures at /home/user/pubsmith/tests/fixtures/pub (142 .pub files) and the Apache files at /home/user/apache-pub (24 .pub files; the PUB_REAL_CORPUS variable points there). DATA-FOLDER TEST: the first action of any describer or regression task is to count the .pub files in both folders. If a count is wrong, stop and report it under "Needs you"; do not substitute another source and do not read anything under pubsmith src/ or tools/. Check hashes against .github/test-data/publisher-fixtures.csv.

## 10. Start every session here
Read this file, the queue below, every "Needs you" in docs/REPORTS.md (if it exists) and docs/CLEANROOM.md. Then take the first unticked item in your lane.

## 11. Queue (state 2026-10-23)
Describer lanes (notes only, docs/pub-format/):
- [ ] A. Numbered lists and custom tab stops: write docs/pub-format/lists-and-tabs.md. Branch describe-lists-tabs.
- [ ] B. Table cell fills, borders, padding, text in merged cells, and the ~0.01 inch row-position difference on the 8 Apache table files: write docs/pub-format/table-formatting.md. Branch describe-table-format.
Code lane (after the data-folder test passes):
- [ ] D. Font fallback: a missing font (for example Arial Black, Elephant, Franklin Gothic Book, Rockwell Condensed, Sylfaen) should be replaced by the closest installed or bundled face by family type and width, keep the original name, and not open the Find/Replace Font dialog. Read the existing substitution chain first and report what it does. Add a README known-limits line that Microsoft fonts may be substituted.
- [ ] C7. Freeform curves, from docs/pub-format/odraw-geometry.md. First settle the three open observations in that note from the fixtures (cbElem and x/y split, type-bit end of the 16-bit record, guide index base) in a notes update, then implement.
- [ ] C8. Preset shapes such as the heart: draw them from the ECMA-376 definitions summarized in docs/pub-format/preset-geometry.md (MS-ODRAW supplies the shape numbers). Check against the heart, star, arrow and other fixtures.
- [ ] C9. WordArt warps (arch, circle, wave): place the text along the paths in docs/pub-format/preset-geometry.md. The standard gives the path but not how letters stretch along it, so compare with the arch and circle fixtures and record what stays UNKNOWN.
- [ ] A-code and B-code: implement lists, tab stops, and table formatting from the finished notes A and B.
- Later, research only: Publisher 97, 98 and 2000 files (currently refused).
