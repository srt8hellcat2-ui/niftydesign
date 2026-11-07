# Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
import copy
import contextlib
import io
import shutil
import subprocess
import sys
import time
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

    def test_workflow_is_manual_read_only_independent_artifacts_and_cache(self):
        text = (demo.ROOT / '.github/workflows/windows-desktop-demo.yml').read_text()
        self.assertIn('workflow_dispatch:', text)
        self.assertNotIn('pull_request:', text)
        self.assertNotIn('secrets.', text)
        self.assertIn('contents: read', text)
        self.assertIn('timeout-minutes: 120', text)
        self.assertIn('cargo +1.95.0 build --locked --release', text)
        self.assertEqual(text.count('uses: actions/upload-artifact@'), 2)
        self.assertLess(text.index('name: windows-desktop-report'), text.index('name: windows-desktop-shots'))
        for part in text.split('      - name: ')[1:]:
            if 'actions/upload-artifact@' in part or 'actions/cache/save@' in part:
                self.assertIn('if: always()', part)
                self.assertIn('continue-on-error: true', part)
        self.assertIn('actions/cache/restore@v4', text)
        self.assertIn('actions/cache/save@v4', text)
        for value in ('runner.os', 'RUST_VERSION', "hashFiles('Cargo.lock')", '~/.cargo/registry', '~/.cargo/git', '            target'):
            self.assertIn(value, text)
        self.assertLess(text.index('Save Cargo build cache'), text.index('Run desktop demo without mouse input'))
        self.assertIn('stage_windows_desktop_evidence.ps1', text)
        self.assertIn("@('REPORT.md', 'comparison.csv')", text)
        self.assertIn('PYTHONUNBUFFERED: 1', text)
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

    @staticmethod
    def no_desktop():
        return {'session_name': 'Services', 'session_id': 0, 'window_station': 'Service-0',
                'desktop_name': None, 'primary_screen_size': [0, 0], 'foreground_window': False,
                'user_name': 'synthetic-user', 'primary_screenshot_possible': False,
                'errors': ['synthetic CopyFromScreen failure']}

    def test_unsupported_host_reports_failure_not_desktop_success(self):
        from unittest.mock import patch
        with tempfile.TemporaryDirectory() as temp:
            out = Path(temp) / 'evidence'
            argv = ['demo', '--app', str(Path(temp) / 'designcraft.exe'), '--sample-dir', str(Path(temp) / 'samples'), '--out', str(out)]
            with patch.object(demo, 'collect_desktop_diagnostics', return_value=self.no_desktop()), \
                 patch.dict(os.environ, {'GITHUB_ACTIONS': 'false'}), patch.object(demo.sys, 'argv', argv), \
                 contextlib.redirect_stdout(io.StringIO()):
                self.assertEqual(demo.main(), 1)
            self.assertIn('FAILED / INCOMPLETE', (out / 'REPORT.md').read_text())
            self.assertFalse(demo.read_json(out / 'facts.json')['demo completed'])
            self.assertFalse((Path(temp) / 'samples').exists())

    def test_failed_startup_attempts_documented_backends_and_stops(self):
        from unittest.mock import patch
        class Exited:
            pid, returncode = 123, 42
            def poll(self): return 42
            def wait(self, **kwargs): return 42
        requested = []
        def launch(*args, **kwargs):
            requested.append(kwargs['env'].get('WGPU_BACKEND'))
            return Exited()
        with tempfile.TemporaryDirectory() as temp, patch.object(demo.subprocess, 'Popen', side_effect=launch), \
             patch.object(demo, 'WindowsJob', return_value=None):
            attempts = []
            with self.assertRaisesRegex(RuntimeError, 'No usable graphics adapter/window'):
                demo.start(Path('designcraft.exe'), Path(temp), attempts)
            self.assertEqual(requested, [None, 'dx12', 'gl'])
            self.assertEqual(len(attempts), 3)
            self.assertTrue(all(not a['ready'] and 'failure' in a for a in attempts))


    def test_owned_parent_is_waited_and_log_closed(self):
        with tempfile.TemporaryDirectory() as temp:
            log = (Path(temp) / 'app.log').open('wb')
            process = subprocess.Popen([sys.executable, '-c', 'import time; time.sleep(60)'], stdout=log, stderr=log)
            owned = demo.OwnedProcess(process, log)
            try:
                result = owned.stop()
                self.assertEqual(result['remaining'], 0)
                self.assertIsNotNone(process.poll())
                self.assertTrue(log.closed)
                self.assertTrue(owned.stop()['already_stopped'])
            finally:
                if process.poll() is None: process.kill(); process.wait()
                log.close()

    def test_job_kills_descendants_even_after_parent_exit_and_verifies_zero(self):
        from unittest.mock import Mock, patch
        process = Mock(pid=17, returncode=0); process.poll.return_value = 0
        job, log = Mock(), Mock()
        job.active.side_effect = [2, 1, 0]
        with patch.object(demo.time, 'sleep'):
            self.assertEqual(demo.OwnedProcess(process, log, job).stop()['remaining'], 0)
        job.terminate.assert_called_once()
        process.wait.assert_called_once_with(timeout=15)
        self.assertEqual(job.active.call_count, 3)
        job.close.assert_called_once(); log.close.assert_called_once()
        process.terminate.assert_not_called()

    def test_unverified_cleanup_is_reported_and_still_closes_handles(self):
        from unittest.mock import Mock, patch
        process = Mock(pid=17, returncode=0); process.poll.return_value = 0
        job, log = Mock(), Mock(); job.active.return_value = 1
        owned = demo.OwnedProcess(process, log, job)
        with patch.object(demo, 'APP_PROCESSES', [owned]), patch.object(demo.time, 'monotonic', side_effect=[0, 16]):
            results = demo.cleanup_apps()
        self.assertEqual(results[0]['remaining'], 'UNVERIFIED')
        self.assertIn('active processes', results[0]['error'])
        job.close.assert_called_once(); log.close.assert_called_once()
        self.assertFalse(owned.stopped)

    def test_timeout_escalates_and_waits_before_closing_log(self):
        from unittest.mock import Mock
        process = Mock(pid=17, returncode=1); process.poll.side_effect = [None, 1]
        process.wait.side_effect = [subprocess.TimeoutExpired('app', 15), 1]
        log = Mock()
        result = demo.OwnedProcess(process, log).stop()
        process.terminate.assert_called_once(); process.kill.assert_called_once()
        self.assertEqual(process.wait.call_count, 2)
        self.assertEqual(result['remaining'], 0); log.close.assert_called_once()

    def test_startup_attaches_suspended_process_before_resuming_and_cleans_failed_attempts(self):
        from unittest.mock import Mock, patch
        order, logs = [], []
        class Process:
            pid, returncode = 123, None
            def poll(self): return self.returncode
            def wait(self, **kwargs): self.returncode = 1; return 1
        class Job:
            def attach(self, p): order.append('attach')
            def resume(self, p): order.append('resume')
            def terminate(self): order.append('terminate')
            def active(self): return 0
            def close(self): order.append('close')
        def launch(*args, **kwargs):
            self.assertEqual(kwargs['creationflags'], 4)
            logs.append(kwargs['stdout']); order.append('spawn'); return Process()
        def capture(*args):
            order.append('capture'); raise RuntimeError('synthetic capture failure')
        with tempfile.TemporaryDirectory() as temp, patch.object(demo.sys, 'platform', 'win32'), \
             patch.object(demo, 'WindowsJob', Job), patch.object(demo.subprocess, 'Popen', side_effect=launch), \
             patch.object(demo, 'Control', return_value=Mock()), patch.object(demo, 'capture', side_effect=capture), \
             patch.object(demo, 'APP_PROCESSES', []):
            with self.assertRaisesRegex(RuntimeError, 'No usable graphics'):
                demo.start(Path('app.exe'), Path(temp), [])
            self.assertEqual(demo.APP_PROCESSES, [])
        self.assertEqual(order, ['spawn', 'attach', 'resume', 'capture', 'terminate', 'close'] * 3)
        self.assertTrue(all(log.closed for log in logs))

    def test_main_failure_runs_cleanup_and_prints_failure_summary(self):
        from unittest.mock import Mock, patch
        owned = Mock(); owned.stop.return_value = {'remaining': 0, 'pid': 17}
        with tempfile.TemporaryDirectory() as temp:
            args = ['demo', '--app', 'unused', '--sample-dir', temp, '--out', str(Path(temp) / 'evidence')]
            console = io.StringIO()
            with patch.object(demo, 'collect_desktop_diagnostics', return_value=self.no_desktop()), \
                 patch.dict(os.environ, {'GITHUB_ACTIONS': 'false'}), patch.object(demo.sys, 'argv', args), \
                 patch.object(demo, 'APP_PROCESSES', [owned]), contextlib.redirect_stdout(console):
                self.assertEqual(demo.main(), 1)
                self.assertEqual(demo.APP_PROCESSES, [])
            owned.stop.assert_called_once()
            self.assertIn('FAIL desktop preflight', console.getvalue())
            self.assertIn('process cleanup', console.getvalue())
            self.assertIn('NOT REACHED', console.getvalue())

    def test_report_write_error_still_cleans_processes_and_prints_summary(self):
        from unittest.mock import Mock, patch
        owned = Mock(); owned.stop.return_value = {'remaining': 0, 'pid': 17}
        with tempfile.TemporaryDirectory() as temp:
            args = ['demo', '--app', 'unused', '--sample-dir', temp, '--out', str(Path(temp) / 'evidence')]
            console = io.StringIO()
            with patch.object(demo, 'collect_desktop_diagnostics', return_value=self.no_desktop()), \
                 patch.dict(os.environ, {'GITHUB_ACTIONS': 'false'}), patch.object(demo.sys, 'argv', args), \
                 patch.object(demo, 'APP_PROCESSES', [owned]), \
                 patch.object(demo.csv, 'DictWriter', side_effect=OSError('synthetic evidence lock')), \
                 contextlib.redirect_stdout(console):
                self.assertEqual(demo.main(), 1)
            owned.stop.assert_called_once()
            self.assertIn('Evidence finalization failed', console.getvalue())
            self.assertIn('synthetic evidence lock', console.getvalue())
            self.assertIn('Windows desktop demo: FAIL / INCOMPLETE', console.getvalue())

    def test_console_summary_has_graphics_fonts_printers_and_comparison_result(self):
        console = io.StringIO()
        facts = {k: {'exact': k} for k in ['graphics exact log records', 'system fonts', 'Get-Printer',
                                         'file.printers', 'file.print dryRun', 'file.print virtual']}
        with contextlib.redirect_stdout(console):
            demo.console_summary(facts, [{'pass': False, 'stage': 'reopen', 'check': 'bounds'}], [], False)
        for key in facts: self.assertIn(json.dumps(facts[key]), console.getvalue())
        self.assertIn('Comparisons: 0/1 passed; 1 failed', console.getvalue())
        self.assertIn('FAIL comparison: reopen / bounds', console.getvalue())

    def test_desktop_guard_uses_capture_not_session_names_or_foreground_window(self):
        runner = {**self.no_desktop(), 'session_name': '', 'window_station': None,
                  'desktop_name': None, 'primary_screen_size': [1920, 1080],
                  'primary_screenshot_possible': True}
        self.assertTrue(demo.desktop_guard(runner)['continue'])
        self.assertTrue(demo.desktop_guard(runner, github_actions=True)['continue'])
        self.assertFalse(demo.desktop_guard(self.no_desktop())['continue'])
        self.assertEqual(demo.desktop_guard(self.no_desktop())['status'], 'FAIL')
        # A missing probe result is not proof that a screenshot is impossible.
        unknown = {**self.no_desktop(), 'primary_screenshot_possible': None}
        self.assertTrue(demo.desktop_guard(unknown)['continue'])
        self.assertEqual(demo.desktop_guard(unknown)['status'], 'WARN')

    def test_github_runner_override_even_with_failed_probe_and_non_win32_platform(self):
        bad = {**self.no_desktop(), 'sys_platform': 'linux'}
        result = demo.desktop_guard(bad, github_actions=True)
        self.assertTrue(result['continue'])
        self.assertEqual(result['status'], 'WARN')
        self.assertIn('trust the runner', result['reason'])

    def test_probe_diagnostics_preserve_fields_and_probe_errors_are_inconclusive(self):
        from unittest.mock import patch
        observed = {**self.no_desktop(), 'primary_screen_size': [1920, 1080],
                    'primary_screenshot_possible': True, 'user_name': 'runneradmin'}
        with tempfile.TemporaryDirectory() as temp:
            out = Path(temp)
            with patch.object(demo.os, 'name', 'nt'), patch.object(demo, 'powershell', return_value={
                    'exit': 0, 'stdout': '\ufeff' + json.dumps(observed), 'stderr': ''}):
                data = demo.collect_desktop_diagnostics(out)
                self.assertEqual(data['user_name'], 'runneradmin')
                self.assertTrue(data['primary_screenshot_possible'])
                self.assertEqual(data['primary_screen_size'], [1920, 1080])
            out = Path(temp)
            with patch.object(demo.os, 'name', 'nt'), patch.object(demo, 'powershell', side_effect=OSError('probe unavailable')):
                data = demo.collect_desktop_diagnostics(out)
            self.assertIsNone(data['primary_screenshot_possible'])
            self.assertTrue(demo.desktop_guard(data)['continue'])
            self.assertIn('probe unavailable', data['errors'])

    def test_diagnostics_are_in_report_before_guard_is_evaluated(self):
        from unittest.mock import patch
        with tempfile.TemporaryDirectory() as temp:
            out = Path(temp) / 'evidence'
            args = ['demo', '--app', 'unused', '--sample-dir', temp, '--out', str(out)]
            def guard(data, github_actions):
                report = (out / 'REPORT.md').read_text()
                for field in ('session_name', 'session_id', 'window_station', 'desktop_name',
                              'primary_screen_size', 'foreground_window', 'user_name'):
                    self.assertIn(field, report)
                self.assertIn('synthetic-user', report)
                return {'continue': False, 'status': 'FAIL', 'reason': 'synthetic no desktop'}
            with patch.object(demo, 'collect_desktop_diagnostics', return_value=self.no_desktop()), \
                 patch.object(demo, 'desktop_guard', side_effect=guard), patch.object(demo.sys, 'argv', args), \
                 patch.object(demo, 'load_module') as load, contextlib.redirect_stdout(io.StringIO()):
                self.assertEqual(demo.main(), 1)
            load.assert_not_called()
            facts = demo.read_json(out / 'facts.json')
            self.assertEqual(facts['failed step'], 'desktop preflight')
            self.assertIn('synthetic-user', (out / 'REPORT.md').read_text())

    def test_github_failed_probe_reaches_launch_and_records_later_failed_step(self):
        from unittest.mock import Mock, patch
        with tempfile.TemporaryDirectory() as temp:
            out = Path(temp) / 'evidence'
            args = ['demo', '--app', 'unused', '--sample-dir', temp, '--out', str(out)]
            pins, plan = Mock(), Mock(); plan.build_plan.return_value = []
            with patch.object(demo, 'collect_desktop_diagnostics', return_value=self.no_desktop()), \
                 patch.dict(os.environ, {'GITHUB_ACTIONS': 'true'}), patch.object(demo.sys, 'argv', args), \
                 patch.object(demo, 'load_module', side_effect=[pins, plan]), \
                 patch.object(demo, 'powershell', return_value={'exit': 0, 'stdout': '{}', 'stderr': ''}), \
                 patch.object(demo, 'start', side_effect=RuntimeError('synthetic launch failure')) as start, \
                 contextlib.redirect_stdout(io.StringIO()):
                self.assertEqual(demo.main(), 1)
            start.assert_called_once()
            facts = demo.read_json(out / 'facts.json')
            self.assertTrue(facts['desktop guard']['continue'])
            self.assertEqual(facts['failed step'], 'startup and graphics')
            self.assertIn('synthetic launch failure', (out / 'REPORT.md').read_text())
            self.assertIn('synthetic-user', (out / 'REPORT.md').read_text())

    def test_after_launch_command_failure_retains_shot_and_failed_step(self):
        from unittest.mock import Mock, patch
        with tempfile.TemporaryDirectory() as temp:
            out = Path(temp) / 'evidence'
            args = ['demo', '--app', 'unused', '--sample-dir', temp, '--out', str(out)]
            pins, plan, control, owned = Mock(), Mock(), Mock(), Mock()
            plan.build_plan.return_value = [{'method': 'engine.execute', 'params': {'command': 'frame.create'}}]
            control.call.side_effect = [{'ok': True, 'result': []}, RuntimeError('synthetic edit refusal')]
            owned.stop.return_value = {'pid': 123, 'remaining': 0}
            def launch(*args):
                (out / 'earlier-app.png').write_bytes(b'synthetic unit-test evidence')
                return Mock(pid=123), control, io.BytesIO(), Path('startup-synthetic')
            with patch.object(demo, 'collect_desktop_diagnostics', return_value=self.no_desktop()), \
                 patch.dict(os.environ, {'GITHUB_ACTIONS': 'true'}), patch.object(demo.sys, 'argv', args), \
                 patch.object(demo, 'load_module', side_effect=[pins, plan]), \
                 patch.object(demo, 'powershell', return_value={'exit': 0, 'stdout': '{}', 'stderr': ''}), \
                 patch.object(demo, 'start', side_effect=launch), patch.object(demo, 'APP_PROCESSES', [owned]), \
                 contextlib.redirect_stdout(io.StringIO()):
                self.assertEqual(demo.main(), 1)
            owned.stop.assert_called_once()
            self.assertEqual((out / 'earlier-app.png').read_bytes(), b'synthetic unit-test evidence')
            report = (out / 'REPORT.md').read_text()
            self.assertIn('frame.create', report)
            self.assertIn('synthetic edit refusal', report)
            self.assertEqual(demo.read_json(out / 'facts.json')['failed step'], 'frame.create')

    @unittest.skipUnless(sys.platform == 'win32', 'Windows Job Object integration')
    def test_windows_job_terminates_orphaned_child(self):
        with tempfile.TemporaryDirectory() as temp:
            out = Path(temp); pid_file = out / 'child.txt'; log = (out / 'log').open('wb')
            job = demo.WindowsJob()
            script = ('import subprocess,sys,pathlib; '
                      'p=subprocess.Popen([sys.executable,"-c","import time;time.sleep(120)"]); '
                      'pathlib.Path(sys.argv[1]).write_text(str(p.pid))')
            process = subprocess.Popen([sys.executable, '-c', script, str(pid_file)], stdout=log, stderr=log, creationflags=4)
            owned = demo.OwnedProcess(process, log, job)
            try:
                job.attach(process); job.resume(process); process.wait(timeout=15)
                self.assertTrue(pid_file.is_file())
                self.assertGreater(job.active(), 0)  # child outlives parent
                self.assertEqual(owned.stop()['remaining'], 0)
                self.assertTrue(log.closed)
            finally:
                if not owned.stopped: owned.stop()

    @unittest.skipUnless(sys.platform == 'win32' and shutil.which('pwsh'), 'Windows Copy-Item/file-lock integration')
    def test_staging_retries_skips_locked_file_and_splits_reports_from_media(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); source = root / 'live'; source.mkdir(); dest = root / 'stage'
            for name in ['REPORT.md', 'comparison.csv', 'control-transcript.jsonl', 'locked.log', 'transient.log',
                         'app.png', 'desktop.png', 'coffee.pdf', 'coffee.designcraft']:
                (source / name).write_text('synthetic evidence')
            (source / 'build.log').write_text('\n'.join(str(i) for i in range(500)))
            script = r"""
$ErrorActionPreference='Stop'
$global:CopyAttempts=0
function Copy-Item {
    [CmdletBinding()] param([string]$LiteralPath,[string]$Destination,[switch]$Force)
    if ((Split-Path -Leaf $LiteralPath) -eq 'transient.log') {
        $global:CopyAttempts++
        if ($global:CopyAttempts -eq 1) { throw 'synthetic transient copy error' }
    }
    Microsoft.PowerShell.Management\Copy-Item -LiteralPath $LiteralPath -Destination $Destination -Force -ErrorAction Stop
}
$locked=[IO.File]::Open((Join-Path $env:TEST_SOURCE 'locked.log'),'Open','Read','None')
try { & $env:TEST_STAGE -Source $env:TEST_SOURCE -Destination $env:TEST_DEST -DelayMilliseconds 0 }
finally { $locked.Dispose() }
if ($global:CopyAttempts -ne 2) { throw "retry not exercised: $global:CopyAttempts" }
"""
            env = dict(os.environ, TEST_SOURCE=str(source), TEST_DEST=str(dest),
                       TEST_STAGE=str(demo.ROOT / 'packaging/stage_windows_desktop_evidence.ps1'))
            result = subprocess.run(['pwsh', '-NoProfile', '-Command', script], env=env, capture_output=True, text=True, timeout=30)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertTrue((dest / 'report/transient.log').is_file())
            self.assertFalse((dest / 'report/locked.log').exists())
            skipped = (dest / 'report/SKIPPED.txt').read_text(encoding='utf-8-sig')
            self.assertIn('locked.log', skipped); self.assertNotIn('transient.log', skipped)
            for name in ['app.png', 'desktop.png', 'coffee.pdf', 'coffee.designcraft']:
                self.assertTrue((dest / 'shots' / name).is_file())
                self.assertFalse((dest / 'report' / name).exists())
            self.assertFalse((dest / 'report/build.log').exists())
            tail = (dest / 'report/build-tail.log').read_text(encoding='utf-8-sig').splitlines()
            self.assertEqual((len(tail), tail[0], tail[-1]), (400, '100', '499'))
            # Uploader reads only copied files: later mutations cannot change evidence.
            (source / 'REPORT.md').write_text('changed after staging')
            self.assertEqual((dest / 'report/REPORT.md').read_text(), 'synthetic evidence')

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
