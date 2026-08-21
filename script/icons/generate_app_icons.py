#!/usr/bin/env python3
"""Generate the Nerminal alternate app icons.

Every variant keeps the same identity: a CRT whose screen is a nerd face, with
thick taped glasses for eyes and a shell prompt for a mouth. Only the era
treatment changes. Homage to the genre, never to anyone's marks or characters.
"""

import pathlib
import subprocess
import sys

W = 512
FRAME = '<rect x="10" y="10" width="492" height="492" rx="112" ry="112"/>'


def head(defs: str) -> str:
    return (
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{W}" '
        f'viewBox="0 0 {W} {W}">\n<defs>\n{defs}\n'
        f'<clipPath id="frame">{FRAME}</clipPath>\n</defs>\n<g clip-path="url(#frame)">\n'
    )


TAIL = "</g>\n</svg>\n"


def glasses(lens_fill, rim, bridge, shine="#eafcff", shine_op="0.55", r=44,
            cx1=190, cx2=322, cy=204, rim_w=15):
    """The eyes: two thick lenses with a taped bridge and a diagonal glare."""
    return f'''
      <g>
        <circle cx="{cx1}" cy="{cy}" r="{r}" fill="{lens_fill}"/>
        <circle cx="{cx2}" cy="{cy}" r="{r}" fill="{lens_fill}"/>
        <g fill="none" stroke="{rim}" stroke-width="{rim_w}">
          <circle cx="{cx1}" cy="{cy}" r="{r}"/>
          <circle cx="{cx2}" cy="{cy}" r="{r}"/>
          <path d="M{cx1 + r} {cy - 4} q22 -14 44 0"/>
          <path d="M{cx1 - r - 2} {cy - 8} l-30 -14"/>
          <path d="M{cx2 + r + 2} {cy - 8} l30 -14"/>
        </g>
        <rect x="243" y="{cy - 26}" width="26" height="30" rx="4" fill="{bridge}"
              opacity="0.92" transform="rotate(-12 256 {cy - 11})"/>
        <g stroke="{shine}" stroke-width="9" stroke-linecap="round" opacity="{shine_op}">
          <line x1="{cx1 - 20}" y1="{cy - 12}" x2="{cx1 + 2}" y2="{cy - 28}"/>
          <line x1="{cx2 - 20}" y1="{cy - 12}" x2="{cx2 + 2}" y2="{cy - 28}"/>
        </g>
      </g>'''


def prompt_mouth(colour, w=13):
    """The mouth: a shell prompt chevron and underscore."""
    return f'''
      <g stroke="{colour}" stroke-width="{w}" stroke-linecap="round"
         stroke-linejoin="round" fill="none">
        <polyline points="212,258 240,278 212,298"/>
      </g>
      <rect x="258" y="288" width="46" height="12" rx="6" fill="{colour}"/>'''


def horns_mouth(colour, w=14):
    """Metal variant mouth: \\m/ ."""
    return f'''
      <g stroke="{colour}" stroke-width="{w}" stroke-linecap="round"
         stroke-linejoin="round" fill="none">
        <line x1="196" y1="256" x2="212" y2="300"/>
        <polyline points="228,300 228,266 246,292 264,266 264,300"/>
        <line x1="300" y1="300" x2="316" y2="256"/>
      </g>'''


def scanlines(x, y, w, h, opacity="0.06", pitch=10):
    rows = "".join(
        f'<rect x="{x}" y="{yy}" width="{w}" height="3"/>'
        for yy in range(y + 4, y + h, pitch)
    )
    return f'<g fill="#ffffff" opacity="{opacity}">{rows}</g>'


def crt(body_fill, rim, rim_w=8, screen_fill="#0a0716"):
    return f'''
    <path d="M226 322 h60 l14 60 h-88 z" fill="{body_fill}"/>
    <rect x="176" y="378" width="160" height="22" rx="11" fill="{body_fill}"
          stroke="{rim}" stroke-width="5"/>
    <rect x="80" y="112" width="352" height="230" rx="36" fill="{body_fill}"
          stroke="{rim}" stroke-width="{rim_w}"/>
    <rect x="116" y="146" width="280" height="168" rx="20" fill="{screen_fill}"/>'''


def synthwave_bg():
    verticals = "".join(
        f'<line x1="256" y1="342" x2="{x}" y2="512"/>'
        for x in (-240, -60, 76, 176, 256, 336, 436, 572, 752)
    )
    horizontals = "".join(
        f'<line x1="0" y1="{y}" x2="512" y2="{y}"/>' for y in (356, 376, 404, 442, 492)
    )
    return f'''
    <rect x="0" y="0" width="512" height="512" fill="url(#sky)"/>
    <g fill="#ffffff" opacity="0.75">
      <circle cx="64" cy="58" r="2.6"/><circle cx="132" cy="104" r="1.8"/>
      <circle cx="418" cy="72" r="2.4"/><circle cx="464" cy="132" r="1.6"/>
      <circle cx="352" cy="44" r="1.7"/><circle cx="228" cy="40" r="2.0"/>
    </g>
    <rect x="0" y="338" width="512" height="5" fill="#00e5ff" opacity="0.9"/>
    <g clip-path="url(#below)" stroke="#00e5ff" fill="none" opacity="0.6">
      <g stroke-width="3">{verticals}</g>
      <g stroke-width="3.5">{horizontals}</g>
    </g>'''


SKY = ('<linearGradient id="sky" x1="0" y1="0" x2="0" y2="1">'
       '<stop offset="0%" stop-color="#150733"/><stop offset="38%" stop-color="#3a1170"/>'
       '<stop offset="66%" stop-color="#7c1a86"/><stop offset="86%" stop-color="#e0257f"/>'
       '<stop offset="100%" stop-color="#ff7a3d"/></linearGradient>')
BELOW = '<clipPath id="below"><rect x="0" y="342" width="512" height="170"/></clipPath>'


def variant_metal():
    defs = ('<linearGradient id="chrome" x1="0" y1="0" x2="0" y2="1">'
            '<stop offset="0%" stop-color="#e9edf2"/><stop offset="45%" stop-color="#8c95a3"/>'
            '<stop offset="55%" stop-color="#3c424d"/><stop offset="100%" stop-color="#9aa3b1"/>'
            '</linearGradient>')
    studs = "".join(
        f'<circle cx="{x}" cy="128" r="6" fill="url(#chrome)"/>'
        f'<circle cx="{x}" cy="326" r="6" fill="url(#chrome)"/>'
        for x in range(112, 421, 44)
    )
    return head(defs) + f'''
    <rect x="0" y="0" width="512" height="512" fill="#0a0a0d"/>
    <g stroke="#1c1f26" stroke-width="2">
      {"".join(f'<line x1="0" y1="{y}" x2="512" y2="{y}"/>' for y in range(0, 512, 16))}
    </g>
    <path d="M300 40 l-150 210 h96 l-58 222 175 -250 h-104 z" fill="#f5f7fa" opacity="0.10"/>
    {crt("#16181d", "url(#chrome)", 9, "#050507")}
    {studs}
    {scanlines(116, 146, 280, 168, "0.05")}
    {glasses("#101318", "#040406", "#f4f1e4", "#ffffff", "0.5")}
    {horns_mouth("#e9edf2")}
    <circle cx="396" cy="328" r="7" fill="#ff2b2b"/>
''' + TAIL


def variant_arcade():
    px = 16
    pixels = []
    for i, (x, y, c) in enumerate([
        (0, 0, "#ff004d"), (1, 0, "#ffa300"), (2, 0, "#ffec27"),
        (0, 1, "#00e436"), (1, 1, "#29adff"), (2, 1, "#83769c"),
    ]):
        pixels.append(f'<rect x="{40 + x * px}" y="{40 + y * px}" width="{px}" height="{px}" fill="{c}"/>')
    return head("") + f'''
    <rect x="0" y="0" width="512" height="512" fill="#1d2b53"/>
    <g opacity="0.28">
      {"".join(f'<rect x="{x}" y="0" width="{px}" height="512" fill="#29adff" opacity="0.06"/>' for x in range(0, 512, px * 2))}
    </g>
    {"".join(pixels)}
    {crt("#7e2553", "#ffec27", 10, "#000000")}
    {scanlines(116, 146, 280, 168, "0.10", 8)}
    {glasses("#00221a", "#000000", "#fff1e8", "#ffffff", "0.65")}
    {prompt_mouth("#00e436", 15)}
    <circle cx="396" cy="328" r="8" fill="#ffec27"/>
''' + TAIL


def variant_cartridge():
    ridges = "".join(
        f'<rect x="{x}" y="392" width="14" height="30" fill="#2a2f3a"/>'
        for x in range(150, 361, 30)
    )
    return head("") + f'''
    <rect x="0" y="0" width="512" height="512" fill="#20242e"/>
    <rect x="96" y="60" width="320" height="392" rx="26" fill="#4a5160"/>
    <rect x="96" y="60" width="320" height="392" rx="26" fill="none"
          stroke="#6d7686" stroke-width="6"/>
    <rect x="128" y="96" width="256" height="200" rx="12" fill="#e8e4d8"/>
    <rect x="128" y="96" width="256" height="34" rx="12" fill="#ff2d95"/>
    <rect x="152" y="146" width="208" height="126" rx="10" fill="#12141b"/>
    {scanlines(152, 146, 208, 126, "0.07", 9)}
    {glasses("#0c2b26", "#07060f", "#f4f1e4", "#eafcff", "0.5", 32, 206, 306, 196, 11)}
    <g stroke="#39ff9c" stroke-width="10" stroke-linecap="round" stroke-linejoin="round" fill="none">
      <polyline points="222,232 244,248 222,264"/>
    </g>
    <rect x="256" y="256" width="36" height="10" rx="5" fill="#39ff9c"/>
    <rect x="128" y="330" width="256" height="46" rx="8" fill="#39404e"/>
    {ridges}
''' + TAIL


def variant_mixtape():
    return head("") + f'''
    <rect x="0" y="0" width="512" height="512" fill="#141019"/>
    <rect x="56" y="132" width="400" height="248" rx="22" fill="#2b2233"
          stroke="#ff2d95" stroke-width="7"/>
    <rect x="88" y="160" width="336" height="80" rx="8" fill="#f0ead6"/>
    <rect x="88" y="160" width="336" height="26" rx="8" fill="#ffb400"/>
    <rect x="112" y="268" width="288" height="86" rx="12" fill="#12141b"/>
    {scanlines(112, 268, 288, 86, "0.06", 9)}
    <g>
      <circle cx="188" cy="311" r="38" fill="#0c2b26"/>
      <circle cx="324" cy="311" r="38" fill="#0c2b26"/>
      <g fill="none" stroke="#07060f" stroke-width="13">
        <circle cx="188" cy="311" r="38"/><circle cx="324" cy="311" r="38"/>
        <path d="M226 307 q30 -12 60 0"/>
      </g>
      <g fill="#3a3f4a">
        <circle cx="188" cy="311" r="13"/><circle cx="324" cy="311" r="13"/>
      </g>
      <g stroke="#eafcff" stroke-width="8" stroke-linecap="round" opacity="0.5">
        <line x1="170" y1="300" x2="188" y2="288"/>
        <line x1="306" y1="300" x2="324" y2="288"/>
      </g>
      <rect x="243" y="286" width="26" height="28" rx="4" fill="#f4f1e4"
            opacity="0.92" transform="rotate(-12 256 300)"/>
    </g>
    <g stroke="#39ff9c" stroke-width="11" stroke-linecap="round" fill="none">
      <line x1="150" y1="404" x2="362" y2="404"/>
    </g>
''' + TAIL


def variant_neon():
    return head("") + f'''
    <rect x="0" y="0" width="512" height="512" fill="#05060a"/>
    <g stroke="#1a2340" stroke-width="2">
      {"".join(f'<line x1="{x}" y1="0" x2="{x}" y2="512"/>' for x in range(0, 512, 32))}
      {"".join(f'<line x1="0" y1="{y}" x2="512" y2="{y}"/>' for y in range(0, 512, 32))}
    </g>
    <g fill="none" stroke="#ff2d95" stroke-width="10" opacity="0.35">
      <rect x="80" y="112" width="352" height="230" rx="36"/>
    </g>
    <g fill="none" stroke="#ff2d95" stroke-width="5">
      <rect x="80" y="112" width="352" height="230" rx="36"/>
      <path d="M226 322 h60 l14 60 h-88 z"/>
      <rect x="176" y="378" width="160" height="22" rx="11"/>
    </g>
    <g fill="none" stroke="#00e5ff" stroke-width="4" opacity="0.9">
      <circle cx="190" cy="204" r="44"/><circle cx="322" cy="204" r="44"/>
      <path d="M234 200 q22 -14 44 0"/>
      <path d="M146 196 l-30 -14"/><path d="M366 196 l30 -14"/>
    </g>
    <g stroke="#39ff9c" stroke-width="11" stroke-linecap="round"
       stroke-linejoin="round" fill="none">
      <polyline points="212,258 240,278 212,298"/>
    </g>
    <rect x="258" y="288" width="46" height="11" rx="6" fill="#39ff9c"/>
''' + TAIL


def variant_vhs():
    bars = "".join(
        f'<rect x="0" y="{y}" width="512" height="{h}" fill="#ffffff" opacity="0.05"/>'
        for y, h in ((150, 6), (238, 4), (300, 8), (392, 5))
    )
    return head("") + f'''
    <rect x="0" y="0" width="512" height="512" fill="#0e1016"/>
    {bars}
    <g opacity="0.55">
      <rect x="84" y="116" width="352" height="230" rx="36" fill="none"
            stroke="#ff2d95" stroke-width="8"/>
    </g>
    <g opacity="0.55">
      <rect x="76" y="108" width="352" height="230" rx="36" fill="none"
            stroke="#00e5ff" stroke-width="8"/>
    </g>
    {crt("#191c24", "#d8dde6", 6, "#07080d")}
    {scanlines(116, 146, 280, 168, "0.09", 7)}
    {glasses("#0c2b26", "#07060f", "#f4f1e4", "#eafcff", "0.5")}
    {prompt_mouth("#39ff9c")}
    <g fill="#ff2d95">
      <polygon points="132,168 132,190 152,179"/>
    </g>
    <rect x="162" y="172" width="58" height="12" rx="3" fill="#ffffff" opacity="0.65"/>
''' + TAIL


VARIANTS = {
    "metal": variant_metal,
    "arcade": variant_arcade,
    "cartridge": variant_cartridge,
    "mixtape": variant_mixtape,
    "neon": variant_neon,
    "vhs": variant_vhs,
}


def main() -> int:
    out = pathlib.Path(sys.argv[1])
    out.mkdir(parents=True, exist_ok=True)
    for name, fn in VARIANTS.items():
        svg = out / f"{name}.svg"
        svg.write_text(fn())
        subprocess.run(
            ["/opt/homebrew/bin/rsvg-convert", "-w", "512", "-h", "512",
             str(svg), "-o", str(out / f"{name}.png")],
            check=True,
        )
        print(f"  {name}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
