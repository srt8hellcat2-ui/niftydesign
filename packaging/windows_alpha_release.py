#!/usr/bin/env python3
# Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
"""Prepare unsigned alpha assets locally; this script never creates a release."""
import argparse
from pathlib import Path
import re
import shutil
import tomllib
import zipfile

import alpha_windows as alpha


# These tokens are prohibited in distributed text and executable strings, including
# UTF-16 Windows resource strings. Historical source records and crate names are
# not package inputs.
FORBIDDEN_STRINGS = ('designcraft-pub', 'REPO_URL_HERE', 'OWNER TO FILL', 'owner must fill',
                     'older Microsoft Publisher')


def reject_forbidden_strings(stream, label):
    patterns = [(text, text.lower().encode(encoding))
                for text in FORBIDDEN_STRINGS for encoding in ('utf-8', 'utf-16-le', 'utf-16-be')]
    overlap = max(len(pattern) for _, pattern in patterns) - 1
    tail = b''
    while chunk := stream.read(1024 * 1024):
        data = (tail + chunk).lower()
        # Line wrapping in packaged text must not hide a forbidden phrase.
        normalized = re.sub(rb'\s+', b' ', data)
        for text, pattern in patterns:
            if pattern in data or pattern in normalized:
                raise ValueError(f'forbidden package string {text!r} in {label}')
        tail = data[-overlap:]


def check_archive_strings(path):
    with zipfile.ZipFile(path) as archive:
        for member in archive.infolist():
            if not member.is_dir():
                with archive.open(member) as stream:
                    reject_forbidden_strings(stream, member.filename)


def validate_version(version):
    current = tomllib.loads((alpha.ROOT / 'Cargo.toml').read_text(encoding='utf-8'))['workspace']['package']['version']
    if not re.fullmatch(r'[0-9]+\.[0-9]+\.[0-9]+-nifty-alpha\.[0-9]+', version):
        raise ValueError('version must be a NiftyDesign alpha version, e.g. 0.12.0-nifty-alpha.4')
    if version != current or version != alpha.VERSION:
        raise ValueError(f'version must match Cargo.toml and the alpha packager ({current})')
    return f'NiftyDesign-{version}-windows-x64.zip'


def package(version, binaries, work, upload):
    name = validate_version(version)
    if upload.exists():
        raise ValueError('upload folder must be fresh')
    work.mkdir(parents=True, exist_ok=False)
    stage = work / 'package'
    alpha.stage(binaries, stage)
    # Only the staged copy changes: leave the original alpha workflow/README intact.
    readme = stage / 'README-ALPHA.txt'
    readme.write_text(readme.read_text(encoding='utf-8').replace(
        f'niftydesign-{version}-windows-x64.zip', name), encoding='utf-8', newline='\n')
    (stage / 'SHA256SUMS').write_text(''.join(
        f'{alpha.digest(p)}  {p.relative_to(stage).as_posix()}\n'
        for p in sorted(stage.rglob('*')) if p.is_file() and p.name != 'SHA256SUMS'),
        encoding='utf-8', newline='\n')
    archive = work / name
    alpha.archive(stage, archive)  # checks branding/inventory, closes ZIP, writes sidecar
    try:
        check_archive_strings(archive)
    except ValueError:
        archive.unlink()
        archive.with_name(name + '.sha256').unlink()
        raise
    upload.mkdir(parents=True, exist_ok=False)
    for source in (archive, archive.with_name(name + '.sha256')):
        target = upload / source.name
        shutil.copyfile(source, target)
        if target.stat().st_size == 0 or alpha.digest(target) != alpha.digest(source):
            raise ValueError(f'upload copy differs: {target.name}')
    print(f'Prepared {name} and its checksum in fresh upload folder {upload}', flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest='command', required=True)
    p = sub.add_parser('validate')
    p.add_argument('version')
    p = sub.add_parser('package')
    p.add_argument('version')
    p.add_argument('binaries', type=Path)
    p.add_argument('work', type=Path)
    p.add_argument('upload', type=Path)
    args = parser.parse_args()
    if args.command == 'validate':
        print(validate_version(args.version))
    else:
        package(args.version, args.binaries, args.work, args.upload)


if __name__ == '__main__':
    main()
