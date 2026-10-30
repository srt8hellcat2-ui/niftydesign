// Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
//! PDF-only folder conversion with matching human and machine-readable reports.
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::OsString,
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process::ExitCode,
};

const HELP: &str = "usage: designcraft-cli convert-folder INPUT_DIR OUTPUT_DIR [--recursive] [--overwrite]\nConvert .pub files (case-insensitive) to PDF, continuing after individual failures.\nBy default, search only INPUT_DIR; --recursive includes subfolders. Symbolic links are not followed.\nPDFs use safe filenames in OUTPUT_DIR; duplicate names receive numbered suffixes.\nWrite report.md and report.json with counts, pages and things to check for each file.\nExisting PDFs and reports are preserved unless --overwrite is given; output links are never followed.\nNo network access. Each file runs separately with a 120-second time limit and a 512 MiB input limit.\nExit codes: 0 = every file converted (also an empty folder); 1 = a file or operation failed; 2 = bad arguments.\nCheck every converted file against the original; no warning does not prove a perfect copy.";
const RESULTS: [&str; 4] = ["Converted, no warnings", "Converted, check these things", "Not supported yet", "Could not read"];
const NOTICE: &str = "Check every converted file against the original, because no warning does not prove a perfect copy.";
const MAX_FILES: usize = 100_000;

struct Options {
    input: PathBuf,
    output: PathBuf,
    recursive: bool,
    overwrite: bool,
}
fn options(args: &[OsString]) -> Result<Options, String> {
    let mut paths = Vec::new();
    let (mut recursive, mut overwrite, mut positional) = (false, false, false);
    for arg in args {
        match arg.to_str() {
            Some("--") if !positional => positional = true,
            Some("--recursive") if !positional && !recursive => recursive = true,
            Some("--overwrite") if !positional && !overwrite => overwrite = true,
            _ if !positional && arg.to_string_lossy().starts_with('-') => {
                return Err(format!("Unknown or repeated option: {}\n{HELP}", arg.to_string_lossy()));
            }
            _ => paths.push(PathBuf::from(arg)),
        }
    }
    let [input, output] = paths.as_slice() else { return Err(HELP.into()) };
    if input.as_os_str().is_empty() || output.as_os_str().is_empty() {
        return Err(format!("Folder paths must not be empty.\n{HELP}"));
    }
    if input.is_file() {
        return Err("INPUT_DIR must be a folder, not a file.".into());
    }
    if let (Ok(input), Ok(output)) = (fs::canonicalize(input), fs::canonicalize(output))
        && input == output
    {
        return Err("INPUT_DIR and OUTPUT_DIR must be different folders.".into());
    }
    Ok(Options { input: input.clone(), output: output.clone(), recursive, overwrite })
}
pub(crate) fn entry(args: &[OsString]) -> ExitCode {
    if args == ["--help"] || args == ["-h"] {
        outln!("{HELP}");
        return ExitCode::SUCCESS;
    }
    let opts = match options(args) {
        Ok(opts) => opts,
        Err(e) => {
            eprintln!("designcraft-cli: {e}");
            return ExitCode::from(2);
        }
    };
    match run(opts) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!("designcraft-cli: {e}");
            ExitCode::FAILURE
        }
    }
}

struct Entry {
    file: String,
    result: usize,
    pages: Option<usize>,
    pdf: Option<String>,
    warnings: Vec<String>,
    notes: Vec<String>,
    message: Option<String>,
}
impl Entry {
    fn failure(file: String, message: String) -> Self {
        Self { file, result: 3, pages: None, pdf: None, warnings: Vec::new(), notes: Vec::new(), message: Some(message) }
    }
    fn label(&self) -> &'static str {
        RESULTS.get(self.result).copied().unwrap_or("Could not read")
    }
    fn value(&self) -> Value {
        json!({"file":self.file,"result":self.label(),"pages":self.pages,"pdf":self.pdf,"warnings":self.warnings,"notes":self.notes,"message":self.message})
    }
}
fn plain(message: &str) -> String {
    if message.starts_with("A formatting setting this version does not understand was skipped;") {
        let kind = if message.contains("Publisher character fields:") { "character" } else { "paragraph" };
        return format!("Some {kind} formatting settings were skipped; the text is kept.");
    }
    if message.starts_with("Freeform escape ") {
        return "A freeform shape effect is not supported; that effect was skipped.".into();
    }
    if message.starts_with("Shape type ") {
        return "A shape type is not supported yet; it was imported as a rectangle of the same size.".into();
    }
    // Preserve descriptive wording while removing hexadecimal field identifiers.
    let mut out = String::new();
    let mut chars = message.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '0' && chars.peek() == Some(&'x') {
            chars.next();
            while chars.peek().is_some_and(char::is_ascii_hexdigit) {
                chars.next();
            }
            out.push_str("unrecognised setting");
        } else {
            out.push(c);
        }
    }
    if let Some((prefix, _)) = out.split_once(" (os error ") {
        return prefix.to_owned();
    }
    out
}
fn failure_words(reason: &str) -> String {
    if reason.contains("536870912-byte limit") {
        "This file is larger than the 512 MiB input limit.".into()
    } else if reason.starts_with("import:") && reason.contains("damaged Publisher file:") {
        "The file appears damaged and could not be read.".into()
    } else if reason.starts_with("import:") && reason.contains("not a Microsoft Publisher file:") {
        "This is not a readable Publisher file.".into()
    } else if let Some(reason) = reason.strip_prefix("file.exportPdf:") {
        format!("The PDF could not be saved: {}", plain(reason.trim()))
    } else {
        plain(reason.strip_prefix("read: ").unwrap_or(reason))
    }
}
fn name(path: &Path, root: &Path) -> String {
    let relative = path.strip_prefix(root).unwrap_or(path);
    if relative.as_os_str().is_empty() { ".".into() } else { relative.to_string_lossy().into_owned() }
}
fn discover(root: &Path, output: &Path, recursive: bool) -> (Vec<PathBuf>, Vec<Entry>) {
    let (mut files, mut errors) = (Vec::new(), Vec::new());
    let mut dirs = vec![(root.to_path_buf(), 0)];
    let mut seen = 0usize;
    while let Some((dir, depth)) = dirs.pop() {
        let entries = match fs::read_dir(&dir) {
            Ok(entries) => entries,
            Err(e) => {
                errors.push(Entry::failure(name(&dir, root), format!("The folder could not be read: {}", plain(&e.to_string()))));
                continue;
            }
        };
        for entry in entries {
            seen += 1;
            if seen > 1_000_000 || files.len() + errors.len() >= MAX_FILES {
                errors.push(Entry::failure(name(&dir, root), "The folder search limit was reached; remaining entries were not visited.".into()));
                return (files, errors);
            }
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    errors.push(Entry::failure(name(&dir, root), format!("A folder entry could not be read: {}", plain(&e.to_string()))));
                    continue;
                }
            };
            let path = entry.path();
            if path == output {
                continue;
            }
            let pub_file = path.extension().is_some_and(|s| s.eq_ignore_ascii_case("pub"));
            match entry.file_type() {
                Ok(t) if t.is_dir() && recursive && depth < 128 => dirs.push((path, depth + 1)),
                Ok(t) if t.is_dir() && recursive => {
                    errors.push(Entry::failure(name(&path, root), "Subfolders are nested too deeply; this folder was skipped.".into()))
                }
                Ok(t) if t.is_file() && pub_file => files.push(path),
                Ok(t) if t.is_symlink() && pub_file => {
                    errors.push(Entry::failure(name(&path, root), "This file is a symbolic link and was skipped.".into()))
                }
                Err(e) => errors.push(Entry::failure(name(&path, root), plain(&e.to_string()))),
                _ => {}
            }
        }
    }
    files.sort();
    (files, errors)
}
fn safe_stem(path: &Path) -> String {
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let clean: String =
        stem.chars().take(100).map(|c| if c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | ' ') { c } else { '_' }).collect();
    let mut clean = clean.trim_matches(['.', ' ']).to_owned();
    if clean.is_empty() {
        clean = "document".into();
    }
    let device = clean.split('.').next().unwrap_or_default().to_ascii_uppercase();
    if matches!(device.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || (device.len() == 4
            && (device.starts_with("COM") || device.starts_with("LPT"))
            && device.ends_with(['1', '2', '3', '4', '5', '6', '7', '8', '9']))
    {
        clean.insert_str(0, "file-");
    }
    clean
}
fn pdf_name(path: &Path, used: &mut BTreeSet<String>, counts: &mut BTreeMap<String, usize>) -> String {
    let stem = safe_stem(path);
    let count = counts.entry(stem.to_ascii_lowercase()).or_default();
    loop {
        *count = count.saturating_add(1);
        let candidate = if *count == 1 { format!("{stem}.pdf") } else { format!("{stem}-{count}.pdf") };
        if used.insert(candidate.to_ascii_lowercase()) {
            return candidate;
        }
    }
}
fn check_target(path: &Path, overwrite: bool) -> Result<(), String> {
    match fs::symlink_metadata(path) {
        Ok(m) if m.file_type().is_symlink() || !m.is_file() => {
            Err(format!("{} is not a regular output file; it was left unchanged.", path.display()))
        }
        Ok(_) if !overwrite => Err(format!("{} already exists; it was left unchanged. Use --overwrite to replace it.", path.display())),
        Ok(_) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(format!("Cannot inspect {}: {}", path.display(), plain(&e.to_string()))),
    }
}
fn publish(staged: &Path, target: &Path, overwrite: bool) -> Result<(), String> {
    check_target(target, overwrite)?;
    // Replace directory entries, never write through existing links or hard links.
    if overwrite {
        fs::rename(staged, target).map_err(|e| format!("Could not save {}: {}", target.display(), plain(&e.to_string())))
    } else {
        fs::hard_link(staged, target).map_err(|e| format!("Could not save {}: {}", target.display(), plain(&e.to_string())))
    }
}
struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
// Reserve both report names before converting, so existing reports are never lost.
fn reserve_reports(output: &Path, overwrite: bool) -> Result<Vec<File>, String> {
    for name in ["report.md", "report.json"] {
        check_target(&output.join(name), overwrite)?;
    }
    let mut files = Vec::new();
    if !overwrite {
        for name in ["report.md", "report.json"] {
            match OpenOptions::new().write(true).create_new(true).open(output.join(name)) {
                Ok(file) => files.push(file),
                Err(e) => {
                    if !files.is_empty() {
                        files.clear();
                        let _ = fs::remove_file(output.join("report.md"));
                    }
                    return Err(format!("Could not create {name}: {}", plain(&e.to_string())));
                }
            }
        }
    }
    Ok(files)
}
fn md_text(text: &str) -> String {
    text.chars()
        .flat_map(|c| match c {
            '\n' | '\r' | '\t' => vec![' '],
            '<' => "&lt;".chars().collect(),
            '>' => "&gt;".chars().collect(),
            '&' => "&amp;".chars().collect(),
            '\\' | '`' | '*' | '_' | '[' | ']' | '#' | '|' | '!' => vec!['\\', c],
            c if c.is_control() => vec![' '],
            c => vec![c],
        })
        .collect()
}
fn reports(entries: &[Entry]) -> (Value, String, bool) {
    let mut counts = [0usize; 4];
    let mut pages = 0usize;
    for entry in entries {
        if let Some(count) = counts.get_mut(entry.result) {
            *count += 1;
        }
        if entry.result < 2 {
            pages = pages.saturating_add(entry.pages.unwrap_or(0));
        }
    }
    let summary = json!({"files":entries.len(),"converted_no_warnings":counts.first(),"converted_check":counts.get(1),"not_supported":counts.get(2),"could_not_read":counts.get(3),"total_pages":pages});
    let message = if entries.is_empty() { "No .pub files were found." } else { "All discovered files were processed." };
    let mut md = format!("# Folder conversion report\n\nFiles: {}. Total converted pages: {pages}.\n\n", entries.len());
    for (label, count) in RESULTS.iter().zip(counts) {
        md.push_str(&format!("- {label}: {count}\n"));
    }
    md.push_str(&format!("\n{message}\n"));
    for entry in entries {
        md.push_str(&format!("\n## {}\n\n{}\n", md_text(&entry.file), entry.label()));
        if let Some(pdf) = &entry.pdf {
            md.push_str(&format!("\nPDF: {}. Pages: {}.\n", md_text(pdf), entry.pages.unwrap_or(0)));
        }
        if let Some(message) = &entry.message {
            for paragraph in message.lines() {
                md.push_str(&format!("\n{}\n", md_text(paragraph)));
            }
        }
        for (label, messages) in [("Things to check", &entry.warnings), ("Notes", &entry.notes)] {
            if !messages.is_empty() {
                md.push_str(&format!("\n{label}:\n\n"));
                for message in messages {
                    md.push_str(&format!("- {}\n", md_text(message)));
                }
            }
        }
    }
    md.push_str(&format!("\n{NOTICE}\n"));
    let ok = counts.get(2..).is_some_and(|counts| counts.iter().all(|n| *n == 0));
    (json!({"summary":summary,"message":message,"files":entries.iter().map(Entry::value).collect::<Vec<_>>(),"notice":NOTICE}), md, ok)
}
fn run(opts: Options) -> Result<bool, String> {
    fs::create_dir_all(&opts.output).map_err(|e| format!("The output folder could not be created: {}", plain(&e.to_string())))?;
    let output = fs::canonicalize(&opts.output).map_err(|e| format!("The output folder could not be opened: {}", plain(&e.to_string())))?;
    if output.to_str().is_none() {
        return Err("The output folder path must be valid UTF-8; choose another output folder.".into());
    }
    let input = fs::canonicalize(&opts.input).unwrap_or(opts.input);
    if input == output {
        return Err("The input and output folders must be different.".into());
    }
    let mut report_files = reserve_reports(&output, opts.overwrite)?;
    let scratch_path = output.join(format!(".folder-work-{}", std::process::id()));
    fs::create_dir(&scratch_path).map_err(|e| format!("Could not create the conversion workspace: {}", plain(&e.to_string())))?;
    let scratch = Scratch(scratch_path);
    let (files, mut entries) = discover(&input, &output, opts.recursive);
    let (mut used, mut counts) = (BTreeSet::new(), BTreeMap::new());
    for (index, path) in files.iter().enumerate() {
        let pdf = pdf_name(path, &mut used, &mut counts);
        let job = scratch.0.join(index.to_string());
        let converted = || -> Result<Entry, String> {
            check_target(&output.join(&pdf), opts.overwrite)?;
            fs::create_dir(&job).map_err(|e| plain(&e.to_string()))?;
            let outcome = crate::batch::run_pdf_worker(path, &job);
            let mut entry = Entry::failure(name(path, &input), String::new());
            if outcome.failed {
                let reason = outcome.failure_reason.as_deref().unwrap_or("The file could not be converted.");
                if reason.contains("Publisher 97/98/2000 files are not supported yet") {
                    entry.result = 2;
                    entry.message = Some(format!(
                        "{}\nSuggestion: Open and re-save this file in Publisher 2002 or newer, then try again.",
                        reason.split_once("unsupported Publisher file: ").map(|(_, message)| message).unwrap_or(reason)
                    ));
                } else {
                    entry.message = Some(failure_words(reason));
                }
            } else {
                publish(&job.join("document.pdf"), &output.join(&pdf), opts.overwrite)?;
                entry.result = if outcome.warnings.is_empty() && outcome.notes.is_empty() { 0 } else { 1 };
                entry.message = None;
                entry.pages = outcome.pages;
                entry.pdf = Some(pdf.clone());
            }
            entry.warnings = outcome.warnings.iter().map(|w| plain(w)).collect();
            entry.notes = outcome.notes.iter().map(|n| plain(n)).collect();
            Ok(entry)
        };
        entries.push(converted().unwrap_or_else(|e| Entry::failure(name(path, &input), e)));
        let _ = fs::remove_dir_all(job);
    }
    entries.sort_by(|a, b| a.file.cmp(&b.file));
    let (value, markdown, ok) = reports(&entries);
    let json = serde_json::to_vec_pretty(&value).map_err(|e| e.to_string())?;
    if opts.overwrite {
        for (name, bytes) in [("report.md", markdown.as_bytes()), ("report.json", json.as_slice())] {
            let staged = scratch.0.join(name);
            fs::write(&staged, bytes).map_err(|e| format!("Could not write {name}: {}", plain(&e.to_string())))?;
            publish(&staged, &output.join(name), true)?;
        }
    } else {
        for (file, bytes) in report_files.iter_mut().zip([markdown.as_bytes(), json.as_slice()]) {
            file.write_all(bytes).and_then(|()| file.flush()).map_err(|e| format!("Could not write the report: {}", plain(&e.to_string())))?;
        }
    }
    outln!("{}", value.get("summary").unwrap_or(&Value::Null));
    outln!("Reports: {} and {}", output.join("report.md").display(), output.join("report.json").display());
    Ok(ok)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn names_are_portable_and_duplicate_suffixes_do_not_collide() {
        let (mut used, mut counts) = (BTreeSet::new(), BTreeMap::new());
        for (input, expected) in [
            ("CON.pub", "file-CON.pdf"),
            ("../a?.pub", "a_.pdf"),
            ("a*.pub", "a_-2.pdf"),
            ("A_.PUB", "A_-3.pdf"),
            ("a_-2.pub", "a_-2-2.pdf"),
            ("....pub", "document.pdf"),
        ] {
            assert_eq!(pdf_name(Path::new(input), &mut used, &mut counts), expected);
        }
    }
    #[test]
    fn reports_use_plain_words_and_escape_markdown_without_losing_json_names() {
        assert_eq!(
            plain("Freeform escape 0x0C is not implemented; its path effect was skipped."),
            "A freeform shape effect is not supported; that effect was skipped."
        );
        assert_eq!(
            plain(
                "A formatting setting this version does not understand was skipped; the text is kept. Publisher paragraph fields: defaults [0x30]; runs [none]."
            ),
            "Some paragraph formatting settings were skipped; the text is kept."
        );
        assert!(!plain("Unknown property 0x1234").contains("0x1234"));
        let filename = "<a>\n# [b]*.pub";
        let (value, md, ok) = reports(&[Entry::failure(filename.into(), "Cannot open file".into())]);
        assert!(!ok);
        assert_eq!(value["files"][0]["file"], filename);
        assert!(md.contains("&lt;a&gt; \\# \\[b\\]\\*.pub"));
        assert_eq!(value["summary"]["could_not_read"], 1);
        assert!(md.ends_with(&format!("{NOTICE}\n")));
    }
}
