#!/usr/bin/env bash
# Regenerate NiftyDesign app icons from assets/nifty/nifty-mark.png.
# Requires Python 3 and Pillow. Outputs are committed; builds need neither tool.
set -euo pipefail
python3 "$(dirname "${BASH_SOURCE[0]}")/nifty-icons.py"
