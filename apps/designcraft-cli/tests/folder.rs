// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! End-to-end PDF folder conversion, including report and output protection.
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};
const PUB: &[u8] = include_bytes!("../../../crates/pub/tests/data/hello-nifty.pub");
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Dir(PathBuf);
impl Dir {
    fn new() -> Self {
        let parent = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/folder-convert-tests");
        fs::create_dir_all(&parent).unwrap();
        let path = parent.join(format!("{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        fs::create_dir(&path).unwrap();
        fs::create_dir(path.join("input")).unwrap();
        Self(path)
    }
    fn input(&self) -> PathBuf {
        self.0.join("input")
    }
    fn output(&self) -> PathBuf {
        self.0.join("output")
    }
    fn run(&self, flags: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_designcraft-cli")).arg("convert-folder").arg(self.input()).arg(self.output()).args(flags).output().unwrap()
    }
    fn report(&self) -> Value {
        serde_json::from_slice(&fs::read(self.output().join("report.json")).unwrap()).unwrap()
    }
    fn markdown(&self) -> String {
        fs::read_to_string(self.output().join("report.md")).unwrap()
    }
}
impl Drop for Dir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn assert_code(out: &Output, code: i32) {
    assert_eq!(out.status.code(), Some(code), "stdout={} stderr={}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    assert!(!String::from_utf8_lossy(&out.stderr).contains("panicked"));
}
#[test]
fn empty_non_pub_and_invalid_files_always_have_reports() {
    let dir = Dir::new();
    assert_code(&dir.run(&[]), 0);
    assert_eq!(dir.report()["summary"]["files"], 0);
    assert!(dir.markdown().contains("No .pub files were found."));
    fs::write(dir.input().join("readme.txt"), "hello").unwrap();
    assert_code(&dir.run(&["--overwrite"]), 0);
    assert_eq!(dir.report()["summary"]["total_pages"], 0);
    fs::write(dir.input().join("empty.pub"), []).unwrap();
    fs::write(dir.input().join("fake.PUB"), b"not a Publisher file").unwrap();
    assert_code(&dir.run(&["--overwrite"]), 1);
    let value = dir.report();
    assert_eq!(value["summary"]["could_not_read"], 2);
    for row in value["files"].as_array().unwrap() {
        assert_eq!(row["result"], "Could not read");
        assert!(row["pdf"].is_null());
        assert_eq!(row["message"], "This is not a readable Publisher file.");
    }
    assert!(dir.markdown().ends_with("Check every converted file against the original, because no warning does not prove a perfect copy.\n"));
}
#[test]
fn recursion_sanitizes_names_and_keeps_case_and_sanitization_collisions_distinct() {
    let dir = Dir::new();
    for sub in ["one", "two", "three"] {
        fs::create_dir(dir.input().join(sub)).unwrap();
    }
    for name in ["one/same.pub", "two/SAME.PUB", "three/same.pub", "CON.pub"] {
        fs::write(dir.input().join(name), PUB).unwrap();
    }
    assert_code(&dir.run(&[]), 0);
    assert_eq!(dir.report()["summary"]["files"], 1);
    assert!(dir.output().join("file-CON.pdf").exists());
    assert_code(&dir.run(&["--recursive", "--overwrite"]), 0);
    let report = dir.report();
    assert_eq!(report["summary"]["files"], 4);
    assert_eq!(report["summary"]["total_pages"], 4);
    let names: std::collections::BTreeSet<_> =
        report["files"].as_array().unwrap().iter().map(|row| row["pdf"].as_str().unwrap().to_ascii_lowercase()).collect();
    assert_eq!(names.len(), 4);
    for row in report["files"].as_array().unwrap() {
        let bytes = fs::read(dir.output().join(row["pdf"].as_str().unwrap())).unwrap();
        assert_eq!(designcraft_render::pdf_page_count(&bytes), Some(1));
    }
    assert!(!fs::read_dir(dir.output()).unwrap().any(|e| e.unwrap().path().extension().is_some_and(|e| e == "designcraft")));
    #[cfg(unix)]
    {
        fs::write(dir.input().join("a?.pub"), PUB).unwrap();
        fs::write(dir.input().join("a*.pub"), PUB).unwrap();
        fs::write(dir.input().join("[odd]\n雪.pub"), PUB).unwrap();
        assert_code(&dir.run(&["--recursive", "--overwrite"]), 0);
        assert!(dir.output().join("a_.pdf").exists());
        assert!(dir.output().join("a_-2.pdf").exists());
        let md = dir.markdown();
        assert!(md.contains("\\[odd\\] 雪.pub"));
    }
}
#[test]
fn outputs_and_reports_are_never_overwritten_without_the_flag() {
    let dir = Dir::new();
    for name in ["a.pub", "b.pub"] {
        fs::write(dir.input().join(name), PUB).unwrap();
    }
    fs::create_dir(dir.output()).unwrap();
    fs::write(dir.output().join("a.pdf"), b"keep this PDF").unwrap();
    assert_code(&dir.run(&[]), 1);
    assert_eq!(fs::read(dir.output().join("a.pdf")).unwrap(), b"keep this PDF");
    assert_eq!(designcraft_render::pdf_page_count(&fs::read(dir.output().join("b.pdf")).unwrap()), Some(1));
    assert!(dir.markdown().contains("already exists; it was left unchanged"));
    let (md, json) = (dir.markdown(), fs::read(dir.output().join("report.json")).unwrap());
    assert_code(&dir.run(&[]), 1);
    assert_eq!(dir.markdown(), md);
    assert_eq!(fs::read(dir.output().join("report.json")).unwrap(), json);
    assert_code(&dir.run(&["--overwrite"]), 0);
    assert_eq!(designcraft_render::pdf_page_count(&fs::read(dir.output().join("a.pdf")).unwrap()), Some(1));
    assert_eq!(dir.report()["summary"]["could_not_read"], 0);
}
#[test]
fn arguments_help_and_missing_input_are_clear() {
    for flag in ["--help", "-h"] {
        let out = Command::new(env!("CARGO_BIN_EXE_designcraft-cli")).args(["convert-folder", flag]).output().unwrap();
        assert_code(&out, 0);
        assert!(out.stderr.is_empty());
        let help = String::from_utf8_lossy(&out.stdout);
        for word in ["INPUT_DIR OUTPUT_DIR", "--recursive", "--overwrite", "report.md", "report.json", "Exit codes: 0", "2 = bad arguments"] {
            assert!(help.contains(word), "{help}");
        }
    }
    for args in [
        vec!["convert-folder"],
        vec!["convert-folder", "--bad"],
        vec!["convert-folder", "a", "b", "extra"],
        vec!["convert-folder", "a", "b", "--recursive", "--recursive"],
    ] {
        let out = Command::new(env!("CARGO_BIN_EXE_designcraft-cli")).args(args).output().unwrap();
        assert_code(&out, 2);
    }
    let dir = Dir::new();
    fs::remove_dir(dir.input()).unwrap();
    assert_code(&dir.run(&[]), 1);
    assert_eq!(dir.report()["summary"]["could_not_read"], 1);
    assert!(dir.markdown().contains("folder could not be read"));
}
#[cfg(unix)]
#[test]
fn symlinks_and_hardlinks_cannot_redirect_writes_and_non_utf8_names_do_not_panic() {
    use std::os::unix::{
        ffi::OsStringExt,
        fs::{PermissionsExt, symlink},
    };
    let dir = Dir::new();
    fs::write(dir.input().join("a.pub"), PUB).unwrap();
    fs::create_dir(dir.output()).unwrap();
    let outside = dir.0.join("outside.pdf");
    fs::write(&outside, b"private existing file").unwrap();
    symlink(&outside, dir.output().join("a.pdf")).unwrap();
    assert_code(&dir.run(&["--overwrite"]), 1);
    assert_eq!(fs::read(&outside).unwrap(), b"private existing file");
    fs::remove_file(dir.output().join("a.pdf")).unwrap();
    fs::hard_link(&outside, dir.output().join("a.pdf")).unwrap();
    assert_code(&dir.run(&["--overwrite"]), 0);
    assert_eq!(fs::read(&outside).unwrap(), b"private existing file");
    fs::remove_file(dir.output().join("report.json")).unwrap();
    symlink(&outside, dir.output().join("report.json")).unwrap();
    assert_code(&dir.run(&["--overwrite"]), 1);
    assert_eq!(fs::read(&outside).unwrap(), b"private existing file");
    fs::remove_file(dir.output().join("report.json")).unwrap();
    fs::write(dir.input().join(std::ffi::OsString::from_vec(b"odd-\xff.pub".to_vec())), PUB).unwrap();
    symlink(&outside, dir.input().join("link.pub")).unwrap();
    symlink(dir.input(), dir.input().join("loop")).unwrap();
    assert_code(&dir.run(&["--overwrite", "--recursive"]), 1);
    assert_eq!(dir.report()["summary"]["files"], 3);
    assert_eq!(dir.report()["summary"]["could_not_read"], 1);
    assert!(dir.output().join("odd-_.pdf").exists());
    fs::write(dir.input().join("unreadable.pub"), PUB).unwrap();
    fs::set_permissions(dir.input().join("unreadable.pub"), fs::Permissions::from_mode(0o000)).unwrap();
    if fs::File::open(dir.input().join("unreadable.pub")).is_err() {
        assert_code(&dir.run(&["--overwrite"]), 1);
        let report = dir.report();
        let entry = report["files"].as_array().unwrap().iter().find(|e| e["file"] == "unreadable.pub").unwrap();
        assert_eq!(entry["result"], "Could not read");
        assert!(entry["message"].as_str().unwrap().to_lowercase().contains("permission denied"));
    }
    fs::set_permissions(dir.input().join("unreadable.pub"), fs::Permissions::from_mode(0o600)).unwrap();
}
#[test]
fn oversized_sparse_file_is_reported_without_reading_its_contents() {
    let dir = Dir::new();
    let file = fs::File::create(dir.input().join("huge.pub")).unwrap();
    file.set_len(512 * 1024 * 1024 + 1).unwrap();
    assert_code(&dir.run(&[]), 1);
    assert_eq!(dir.report()["files"][0]["message"], "This file is larger than the 512 MiB input limit.");
}
#[test]
fn output_nested_in_input_is_not_searched() {
    let dir = Dir::new();
    let output = dir.input().join("converted");
    fs::create_dir(&output).unwrap();
    fs::write(output.join("skip.pub"), PUB).unwrap();
    fs::write(dir.input().join("good.pub"), PUB).unwrap();
    let out =
        Command::new(env!("CARGO_BIN_EXE_designcraft-cli")).arg("convert-folder").arg(dir.input()).arg(&output).arg("--recursive").output().unwrap();
    assert_code(&out, 0);
    let value: Value = serde_json::from_slice(&fs::read(output.join("report.json")).unwrap()).unwrap();
    assert_eq!(value["summary"]["files"], 1);
}
#[test]
#[ignore = "needs PUB_CORPUS and PUB_REAL_CORPUS with the pinned external data"]
fn mixed_pinned_files_produce_both_reports_and_all_four_results() {
    let controlled = PathBuf::from(std::env::var_os("PUB_CORPUS").expect("set PUB_CORPUS"));
    let real = PathBuf::from(std::env::var_os("PUB_REAL_CORPUS").expect("set PUB_REAL_CORPUS"));
    let dir = Dir::new();
    for name in ["fill__fill-green.pub", "shapes__heart.pub", "wordart__arch.pub"] {
        fs::copy(controlled.join(name), dir.input().join(name)).unwrap();
    }
    fs::copy(real.join("Sample98.pub"), dir.input().join("older.pub")).unwrap();
    fs::copy(real.join("clusterfuzz-testcase-minimized-POIHPBFFuzzer-4701121678278656.pub"), dir.input().join("damaged.pub")).unwrap();
    assert_code(&dir.run(&[]), 1);
    let value = dir.report();
    assert_eq!(
        value["summary"],
        serde_json::json!({"files":5,"converted_no_warnings":2,"converted_check":1,"not_supported":1,"could_not_read":1,"total_pages":3})
    );
    let md = dir.markdown();
    for label in [
        "Converted, no warnings",
        "Converted, check these things",
        "Not supported yet",
        "Could not read",
        "Suggestion: Open and re-save this file in Publisher 2002 or newer",
        "Curved WordArt uses measured font advances",
        "appears damaged",
    ] {
        assert!(md.contains(label), "{md}");
    }
    assert!(!md.contains("0x"));
    for entry in value["files"].as_array().unwrap() {
        assert!(md.contains(entry["result"].as_str().unwrap()));
        if let Some(pdf) = entry["pdf"].as_str() {
            assert_eq!(designcraft_render::pdf_page_count(&fs::read(dir.output().join(pdf)).unwrap()), Some(1));
        }
    }
    let original = fs::read(dir.output().join("shapes__heart.pdf")).unwrap();
    assert_code(&dir.run(&[]), 1);
    assert_eq!(fs::read(dir.output().join("shapes__heart.pdf")).unwrap(), original);
    // Keep an example report in the ignored review directory for inspection.
    let example = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/ab-review/sample");
    fs::create_dir_all(&example).unwrap();
    fs::write(example.join("report.md"), md).unwrap();
    fs::write(example.join("report.json"), serde_json::to_vec_pretty(&value).unwrap()).unwrap();
}
#[test]
#[ignore = "needs PUB_REAL_CORPUS with the pinned external data"]
fn whole_real_folder_reports_nineteen_conversions_two_refusals_three_damaged() {
    let dir = Dir::new();
    let input = PathBuf::from(std::env::var_os("PUB_REAL_CORPUS").expect("set PUB_REAL_CORPUS"));
    let out = Command::new(env!("CARGO_BIN_EXE_designcraft-cli")).arg("convert-folder").arg(input).arg(dir.output()).output().unwrap();
    assert_code(&out, 1);
    let value = dir.report();
    let s = &value["summary"];
    assert_eq!(s["files"], 24);
    assert_eq!(s["converted_no_warnings"].as_u64().unwrap() + s["converted_check"].as_u64().unwrap(), 19);
    assert_eq!(s["not_supported"], 2);
    assert_eq!(s["could_not_read"], 3);
    assert_eq!(s["total_pages"], 31);
    assert!(!dir.markdown().contains("0x"));
}
