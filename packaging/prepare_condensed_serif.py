"""Prepare the two bundled static serif faces from the pinned open-licence input.

Optional asset preparation only; normal builds do not need this script's font library.
The original input, its licence and provenance are recorded in SOURCES.md.
"""
import argparse
import hashlib
from pathlib import Path


def prepare(source, destination):
    # Imported only for this manual asset-preparation step, never by packaging/builds.
    from fontTools.ttLib import TTFont
    from fontTools.varLib.instancer import instantiateVariableFont

    expected = '351ced75f3851806aa6d846b669361521eb1925cfc530396df9c1a1b77061ddb'
    if hashlib.sha256(source.read_bytes()).hexdigest() != expected:
        raise ValueError('The serif input does not match the pinned source hash')
    destination.mkdir(parents=True, exist_ok=True)
    for style, weight in [('Regular', 400), ('Bold', 700)]:
        font = TTFont(source)
        instantiateVariableFont(font, {'wdth': 62.5, 'opsz': 144, 'wght': weight, 'GRAD': 0}, inplace=True)
        names = {
            1: 'Nifty Narrow Serif', 2: style,
            3: f'NiftyNarrowSerif-{style}-static-62.5-144',
            4: f'Nifty Narrow Serif {style}', 6: f'NiftyNarrowSerif-{style}',
            16: 'Nifty Narrow Serif', 17: style,
            21: 'Nifty Narrow Serif', 22: style,
        }
        for record in font['name'].names:
            if record.nameID in names:
                record.string = names[record.nameID].encode(record.getEncoding(), errors='replace')
        font.recalcTimestamp = False
        font.save(destination / f'NiftyNarrowSerif-{style}.ttf')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('source', type=Path, help='Pinned original variable serif file, outside the checkout')
    parser.add_argument('destination', type=Path, help='Directory for the two generated static files')
    args = parser.parse_args()
    prepare(args.source, args.destination)
