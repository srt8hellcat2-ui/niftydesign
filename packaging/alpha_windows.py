# Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
"""Unsigned alpha staging and headless smoke checks; no build, signing or publishing."""
import argparse
import csv
import hashlib
import shutil
import subprocess
import sys
import tempfile
import tomllib
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
VERSION = '0.12.0-nifty-alpha.3'


def digest(path):
    h = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            h.update(block)
    return h.hexdigest()


def run(args):
    subprocess.run([str(x) for x in args], check=True, timeout=180)


def stage(binary_dir, dest):
    version = tomllib.loads((ROOT / 'Cargo.toml').read_text(encoding='utf-8'))['workspace']['package']['version']
    if version != VERSION:
        raise ValueError(f'expected {VERSION}, got {version}')
    # Refuse reuse rather than shipping stale files from a previous build.
    dest.mkdir(parents=True, exist_ok=False)
    for name in ('designcraft.exe', 'designcraft-cli.exe'):
        source = binary_dir / name
        if not source.is_file() or source.stat().st_size == 0:
            raise ValueError(f'missing or empty executable: {source}')
        shutil.copyfile(source, dest / name)
    # Fonts, icons and About art are embedded in the binaries. No optional external
    # craft-fonts input is used by this workflow. Stage their licences and owner PNGs.
    run([sys.executable, ROOT / 'packaging/check-branding.py', '--copy-legal', dest])
    shutil.copyfile(ROOT / 'packaging/windows/README-ALPHA.txt', dest / 'README-ALPHA.txt')
    manifest = ''.join(f'{digest(p)}  {p.relative_to(dest).as_posix()}\n'
                       for p in sorted(dest.rglob('*')) if p.is_file())
    (dest / 'SHA256SUMS').write_text(manifest, encoding='utf-8', newline='\n')


def archive(dest, output):
    # Check again immediately before zipping, including a complete hash inventory.
    run([sys.executable, ROOT / 'packaging/check-branding.py', dest])
    expected = {}
    for line in (dest / 'SHA256SUMS').read_text(encoding='utf-8').splitlines():
        value, name = line.split('  ', 1)
        if name in expected:
            raise ValueError(f'duplicate manifest entry: {name}')
        expected[name] = value
    actual = {p.relative_to(dest).as_posix(): digest(p) for p in dest.rglob('*')
              if p.is_file() and p.name != 'SHA256SUMS'}
    if actual != expected:
        raise ValueError('staged files differ from SHA256SUMS')
    output.parent.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(output, 'x', compression=zipfile.ZIP_DEFLATED) as result:
        for p in sorted(dest.rglob('*')):
            if p.is_file():
                result.write(p, p.relative_to(dest).as_posix())
    # Hash only after ZipFile.close() has written the final directory/footer.
    output.with_name(output.name + '.sha256').write_text(
        f'{digest(output)}  {output.name}\n', encoding='utf-8', newline='\n')


def smoke(cli, output):
    output.mkdir(parents=True, exist_ok=False)
    with tempfile.TemporaryDirectory(prefix='nifty-alpha-input-') as folder:
        sample = Path(folder) / 'hello-nifty.pub'
        source = ROOT / 'crates/pub/tests/data/hello-nifty.pub'
        shutil.copyfile(source, sample)
        before = digest(sample)
        run([cli, 'batch-convert', sample.parent, '--out', output / 'converted'])
        if digest(sample) != before or digest(source) != before:
            raise ValueError('smoke check modified its source .pub')
    converted = output / 'converted'
    with (converted / 'report.csv').open(newline='', encoding='utf-8') as stream:
        rows = list(csv.DictReader(stream))
    if len(rows) != 1 or rows[0]['failure_reason'] or int(rows[0]['pages']) < 1:
        raise ValueError('hello-nifty batch conversion failed')
    document = converted / 'hello-nifty.pub.designcraft'
    pdf = converted / 'hello-nifty.pub.pdf'
    png = output / 'hello-nifty-page-1.png'
    # CLI page indices are zero based. Reopening the saved document also checks save/reopen.
    run([cli, 'run', '--in', document, '--page', '0', '--export', png])
    for p in (document, pdf, png):
        if not p.is_file() or p.stat().st_size == 0:
            raise ValueError(f'missing or empty smoke output: {p}')
    if not png.read_bytes().startswith(b'\x89PNG\r\n\x1a\n') or not pdf.read_bytes().startswith(b'%PDF-'):
        raise ValueError('smoke output has an unexpected format')
    shutil.copyfile(pdf, output / 'hello-nifty.pdf')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest='command', required=True)
    p = sub.add_parser('stage')
    p.add_argument('binary_dir', type=Path)
    p.add_argument('destination', type=Path)
    p = sub.add_parser('zip')
    p.add_argument('destination', type=Path)
    p.add_argument('output', type=Path)
    p = sub.add_parser('smoke')
    p.add_argument('cli', type=Path)
    p.add_argument('output', type=Path)
    args = parser.parse_args()
    if args.command == 'stage':
        stage(args.binary_dir, args.destination)
    elif args.command == 'zip':
        archive(args.destination, args.output)
    else:
        smoke(args.cli.resolve(), args.output.resolve())


if __name__ == '__main__':
    main()
