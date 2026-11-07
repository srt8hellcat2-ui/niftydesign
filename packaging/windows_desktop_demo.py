#!/usr/bin/env python3
# Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
"""Scripted REAL Windows app demo; never a mouse, physical printer or release test."""
import argparse
import atexit
import csv
import hashlib
import importlib.util
import json
import math
import os
from pathlib import Path
import shutil
import socket
import subprocess
import sys
import time
import traceback
import zipfile

ROOT = Path(__file__).resolve().parents[1]
LIMIT = 64 * 1024 * 1024


class WindowsJob:
    """Own the app and descendants, even when the parent exits first.

    Closing the driver's last job handle also kills the tree if the driver is
    forcibly stopped before Python finally/atexit can run.
    """
    def __init__(self):
        import ctypes as c
        from ctypes import wintypes as w
        class BasicLimit(c.Structure):
            _fields_ = [('per_process', c.c_longlong), ('per_job', c.c_longlong),
                        ('flags', w.DWORD), ('min_ws', c.c_size_t), ('max_ws', c.c_size_t),
                        ('active_limit', w.DWORD), ('affinity', c.c_size_t),
                        ('priority', w.DWORD), ('scheduling', w.DWORD)]
        class Io(c.Structure):
            _fields_ = [(name, c.c_ulonglong) for name in
                        ('reads', 'writes', 'other', 'read_bytes', 'write_bytes', 'other_bytes')]
        class ExtendedLimit(c.Structure):
            _fields_ = [('basic', BasicLimit), ('io', Io),
                        ('process_memory', c.c_size_t), ('job_memory', c.c_size_t),
                        ('peak_process_memory', c.c_size_t), ('peak_job_memory', c.c_size_t)]
        class Accounting(c.Structure):
            _fields_ = [(name, c.c_longlong) for name in ('user', 'kernel', 'period_user', 'period_kernel')] + [
                (name, w.DWORD) for name in ('faults', 'total', 'active', 'terminated')]
        self.c, self.accounting = c, Accounting
        self.api = c.WinDLL('kernel32', use_last_error=True)
        signatures = {
            'CreateJobObjectW': ([c.c_void_p, w.LPCWSTR], w.HANDLE),
            'SetInformationJobObject': ([w.HANDLE, c.c_int, c.c_void_p, w.DWORD], w.BOOL),
            'AssignProcessToJobObject': ([w.HANDLE, w.HANDLE], w.BOOL),
            'TerminateJobObject': ([w.HANDLE, w.UINT], w.BOOL),
            'QueryInformationJobObject': ([w.HANDLE, c.c_int, c.c_void_p, w.DWORD, c.c_void_p], w.BOOL),
            'CloseHandle': ([w.HANDLE], w.BOOL),
            'CreateToolhelp32Snapshot': ([w.DWORD, w.DWORD], w.HANDLE),
            'Thread32First': ([w.HANDLE, c.c_void_p], w.BOOL),
            'Thread32Next': ([w.HANDLE, c.c_void_p], w.BOOL),
            'OpenThread': ([w.DWORD, w.BOOL, w.DWORD], w.HANDLE),
            'ResumeThread': ([w.HANDLE], w.DWORD),
        }
        for name, (args, result) in signatures.items():
            fn = getattr(self.api, name); fn.argtypes, fn.restype = args, result
        self.handle = self.api.CreateJobObjectW(None, None)
        if not self.handle: raise c.WinError(c.get_last_error())
        limits = ExtendedLimit(); limits.basic.flags = 0x2000  # JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE
        if not self.api.SetInformationJobObject(self.handle, 9, c.byref(limits), c.sizeof(limits)):
            error = c.WinError(c.get_last_error()); self.close(); raise error

    def attach(self, process):
        if not self.api.AssignProcessToJobObject(self.handle, int(process._handle)):
            raise self.c.WinError(self.c.get_last_error())

    def resume(self, process):
        # Popen starts suspended: attach the job BEFORE any application code can
        # create a child. Resume its initial thread using documented Win32 APIs.
        c = self.c
        from ctypes import wintypes as w
        class ThreadEntry(c.Structure):
            _fields_ = [(name, w.DWORD) for name in ('size', 'usage', 'tid', 'pid')] + [
                ('priority', w.LONG), ('delta', w.LONG), ('flags', w.DWORD)]
        snapshot = self.api.CreateToolhelp32Snapshot(4, 0)  # TH32CS_SNAPTHREAD
        if snapshot == c.c_void_p(-1).value: raise c.WinError(c.get_last_error())
        try:
            entry = ThreadEntry(); entry.size = c.sizeof(entry)
            found = self.api.Thread32First(snapshot, c.byref(entry))
            while found:
                if entry.pid == process.pid:
                    thread = self.api.OpenThread(2, False, entry.tid)  # THREAD_SUSPEND_RESUME
                    if not thread: raise c.WinError(c.get_last_error())
                    try:
                        if self.api.ResumeThread(thread) == 0xFFFFFFFF: raise c.WinError(c.get_last_error())
                    finally: self.api.CloseHandle(thread)
                    return
                found = self.api.Thread32Next(snapshot, c.byref(entry))
            raise RuntimeError('suspended app initial thread was not found')
        finally: self.api.CloseHandle(snapshot)

    def active(self):
        info = self.accounting()
        if not self.api.QueryInformationJobObject(self.handle, 1, self.c.byref(info), self.c.sizeof(info), None):
            raise self.c.WinError(self.c.get_last_error())
        return info.active

    def terminate(self):
        if not self.api.TerminateJobObject(self.handle, 1):
            raise self.c.WinError(self.c.get_last_error())

    def close(self):
        if self.handle:
            self.api.CloseHandle(self.handle); self.handle = None


class OwnedProcess:
    def __init__(self, process, log, job=None):
        self.process, self.log, self.job, self.stopped = process, log, job, False

    def stop(self):
        if self.stopped: return {'pid': self.process.pid, 'remaining': 0, 'already_stopped': True}
        try:
            if self.job:
                self.job.terminate()  # also kill descendants when the parent has already exited
            elif self.process.poll() is None:
                self.process.terminate()
            try:
                self.process.wait(timeout=15)
            except subprocess.TimeoutExpired:
                self.process.kill(); self.process.wait(timeout=15)
            if self.job:
                deadline = time.monotonic() + 15
                while self.job.active():
                    if time.monotonic() >= deadline: raise RuntimeError('app job still has active processes')
                    time.sleep(0.1)
            if self.process.poll() is None: raise RuntimeError('app parent is still running')
            self.stopped = True
            return {'pid': self.process.pid, 'exit': self.process.returncode, 'remaining': 0}
        finally:
            if self.job: self.job.close()
            self.log.close()


APP_PROCESSES = []


def cleanup_apps():
    records = []
    for owned in APP_PROCESSES[:]:
        try:
            records.append(owned.stop())
            APP_PROCESSES.remove(owned)
        except Exception as exc:
            records.append({'pid': owned.process.pid, 'error': str(exc), 'remaining': 'UNVERIFIED'})
            print(f'FAIL process cleanup: {exc}', flush=True)
    return records


atexit.register(cleanup_apps)


def progress(steps, label, status='PASS', detail=''):
    steps.append({'step': label, 'status': status, 'detail': detail})
    print(f'{status} {label}' + (f': {detail}' if detail else ''), flush=True)


def console_summary(facts, comparisons, steps, completed):
    print('\nWindows desktop demo: ' + ('PASS' if completed else 'FAIL / INCOMPLETE'), flush=True)
    print(f"Steps: {sum(s['status'] == 'PASS' for s in steps)} passed, "
          f"{sum(s['status'] == 'FAIL' for s in steps)} failed", flush=True)
    for key in ('desktop diagnostics', 'desktop guard', 'failed step', 'selected startup attempt', 'graphics exact log records', 'system fonts',
                'Get-Printer', 'file.printers', 'file.print dryRun', 'file.print virtual',
                'virtual print', 'process cleanup'):
        value = facts.get(key, 'NOT REACHED')
        if key in ('system fonts', 'Get-Printer') and isinstance(value, dict) and value.get('exit') == 0:
            try: value = json.loads(value['stdout'].lstrip('\ufeff').strip())
            except (KeyError, ValueError): pass
        print(f'{key}: {json.dumps(value)}', flush=True)
    failed = [r for r in comparisons if not r['pass']]
    print(f'Comparisons: {len(comparisons) - len(failed)}/{len(comparisons)} passed; '
          f'{len(failed)} failed' + ('; NOT REACHED' if not comparisons else ''), flush=True)
    for row in failed: print(f"FAIL comparison: {row['stage']} / {row['check']}", flush=True)
    if facts.get('error'): print(facts['error'], flush=True)


def load_module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def read_json(path):
    if path.stat().st_size > LIMIT:
        raise RuntimeError(f'oversized JSON: {path}')
    return json.loads(path.read_text(encoding='utf-8'))


def native(path):
    with zipfile.ZipFile(path) as z:
        if sum(i.file_size for i in z.infolist()) > LIMIT or len(z.infolist()) > 1000:
            raise RuntimeError('oversized native document')
        return json.loads(z.read('document.json')), {
            n: hashlib.sha256(z.read(n)).hexdigest() for n in z.namelist() if n.startswith('assets/')}


def near(a, b, tolerance=0.02):
    if isinstance(a, bool) or isinstance(b, bool):
        return a == b
    if isinstance(a, (int, float)) and isinstance(b, (int, float)):
        return math.isfinite(a) and math.isfinite(b) and abs(a - b) <= tolerance
    if isinstance(a, list) and isinstance(b, list):
        return len(a) == len(b) and all(near(x, y, tolerance) for x, y in zip(a, b))
    if isinstance(a, dict) and isinstance(b, dict):
        return a.keys() == b.keys() and all(near(a[k], b[k], tolerance) for k in a)
    return a == b


def compare_state(state, expected):
    """Return every check; don't hide mismatches behind a summary count."""
    rows = []
    def add(label, want, got):
        rows.append({'check': label, 'expected': want, 'actual': got, 'pass': near(want, got)})
    items = [{'spread': s['index'], **i} for s in state['spreads'] for i in s['items']]
    add('object count', len(expected['objects']), len(items))
    actual = {x['id']: x for x in items}
    add('object IDs', sorted(x['id'] for x in expected['objects']), sorted(actual))
    for obj in expected['objects']:
        add(f"object {obj['id']} (bounds in pt, tolerance 0.02)", obj, actual.get(obj['id']))
    add('stories (preview, length, frames, composition)', expected['stories'], state['stories'])
    pages = [p['bounds'][2:] for s in state['spreads'] for p in s['pages']]
    add('page sizes in pt', expected['page_sizes_pt'], pages)
    return rows


class Control:
    def __init__(self, port, transcript):
        self.port, self.transcript, self.seq = port, transcript, 0

    def call(self, method, params=None, timeout=60, strict=True):
        self.seq += 1
        request = {'id': self.seq, 'method': method, 'params': params or {}}
        entry = {'port': self.port, 'request': request}
        try:
            with socket.create_connection(('127.0.0.1', self.port), timeout=timeout) as s:
                s.settimeout(timeout)
                s.sendall((json.dumps(request) + '\n').encode())
                with s.makefile('rb') as stream:
                    raw = stream.readline(4 * 1024 * 1024 + 1)
                if len(raw) > 4 * 1024 * 1024 or not raw.endswith(b'\n'):
                    raise RuntimeError('missing or oversized control reply')
                reply = json.loads(raw)
            entry['response'] = reply
        except Exception as exc:
            entry['transport_error'] = str(exc)
            raise
        finally:
            with self.transcript.open('a', encoding='utf-8') as f:
                f.write(json.dumps(entry) + '\n')
        if strict and not reply.get('ok'):
            raise RuntimeError(json.dumps(entry))
        return reply

    def engine(self, command, params=None, **kwargs):
        return self.call('engine.execute', {'command': command, 'params': params or {}}, **kwargs)


def powershell(script, out, name, env=None, timeout=30):
    r = subprocess.run(['powershell.exe', '-NoProfile', '-NonInteractive', '-Command',
                        "$ErrorActionPreference='Stop'; " + script],
                       capture_output=True, text=True, timeout=timeout, env=env)
    data = {'exit': r.returncode, 'stdout': r.stdout, 'stderr': r.stderr}
    (out / (name + '.json')).write_text(json.dumps(data, indent=2), encoding='utf-8')
    return data



PRIMARY_DESKTOP = r"""
$diag=@{session_name=$env:SESSIONNAME; session_id=[Diagnostics.Process]::GetCurrentProcess().SessionId;
  window_station=$null; desktop_name=$null; primary_screen_size=$null; foreground_window=$null;
  user_name=[Environment]::UserName; primary_screenshot_possible=$null; errors=@()};
try {
  Add-Type -TypeDefinition 'using System; using System.Text; using System.Runtime.InteropServices;
public class DesktopProbe {
[DllImport("user32.dll")] public static extern IntPtr GetProcessWindowStation();
[DllImport("kernel32.dll")] public static extern uint GetCurrentThreadId();
[DllImport("user32.dll")] public static extern IntPtr GetThreadDesktop(uint id);
[DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
[DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
[DllImport("user32.dll", CharSet=CharSet.Unicode, SetLastError=true)]
public static extern bool GetUserObjectInformation(IntPtr h, int index, StringBuilder text, uint size, out uint needed);
public static string Name(IntPtr h) { var text=new StringBuilder(256); uint needed;
return GetUserObjectInformation(h,2,text,512,out needed) ? text.ToString() : null; }
}';
  $diag.dpi_aware=[DesktopProbe]::SetProcessDPIAware();
  $diag.window_station=[DesktopProbe]::Name([DesktopProbe]::GetProcessWindowStation());
  $diag.desktop_name=[DesktopProbe]::Name([DesktopProbe]::GetThreadDesktop([DesktopProbe]::GetCurrentThreadId()));
  $diag.foreground_window=([DesktopProbe]::GetForegroundWindow() -ne [IntPtr]::Zero);
} catch { $diag.errors += $_.Exception.Message }
try {
  Add-Type -AssemblyName System.Windows.Forms; Add-Type -AssemblyName System.Drawing;
  $b=[System.Windows.Forms.Screen]::PrimaryScreen.Bounds;
  $diag.primary_screen_size=@($b.Width,$b.Height);
  if ($b.Width -le 0 -or $b.Height -le 0) { throw 'Primary screen has no drawable pixels' }
  $bmp=[System.Drawing.Bitmap]::new($b.Width,$b.Height);
  try {
    $g=[System.Drawing.Graphics]::FromImage($bmp);
    try { $g.CopyFromScreen($b.Left,$b.Top,0,0,$b.Size);
      $bmp.Save($env:DEMO_PRIMARY_SCREEN,[System.Drawing.Imaging.ImageFormat]::Png)
    } finally { $g.Dispose() }
  } finally { $bmp.Dispose() }
  $diag.primary_screenshot_possible=$true;
} catch { $diag.primary_screenshot_possible=$false; $diag.errors += $_.Exception.Message }
$diag | ConvertTo-Json -Depth 8 -Compress
"""


def collect_desktop_diagnostics(out):
    # Keep explicit unknowns on probe errors, rather than inventing a session.
    data = {'sys_platform': sys.platform, 'os_name': os.name,
            'github_actions': os.environ.get('GITHUB_ACTIONS'), 'runner_os': os.environ.get('RUNNER_OS'),
            'session_name': os.environ.get('SESSIONNAME'), 'session_id': None,
            'window_station': None, 'desktop_name': None, 'primary_screen_size': None,
            'foreground_window': None, 'user_name': os.environ.get('USERNAME') or os.environ.get('USER'),
            'primary_screenshot_possible': None, 'errors': []}
    if os.name != 'nt':
        data['primary_screenshot_possible'] = False
        data['errors'].append(f'Windows CopyFromScreen is unavailable on os.name={os.name}')
        return data
    try:
        raw = powershell(PRIMARY_DESKTOP, out, 'desktop-probe',
                         dict(os.environ, DEMO_PRIMARY_SCREEN=str(out / '00-primary-screen-probe.png')))
        data['probe_response'] = raw
        if raw['exit']:
            data['errors'].append('PowerShell diagnostics failed; capture availability is UNKNOWN')
        else:
            observed = json.loads(raw['stdout'].lstrip('\ufeff').strip())
            data.update(observed)
    except Exception as exc:
        data['errors'].append(str(exc))
    return data


def desktop_guard(diagnostics, github_actions=False):
    possible = diagnostics.get('primary_screenshot_possible')
    if github_actions:
        return {'continue': True, 'status': 'PASS' if possible is True else 'WARN',
                'reason': 'GITHUB_ACTIONS=true: trust the runner and attempt app launch; diagnostics are advisory.'}
    if possible is False:
        return {'continue': False, 'status': 'FAIL', 'reason': 'Primary-screen capture is unavailable; see desktop diagnostics.'}
    return {'continue': True, 'status': 'PASS' if possible is True else 'WARN',
            'reason': 'Primary-screen capture succeeded.' if possible is True else
                      'Capture probe is inconclusive; attempt app launch instead of rejecting a session signal.'}


def checkpoint_report(out, facts, step, error=None):
    (out / 'REPORT.md').write_text(
        '# Windows desktop demo — ' + ('FAILED / INCOMPLETE' if error else 'INCOMPLETE') +
        '\n\n' + ('Failed step: ' if error else 'Current step: ') + step +
        '\n\nNo completion claim. Desktop diagnostics and facts collected so far:\n\n```json\n' +
        json.dumps(facts, indent=2) + '\n```\n' + (error or ''), encoding='utf-8')


WINDOW = r'''
Add-Type -TypeDefinition 'using System; using System.Runtime.InteropServices;
public class DemoWindow { [StructLayout(LayoutKind.Sequential)] public struct R { public int L,T,Ri,B; }
[DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out R r);
[DllImport("user32.dll")] public static extern bool SetProcessDPIAware(); }';
$aware=[DemoWindow]::SetProcessDPIAware();
$p=Get-Process -Id ([int]$env:DEMO_PID); $r=New-Object DemoWindow+R;
$valid=[DemoWindow]::GetWindowRect($p.MainWindowHandle,[ref]$r);
@{dpiAware=$aware; title=$p.MainWindowTitle; handle=$p.MainWindowHandle.ToInt64(); rectValid=$valid;
  bounds=@($r.L,$r.T,$r.Ri,$r.B); size=@(($r.Ri - $r.L),($r.B - $r.T))} | ConvertTo-Json -Compress
'''
DESKTOP = r'''
Add-Type -TypeDefinition 'using System.Runtime.InteropServices; public class DemoDpi {
[DllImport("user32.dll")] public static extern bool SetProcessDPIAware(); }';
$aware=[DemoDpi]::SetProcessDPIAware();
Add-Type -AssemblyName System.Windows.Forms; Add-Type -AssemblyName System.Drawing;
$b=[System.Windows.Forms.SystemInformation]::VirtualScreen;
$bmp=[System.Drawing.Bitmap]::new($b.Width,$b.Height);
$g=[System.Drawing.Graphics]::FromImage($bmp);
try { $g.CopyFromScreen($b.Left,$b.Top,0,0,$b.Size);
$bmp.Save($env:DEMO_SHOT,[System.Drawing.Imaging.ImageFormat]::Png) }
finally { $g.Dispose(); $bmp.Dispose() }
@{dpiAware=$aware; bounds=@($b.Left,$b.Top,$b.Width,$b.Height)} | ConvertTo-Json -Compress
'''


def capture(control, out, name, pid):
    control.call('ui.focus')
    control.call('ui.screenshot', {'path': str(out / (name + '-app.png'))})
    (out / (name + '-ui.json')).write_text(json.dumps(control.call('ui.inspect'), indent=2), encoding='utf-8')
    env = dict(os.environ, DEMO_SHOT=str(out / (name + '-desktop.png')), DEMO_PID=str(pid))
    shot = powershell(DESKTOP, out, name + '-desktop-capture', env)
    window = powershell(WINDOW, out, name + '-window', env)
    if shot['exit'] or window['exit']:
        raise RuntimeError(f'OS screenshot/window query failed at {name}; see exact PowerShell results')
    state = json.loads(window['stdout'].lstrip('\ufeff').strip())
    if not state['handle'] or not state['rectValid'] or min(state['size']) <= 0 or 'NiftyDesign' not in state['title']:
        raise RuntimeError(f'no visible NiftyDesign main window at {name}: {state}')
    from PIL import Image
    for kind in ('app', 'desktop'):
        with Image.open(out / f'{name}-{kind}.png') as im:
            if all(lo == hi for lo, hi in im.convert('RGB').getextrema()):
                raise RuntimeError(f'{kind} screenshot is uniform at {name}; desktop/session may be unavailable')


def start(app, out, attempts):
    for backend in (None, 'dx12', 'gl'):
        name = backend or 'default'
        folder = out / ('startup-' + name)
        folder.mkdir()
        env = dict(os.environ, DESIGNCRAFT_CONFIG_DIR=str(folder / 'settings'), RUST_LOG='info')
        if backend:
            env['WGPU_BACKEND'] = backend
        else:
            env.pop('WGPU_BACKEND', None)
        with socket.socket() as s:
            s.bind(('127.0.0.1', 0)); port = s.getsockname()[1]
        log = (folder / 'process.log').open('wb')
        job = None
        try:
            job = WindowsJob() if sys.platform == 'win32' else None
            process = subprocess.Popen([str(app), '--control', str(port)], cwd=ROOT, env=env,
                                       stdout=log, stderr=subprocess.STDOUT, creationflags=4 if job else 0)
        except BaseException:
            if job: job.close()
            log.close()
            raise
        owned = OwnedProcess(process, log, job)
        APP_PROCESSES.append(owned)
        if job:
            try:
                job.attach(process)
                job.resume(process)
            except BaseException:
                # An attach failure leaves a suspended parent with no children.
                process.kill()
                owned.stop()
                APP_PROCESSES.remove(owned)
                raise
        control = Control(port, out / 'control-transcript.jsonl')
        record = {'backend_requested': name, 'pid': process.pid, 'ready': False}
        attempts.append(record)
        deadline = time.monotonic() + 120
        try:
            while time.monotonic() < deadline and process.poll() is None:
                try:
                    control.call('ui.inspect', timeout=2)
                    control.call('ui.resize', {'width': 1280, 'height': 900})
                    capture(control, out, '00-start-' + name, process.pid)
                    record['ready'] = True
                    print(f'PASS startup backend={name} pid={process.pid}', flush=True)
                    return process, control, log, folder
                except (ConnectionError, OSError):
                    time.sleep(0.5)
            raise RuntimeError(f'no usable window/control channel within 120s; exit={process.poll()}')
        except Exception as exc:
            record['failure'] = str(exc)
            print(f'FAIL startup backend={name}: {exc}', flush=True)
            record['cleanup'] = owned.stop()
            APP_PROCESSES.remove(owned)
    raise RuntimeError('No usable graphics adapter/window with default, dx12 or gl. See startup logs; no demo was claimed.')


def print_demo(control, out, facts):
    facts['Get-Printer'] = powershell('Get-Printer | Select-Object Name,DriverName,PortName | ConvertTo-Json', out, 'get-printer')
    listed = control.engine('file.printers', strict=False)
    facts['file.printers'] = listed
    names = listed.get('result', {}).get('printers', []) if listed.get('ok') else []
    printer = next((p for p in ('Microsoft Print to PDF', 'Microsoft XPS Document Writer') if p in names), None)
    params = {'dryRun': True, 'systemDialog': False, 'pages': '1-2', 'copies': 1}
    if printer:
        params['printer'] = printer
    dry = control.engine('file.print', params, strict=False)
    facts['file.print dryRun'] = dry
    if printer and dry.get('ok'):
        # A fixed local port avoids a Save Print Output dialog. Never select a physical queue.
        output = out / ('virtual-printer.pdf' if printer == 'Microsoft Print to PDF' else 'virtual-printer.xps')
        facts['virtual output path'] = str(output)
        env = dict(os.environ, DEMO_PRINT_OUTPUT=str(output), DEMO_PRINTER=printer)
        facts['virtual port setup'] = powershell(r'''
$p=$env:DEMO_PRINTER; $port=$env:DEMO_PRINT_OUTPUT;
if (-not (Get-PrinterPort -Name $port -ErrorAction SilentlyContinue)) { Add-PrinterPort -Name $port }
Set-Printer -Name $p -PortName $port; Get-Printer -Name $p | Select Name,PortName | ConvertTo-Json
''', out, 'virtual-port', env)
        params['dryRun'] = False
        try:
            facts['file.print virtual'] = control.engine('file.print', params, timeout=90, strict=False)
        except Exception as exc:
            facts['file.print virtual'] = {'response': None, 'transport_error': str(exc)}
            raise
        deadline = time.monotonic() + 20
        while time.monotonic() < deadline and (not output.exists() or output.stat().st_size == 0):
            time.sleep(0.5)
        facts['virtual output bytes'] = output.stat().st_size if output.exists() else None
    else:
        facts['virtual print'] = 'SKIPPED: installed allowlisted PDF printer and successful dry run required.'
    facts['physical printing'] = 'UNVERIFIED: runner has no physical printer.'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--app', type=Path, required=True)
    parser.add_argument('--sample-dir', type=Path, required=True)
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args()
    app, out = args.app.resolve(), args.out.resolve()
    if out.exists() and any(p.name not in {'REPORT.md', 'build.log'} for p in out.iterdir()):
        raise RuntimeError('choose a fresh evidence folder (only workflow preflight REPORT.md/build.log may exist)')
    out.mkdir(parents=True, exist_ok=True)
    (out / 'REPORT.md').write_text('# INCOMPLETE: driver started\n\nNo completion claim. If interrupted, inspect the partial logs and captures.\n', encoding='utf-8')
    facts, attempts, comparisons, steps = {}, [], [], []
    facts['steps'] = steps
    current_step = 'desktop preflight'
    process = log = None
    error = None
    completed = False
    try:
        facts['desktop diagnostics'] = collect_desktop_diagnostics(out)
        # Persist observations BEFORE applying any decision, including a runner override.
        checkpoint_report(out, facts, current_step)
        decision = desktop_guard(facts['desktop diagnostics'],
                                 os.environ.get('GITHUB_ACTIONS', '').strip().lower() == 'true')
        facts['desktop guard'] = decision
        checkpoint_report(out, facts, current_step)
        progress(steps, current_step, decision['status'], json.dumps(facts['desktop diagnostics']) + ' ' + decision['reason'])
        if not decision['continue']: raise RuntimeError(decision['reason'])
        current_step = 'pinned sample'
        checkpoint_report(out, facts, current_step)
        pins = load_module('demo_pins', ROOT / '.github/scripts/test-pub-cli.py')
        with (ROOT / '.github/test-data/publisher-fixtures.csv').open(encoding='utf-8', newline='') as f:
            records = [r for r in csv.DictReader(f) if r['set'] == 'real' and r['file'] == 'Sample2.pub']
        if len(records) != 1:
            raise RuntimeError('missing unique Sample2 pin')
        pins.prepare(args.sample_dir.resolve(), records, True)
        sample = args.sample_dir.resolve() / 'Sample2.pub'
        facts['sample'] = records[0]
        progress(steps, current_step)
        current_step = 'system fonts'
        checkpoint_report(out, facts, current_step)
        facts['system fonts'] = powershell(r'''
Add-Type -AssemblyName System.Drawing; $f=New-Object System.Drawing.Text.InstalledFontCollection;
$n=@($f.Families | ForEach-Object Name); @{Arial=($n -contains 'Arial');
TimesNewRoman=($n -contains 'Times New Roman')} | ConvertTo-Json; $f.Dispose()
''', out, 'system-fonts')
        progress(steps, current_step, detail=json.dumps(facts['system fonts']))
        current_step = 'startup and graphics'
        checkpoint_report(out, facts, current_step)
        demo = load_module('coffee_plan', ROOT / 'docs/demo/coffee-shop/reproduce.py')
        demo.OUT, demo.WORK = out, out / 'engine'
        plan = demo.build_plan(sample)
        (out / 'command-plan.json').write_text(json.dumps(plan, indent=2), encoding='utf-8')
        process, control, log, settings = start(app, out, attempts)
        facts['selected startup attempt'] = settings.name
        progress(steps, current_step)
        (out / 'documented-command-specs.json').write_text(json.dumps(control.call('engine.commands'), indent=2), encoding='utf-8')
        before = same = None
        for row in plan:
            if 'steps' in row:
                time.sleep(min(row['steps'] / 60, 2))
            elif 'shot' in row:
                current_step = 'capture ' + Path(row['shot']).stem
                checkpoint_report(out, facts, current_step)
                capture(control, out, Path(row['shot']).stem, process.pid)
                progress(steps, current_step)
            elif 'method' in row:
                current_step = row.get('params', {}).get('command', row['method'])
                checkpoint_report(out, facts, current_step)
                response = control.call(row['method'], row.get('params'))
                if 'response' in row:
                    dest = Path(row['response']); dest.parent.mkdir(parents=True, exist_ok=True)
                    dest.write_text(json.dumps(response, indent=2), encoding='utf-8')
                    if dest.name == 'before-save.json': before = response['result']
                    if dest.name == 'same-reopened.json': same = response['result']
                    if dest.name == 'open.json':
                        facts['import'] = response
                        facts['import font resolution'] = control.engine('font.list')
                        facts['initial ui (missing-font dialog)'] = control.call('ui.inspect')
                progress(steps, current_step)
        current_step = 'save/reopen engine comparisons'
        checkpoint_report(out, facts, current_step)
        facts['Sample2 SHA-256 after demo'] = pins.digest(sample)
        if facts['Sample2 SHA-256 after demo'] != records[0]['sha256']:
            raise RuntimeError('source Sample2.pub changed')
        expected = read_json(ROOT / 'docs/demo/coffee-shop/engine-values.json')
        for label, state in [('before save', before), ('same-session reopen', same)]:
            for row in compare_state(state, expected):
                comparisons.append({'stage': label, **row})
        reference, _ = native(ROOT / 'docs/demo/coffee-shop/coffee-shop.designcraft')
        for label, name in [('saved', 'coffee-shop.designcraft'), ('same-session', 'engine/same.designcraft')]:
            document, assets = native(out / name)
            for sid, story in reference['stories'].items():
                actual = control.engine('story.get', {'story': int(sid)})['result']['text']
                comparisons.append({'stage': label, 'check': 'full story ' + sid,
                                    'expected': story['text'], 'actual': {'engine': actual, 'saved': document['stories'].get(sid, {}).get('text')},
                                    'pass': actual == story['text'] == document['stories'].get(sid, {}).get('text')})
            for ident, want in expected['assets'].items():
                got = document['assets'].get(ident, {})
                ok = (got.get('name') == want['name'] and got.get('pixels') == want['pixels']
                      and Path(got.get('link', '')).name == want['name']
                      and (ROOT / 'assets/demo' / want['name']).samefile(got.get('link', '')))
                comparisons.append({'stage': label, 'check': 'asset link/pixels ' + ident,
                                    'expected': want, 'actual': got, 'pass': ok})
            if label == 'saved': original_doc, original_assets = document, assets
            else:
                comparisons.append({'stage': label, 'check': 'all document fields and embedded bytes',
                                    'expected': True, 'actual': document == original_doc and assets == original_assets,
                                    'pass': document == original_doc and assets == original_assets})
        progress(steps, current_step, 'PASS' if all(r['pass'] for r in comparisons) else 'FAIL')
        current_step = 'PDF page rasters'
        checkpoint_report(out, facts, current_step)
        import pypdfium2 as pdfium
        pdf = pdfium.PdfDocument(out / 'coffee-shop.pdf')
        try:
            if len(pdf) != 2: raise RuntimeError(f'export has {len(pdf)} pages, expected 2')
            for i in range(len(pdf)):
                page = pdf[i]; bitmap = page.render(scale=2)
                try:
                    image = bitmap.to_pil()
                    image.save(out / f'pdf-page-{i+1}.png')
                    if all(lo == hi for lo, hi in image.convert('RGB').getextrema()):
                        raise RuntimeError(f'PDF page {i+1} is uniformly colored')
                finally: bitmap.close(); page.close()
        finally: pdf.close()
        progress(steps, current_step)
        current_step = 'final fonts and preflight'
        checkpoint_report(out, facts, current_step)
        facts['final fonts'] = control.engine('font.list')
        facts['final preflight'] = control.engine('preflight.run')
        fonts = facts['final fonts']['result']
        comparisons.append({'stage': 'final', 'check': 'fonts resolve from bundled assets',
                            'expected': True, 'actual': fonts,
                            'pass': bool(fonts) and all(not f.get('missing') and not f.get('styleMissing') and f.get('source') == 'bundled' for f in fonts)})
        actual_errors = facts['final preflight']['result']['errors']
        comparisons.append({'stage': 'final', 'check': 'preflight error count',
                            'expected': expected['preflight']['errors'], 'actual': actual_errors,
                            'pass': actual_errors == expected['preflight']['errors']})
        progress(steps, current_step, 'PASS' if all(r['pass'] for r in comparisons) else 'FAIL')
        current_step = 'print probes'
        checkpoint_report(out, facts, current_step)
        print_demo(control, out, facts)
        probes_ok = all(facts[k].get('ok') for k in ('file.printers', 'file.print dryRun'))
        if 'file.print virtual' in facts:
            probes_ok = probes_ok and bool(facts['file.print virtual'].get('ok'))
        progress(steps, current_step, 'PASS' if probes_ok else 'FAIL',
                 json.dumps({k: v for k, v in facts.items() if 'print' in k.lower()}))
        if not probes_ok: raise RuntimeError('print probe returned an error; see exact responses')
        current_step = 'comparison table result'
        checkpoint_report(out, facts, current_step)
        if any(not row['pass'] for row in comparisons):
            raise RuntimeError('engine comparison failed; see comparison.csv')
        progress(steps, current_step)
        completed = True
    except Exception as exc:
        error = traceback.format_exc()
        facts['failed step'] = current_step
        progress(steps, current_step, 'FAIL', str(exc))
        try: checkpoint_report(out, facts, current_step, error)
        except OSError as write_error: print(f'FAIL checkpoint report: {write_error}', flush=True)
    finally:
        facts['process cleanup'] = cleanup_apps()
        if any(r.get('remaining') != 0 for r in facts['process cleanup']):
            completed = False
            error = (error or '') + '\nProcess cleanup could not verify that every process exited.'
            progress(steps, 'process cleanup', 'FAIL')
        else:
            progress(steps, 'process cleanup')
        facts['steps'] = steps
        try:
            with (out / 'comparison.csv').open('w', newline='', encoding='utf-8') as f:
                writer = csv.DictWriter(f, fieldnames=['stage','check','pass','expected','actual']); writer.writeheader()
                for row in comparisons:
                    writer.writerow({**row, 'expected': json.dumps(row['expected']), 'actual': json.dumps(row['actual'])})
            graphics = []
            for path in out.glob('startup-*/settings/logs/*.log'):
                for line in path.read_text(errors='replace').splitlines():
                    if 'drawing on ' in line or 'graphics:' in line:
                        graphics.append({'log': str(path.relative_to(out)), 'line': line,
                                         'software': True if 'Cpu' in line else (False if 'DiscreteGpu' in line or 'IntegratedGpu' in line else 'UNKNOWN')})
            windows = {}
            for path in out.glob('*-window.json'):
                raw = read_json(path)
                try: windows[path.name] = json.loads(raw['stdout'].lstrip('\ufeff').strip())
                except (ValueError, KeyError): windows[path.name] = raw
            facts['window titles and sizes'] = windows
            initial_ui = out / '01-original-ui.json'
            if initial_ui.exists():
                facts['initial ui after first screenshot (missing-font dialog)'] = read_json(initial_ui)
            facts['graphics exact log records'] = graphics or 'UNKNOWN: no selected-adapter record; inspect all startup logs.'
            facts['attempts'] = attempts
            facts['demo completed'] = completed
            facts['error'] = error
            facts['limits'] = 'Scripted real Windows app; no mouse, physical printer, SmartScreen, or native file-dialog verification. Not proof of a human desktop pass.'
            (out / 'facts.json').write_text(json.dumps(facts, indent=2), encoding='utf-8')
            (out / 'REPORT.md').write_text('# Windows desktop demo\n\n' + ('COMPLETED' if completed else 'FAILED / INCOMPLETE') +
                '\n\nExact facts and responses (including Get-Printer, system fonts, printing and graphics):\n\n```json\n' +
                json.dumps(facts, indent=2) + '\n```\n\nWindow titles/sizes: `*-window.json`. App logs: '
                '`startup-*/settings/logs/designcraft*.log`; process output: `startup-*/process.log`. '
                'Screenshots are complete and unretouched; app and desktop captures are separate files.\n', encoding='utf-8')
        except Exception:
            completed = False
            facts['error'] = (error or '') + '\nEvidence finalization failed:\n' + traceback.format_exc()
            progress(steps, 'write final evidence', 'FAIL', facts['error'])
        finally:
            console_summary(facts, comparisons, steps, completed)
    return 0 if completed else 1


if __name__ == '__main__':
    sys.exit(main())
