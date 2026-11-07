#!/usr/bin/env python3
# Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
"""Regenerate app icons from the owner's Nifty mark (Pillow required)."""
import base64
from pathlib import Path
from PIL import Image, ImageOps

ROOT = Path(__file__).resolve().parents[1]
source = ROOT / 'assets/nifty/nifty-mark.png'
icon = ImageOps.pad(Image.open(source).convert('RGBA'), (1024, 1024), color=(0, 0, 0, 0))
out = ROOT / 'assets/app-icon'
icon.save(out / 'designcraft-1024.png')
icon.resize((512, 512), Image.Resampling.LANCZOS).save(out / 'designcraft-macos-512.png')
icon.save(out / 'designcraft.ico', sizes=[(s, s) for s in (16, 24, 32, 48, 64, 128, 256)])
icon.save(out / 'designcraft.icns')
for size in (16, 24, 32, 48, 64, 128, 256, 512):
    icon.resize((size, size), Image.Resampling.LANCZOS).save(out / f'hicolor/{size}x{size}/apps/ai.storyteller.designcraft.png')
# Existing scalable-icon install paths stay compatible, but now contain only the owner's mark.
data = base64.b64encode((out / 'designcraft-1024.png').read_bytes()).decode('ascii')
svg = f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024"><image width="1024" height="1024" href="data:image/png;base64,{data}"/></svg>\n'
for name in ('designcraft.svg', 'designcraft-small.svg', 'hicolor/scalable/apps/ai.storyteller.designcraft.svg'):
    (out / name).write_text(svg)
