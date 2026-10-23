// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! Batch conversion must finish good files even when an earlier input or output is bad.
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
};
const PUB: &[u8] = include_bytes!("../../../crates/pub/tests/data/hello-nifty.pub");
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Dir(PathBuf);
impl Dir {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!("dc-batch-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        fs::create_dir(&p).unwrap();
        Self(p)
    }
}
impl Drop for Dir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn cli(input: &Path, output: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_designcraft-cli")).arg("batch-convert").arg(input).arg("--out").arg(output).output().unwrap()
}
#[test]
fn recursive_batch_continues_after_bad_input_and_writes_both_valid_formats() {
    let dir = Dir::new();
    let input = dir.0.join("input");
    let output = input.join("output");
    fs::create_dir_all(input.join("nested,folder")).unwrap();
    fs::create_dir(&output).unwrap();
    fs::write(output.join("excluded.pub"), b"not part of input").unwrap();
    fs::write(input.join("00-bad.pub"), b"broken file").unwrap();
    fs::write(input.join("same.pub"), PUB).unwrap();
    fs::write(input.join("nested,folder/same.PUB"), PUB).unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink(&input, input.join("loop")).unwrap();
    let result = cli(&input, &output);
    assert!(!result.status.success(), "one failed input must produce an unsuccessful exit after processing everything");
    let summary: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(summary["files"], 3, "{}", String::from_utf8_lossy(&result.stderr));
    assert_eq!(summary["failed"], 1);
    assert_eq!(summary["converted_ok"].as_u64().unwrap() + summary["converted_with_warnings"].as_u64().unwrap(), 2);
    for name in ["same.pub", "nested,folder/same.PUB"] {
        let native = fs::read(output.join(format!("{name}.designcraft"))).unwrap();
        let doc = designcraft_format::load(&native).unwrap();
        assert_eq!(doc.page_count(), 1);
        assert_eq!(doc.spreads.iter().map(|s| s.items.len()).sum::<usize>(), 2);
        let pdf = fs::read(output.join(format!("{name}.pdf"))).unwrap();
        assert_eq!(designcraft_render::pdf_page_count(&pdf), Some(1));
    }
    let report = fs::read_to_string(output.join("report.csv")).unwrap();
    assert!(report.starts_with("file,pages,objects,status,warnings,pdf_path,failure_reason,notes,notice\n"));
    assert!(report.contains(designcraft_pub::CONVERSION_NOTICE));
    assert!(report.contains("\"00-bad.pub\",,,\"failed\""));
    assert!(report.contains(&format!("\"{}\",1,2,", Path::new("nested,folder").join("same.PUB").display())));
    assert!(report.contains("import:"));
    assert!(!report.contains("excluded.pub"));
    for name in ["same.pub", "nested,folder/same.PUB"] {
        assert_eq!(fs::read(input.join(name)).unwrap(), PUB);
        let pdf = output.join(format!("{name}.pdf"));
        assert!(report.contains(&format!(",\"{}\",\"\"", pdf.to_string_lossy())), "{report}");
    }
    assert_eq!(fs::read(input.join("00-bad.pub")).unwrap(), b"broken file");
    assert!(!output.join("00-bad.pub.pdf").exists());
    assert!(report.contains("\"failed\",\"\",\"\",\"import:"), "failure reason must be separate from warnings and PDF path: {report}");
}
#[test]
fn existing_output_and_report_are_preserved_while_other_files_convert() {
    let dir = Dir::new();
    let input = dir.0.join("input");
    let output = dir.0.join("output");
    fs::create_dir(&input).unwrap();
    fs::create_dir(&output).unwrap();
    fs::write(input.join("a.pub"), PUB).unwrap();
    fs::write(input.join("b.pub"), PUB).unwrap();
    fs::write(output.join("a.pub.pdf"), b"existing output").unwrap();
    let result = cli(&input, &output);
    assert!(!result.status.success());
    let summary: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(summary["failed"], 1);
    assert!(output.join("b.pub.pdf").is_file());
    assert_eq!(fs::read(output.join("a.pub.pdf")).unwrap(), b"existing output");
    assert!(!output.join("a.pub.designcraft").exists());
    let report = fs::read(output.join("report.csv")).unwrap();
    assert!(!cli(&input, &output).status.success());
    assert_eq!(fs::read(output.join("report.csv")).unwrap(), report);
}
#[test]
fn empty_folder_writes_a_zero_file_report_and_help_is_noninteractive() {
    let dir = Dir::new();
    let input = dir.0.join("input");
    let output = dir.0.join("output");
    fs::create_dir(&input).unwrap();
    let result = cli(&input, &output);
    assert!(result.status.success());
    let summary: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(summary["files"], 0);
    assert_eq!(fs::read_to_string(output.join("report.csv")).unwrap(), "file,pages,objects,status,warnings,pdf_path,failure_reason,notes,notice\n");
    assert!(Command::new(env!("CARGO_BIN_EXE_designcraft-cli")).args(["batch-convert", "--help"]).status().unwrap().success());
}

#[test]
#[ignore = "needs PUB_REAL_CORPUS pointing outside the repo at the pinned 24-file Apache POI .pub set"]
fn real_publisher_corpus_keeps_expected_status_counts() {
    use std::io::Read;

    let input =
        PathBuf::from(std::env::var_os("PUB_REAL_CORPUS").expect("set PUB_REAL_CORPUS to the pinned 24-file sample folder")).canonicalize().unwrap();
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").canonicalize().unwrap();
    assert!(input.is_dir() && !input.starts_with(&repo), "keep the sample inputs outside the repository");
    let dir = Dir::new();
    let output = dir.0.join("output");
    let result = cli(&input, &output);
    assert_eq!(result.status.code(), Some(1), "refused and damaged files must be reported after processing the whole set");
    let summary: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(summary["files"], 24);
    assert_eq!(summary["converted_ok"], 9);
    assert_eq!(summary["converted_with_warnings"], 10);
    assert_eq!(summary["failed"], 5, "the CLI combines two refusals with three damaged-file failures");

    const LIMIT: u64 = 16 * 1024 * 1024;
    let mut report = String::new();
    fs::File::open(output.join("report.csv")).unwrap().take(LIMIT + 1).read_to_string(&mut report).unwrap();
    assert!(report.len() as u64 <= LIMIT);
    // Failure rows in this pinned set have single-line errors and empty warnings.
    // Successful rows can contain multiline warnings, so count only failure records.
    let failures: Vec<_> = report.lines().filter(|line| line.contains(",,,\"failed\",\"\",\"\",\"import:")).collect();
    assert_eq!(failures.len(), 5);
    let refused = failures.iter().filter(|line| line.contains("unsupported Publisher file:")).count();
    assert_eq!(refused, 2);
    assert_eq!(failures.len() - refused, 3);
    for name in ["Sample98.pub", "Sample2000.pub"] {
        assert!(failures.iter().any(|line| line.starts_with(&format!("\"{name}\",")) && line.contains("unsupported Publisher file:")));
    }
}
