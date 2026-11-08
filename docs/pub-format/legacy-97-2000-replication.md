# Legacy-file replication: nine-file structural checkpoint

## Evidence and counting

OBSERVATION (9/9 files): this replication task reuses Sample98.pub and Sample2000.pub and verifies all seven supplied attachments against the exact requested file sizes and SHA-256 values before inspecting streams. The following tables name all nine samples. mspub.pub is the same binary already used in the A note; its reattachment is not a new independently varied sample. The n/9 convention counts named files, not independently confirmed saving versions. Only binary data, own experiments and the repository's own notes supply evidence; no scoring exports, third-party format source or external specification was consulted.

OBSERVATION (scope): the earlier selector-projection task also used the name AP19; this note is the subsequent nine-file replication task. No binary or sample text is committed or redistributed. Originals remain in attachment/restored-data locations; only needed root Contents and available Quill CONTENTS copies and disposable experiment outputs are kept under ignored target/ap-research/. The importer and environment settings/scripts are unchanged, and no full CI or pull request is run. Stream tables report container-advertised lengths; actual read lengths were separately checked for extracted Contents/Quill streams, not all embedded object streams.

| Basis / file coverage | File | Bytes | SHA-256 | Classification |
| --- | --- | ---: | --- | --- |
| OBSERVATION (1/9) | Sample98.pub | 61440 | `8912f295ff0cd1f55b873b3b98a182218f33c420877e025d2f0fd795f12dd0c0` | A |
| OBSERVATION (1/9) | Sample2000.pub | 61440 | `40701ca47b26d04771cdd58467764e9ab39d3da69b59fbc529afd16d263d2f86` | B |
| OBSERVATION (1/9) | CATALOG.PUB | 79360 | `bc95ef2569e8d2384ecafcc3449516076902148ac08259fde408c39543486c0a` | Neither |
| OBSERVATION (1/9) | JAZZY.PUB | 530944 | `49933de05204e9f67696a47ee570fbd6041889a92865665f5488fd2e2606b8b1` | Neither |
| OBSERVATION (1/9) | MODERN.PUB | 337408 | `aa446e31e352677a87c01db22d664fd969b8ea3d7bc081b881a89dbee7afa24d` | Neither |
| OBSERVATION (1/9) | mspub.pub | 19968 | `0eff0498172805bf1e52c74a6334ecc4cdc84fb2e531c0ea25734cfac7923c7d` | A |
| OBSERVATION (1/9) | PLAIN.PUB | 343552 | `48384326430f61ecd6b924d0010631eb72838b0ad09dc33e0e53f3998ff642fe` | Neither |
| OBSERVATION (1/9) | SCHOOL.PUB | 88064 | `e859a1a272e0e37d25249ea2807dd858d2b12bb1023c6023182c3ea2f4abad41` | Neither |
| OBSERVATION (1/9) | zmspub.pub | 17920 | `8a7962fc897a717b0057c5cf1ef8acd481201106ffeb82c4f89c069f103daa97` | A |

OBSERVATION (3/9 A, 1/9 B, 5/9 neither): A is Sample98, mspub and zmspub; B is Sample2000 alone. Classification requires the documented whole-table/count relationship and control/directory relationships, not merely the shared old-family discriminator or the importer refusal. A walks 88 entries labelled 1..88 under table word 89, ending at control 0x37E with directory at 0x3AC. B's masked-tail candidate walks 92 ordinary entries labelled 2..93 under word 94, ending at control 0x3BA with directory at 0x3E8, and retains its unresolved leading slot. Neither describes the other five files: all have table word 64, control at 0x280, a 46-byte control region ending 0x2AE, and a later directory. They meet neither documented A nor B classification. The masked B termination remains a candidate, not a proven general selector encoding.

OBSERVATION (5/9): the unfamiliar group shares root CLSID 00021200-0000-0000-00c0-000000000046, versus 00021201-0000-0000-00c0-000000000046 in the four A/B files (4/9). A literal-zero-tail walk consumes 64 entries exactly to 0x280. Interpreting these entries with zero-based indices 0..63 makes descriptor word-0 high byte match record H in 2545/2545 records; the A one-based convention gives only 19/101, 114/611, 54/979, 56/735 and 28/119 respectively. This zero-based mapping is a new sampled structural observation, not a repair to either existing A/B rule.

INFERENCE (5/9): shared table cardinality/index base, nonadjacent directory, distinct CLSID, missing Quill and repeated equation failures support a third structural family, provisionally called C here. C is a research label for neither-A-nor-B, not an established Publisher release. UNKNOWN (9/9): exact saving versions and independent manufacturing provenance beyond what the owner reported.

## Container streams

OBSERVATION (9/9): all containers open as compound version 3. Each following subsection covers one of nine files. Storage plus child name reconstructs each full stream path; a semicolon separates streams, never sample content. Escaped control characters are literal path notation. All 732 stream names and advertised sizes are included; embedded payloads are not decoded.

### Sample98.pub

OBSERVATION (1/9): 10 streams; root CLSID 00021201-0000-0000-00c0-000000000046.

| Basis / coverage | Storage | Child stream: bytes |
| --- | --- | --- |
| OBSERVATION (1/9) | `/Quill/QuillSub` | `\u{1}CompObj`: 86; `CONTENTS`: 6144 |
| OBSERVATION (1/9) | `/Escher` | `EscherStm`: 0; `EscherDelayStm`: 0 |
| OBSERVATION (1/9) | `/` | `\u{1}CompObj`: 94; `Contents`: 5930; `Envelope`: 0; `\u{3}Internal`: 12; `\u{5}SummaryInformation`: 44640; `\u{5}DocumentSummaryInformation`: 152 |

### Sample2000.pub

OBSERVATION (1/9): 10 streams; root CLSID 00021201-0000-0000-00c0-000000000046.

| Basis / coverage | Storage | Child stream: bytes |
| --- | --- | --- |
| OBSERVATION (1/9) | `/Quill/QuillSub` | `\u{1}CompObj`: 86; `CONTENTS`: 6144 |
| OBSERVATION (1/9) | `/Escher` | `EscherStm`: 0; `EscherDelayStm`: 0 |
| OBSERVATION (1/9) | `/` | `\u{1}CompObj`: 94; `Contents`: 6042; `Envelope`: 0; `\u{3}Internal`: 12; `\u{5}SummaryInformation`: 44640; `\u{5}DocumentSummaryInformation`: 152 |

### CATALOG.PUB

OBSERVATION (1/9): 30 streams; root CLSID 00021200-0000-0000-00c0-000000000046.

| Basis / coverage | Storage | Child stream: bytes |
| --- | --- | --- |
| OBSERVATION (1/9) | `/Objects/Object 0` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 220; `\u{3}Internal`: 16; `\u{2}OlePres000`: 6010 |
| OBSERVATION (1/9) | `/Objects/Object 13` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 236; `\u{3}Internal`: 16; `\u{2}OlePres000`: 12836 |
| OBSERVATION (1/9) | `/Objects/Object 15` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 221; `\u{3}Internal`: 16; `\u{2}OlePres000`: 6830 |
| OBSERVATION (1/9) | `/Objects/Object 18` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 222; `\u{3}Internal`: 16; `\u{2}OlePres000`: 8310 |
| OBSERVATION (1/9) | `/Objects/Object 19` | `\u{1}CompObj`: 120; `\u{2}OlePres000`: 3038; `\u{1}Ole10Native`: 896 |
| OBSERVATION (1/9) | `/Objects/Object 20` | `\u{1}CompObj`: 120; `\u{2}OlePres000`: 2950; `\u{1}Ole10Native`: 896 |
| OBSERVATION (1/9) | `/` | `\u{1}CompObj`: 70; `Contents`: 23990; `\u{3}Internal`: 12; `\u{5}SummaryInformation`: 1648 |

### JAZZY.PUB

OBSERVATION (1/9): 311 streams; root CLSID 00021200-0000-0000-00c0-000000000046.

| Basis / coverage | Storage | Child stream: bytes |
| --- | --- | --- |
| OBSERVATION (1/9) | `/Objects/Object 147` | `\u{1}Ole`: 20; `\u{1}CompObj`: 102; `Contents`: 109; `\u{3}Internal`: 16; `\u{2}OlePres000`: 2940; `\u{2}OlePres001`: 952 |
| OBSERVATION (1/9) | `/Objects/Object 148` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 220; `\u{3}Internal`: 16; `\u{2}OlePres000`: 9048; `\u{2}OlePres001`: 952 |
| OBSERVATION (1/9) | `/Objects/Object 149` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 210; `\u{3}Internal`: 16; `\u{2}OlePres000`: 8016; `\u{2}OlePres001`: 952 |
| OBSERVATION (1/9) | `/Objects/Object 151` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 203; `\u{3}Internal`: 16; `\u{2}OlePres000`: 5872; `\u{2}OlePres001`: 952 |
| OBSERVATION (1/9) | `/Objects/Object 152` | `\u{1}Ole`: 20; `\u{1}CompObj`: 92; `\u{2}OlePres000`: 10758; `\u{2}OlePres001`: 952; `\u{1}Ole10Native`: 10756 |
| OBSERVATION (1/9) | `/Objects/Object 153` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 204; `\u{3}Internal`: 12; `\u{2}OlePres000`: 5244; `\u{2}OlePres001`: 952 |
| OBSERVATION (1/9) | `/Objects/Object 154` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446; `\u{2}OlePres001`: 952 |
| OBSERVATION (1/9) | `/Objects/Object 155` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446; `\u{2}OlePres001`: 952 |
| OBSERVATION (1/9) | `/Objects/Object 156` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446; `\u{2}OlePres001`: 952 |
| OBSERVATION (1/9) | `/Objects/Object 157` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446; `\u{2}OlePres001`: 952 |
| OBSERVATION (1/9) | `/Objects/Object 158` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446; `\u{2}OlePres001`: 952 |
| OBSERVATION (1/9) | `/Objects/Object 159` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446; `\u{2}OlePres001`: 952 |
| OBSERVATION (1/9) | `/Objects/Object 160` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446; `\u{2}OlePres001`: 952 |
| OBSERVATION (1/9) | `/Objects/Object 161` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 12; `\u{2}OlePres000`: 446; `\u{2}OlePres001`: 952 |
| OBSERVATION (1/9) | `/Objects/Object 162` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 215; `\u{3}Internal`: 12; `\u{2}OlePres000`: 8848; `\u{2}OlePres001`: 952 |
| OBSERVATION (1/9) | `/Objects/Object 163` | `\u{1}Ole`: 20; `\u{1}CompObj`: 102; `Contents`: 104; `\u{3}Internal`: 16; `\u{2}OlePres000`: 2908; `\u{2}OlePres001`: 952 |
| OBSERVATION (1/9) | `/Objects/Object 165` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 166` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 167` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 168` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 169` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 170` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 171` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 173` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 214; `\u{3}Internal`: 12; `\u{2}OlePres000`: 9544; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 174` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 175` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 176` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 177` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 178` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 179` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 180` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 181` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 12; `\u{2}OlePres000`: 446; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 182` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 215; `\u{3}Internal`: 12; `\u{2}OlePres000`: 8848; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 183` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 214; `\u{3}Internal`: 12; `\u{2}OlePres000`: 9512; `\u{2}OlePres001`: 952 |
| OBSERVATION (1/9) | `/Objects/Object 184` | `\u{1}Ole`: 20; `\u{1}CompObj`: 92; `\u{2}OlePres000`: 10758; `\u{2}OlePres001`: 940; `\u{1}Ole10Native`: 10756 |
| OBSERVATION (1/9) | `/Objects/Object 185` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446 |
| OBSERVATION (1/9) | `/Objects/Object 186` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446 |
| OBSERVATION (1/9) | `/Objects/Object 187` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446 |
| OBSERVATION (1/9) | `/Objects/Object 188` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446 |
| OBSERVATION (1/9) | `/Objects/Object 189` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446 |
| OBSERVATION (1/9) | `/Objects/Object 190` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446 |
| OBSERVATION (1/9) | `/Objects/Object 191` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 16; `\u{2}OlePres000`: 446 |
| OBSERVATION (1/9) | `/Objects/Object 192` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 12; `\u{2}OlePres000`: 446 |
| OBSERVATION (1/9) | `/Objects/Object 202` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 203; `\u{3}Internal`: 12; `\u{2}OlePres000`: 5632; `\u{2}OlePres001`: 12468 |
| OBSERVATION (1/9) | `/Objects/Object 205` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 204; `\u{3}Internal`: 12; `\u{2}OlePres000`: 8656; `\u{2}OlePres001`: 18928 |
| OBSERVATION (1/9) | `/Objects/Object 206` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 209; `\u{3}Internal`: 12; `\u{2}OlePres000`: 4472; `\u{2}OlePres001`: 7804 |
| OBSERVATION (1/9) | `/Objects/Object 207` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 208; `\u{3}Internal`: 12; `\u{2}OlePres000`: 7460; `\u{2}OlePres001`: 16112 |
| OBSERVATION (1/9) | `/Objects/Object 208` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 209; `\u{3}Internal`: 12; `\u{2}OlePres000`: 9220; `\u{2}OlePres001`: 12184 |
| OBSERVATION (1/9) | `/Objects/Object 211` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 194; `\u{3}Internal`: 12; `\u{2}OlePres000`: 2528; `\u{2}OlePres001`: 5724 |
| OBSERVATION (1/9) | `/Objects/Object 212` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 204; `\u{3}Internal`: 12; `\u{2}OlePres000`: 5280; `\u{2}OlePres001`: 8168 |
| OBSERVATION (1/9) | `/Objects/Object 271` | `\u{1}Ole`: 20; `\u{1}CompObj`: 102; `Contents`: 110; `\u{3}Internal`: 16; `\u{2}OlePres000`: 2908; `\u{2}OlePres001`: 952 |
| OBSERVATION (1/9) | `/Objects/Object 273` | `\u{1}Ole`: 20; `\u{1}CompObj`: 92; `\u{2}OlePres000`: 7916; `\u{2}OlePres001`: 280; `\u{1}Ole10Native`: 7908 |
| OBSERVATION (1/9) | `/Objects/Object 276` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 207; `\u{3}Internal`: 12; `\u{2}OlePres000`: 4276; `\u{2}OlePres001`: 9432 |
| OBSERVATION (1/9) | `/` | `\u{1}CompObj`: 94; `Contents`: 121952; `\u{3}Internal`: 12; `\u{5}SummaryInformation`: 7692 |

### MODERN.PUB

OBSERVATION (1/9): 148 streams; root CLSID 00021200-0000-0000-00c0-000000000046.

| Basis / coverage | Storage | Child stream: bytes |
| --- | --- | --- |
| OBSERVATION (1/9) | `/Objects/Object 46` | `\u{1}Ole`: 20; `\u{1}CompObj`: 102; `Contents`: 113; `\u{3}Internal`: 16; `\u{2}OlePres000`: 2280; `\u{2}OlePres001`: 948 |
| OBSERVATION (1/9) | `/Objects/Object 47` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 202; `\u{3}Internal`: 16; `\u{2}OlePres000`: 2522; `\u{2}OlePres001`: 948 |
| OBSERVATION (1/9) | `/Objects/Object 48` | `\u{1}Ole`: 20; `\u{1}CompObj`: 102; `Contents`: 103; `\u{3}Internal`: 16; `\u{2}OlePres000`: 2522; `\u{2}OlePres001`: 948 |
| OBSERVATION (1/9) | `/Objects/Object 49` | `\u{1}Ole`: 20; `\u{1}CompObj`: 102; `Contents`: 103; `\u{3}Internal`: 16; `\u{2}OlePres000`: 2522; `\u{2}OlePres001`: 948 |
| OBSERVATION (1/9) | `/Objects/Object 56` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 203; `\u{3}Internal`: 12; `\u{2}OlePres000`: 5872; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 57` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 219; `\u{3}Internal`: 12; `\u{2}OlePres000`: 5790; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 58` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 204; `\u{3}Internal`: 12; `\u{2}OlePres000`: 8450; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 59` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 202; `\u{3}Internal`: 16; `\u{2}OlePres000`: 5244; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 73` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 197; `\u{3}Internal`: 12; `\u{2}OlePres000`: 2984; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 74` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 196; `\u{3}Internal`: 12; `\u{2}OlePres000`: 2826; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 75` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 200; `\u{3}Internal`: 12; `\u{2}OlePres000`: 672; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 76` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 211; `\u{3}Internal`: 12; `\u{2}OlePres000`: 4300; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 79` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 201; `\u{3}Internal`: 12; `\u{2}OlePres000`: 5160; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 82` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 206; `\u{3}Internal`: 12; `\u{2}OlePres000`: 4340; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 84` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 200; `\u{3}Internal`: 12; `\u{2}OlePres000`: 3928; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 92` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 199; `\u{3}Internal`: 12; `\u{2}OlePres000`: 3926; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 94` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 209; `\u{3}Internal`: 12; `\u{2}OlePres000`: 5096; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 98` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 202; `\u{3}Internal`: 12; `\u{2}OlePres000`: 8512; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 102` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 202; `\u{3}Internal`: 12; `\u{2}OlePres000`: 8754; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 108` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 202; `\u{3}Internal`: 16; `\u{2}OlePres000`: 8436; `\u{2}OlePres001`: 948 |
| OBSERVATION (1/9) | `/Objects/Object 137` | `\u{1}Ole`: 20; `\u{1}CompObj`: 102; `Contents`: 103; `\u{3}Internal`: 16; `\u{2}OlePres000`: 2064; `\u{2}OlePres001`: 948 |
| OBSERVATION (1/9) | `/Objects/Object 138` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 222; `\u{3}Internal`: 12; `\u{2}OlePres000`: 5790; `\u{2}OlePres001`: 948 |
| OBSERVATION (1/9) | `/Objects/Object 139` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 206; `\u{3}Internal`: 12; `\u{2}OlePres000`: 9078; `\u{2}OlePres001`: 948 |
| OBSERVATION (1/9) | `/Objects/Object 141` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 202; `\u{3}Internal`: 16; `\u{2}OlePres000`: 8436; `\u{2}OlePres001`: 948 |
| OBSERVATION (1/9) | `/` | `\u{1}CompObj`: 94; `Contents`: 149719; `\u{3}Internal`: 12; `\u{5}SummaryInformation`: 7692 |

### mspub.pub

OBSERVATION (1/9): 6 streams; root CLSID 00021201-0000-0000-00c0-000000000046.

| Basis / coverage | Storage | Child stream: bytes |
| --- | --- | --- |
| OBSERVATION (1/9) | `/Quill/QuillSub` | `\u{1}CompObj`: 86; `CONTENTS`: 1024 |
| OBSERVATION (1/9) | `/` | `\u{1}CompObj`: 94; `Contents`: 4152; `\u{3}Internal`: 12; `\u{5}SummaryInformation`: 8232 |

### PLAIN.PUB

OBSERVATION (1/9): 166 streams; root CLSID 00021200-0000-0000-00c0-000000000046.

| Basis / coverage | Storage | Child stream: bytes |
| --- | --- | --- |
| OBSERVATION (1/9) | `/Objects/Object 137` | `\u{1}Ole`: 20; `\u{1}CompObj`: 102; `Contents`: 118; `\u{3}Internal`: 16; `\u{2}OlePres000`: 5010; `\u{2}OlePres001`: 952 |
| OBSERVATION (1/9) | `/Objects/Object 139` | `\u{1}Ole`: 20; `\u{1}CompObj`: 102; `Contents`: 120; `\u{3}Internal`: 16; `\u{2}OlePres000`: 2280; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 140` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 202; `\u{3}Internal`: 16; `\u{2}OlePres000`: 5244; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 143` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 211; `\u{3}Internal`: 12; `\u{2}OlePres000`: 7156; `\u{2}OlePres001`: 952 |
| OBSERVATION (1/9) | `/Objects/Object 144` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 219; `\u{3}Internal`: 12; `\u{2}OlePres000`: 5584; `\u{2}OlePres001`: 940 |
| OBSERVATION (1/9) | `/Objects/Object 145` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 219; `\u{3}Internal`: 12; `\u{2}OlePres000`: 5244; `\u{2}OlePres001`: 940 |
| OBSERVATION (1/9) | `/Objects/Object 146` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 213; `\u{3}Internal`: 16; `\u{2}OlePres000`: 7974; `\u{2}OlePres001`: 940 |
| OBSERVATION (1/9) | `/Objects/Object 147` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 202; `\u{3}Internal`: 16; `\u{2}OlePres000`: 5244; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 148` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 202; `\u{3}Internal`: 16; `\u{2}OlePres000`: 5244; `\u{2}OlePres001`: 952 |
| OBSERVATION (1/9) | `/Objects/Object 149` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 202; `\u{3}Internal`: 16; `\u{2}OlePres000`: 5244; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 151` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 219; `\u{3}Internal`: 12; `\u{2}OlePres000`: 5584; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 152` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 211; `\u{3}Internal`: 12; `\u{2}OlePres000`: 7156; `\u{2}OlePres001`: 952 |
| OBSERVATION (1/9) | `/Objects/Object 153` | `\u{1}Ole`: 20; `\u{1}CompObj`: 102; `Contents`: 102; `\u{3}Internal`: 16; `\u{2}OlePres000`: 816; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 161` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 201; `\u{3}Internal`: 12; `\u{2}OlePres000`: 4200; `\u{2}OlePres001`: 6536 |
| OBSERVATION (1/9) | `/Objects/Object 166` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 210; `\u{3}Internal`: 12; `\u{2}OlePres000`: 5604; `\u{2}OlePres001`: 11012 |
| OBSERVATION (1/9) | `/Objects/Object 173` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 208; `\u{3}Internal`: 12; `\u{2}OlePres000`: 3836; `\u{2}OlePres001`: 6336 |
| OBSERVATION (1/9) | `/Objects/Object 174` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 209; `\u{3}Internal`: 12; `\u{2}OlePres000`: 6024; `\u{2}OlePres001`: 8596 |
| OBSERVATION (1/9) | `/Objects/Object 175` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 199; `\u{3}Internal`: 12; `\u{2}OlePres000`: 5752; `\u{2}OlePres001`: 8504 |
| OBSERVATION (1/9) | `/Objects/Object 176` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 207; `\u{3}Internal`: 12; `\u{2}OlePres000`: 4506; `\u{2}OlePres001`: 7662 |
| OBSERVATION (1/9) | `/Objects/Object 180` | `\u{1}Ole`: 20; `\u{1}CompObj`: 102; `Contents`: 120; `\u{3}Internal`: 16; `\u{2}OlePres000`: 2280; `\u{2}OlePres001`: 952 |
| OBSERVATION (1/9) | `/Objects/Object 182` | `\u{1}Ole`: 20; `\u{1}CompObj`: 102; `Contents`: 120; `\u{3}Internal`: 16; `\u{2}OlePres000`: 2280; `\u{2}OlePres001`: 952 |
| OBSERVATION (1/9) | `/Objects/Object 183` | `\u{1}Ole`: 20; `\u{1}CompObj`: 102; `Contents`: 120; `\u{3}Internal`: 16; `\u{2}OlePres000`: 2280; `\u{2}OlePres001`: 940 |
| OBSERVATION (1/9) | `/Objects/Object 185` | `\u{1}Ole`: 20; `\u{1}CompObj`: 102; `Contents`: 120; `\u{3}Internal`: 16; `\u{2}OlePres000`: 2280; `\u{2}OlePres001`: 940 |
| OBSERVATION (1/9) | `/Objects/Object 203` | `\u{1}Ole`: 20; `\u{1}CompObj`: 102; `Contents`: 120; `\u{3}Internal`: 16; `\u{2}OlePres000`: 2280; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 204` | `\u{1}Ole`: 20; `\u{1}CompObj`: 102; `Contents`: 120; `\u{3}Internal`: 16; `\u{2}OlePres000`: 2280; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 205` | `\u{1}Ole`: 20; `\u{1}CompObj`: 102; `Contents`: 120; `\u{3}Internal`: 16; `\u{2}OlePres000`: 2280; `\u{2}OlePres001`: 280 |
| OBSERVATION (1/9) | `/Objects/Object 207` | `\u{1}Ole`: 20; `\u{1}CompObj`: 77; `\u{2}OlePres000`: 5326; `\u{2}OlePres001`: 4414; `\u{1}Ole10Native`: 4324; `\u{1}Ole10ItemName`: 5 |
| OBSERVATION (1/9) | `/` | `\u{1}CompObj`: 94; `Contents`: 94770; `\u{3}Internal`: 12; `\u{5}SummaryInformation`: 8480 |

### SCHOOL.PUB

OBSERVATION (1/9): 45 streams; root CLSID 00021200-0000-0000-00c0-000000000046.

| Basis / coverage | Storage | Child stream: bytes |
| --- | --- | --- |
| OBSERVATION (1/9) | `/Objects/Object 4` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 207; `\u{3}Internal`: 16; `\u{2}OlePres000`: 2724 |
| OBSERVATION (1/9) | `/Objects/Object 5` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 206; `\u{3}Internal`: 16; `\u{2}OlePres000`: 6898 |
| OBSERVATION (1/9) | `/Objects/Object 13` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 208; `\u{3}Internal`: 16; `\u{2}OlePres000`: 2490 |
| OBSERVATION (1/9) | `/Objects/Object 16` | `\u{1}Ole`: 20; `\u{1}CompObj`: 76; `Contents`: 207; `\u{3}Internal`: 16; `\u{2}OlePres000`: 2724 |
| OBSERVATION (1/9) | `/Objects/Object 17` | `\u{1}CompObj`: 120; `\u{2}OlePres000`: 1738; `\u{1}Ole10Native`: 896 |
| OBSERVATION (1/9) | `/Objects/Object 18` | `\u{1}CompObj`: 120; `\u{2}OlePres000`: 4052; `\u{1}Ole10Native`: 896 |
| OBSERVATION (1/9) | `/Objects/Object 19` | `\u{1}CompObj`: 120; `\u{2}OlePres000`: 1738; `\u{1}Ole10Native`: 896 |
| OBSERVATION (1/9) | `/Objects/Object 20` | `\u{1}CompObj`: 120; `\u{2}OlePres000`: 4052; `\u{1}Ole10Native`: 896 |
| OBSERVATION (1/9) | `/Objects/Object 21` | `\u{1}CompObj`: 120; `\u{2}OlePres000`: 1738; `\u{1}Ole10Native`: 896 |
| OBSERVATION (1/9) | `/Objects/Object 22` | `\u{1}CompObj`: 120; `\u{2}OlePres000`: 4052; `\u{1}Ole10Native`: 896 |
| OBSERVATION (1/9) | `/Objects/Object 23` | `\u{1}CompObj`: 120; `\u{2}OlePres000`: 4052; `\u{1}Ole10Native`: 896 |
| OBSERVATION (1/9) | `/` | `\u{1}CompObj`: 78; `Contents`: 26606; `\u{3}Internal`: 12; `\u{5}SummaryInformation`: 2704 |

### zmspub.pub

OBSERVATION (1/9): 6 streams; root CLSID 00021201-0000-0000-00c0-000000000046.

| Basis / coverage | Storage | Child stream: bytes |
| --- | --- | --- |
| OBSERVATION (1/9) | `/Quill/QuillSub` | `\u{1}CompObj`: 86; `CONTENTS`: 1024 |
| OBSERVATION (1/9) | `/` | `\u{1}CompObj`: 94; `Contents`: 4152; `\u{3}Internal`: 12; `\u{5}SummaryInformation`: 8160 |

## Header and region checks

OBSERVATION (9/9): unaligned LE u32 at +0x08 equals actual Contents size; +0x12/+0x16/+0x1A are bounded control/directory/table pointers. +0x1A is 34, and u16 +0x02 is also 34. The old-family discriminator and u16 +0x04=300 are shared by all nine. Header u16 +0x06 is 4 in the seven attachments (7/9), 261 in Sample98/Sample2000 (2/9), so it cannot by itself select A/B. Values below are decimal except hex offsets. u32 length is not truncated into the two u16 words that overlap it.

| Basis / coverage | File | Contents size = u32 +08 | +12 control | +16 directory | +1A table | u16 +0C / +0E / +10 / +1E / +20 | Table u16 +22 / +24 |
| --- | --- | ---: | --- | --- | --- | --- | --- |
| OBSERVATION (1/9) | Sample98.pub | 5930 | 0x37E | 0x3AC | 0x22 | 616 / 256 / 0 / 248 / 12 | 89 / 6 |
| OBSERVATION (1/9) | Sample2000.pub | 6042 | 0x3BA | 0x3E8 | 0x22 | 717 / 256 / 0 / 252 / 12 | 94 / 6 |
| OBSERVATION (1/9) | CATALOG.PUB | 23990 | 0x280 | 0x2C8A | 0x22 | 136 / 532 / 419 / 256 / 512 | 64 / 6 |
| OBSERVATION (1/9) | JAZZY.PUB | 121952 | 0x280 | 0x3084 | 0x22 | 136 / 256 / 655 / 512 / 512 | 64 / 6 |
| OBSERVATION (1/9) | MODERN.PUB | 149719 | 0x280 | 0x2E7E | 0x22 | 136 / 256 / 948 / 512 / 512 | 64 / 6 |
| OBSERVATION (1/9) | mspub.pub | 4152 | 0x37E | 0x3AC | 0x22 | 616 / 295 / 0 / 1280 / 1280 | 89 / 6 |
| OBSERVATION (1/9) | PLAIN.PUB | 94770 | 0x280 | 0x3284 | 0x22 | 136 / 256 / 1132 / 512 / 512 | 64 / 6 |
| OBSERVATION (1/9) | SCHOOL.PUB | 26606 | 0x280 | 0x3872 | 0x22 | 136 / 438 / 400 / 0 / 512 | 64 / 6 |
| OBSERVATION (1/9) | zmspub.pub | 4152 | 0x37E | 0x3AC | 0x22 | 616 / 295 / 0 / 1280 / 1280 | 89 / 6 |

OBSERVATION (9/9): each control starts with the previously observed marker and stored length 46. Control +0x0A points to the directory in 9/9. Control +0x0E also equals the directory only in the four A/B files (4/9); in the five C files (5/9) it equals 0x2AE, exactly control+46, not the directory. Header +0x0E equals the independently located root-like directory ID in 9/9. Other numeric header fields retain UNKNOWN roles unless a measured relation is explicitly stated.

OBSERVATION (9/9): exact interval accounting uses header [0,T), table [T,C), control [C,C+46), then directory [D,D+4+10*N), a zero-filled gap before the first record, and the directory spans to stream end. In C only (5/9), add the unassigned interval [C+46,D); it must not be skipped or called padding without decoding it. The contiguous A/B partition and duplicate-control-pointer rules fail in each C file at these exact fields. Zero pre-record gaps and all other intervals are fully bounded and nonoverlapping in all nine.

| Basis / coverage | File | Ordinary entries / index convention | Unassigned [control+46,directory) | Directory [D,end) | Zero gap bytes | Records to stream end |
| --- | --- | --- | --- | --- | ---: | --- |
| OBSERVATION (1/9) | Sample98.pub | 88 / 1..88 | [0x3AC,0x3AC) | [0x3AC,0x5CC) | 20 | [0x5E0,0x172A) |
| OBSERVATION (1/9) | Sample2000.pub | 92 / 2..93; leading slot UNKNOWN | [0x3E8,0x3E8) | [0x3E8,0x608) | 10 | [0x612,0x179A) |
| OBSERVATION (1/9) | CATALOG.PUB | 64 / 0..63 (diagnostic C mapping) | [0x2AE,0x2C8A) | [0x2C8A,0x3080) | 10 | [0x308A,0x5DB6) |
| OBSERVATION (1/9) | JAZZY.PUB | 64 / 0..63 (diagnostic C mapping) | [0x2AE,0x3084) | [0x3084,0x4866) | 10 | [0x4870,0x1DC60) |
| OBSERVATION (1/9) | MODERN.PUB | 64 / 0..63 (diagnostic C mapping) | [0x2AE,0x2E7E) | [0x2E7E,0x54C0) | 10 | [0x54CA,0x248D7) |
| OBSERVATION (1/9) | mspub.pub | 88 / 1..88 | [0x3AC,0x3AC) | [0x3AC,0x536) | 10 | [0x540,0x1038) |
| OBSERVATION (1/9) | PLAIN.PUB | 64 / 0..63 (diagnostic C mapping) | [0x2AE,0x3284) | [0x3284,0x4F3E) | 10 | [0x4F48,0x17232) |
| OBSERVATION (1/9) | SCHOOL.PUB | 64 / 0..63 (diagnostic C mapping) | [0x2AE,0x3872) | [0x3872,0x3D1C) | 10 | [0x3D26,0x67EE) |
| OBSERVATION (1/9) | zmspub.pub | 88 / 1..88 | [0x3AC,0x3AC) | [0x3AC,0x536) | 10 | [0x540,0x1038) |

## Rule-by-file results

OBSERVATION (9/9): N ten-byte entries at D+4 contain distinct u16 IDs, u16 parent candidates, u32 starts and trailing u16 attributes. Directory maximum-ID word equals the actual maximum in all nine. All starts are distinct, ascending, after the directory and inside Contents; E is next start or stream end. Every four-byte record prefix and H boundary fits its span, and descriptor-high-byte H correlation succeeds in 2731/2731 records with the file-specific mapping above. Each directory-parent graph has one external root parent and no cycle: this is an ownership-candidate tree, not proof that every record reference is a parent edge or that a collection is a page.

OBSERVATION (4/9 files): all 186 A/B record endpoints satisfy the prior class-selected equations with their stated tail checks; A alone now has 132/132 endpoints across 3/9 files, B remains 54/54 in 1/9. Class 67 still has only B's one empty sampled instance as a B rule. The five C files are diagnostic transfer tests, not newly confirmed A/B files. Their strict checks pass 2355/2545; ten additional endpoints have matching arithmetic but fail offset-table structure, giving 2365/2545 arithmetic matches. Universal transfer is refuted in each C file (5/9), while these failures do not refute a rule confined to A or B.

| Basis / coverage | File | N: directory/range/H passes | Strict equation + tail checks | Length arithmetic only | Directory internal parents / root / cycles | Raw L equals span (control only) |
| --- | --- | ---: | --- | --- | --- | --- |
| OBSERVATION (1/9) | Sample98.pub | 54/54 | 54/54 | 54/54 | 53 / 0x100 / 0 | 1/54 |
| OBSERVATION (1/9) | Sample2000.pub | 54/54 | 54/54 | 54/54 | 53 / 0x100 / 0 | 4/54 |
| OBSERVATION (1/9) | CATALOG.PUB | 101/101 | 93/101 | 93/101 | 100 / 0x214 / 0 | 12/101 |
| OBSERVATION (1/9) | JAZZY.PUB | 611/611 | 535/611 | 538/611 | 610 / 0x100 / 0 | 113/611 |
| OBSERVATION (1/9) | MODERN.PUB | 979/979 | 925/979 | 928/979 | 978 / 0x100 / 0 | 310/979 |
| OBSERVATION (1/9) | mspub.pub | 39/39 | 39/39 | 39/39 | 38 / 0x127 / 0 | 4/39 |
| OBSERVATION (1/9) | PLAIN.PUB | 735/735 | 697/735 | 700/735 | 734 / 0x100 / 0 | 308/735 |
| OBSERVATION (1/9) | SCHOOL.PUB | 119/119 | 105/119 | 106/119 | 118 / 0x1B6 / 0 | 28/119 |
| OBSERVATION (1/9) | zmspub.pub | 39/39 | 39/39 | 39/39 | 38 / 0x127 / 0 | 4/39 |

OBSERVATION (tested in 9/9): unchanged equation names are F=H (class 0); A=H+10+n*w with equal u16 counts at T/T+2 and u16 width T+4 (classes 1/9/25); B=H+8+q with u32 q at T (class 4); C=H+22+n*(4+w) with u16 count pair/width (classes 2/10); O=H+18+q with u32 count pair/q at T/T+4/T+8 and n+1 offsets at T+18 (classes 3/195, candidate 67). T=R+H. O additionally requires first offset 4*(n+1), final q and strict increase. Every read is bounded first. Classes 163 and 8 have no prior equation. Descriptor stride is not substituted for the stored width; L is only a failed length-control hypothesis, not an established rule. Exact attribute values are listed later without assigning semantics.

### Every diagnostic extent/framing failure

OBSERVATION (5/9 files): the following 190 records are the complete failed/unmapped strict-transfer set; there are no failures in the four A/B files (4/9). Prediction is calculated from fields, never copied from the span. A missing prior rule is UNKNOWN rather than a falsified equation. For O rows, arithmetic matches in all ten cases: the tail-index checks are what fail. Offset-control details are listed separately. Zero-based C class/index labels are observations of that diagnostic mapping, not decoded semantic meanings.

| Basis / coverage | File | ID | Directory entry / record start | Index / class | Span / H | Prior rule / prediction | Other fitting families |
| --- | --- | --- | --- | --- | --- | --- | --- |
| OBSERVATION (1/9) | CATALOG.PUB | 0x102 | 0x2C98 / 0x3132 | 41 / 163 | 416 / 44 | UNKNOWN class / UNKNOWN | O |
| OBSERVATION (1/9) | CATALOG.PUB | 0x118 | 0x2DBA / 0x4695 | 53 / 8 | 200 / 200 | UNKNOWN class / UNKNOWN | F |
| OBSERVATION (1/9) | CATALOG.PUB | 0x273 | 0x2E64 / 0x4AAD | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | CATALOG.PUB | 0x123 | 0x2E78 / 0x4B3D | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | CATALOG.PUB | 0x110 | 0x2EAA / 0x4CD5 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | CATALOG.PUB | 0x119 | 0x301C / 0x5B46 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | CATALOG.PUB | 0x1A2 | 0x304E / 0x5C8A | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | CATALOG.PUB | 0x1A6 | 0x3062 / 0x5D1A | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x112 | 0x3092 / 0x490A | 41 / 163 | 211 / 44 | UNKNOWN class / UNKNOWN | O |
| OBSERVATION (1/9) | JAZZY.PUB | 0x106 | 0x3114 / 0x4FB9 | 50 / 3 | 1098 / 4 | O / 1098 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x104 | 0x31AA / 0x634B | 50 / 3 | 605 / 4 | O / 605 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x124 | 0x31E6 / 0x674E | 58 / 3 | 430 / 4 | O / 430 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x1B1 | 0x3240 / 0x6AB4 | 33 / 0 | 2404 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x1B3 | 0x3254 / 0x7496 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x1A6 | 0x327C / 0x75B2 | 33 / 0 | 716 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x1A9 | 0x329A / 0x7924 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x1DF | 0x32CC / 0x7AA6 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x1ED | 0x32E0 / 0x7B36 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x1D2 | 0x32FE / 0x7BEC | 33 / 0 | 4656 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x1D5 | 0x331C / 0x8EC2 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x17C | 0x334E / 0x9048 | 33 / 0 | 1816 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x18E | 0x3362 / 0x97DE | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x229 | 0x3394 / 0x9962 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x204 | 0x33DA / 0x9B56 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x208 | 0x33EE / 0x9BE6 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x20F | 0x3402 / 0x9C76 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x219 | 0x3416 / 0x9D06 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x21B | 0x342A / 0x9D96 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x21E | 0x343E / 0x9E26 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x220 | 0x3452 / 0x9EB6 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x222 | 0x3466 / 0x9F46 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x225 | 0x347A / 0x9FD6 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x1C0 | 0x34C0 / 0xA199 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x199 | 0x34F2 / 0xA2E7 | 33 / 0 | 656 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x34E | 0x3560 / 0xA7FC | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x151 | 0x3588 / 0xA918 | 33 / 0 | 2328 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x23F | 0x35B0 / 0xB33A | 33 / 0 | 2244 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x6F2 | 0x35F6 / 0xBDDE | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x1C8 | 0x360A / 0xBE6E | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x136 | 0x361E / 0xBEFE | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x207 | 0x3632 / 0xBF8E | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x2D4 | 0x3646 / 0xC01E | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x1B7 | 0x365A / 0xC0AE | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x2D6 | 0x366E / 0xC13E | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x217 | 0x36DC / 0xC3E1 | 33 / 0 | 652 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x246 | 0x36FA / 0xC729 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x265 | 0x3740 / 0xC8CB | 33 / 0 | 7368 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x157 | 0x3768 / 0xE69D | 33 / 0 | 716 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x718 | 0x3790 / 0xEA73 | 33 / 0 | 2232 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x702 | 0x37A4 / 0xF3A9 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x6E3 | 0x37E0 / 0xF537 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x6E1 | 0x37F4 / 0xF5C7 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x6DF | 0x3808 / 0xF657 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x6DD | 0x381C / 0xF6E7 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x6DB | 0x3830 / 0xF777 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x2D8 | 0x3844 / 0xF807 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x19D | 0x3858 / 0xF897 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x14C | 0x386C / 0xF927 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x699 | 0x3880 / 0xF9B7 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x164 | 0x38DA / 0xFBD0 | 33 / 0 | 660 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x2DB | 0x393E / 0x100C3 | 33 / 0 | 12220 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x1E1 | 0x3966 / 0x13189 | 33 / 0 | 4656 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x713 | 0x3A6A / 0x14A06 | 33 / 0 | 648 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x1D8 | 0x3AD8 / 0x14FC1 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x6C5 | 0x3B00 / 0x15135 | 33 / 0 | 4656 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x264 | 0x3B28 / 0x164C7 | 33 / 0 | 2232 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x16D | 0x3B6E / 0x17011 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x1CC | 0x3B82 / 0x170A1 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x169 | 0x3B96 / 0x17131 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x16C | 0x3BAA / 0x171C1 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x16E | 0x3BBE / 0x17251 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x701 | 0x3BD2 / 0x172E1 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x6D1 | 0x3BE6 / 0x17371 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x6CF | 0x3BFA / 0x17401 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x6E4 | 0x3CA4 / 0x17886 | 33 / 0 | 6032 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x2F9 | 0x42D0 / 0x1B326 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x30D | 0x430C / 0x1B47F | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x2B8 | 0x433E / 0x1B5B5 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x2BA | 0x4352 / 0x1B645 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x175 | 0x4384 / 0x1B75B | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x300 | 0x43C0 / 0x1B8AD | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x2B3 | 0x4442 / 0x1BB4D | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x312 | 0x446A / 0x1BC38 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | JAZZY.PUB | 0x13F | 0x4550 / 0x1C1D6 | 33 / 0 | 1700 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x112 | 0x2E8C / 0x5564 | 41 / 163 | 205 / 44 | UNKNOWN class / UNKNOWN | O |
| OBSERVATION (1/9) | MODERN.PUB | 0x106 | 0x2F0E / 0x5C0D | 50 / 3 | 1178 / 4 | O / 1178 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x104 | 0x2FA4 / 0x6FD0 | 50 / 3 | 550 / 4 | O / 550 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x16F | 0x2FE0 / 0x73E2 | 58 / 3 | 714 / 4 | O / 714 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x290 | 0x3058 / 0x7924 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x2E3 | 0x30A8 / 0x7B61 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x311 | 0x3102 / 0x7DFC | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x31D | 0x3116 / 0x7E8C | 33 / 0 | 1368 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x37F | 0x312A / 0x8462 | 33 / 0 | 1368 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x381 | 0x313E / 0x8A38 | 33 / 0 | 1368 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x383 | 0x3152 / 0x900E | 33 / 0 | 1368 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x387 | 0x3166 / 0x95E4 | 33 / 0 | 1368 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x38A | 0x317A / 0x9BBA | 33 / 0 | 1368 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x38C | 0x318E / 0xA190 | 33 / 0 | 1368 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x38E | 0x31A2 / 0xA766 | 33 / 0 | 1368 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x391 | 0x31B6 / 0xAD3C | 33 / 0 | 1368 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x398 | 0x31CA / 0xB312 | 33 / 0 | 1368 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x3A1 | 0x31DE / 0xB8E8 | 33 / 0 | 1368 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x3A3 | 0x31F2 / 0xBEBE | 33 / 0 | 1368 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x3A5 | 0x3206 / 0xC494 | 33 / 0 | 1368 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x3A7 | 0x321A / 0xCA6A | 33 / 0 | 1368 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x3A9 | 0x322E / 0xD040 | 33 / 0 | 1368 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x3AE | 0x3242 / 0xD616 | 33 / 0 | 1368 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x3B6 | 0x3260 / 0xDC16 | 33 / 0 | 1368 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x3B8 | 0x3274 / 0xE1EC | 33 / 0 | 1368 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x3BA | 0x3288 / 0xE7C2 | 33 / 0 | 1380 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x3BC | 0x329C / 0xEDA4 | 33 / 0 | 1368 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x3BE | 0x32B0 / 0xF37A | 33 / 0 | 1368 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x288 | 0x3454 / 0x1035B | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x3C5 | 0x34CC / 0x1065E | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x147 | 0x34FE / 0x107C0 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x421 | 0x35D0 / 0x10C92 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x301 | 0x3602 / 0x10E1A | 33 / 0 | 5904 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x309 | 0x3616 / 0x125A8 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x448 | 0x367A / 0x1283C | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x431 | 0x3922 / 0x13821 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x135 | 0x3990 / 0x13A7A | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x1CD | 0x39B8 / 0x13B72 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x1E5 | 0x3A58 / 0x13EE2 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x12E | 0x3A76 / 0x13F98 | 33 / 0 | 3916 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x12A | 0x3A8A / 0x14F62 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x443 | 0x3DDC / 0x162DA | 33 / 0 | 7016 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x500 | 0x4174 / 0x19421 | 33 / 0 | 6924 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x390 | 0x448A / 0x1C525 | 33 / 0 | 6924 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x345 | 0x44BC / 0x1E1CF | 33 / 0 | 2872 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x388 | 0x4B4C / 0x20F09 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x57E | 0x4B6A / 0x20FC1 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x52B | 0x4BC4 / 0x211E8 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x513 | 0x4C1E / 0x213F2 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x372 | 0x4C3C / 0x214B2 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x379 | 0x4C6E / 0x215F6 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x43B | 0x4D22 / 0x219F2 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x437 | 0x4D36 / 0x21A82 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | MODERN.PUB | 0x591 | 0x4D68 / 0x21B98 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x112 | 0x3292 / 0x4FE2 | 41 / 163 | 211 / 44 | UNKNOWN class / UNKNOWN | O |
| OBSERVATION (1/9) | PLAIN.PUB | 0x106 | 0x3314 / 0x56D1 | 50 / 3 | 1362 / 4 | O / 1362 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x104 | 0x33AA / 0x7472 | 50 / 3 | 741 / 4 | O / 741 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x126 | 0x33E6 / 0x78C5 | 58 / 3 | 304 / 4 | O / 304 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x248 | 0x3454 / 0x7C57 | 33 / 0 | 1908 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x24A | 0x3468 / 0x8449 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x236 | 0x349A / 0x8583 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x22D | 0x34FE / 0x8829 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x47D | 0x3558 / 0x8A7D | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x243 | 0x35B2 / 0x8C8F | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x332 | 0x360C / 0x8EE7 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x195 | 0x367A / 0x91CD | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x1F2 | 0x385A / 0x9D69 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x24C | 0x388C / 0x9EEF | 33 / 0 | 2260 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x288 | 0x38A0 / 0xA841 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x21F | 0x38FA / 0xAA8D | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x285 | 0x394A / 0xACA5 | 33 / 0 | 1900 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x2A5 | 0x3BE8 / 0xC410 | 33 / 0 | 1896 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x1CD | 0x3BFC / 0xCBF6 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x2EB | 0x3C42 / 0xCD8E | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x1FD | 0x3CE2 / 0xD126 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x2E9 | 0x3F58 / 0xE020 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x2E0 | 0x3F76 / 0xE0D6 | 33 / 0 | 2260 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x35A | 0x3F8A / 0xEA28 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x2D4 | 0x3FC6 / 0xEB80 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x273 | 0x4002 / 0xECCC | 33 / 0 | 1908 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x2A8 | 0x42FA / 0x1057D | 33 / 0 | 1452 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x1D4 | 0x4606 / 0x11FE7 | 33 / 0 | 2260 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x3ED | 0x4714 / 0x12FDA | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x3D9 | 0x4728 / 0x1306A | 33 / 0 | 2184 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x3EF | 0x475A / 0x13A00 | 33 / 0 | 1492 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x17F | 0x49EE / 0x1504F | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x204 | 0x4AB6 / 0x15528 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x488 | 0x4B06 / 0x15716 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x45E | 0x4B42 / 0x158AF | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x389 | 0x4B74 / 0x159D8 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x484 | 0x4B9C / 0x15AC3 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | PLAIN.PUB | 0x469 | 0x4BC4 / 0x15BAE | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x119 | 0x3880 / 0x3DC6 | 41 / 163 | 205 / 44 | UNKNOWN class / UNKNOWN | O |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x104 | 0x395C / 0x482A | 38 / 67 | 626 / 4 | O / 626 | none |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x116 | 0x3998 / 0x4BB8 | 53 / 8 | 200 / 200 | UNKNOWN class / UNKNOWN | F |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x124 | 0x3A1A / 0x5022 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x15A | 0x3A2E / 0x50B2 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x155 | 0x3A9C / 0x53D6 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x1D5 | 0x3AB0 / 0x5466 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x118 | 0x3AC4 / 0x54F6 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x15B | 0x3AEC / 0x5658 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x111 | 0x3B1E / 0x581A | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x1AB | 0x3C18 / 0x5FDC | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x1C3 | 0x3C54 / 0x6204 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x127 | 0x3C86 / 0x6360 | 34 / 0 | 18 / 8 | F / 8 | none |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x115 | 0x3D08 / 0x67B8 | 34 / 0 | 18 / 8 | F / 8 | none |

| Basis / coverage | File / ID | O fields n / duplicate / q | First / required first | Final | Nonincreasing adjacent offset pairs (zero-based indices) |
| --- | --- | --- | --- | --- | --- |
| OBSERVATION (1/9) | JAZZY.PUB / 0x106 | 80 / 80 / 1076 | 324 / 324 | 1076 | 78 |
| OBSERVATION (1/9) | JAZZY.PUB / 0x104 | 24 / 24 / 583 | 5 / 100 | 583 | 4,7 |
| OBSERVATION (1/9) | JAZZY.PUB / 0x124 | 38 / 38 / 408 | 4294967295 / 156 | 408 | 0,3,12,13,18,23,26,29,31,33,36 |
| OBSERVATION (1/9) | MODERN.PUB / 0x106 | 80 / 80 / 1156 | 324 / 324 | 1156 | 17 |
| OBSERVATION (1/9) | MODERN.PUB / 0x104 | 16 / 16 / 528 | 68 / 68 | 528 | 3,6 |
| OBSERVATION (1/9) | MODERN.PUB / 0x16F | 81 / 81 / 692 | 328 / 328 | 692 | 1,12,13,15,22,26,30,38,42,46,60,62,65,66,69 |
| OBSERVATION (1/9) | PLAIN.PUB / 0x106 | 88 / 88 / 1340 | 356 / 356 | 1340 | 82 |
| OBSERVATION (1/9) | PLAIN.PUB / 0x104 | 35 / 35 / 719 | 25 / 144 | 719 | 25,27 |
| OBSERVATION (1/9) | PLAIN.PUB / 0x126 | 31 / 31 / 282 | 1 / 128 | 282 | 1,5,6,10,12,13,15,16,18,21,22,26 |
| OBSERVATION (1/9) | SCHOOL.PUB / 0x104 | 23 / 23 / 604 | 96 / 96 | 604 | 4,7,9,14 |

UNKNOWN (5/9 C files): these residual spans may contain additional fields, opaque strings/runs, other offset conventions or internal padding. A matching H+18+q alone does not establish an offset table. No new class rule, text encoding or importer behavior is invented to make the failures disappear.

## References and bounds controls

OBSERVATION (9/9 tested): root index-21 array-tail IDs all resolve to children under the directory-parent candidate, but unchanged index-20 child-tail interpretation fails in CATALOG and SCHOOL. The table separates the previously scoped A selectors (indices 10/20/21/41/49) and B low-byte projections (8/10) from diagnostic C traversal. Zero values are not positive reference evidence. The whole descriptor-tail bounds census uses A literal offsets, B low-byte projected offsets and C literal offsets as diagnostics only; no interpretation is substituted merely because it fits.

| Basis / coverage | File | Root-tail child matches | Index-20 tail child matches | Previously scoped selector bounded / probes; child hits | All-tail bounds failures |
| --- | --- | --- | --- | --- | --- |
| OBSERVATION (1/9) | Sample98.pub | 7/7 | 17/17 | 72/73; 27 | 3 |
| OBSERVATION (1/9) | Sample2000.pub | 7/7 | 17/17 | 14/14; 3 | 1 |
| OBSERVATION (1/9) | CATALOG.PUB | 11/11 | 32/51 | 0/0; 0 | 11 |
| OBSERVATION (1/9) | JAZZY.PUB | 4/4 | 19/19 | 0/0; 0 | 77 |
| OBSERVATION (1/9) | MODERN.PUB | 4/4 | 19/19 | 0/0; 0 | 55 |
| OBSERVATION (1/9) | mspub.pub | 6/6 | 11/11 | 62/63; 19 | 3 |
| OBSERVATION (1/9) | PLAIN.PUB | 4/4 | 20/20 | 0/0; 0 | 39 |
| OBSERVATION (1/9) | SCHOOL.PUB | 7/7 | 50/70 | 0/0; 0 | 16 |
| OBSERVATION (1/9) | zmspub.pub | 6/6 | 11/11 | 62/63; 19 | 3 |

OBSERVATION (3/9 A files): the old scoped selector interpretation produces 65 nonzero child-ID hits, all corroborated by directory parents; remaining bounded values are zero. B's scoped 14 projected probes still give its three corroborated hits in 1/9 files. This adds no new positive B instance. The parked A +208/+370 and other diagnostic out-of-bounds fields are not treated as fatal to a bounds-checked reader; they must be skipped/preserved as unknown. Attribute words remain UNKNOWN; C introduces values outside the four previously seen A/B words.

| Basis / coverage | File | Attribute word: count |
| --- | --- | --- |
| OBSERVATION (1/9) | Sample98.pub | 0x16c0: 21; 0x1640: 23; 0x16e0: 5; 0x1740: 5 |
| OBSERVATION (1/9) | Sample2000.pub | 0x16c0: 20; 0x1640: 23; 0x16e0: 6; 0x1740: 5 |
| OBSERVATION (1/9) | CATALOG.PUB | 0x16c8: 1; 0x16c0: 17; 0x1640: 21; 0x16e0: 12; 0x1440: 8; 0x1748: 4; 0x1648: 12; 0x1740: 5; 0x1448: 18; 0x1548: 3 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x16c8: 1; 0x16c0: 13; 0x1640: 360; 0x16e0: 16; 0x1448: 1; 0x1648: 84; 0x1660: 19; 0x1740: 53; 0x1540: 60; 0x1548: 4 |
| OBSERVATION (1/9) | MODERN.PUB | 0x16c8: 1; 0x16c0: 13; 0x1640: 809; 0x16e0: 16; 0x1448: 1; 0x1648: 80; 0x1740: 23; 0x1540: 8; 0x1660: 27; 0x1748: 1 |
| OBSERVATION (1/9) | mspub.pub | 0x16c0: 13; 0x1640: 15; 0x16e0: 11 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x16c8: 1; 0x16c0: 13; 0x1640: 484; 0x16e0: 16; 0x1448: 1; 0x1740: 25; 0x1648: 105; 0x1540: 80; 0x1660: 10 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x16c8: 1; 0x16c0: 16; 0x1640: 11; 0x16e0: 11; 0x1540: 1; 0x1440: 1; 0x1448: 31; 0x1648: 35; 0x1740: 11; 0x1748: 1 |
| OBSERVATION (1/9) | zmspub.pub | 0x16c0: 13; 0x1640: 13; 0x16e0: 11; 0x1740: 2 |

OBSERVATION (4/9 A/B files): a conservative selector graph was rerun separately from the directory-parent tree. For A it includes only the prior fifteen repeatedly corroborated fixed-selector families plus checked index-20 and root array tails; for B it includes the three checked descriptor-10 projections plus checked index-20/root tails. Every included edge follows the independently acyclic directory-parent relation, and each graph is a partial forest with no dangling included ID. Multiple indegree-zero vertices include disconnected records and do not count pages. Nonselector controls use the same aligned fixed-prefix census as the own earlier notes, not random data. All positive reference conclusions still require those parent controls.

| Basis / coverage | File | Included edges / indegree-zero vertices | Cycles / dangling IDs | Nonselector fields / ID hits / parent hits |
| --- | --- | --- | --- | --- |
| OBSERVATION (1/9) | Sample98.pub | 45 / 9 | 0 / 0 | 261 / 2 / 0 |
| OBSERVATION (1/9) | Sample2000.pub | 27 / 27 | 0 / 0 | 270 / 2 / 0 |
| OBSERVATION (1/9) | mspub.pub | 33 / 6 | 0 / 0 | 191 / 2 / 0 |
| OBSERVATION (1/9) | zmspub.pub | 33 / 6 | 0 / 0 | 191 / 2 / 0 |

UNKNOWN (5/9 C files): a semantic selector graph is not constructed by promoting the diagnostic tail words; directory-parent trees and the explicit tail disagreements are all that is established here.

### Every tail-ID parent disagreement

OBSERVATION (2/9 files): these are all 39 failures of the unchanged index-20 tail-child interpretation. The other seven files have no disagreement among the tested tail IDs. A missing target is not silently resolved and a differing parent is not rewritten.

| Basis / coverage | File | Source ID | Absolute tail field | Target ID | Target's directory parent |
| --- | --- | --- | --- | --- | --- |
| OBSERVATION (1/9) | CATALOG.PUB | 0x20B | 0x47FD | 0x11E | 0x11A |
| OBSERVATION (1/9) | CATALOG.PUB | 0x10A | 0x4C47 | 0x19E | 0x108 |
| OBSERVATION (1/9) | CATALOG.PUB | 0x10A | 0x4C49 | 0x19A | 0x108 |
| OBSERVATION (1/9) | CATALOG.PUB | 0x10A | 0x4C51 | 0x120 | 0x108 |
| OBSERVATION (1/9) | CATALOG.PUB | 0x10A | 0x4C53 | 0x121 | 0x108 |
| OBSERVATION (1/9) | CATALOG.PUB | 0x10A | 0x4C55 | 0x13B | 0x108 |
| OBSERVATION (1/9) | CATALOG.PUB | 0x10B | 0x517F | 0x178 | 0x10D |
| OBSERVATION (1/9) | CATALOG.PUB | 0x10B | 0x5181 | 0x1A1 | 0x10D |
| OBSERVATION (1/9) | CATALOG.PUB | 0x10B | 0x5183 | 0x17C | 0x10D |
| OBSERVATION (1/9) | CATALOG.PUB | 0x10B | 0x5187 | 0x1A7 | 0x10D |
| OBSERVATION (1/9) | CATALOG.PUB | 0x10B | 0x518B | 0x161 | 0x10D |
| OBSERVATION (1/9) | CATALOG.PUB | 0x10B | 0x518D | 0x16C | 0x10D |
| OBSERVATION (1/9) | CATALOG.PUB | 0x10B | 0x518F | 0x139 | 0x10D |
| OBSERVATION (1/9) | CATALOG.PUB | 0x10B | 0x5191 | 0x140 | 0x10D |
| OBSERVATION (1/9) | CATALOG.PUB | 0x10E | 0x566D | 0x141 | 0x1AA |
| OBSERVATION (1/9) | CATALOG.PUB | 0x10E | 0x566F | 0x11C | 0x1AA |
| OBSERVATION (1/9) | CATALOG.PUB | 0x10E | 0x5671 | 0x143 | 0x1AA |
| OBSERVATION (1/9) | CATALOG.PUB | 0x10E | 0x5673 | 0x12D | 0x1AA |
| OBSERVATION (1/9) | CATALOG.PUB | 0x10E | 0x5679 | 0x14D | 0x1AA |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x199 | 0x4D0E | 0x198 | 0x10F |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x10E | 0x554C | 0x171 | 0x1A2 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x10E | 0x554E | 0x20B | 0x1A2 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x10E | 0x5550 | 0x207 | 0x1A2 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x10E | 0x5552 | 0x1B8 | 0x1A2 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x10E | 0x5554 | 0x1D6 | 0x1A2 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x10E | 0x5556 | 0x162 | 0x1A2 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x10E | 0x5558 | 0x1A0 | 0x1A2 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x10E | 0x555A | 0x16D | 0x1A2 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x10E | 0x555C | 0x176 | 0x1A2 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x10E | 0x555E | 0x14E | 0x1A2 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x10E | 0x5560 | 0x15D | 0x1A2 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x10E | 0x5562 | 0x1DF | 0x1A2 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x10E | 0x5564 | 0x172 | 0x1A2 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x10E | 0x5566 | 0x1AA | 0x1A2 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x10E | 0x5568 | 0x197 | 0x1A2 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x10E | 0x556A | 0x1CB | 0x1A2 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x10E | 0x556C | 0x34D | 0x1A2 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x10E | 0x556E | 0x1CA | 0x1A2 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x10E | 0x5570 | 0x1A3 | 0x1A2 |

### Every out-of-fixed-prefix selector probe

OBSERVATION (9/9 tested): the following grouped rows list all failed selector bounds by source record/index and raw word -> tested offset. Multiple words in a cell remain separate failed probes. This is a list of bounds failures for the stated interpretations, not proof that all descriptors contain unconditional live references. In particular B's mask still has no universal semantic proof, and the five C selector conventions are not decoded.

| Basis / coverage | File | Source ID / index | Raw word -> tested offset |
| --- | --- | --- | --- |
| OBSERVATION (1/9) | Sample98.pub | 0x100 / 21 | 0xD0 -> 208 |
| OBSERVATION (1/9) | Sample98.pub | 0x133 / 2 | 0x172 -> 370 |
| OBSERVATION (1/9) | Sample98.pub | 0x134 / 3 | 0x172 -> 370 |
| OBSERVATION (1/9) | Sample2000.pub | 0x100 / 21 | 0xDC -> 220 |
| OBSERVATION (1/9) | CATALOG.PUB | 0x1F2 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | CATALOG.PUB | 0x194 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | CATALOG.PUB | 0x100 / 22 | 0x12 -> 18; 0x14 -> 20; 0x16 -> 22 |
| OBSERVATION (1/9) | CATALOG.PUB | 0x272 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | CATALOG.PUB | 0x122 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | CATALOG.PUB | 0x144 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | CATALOG.PUB | 0x114 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | CATALOG.PUB | 0x1A0 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | CATALOG.PUB | 0x16E / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x11A / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x11B / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x10E / 22 | 0x12 -> 18; 0x14 -> 20; 0x16 -> 22 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x1B0 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x1B2 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x19A / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x1A8 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x1DE / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x1EC / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x1CE / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x1D4 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x17B / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x18C / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x228 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x203 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x205 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x20B / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x215 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x21A / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x21D / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x21F / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x221 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x223 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x1BF / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x71D / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x34D / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x274 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x23E / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x269 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x6DA / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x1C9 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x206 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x2E2 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x133 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x1B5 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x24F / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x247 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x153 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x188 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x314 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x715 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x6E6 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x6E2 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x6E0 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x6DE / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x6DC / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x302 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x711 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x694 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x2B0 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x163 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x1D7 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x2E5 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x212 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x1D1 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x6C6 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x155 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x22F / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x278 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x166 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x16A / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x168 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x18F / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x700 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x6D0 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x152 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x2F8 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x30C / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x2B7 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x2B9 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x174 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x2FF / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x2B2 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x310 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x13E / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x11A / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x11B / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x10E / 22 | 0x12 -> 18; 0x14 -> 20; 0x16 -> 22 |
| OBSERVATION (1/9) | MODERN.PUB | 0x28F / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x2DC / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x4EB / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x312 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x375 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x380 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x382 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x386 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x389 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x38B / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x38D / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x38F / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x396 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x3A0 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x3A2 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x3A4 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x3A6 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x3A8 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x3AC / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x3B5 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x3B7 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x3B9 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x3BB / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x3BD / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x287 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x62B / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x146 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x420 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x2FE / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x302 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x447 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x2E9 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x136 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x156 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x1E4 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x12F / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x12B / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x442 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x4FE / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x3B3 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x344 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x29D / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x57F / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x52C / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x514 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x371 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x378 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x43C / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x438 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | MODERN.PUB | 0x592 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | mspub.pub | 0x127 / 21 | 0xD0 -> 208 |
| OBSERVATION (1/9) | mspub.pub | 0x10C / 2 | 0x172 -> 370 |
| OBSERVATION (1/9) | mspub.pub | 0x10B / 3 | 0x172 -> 370 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x11A / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x11B / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x10E / 22 | 0x12 -> 18; 0x14 -> 20; 0x16 -> 22 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x247 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x249 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x235 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x22C / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x47B / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x242 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x2DB / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x194 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x1EF / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x28E / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x381 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x210 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x284 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x2A6 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x24B / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x2B0 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x1FC / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x1C1 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x36E / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x356 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x2D5 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x272 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x2A9 / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x34A / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x3EC / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x3DB / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x3EE / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x198 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x173 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x489 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x462 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x38B / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x1B2 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | PLAIN.PUB | 0x46A / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x11F / 2 | 0x16A -> 362 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x156 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x187 / 22 | 0x12 -> 18; 0x14 -> 20; 0x16 -> 22 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x123 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x134 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x157 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x1EA / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x117 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x135 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x17E / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x1AA / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x1A3 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x136 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x110 / 3 | 0x16A -> 362 |
| OBSERVATION (1/9) | zmspub.pub | 0x127 / 21 | 0xD0 -> 208 |
| OBSERVATION (1/9) | zmspub.pub | 0x10C / 2 | 0x172 -> 370 |
| OBSERVATION (1/9) | zmspub.pub | 0x10B / 3 | 0x172 -> 370 |

## Text, ranges and encoding

OBSERVATION (4/9): Quill CONTENTS exists only in Sample98, Sample2000, mspub and zmspub. The two Sample files (2/9) contain six stories each, with IDs 1,3,4,6,7,8 and lengths 86,98,62,80,87,75; sums equal 488 UTF-16 units and TEXT is 976 bytes. Strict UTF-16LE decoding succeeds, each story ends at a u16 CR, and SYID/STRS counts and bounded cumulative ranges agree. These length-by-two and unit-boundary checks support UTF-16LE, rather than merely successful decoding or an ASCII-looking run. An 8-bit reading would leave twice the logical units and interleaved zero bytes, contradicting that same partition. The TEXT payload is identical across these two known files, so this is not a new text-content variation.

OBSERVATION (2/9): mspub and zmspub have SYID count zero, no TEXT/STRS/TCD section and zero Quill text units. Their entire Quill streams are byte-identical; no encoding can be inferred from absent text. Their empty story sets supply negative controls, not positive text-link replication. Their root Contents streams are not identical: 49 byte positions differ despite equal 4152-byte sizes. The earlier proposed owner-pair equality is refuted for these verified attachments.

UNKNOWN (5/9): the C files have no Quill storage/CONTENTS stream anywhere in the container inventory. Their publication text count, story identities, linkage and encoding are not decoded at this checkpoint. Quill unit counts are not applicable, not zero publication text. No index-8 or index-10 record occurs in any of them. The diagnostic zero-based index-0/class-9 cohorts contain 26/65/67/81/29 records respectively, but that numeric class similarity does not prove they are text records. Neither UTF-16 nor an 8-bit code page is assigned to their possible non-Quill text merely from printable bytes or file names. No private sample text is reproduced.

| Basis / coverage | File | Quill bytes | Logical text units / encoding evidence | Positive old-anchor links / candidate records | Other aligned non-anchor fields hitting story IDs | Positive rectangle candidates / contained |
| --- | --- | --- | --- | --- | --- | --- |
| OBSERVATION (1/9) | Sample98.pub | 6144 | 488 / UTF-16LE | 6/8 | idx8: 18/336; idx10: 6/110 | 6/6 |
| OBSERVATION (1/9) | Sample2000.pub | 6144 | 488 / UTF-16LE | 6/8 | idx8: 18/336; idx10: 6/110 | 6/6 |
| UNKNOWN (1/9) | CATALOG.PUB | absent | UNKNOWN / no Quill stream | not testable; zero old-index candidates | not applicable | not testable |
| UNKNOWN (1/9) | JAZZY.PUB | absent | UNKNOWN / no Quill stream | not testable; zero old-index candidates | not applicable | not testable |
| UNKNOWN (1/9) | MODERN.PUB | absent | UNKNOWN / no Quill stream | not testable; zero old-index candidates | not applicable | not testable |
| OBSERVATION (1/9) | mspub.pub | 1024 | 0 / no TEXT | 0/2 | idx8: 0/56; idx10: 0/55 | 0/0 |
| UNKNOWN (1/9) | PLAIN.PUB | absent | UNKNOWN / no Quill stream | not testable; zero old-index candidates | not applicable | not testable |
| UNKNOWN (1/9) | SCHOOL.PUB | absent | UNKNOWN / no Quill stream | not testable; zero old-index candidates | not applicable | not testable |
| OBSERVATION (1/9) | zmspub.pub | 1024 | 0 / no TEXT | 0/2 | idx8: 0/56; idx10: 0/55 | 0/0 |

OBSERVATION (4/9 eligible files): all 20 index-8/index-10 story fields are bounded and contain either zero or a known Quill ID. The two positive files (2/9) give 12/12 positive same-record story-to-rectangle joins; no positive link failure is found. The other eight candidates are zero-story/all-zero rectangles. The five C files cannot enter that test without decoding their text storage. Thus no new positive text-link or rectangle example is added by these seven attachments. The TCD section remains 36 bytes and tied to story 6 in the two known positive files (2/9); no TCD occurs in the other seven (7/9). No new page/order evidence follows from a zero or absent text section.

## Root size probes, scale and translation

OBSERVATION (9/9): the directory's sole root-like record has index 21 and enough fixed prefix for LE u32 reads at +0x14/+0x18. Both candidate dimensions are positive. All derived dimensions below use the pre-existing 36000-units/mm hypothesis, not an independent measurement. A4 matches occur in 3/9; Letter in 2/9; a 5.5-by-8.5-inch half-Letter numerical fit in 1/9; 20-inch squares in 2/9; a 22-inch square in 1/9. The square values are called custom-size candidates, not confirmed physical pages or proof that the root describes a page instead of another coordinate domain.

| Basis / coverage | File | Root ID / start | Absolute width / height fields | Stored W / H | Candidate mm / numerical size |
| --- | --- | --- | --- | --- | --- |
| OBSERVATION (1/9) | Sample98.pub | 0x100 / 0x5E0 | 0x5F4 / 0x5F8 | 7560000 / 10692000 | 210 x 297 / A4 |
| OBSERVATION (1/9) | Sample2000.pub | 0x100 / 0x612 | 0x626 / 0x62A | 7560000 / 10692000 | 210 x 297 / A4 |
| OBSERVATION (1/9) | CATALOG.PUB | 0x214 / 0x308A | 0x309E / 0x30A2 | 5029200 / 7772400 | 139.7 x 215.9 / half-Letter 5.5 x 8.5 in |
| OBSERVATION (1/9) | JAZZY.PUB | 0x100 / 0x4870 | 0x4884 / 0x4888 | 18288000 / 18288000 | 508 x 508 / custom 20 x 20 in candidate |
| OBSERVATION (1/9) | MODERN.PUB | 0x100 / 0x54CA | 0x54DE / 0x54E2 | 18288000 / 18288000 | 508 x 508 / custom 20 x 20 in candidate |
| OBSERVATION (1/9) | mspub.pub | 0x127 / 0x540 | 0x554 / 0x558 | 7772400 / 10058400 | 215.9 x 279.4 / Letter |
| OBSERVATION (1/9) | PLAIN.PUB | 0x100 / 0x4F48 | 0x4F5C / 0x4F60 | 20116800 / 20116800 | 558.8 x 558.8 / custom 22 x 22 in candidate |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x1B6 / 0x3D26 | 0x3D3A / 0x3D3E | 7772400 / 10058400 | 215.9 x 279.4 / Letter |
| OBSERVATION (1/9) | zmspub.pub | 0x127 / 0x540 | 0x554 / 0x558 | 7560000 / 10692000 | 210 x 297 / A4 |

OBSERVATION (9/9): numeric index-44 records also contain a duplicate of their root-size pair at record +4/+8. Matching record IDs are listed below. This corroborates a repeated numeric relationship, not physical page identity; C's numeric index convention differs from A/B.

| Basis / coverage | File | Index-44 duplicate-pair record IDs |
| --- | --- | --- |
| OBSERVATION (1/9) | Sample98.pub | 0x121 |
| OBSERVATION (1/9) | Sample2000.pub | 0x121 |
| OBSERVATION (1/9) | CATALOG.PUB | 0x111, 0x103 |
| OBSERVATION (1/9) | JAZZY.PUB | 0x116 |
| OBSERVATION (1/9) | MODERN.PUB | 0x116 |
| OBSERVATION (1/9) | mspub.pub | 0x10f |
| OBSERVATION (1/9) | PLAIN.PUB | 0x116 |
| OBSERVATION (1/9) | SCHOOL.PUB | 0x125, 0x102 |
| OBSERVATION (1/9) | zmspub.pub | 0x10f |

OBSERVATION (2/9 positive geometry files): the known six rectangles in Sample98 and six in Sample2000 remain positive, distinct per file and contained under the candidate half-page translation (12/12). The other-class fixed-prefix numerical controls still pass 2/24 in each. mspub/zmspub (2/9) have only zero old-index frame controls; their different Letter/A4 root values provide size variation without positive frame positions. Their other-class controls each pass 1/23. The five C files (5/9) have no proven old-index text frames or story identities, so no containment or coordinate-origin conclusion is transferred to them.

INFERENCE (size probes in 9/9, physical conclusion in 0/9): the varied root numbers are consistent with the candidate common physical grid and broaden numeric plausibility. They do not separate unit scale from coordinate translation: no new nonzero, story-identified box with independently known physical position exists, and the varied C roots are not independently proven page dimensions. The earlier alternative-origin controls therefore remain valid. A true unit conversion rescales widths/heights; a translation changes left/top without changing width/height. Unknown physical widths and unknown root roles cannot distinguish these effects from numerical data alone.

UNKNOWN (9/9): independently documented physical page/frame placement is still absent. One known unrotated story-linked frame rectangle on a known-size A/B page would test scale from its width/height and translation from its left/top, without treating containment as an oracle. The five C files do not constitute a second B replication; B remains one file (1/9). Positive A text/frame evidence still rests on Sample98 alone (1/9), while A's empty-story controls now cover mspub and zmspub (2/9).

## Ranked unknowns and next smallest step

1. UNKNOWN (5/9 C files, all 5/5 third-family files): unique code page, control-marker rendering and displayed text/page semantics. OBSERVATION: AP21 finds a counted map in index 26/class 10 linking 28/72/74/88/30 byte spans to direct directory IDs. All 292 spans cover the AP20 core buffers contiguously and end in CR/LF plus form feed; the final two-byte CR/LF trailer is outside the map. Shifted/other-class controls reject competing rules. The byte-range-to-record link is no longer UNKNOWN for these samples; page ownership and visual reading order are not established.
2. UNKNOWN (5/9 C files): the 190 failed/unmapped strict-transfer records, including 180 missing/mismatching extent rules and ten matching-length/invalid-offset-table records, plus 39 tail-parent disagreements in 2/9 files. No tail or padding interpretation is generalized to remove these counterexamples.
3. UNKNOWN (9/9 physical evidence absent): page identity/order, root-size semantics and actual frame unit/origin. A jointly documented physical page/frame observation remains the smallest decisive placement test; new root-size plausibility alone does not close it. OBSERVATION: AP21 joins 292 third-family ranges to positive co-located rectangle probes; the ten unlinked index-0/index-1 controls are all zero. Half-root containment fails for 58 linked records in CATALOG/SCHOOL (2/5 third-family files), while three files admit multiple origins. INFERENCE: the next smallest existing-data test is an ancestor-relative coordinate-base check on those parent chains, preserving the need for independent physical measurement.
4. UNKNOWN (A 3/9, B 1/9): selector/tag applicability, unsupported offsets, full reference semantics, flags and authoritative story-field widths. Scoped A reference hits replicate in zmspub, but zero-only fields remain unproved and B gains no new positive instance.
5. UNKNOWN (B still 1/9; positive A text still 1/9): saving-version generality, independent B manufacture and a second positive A text/geometry sample. zmspub is now verified and its purported Contents equality with mspub is refuted; that identity question is closed for these attachments, not for every file carrying those names.

OBSERVATION (checkpoint): all seven requested identities pass; all nine containers, headers, directories, descriptor-region walks, field-driven extent candidates, available Quill partitions and root probes are accounted for, including explicit failures and untestable cases. The third layout, absent text store and incomplete C framing are the first checkpoint. No wider C text/geometry decoding, importer implementation, full CI, environment publication or pull request is attempted.

## Third-family root text buffer

OBSERVATION (5/5 third-family files; AP20): all seven supplied attachment sizes and SHA-256 values were reverified before use. The five third-family files were freshly opened with the existing container dependency; the 700 stream paths and advertised sizes exactly match the exhaustive inventories above. Fresh root Contents snapshots match the prior verified copies. Only these already listed binary inputs and the repository's own notes supply this step; no new source, external format specification, scoring export or third-party reader is consulted. No binary or private paragraph is committed; extracted bytes remain ignored. No importer or environment setting/script change, full CI or pull request is made.

### Stream candidates and exhaustive inventories

OBSERVATION (5/5): the complete stream-by-stream lists, including every embedded object stream and size, remain in the linked tables. Re-enumeration matched 700/700 entries; no Quill storage or Quill CONTENTS stream exists. Root Contents is the best publication-text candidate because its control fields select the bounded character-rich range below, which lies outside both the layout table and record directory. Embedded object streams were inventoried, not decoded or ruled out as additional text sources.

| Basis / coverage | File | Exhaustive current inventory | Streams | Root Contents bytes | Best candidate range |
| --- | --- | --- | ---: | ---: | --- |
| OBSERVATION (1/5) | CATALOG.PUB | [all paths and sizes](#catalogpub) | 30 | 23990 | root Contents [0x2AE,0x173E) |
| OBSERVATION (1/5) | JAZZY.PUB | [all paths and sizes](#jazzypub) | 311 | 121952 | root Contents [0x2AE,0x1D6D) |
| OBSERVATION (1/5) | MODERN.PUB | [all paths and sizes](#modernpub) | 148 | 149719 | root Contents [0x2AE,0x1D20) |
| OBSERVATION (1/5) | PLAIN.PUB | [all paths and sizes](#plainpub) | 166 | 94770 | root Contents [0x2AE,0x1E74) |
| OBSERVATION (1/5) | SCHOOL.PUB | [all paths and sizes](#schoolpub) | 45 | 26606 | root Contents [0x2AE,0x2BF2) |

### CATALOG bounds first

OBSERVATION (single-file, 1/5): CATALOG's header u32 +0x12 gives control C=0x280 and +0x16 gives directory D=0x2C8A. The control's stored length 46 ends at 0x2AE, so the previously unassigned interval [0x2AE,0x2C8A) has exactly 10716 bytes. Control-relative +0x0E (absolute 0x28E) contains S=0x2AE; control-relative +0x12 (absolute 0x292) contains P=0x173C. These are bounded, unaligned LE u32 values. They partition the character-rich interval without scanning for a known paragraph or assuming Quill.

OBSERVATION (single-file, 1/5): [S,P) contains 5262 non-NUL bytes; byte P-1 is form feed. At P there is an additional CR/LF pair, then 194 zero bytes to the next 512-byte boundary F=0x1800. The remaining [0x1800,D) has 5258 bytes and is not decoded. Thus 5262 + 2 + 194 + 5258 = 10716 exactly. The candidate text span [S,P+2) includes 5264 single-byte units, including the observed terminal CR/LF. Its ASCII evidence can be illustrated by the single short word "Your"; no paragraph is reproduced.

UNKNOWN (5/5): P is observed to point at the final CR/LF pair, not proved to be an ordinary exclusive-end pointer. The logical role of that trailer, form feeds and other control markers remains undecoded. Counts below explicitly distinguish the core [S,P) from [S,P+2); they are raw decoded units, not a claimed count of visible characters or every text object in the publication.

### The 64-entry table is a layout table, not a text directory

OBSERVATION (5/5): root Contents [0x22,0x280) is 606 bytes: u16 64 at 0x22, u16 6 at 0x24, then 64 variable-width entries consuming exactly 602 bytes from 0x26. An entry reads three LE u16 words followed by u16 tail words through a literal zero. With k nonzero tail words its serialized width is 6+2*(k+1); it is not a fixed-width 64-row directory. Widths are 8 bytes for 52 entries, 10 for eight, 12 for two, 24 for one and 58 for one, in every file (64/64 bounded entries per file; 320/320 overall). No entry fails these bounds or width arithmetic.

| Basis / coverage | Location within entry | What the samples establish | Limit |
| --- | --- | --- | --- |
| OBSERVATION (5/5) | Word 0 at +0 / u16 | Its high byte equals record H for every referencing directory record under zero-based entry indexing: 2545/2545. H locates the tested fixed-prefix boundary. | Low byte is not proved to be a length, and H is not total record extent. |
| OBSERVATION (5/5) | Word 1 at +2 / u16 | A stable numeric class code groups the sampled extent tests. | Class semantics are not record meanings; prior framing still has the AP19 failures. |
| OBSERVATION (5/5) | Word 2 at +4 / u16 | Equals the stored array width in 977/977 records where the prior classes 1/9/25 array equation and bounds pass. | Not a universal stride or a text-unit width for other classes. |
| OBSERVATION (5/5) | Tail from +6, u16 through zero | Bounds each entry; 45 nonzero tail words per table. None is an absolute address inside [S,P+2): 0/45 per file. | Relative selectors/reference roles are not generally decoded; these are not a proven text-offset list. |
| UNKNOWN (5/5) | Table-prefix u16 at 0x24 | Literal 6 in all five. | Exact semantic role remains UNKNOWN; it is not promoted to a text pointer. |

OBSERVATION (5/5 geometry of the table, CATALOG numeric examples single-file 1/5): representative entries below use CATALOG's words. All offsets, widths, high bytes, classes, width hints and tails agree across five files; MODERN alone differs in word-0 low byte for indices 2 and 3 (164/166 become 156/158). It does not change entry framing, H, class or width hint. No raw text is in this table.

| Basis / coverage | Zero-based index | Entry [start,end) | Width | Word-0 low / high | Class word | Width-hint word | Tail words before zero |
| --- | ---: | --- | ---: | --- | ---: | ---: | ---: |
| OBSERVATION (1/5 numeric example; bounds 5/5) | 0 | [0x26,0x2E) | 8 | 94 / 92 | 9 | 2 | 0 |
| OBSERVATION (1/5 numeric example; bounds 5/5) | 1 | [0x2E,0x36) | 8 | 96 / 96 | 1 | 14 | 0 |
| OBSERVATION (1/5 numeric example; bounds 5/5) | 2 | [0x36,0x40) | 10 | 164 / 116 | 1 | 8 | 1 |
| OBSERVATION (1/5 numeric example; bounds 5/5) | 3 | [0x40,0x4A) | 10 | 166 / 116 | 1 | 8 | 1 |
| OBSERVATION (1/5 numeric example; bounds 5/5) | 21 | [0xD8,0x112) | 58 | 158 / 136 | 25 | 2 | 25 |
| OBSERVATION (1/5 numeric example; bounds 5/5) | 33 | [0x17A,0x182) | 8 | 0 / 8 | 0 | 0 | 0 |
| OBSERVATION (1/5 numeric example; bounds 5/5) | 34 | [0x182,0x18A) | 8 | 0 / 8 | 0 | 0 | 0 |
| OBSERVATION (1/5 numeric example; bounds 5/5) | 63 | [0x278,0x280) | 8 | 180 / 180 | 0 | 0 | 0 |

OBSERVATION (5/5): the separate record directory is at header pointer D, has u16 count N and maximum ID, and N ten-byte entries at D+4: ID u16 +0, parent candidate u16 +2, absolute record start u32 +4, attribute u16 +8. Each addressed record begins with the zero-based layout-entry index. CATALOG has 101 directory entries, not 64; the five counts are 101/611/979/735/119.

INFERENCE (5/5): the table describes record layout while the directory locates record instances. The ten-byte directory organization resembles A/B; the zero-based 64-entry layout cardinality, extra pre-directory region and lack of Quill distinguish this family. No direct mapping from a table row to a text story is proven.

### One locating rule, tested on five files

OBSERVATION (5/5): read bounded S=u32(C+0x0E), P=u32(C+0x12), and D=u32(header+0x16). Require C+46 <= S < P, P+2 <= D <= actual Contents size, and checked pointer arithmetic. In these samples S=C+46. Verify form feed at P-1, CR/LF at [P,P+2), and zero bytes from P+2 to F=ceil((P+2)/512)*512, with F<=D. These observed checks pass in 5/5 files, with no file or record exception. Treating P as a byte length instead of an absolute address adds 686 bytes past the observed marker and introduces NUL-filled/binary data in every file; the wrong-length control below reports the NUL count.

| Basis / coverage | File | S / P / P+2 / D | Core [S,P) units | Candidate [S,P+2) units | Zero gap to F | Undecoded [F,D) bytes |
| --- | --- | --- | ---: | ---: | --- | ---: |
| OBSERVATION (1/5) | CATALOG.PUB | 0x2AE / 0x173C / 0x173E / 0x2C8A | 5262 | 5264 | 194 to 0x1800 | 5258 |
| OBSERVATION (1/5) | JAZZY.PUB | 0x2AE / 0x1D6B / 0x1D6D / 0x3084 | 6845 | 6847 | 147 to 0x1E00 | 4740 |
| OBSERVATION (1/5) | MODERN.PUB | 0x2AE / 0x1D1E / 0x1D20 / 0x2E7E | 6768 | 6770 | 224 to 0x1E00 | 4222 |
| OBSERVATION (1/5) | PLAIN.PUB | 0x2AE / 0x1E72 / 0x1E74 / 0x3284 | 7108 | 7110 | 396 to 0x2000 | 4740 |
| OBSERVATION (1/5) | SCHOOL.PUB | 0x2AE / 0x2BF0 / 0x2BF2 / 0x3872 | 10562 | 10564 | 14 to 0x2C00 | 3186 |

### Encoding and false-positive controls

OBSERVATION (5/5): strict Windows-1252 decoding yields one unit per byte without replacement, with no NUL in the candidate range. More than 96% of bytes are ASCII printable or tab/CR/LF. Every CR is followed by LF and all LF bytes belong to such pairs. The raw candidate count includes each CR and LF separately plus form feeds and other control markers. It is 36555 units over these five buffers, not a decoded story/page count.

OBSERVATION (5/5): UTF-16LE is the competing experiment, not assumed from A/B. Four even-length buffers decode strictly but yield zero ASCII words of at least four letters and zero CR/LF characters. JAZZY's odd-length 6847-byte range fails strict UTF-16LE decoding as truncated data. Merely succeeding in a Unicode decoder is therefore a false-positive criterion. Byte-sized line breaks, ASCII words and one-byte boundaries support an ASCII-compatible single-byte text buffer, while the UTF-16 interpretation loses those structures. The short word "Lorem" in JAZZY is the second and final quoted content example in this section.

| Basis / coverage | File | Candidate ASCII-compatible bytes / total | CR/LF pairs | Non-ASCII bytes | Strict UTF-16 result / line breaks / ASCII words | Single-byte ASCII words (4+ letters) |
| --- | --- | --- | ---: | ---: | --- | ---: |
| OBSERVATION (1/5) | CATALOG.PUB | 5120 / 5264 | 206 | 0 | accepts / 0 / 0 | 562 |
| OBSERVATION (1/5) | JAZZY.PUB | 6696 / 6847 | 140 | 15 | rejects odd byte count | 804 |
| OBSERVATION (1/5) | MODERN.PUB | 6603 / 6770 | 157 | 13 | accepts / 0 / 0 | 815 |
| OBSERVATION (1/5) | PLAIN.PUB | 6929 / 7110 | 187 | 13 | accepts / 0 / 0 | 842 |
| OBSERVATION (1/5) | SCHOOL.PUB | 10520 / 10564 | 77 | 3 | accepts / 0 / 0 | 1220 |

OBSERVATION (5/5): nearby non-text controls were chosen by region, not by searching for a favorable string. The entire descriptor-table interval [0x22,0x280) has only 57/606 ASCII-compatible bytes in each file, rather than the candidate's >96%. The following-window control starts at P+2 and takes at most the candidate length without crossing D; it contains many NULs and few printable bytes. Its actual denominator is reported, since equal-length space is not available in every file. The wrong-length control reads [S,S+P), bounded in all five, and crosses the character/binary boundary. These controls refute the rule "any decodable slice is text" and the mistaken use of P as a length, not every conceivable non-text encoding.

| Basis / coverage | File | Following window bytes | Following ASCII-compatible / NUL | Wrong-length-window NULs | Table ASCII-compatible / bytes |
| --- | --- | ---: | --- | ---: | --- |
| OBSERVATION (1/5) | CATALOG.PUB | 5264 | 394 / 3430 | 445 | 57 / 606 |
| OBSERVATION (1/5) | JAZZY.PUB | 4887 | 388 / 2944 | 440 | 57 / 606 |
| OBSERVATION (1/5) | MODERN.PUB | 4446 | 299 / 2794 | 479 | 57 / 606 |
| OBSERVATION (1/5) | PLAIN.PUB | 5136 | 355 / 3175 | 540 | 57 / 606 |
| OBSERVATION (1/5) | SCHOOL.PUB | 3200 | 231 / 2349 | 431 | 57 / 606 |

INFERENCE (5/5): these are root publication-text buffers in an ASCII-compatible single-byte representation. Windows-1252 is a usable experimental decode for the observed bytes, not an established unique code page. CATALOG has no high bytes (single-file, 1/5), so its buffer cannot distinguish ASCII-compatible code pages at all. In the other four (4/5), bytes 0x92/0x93/0x94 map to typographic punctuation in Windows-1252, while a Latin-1 decode maps them to C1 controls (15/13/12/3 such controls for JAZZY/MODERN/PLAIN/SCHOOL). Other single-byte code pages can share those mappings. UNKNOWN (5/5): authoritative code-page metadata, semantics of nonprinting markers, logical visible-character totals and whether embedded objects add more publication text. No universal 8-bit code page or replacement policy is claimed.

### Frame/page linkage and next checkpoint

OBSERVATION (5/5): there is no Quill story-ID list and no index-8/index-10 record to reuse A's text-ID join. A diagnostic check on the numeric index-0/class-9 cohort reads u32 at the old +0x58 anchor only where bounded: 268/268 reads are bounded, 260 are zero and none addresses [S,P+2). This does not prove those records are text frames or disprove a small story-ID interpretation. The index-1/class-1 cohort has 34 records; the old +0x66 anchor does not fit any of their 96-byte fixed prefixes, so it is not read as a live fixed-prefix field. Those numeric class similarities cannot establish type equivalence with A.

| Basis / coverage | File | Index-0 +0x58 bounded / cohort | Zeros / absolute-text-address hits | Index-1 +0x66 prefix-bounded / cohort |
| --- | --- | --- | --- | --- |
| OBSERVATION (1/5) | CATALOG.PUB | 26 / 26 | 25 / 0 | 0 / 4 |
| OBSERVATION (1/5) | JAZZY.PUB | 65 / 65 | 65 / 0 | 0 / 9 |
| OBSERVATION (1/5) | MODERN.PUB | 67 / 67 | 63 / 0 | 0 / 9 |
| OBSERVATION (1/5) | PLAIN.PUB | 81 / 81 | 80 / 0 | 0 / 9 |
| OBSERVATION (1/5) | SCHOOL.PUB | 29 / 29 | 27 / 0 | 0 / 3 |

UNKNOWN (5/5): per-story starts/lengths, links from these buffers to frame records, page grouping and reading order. Form feeds and CR/LF counts are not promoted to page/story counts. Unlike the verified A story-ID-to-SYID/STRS association, no cross-structure field here yet corroborates ownership of a particular byte range by a particular frame. Finding root text does not decode that link or establish a faithful editable page.

INFERENCE (next smallest step, 5/5 applicable data): use CATALOG's now-bounded byte buffer to test candidate record fields for starts/lengths or indirection into its control-delimited ranges, with matched-width fields from other index cohorts and separately corroborated zero-length/inactive controls. First require exact bounded ranges and independent record/parent corroboration; do not choose matches from common small numbers or assume a form feed is a page break. Exact code-page selection and trailer/control semantics remain separate unknowns.

OBSERVATION (first AP20 checkpoint): the text locating rule and single-byte evidence replicate in 5/5 files, with zero locating-rule failures. Table framing has 64/64 entries per file and H correlation 2545/2545; candidate buffer units are bounded and counted exactly with their trailer convention stated. Frame/story/page linkage and unique code-page selection remain UNKNOWN. Research stops here; no importer or broader following-region decoding is attempted. SOURCES.md is unchanged because every binary evidence source was already listed.

## Third-family counted text ranges and record links

OBSERVATION (5/5 third-family files; AP21): all seven attachments again match the AP19 sizes and SHA-256 values before use. The evidence remains the same five third-family binaries and own preceding notes; no new source, scoring export, external specification or other reader is consulted. Only derived numbers and structure are recorded; no sample text or byte dump is quoted in this section. Needed stream bytes and experiment outputs remain ignored. No importer, environment setting/script, full CI or pull request changes are made.

### CATALOG search scope and false positives

OBSERVATION (single-file, 1/5): every byte offset that permits a complete u16 or u32 read was scanned within all 101 directory-addressed record spans, all 64 layout entries and all 101 ten-byte directory entries. Positive adjacent equal-width pairs were tested as start/end (SE) and start/length (SL), with 0 <= start < end <= N and N=5264, the AP20 candidate buffer length. Scalar hits use [0,N), including zero. Record prefixes and tails were both included; reads never cross their enclosing span. These scopes are explicit: this is not a claim to exhaust mixed-width, nonadjacent, compressed or arbitrary-indirection encodings.

| Basis / coverage | Domain | u16 hits / fields; records with hits | u32 hits / fields; records with hits | SE16 positive pairs / tested; records | SL16 positive pairs / tested; records | SE32 positive pairs / tested; records | SL32 positive pairs / tested; records |
| --- | --- | --- | --- | --- | --- | --- | --- |
| OBSERVATION (1/5) | record (101 entries) | 8710/11463; 101 | 4539/11261; 99 | 1728/11261; 101 | 2404/11261; 101 | 578/10857; 75 | 695/10857; 79 |
| OBSERVATION (1/5) | descriptor (64 entries) | 477/538; 64 | 198/410; 64 | 93/410; 27 | 138/410; 47 | 2/154; 2 | 3/154; 3 |
| OBSERVATION (1/5) | directory (101 entries) | 509/909; 101 | 0/707; 0 | 74/707; 66 | 140/707; 101 | 0/303; 0 | 0/303; 0 |

OBSERVATION (single-file, 1/5): small headers, record IDs, counts and zeros create thousands of numerical hits. A second scan tested 221180 seeds: every bounded starting byte in each record, width two/four, SE/SL interpretation, and strides 2*w,2*w+2,...,2*w+8. It found 213 ordered nonoverlapping runs of at least three positive spans, including overlapping suffix hypotheses. Exactly one run covered the independently bounded core [0,5262) contiguously from zero: 28 u32-SE pairs at record 0x203 relative +22, stride eight. The different AP20 full length, 5264, is not silently substituted; its final two-byte trailer remains outside this partition.

OBSERVATION (single-file, 1/5): all eight simple across-record families that yielded two or more ordered, nonoverlapping positive pairs were given shifted-offset and other-class controls below. A ninth repeated-header example demonstrates that paragraph-boundary coincidence alone also fails. Counts in shifted cells are positive pairs / eligible fields at offsets -2,-1,+1,+2 bytes from the nominated offset; eligibility requires the whole pair inside its source record. Other-class controls apply the identical offset/width/mode to records whose numeric class differs, not just a different record ID. None of these simple families is selected as a text map: none covers the core and none has the independently corroborated counted-ID structure found below. Control positives show why merely fitting [0,N) or preserving order is insufficient; they do not prove that each field has no other role.

| Basis / coverage | Index, offset, width, mode | Base positive / eligible | Sum of span lengths | Shift controls -2,-1,+1,+2 | Other-class positive / eligible | Rejection |
| --- | --- | --- | ---: | --- | --- | --- |
| OBSERVATION (1/5) | 15, +34, u16, SL | 2/2 | 1645 | -2: 2/2; -1: 0/2; 1: 0/0; 2: 0/0 | 1/67 | partial/subset cover; no counted partition or marker/ID corroboration |
| OBSERVATION (1/5) | 20, +56, u16, SE | 2/2 | 79 | -2: 2/2; -1: 0/2; 1: 0/1; 2: 0/1 | 6/46 | partial/subset cover; no counted partition or marker/ID corroboration |
| OBSERVATION (1/5) | 20, +54, u16, SE | 2/2 | 49 | -2: 1/2; -1: 0/2; 1: 0/2; 2: 2/2 | 16/55 | partial/subset cover; no counted partition or marker/ID corroboration |
| OBSERVATION (1/5) | 15, +34, u16, SE | 2/2 | 11 | -2: 2/2; -1: 0/2; 1: 0/0; 2: 0/0 | 1/67 | partial/subset cover; no counted partition or marker/ID corroboration |
| OBSERVATION (1/5) | 15, +22, u16, SL | 2/2 | 4 | -2: 2/2; -1: 0/2; 1: 2/2; 2: 2/2 | 12/78 | partial/subset cover; no counted partition or marker/ID corroboration |
| OBSERVATION (1/5) | 20, +40, u16, SE | 2/5 | 3 | -2: 3/6; -1: 0/5; 1: 0/4; 2: 4/4 | 4/63 | partial/subset cover; no counted partition or marker/ID corroboration |
| OBSERVATION (1/5) | 6, +31, u16, SL | 2/4 | 512 | -2: 3/4; -1: 3/4; 1: 4/4; 2: 1/4 | 2/66 | partial/subset cover; no counted partition or marker/ID corroboration |
| OBSERVATION (1/5) | 1, +92, u16, SL | 2/4 | 29 | -2: 0/4; -1: 0/4; 1: 0/4; 2: 2/4 | 12/33 | partial/subset cover; no counted partition or marker/ID corroboration |
| OBSERVATION (1/5) | 49, +8, u32, SE | 4/4 | 16 | -2: 0/4; -1: 4/4; 1: 0/4; 2: 0/4 | 11/94 | repeated [0,4) spans overlap; no core cover |

### Surviving rule: counted pairs plus an ID array

OBSERVATION (single-file discovery, replicated 5/5): the zero-based layout entry 26 has high-byte H=4, class code 10 and width hint 10. Each file has exactly one directory record of index 26. Its extent already fits H+22+n*(4+w), with n from u16 T, a duplicate at T+2, w=u16 T+4=10, and T=R+H. This independently bounds the n u32 slots and n ten-byte metadata elements before interpreting text. In all five n is positive and even. No count is inferred from a desired number of paragraphs or frames.

| Basis / coverage | Field / formula | CATALOG evidence | Interpretation and limit |
| --- | --- | --- | --- |
| OBSERVATION (1/5 absolute example; rule 5/5) | Directory entry / source record | Entry 0x2D42 names ID 0x203 at R=0x3FD9; E=0x4303, span 810, index 26, H=4 | Same directory/extent method as before. |
| OBSERVATION (5/5) | n=u16 T; duplicate=u16 T+2; w=u16 T+4 | At 0x3FDD/0x3FDF/0x3FE1: 56/56/10 | n counts slots and metadata elements; number of paired ranges is n/2. Duplicate authority/capacity semantics remain UNKNOWN. |
| OBSERVATION (5/5) | A=T+18; slot i=u32(A+4*i), 0<=i<n | A=0x3FEF; 56 slots fit before 0x40CF | Pair slots 2*j/2*j+1 as relative byte start/end. |
| OBSERVATION (5/5) | u32(A+4*n) | At 0x40CF equals 5262 | Extra boundary word equals control-derived M=P-S, the core length, and repeats final range end. |
| OBSERVATION (5/5) | B=T+22+4*n; element i at B+10*i | B=0x40D3; 56 elements consume 560 bytes to E | Odd/even element phases are tested, not conflated. |
| OBSERVATION (5/5) | u16(B+20*j) | Every even element's first field names one directory ID | The record target for range j; 292/292 hits and distinct per file. |
| OBSERVATION (5/5) | u16(B+20*j+2) | Value 1 when target index is 0; value 0 when target index is 1 | Correlation checked 292/292; named flag meaning is INFERENCE, not decoded semantics. |
| OBSERVATION (5/5) | u16(B+20*j+10) | Every odd element's first field is zero | Zero is not a second target ID. Other metadata bytes remain UNKNOWN. |

INFERENCE (5/5): record index 26 is a central byte-range-to-record map. It does not use a Quill story-ID intermediary or store one range pair directly in every frame prefix. A bounds-checked experimental walker must validate S/P, R/E/H, n parity/duplicate, w, both array spans, the final boundary, each pair and each directory target before accepting a link; checked wide arithmetic must reject overflow. The matching length equation alone is not the reference proof.

| Basis / coverage | File | Source ID / directory entry / R | n / pairs | Slot array / boundary / metadata array | Core end M / source end E | Range and ID checks |
| --- | --- | --- | --- | --- | --- | --- |
| OBSERVATION (1/5) | CATALOG.PUB | 0x203 / 0x2D42 / 0x3FD9 | 56 / 28 | 0x3FEF / 0x40CF / 0x40D3 | 5262 / 0x4303 | 28/28 |
| OBSERVATION (1/5) | JAZZY.PUB | 0x110 / 0x313C / 0x563B | 144 / 72 | 0x5651 / 0x5891 / 0x5895 | 6845 / 0x5E35 | 72/72 |
| OBSERVATION (1/5) | MODERN.PUB | 0x110 / 0x2F36 / 0x62C8 | 148 / 74 | 0x62DE / 0x652E / 0x6532 | 6768 / 0x6AFA | 74/74 |
| OBSERVATION (1/5) | PLAIN.PUB | 0x110 / 0x333C / 0x647B | 176 / 88 | 0x6491 / 0x6751 / 0x6755 | 7108 / 0x6E35 | 88/88 |
| OBSERVATION (1/5) | SCHOOL.PUB | 0x17F / 0x392A / 0x4330 | 60 / 30 | 0x4346 / 0x4436 / 0x443A | 10562 / 0x4692 | 30/30 |

OBSERVATION (5/5): the locating rule frozen from CATALOG passes on all four other files without changing field offsets, widths, count handling, pairing or target lookup. There are zero source-record, range or target-ID exceptions: 292/292 positive spans and links overall. Each file's span IDs are unique; all linked targets are index 0/class 9 or index 1/class 1. Across five files, the distribution is 263 targets of index 0 and 29 of index 1, exactly matching the metadata kind correlation above.

### Controls for the counted rule and metadata links

OBSERVATION (5/5): moving A by -4,-3,-2,-1,+1,+2,+3,+4 bytes while retaining stored n gives 0/8 complete partitions per file, 0/40 overall. Several shifts preserve apparent adjacency while their numerical bounds fail; order alone is not enough. Re-reading as u16-SE, u16-SL or u32-SL also gives 0/3 complete partitions per file. These are the same fields, counts and source records, rather than arbitrary unrelated bytes.

OBSERVATION (5/5): every record of another numeric class is also tested at its own T+18 with the same count-pair/parity/bounds predicate, deliberately relaxing the w=10 requirement. Twenty-one controls have eligible arrays; none forms a complete partition. The other 2519 fail header/count/slot bounds before payload interpretation. This gives 0/2540 other-class full-map matches, not a silent exclusion of unmatched records. The CATALOG eligible controls are IDs 0x197/index 28, 0x111/index 44 and 0x103/index 44; all fail the partition. Other files' eligible control IDs are listed below.

| Basis / coverage | File | Shifted partition passes | Alternative width/mode partition passes | Other-class eligible / tested; partition passes | Eligible control IDs |
| --- | --- | --- | --- | --- | --- |
| OBSERVATION (1/5) | CATALOG.PUB | 0/8 | 0/3 | 3/100; 0 | 0x197, 0x111, 0x103 |
| OBSERVATION (1/5) | JAZZY.PUB | 0/8 | 0/3 | 10/610; 0 | 0x115, 0x116, 0x2E3, 0x261, 0x184, 0x6C4, 0x148, 0x717, 0x6CD, 0x729 |
| OBSERVATION (1/5) | MODERN.PUB | 0/8 | 0/3 | 2/978; 0 | 0x125, 0x116 |
| OBSERVATION (1/5) | PLAIN.PUB | 0/8 | 0/3 | 3/734; 0 | 0x111, 0x103, 0x116 |
| OBSERVATION (1/5) | SCHOOL.PUB | 0/8 | 0/3 | 3/118; 0 | 0x11B, 0x158, 0x191 |

OBSERVATION (5/5): metadata reference controls shift the same even-element u16 field. All probes below remain within their source record; each file uses the same number of fields as its positive spans. Values shown are directory-ID hits / hits whose targets are index 0 or 1. Zero shift uniquely gives all distinct targets of the correct cohorts, whereas shifted ID coincidences can be numerous. For example JAZZY +1 hits 64 IDs but zero target-cohort IDs and only six distinct IDs. Reading the true field as u8 yields 0/292 directory IDs; u16 yields 292/292; reading the combined u32 ID/kind gives only 29/292 hits. No per-field match is accepted just because it resembles a small ID.

| Basis / coverage | File | -2 | -1 | 0 (selected) | +1 | +2 | +4 | +6 | +8 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| OBSERVATION (1/5) | CATALOG.PUB | 0/0 | 0/0 | 28/28 | 2/0 | 0/0 | 0/0 | 0/0 | 0/0 |
| OBSERVATION (1/5) | JAZZY.PUB | 0/0 | 1/0 | 72/72 | 64/0 | 0/0 | 12/2 | 0/0 | 0/0 |
| OBSERVATION (1/5) | MODERN.PUB | 0/0 | 3/1 | 74/74 | 66/0 | 0/0 | 16/1 | 0/0 | 0/0 |
| OBSERVATION (1/5) | PLAIN.PUB | 0/0 | 0/0 | 88/88 | 80/0 | 0/0 | 33/4 | 0/0 | 0/0 |
| OBSERVATION (1/5) | SCHOOL.PUB | 0/0 | 0/0 | 30/30 | 28/0 | 0/0 | 0/0 | 0/0 | 0/0 |

### Coverage, markers and paragraph boundaries

OBSERVATION (5/5): the paired spans are already ascending and adjacent in serialized array order; no sorting or gap filling is performed. They cover [0,M) exactly, with positive lengths and no overlaps or gaps. They leave precisely the AP20 terminal CR/LF at [M,M+2) outside every mapped span. Doubling offsets as if they counted UTF-16 units overruns the core for many ranges and fails complete coverage in 5/5; observed one-byte units preserve every marker boundary.

OBSERVATION (5/5): each mapped span contains exactly one form feed, its final byte at end-1; that form feed is immediately preceded by CR/LF at [end-3,end-1). Hence end itself is not a terminal CR/LF boundary: it is one byte later, after the form-feed marker. No boundary splits a CR/LF pair. CR/LF pairs after a boundary occur once per file except twice in SCHOOL. This distinguishes the mapped ranges from arbitrary paragraph runs. The role of form feed as a storage delimiter is corroborated; calling it a displayed page break is not proven.

| Basis / coverage | File | Positive spans / adjacent joins | Core bytes covered / full candidate bytes | Spans with exactly one terminal FF / preceding CRLF | Core CRLF pairs / full-buffer CRLF pairs | End at CRLF boundary / CRLF immediately after end | Split CRLF pairs |
| --- | --- | --- | --- | --- | --- | --- | --- |
| OBSERVATION (1/5) | CATALOG.PUB | 28 / 27 | 5262 / 5264 | 28/28 / 28/28 | 205 / 206 | 0 / 1 | 0 |
| OBSERVATION (1/5) | JAZZY.PUB | 72 / 71 | 6845 / 6847 | 72/72 / 72/72 | 139 / 140 | 0 / 1 | 0 |
| OBSERVATION (1/5) | MODERN.PUB | 74 / 73 | 6768 / 6770 | 74/74 / 74/74 | 156 / 157 | 0 / 1 | 0 |
| OBSERVATION (1/5) | PLAIN.PUB | 88 / 87 | 7108 / 7110 | 88/88 / 88/88 | 186 / 187 | 0 / 1 | 0 |
| OBSERVATION (1/5) | SCHOOL.PUB | 30 / 29 | 10562 / 10564 | 30/30 / 30/30 | 76 / 77 | 0 / 2 | 0 |

OBSERVATION (single-file, 1/5): the complete CATALOG span/target census below is the positive discovery evidence. It contains only offsets and IDs, not text. For range j, its pair fields are A+8*j and A+8*j+4 and its target field is B+20*j. The first range is not selected by quoting or searching a private phrase.

| Basis / coverage | j | Byte range [start,end) | Target ID / index | Target-ID field | Immediate directory parent |
| --- | ---: | --- | --- | --- | --- |
| OBSERVATION (1/5) | 0 | [0,28) | 0x274 / 0 | 0x40D3 | 0x1AB |
| OBSERVATION (1/5) | 1 | [28,344) | 0x152 / 0 | 0x40E7 | 0x10A |
| OBSERVATION (1/5) | 2 | [344,364) | 0x12D / 0 | 0x40FB | 0x1AA |
| OBSERVATION (1/5) | 3 | [364,374) | 0x14D / 0 | 0x410F | 0x1AA |
| OBSERVATION (1/5) | 4 | [374,381) | 0x140 / 0 | 0x4123 | 0x10D |
| OBSERVATION (1/5) | 5 | [381,391) | 0x13A / 0 | 0x4137 | 0x10B |
| OBSERVATION (1/5) | 6 | [391,395) | 0x508 / 0 | 0x414B | 0x503 |
| OBSERVATION (1/5) | 7 | [395,598) | 0x150 / 1 | 0x415F | 0x10A |
| OBSERVATION (1/5) | 8 | [598,1363) | 0x19A / 0 | 0x4173 | 0x108 |
| OBSERVATION (1/5) | 9 | [1363,2128) | 0x141 / 0 | 0x4187 | 0x1AA |
| OBSERVATION (1/5) | 10 | [2128,2138) | 0x139 / 0 | 0x419B | 0x10D |
| OBSERVATION (1/5) | 11 | [2138,2463) | 0x130 / 1 | 0x41AF | 0x10E |
| OBSERVATION (1/5) | 12 | [2463,2732) | 0x120 / 0 | 0x41C3 | 0x108 |
| OBSERVATION (1/5) | 13 | [2732,2752) | 0x19E / 0 | 0x41D7 | 0x108 |
| OBSERVATION (1/5) | 14 | [2752,3021) | 0x178 / 0 | 0x41EB | 0x10D |
| OBSERVATION (1/5) | 15 | [3021,3290) | 0x170 / 0 | 0x41FF | 0x10B |
| OBSERVATION (1/5) | 16 | [3290,3310) | 0x1A7 / 0 | 0x4213 | 0x10D |
| OBSERVATION (1/5) | 17 | [3310,3330) | 0x19D / 0 | 0x4227 | 0x10B |
| OBSERVATION (1/5) | 18 | [3330,3351) | 0x13F / 0 | 0x423B | 0x10E |
| OBSERVATION (1/5) | 19 | [3351,3364) | 0x115 / 0 | 0x424F | 0x1A3 |
| OBSERVATION (1/5) | 20 | [3364,3405) | 0x16B / 1 | 0x4263 | 0x10E |
| OBSERVATION (1/5) | 21 | [3405,3409) | 0x165 / 0 | 0x4277 | 0x11E |
| OBSERVATION (1/5) | 22 | [3409,4174) | 0x179 / 0 | 0x428B | 0x10B |
| OBSERVATION (1/5) | 23 | [4174,4939) | 0x17C / 0 | 0x429F | 0x10D |
| OBSERVATION (1/5) | 24 | [4939,4966) | 0x187 / 0 | 0x42B3 | 0x10A |
| OBSERVATION (1/5) | 25 | [4966,4983) | 0x116 / 0 | 0x42C7 | 0x1A3 |
| OBSERVATION (1/5) | 26 | [4983,5252) | 0x11C / 0 | 0x42DB | 0x1AA |
| OBSERVATION (1/5) | 27 | [5252,5262) | 0x13B / 0 | 0x42EF | 0x108 |

### Frame rectangles and parent chains

OBSERVATION (5/5): every target directory ID resolves once, and its acyclic parent chain reaches the same independently located root-like record as the rest of its file (292/292). Immediate parents are index 20 or 15, not necessarily the range-map source; the table below records those cohort counts. No decoded page-count or displayed page identity validates those collections as pages. Serialized range order is buffer order, not proven reading or page order.

OBSERVATION (5/5): the target records carry signed32 probes at +0x06/+0x0A/+0x0E/+0x12, as in the earlier geometry experiment. All 292 have positive width and height. In each file exactly one unlinked index-0 record and one unlinked index-1 record remain; all ten have zero rectangles. They are explicit negative controls, not unmatched positive text spans. This co-location plus counted IDs and zero controls supports the interpretation of the targets as text hosts, while actual displayed geometry remains inferred.

| Basis / coverage | File | Targets index 0 / 1 | Parent-index counts; distinct parents | Positive rectangles / targets | Half-root translated containment | Untranslated containment | Unlinked zero-rectangle IDs |
| --- | --- | --- | --- | --- | --- | --- | --- |
| OBSERVATION (1/5) | CATALOG.PUB | 25 / 3 | 20: 26; 15: 2; 10 | 28/28 | 0/28 | 0/28 | 0x1FD, 0x195 |
| OBSERVATION (1/5) | JAZZY.PUB | 64 / 8 | 15: 72; 51 | 72/72 | 72/72 | 72/72 | 0x118, 0x119 |
| OBSERVATION (1/5) | MODERN.PUB | 66 / 8 | 15: 74; 54 | 74/74 | 74/74 | 74/74 | 0x118, 0x119 |
| OBSERVATION (1/5) | PLAIN.PUB | 80 / 8 | 15: 88; 69 | 88/88 | 88/88 | 87/88 | 0x118, 0x119 |
| OBSERVATION (1/5) | SCHOOL.PUB | 28 / 2 | 20: 30; 6 | 30/30 | 0/30 | 0/30 | 0x17D, 0x160 |

OBSERVATION (2/5 files): the inherited half-root translation fails containment for every linked CATALOG and SCHOOL target; all 58 affected record IDs are listed below. This is a geometry-assumption failure, not a failed byte-range/record link. The other three files pass translated containment for 234/234 targets (3/5). Raw-origin containment also passes 72/72,74/74 and87/88 there, so containment still does not establish the true origin.

| Basis / coverage | File | Every half-root containment failure |
| --- | --- | --- |
| OBSERVATION (1/5) | CATALOG.PUB | 0x274, 0x152, 0x12D, 0x14D, 0x140, 0x13A, 0x508, 0x150, 0x19A, 0x141, 0x139, 0x130, 0x120, 0x19E, 0x178, 0x170, 0x1A7, 0x19D, 0x13F, 0x115, 0x16B, 0x165, 0x179, 0x17C, 0x187, 0x116, 0x11C, 0x13B |
| OBSERVATION (1/5) | SCHOOL.PUB | 0x1DE, 0x16E, 0x19D, 0x1A5, 0x198, 0x19A, 0x1A6, 0x1A8, 0x204, 0x1C0, 0x1DF, 0x207, 0x172, 0x1B8, 0x171, 0x13E, 0x191, 0x158, 0x1CA, 0x1C2, 0x203, 0x197, 0x1CB, 0x1FA, 0x1E2, 0x130, 0x16F, 0x1CE, 0x34D, 0x20B |

OBSERVATION (5/5): other-index, fixed-prefix-bounded rectangle probes supply a separate geometry false-positive control. Positive-size and translated-containment hits are reported rather than treated as new frames; three large-root files admit many such numerical hits. Geometry plausibility alone is therefore much weaker than the central map's count/partition/ID/marker corroboration.

| Basis / coverage | File | Other-index eligible probes | Positive size | Half-root containment |
| --- | --- | ---: | ---: | ---: |
| OBSERVATION (1/5) | CATALOG.PUB | 44 | 16 | 1 |
| OBSERVATION (1/5) | JAZZY.PUB | 442 | 415 | 415 |
| OBSERVATION (1/5) | MODERN.PUB | 830 | 771 | 771 |
| OBSERVATION (1/5) | PLAIN.PUB | 588 | 562 | 561 |
| OBSERVATION (1/5) | SCHOOL.PUB | 57 | 35 | 1 |

INFERENCE (5/5): a byte span can now be joined to a candidate text-host rectangle through its direct record ID, in contrast to A's Quill story-ID join. The encoded byte-range association is established; interpretation as actual displayed frame geometry and pages is not. UNKNOWN (5/5): collection/page semantics, coordinate origin/unit/ancestor transforms, range order versus visual reading order, uninterpreted metadata fields beyond the target-ID/kind observations and control-marker rendering. The ten unlinked zero records are not called hidden text, and mapped form feeds are not counted as pages.

INFERENCE (single decisive observation): one independently documented creation observation tying a known CATALOG span/target to an actual page and physical unrotated frame rectangle would test whether these numeric probes describe that visible frame, and distinguish physical scale/origin assumptions. Such an observation is unavailable; a successful byte partition or a small directory-ID hit cannot replace it.

INFERENCE (next smallest existing-data step, 5/5 available files): test whether the linked rectangles use an ancestor-relative coordinate base, beginning with CATALOG's index-20/index-15 parent chains and the 58 CATALOG/SCHOOL containment failures. Freeze the text-range map, test candidate parent transforms with shifted/other-class controls, and retain physical placement as UNKNOWN without an independent measurement. Do not decode unrelated metadata or invent a page assignment to make the boxes fit.

OBSERVATION (first AP21 checkpoint): five source records give 292/292 ordered, contiguous core spans, 292/292 directory links and 292/292 positive co-located rectangle probes. No positive range/map record fails the frozen rule; the two-byte trailers, ten unlinked zero controls and 58 failed placement assumptions are accounted for explicitly. No private text is quoted and no new evidence source is added, so SOURCES.md remains unchanged. Research stops here.
