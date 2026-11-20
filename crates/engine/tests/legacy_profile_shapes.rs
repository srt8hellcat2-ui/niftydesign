// OBSERVATION: invented profile-shape tests; no tester file or content is used.
// INFERENCE: apply to the implementation checkout with its existing fixture dependencies.
// INFERENCE: ignored cases define the pending reason/probe/admission contract in legacy-followup-5.md.
#![cfg(not(target_arch = "wasm32"))]

use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::Path;
use std::process::Command;

use designcraft_engine::{Session, cmd::base64_encode};
use designcraft_pub_legacy::fixtures::{legacy_a_file_without_text_store, legacy_b_file};
use designcraft_pub_legacy::{Family, MAX_BYTES, Scope, read_legacy};
use serde_json::{Value, json};

fn set16(bytes: &mut [u8], at: usize, value: u16) {
    bytes[at..at + 2].copy_from_slice(&value.to_le_bytes());
}

fn set32(bytes: &mut [u8], at: usize, value: u32) {
    bytes[at..at + 4].copy_from_slice(&value.to_le_bytes());
}

fn get32(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap())
}

fn change_stream(bytes: Vec<u8>, path: &str, change: impl FnOnce(&mut Vec<u8>)) -> Vec<u8> {
    let mut cf = cfb::CompoundFile::open(Cursor::new(bytes)).unwrap();
    let mut body = Vec::new();
    cf.open_stream(path).unwrap().read_to_end(&mut body).unwrap();
    change(&mut body);
    cf.remove_stream(path).unwrap();
    cf.create_stream(path).unwrap().write_all(&body).unwrap();
    cf.flush().unwrap();
    cf.into_inner().into_inner()
}

fn with_opaque_streams(bytes: Vec<u8>, count: usize) -> Vec<u8> {
    let mut cf = cfb::CompoundFile::open(Cursor::new(bytes)).unwrap();
    for i in 0..count {
        cf.create_stream(format!("/Invented{i}")).unwrap().write_all(&[0; 8]).unwrap();
    }
    cf.flush().unwrap();
    cf.into_inner().into_inner()
}

fn generation_200_shape() -> Vec<u8> {
    // INFERENCE: only a recognition fixture, not a claimed valid generation-200 record layout.
    let bytes = change_stream(legacy_a_file_without_text_store(), "/Contents", |body| {
        *body = vec![0; 4096];
        body[..4].copy_from_slice(&[0xe8, 0xac, 0x22, 0x00]);
        set16(body, 4, 200);
        set32(body, 8, 4096);
        set32(body, 0x12, 3072);
        set32(body, 0x16, 256);
        set32(body, 0x1a, 0x34);
        set16(body, 0x22, 20552);
        set16(body, 3072, 0xf0de);
        set16(body, 3074, 46);
        set32(body, 3072 + 0x0a, 256);
    });
    with_opaque_streams(bytes, 2)
}

fn table_58_shape() -> Vec<u8> {
    // INFERENCE: an unknown cardinality must be refused before assuming how many entries it has.
    let bytes = change_stream(legacy_a_file_without_text_store(), "/Contents", |body| {
        set16(body, 0x22, 58);
    });
    with_opaque_streams(bytes, 3)
}

fn alternate_root_b() -> Vec<u8> {
    let mut cf = cfb::CompoundFile::open(Cursor::new(legacy_b_file())).unwrap();
    cf.set_storage_clsid("/", "00021200-0000-0000-00c0-000000000046".parse().unwrap()).unwrap();
    cf.flush().unwrap();
    cf.into_inner().into_inner()
}

fn assert_no_content_fields(value: &Value) {
    match value {
        Value::Object(fields) => {
            for (key, item) in fields {
                let name = key.to_ascii_lowercase().replace('_', "");
                assert!(
                    !matches!(
                        name.as_str(),
                        "text" | "sourcetext" | "path" | "filename" | "name" | "title"
                            | "streamnames" | "rawbytes" | "hexdump"
                    ),
                    "structure-only output contains a forbidden field: {key}"
                );
                assert_no_content_fields(item);
            }
        }
        Value::Array(items) => {
            for item in items {
                assert_no_content_fields(item);
            }
        }
        _ => {}
    }
}

fn probe(path: &Path) -> Value {
    let mut session = Session::new();
    let value = session.execute("file.probeLegacy", &json!({"path": path})).expect(
        "IMPLEMENTATION PENDING: register file.probeLegacy; unknown-command is a setup block, not a profile result",
    );
    assert!(session.doc().is_err(), "a structural probe must not create a document");
    assert_eq!(value["mode"], "structure-only");
    assert_eq!(value["textDecoded"], false);
    assert_no_content_fields(&value);
    value
}

fn probe_bytes(bytes: &[u8]) -> Value {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("synthetic");
    std::fs::write(&path, bytes).unwrap();
    probe(&path)
}

fn assert_refused(value: &Value, code: &str) {
    assert_eq!(value["scope"], "Refused");
    assert_eq!(value["reasonCode"], code);
    assert_eq!(value["family"], "legacy-family-unknown");
}

#[test]
#[ignore = "IMPLEMENTATION PENDING: generation-200 recognition, stable codes and file.probeLegacy"]
fn generation_200_is_recognised_and_refused_before_text() {
    let value = probe_bytes(&generation_200_shape());
    assert_refused(&value, "header-gen-200");
    assert_eq!(value["classification"], "recognised-refused");
    assert_eq!(value["headerWord02"], 34);
    assert_eq!(value["headerWord04"], 200);
    assert_eq!(value["contentsLength"], 4096);
    assert_eq!(value["declaredContentsLength"], 4096);
    assert_eq!(value["controlOffset"], 3072);
    assert_eq!(value["directoryOffset"], 256);
    assert_eq!(value["tableOffset"], 0x34);
    assert_eq!(value["word22"], 20552);
    assert_eq!(value["controlMarker"], 0xf0de);
    assert_eq!(value["streamCount"], 3);
    assert_eq!(value["quillPresent"], false);
}

#[test]
#[ignore = "IMPLEMENTATION PENDING: stable profile-unknown code and file.probeLegacy"]
fn table_58_stays_unknown_and_is_not_an_empty_store() {
    let value = probe_bytes(&table_58_shape());
    assert_refused(&value, "profile-unknown");
    assert_eq!(value["classification"], "recognised-refused");
    assert_eq!(value["word22"], 58);
    assert_eq!(value["word24"], 6);
    assert_eq!(value["streamCount"], 4);
    assert_eq!(value["quillPresent"], false);
}

#[test]
#[ignore = "IMPLEMENTATION PENDING: stable not-compound code and file.probeLegacy"]
fn non_compound_has_its_own_reason_without_a_version_guess() {
    let value = probe_bytes(b"invented non-container input");
    assert_refused(&value, "not-compound");
    assert_eq!(value["classification"], "not-compound");
    assert_eq!(value["isCompound"], false);
    assert!(value["headerWord04"].is_null());
    assert!(value.get("savingRelease").is_none(), "bytes alone do not establish a release");
}

#[test]
#[ignore = "IMPLEMENTATION PENDING: the explicitly allowed alternate root identity must pass the complete B path"]
fn alternate_root_21200_word_94_opens_as_b_after_all_gates() {
    let bytes = alternate_root_b();
    let result = read_legacy(&bytes).expect("the complete invented B profile should pass under the allowed root identity");
    assert_eq!(result.family, Family::B);
    assert_eq!(result.scope, Scope::Complete);
    assert_eq!(result.segments.len(), 2);
    assert_eq!(result.segments.iter().filter(|s| s.text.is_some()).count(), 2);
    let mut session = Session::new();
    let opened = session.execute(
        "file.openBytes",
        &json!({"name": "invented-profile.pub", "base64": base64_encode(&bytes)}),
    ).unwrap();
    assert_eq!(opened["legacy"]["family"], "experimental legacy B");
    assert_eq!(opened["legacy"]["scope"], "Complete");
    assert_eq!(opened["legacy"]["segmentCount"], 2);
}

#[test]
#[ignore = "IMPLEMENTATION PENDING: alternate-root admission must not bypass existing G02 or AB04 checks"]
fn alternate_root_b_keeps_header_and_quill_count_controls() {
    let bad_header = change_stream(alternate_root_b(), "/Contents", |body| {
        let length = u32::try_from(body.len()).unwrap();
        set32(body, 8, length + 1);
    });
    assert!(read_legacy(&bad_header).is_err(), "G02 length equality must still reject");

    let bad_catalog = change_stream(alternate_root_b(), "/Quill/QuillSub/CONTENTS", |body| {
        let count = u16::from_le_bytes(body[0x1a..0x1c].try_into().unwrap()) as usize;
        let mut found = false;
        for k in 0..count {
            let entry = 0x20 + 24 * k;
            if body.get(entry + 2..entry + 6) == Some(b"STRS") {
                let offset = get32(body, entry + 16) as usize;
                let count = get32(body, offset);
                set32(body, offset, count + 1);
                found = true;
                break;
            }
        }
        assert!(found, "invented STRS section not found");
    });
    let result = read_legacy(&bad_catalog).expect("safe admitted structure should retain its text-path refusal");
    assert_eq!(result.family, Family::B);
    assert_eq!(result.scope, Scope::TextRefused);
    assert!(result.segments.is_empty(), "AB04 must still refuse mismatched counts");
}

#[test]
#[ignore = "IMPLEMENTATION PENDING: metadata-first size guard and file.probeLegacy; no whole-file test allocation"]
fn over_limit_probe_refuses_from_metadata_without_reading_input() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("synthetic");
    let limit = MAX_BYTES;
    let length = limit.checked_add(1).unwrap();
    let mut file = File::create(&path).unwrap();
    let prefix = legacy_b_file();
    assert!((prefix.len() as u64) < limit);
    file.write_all(&prefix).unwrap();
    file.set_len(length).unwrap();
    file.sync_all().unwrap();
    drop(file);
    assert_eq!(std::fs::metadata(&path).unwrap().len(), length);
    let value = probe(&path);
    assert_refused(&value, "over-size-limit");
    assert_eq!(value["inputBytes"], length);
    assert_eq!(value["limitBytes"], limit);
    assert_eq!(value["bytesRead"], 0);
    assert!(value["contentsLength"].is_null(), "no container/text parsing is allowed before the size guard");
}

fn csv_rows(text: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' if quoted && chars.peek() == Some(&'"') => {
                field.push('"');
                let _ = chars.next();
            }
            '"' => quoted = !quoted,
            ',' if !quoted => row.push(std::mem::take(&mut field)),
            '\n' if !quoted => {
                row.push(std::mem::take(&mut field));
                rows.push(std::mem::take(&mut row));
            }
            '\r' if !quoted && chars.peek() == Some(&'\n') => {}
            _ => field.push(c),
        }
    }
    assert!(!quoted);
    if !field.is_empty() || !row.is_empty() {
        row.push(field);
        rows.push(row);
    }
    rows
}

#[test]
#[ignore = "IMPLEMENTATION PENDING: typed reasonCode in both reports; set LEGACY_FOLLOWUP_CLI to the matching built CLI"]
fn stable_reason_codes_survive_batch_and_folder_reports() {
    let cli = std::env::var_os("LEGACY_FOLLOWUP_CLI").expect("set the matching built CLI");
    for (bytes, expected) in [
        (b"invented non-container input".to_vec(), "not-compound"),
        (table_58_shape(), "profile-unknown"),
    ] {
        let temp = tempfile::tempdir().unwrap();
        let input = temp.path().join("input");
        std::fs::create_dir(&input).unwrap();
        std::fs::write(input.join("synthetic.pub"), bytes).unwrap();

        let batch = temp.path().join("batch");
        let output = Command::new(&cli).arg("batch-convert").arg(&input).arg("--out").arg(&batch).output().unwrap();
        assert!(!output.status.success(), "unsupported input must not become a successful blank conversion");
        let rows = csv_rows(&std::fs::read_to_string(batch.join("report.csv")).unwrap());
        assert_eq!(rows.len(), 2);
        let code = rows[0].iter().position(|s| s == "reasonCode").expect("missing stable reasonCode column");
        assert_eq!(rows[1][code], expected);

        let folder = temp.path().join("folder");
        let output = Command::new(&cli).arg("convert-folder").arg(&input).arg(&folder).output().unwrap();
        assert!(!output.status.success());
        let report: Value = serde_json::from_slice(&std::fs::read(folder.join("report.json")).unwrap()).unwrap();
        assert_eq!(report["files"].as_array().unwrap().len(), 1);
        assert_eq!(report["files"][0]["reasonCode"], expected);
    }
}
