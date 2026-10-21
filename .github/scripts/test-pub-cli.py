#!/usr/bin/env python3
# Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
"""Exercise a tester binary against pinned .pub data only; never fetch reader code."""
import argparse
import collections
import csv
import hashlib
import json
from pathlib import Path
import subprocess
import urllib.parse
import urllib.request

REPO = Path(__file__).resolve().parents[2]
LIMIT = 32 * 1024 * 1024
SOURCES = {
    "controlled": "https://raw.githubusercontent.com/FuncularLabs/pubsmith/0ccdf27/tests/fixtures/pub/",
    "real": "https://raw.githubusercontent.com/apache/poi/bf7c9b6a1490046517cbd8e1504e702a89be4bc9/test-data/publisher/",
}
EXPECTED = {
    "controlled": ({"OK": 79, "warnings": 63}, 146, 151, 0),
    "real": ({"warnings": 19, "refused": 2, "failed": 3}, 31, 280, 1),
}


def check(condition, message):
    if not condition:
        raise RuntimeError(message)


def digest(path):
    check(path.stat().st_size <= LIMIT, f"oversized input: {path}")
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def prepare(folder, records, download):
    check(not folder.is_relative_to(REPO), "sample folders must be outside the repo")
    folder.mkdir(parents=True, exist_ok=True)
    for row in records:
        name = row["file"]
        check(Path(name).name == name and name.endswith(".pub"), "unsafe manifest filename")
        path = folder / name
        if download and not path.exists():
            # Only these individual .pub URLs are requested. Never list/clone a repo.
            url = SOURCES[row["set"]] + urllib.parse.quote(name)
            with urllib.request.urlopen(url, timeout=30) as response:
                data = response.read(LIMIT + 1)
            check(len(data) <= LIMIT, f"oversized download: {name}")
            check(hashlib.sha256(data).hexdigest() == row["sha256"], f"download hash mismatch: {name}")
            path.write_bytes(data)
        check(path.is_file() and not path.is_symlink(), f"missing or linked sample: {path}")
        check(path.stat().st_size == int(row["bytes"]) and digest(path) == row["sha256"], f"input mismatch: {name}")
    expected = {row["file"] for row in records}
    actual = {str(p.relative_to(folder)) for p in folder.rglob("*") if p.suffix.lower() == ".pub"}
    check(actual == expected, "folder must contain exactly the pinned .pub inputs")


def run_batch(binary, label, folder, output, records):
    result = subprocess.run([str(binary), "batch-convert", str(folder), "--out", str(output)], capture_output=True, timeout=1800, check=False)
    output.mkdir(parents=True, exist_ok=True)
    (output / "stdout.log").write_bytes(result.stdout)
    (output / "stderr.log").write_bytes(result.stderr)
    check(len(result.stdout) <= LIMIT and len(result.stderr) <= LIMIT, "oversized CLI logs")
    summary = json.loads(result.stdout)
    report = output / "report.csv"
    check(report.stat().st_size <= LIMIT, "oversized report")
    with report.open(newline="", encoding="utf-8") as stream:
        rows = list(csv.DictReader(stream))
    check(len(rows) == len(records) and {r["file"] for r in rows} == {r["file"] for r in records}, "batch did not process every input")
    counts = collections.Counter()
    pages = objects = 0
    for row in rows:
        status = row["status"]
        if status == "failed":
            check(bool(row["failure_reason"]), "failure missing its reason")
            refused = "unsupported Publisher file:" in row["failure_reason"]
            if label == "real":
                expected_refusal = row["file"] in {"Sample98.pub", "Sample2000.pub"}
                check(refused == expected_refusal, "wrong input was refused")
                check(expected_refusal or row["file"].startswith("clusterfuzz-testcase-"), "previously successful real file failed")
            counts["refused" if refused else "failed"] += 1
            continue
        check(status in {"converted OK", "converted with warnings"}, f"unknown status: {status}")
        counts["OK" if status == "converted OK" else "warnings"] += 1
        check((status == "converted with warnings") == bool(row["warnings"]), "warning/status mismatch")
        check(not row["failure_reason"], "successful input has a failure reason")
        check(int(row["pages"]) > 0 and int(row["objects"]) >= 0, "invalid page/object count")
        pages += int(row["pages"])
        objects += int(row["objects"])
        pdf = Path(row["pdf_path"])
        check(pdf.is_relative_to(output) and pdf.is_file(), "missing PDF output")
        with pdf.open("rb") as stream:
            check(stream.read(5) == b"%PDF-", "invalid PDF header")
        check((output / (row["file"] + ".designcraft")).is_file(), "missing native output")
    wanted, expected_pages, expected_objects, expected_exit = EXPECTED[label]
    check(dict(counts) == wanted, f"{label}: unexpected statuses {dict(counts)}")
    check((pages, objects, result.returncode) == (expected_pages, expected_objects, expected_exit), f"{label}: page/object/exit mismatch")
    check(summary["files"] == len(records), "JSON file count mismatch")
    check(summary["converted_ok"] == counts["OK"] and summary["converted_with_warnings"] == counts["warnings"], "JSON status mismatch")
    check(summary["failed"] == counts["failed"] + counts["refused"], "JSON failure count mismatch")
    # Verify all sources again after conversion, including refused and damaged data.
    prepare(folder, records, False)
    return {"files": len(rows), "statuses": dict(counts), "pages": pages, "objects": objects, "exit": result.returncode}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--controlled", type=Path, required=True)
    parser.add_argument("--real", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--download", action="store_true")
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    output = args.out.resolve()
    check(not output.exists(), "choose a fresh output folder")
    with (REPO / ".github/test-data/publisher-fixtures.csv").open(newline="", encoding="utf-8") as stream:
        manifest = list(csv.DictReader(stream))
    summaries = {}
    for label in SOURCES:
        records = [r for r in manifest if r["set"] == label]
        check(len(records) == (142 if label == "controlled" else 24), "manifest count mismatch")
        folder = getattr(args, label).resolve()
        prepare(folder, records, args.download)
        summaries[label] = run_batch(binary, label, folder, output / label, records)
    (output / "summary.json").write_text(json.dumps(summaries, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(summaries))


if __name__ == "__main__":
    main()
