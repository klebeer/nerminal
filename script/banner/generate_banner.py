#!/usr/bin/env python3
"""Draws the README banner: the app icon, the wordmark, and the release name.

The banner used to be a PNG with no source, so changing the release name under
the wordmark meant redrawing the image. It is generated here instead, and a new
release is a new argument rather than a new asset.

    ./script/banner/generate_banner.py Ruthven
    ./script/banner/generate_banner.py Ruthven --variant bloodmoon

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
OUTPUT = REPO_ROOT / "images/nerminal-banner.png"

# rsvg-convert resolves families through fontconfig, so this asks for the font
# the terminal ships and falls back to what macOS always has.
MONO = "JetBrainsMono Nerd Font Mono, JetBrains Mono, Menlo, monospace"

# One entry per icon variant the banner can wear. The background has to come
# from the same family as the icon or the two read as separate images pasted
# together, which is what a single accent colour cannot rescue.
#
# `bloodmoon` is sampled from the drawing this generator replaced, so the warm
# synthwave banner is reproducible rather than lost with the file.
PALETTES = {
    "catppuccin": dict(
        sky_top=CATPPUCCIN["crust"],
        sky_mid=CATPPUCCIN["base"],
        sky_low=CATPPUCCIN["surface"],
        edge=CATPPUCCIN["overlay"],
        wordmark=CATPPUCCIN["text"],
        chevron=CATPPUCCIN["peach"],
        codename=CATPPUCCIN["mauve"],
        scanline="#000000",
    ),
    # The Ruthven icon is Catppuccin with a different face, so the banner around
    # it is the same page.
    "ruthven": None,
    "bloodmoon": dict(
        sky_top="#28172f",
        sky_mid="#3a1f2f",
        sky_low="#45242f",
        edge="#7a4a55",
        wordmark="#f2e3c8",
        chevron="#f5993f",
        codename="#e57bc1",
        scanline="#000000",
    ),
}


def scanlines() -> str:
    """The horizontal banding of a CRT, at the same 4px pitch as the drawing."""
    return "".join(
        f'<rect x="0" y="{y}" width="{WIDTH}" height="1"/>'
        for y in range(0, HEIGHT, 4)
    )


def banner(codename: str, palette: dict, icon_href: str) -> str:
    c = palette
    return f"""<svg xmlns="http://www.w3.org/2000/svg" \
xmlns:xlink="http://www.w3.org/1999/xlink" \
width="{WIDTH}" height="{HEIGHT}" viewBox="0 0 {WIDTH} {HEIGHT}">
  <defs>
    <linearGradient id="bg" x1="0" y1="0" x2="0.35" y2="1">
      <stop offset="0%" stop-color="{c['sky_top']}"/>
      <stop offset="60%" stop-color="{c['sky_mid']}"/>
      <stop offset="100%" stop-color="{c['sky_low']}"/>
    </linearGradient>
  </defs>

  <rect width="{WIDTH}" height="{HEIGHT}" fill="url(#bg)"/>
  <g fill="{c['scanline']}" opacity="0.10">{scanlines()}</g>

  <image x="{ICON_X}" y="{ICON_Y}" width="{ICON_SIZE}" height="{ICON_SIZE}"
         xlink:href="{icon_href}"/>
  <!-- The icon and the background share a palette, so without an edge the top
       of the icon dissolves into the page. -->
  <rect x="{ICON_X}" y="{ICON_Y}" width="{ICON_SIZE}" height="{ICON_SIZE}"
        rx="{ICON_SIZE * 0.22:.0f}" fill="none"
        stroke="{c['edge']}" stroke-width="2" opacity="0.55"/>

  <text x="720" y="400" font-family="{MONO}" font-size="150"
        fill="{c['wordmark']}">Nerminal</text>

  <text x="722" y="500" font-family="{MONO}" font-size="42"
        fill="{c['chevron']}">&#8250;</text>
  <text x="790" y="500" font-family="{MONO}" font-size="42"
        letter-spacing="9" fill="{c['codename']}">{codename.upper()}</text>
</svg>
"""


def main() -> int:
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    codename = args[0] if args else "Ruthven"

    variant = DEFAULT_VARIANT
    if "--variant" in sys.argv:
        variant = sys.argv[sys.argv.index("--variant") + 1]
    if variant not in PALETTES:
        print(f"no banner palette for {variant}; have {', '.join(PALETTES)}")
        return 1

    palette = PALETTES[variant] or PALETTES["catppuccin"]
    icon_png = REPO_ROOT / f"app/assets/bundled/png/nerminal-{variant}.png"
    if not icon_png.exists():
        print(f"missing {icon_png}, run script/icons/install_app_icons.py first")
        return 1

    out = sys.argv[sys.argv.index("--out") + 1] if "--out" in sys.argv else OUTPUT

    # librsvg refuses to follow a file reference out of an SVG, so the icon
    # travels inside the document.
    encoded = base64.b64encode(icon_png.read_bytes()).decode("ascii")
    # The SVG is an intermediate, not a source: it carries the icon inline as
    # base64 and would be rewritten wholesale on every run. Keep it out of the
    # repository so the history holds the script and the result, not the blob.
    with tempfile.TemporaryDirectory() as tmp:
        svg = pathlib.Path(tmp) / "banner.svg"
        svg.write_text(
            banner(codename, palette, f"data:image/png;base64,{encoded}")
        )
        subprocess.run(
            ["/opt/homebrew/bin/rsvg-convert", "-w", str(WIDTH), "-h", str(HEIGHT),
             str(svg), "-o", str(out)],
            check=True,
        )
    print(f"  {out}  ({codename}, {variant})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
