#!/usr/bin/env python3
# Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
"""Stage required legal files and reject upstream marks in an unpacked package.

Usage: python3 packaging/check-branding.py [--copy-legal] PACKAGE_DIRECTORY
Run on the staging directory before archiving/signing; never on the source tree.
"""
import argparse
import hashlib
import shutil
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
REQUIRED = [Path(p) for p in (
    'NOTICE', 'LICENSE-MIT', 'LICENSE-APACHE', 'ASSETS.md',
    'docs/brand/LICENSE-brand.txt', 'assets/app-icon/LICENSE.txt',
    'assets/nifty/nifty-wordmark.png', 'assets/nifty/nifty-mark.png',
)] + sorted(p.relative_to(ROOT) for p in (ROOT / 'assets/fonts').glob('OFL*.txt'))
REQUIRED += sorted(p.relative_to(ROOT) for p in (ROOT / 'licenses/pdf-backend').rglob('*') if p.is_file())


# Fingerprints only: the trademark artwork is deliberately excluded from this tree.
KNOWN_MARKS = {'artcraft-logo-white.png': (32161, '8b86aeb06a12a90ba77a60e0d90329272310e244ee5c1f9b161b391280ca79d8'),
 'artcraft-logo-white.svg': (5394, 'a3e03dc49f9ad4225b8517d5424309c8d1c864d3996d862190d9cc797d5af427'),
 'artcraft-logo.png': (31302, '08fc6d9d3717e8f13417bac6f9025b50e11d1e0b32e9e000883d2fe0c4a0c67b'),
 'artcraft-logo.svg': (5394, '3188971e1affdf4f6be4fa65f715376a7a305e6aa741ec78b0296c8f934eff08'),
 'artcraft-mark-black.png': (26793, '5954cc0306e1b9d7beb47097e2e842157113a39d9aba078b57304902687936a9'),
 'artcraft-mark-black.svg': (341, '570adcb4f7ba8ca0647f97074476bab81d3272988f1cdee52efa21bf026036a4'),
 'artcraft-mark.png': (24715, '9d372157a949c147318561e629b11561c28b7cbc14c84b516a3fe6e84d8fbbad'),
 'artcraft-mark.svg': (341, 'f87af81b6fa411da8820eed2bfdcc54c0ee9cd453b4e868c64f56c69db142d0e')}


def stage_legal(dest):
    for path in REQUIRED:
        target = dest / path
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(ROOT / path, target)


def check(dest):
    errors = []
    marks = [p for p in (ROOT / 'docs/brand').rglob('*') if p.is_file() and p.name != 'LICENSE-brand.txt']
    names = set(KNOWN_MARKS) | {p.name.casefold() for p in marks}
    hashes = {bytes.fromhex(h) for _, h in KNOWN_MARKS.values()} | {hashlib.sha256(p.read_bytes()).digest() for p in marks}
    sizes = {size for size, _ in KNOWN_MARKS.values()} | {p.stat().st_size for p in marks}
    for p in dest.rglob('*'):
        if not p.is_file():
            continue
        rel = p.relative_to(dest)
        if p.name.casefold() in names or ('docs/brand' in rel.as_posix().casefold() and p.name != 'LICENSE-brand.txt'):
            errors.append(f'upstream brand file: {rel}')
        elif p.stat().st_size in sizes and hashlib.sha256(p.read_bytes()).digest() in hashes:
            errors.append(f'upstream mark bytes (possibly renamed): {rel}')
    for path in REQUIRED:
        if not (dest / path).is_file() or (dest / path).read_bytes() != (ROOT / path).read_bytes():
            errors.append(f'missing or changed legal file: {path}')
    return errors


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--copy-legal', action='store_true')
    parser.add_argument('package', type=Path)
    args = parser.parse_args()
    if args.package.resolve() == ROOT:
        parser.error('use a package staging directory, not the source tree')
    if args.copy_legal:
        stage_legal(args.package)
    errors = check(args.package)
    if errors:
        parser.exit(1, '\n'.join(errors) + '\n')
    print('Package branding/legal check passed.')


if __name__ == '__main__':
    main()
