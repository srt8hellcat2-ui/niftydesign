# Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
"""Draft-only workflow and exact release asset contract; no GitHub calls."""
import hashlib
import io
from pathlib import Path
import tempfile
import unittest
import zipfile

import windows_alpha_release as release


class AlphaRelease(unittest.TestCase):
    def test_rejects_mismatched_or_unsafe_version_before_creating_files(self):
        for version in ('1.0.0', '0.9.0-nifty-alpha.2', '../bad', '0.9.0-nifty-alpha.1;whoami'):
            with self.assertRaises(ValueError): release.validate_version(version)
        self.assertEqual(release.validate_version(release.alpha.VERSION),
                         'NiftyDesign-0.9.0-nifty-alpha.1-windows-x64.zip')

    def test_copies_complete_zip_and_checksum_into_fresh_upload_directory(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); binaries = root / 'bin'; binaries.mkdir()
            for name in ('designcraft.exe', 'designcraft-cli.exe'):
                (binaries / name).write_bytes(b'synthetic executable stand-in')
            upload = root / 'upload'
            release.package(release.alpha.VERSION, binaries, root / 'work', upload)
            name = release.validate_version(release.alpha.VERSION)
            self.assertEqual({p.name for p in upload.iterdir()}, {name, name + '.sha256'})
            zip_path = upload / name
            self.assertEqual((upload / (name + '.sha256')).read_text(),
                             f'{hashlib.sha256(zip_path.read_bytes()).hexdigest()}  {name}\n')
            with zipfile.ZipFile(zip_path) as z:
                self.assertTrue({'designcraft.exe', 'README-ALPHA.txt', 'LICENSE-MIT', 'LICENSE-APACHE',
                                 'NOTICE', 'SHA256SUMS'} <= set(z.namelist()))
                sums = dict(line.split('  ', 1)[::-1] for line in z.read('SHA256SUMS').decode().splitlines())
                self.assertEqual(set(sums), set(z.namelist()) - {'SHA256SUMS'})
                for file, digest in sums.items(): self.assertEqual(hashlib.sha256(z.read(file)).hexdigest(), digest)
                self.assertIn(name + '.sha256', z.read('README-ALPHA.txt').decode())
            # Uploads use copies; changing the working zip cannot change uploaded bytes.
            original = zip_path.read_bytes()
            (root / 'work' / name).write_bytes(b'changed')
            self.assertEqual(zip_path.read_bytes(), original)
            with self.assertRaisesRegex(ValueError, 'fresh'):
                release.package(release.alpha.VERSION, binaries, root / 'other-work', upload)
            self.assertFalse((root / 'other-work').exists())

    def test_package_rejects_private_repo_and_placeholders_in_text_and_executables(self):
        for token in release.FORBIDDEN_STRINGS:
            for encoding in ('utf-8', 'utf-16-le', 'utf-16-be'):
                for name in ('README-ALPHA.txt', 'designcraft.exe', 'designcraft-cli.exe'):
                    with self.subTest(token=token, encoding=encoding, name=name):
                        with tempfile.TemporaryDirectory() as temp:
                            path = Path(temp) / 'bad.zip'
                            with zipfile.ZipFile(path, 'w') as archive:
                                archive.writestr(name, b'prefix' + token.upper().encode(encoding) + b'suffix')
                            with self.assertRaisesRegex(ValueError, 'forbidden package string'):
                                release.check_archive_strings(path)
        # A token crossing the read boundary is also rejected.
        with self.assertRaises(ValueError):
            release.reject_forbidden_strings(io.BytesIO(b'x' * (1024 * 1024 - 3) +
                                                        release.FORBIDDEN_STRINGS[0].encode()), 'boundary.exe')
        release.reject_forbidden_strings(io.BytesIO(b'https://github.com/srt8hellcat2-ui/niftydesign'), 'good.txt')

    def test_bad_executable_never_reaches_upload(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); binaries = root / 'bin'; binaries.mkdir()
            for name in ('designcraft.exe', 'designcraft-cli.exe'):
                (binaries / name).write_bytes(b'https://github.com/srt8hellcat2-ui/' +
                                            release.FORBIDDEN_STRINGS[0].encode())
            with self.assertRaisesRegex(ValueError, 'forbidden package string'):
                release.package(release.alpha.VERSION, binaries, root / 'work', root / 'upload')
            self.assertFalse((root / 'upload').exists())
            self.assertFalse(list((root / 'work').glob('*.zip*')))

    def test_workflow_is_manual_draft_only_and_write_is_release_scoped(self):
        text = (release.alpha.ROOT / '.github/workflows/windows-alpha-release.yml').read_text()
        self.assertIn('workflow_dispatch:', text)
        self.assertNotIn('  push:', text); self.assertNotIn('pull_request:', text)
        self.assertIn('type: boolean', text); self.assertIn('default: true', text)
        self.assertIn('needs: build', text); self.assertIn('if: ${{ inputs.draft_release }}', text)
        before, job = text.split('\n  release:\n', 1)
        self.assertIn('contents: read', before); self.assertNotIn('contents: write', before)
        self.assertIn('contents: write', job)
        self.assertIn('--draft --prerelease --target "$env:RELEASE_SHA"', job)
        self.assertIn('--notes-file docs/RELEASE-NOTES.md', job)
        self.assertIn('GH_TOKEN: ${{ github.token }}', job)
        self.assertNotIn('secrets.', text)
        self.assertEqual(text.count('runs-on: windows-latest'), 2)
        self.assertLess(before.index('Open controlled sample'), before.index('Stop and verify'))
        self.assertLess(before.index('Stop and verify'), before.index('Package and copy'))
        self.assertIn('path: ${{ runner.temp }}/alpha-release-upload', text)
        self.assertIn('cargo +1.95.0 build --locked --release', text)
        self.assertFalse((release.alpha.ROOT / '.github/workflows/freebsd.yml').exists())
        self.assertFalse((release.alpha.ROOT / '.github/workflows/windows-arm64.yml').exists())


if __name__ == '__main__':
    unittest.main()
