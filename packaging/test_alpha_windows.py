# Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
"""Package contents and tamper detection; tiny stand-ins are not Windows builds."""
import hashlib
import subprocess
import tempfile
import unittest
import zipfile
from pathlib import Path

import alpha_windows as alpha


class AlphaPackage(unittest.TestCase):
    def test_zip_contains_every_required_file_and_matching_hashes(self):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            binaries = root / 'bin'
            binaries.mkdir()
            for name in ('designcraft.exe', 'designcraft-cli.exe'):
                (binaries / name).write_bytes(b'synthetic executable stand-in')
            stage = root / 'stage'
            alpha.stage(binaries, stage)
            output = root / 'alpha.zip'
            alpha.archive(stage, output)
            # The sidecar can verify the finished archive before any extraction.
            sidecar = output.with_name(output.name + '.sha256')
            checksum, filename = sidecar.read_text().strip().split('  ', 1)
            self.assertEqual(filename, output.name)
            self.assertEqual(checksum, hashlib.sha256(output.read_bytes()).hexdigest())
            with zipfile.ZipFile(output) as z:
                files = set(z.namelist())
                self.assertTrue({'designcraft.exe', 'designcraft-cli.exe', 'README-ALPHA.txt',
                                 'SHA256SUMS', 'LICENSE-MIT', 'LICENSE-APACHE', 'NOTICE',
                                 'ASSETS.md', 'assets/app-icon/LICENSE.txt',
                                 'assets/nifty/nifty-mark.png', 'assets/nifty/nifty-wordmark.png',
                                 'docs/brand/LICENSE-brand.txt'} <= files)
                self.assertEqual({n for n in files if n.startswith('docs/brand/')},
                                 {'docs/brand/LICENSE-brand.txt'})
                self.assertEqual(len([n for n in files if n.startswith('assets/fonts/OFL')]), 4)
                hashes = dict(line.split('  ', 1)[::-1] for line in z.read('SHA256SUMS').decode().splitlines())
                self.assertEqual(set(hashes), files - {'SHA256SUMS'})
                for name, digest in hashes.items():
                    self.assertEqual(hashlib.sha256(z.read(name)).hexdigest(), digest)
                readme = z.read('README-ALPHA.txt').decode()
                for text in [alpha.VERSION, 'REPO_URL_HERE', 'UNVERIFIED', 'SmartScreen',
                             '.zip.sha256', 'before extracting',
                             '19 opened with warnings', 'numbered lists', 'custom tab stops',
                             'cell', 'WordArt', 'WMF/EMF pictures', '97-2000', 'personal information']:
                    self.assertIn(text, readme)
            (stage / 'README-ALPHA.txt').write_text('modified')
            with self.assertRaisesRegex(ValueError, 'SHA256SUMS'):
                alpha.archive(stage, root / 'tampered.zip')
            self.assertFalse((root / 'tampered.zip').exists())
            (stage / 'docs/brand/unexpected.bin').write_bytes(b'not permitted')
            with self.assertRaises(subprocess.CalledProcessError):
                alpha.archive(stage, root / 'bad-brand.zip')
            self.assertFalse((root / 'bad-brand.zip').exists())

    def test_missing_executables_are_rejected(self):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            with self.assertRaisesRegex(ValueError, 'executable'):
                alpha.stage(root, root / 'stage')


if __name__ == '__main__':
    unittest.main()
