// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! Recursive Publisher conversion. Each file runs in a fresh process so a crash cannot end a batch.
use designcraft_engine::Session;
use serde_json::{Value, json};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{Duration, Instant},
};

const HELP: &str = "usage: designcraft-cli batch-convert INPUT_DIR --out OUTPUT_DIR [--timeout-seconds N]\nRecursively converts .pub files (case-insensitive) to PDF and .designcraft.\nOutputs preserve subfolders and append .pdf/.designcraft to the complete input name.\nWrites report.csv with file, pages, objects, status, warnings, pdf_path and failure_reason; existing outputs/reports are never overwritten.\nSymbolic links are not followed. Each file has a separate worker and a 120-second default timeout.\nProcesses all files, then exits unsuccessfully if any failed.";
const MAX_FILES: usize = 100_000;
const MAX_INPUT: u64 = 512 * 1024 * 1024;
const MAX_RESULT: u64 = 2 * 1024 * 1024;

#[derive(Default)]
struct Outcome {
    pages: Option<usize>,
    objects: Option<usize>,
    pdf_path: Option<PathBuf>,
    failure_reason: Option<String>,
    failed: bool,
    warnings: Vec<String>,
}
impl Outcome {
    fn failure(message: impl Into<String>) -> Self {
        let mut r = Self::default();
        r.fail(message);
        r
    }
    fn fail(&mut self, message: impl Into<String>) {
        self.failed = true;
        if self.failure_reason.is_none() {
            self.failure_reason = Some(message.into().chars().take(1000).collect());
        }
    }
    fn warn(&mut self, message: impl Into<String>) {
        let original = message.into();
        let mut message: String = original.chars().take(1000).collect();
        if original.chars().nth(1000).is_some() {
            message.push_str("… [truncated]");
        }
        if !self.warnings.contains(&message) {
            if self.warnings.len() < 200 {
                self.warnings.push(message);
            } else if self.warnings.len() == 200 {
                self.warnings.push("Additional warnings omitted (report limit).".into());
            }
        }
    }
    fn add_warnings(&mut self, value: &Value) {
        for warning in value.get("warnings").and_then(Value::as_array).into_iter().flatten() {
            if let Some(message) = warning.as_str() {
                self.warn(message);
            }
        }
    }
    fn status(&self) -> &'static str {
        if self.failed {
            "failed"
        } else if self.warnings.is_empty() {
            "converted OK"
        } else {
            "converted with warnings"
        }
    }
    fn value(&self) -> Value {
        json!({"pages": self.pages, "objects": self.objects, "failed": self.failed, "warnings": self.warnings, "failure_reason": self.failure_reason})
    }
    fn from_value(v: Value) -> Result<Self, String> {
        let failed = v.get("failed").and_then(Value::as_bool).ok_or("invalid worker result")?;
        let pages = match v.get("pages") {
            Some(Value::Null) => None,
            Some(n) => Some(n.as_u64().filter(|n| *n <= 10_000).ok_or("invalid worker page count")? as usize),
            None => return Err("missing worker page count".into()),
        };
        let objects = match v.get("objects") {
            Some(Value::Null) => None,
            Some(n) => Some(n.as_u64().filter(|n| *n <= 1_000_000).ok_or("invalid worker object count")? as usize),
            None => return Err("missing worker object count".into()),
        };
        let failure_reason = match v.get("failure_reason") {
            Some(Value::Null) if !failed => None,
            Some(Value::String(reason)) if failed && !reason.is_empty() => Some(reason.chars().take(1000).collect()),
            _ => return Err("invalid worker failure reason".into()),
        };
        if !failed && (pages.is_none() || objects.is_none()) {
            return Err("successful worker is missing document counts".into());
        }
        let mut out = Self { pages, objects, failed, failure_reason, ..Self::default() };
        if !v.get("warnings").is_some_and(Value::is_array) {
            return Err("invalid worker warnings".into());
        }
        out.add_warnings(&v);
        Ok(out)
    }
}
fn bounded_read(path: &Path, limit: u64) -> Result<Vec<u8>, String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    if file.metadata().map_err(|e| e.to_string())?.len() > limit {
        return Err(format!("file exceeds {limit}-byte limit"));
    }
    let mut bytes = Vec::new();
    file.take(limit + 1).read_to_end(&mut bytes).map_err(|e| e.to_string())?;
    if bytes.len() as u64 > limit {
        return Err(format!("file exceeds {limit}-byte limit"));
    }
    Ok(bytes)
}
fn convert(input: &Path, job: &Path) -> Outcome {
    let bytes = match bounded_read(input, MAX_INPUT) {
        Ok(b) => b,
        Err(e) => return Outcome::failure(format!("read: {e}")),
    };
    let mut session = Session::new();
    // The same engine Open Bytes command used by the app, with a bounded input read.
    let opened = match session.execute(
        "file.openBytes",
        &json!({"name": input.file_name().map(|s| s.to_string_lossy()).unwrap_or_default(), "base64": designcraft_engine::cmd::base64_encode(&bytes)}),
    ) {
        Ok(result) => result,
        Err(e) => return Outcome::failure(format!("import: {e}")),
    };
    let document = match session.doc() {
        Ok(state) => &state.doc,
        Err(e) => return Outcome::failure(format!("import: {e}")),
    };
    // Count editable spread objects, including nested objects and their group containers.
    let mut objects = 0usize;
    for item in document.spreads.iter().flat_map(|spread| &spread.items) {
        item.walk(&mut |_| objects = objects.saturating_add(1));
    }
    let mut out = Outcome { pages: Some(document.page_count()), objects: Some(objects), ..Outcome::default() };
    out.add_warnings(&opened);
    for (command, filename) in [("file.saveAs", "document.designcraft"), ("file.exportPdf", "document.pdf")] {
        let target = job.join(filename);
        match session.execute(command, &json!({"path": target.to_string_lossy()})) {
            Ok(result) => out.add_warnings(&result),
            Err(e) => {
                out.fail(format!("{command}: {e}"));
            }
        }
    }
    out
}
/// Internal process entry point, deliberately absent from general CLI help.
pub(crate) fn worker(args: &[String]) -> Result<(), String> {
    let [input, job] = args else { return Err("invalid batch worker arguments".into()) };
    let (input, job) = (Path::new(input), Path::new(job));
    for name in ["document.designcraft", "document.pdf", "result.json"] {
        if job.join(name).symlink_metadata().is_ok() {
            return Err("batch worker output already exists".into());
        }
    }
    let result = if fs::symlink_metadata(input).is_ok_and(|m| m.is_file() && !m.file_type().is_symlink()) {
        convert(input, job)
    } else {
        Outcome::failure("input is not a regular file (symbolic links are not followed)")
    };
    let bytes = serde_json::to_vec(&result.value()).map_err(|e| e.to_string())?;
    let mut file = OpenOptions::new().write(true).create_new(true).open(job.join("result.json")).map_err(|e| e.to_string())?;
    file.write_all(&bytes).map_err(|e| e.to_string())
}
fn is_pub(path: &Path) -> bool {
    path.extension().is_some_and(|s| s.eq_ignore_ascii_case("pub"))
}
fn discover(root: &Path, output: &Path) -> (Vec<PathBuf>, Vec<(PathBuf, Outcome)>) {
    let mut files = Vec::new();
    let mut errors = Vec::new();
    let mut dirs = vec![(root.to_path_buf(), 0)];
    let mut entries_seen = 0usize;
    while let Some((dir, depth)) = dirs.pop() {
        let entries = match fs::read_dir(&dir) {
            Ok(e) => e,
            Err(e) => {
                errors.push((dir, Outcome::failure(format!("discovery: {e}"))));
                continue;
            }
        };
        for entry in entries {
            entries_seen += 1;
            if entries_seen > 1_000_000 || files.len() + errors.len() >= MAX_FILES {
                errors.push((dir, Outcome::failure("discovery limit exceeded; remaining entries were not visited")));
                return (files, errors);
            }
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    errors.push((dir.clone(), Outcome::failure(format!("discovery: {e}"))));
                    continue;
                }
            };
            let path = entry.path();
            if path == output {
                continue;
            }
            match entry.file_type() {
                Ok(t) if t.is_dir() && depth < 128 => dirs.push((path, depth + 1)),
                Ok(t) if t.is_dir() => errors.push((path, Outcome::failure("discovery depth limit exceeded"))),
                Ok(t) if t.is_file() && is_pub(&path) => files.push(path),
                Ok(t) if t.is_symlink() && is_pub(&path) => errors.push((path, Outcome::failure("symbolic link skipped"))),
                Err(e) => errors.push((path, Outcome::failure(format!("discovery: {e}")))),
                _ => {}
            }
        }
    }
    files.sort();
    errors.sort_by(|a, b| a.0.cmp(&b.0));
    (files, errors)
}
fn csv(value: &str) -> String {
    format!("\"{}\"", value.replace('"', "\"\""))
}
fn row(report: &mut File, name: &Path, outcome: &Outcome) -> Result<(), String> {
    writeln!(
        report,
        "{},{},{},{},{},{},{}",
        csv(&name.to_string_lossy()),
        outcome.pages.map(|n| n.to_string()).unwrap_or_default(),
        outcome.objects.map(|n| n.to_string()).unwrap_or_default(),
        csv(outcome.status()),
        csv(&outcome.warnings.join("\n")),
        csv(&outcome.pdf_path.as_ref().map(|p| p.to_string_lossy()).unwrap_or_default()),
        csv(outcome.failure_reason.as_deref().unwrap_or_default())
    )
    .map_err(|e| format!("write report: {e}"))?;
    report.flush().map_err(|e| format!("flush report: {e}"))
}
fn run_worker(input: &Path, job: &Path, timeout: Duration) -> Outcome {
    if input.to_str().is_none() || job.to_str().is_none() {
        return Outcome::failure("path cannot be represented by this CLI's UTF-8 arguments");
    }
    let execute = || -> Result<Outcome, String> {
        let mut child = Command::new(std::env::current_exe().map_err(|e| e.to_string())?)
            .arg("__batch-convert-one")
            .arg(input)
            .arg(job)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| e.to_string())?;
        let started = Instant::now();
        let status = loop {
            match child.try_wait() {
                Ok(Some(status)) => break status,
                Ok(None) if started.elapsed() < timeout => std::thread::sleep(Duration::from_millis(25)),
                Ok(None) => {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err("conversion worker timed out".into());
                }
                Err(e) => {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(format!("wait for worker: {e}"));
                }
            }
        };
        if !status.success() {
            return Err(format!("conversion worker exited {status}; no successful conversion was recorded"));
        }
        let bytes = bounded_read(&job.join("result.json"), MAX_RESULT)?;
        Outcome::from_value(serde_json::from_slice(&bytes).map_err(|e| format!("worker result: {e}"))?)
    };
    execute().unwrap_or_else(Outcome::failure)
}
fn destinations(root: &Path, relative: &Path) -> Result<(PathBuf, PathBuf), String> {
    let name = relative.file_name().ok_or("input has no filename")?;
    let parent = root.join(relative.parent().unwrap_or_else(|| Path::new("")));
    let mut native = name.to_os_string();
    native.push(".designcraft");
    let mut pdf = name.to_os_string();
    pdf.push(".pdf");
    Ok((parent.join(native), parent.join(pdf)))
}
fn output_parent(root: &Path, relative: &Path) -> Result<(), String> {
    let mut dir = root.to_path_buf();
    for part in relative.parent().unwrap_or_else(|| Path::new("")).components() {
        if !matches!(part, std::path::Component::Normal(_)) {
            return Err("invalid relative output directory".into());
        }
        dir.push(part);
        match fs::create_dir(&dir) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                let m = fs::symlink_metadata(&dir).map_err(|e| e.to_string())?;
                if !m.is_dir() || m.file_type().is_symlink() {
                    return Err("output subfolder is not a regular directory".into());
                }
            }
            Err(e) => return Err(e.to_string()),
        }
    }
    Ok(())
}
struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
pub(crate) fn run(args: &[String]) -> Result<(), String> {
    if args == ["--help"] || args == ["-h"] {
        outln!("{HELP}");
        return Ok(());
    }
    let input = args.first().filter(|a| !a.starts_with('-')).ok_or(HELP)?;
    let mut output = None;
    let mut seconds = 120u64;
    let mut it = args.iter().skip(1);
    while let Some(flag) = it.next() {
        match flag.as_str() {
            "--out" if output.is_none() => output = Some(it.next().ok_or("--out needs a folder")?),
            "--timeout-seconds" => {
                seconds = it
                    .next()
                    .ok_or("--timeout-seconds needs a number")?
                    .parse()
                    .ok()
                    .filter(|n| (1..=3600).contains(n))
                    .ok_or("timeout must be 1..3600 seconds")?
            }
            _ => return Err(format!("unknown or repeated option {flag}\n{HELP}")),
        }
    }
    let input = fs::canonicalize(input).map_err(|e| format!("input folder: {e}"))?;
    if !input.is_dir() {
        return Err("input must be a folder".into());
    }
    let output = Path::new(output.ok_or(HELP)?);
    fs::create_dir_all(output).map_err(|e| format!("output folder: {e}"))?;
    let output = fs::canonicalize(output).map_err(|e| e.to_string())?;
    if input == output {
        return Err("output folder must differ from input folder".into());
    }
    if output.to_str().is_none() {
        return Err("output path is not UTF-8".into());
    }
    let report_path = output.join("report.csv");
    let mut report = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&report_path)
        .map_err(|e| format!("create {} (existing reports are not overwritten): {e}", report_path.display()))?;
    writeln!(report, "file,pages,objects,status,warnings,pdf_path,failure_reason").map_err(|e| e.to_string())?;
    let scratch_path = output.join(format!(".batch-work-{}", std::process::id()));
    fs::create_dir(&scratch_path).map_err(|e| format!("create batch workspace: {e}"))?;
    let scratch = Scratch(scratch_path);
    let (files, errors) = discover(&input, &output);
    let mut ok = 0usize;
    let mut warned = 0usize;
    let mut failed = 0usize;
    for (path, outcome) in errors {
        row(&mut report, path.strip_prefix(&input).unwrap_or(&path), &outcome)?;
        failed += 1;
    }
    for (i, path) in files.iter().enumerate() {
        let relative = path.strip_prefix(&input).map_err(|e| e.to_string())?;
        let job = scratch.0.join(i.to_string());
        let outcome = match fs::create_dir(&job) {
            Ok(()) => match destinations(&output, relative) {
                Ok((native, pdf)) if !native.try_exists().unwrap_or(true) && !pdf.try_exists().unwrap_or(true) => {
                    let mut result = run_worker(path, &job, Duration::from_secs(seconds));
                    if !result.failed {
                        let publish = || -> Result<(), String> {
                            output_parent(&output, relative)?;
                            // Hard links publish complete staged files without replacing existing paths.
                            fs::hard_link(job.join("document.designcraft"), &native).map_err(|e| format!("publish NiftyDesign: {e}"))?;
                            fs::hard_link(job.join("document.pdf"), &pdf).map_err(|e| format!("publish PDF (native output may already exist): {e}"))
                        };
                        if let Err(e) = publish() {
                            result.fail(e);
                        } else {
                            result.pdf_path = Some(pdf);
                        }
                    }
                    result
                }
                Ok(_) => Outcome::failure("output already exists or cannot be inspected; neither output was overwritten"),
                Err(e) => Outcome::failure(e),
            },
            Err(e) => Outcome::failure(format!("create worker folder: {e}")),
        };
        if outcome.failed {
            failed += 1;
        } else if outcome.warnings.is_empty() {
            ok += 1;
        } else {
            warned += 1;
        }
        row(&mut report, relative, &outcome)?;
        let _ = fs::remove_dir_all(&job);
    }
    outln!(
        "{}",
        json!({"files": ok + warned + failed, "converted_ok": ok, "converted_with_warnings": warned, "failed": failed, "report": report_path})
    );
    if failed > 0 {
        return Err(format!("{failed} entries failed; all discoverable files were processed; see {}", report_path.display()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn csv_quotes_commas_quotes_and_newlines_and_names_do_not_collide() {
        assert_eq!(csv("a,\"b\n"), "\"a,\"\"b\n\"");
        let (a, _) = destinations(Path::new("out"), Path::new("nested/a.pub")).unwrap();
        let (b, _) = destinations(Path::new("out"), Path::new("nested/a.PUB")).unwrap();
        assert_ne!(a, b);
        assert_eq!(a, Path::new("out/nested/a.pub.designcraft"));
    }
    #[test]
    fn worker_reports_are_bounded_and_failures_stay_failures() {
        let mut out = Outcome::failure("bad input");
        for i in 0..1000 {
            out.warn(format!("warning {i}"));
        }
        assert_eq!(out.warnings.len(), 201);
        assert_eq!(Outcome::from_value(out.value()).unwrap().status(), "failed");
        for value in [
            json!({"failed":false,"warnings":[],"pages":10001,"objects":2,"failure_reason":null}),
            json!({"failed":false,"warnings":[],"pages":1,"objects":1_000_001,"failure_reason":null}),
            json!({"failed":false,"warnings":[],"pages":1,"objects":null,"failure_reason":null}),
            json!({"failed":true,"warnings":[],"pages":null,"objects":null,"failure_reason":null}),
        ] {
            assert!(Outcome::from_value(value).is_err());
        }
    }
}
