#!/usr/bin/env python3
# Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
"""Scripted REAL Windows app demo; never a mouse, physical printer or release test."""
import argparse
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
        process = subprocess.Popen([str(app), '--control', str(port)], cwd=ROOT, env=env,
                                   stdout=log, stderr=subprocess.STDOUT)
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
                    return process, control, log, folder
                except (ConnectionError, OSError):
                    time.sleep(0.5)
            raise RuntimeError(f'no usable window/control channel within 120s; exit={process.poll()}')
        except Exception as exc:
            record['failure'] = str(exc)
            if process.poll() is None:
                process.kill()
            process.wait(timeout=15)
            log.close()
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
    facts, attempts, comparisons = {}, [], []
    process = log = None
    error = None
    completed = False
    try:
        if sys.platform != 'win32':
            raise RuntimeError('This driver requires a real Windows interactive desktop.')
        pins = load_module('demo_pins', ROOT / '.github/scripts/test-pub-cli.py')
        with (ROOT / '.github/test-data/publisher-fixtures.csv').open(encoding='utf-8', newline='') as f:
            records = [r for r in csv.DictReader(f) if r['set'] == 'real' and r['file'] == 'Sample2.pub']
        if len(records) != 1:
            raise RuntimeError('missing unique Sample2 pin')
        pins.prepare(args.sample_dir.resolve(), records, True)
        sample = args.sample_dir.resolve() / 'Sample2.pub'
        facts['sample'] = records[0]
        facts['system fonts'] = powershell(r'''
Add-Type -AssemblyName System.Drawing; $f=New-Object System.Drawing.Text.InstalledFontCollection;
$n=@($f.Families | ForEach-Object Name); @{Arial=($n -contains 'Arial');
TimesNewRoman=($n -contains 'Times New Roman')} | ConvertTo-Json; $f.Dispose()
''', out, 'system-fonts')
        demo = load_module('coffee_plan', ROOT / 'docs/demo/coffee-shop/reproduce.py')
        demo.OUT, demo.WORK = out, out / 'engine'
        plan = demo.build_plan(sample)
        (out / 'command-plan.json').write_text(json.dumps(plan, indent=2), encoding='utf-8')
        process, control, log, settings = start(app, out, attempts)
        facts['selected startup attempt'] = settings.name
        (out / 'documented-command-specs.json').write_text(json.dumps(control.call('engine.commands'), indent=2), encoding='utf-8')
        before = same = None
        for row in plan:
            if 'steps' in row:
                time.sleep(min(row['steps'] / 60, 2))
            elif 'shot' in row:
                capture(control, out, Path(row['shot']).stem, process.pid)
            elif 'method' in row:
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
        print_demo(control, out, facts)
        if any(not row['pass'] for row in comparisons):
            raise RuntimeError('engine comparison failed; see comparison.csv')
        completed = True
    except Exception:
        error = traceback.format_exc()
    finally:
        if process is not None:
            facts['process exit before cleanup'] = process.poll()
            if process.poll() is None:
                process.terminate()
            try: process.wait(timeout=15)
            except subprocess.TimeoutExpired: process.kill(); process.wait(timeout=15)
            log.close()
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
    return 0 if completed else 1


if __name__ == '__main__':
    sys.exit(main())
