// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
use super::*;

#[test]
#[ignore = "needs PUB_REAL_CORPUS with the pinned Apache sample files"]
fn real_scratch() {
    let dir = std::env::var("PUB_REAL_CORPUS").unwrap();
    for name in ["Sample98.pub", "Sample2000.pub"] {
        let bytes = std::fs::read(std::path::Path::new(&dir).join(name)).unwrap();
        match open(&bytes) {
            Ok(c) => eprintln!("{name}: clsid {} contents {} quill {:?}", c.clsid, c.contents.len(), c.quill.as_ref().map(Vec::len)),
            Err(e) => eprintln!("{name}: open error {e}"),
        }
        match read_legacy(&bytes) {
            Ok(r) => {
                eprintln!("{name}: {:?} {:?} segs {} size {:?}", r.family, r.scope, r.segments.len(), r.size);
                for s in &r.segments {
                    eprintln!("   {:?} len {}", s.source, s.text.as_ref().map_or(0, |t| t.len()));
                }
                for w in &r.warnings {
                    eprintln!("   W {w}");
                }
            }
            Err(e) => eprintln!("{name}: ERR {e}"),
        }
    }
}
