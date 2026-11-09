// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! File > Open for Publisher 97, 98 and 2000: the experimental text-recovery route.
//! Files are written byte by byte by `designcraft_pub_legacy::fixtures`; nothing is Publisher-made.
#![cfg(not(target_arch = "wasm32"))]

use designcraft_engine::Session;
use designcraft_engine::cmd::base64_encode;
use designcraft_pub_legacy::fixtures::{
    legacy_a_empty_file, legacy_a_file, legacy_a_file_without_text_store, legacy_b_file, legacy_c_all_unknown_file, legacy_c_file,
    legacy_c_mixed_file,
};
use serde_json::json;

fn open(session: &mut Session, name: &str, bytes: &[u8]) -> Result<serde_json::Value, String> {
    session.execute("file.openBytes", &json!({"name": name, "base64": base64_encode(bytes)})).map_err(|e| e.to_string())
}

fn frame_texts(session: &Session) -> Vec<String> {
    let doc = &session.doc().unwrap().doc;
    let mut v = Vec::new();
    for sp in &doc.spreads {
        for item in &sp.items {
            if let Some(tf) = item.text_frame() {
                v.push(doc.story(tf.story).unwrap().text.clone());
            }
        }
    }
    v
}

#[test]
fn a_legacy_file_opens_as_labelled_recovered_text() {
    let mut s = Session::new();
    let r = open(&mut s, "older.pub", &legacy_a_file()).unwrap();
    assert_eq!(r["legacy"]["route"], "experimental-legacy");
    assert_eq!(r["legacy"]["family"], "experimental legacy A");
    assert_eq!(r["legacy"]["scope"], "Complete");
    assert_eq!(
        (r["legacy"]["segmentCount"].as_u64(), r["legacy"]["decodedCount"].as_u64(), r["legacy"]["undecodedCount"].as_u64()),
        (Some(2), Some(2), Some(0))
    );
    assert_eq!(r["legacy"]["rawSizePair"], json!([7_560_000, 10_692_000]));
    assert_eq!(r["legacy"]["sizeStatus"], "inferred-from-raw");
    assert_eq!(r["legacy"]["storageOrderOnly"], true);
    assert!(r["notice"].as_str().unwrap().starts_with("Experimental text recovery."), "{r}");
    assert!(r["notice"].as_str().unwrap().contains("not the original layout"));
    assert_eq!(frame_texts(&s), ["A", "B"]);
    // The copy is unsaved and would save as .designcraft, never over the .pub.
    assert_eq!(s.doc().unwrap().path, None);
    // The page is the inferred 210 x 297 mm.
    let page = &s.doc().unwrap().doc.spreads[0].pages[0];
    let (w, h) = (page.width, page.height);
    assert!((w - 595.276).abs() < 0.1 && (h - 841.89).abs() < 0.1, "{w} x {h}");
}

#[test]
fn a_c_family_file_opens_with_its_ascii_segments() {
    let mut s = Session::new();
    let r = open(&mut s, "older-c.pub", &legacy_c_file()).unwrap();
    assert_eq!(r["legacy"]["family"], "experimental legacy C");
    assert_eq!(r["legacy"]["segmentCount"], 2);
    assert_eq!(frame_texts(&s), ["Alpha", "Beta"]);
}

#[test]
fn a_refused_legacy_file_keeps_the_refusal_and_adds_the_reason() {
    let mut s = Session::new();
    let e = open(&mut s, "older.pub", &legacy_a_file_without_text_store()).unwrap_err();
    assert!(e.contains("Publisher 97/98/2000 files are not supported yet"), "{e}");
    assert!(e.contains("Experimental text recovery could not open this file. Reason:"), "{e}");
    assert!(e.contains("no Quill text stream"), "{e}");
    assert!(s.doc().is_err(), "no blank document replaces a refused open");
    // A damaged container is refused with the original wording alone.
    let mut cut = legacy_a_file();
    cut.truncate(300);
    assert!(open(&mut s, "cut.pub", &cut).is_err());
}

#[test]
fn a_current_publisher_file_never_takes_the_legacy_route() {
    let bytes = include_bytes!("../../pub/tests/data/hello.pub");
    let mut s = Session::new();
    let r = open(&mut s, "flyer.pub", bytes).unwrap();
    assert!(r.get("legacy").is_none());
    assert_eq!(r["notice"], designcraft_pub::CONVERSION_NOTICE);
}

const PLACEHOLDER: &str = "[Text could not be read. No replacement text was guessed.]";

#[test]
fn a_b_family_file_opens_through_the_same_route() {
    let mut s = Session::new();
    let r = open(&mut s, "b.pub", &legacy_b_file()).unwrap();
    assert_eq!(r["legacy"]["family"], "experimental legacy B");
    assert_eq!(r["legacy"]["segmentCount"], 2);
    assert_eq!(frame_texts(&s), ["A", "B"]);
}

#[test]
fn a_known_empty_store_opens_one_empty_workspace_page_with_its_own_notice() {
    let mut s = Session::new();
    let r = open(&mut s, "empty.pub", &legacy_a_empty_file()).unwrap();
    assert_eq!(r["legacy"]["scope"], "EmptyKnownStore");
    assert_eq!(r["legacy"]["openResult"], "empty-recovery-workspace");
    assert_eq!(r["legacy"]["segmentCount"], 0);
    assert!(r["notice"].as_str().unwrap().starts_with("Experimental text recovery found no text"));
    assert!(frame_texts(&s).is_empty());
    assert_eq!(s.doc().unwrap().doc.spreads.len(), 1);
}

#[test]
fn undecodable_c_text_is_marked_never_guessed() {
    let mut s = Session::new();
    let r = open(&mut s, "mixed.pub", &legacy_c_mixed_file()).unwrap();
    assert_eq!(r["legacy"]["scope"], "Partial");
    let l = &r["legacy"];
    assert_eq!(
        (l["segmentCount"].as_u64(), l["decodedCount"].as_u64(), l["undecodedCount"].as_u64(), l["encodingUnknownCount"].as_u64()),
        (Some(2), Some(1), Some(1), Some(1))
    );
    assert_eq!(frame_texts(&s), [PLACEHOLDER, "Beta"]);
    assert!(r["notice"].as_str().unwrap().starts_with("Experimental text recovery. Some text could not be recovered."));
    // Every segment unknown: placeholders, still Partial, never the empty-store state or a refusal.
    let mut s = Session::new();
    let r = open(&mut s, "unknown.pub", &legacy_c_all_unknown_file()).unwrap();
    let l = &r["legacy"];
    assert_eq!(l["scope"], "Partial");
    assert_eq!((l["segmentCount"].as_u64(), l["decodedCount"].as_u64(), l["undecodedCount"].as_u64()), (Some(2), Some(0), Some(2)));
    assert_eq!(frame_texts(&s), [PLACEHOLDER, PLACEHOLDER]);
}

#[test]
fn a_refused_open_leaves_the_open_document_alone() {
    let mut s = Session::new();
    s.execute("file.new", &json!({})).unwrap();
    s.execute("frame.create", &json!({"rect": [10, 10, 200, 60], "content": "text", "text": "keep me"})).unwrap();
    let before = serde_json::to_value(&*s.doc().unwrap().doc).unwrap();
    let e = open(&mut s, "older.pub", &legacy_a_file_without_text_store()).unwrap_err();
    assert!(e.contains("Reason:"), "{e}");
    assert_eq!(serde_json::to_value(&*s.doc().unwrap().doc).unwrap(), before);
}

#[test]
fn report_fields_follow_the_shared_schema() {
    let mut s = Session::new();
    let r = open(&mut s, "older.pub", &legacy_a_file()).unwrap();
    let l = &r["legacy"];
    assert_eq!(l["withheldCount"], "unknown");
    assert!(l["readerRevision"].as_str().unwrap().starts_with("designcraft-pub-legacy "));
    assert!(l["recoveryPageSize"]["widthMm"].as_f64().unwrap() > 200.0);
    assert_eq!(l["segmentCount"].as_u64().unwrap(), l["decodedCount"].as_u64().unwrap() + l["undecodedCount"].as_u64().unwrap());
}

#[test]
fn a_tiny_page_keeps_its_frames_inside_the_margins() {
    // A raw 72 x 72 pt page is 914400 raw units square.
    let mut s = Session::new();
    let r = open(&mut s, "tiny.pub", &designcraft_pub_legacy::fixtures::legacy_a_file_with_size(914_400, 914_400)).unwrap();
    assert_eq!(r["legacy"]["sizeStatus"], "inferred-from-raw");
    let doc = &s.doc().unwrap().doc;
    let page = &doc.spreads[0].pages[0];
    for item in &doc.spreads[0].items {
        let b = item.bounds();
        assert!(b.x0 > 0.0 && b.y0 > 0.0 && b.x1 < page.width && b.y1 < page.height, "{b:?} on {} x {}", page.width, page.height);
    }
}
