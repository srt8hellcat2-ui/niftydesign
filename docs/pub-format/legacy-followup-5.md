# Legacy follow-up: larger-set structural evidence

OBSERVATION: this note records the owner's structure-only run over 52 tester inputs. No document text was printed in that run. Only the supplied aggregate numbers are used here; no file identity, document content or personal detail from the set is recorded. No private file was requested, transferred or run here, and no other Publisher reader, external format description or prohibited repository was consulted.

OBSERVATION: this branch starts from the [round-4 documentation](legacy-followup-4.md). This task adds this note and an invented-bytes integration test file only. No reader/importer logic, resource limit or admission rule is changed. The current implementation commit for the larger run was not supplied; these are owner-reported execution results, not a new source audit.

## Complete accounting of the reported set

| Basis | Group | Count | Reported result and limits |
| --- | --- | ---: | --- |
| OBSERVATION | Legacy discriminator e8 ac 22 00 | 32 | The five subgroups below account for all 32. |
| OBSERVATION | Current discriminator e8 ac 2c 00 with Escher streams | 19 | The legacy reader skips these as not a legacy file. They have not yet been run through the current importer. |
| OBSERVATION | Non-compound, described by the owner as pre-OLE | 1 | Separate container category; not an A/B/C profile. |
| OBSERVATION | Supported B layout within the 32 | 18 | Open Complete, 1..52 segments per input, three warnings each except one with four. Complete is the reader's supported-text scope, not native-layout fidelity. |
| OBSERVATION | Header word +0x04=200 within the 32 | 8 | Currently called damaged because the implementation expects 300. Structural pattern below. |
| OBSERVATION | Unknown table word 58 within the 32 | 3 | Root class 00021201, table prefix 6, no Quill store, four streams each. |
| OBSERVATION | Root class 00021200 with table word 94 within the 32 | 2 | Both have the Quill CONTENTS store; one root Contents stream is about 2.2 MB. Refused by the current profile pairing. |
| OBSERVATION | File over MAX_BYTES within the 32 | 1 | Refused at the 32 MiB operational limit. Exact input size was not supplied. |

OBSERVATION: 18+8+3+2+1=32 and 32+19+1=52. There are 18 supported legacy results, 14 refused legacy inputs, 19 legacy-reader skips and one non-compound input. These are assessment counts; they are not a 52-file batch-conversion success/failure score. Warning contents were not supplied and cannot be reconstructed from warning counts.

OBSERVATION: all eight word-200 headers have +0x02=34 and u32 +0x08 equal to the actual Contents length. The 0xf0de marker exists at the control pointer. The directory pointer is below the control pointer by 2242..145772 bytes. The +0x1a pointer is 0x22 in two, 0x31 in one and 0x34 in five. The word at 0x22 is 44 in two and 20552 in six; it is not a universal table cardinality. None has Quill; stream counts range from three to 28. No pointer/word cross-tabulation beyond these aggregates is inferred.

INFERENCE: the repeated word-200 pattern, different pointer order and varying table-start locations support an earlier or distinct generation from the word-300 layouts. Word 200 alone is not demonstrated damage, and is not proof of a particular product release. Recognise the bounded header pattern and refuse it as unsupported before interpreting records or extracting text. This does not certify every such file as undamaged.

INFERENCE: the two root-00021200/table-94 inputs are B candidates, not yet B successes. Admit the additional identity pairing only after the full B path passes; neither a cardinality nor Quill presence is sufficient. The table-58 group remains an undocumented profile; do not assume its entry count/index base or treat missing Quill as an empty supported store.

## What is now known, and what remains unknown

OBSERVATION: B reader execution now has 18 successful inputs in this set, with varied segment counts. The earlier single-input execution limitation is superseded for this set. There is no supplied identity/independence evidence with which to add these counts to an older corpus or claim 18 independent saving histories. Native text correctness was not scored by this structure-only run.

UNKNOWN: exact saving releases, native page semantics, C's code page, reading order, generation-200/table-58 framing and text locations, success of the two B candidates, exact oversize input length and the 19 current-importer outcomes remain unproven. Repeated unsupported profiles are a classification problem, not evidence that lowering or bypassing bounds checks is safe.

OBSERVATION: the single pre-OLE input belongs in a separate non-compound category with stable reason code not-compound. INFERENCE: arbitrary non-compound bytes must get the same container classification without guessing a Publisher release. The code does not acquire pre-OLE text support by recognising that category.
