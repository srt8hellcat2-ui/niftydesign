#!/usr/bin/env python3
# Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
"""Check every saved field, embedded image bytes, moves/resizes and PDF edge pixels.
Run after reproduce.py: python3 docs/demo/coffee-shop/verify.py --cli PATH
Uses only the app's exported document/PDF and owner-supplied images, not PUB encodings.
"""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import zipfile
from PIL import Image

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/demo/coffee-shop'
WORK = ROOT / '.builder/coffee-demo'


def reply(name):
    data = json.loads((WORK / f'{name}.json').read_text())
    assert data['ok'], data
    return data['result']


def load(path):
    with zipfile.ZipFile(path) as archive:
        assert archive.getinfo('document.json').file_size < 4 * 1024 * 1024
        return json.loads(archive.read('document.json')), {
            n: archive.read(n) for n in archive.namelist() if n.startswith('assets/')}


def item(snapshot, ident):
    return next(i for s in snapshot['spreads'] for i in s['items'] if i['id'] == ident)


def affine(xf, point):
    a, b, c, d, e, f = xf
    x, y = point
    return a*x + c*y + e, b*x + d*y + f


def verify(cli):
    native = OUT / 'coffee-shop.designcraft'
    document, embedded = load(native)
    for name in ('same', 'fresh'):
        reopened, images = load(WORK / f'{name}.designcraft')
        assert reopened == document, f'{name}: some document field changed'
        assert images == embedded, f'{name}: embedded image bytes changed'
        state = reply(name + '-reopened')
        for key in ('spreads', 'stories', 'swatches', 'settings', 'parents', 'layers'):
            assert state[key] == reply('before-save')[key], (name, key)
    pages = [p for s in document['spreads'] for p in s['pages']]
    assert [(p['width'], p['height']) for p in pages] == [(612, 792), (252, 144)]
    assert sum(len(s['items']) for s in document['spreads']) == 19
    assert len(document['stories']) == 10
    assert reply('preflight')['errors'] == 0
    assert all(not f['missing'] and not f['styleMissing'] and f['source'] == 'bundled' for f in reply('fonts'))
    assert reply('final-ui')['dialog'] is None
    assert reply('fresh-ui')['dialog'] is None
    assert reply('pdf')['pages'] == 2 and reply('pdf')['warnings'] == []
    moves = []
    for ident, delta, final_width in [(34, (12, -18), 280), (36, (0, -18), 230), (57, (-4, 8), 58)]:
        a = item(reply(f'placed-{ident}'), ident)['bounds']
        b = item(reply(f'moved-{ident}'), ident)['bounds']
        c = item(reply(f'resized-{ident}'), ident)['bounds']
        assert abs(b[0]-a[0]-delta[0]) < 1e-8 and abs(b[1]-a[1]-delta[1]) < 1e-8
        assert abs(c[2]-c[0]-final_width) < 1e-8 and abs(a[2]-a[0]-final_width) > 1
        moves.append({'id': ident, 'placed': a, 'moved': b, 'resized': c})
    for asset_id, asset in document['assets'].items():
        assert embedded[f'assets/{asset_id}.png'] == (ROOT / 'assets/demo' / asset['name']).read_bytes()
    # Render a background-only reference through the actual PDF exporter. This does not
    # edit any committed image or native document: a separate CLI session owns the edits.
    remove = []
    for spread_index, spread in enumerate(document['spreads']):
        for obj in spread['items']:
            if spread_index == 0 and obj['id'] != 26:
                remove.append(obj['id'])
            elif spread_index == 1 and obj['content']['type'] != 'unassigned':
                remove.append(obj['id'])
    reference = WORK / 'background-only.pdf'
    subprocess.run([cli, 'run', '--in', str(native), '--cmd', 'edit.clear=' + json.dumps({'ids': remove}),
                    '--cmd', 'file.exportPdf=' + json.dumps({'path': str(reference)})], check=True,
                   stdout=subprocess.DEVNULL)
    for page in (1, 2):
        subprocess.run(['pdftoppm', '-f', str(page), '-l', str(page), '-singlefile', '-r', '144', '-png',
                        str(reference), str(WORK / f'background-{page}')], check=True)
    probes = []
    for page, spread in enumerate(document['spreads'], 1):
        foreground = Image.open(OUT / f'pdf-page-{page}.png').convert('RGB')
        background = Image.open(WORK / f'background-{page}.png').convert('RGB')
        for obj in spread['items']:
            content = obj['content']
            if content['type'] != 'graphic' or obj['id'] == 26:
                continue
            asset = document['assets'][str(content['asset'])]
            source = Image.open(ROOT / 'assets/demo' / asset['name'])
            assert source.mode == 'RGBA' and source.getextrema()[3][0] == 0
            # Multiple zero-alpha edge pixels around each placed illustration. Compare
            # those PDF pixels with the export of the identical underlying background.
            count, largest = 0, 0
            for sx in (2, source.width//4, source.width//2, source.width-3):
                for sy in (2, source.height-3):
                    if source.getpixel((sx, sy))[3] != 0:
                        continue
                    point = affine(obj['xf'], affine(content['xf'], (sx, sy)))
                    px = tuple(int(v*2) for v in point)
                    actual, expected = foreground.getpixel(px), background.getpixel(px)
                    difference = max(abs(a-b) for a, b in zip(actual, expected))
                    largest = max(largest, difference)
                    assert difference <= 2, (obj['id'], px, actual, expected)
                    count += 1
            assert count >= 4, (obj['id'], count)
            probes.append({'page': page, 'id': obj['id'], 'image': asset['name'],
                           'zero_alpha_edge_probes': count, 'maximum_RGB_difference_0_to_255': largest})
    assert len(probes) == 3
    assert b'/SMask' in (OUT / 'coffee-shop.pdf').read_bytes()
    evidence = {'same_and_fresh_all_document_fields_identical': True,
                'same_and_fresh_all_embedded_bytes_identical': True,
                'page_sizes_pt': [[p['width'], p['height']] for p in pages],
                'image_moves_and_resizes': moves, 'PDF_alpha_edge_checks': probes,
                'assets': document['assets'], 'fonts': reply('fonts'),
                'preflight': reply('preflight'), 'import_warnings': reply('open')['warnings'],
                'objects': [{'spread': s['index'], **i} for s in reply('before-save')['spreads'] for i in s['items']],
                'stories': reply('before-save')['stories']}
    (OUT / 'engine-values.json').write_text(json.dumps(evidence, indent=2) + '\n')
    print('PASS: 19 objects / 10 stories, exact same/fresh round trips, embedded PNG bytes, image moves/resizes, PDF alpha edges.')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--cli', required=True)
    verify(parser.parse_args().cli)
