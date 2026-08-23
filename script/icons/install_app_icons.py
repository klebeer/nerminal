#!/usr/bin/env python3
"""Render the app icons and put every copy where the build expects it.

`generate_app_icons.py` draws the SVGs and rasterises them into a directory of
your choosing. Getting from there into a build takes three more steps that used
to be done by hand, which is how they end up half-done:

  1. every variant becomes a bundled PNG, so the icon picker can offer it
  2. the default variant also becomes the app's own icon, which on macOS is the
     512px PNG named in `app/Cargo.toml` under `package.metadata.bundle`
  3. and the Windows `.ico`, which is not one image but six resolutions in one
     container, and cannot be produced by `rsvg-convert` alone

Usage:

    script/icons/install_app_icons.py                 # default variant, all sizes
    script/icons/install_app_icons.py --variant dawn  # a different app icon
    script/icons/install_app_icons.py --dry-run

Requires `rsvg-convert` (brew install librsvg).
"""

import argparse
import pathlib
import struct
import subprocess
import sys
import tempfile

import generate_app_icons

ROOT = pathlib.Path(__file__).resolve().parent.parent.parent
BUNDLED_PNG = ROOT / "app/assets/bundled/png"
CHANNEL_ICON = ROOT / "app/channels/oss/icon"

RSVG = "/opt/homebrew/bin/rsvg-convert"

# The sizes Windows Explorer picks between. Below 32 the drawing has to survive
# losing its detail, which is worth checking by eye after a redesign.
ICO_SIZES = (16, 32, 48, 64, 128, 256)

# The variant the app itself wears. Must match `AppIcon`'s `#[default]` in
# app/src/settings/app_icon.rs, or the dock shows one icon and the picker
# claims another.
DEFAULT_VARIANT = "catppuccin"


def render(svg: pathlib.Path, out: pathlib.Path, size: int) -> None:
    subprocess.run(
        [RSVG, "-w", str(size), "-h", str(size), str(svg), "-o", str(out)],
        check=True,
    )


def pack_ico(svg: pathlib.Path, dest: pathlib.Path, work: pathlib.Path) -> None:
    """Build a multi-resolution .ico.

    The format is a small header, one 16-byte directory entry per image, then
    the images back to back. Each entry stores the offset of its own image, so
    the offsets have to be accumulated as the blob is assembled. A side of 256
    is written as 0, since the field is a single byte.
    """
    images = []
    for size in ICO_SIZES:
        png = work / f"{size}.png"
        render(svg, png, size)
        images.append((size, png.read_bytes()))

    header = struct.pack("<HHH", 0, 1, len(images))
    offset = len(header) + 16 * len(images)
    entries, blob = bytearray(), bytearray()
    for size, data in images:
        side = 0 if size == 256 else size
        entries += struct.pack(
            "<BBBBHHII", side, side, 0, 0, 1, 32, len(data), offset
        )
        blob += data
        offset += len(data)

    dest.write_bytes(header + bytes(entries) + bytes(blob))


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--variant", default=DEFAULT_VARIANT,
                        help=f"which variant the app wears (default: {DEFAULT_VARIANT})")
    parser.add_argument("--dry-run", action="store_true")
    args = parser.parse_args()

    if args.variant not in generate_app_icons.VARIANTS:
        known = ", ".join(sorted(generate_app_icons.VARIANTS))
        print(f"unknown variant {args.variant!r}. known: {known}", file=sys.stderr)
        return 1

    if not pathlib.Path(RSVG).exists():
        print(f"{RSVG} not found; brew install librsvg", file=sys.stderr)
        return 1

    with tempfile.TemporaryDirectory() as tmp:
        work = pathlib.Path(tmp)

        for name, draw in generate_app_icons.VARIANTS.items():
            svg = work / f"{name}.svg"
            svg.write_text(draw())
            dest = BUNDLED_PNG / f"nerminal-{name}.png"
            if args.dry_run:
                print(f"  would write {dest.relative_to(ROOT)}")
            else:
                render(svg, dest, 512)
                print(f"  {dest.relative_to(ROOT)}")

        svg = work / f"{args.variant}.svg"
        targets = [
            (CHANNEL_ICON / "nerminal-icon.svg", "svg"),
            (CHANNEL_ICON / "no-padding/512x512.png", "png"),
            (CHANNEL_ICON / "no-padding/icon.ico", "ico"),
        ]
        print(f"\n  app icon, from '{args.variant}':")
        for dest, kind in targets:
            if args.dry_run:
                print(f"  would write {dest.relative_to(ROOT)}")
                continue
            if kind == "svg":
                dest.write_text(svg.read_text())
            elif kind == "png":
                render(svg, dest, 512)
            else:
                pack_ico(svg, dest, work)
            print(f"  {dest.relative_to(ROOT)}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
