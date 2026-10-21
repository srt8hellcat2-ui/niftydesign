#!/usr/bin/env python3
# Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
"""Reproduce the demo using real engine commands and untouched ui_shot captures.
Run from checkout root: python3 docs/demo/coffee-shop/reproduce.py --harness UI_SHOT --cli CLI
The pinned Apache input stays outside the checkout. Full engine replies stay .builder/.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/demo/coffee-shop'
WORK = ROOT / '.builder/coffee-demo'
ROWS = []
NEXT = 27


def command(name, params=None, reply=None):
    row = {'method': 'engine.execute', 'params': {'command': name, 'params': params or {}}}
    row['response'] = str(WORK / (reply or f'command-{len(ROWS):03}') ) + '.json'
    ROWS.append(row)


def shot(name, page=1):
    ROWS.extend([{'method': 'ui.set', 'params': {'page': page, 'fit': 'page', 'dockExpanded': True,
                  'controlBar': True, 'rulers': True, 'frameEdges': False}}, {'steps': 40},
                 {'method': 'ui.set', 'params': {'fit': 'page'}}, {'steps': 20},
                 {'shot': str(OUT / (name + '.png'))}])


def shape(rect, fill, spread=0, ellipse=False, rounded=False):
    global NEXT
    ident = NEXT
    NEXT += 1
    command('frame.create', {'spread': spread, 'rect': rect, 'shape': 'ellipse' if ellipse else 'rectangle'})
    command('object.fill', {'ids': [ident], 'swatch': fill})
    command('object.stroke', {'ids': [ident], 'swatch': '[None]'})
    if rounded:
        command('object.cornerOptions', {'ids': [ident], 'shape': 'rounded', 'size': 14})
    return ident


def text(value, rect, size, family='Source Sans 3', style='Regular', color='Espresso', spread=0):
    global NEXT
    ident, story = NEXT, NEXT + 1
    NEXT += 2
    command('frame.create', {'spread': spread, 'rect': rect, 'content': 'text', 'text': value})
    command('text.select', {'story': story, 'anchor': 0, 'focus': len(value.encode())})
    command('type.char', {'attrs': {'fontFamily': family, 'fontStyle': style, 'size': size,
                                   'leading': {'kind': 'points', 'value': size * 1.3}, 'fill': color}})
    command('type.para', {'attrs': {'align': 'left', 'hyphenate': False}})
    command('edit.deselectAll')
    return ident


def picture(name, x, y, width, dx, dy, final_width, spread=0):
    global NEXT
    asset, ident = NEXT, NEXT + 1
    NEXT += 2
    command('file.place', {'path': str(ROOT / 'assets/demo' / name), 'spread': spread,
                          'x': x, 'y': y, 'width': width}, f'place-{ident}')
    command('document.inspect', reply=f'placed-{ident}')
    from PIL import Image
    with Image.open(ROOT / 'assets/demo' / name) as im:
        height = final_width * im.height / im.width
    command('transform.move', {'ids': [ident], 'dx': dx, 'dy': dy})
    command('document.inspect', reply=f'moved-{ident}')
    command('transform.set', {'ids': [ident], 'width': final_width, 'height': height, 'ref': 0})
    command('object.fit', {'ids': [ident], 'mode': 'fitProportionally'})
    command('document.inspect', reply=f'resized-{ident}')
    return asset, ident


def run(harness, sample):
    global ROWS, NEXT
    NEXT = 27
    os.chdir(ROOT)
    WORK.mkdir(parents=True, exist_ok=True)
    assert hashlib.sha256(sample.read_bytes()).hexdigest() == 'c87ea7cc5606023c5fa99376631bbd45a8e9bdaba34c958057995b0e7ac8cc86'
    ROWS = [{'empty': True}]
    command('file.open', {'path': str(sample)}, 'open')
    shot('01-original')
    ROWS.append({'method': 'ui.dialog.cancel', 'params': {}})
    shot('01b-original-page')
    command('edit.clear', {'ids': [9, 11, 13, 15, 18, 19, 21]})
    command('layout.documentSetup', {'width': 612, 'height': 792, 'facingPages': False})
    command('document.preferences', {'horizontalUnits': 'inches', 'verticalUnits': 'inches'})
    command('layout.pageSize', {'pages': [1], 'width': 612, 'height': 792})
    command('layout.pageSize', {'pages': [2], 'width': 252, 'height': 144})
    command('layout.marginsAndColumns', {'pages': [1], 'margins': 12})
    command('file.place', {'path': str(ROOT / 'assets/demo/bakery-background.png'), 'spread': 0,
                          'x': 0, 'y': 0, 'width': 612}, 'background')
    command('transform.set', {'ids': [26], 'width': 612, 'height': 792, 'ref': 0})
    shot('02-background')
    for name, color in [('Espresso', '#543422'), ('Caramel', '#9B5933'), ('Cream', '#FFF4DE'), ('Gold', '#D79946')]:
        command('swatch.create', {'name': name, 'color': color})
    text('Fresh Today', [44, 55, 570, 148], 66, 'Source Serif 4', 'Bold')
    text('NiftyDesign Coffee Shop', [48, 151, 565, 192], 27, style='Semibold')
    text('Small-batch coffee, made with care.\nButtery pastries, baked for your morning.\nA warm welcome, every day.',
         [50, 207, 560, 290], 17)
    shot('03-headline')
    picture('croissant.png', 40, 330, 240, 12, -18, 280)
    picture('coffee-cup.png', 320, 315, 210, 0, -18, 230)
    shot('04-images-placed')
    shape([48, 513, 564, 626], 'Caramel', rounded=True)
    text("Today's special", [70, 529, 535, 562], 20, style='Semibold', color='Cream')
    text('Croissant + coffee, $4', [70, 568, 535, 608], 28, 'Source Serif 4', 'Bold', 'Cream')
    shot('05-special-box')
    shape([50, 652, 62, 664], 'Caramel', ellipse=True)
    line = NEXT
    NEXT += 1
    command('path.create', {'anchors': [{'p': [78, 658]}, {'p': [560, 658]}]})
    command('object.stroke', {'ids': [line], 'swatch': 'Caramel', 'weight': 1.5})
    text('Mon-Sat 7am-3pm', [50, 680, 560, 712], 19, style='Semibold')
    text('123 Example Street  |  example.com', [50, 722, 565, 752], 16)
    command('edit.deselectAll')
    command('document.inspect', reply='final-flyer')
    shot('06-final-flyer')
    shape([0, 0, 252, 144], 'Cream', spread=1)
    shape([0, 0, 252, 17], 'Caramel', spread=1)
    text('Alex Example', [16, 29, 184, 57], 21, 'Source Serif 4', 'Bold', spread=1)
    text('Barista and owner', [16, 60, 180, 80], 11, spread=1)
    text('alex@example.com\n555-0100', [16, 91, 180, 132], 11, spread=1)
    picture('coffee-cup.png', 185, 64, 53, -4, 8, 58, spread=1)
    command('edit.deselectAll')
    shot('07-business-card', 2)
    command('font.list', reply='fonts')
    command('preflight.run', reply='preflight')
    ROWS.append({'method': 'ui.inspect', 'params': {}, 'response': str(WORK / 'final-ui.json')})
    command('document.inspect', reply='before-save')
    command('file.saveAs', {'path': str(OUT / 'coffee-shop.designcraft')}, 'save')
    command('file.open', {'path': str(OUT / 'coffee-shop.designcraft')}, 'same-open')
    ROWS.append({'steps': 120})
    command('document.inspect', reply='same-reopened')
    shot('08-same-session-reopened')
    command('file.saveAs', {'path': str(WORK / 'same.designcraft')}, 'same-save')
    command('file.exportPdf', {'path': str(OUT / 'coffee-shop.pdf')}, 'pdf')
    script = WORK / 'editing.jsonl'
    script.write_text('\n'.join(map(json.dumps, ROWS)) + '\n')
    env = dict(os.environ, MESA_SHADER_CACHE_DISABLE='true')
    with (WORK / 'editing.log').open('w') as log:
        subprocess.run([harness, str(script)], env=env, stdout=log, stderr=subprocess.STDOUT, check=True)
    for row in ROWS:
        if 'response' in row:
            result = json.loads(Path(row['response']).read_text())
            assert result['ok'], (row, result)
    ROWS = [{'empty': True}]
    command('file.open', {'path': str(OUT / 'coffee-shop.designcraft')}, 'fresh-open')
    ROWS.append({'steps': 120})
    command('document.inspect', reply='fresh-reopened')
    ROWS.append({'method': 'ui.inspect', 'params': {}, 'response': str(WORK / 'fresh-ui.json')})
    shot('09-fresh-session-reopened')
    command('file.saveAs', {'path': str(WORK / 'fresh.designcraft')}, 'fresh-save')
    script = WORK / 'fresh.jsonl'
    script.write_text('\n'.join(map(json.dumps, ROWS)) + '\n')
    with (WORK / 'fresh.log').open('w') as log:
        subprocess.run([harness, str(script)], env=env, stdout=log, stderr=subprocess.STDOUT, check=True)
    for page in (1, 2):
        subprocess.run(['pdftoppm', '-f', str(page), '-l', str(page), '-singlefile', '-r', '144', '-png',
                        str(OUT / 'coffee-shop.pdf'), str(OUT / f'pdf-page-{page}')], check=True)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--harness', required=True)
    parser.add_argument('--cli', required=True)
    parser.add_argument('--sample', type=Path, default=Path('/workspace/sample-inputs/apache-poi-publisher-bf7c9b6a1490/Sample2.pub'))
    args = parser.parse_args()
    run(args.harness, args.sample)
    assert hashlib.sha256(args.sample.read_bytes()).hexdigest() == 'c87ea7cc5606023c5fa99376631bbd45a8e9bdaba34c958057995b0e7ac8cc86'
    subprocess.run([sys.executable, str(OUT / 'verify.py'), '--cli', args.cli], check=True)
