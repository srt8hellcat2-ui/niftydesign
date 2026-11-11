// OBSERVATION: byte-built integration checks for the implementation at b1e4dbec.
// INFERENCE: apply this test-only file to that implementation; the docs branch has no reader dependency.
#![cfg(not(target_arch = "wasm32"))]

use std::io::{Cursor, Write};
use std::process::{Command, Output, Stdio};

use designcraft_engine::{Session, cmd::base64_encode};
use designcraft_pub_legacy::fixtures::{legacy_a_file, legacy_c_all_unknown_file, legacy_c_mixed_file};
use serde_json::{Value, json};

const PLACEHOLDER: &str = "[Text could not be read. No replacement text was guessed.]";
const CONTROLS: &str = "A\tB\u{000c}C\u{0001}D\r\r";
const FIELDS: &[&str] = &[
    "readerRevision", "route", "family", "scope", "openResult", "segmentCount", "decodedCount", "encodingUnknownCount",
    "undecodedCount", "rawSizePair", "sizeStatus", "recoveryPageSize", "withheldCount", "provenance", "storageOrderOnly",
];

fn open(bytes: &[u8]) -> (Session, Value) {
    let mut session = Session::new();
    let result = session.execute("file.openBytes", &json!({"name": "invented.pub", "base64": base64_encode(bytes)})).unwrap();
    (session, result)
}

fn texts(session: &Session) -> Vec<String> {
    let doc = &session.doc().unwrap().doc;
    doc.spreads.iter().flat_map(|s| &s.items).filter_map(|item| item.text_frame())
        .map(|frame| doc.story(frame.story).unwrap().text.clone()).collect()
}

fn counts(result: &Value, decoded: u64, unknown: u64) {
    let legacy = &result["legacy"];
    assert_eq!(legacy["segmentCount"], decoded + unknown);
    assert_eq!(legacy["decodedCount"], decoded);
    assert_eq!(legacy["undecodedCount"], unknown);
    assert_eq!(legacy["encodingUnknownCount"], unknown);
    assert_eq!(legacy["scope"], "Partial");
    assert_eq!(legacy["openResult"], "recovery-document");
    assert!(result["notice"].as_str().unwrap().contains("Some text could not be recovered"));
    assert!(!result["notice"].as_str().unwrap().contains("found no text"));
}

#[test]
fn all_unknown_c_retains_every_placeholder_and_is_not_empty() {
    let (session, result) = open(&legacy_c_all_unknown_file());
    counts(&result, 0, 2);
    assert_eq!(texts(&session), [PLACEHOLDER, PLACEHOLDER]);
    assert_eq!(result["legacy"]["provenance"].as_array().unwrap().len(), 2);
}

#[test]
fn mixed_c_retains_one_decoded_frame_and_one_placeholder() {
    let (session, result) = open(&legacy_c_mixed_file());
    counts(&result, 1, 1);
    assert_eq!(texts(&session), [PLACEHOLDER, "Beta"]);
    assert_eq!(result["legacy"]["provenance"][0]["decoded"], false);
    assert_eq!(result["legacy"]["provenance"][1]["decoded"], true);
}

fn put16(bytes: &mut [u8], at: usize, value: u16) {
    bytes[at..at + 2].copy_from_slice(&value.to_le_bytes());
}

fn put32(bytes: &mut [u8], at: usize, value: u32) {
    bytes[at..at + 4].copy_from_slice(&value.to_le_bytes());
}

// INFERENCE: build the contract's bounded three-section Quill store using invented text only.
fn a_with_text(first: &str) -> Vec<u8> {
    let second = "E\r";
    let mut syid = vec![0; 16];
    put32(&mut syid, 4, 2);
    put32(&mut syid, 8, 5);
    put32(&mut syid, 12, 9);
    let mut strs = vec![0; 20];
    put32(&mut strs, 0, 2);
    put32(&mut strs, 4, 8);
    put32(&mut strs, 12, first.encode_utf16().count() as u32);
    put32(&mut strs, 16, second.encode_utf16().count() as u32);
    let text: Vec<u8> = first.encode_utf16().chain(second.encode_utf16()).flat_map(u16::to_le_bytes).collect();
    let mut quill = vec![0; 512];
    quill[..8].copy_from_slice(b"CHNKINK ");
    put16(&mut quill, 0x1a, 3);
    put32(&mut quill, 0x1c, u32::MAX);
    for (i, (tag, body)) in [(b"SYID", syid), (b"TEXT", text), (b"STRS", strs)].into_iter().enumerate() {
        let entry = 0x20 + 24 * i;
        quill[entry + 2..entry + 6].copy_from_slice(tag);
        let offset = quill.len() as u32;
        put32(&mut quill, entry + 16, offset);
        put32(&mut quill, entry + 20, body.len() as u32);
        quill.extend(body);
    }
    let mut container = cfb::CompoundFile::open(Cursor::new(legacy_a_file())).unwrap();
    container.remove_stream("/Quill/QuillSub/CONTENTS").unwrap();
    container.create_stream("/Quill/QuillSub/CONTENTS").unwrap().write_all(&quill).unwrap();
    container.flush().unwrap();
    container.into_inner().into_inner()
}

#[test]
fn controls_are_kept_by_reader_but_only_tab_is_shown_by_adapter() {
    let bytes = a_with_text(CONTROLS);
    let raw = designcraft_pub_legacy::read_legacy(&bytes).unwrap();
    assert_eq!(raw.segments[0].text.as_deref(), Some(CONTROLS));
    let (session, result) = open(&bytes);
    assert_eq!(texts(&session), ["A\tBCD", "E"]);
    assert!(result["warnings"].as_array().unwrap().iter().any(|w| w.as_str().unwrap().contains("stored control characters")));
    assert!(result["legacy"]["provenance"][0].get("units").is_some());
}

#[test]
#[ignore = "EXPECTED FAILURE b1e4dbec: document/report preserve ranges but not the original control-bearing text"]
fn original_control_sequence_is_retained_after_open() {
    let (session, result) = open(&a_with_text(CONTROLS));
    let document = serde_json::to_value(&*session.doc().unwrap().doc).unwrap();
    fn contains_text(value: &Value) -> bool {
        match value {
            Value::String(s) => s.contains(CONTROLS),
            Value::Array(items) => items.iter().any(contains_text),
            Value::Object(items) => items.values().any(contains_text),
            _ => false,
        }
    }
    assert!(contains_text(&document) || contains_text(&result), "original controls/trailing paragraphs were lost after open");
}

#[test]
#[ignore = "EXPECTED FAILURE b1e4dbec: trailing paragraph marks alone do not increment trimmed_segments"]
fn removed_trailing_paragraphs_alone_are_reported() {
    let (session, result) = open(&a_with_text("A\r\r"));
    assert_eq!(texts(&session)[0], "A");
    assert!(result["warnings"].as_array().unwrap().iter().any(|w| w.as_str().unwrap().contains("trailing paragraph marks")));
}

// INFERENCE: these tests invoke a separately built CLI supplied explicitly, never a private file.
fn cli() -> Command {
    Command::new(std::env::var_os("LEGACY_FOLLOWUP_CLI").expect("set LEGACY_FOLLOWUP_CLI to the pinned built CLI"))
}

fn command_output(mut command: Command) -> Output {
    let output = command.output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    output
}

fn run_result(path: &std::path::Path) -> Value {
    let mut child = cli().args(["script", "-"]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    let step = json!({"command": "file.open", "params": {"path": path}});
    writeln!(child.stdin.take().unwrap(), "{step}").unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["completed"], 1);
    report["results"][0].clone()
}

fn assert_legacy_fields(result: &Value) {
    let legacy = result.get("legacy").expect("missing typed legacy object");
    for field in FIELDS {
        assert!(legacy.get(*field).is_some(), "missing legacy.{field}");
    }
    assert_eq!(legacy["recoveryPageSize"]["unit"], "pt");
    assert!(legacy["recoveryPageSize"]["widthMm"].is_number());
    assert!(legacy["recoveryPageSize"]["heightMm"].is_number());
    assert_eq!(legacy["withheldCount"], "unknown");
    assert_eq!(legacy["storageOrderOnly"], true);
    counts(result, 1, 1);
}

#[test]
#[ignore = "requires LEGACY_FOLLOWUP_CLI; compiled CLI command-result check, not a known expected failure"]
fn cli_run_and_script_keep_mixed_c_report_fields() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("invented.pub");
    std::fs::write(&path, legacy_c_mixed_file()).unwrap();
    let mut command = cli();
    command.args(["run", "--cmd", &format!("file.open={}", json!({"path": path}))]);
    let output = command_output(command);
    let result: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_legacy_fields(&result);
    assert_legacy_fields(&run_result(&path));
}

#[test]
#[ignore = "EXPECTED FAILURE b1e4dbec: batch report.csv drops the typed legacy fields; requires LEGACY_FOLLOWUP_CLI"]
fn batch_report_has_parity_with_mixed_c_command_result() {
    let temp = tempfile::tempdir().unwrap();
    let input = temp.path().join("input");
    std::fs::create_dir(&input).unwrap();
    let path = input.join("invented.pub");
    std::fs::write(&path, legacy_c_mixed_file()).unwrap();
    assert_legacy_fields(&run_result(&path));
    let out = temp.path().join("batch");
    let mut command = cli();
    command.arg("batch-convert").arg(&input).arg("--out").arg(&out);
    command_output(command);
    let csv = std::fs::read_to_string(out.join("report.csv")).unwrap();
    let header: Vec<_> = csv.lines().next().unwrap().split(',').collect();
    for existing in ["file", "pages", "objects", "status", "warnings", "pdf_path", "failure_reason", "notes", "notice"] {
        assert!(header.contains(&existing), "missing existing CSV column {existing}");
    }
    let missing: Vec<_> = FIELDS.iter().filter(|f| !header.contains(f)).collect();
    assert!(missing.is_empty(), "EXPECTED FAILURE: batch CSV has no typed legacy columns: {missing:?}");
}

#[test]
#[ignore = "EXPECTED FAILURE b1e4dbec: convert-folder report.json drops the typed legacy object; requires LEGACY_FOLLOWUP_CLI"]
fn folder_report_has_parity_with_mixed_c_command_result() {
    let temp = tempfile::tempdir().unwrap();
    let input = temp.path().join("input");
    std::fs::create_dir(&input).unwrap();
    let path = input.join("invented.pub");
    std::fs::write(&path, legacy_c_mixed_file()).unwrap();
    assert_legacy_fields(&run_result(&path));
    let out = temp.path().join("folder");
    let mut command = cli();
    command.arg("convert-folder").arg(&input).arg(&out);
    command_output(command);
    let report: Value = serde_json::from_slice(&std::fs::read(out.join("report.json")).unwrap()).unwrap();
    let entry = &report["files"][0];
    assert!(entry.get("warnings").is_some(), "existing folder warnings missing");
    assert!(entry.get("legacy").is_some(), "EXPECTED FAILURE: folder file entry omits typed legacy object");
    for field in FIELDS {
        assert!(entry["legacy"].get(*field).is_some(), "missing legacy.{field}");
    }
}

#[test]
fn standard_positive_raw_pair_keeps_its_own_recovery_size() {
    // INFERENCE: an invented Letter-size candidate is inside the proposed plausibility range.
    let bytes = designcraft_pub_legacy::fixtures::legacy_a_file_with_size(7_772_400, 10_058_400);
    let (session, result) = open(&bytes);
    let legacy = &result["legacy"];
    assert_eq!(legacy["scope"], "Complete");
    assert_eq!(legacy["segmentCount"], 2);
    assert_eq!(legacy["decodedCount"], 2);
    assert_eq!(legacy["rawSizePair"], json!([7_772_400, 10_058_400]));
    assert_eq!(legacy["sizeStatus"], "inferred-from-raw");
    assert!((legacy["recoveryPageSize"]["widthMm"].as_f64().unwrap() - 215.9).abs() < 1e-6);
    assert!((legacy["recoveryPageSize"]["heightMm"].as_f64().unwrap() - 279.4).abs() < 1e-6);
    let doc = &session.doc().unwrap().doc;
    let page = &doc.spreads[0].pages[0];
    assert!((page.width - 612.0).abs() < 1e-6);
    assert!((page.height - 792.0).abs() < 1e-6);
    assert!(result["warnings"].as_array().unwrap().iter().any(|w| {
        w.as_str().unwrap() == "Page size is estimated from stored numbers. It has not been verified."
    }));
}

#[test]
#[ignore = "EXPECTED FAILURE b1e4dbec: the proposed greater-than-450-mm plausibility fallback is not implemented"]
fn oversized_square_uses_a4_workspace_and_preserves_raw_pair() {
    // INFERENCE: 18,000,000 / 36,000 = 500 mm; this is invented test metadata, not a private file.
    let bytes = designcraft_pub_legacy::fixtures::legacy_a_file_with_size(18_000_000, 18_000_000);
    let (session, result) = open(&bytes);
    let legacy = &result["legacy"];
    assert_eq!(legacy["scope"], "Complete");
    assert_eq!(legacy["segmentCount"], 2);
    assert_eq!(legacy["rawSizePair"], json!([18_000_000, 18_000_000]));
    assert_eq!(legacy["sizeStatus"], "default-A4");
    assert!((legacy["recoveryPageSize"]["widthMm"].as_f64().unwrap() - 210.0).abs() < 0.01);
    assert!((legacy["recoveryPageSize"]["heightMm"].as_f64().unwrap() - 297.0).abs() < 0.01);
    let doc = &session.doc().unwrap().doc;
    let page = &doc.spreads[0].pages[0];
    assert!((page.width - 595.276).abs() < 0.01);
    assert!((page.height - 841.89).abs() < 0.01);
    assert!(result["warnings"].as_array().unwrap().iter().any(|w| {
        w.as_str().unwrap() == "Original page size is unverified. A standard A4 recovery page is being used."
    }));
}

#[test]
#[ignore = "requires LEGACY_FOLLOWUP_CLI; positive A-family batch-notice check, not an expected failure"]
fn a_family_file_open_and_batch_keep_legacy_notice_and_warning_status() {
    // INFERENCE: parse CSV quoting so commas/newlines in warnings cannot masquerade as the notice column.
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
        assert!(!quoted, "unterminated CSV field");
        if !field.is_empty() || !row.is_empty() {
            row.push(field);
            rows.push(row);
        }
        rows
    }

    let temp = tempfile::tempdir().unwrap();
    let input = temp.path().join("input");
    std::fs::create_dir(&input).unwrap();
    let path = input.join("invented-a.pub");
    std::fs::write(&path, legacy_a_file()).unwrap();
    let mut session = Session::new();
    let opened = session.execute("file.open", &json!({"path": path})).unwrap();
    assert_eq!(opened["legacy"]["family"], "experimental legacy A");
    assert_eq!(opened["legacy"]["scope"], "Complete");
    assert_eq!(texts(&session), ["A", "B"]);
    let notice = opened["notice"].as_str().unwrap();
    assert!(notice.starts_with("Experimental text recovery."));
    assert!(notice.contains("not the original layout"));

    let out = temp.path().join("batch");
    let mut command = cli();
    command.arg("batch-convert").arg(&input).arg("--out").arg(&out);
    let output = command_output(command);
    let summary: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(summary["files"], 1);
    assert_eq!(summary["converted_with_warnings"], 1);
    assert_eq!(summary["failed"], 0);

    let csv = std::fs::read_to_string(out.join("report.csv")).unwrap();
    let rows = csv_rows(&csv);
    assert_eq!(rows.len(), 2, "expected a header and exactly one CSV record");
    assert_eq!(rows[0].len(), rows[1].len());
    let column = |name: &str| rows[0].iter().position(|field| field == name).unwrap();
    assert_eq!(rows[1][column("file")], "invented-a.pub");
    assert_eq!(rows[1][column("status")], "converted with warnings");
    assert!(rows[1][column("notice")].contains(notice));
    assert!(rows[1][column("failure_reason")].is_empty());
}
