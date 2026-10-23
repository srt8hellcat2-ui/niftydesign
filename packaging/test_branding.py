# Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
"""Run with python3 -m unittest discover -s packaging -p test_branding.py."""
import hashlib
import importlib.util
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

spec = importlib.util.spec_from_file_location('branding', Path(__file__).with_name('check-branding.py'))
branding = importlib.util.module_from_spec(spec)
spec.loader.exec_module(branding)


class PackageBranding(unittest.TestCase):
    def test_pdf_backend_licenses_are_staged_and_required(self):
        sources = sorted(p for p in (branding.ROOT / 'licenses/pdf-backend').rglob('*') if p.is_file())
        self.assertTrue(sources)
        with tempfile.TemporaryDirectory() as folder:
            dest = Path(folder)
            branding.stage_legal(dest)
            for source in sources:
                path = source.relative_to(branding.ROOT)
                self.assertEqual((dest / path).read_bytes(), source.read_bytes())
                (dest / path).unlink()
                self.assertIn(f'missing or changed legal file: {path}', branding.check(dest))
                (dest / path).write_bytes(source.read_bytes())
            self.assertEqual(branding.check(dest), [])

    def test_legal_files_kept_but_marks_rejected_even_renamed(self):
        with tempfile.TemporaryDirectory() as d:
            dest = Path(d)
            branding.stage_legal(dest)
            self.assertEqual(branding.check(dest), [])
            # The amber ledger 3541 check uses synthetic bytes, not upstream artwork.
            mark = b'synthetic branding-test bytes, not a logo'
            self.assertIn('artcraft-mark.png', branding.KNOWN_MARKS)
            with patch.dict(branding.KNOWN_MARKS, {'synthetic-mark.png': (len(mark), hashlib.sha256(mark).hexdigest())}):
                self.check_marks(dest, mark)
            (dest / 'NOTICE').unlink()
            self.assertIn('missing or changed legal file: NOTICE', branding.check(dest))

    def check_marks(self, dest, mark):
        for name in ['artcraft-mark.png', 'renamed.png', 'docs/brand/unexpected.bin']:
            target = dest / name
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(mark)
            self.assertTrue(branding.check(dest), name)
            target.unlink()


if __name__ == '__main__':
    unittest.main()
