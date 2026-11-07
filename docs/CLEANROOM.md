<!-- Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0 -->
# Clean-room build (task 3.3)

This repository was created on 2026-10-18 from designcraft-pub-development at commit 6a979401aa23b0d3325e590057385b3c36c66a18, copied without git history. The original crates/pub/src/contents.rs, quill.rs and read.rs were removed and must be written from scratch.

The builder may use only: docs/pub-format/ in this repository, Microsoft's public specifications (MS-CFB, MS-ODRAW), the sample file crates/pub/tests/data/hello-nifty.pub, and the pubsmith fixture .pub files (fixture files only, never pubsmith's source code). The builder must never open designcraft-pub-development, Nifty-Publisher-Lite, libmspub, or any other .pub reader.

truth/*.json files may be used only to score imports. Never use them to infer how the format is encoded. Record any use in SOURCES.md.

Done when: crates/pub builds; cargo fmt --check, cargo clippy -p designcraft-pub -- -D warnings and cargo test -p designcraft-pub pass; no unsafe and no unwrap/expect/panic outside tests. Record every source used in docs/pub-format/SOURCES.md under "Builder".

Files found inside commercial software images are data files only. They are never published, and nothing about how another program reads them is used.
