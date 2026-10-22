# Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
import copy
import importlib.util
import json
import os
from pathlib import Path
import socket
import tempfile
import threading
import unittest

SPEC = importlib.util.spec_from_file_location('desktop_demo', Path(__file__).with_name('windows_desktop_demo.py'))
demo = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(demo)


class DemoChecks(unittest.TestCase):
    def setUp(self):
        self.expected = demo.read_json(demo.ROOT / 'docs/demo/coffee-shop/engine-values.json')
        self.state = {'spreads': [], 'stories': copy.deepcopy(self.expected['stories'])}
        for index, size in enumerate(self.expected['page_sizes_pt']):
            items = [{k: v for k, v in x.items() if k != 'spread'} for x in self.expected['objects'] if x['spread'] == index]
            self.state['spreads'].append({'index': index, 'items': items, 'pages': [{'bounds': [0, 0, *size]}]})

    def test_comparison_accepts_reference_and_tolerance(self):
        self.state['spreads'][0]['items'][0]['bounds'][0] += 0.01
        self.assertTrue(all(x['pass'] for x in demo.compare_state(self.state, self.expected)))

    def test_changed_bounds_text_and_missing_image_fail(self):
        self.state['spreads'][0]['items'][0]['bounds'][0] += 0.1
        self.state['stories'][0]['preview'] = 'wrong text'
        self.state['spreads'][1]['items'].pop()
        failed = [x['check'] for x in demo.compare_state(self.state, self.expected) if not x['pass']]
        self.assertIn('object count', failed)
        self.assertIn('stories (preview, length, frames, composition)', failed)
        self.assertTrue(any('bounds' in x for x in failed))
        self.assertFalse(demo.near(float('nan'), float('nan')))

    def test_control_preserves_exact_request_and_error(self):
        with tempfile.TemporaryDirectory() as temp, socket.socket() as listener:
            listener.bind(('127.0.0.1', 0)); listener.listen()
            def server():
                conn, _ = listener.accept()
                with conn, conn.makefile('rb') as f:
                    request = json.loads(f.readline())
                    conn.sendall((json.dumps({'id': request['id'], 'ok': False, 'error': 'synthetic refusal'}) + '\n').encode())
            thread = threading.Thread(target=server); thread.start()
            transcript = Path(temp) / 'transcript.jsonl'
            with self.assertRaisesRegex(RuntimeError, 'synthetic refusal'):
                demo.Control(listener.getsockname()[1], transcript).engine('file.print', {'dryRun': True})
            thread.join(timeout=5)
            self.assertFalse(thread.is_alive())
            row = json.loads(transcript.read_text())
            self.assertEqual(row['request']['params']['params'], {'dryRun': True})
            self.assertEqual(row['response']['error'], 'synthetic refusal')

    def test_workflow_is_manual_read_only_one_artifact(self):
        text = (demo.ROOT / '.github/workflows/windows-desktop-demo.yml').read_text()
        self.assertIn('workflow_dispatch:', text)
        self.assertNotIn('pull_request:', text)
        self.assertNotIn('secrets.', text)
        self.assertIn('contents: read', text)
        self.assertIn('timeout-minutes: 120', text)
        self.assertIn('cargo +1.95.0 build --locked --release', text)
        self.assertEqual(text.count('uses: actions/upload-artifact@'), 1)
        self.assertIn('if: always()', text)

    def test_print_is_dry_first_and_only_allowlisted_virtual_queue(self):
        class FakeControl:
            def __init__(self, names): self.calls, self.names = [], names
            def engine(self, name, params=None, **kwargs):
                self.calls.append((name, copy.deepcopy(params)))
                return {'ok': True, 'result': {'printers': self.names} if name == 'file.printers' else {}}
        from unittest.mock import patch
        with tempfile.TemporaryDirectory() as temp:
            out = Path(temp)
            (out / 'virtual-printer.pdf').write_bytes(b'%PDF-synthetic')
            with patch.object(demo, 'powershell', return_value={'exit': 0, 'stdout': '[]', 'stderr': ''}):
                control = FakeControl(['Office physical printer'])
                demo.print_demo(control, out, {})
                self.assertEqual(len(control.calls), 2)
                self.assertTrue(control.calls[1][1]['dryRun'])
                control = FakeControl(['Microsoft Print to PDF', 'Office physical printer'])
                demo.print_demo(control, out, {})
                self.assertEqual(len(control.calls), 3)
                self.assertTrue(control.calls[1][1]['dryRun'])
                self.assertFalse(control.calls[2][1]['dryRun'])
                self.assertEqual(control.calls[2][1]['printer'], 'Microsoft Print to PDF')
                self.assertFalse(control.calls[2][1]['systemDialog'])
                (out / 'virtual-printer.xps').write_bytes(b'synthetic-XPS')
                control = FakeControl(['Microsoft XPS Document Writer'])
                demo.print_demo(control, out, {})
                self.assertEqual(len(control.calls), 3)
                self.assertEqual(control.calls[2][1]['printer'], 'Microsoft XPS Document Writer')

    def test_unsupported_host_reports_failure_not_desktop_success(self):
        from unittest.mock import patch
        with tempfile.TemporaryDirectory() as temp:
            out = Path(temp) / 'evidence'
            argv = ['demo', '--app', str(Path(temp) / 'designcraft.exe'), '--sample-dir', str(Path(temp) / 'samples'), '--out', str(out)]
            with patch.object(demo.sys, 'platform', 'linux'), patch.object(demo.sys, 'argv', argv):
                self.assertEqual(demo.main(), 1)
            self.assertIn('FAILED / INCOMPLETE', (out / 'REPORT.md').read_text())
            self.assertFalse(demo.read_json(out / 'facts.json')['demo completed'])
            self.assertFalse((Path(temp) / 'samples').exists())

    def test_failed_startup_attempts_documented_backends_and_stops(self):
        from unittest.mock import patch
        class Exited:
            pid = 123
            def poll(self): return 42
            def wait(self, **kwargs): return 42
        requested = []
        def launch(*args, **kwargs):
            requested.append(kwargs['env'].get('WGPU_BACKEND'))
            return Exited()
        with tempfile.TemporaryDirectory() as temp, patch.object(demo.subprocess, 'Popen', side_effect=launch):
            attempts = []
            with self.assertRaisesRegex(RuntimeError, 'No usable graphics adapter/window'):
                demo.start(Path('designcraft.exe'), Path(temp), attempts)
            self.assertEqual(requested, [None, 'dx12', 'gl'])
            self.assertEqual(len(attempts), 3)
            self.assertTrue(all(not a['ready'] and 'failure' in a for a in attempts))

    @unittest.skipUnless(os.environ.get('NIFTY_SAMPLE2'), 'opt-in pinned external Sample2 plan check')
    def test_reused_linux_plan_and_reference_commands(self):
        mod = demo.load_module('plan_test', demo.ROOT / 'docs/demo/coffee-shop/reproduce.py')
        old_cwd = Path.cwd()
        try:
            with tempfile.TemporaryDirectory() as temp:
                mod.OUT = Path(temp); mod.WORK = Path(temp) / 'engine'
                rows = mod.build_plan(Path(os.environ['NIFTY_SAMPLE2']))
                self.assertEqual(sum('shot' in r for r in rows), 9)
                commands = [r['params']['command'] for r in rows if r.get('method') == 'engine.execute']
                self.assertEqual(commands[0], 'file.open')
                self.assertEqual(commands[-1], 'file.exportPdf')
                self.assertEqual(commands.count('file.open'), 2)
                created_texts = [r['params']['params']['text'] for r in rows
                                 if r.get('params', {}).get('command') == 'frame.create'
                                 and 'text' in r['params']['params']]
                self.assertIn('Fresh Today', created_texts)
                self.assertIn('alex@example.com\n555-0100', created_texts)
        finally:
            os.chdir(old_cwd)


if __name__ == '__main__':
    unittest.main()
