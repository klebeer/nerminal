#!/usr/bin/env python3
"""Draws the README banner: the app icon, the wordmark, and the release name.

The banner used to be a PNG with no source, so changing the release name under
the wordmark meant redrawing the image. It is generated here instead, from the
same Catppuccin palette the default icon uses, so a new release is a new
argument rather than a new asset.

    ./script/banner/generate_banner.py Carmilla

Needs `rsvg-convert` (`brew install librsvg`), the same tool the icon generator
uses.
"""

import base64
import pathlib
import subprocess
import sys
import tempfile

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent.parent / "icons"))
from generate_app_icons import CATPPUCCIN  # noqa: E402
from install_app_icons import DEFAULT_VARIANT  # noqa: E402

WIDTH, HEIGHT = 1600, 800
ICON_SIZE = 380
ICON_X, ICON_Y = 250, 210

REPO_ROOT = pathlib.Path(__file__).resolve().parent.parent.parent
# Follows whichever variant the app wears, so the banner cannot drift from
# the icon on the dock.
ICON_PNG = REPO_ROOT / f"app/assets/bundled/png/nerminal-{DEFAULT_VARIANT}.png"
OUTPUT = REPO_ROOT / "images/nerminal-banner.png"

# rsvg-convert resolves families through fontconfig, so this asks for the font
# the terminal ships and falls back to what macOS always has.
MONO = "JetBrainsMono Nerd Font Mono, JetBrains Mono, Menlo, monospace"


def banner(codename: str, icon_href: str) -> str:
    c = CATPPUCCIN
    return f"""<svg xmlns="http://www.w3.org/2000/svg" \
xmlns:xlink="http://www.w3.org/1999/xlink" \
width="{WIDTH}" height="{HEIGHT}" viewBox="0 0 {WIDTH} {HEIGHT}">
  <defs>
    <linearGradient id="bg" x1="0" y1="0" x2="0.35" y2="1">
      <stop offset="0%" stop-color="{c['crust']}"/>
      <stop offset="60%" stop-color="{c['base']}"/>
      <stop offset="100%" stop-color="{c['surface']}"/>
    </linearGradient>
  </defs>

  <rect width="{WIDTH}" height="{HEIGHT}" fill="url(#bg)"/>

  <image x="{ICON_X}" y="{ICON_Y}" width="{ICON_SIZE}" height="{ICON_SIZE}"
         xlink:href="{icon_href}"/>
  <!-- The icon and the background are both dark Catppuccin, so without an edge
       the top of the icon dissolves into the page. -->
  <rect x="{ICON_X}" y="{ICON_Y}" width="{ICON_SIZE}" height="{ICON_SIZE}"
        rx="{ICON_SIZE * 0.22:.0f}" fill="none"
        stroke="{c['overlay']}" stroke-width="2" opacity="0.55"/>

  <text x="720" y="400" font-family="{MONO}" font-size="150"
        fill="{c['text']}">Nerminal</text>

  <text x="722" y="500" font-family="{MONO}" font-size="42"
        fill="{c['peach']}">&#8250;</text>
  <text x="790" y="500" font-family="{MONO}" font-size="42"
        letter-spacing="9" fill="{c['mauve']}">{codename.upper()}</text>
</svg>
"""


def main() -> int:
    codename = sys.argv[1] if len(sys.argv) > 1 else "Carmilla"
    if not ICON_PNG.exists():
        print(f"missing {ICON_PNG}, run script/icons/install_app_icons.py first")
        return 1

    # librsvg refuses to follow a file reference out of an SVG, so the icon
    # travels inside the document.
    encoded = base64.b64encode(ICON_PNG.read_bytes()).decode("ascii")
    # The SVG is an intermediate, not a source: it carries the icon inline as
    # base64 and would be rewritten wholesale on every run. Keep it out of the
    # repository so the history holds the script and the result, not the blob.
    with tempfile.TemporaryDirectory() as tmp:
        svg = pathlib.Path(tmp) / "banner.svg"
        svg.write_text(banner(codename, f"data:image/png;base64,{encoded}"))
        subprocess.run(
            ["/opt/homebrew/bin/rsvg-convert", "-w", str(WIDTH), "-h", str(HEIGHT),
             str(svg), "-o", str(OUTPUT)],
            check=True,
        )
    print(f"  {OUTPUT.relative_to(REPO_ROOT)}  ({codename})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
