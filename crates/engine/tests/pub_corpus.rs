// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! `.pub` import → native save → reopen → edit → save → PDF, over local Publisher files.
//! Genuine Publisher files are not redistributable, so they are never committed: point
//! `PUB_CORPUS` at folders of `.pub` files (`:`-separated); `PUB_PDF_OUT=dir` keeps the PDFs. Run:
//! `PUB_CORPUS=dir1:dir2 cargo test -p designcraft-engine --test pub_corpus -- --ignored --nocapture`.

use std::path::PathBuf;

use designcraft_doc::{Content, Document, Item};

fn corpus() -> Vec<PathBuf> {
    let dirs = std::env::var("PUB_CORPUS").expect("set PUB_CORPUS to folders of .pub files");
    let mut v: Vec<PathBuf> = dirs
        .split(':')
        .filter(|d| !d.is_empty())
        .flat_map(|d| std::fs::read_dir(d).into_iter().flatten().flatten().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|e| e.eq_ignore_ascii_case("pub")))
        .collect();
    v.sort();
    v
}

fn items(d: &Document) -> Vec<&Item> {
    d.spreads.iter().flat_map(|s| s.items.iter().map(|i| &**i)).collect()
}

#[test]
#[ignore = "needs PUB_CORPUS: genuine Publisher files are not redistributable"]
fn corpus_save_reopen_edit_export() {
    let mut checked = 0;
    for f in corpus() {
        let Ok(imp) = designcraft_pub::import_pub(&std::fs::read(&f).unwrap()) else { continue };
        let mut d = imp.document;
        let bytes = designcraft_format::save(&d).unwrap_or_else(|e| panic!("{f:?}: save {e}"));
        let mut back = designcraft_format::load(&bytes).unwrap_or_else(|e| panic!("{f:?}: load {e}"));
        assert_eq!(back.page_count(), d.page_count(), "{f:?}");
        assert_eq!(items(&back).len(), items(&d).len(), "{f:?}");
        // Edit after reopening: append text to the first text frame and move it.
        let frame = items(&back).iter().find(|i| i.is_text_frame()).map(|i| i.id);
        if let Some(id) = frame {
            let sid = match &back.item(id).unwrap().content {
                Content::Text(t) => t.story,
                _ => unreachable!(),
            };
            let st = back.story_mut(sid).unwrap();
            let end = st.text.len();
            st.insert(end, " (edited)");
            back.item_mut(id).unwrap().xf = designcraft_geom::Affine::translate((5.0, 0.0)) * back.item(id).unwrap().xf;
            let again = designcraft_format::load(&designcraft_format::save(&back).unwrap()).unwrap();
            assert!(again.stories.get(&sid).unwrap().text.ends_with(" (edited)"), "{f:?}");
        }
        d = back;
        let pdf = designcraft_pdf::export_pdf(&d, &designcraft_compose::Cache::new(), &designcraft_pdf::PdfOptions::default())
            .unwrap_or_else(|e| panic!("{f:?}: pdf {e}"));
        assert!(pdf.starts_with(b"%PDF"), "{f:?}");
        // PUB_PDF_OUT=dir keeps the PDFs for inspection.
        if let (Ok(dir), Some(name)) = (std::env::var("PUB_PDF_OUT"), f.file_stem()) {
            std::fs::write(PathBuf::from(dir).join(name).with_extension("pdf"), &pdf).unwrap();
        }
        checked += 1;
    }
    println!("save/reopen/edit/PDF checked on {checked} imported files");
    assert!(checked > 0);
}
