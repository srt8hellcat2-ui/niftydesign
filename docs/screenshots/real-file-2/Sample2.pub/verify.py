# Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
"""Verify private full engine replies after same-session and fresh-session reopen."""
import argparse
import hashlib
import json
import math
from pathlib import Path


def verify(folder, sample):
    def reply(name):
        p = folder / name
        assert p.stat().st_size <= 16 * 1024 * 1024
        r = json.loads(p.read_text())
        assert r['ok'], r
        return r['result']

    def bounds(name):
        return next(i['bounds'] for s in reply(name)['spreads'] for i in s['items'] if i['id'] == 9)

    a, b, c, same, fresh = [bounds(n) for n in ['before.json', 'moved.json', 'resized.json',
                                             'same-reopened.json', 'fresh-reopened.json']]
    assert a != b
    assert math.isclose(a[2] - a[0], b[2] - b[0])
    assert math.isclose(a[3] - a[1], b[3] - b[1])
    assert c == same == fresh and b[:2] == c[:2]
    assert math.isclose(c[2] - c[0], 420) and math.isclose(c[3] - c[1], 90)
    before = reply('story-before.json')
    edited = reply('story-edited.json')
    assert edited == reply('same-story.json') == reply('fresh-story.json')
    expected = 'NiftyDesign clean-file editing check.' + before['text'][before['text'].index('\n'):]
    assert edited['text'] == expected and edited['length'] == 165
    assert edited['frames'] == [9] and edited['overset'] is None
    assert reply('preflight-before.json')['errors'] == reply('preflight-after.json')['errors'] == 3
    assert reply('pdf.json')['pages'] == 2
    with (folder / 'edited.pdf').open('rb') as pdf:
        assert pdf.read(5) == b'%PDF-'
    assert sample.stat().st_size == 72704
    with sample.open('rb') as stream:
        assert hashlib.file_digest(stream, 'sha256').hexdigest() == 'c87ea7cc5606023c5fa99376631bbd45a8e9bdaba34c958057995b0e7ac8cc86'
    print('PASS: move, resize and text survive same/fresh sessions; source unchanged; two-page PDF.')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('folder', type=Path)
    parser.add_argument('sample', type=Path)
    args = parser.parse_args()
    verify(args.folder, args.sample)
