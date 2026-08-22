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


def fangs(colour="#fdf6ff", tip="#c2143c"):
    """Two canines hanging off the prompt mouth, tipped red."""
    return f'''
      <g fill="{colour}">
        <path d="M262 300 h20 l-10 34 z"/>
        <path d="M292 300 h20 l-10 26 z"/>
      </g>
      <g fill="{tip}">
        <path d="M266 322 h12 l-6 12 z"/>
        <path d="M296 316 h12 l-6 10 z"/>
      </g>'''


def blood_drip(x, y, colour="#c2143c"):
    """A bead of blood about to fall."""
    return (f'<g fill="{colour}"><path d="M{x} {y} c 10 14 10 22 0 22 '
            f'c -10 0 -10 -8 0 -22 z"/>'
            f'<circle cx="{x}" cy="{y + 40}" r="5" opacity="0.8"/></g>')


def bat(cx, cy, s, fill, opacity="1"):
    """A small bat in silhouette, drawn from the centre outwards."""
    return (f'<g transform="translate({cx} {cy}) scale({s})" fill="{fill}" '
            f'opacity="{opacity}">'
            '<path d="M0 -6 c 6 0 9 4 9 9 c 0 5 -3 9 -9 9 c -6 0 -9 -4 -9 -9 '
            'c 0 -5 3 -9 9 -9 z"/>'
            '<path d="M-8 -2 c -14 -12 -28 -12 -40 -4 c 10 0 12 6 10 12 '
            'c 8 -4 14 -2 18 4 c 4 -6 8 -10 12 -12 z"/>'
            '<path d="M8 -2 c 14 -12 28 -12 40 -4 c -10 0 -12 6 -10 12 '
            'c -8 -4 -14 -2 -18 4 c -4 -6 -8 -10 -12 -12 z"/>'
            '<path d="M-5 -7 l -3 -8 l 6 4 z"/><path d="M5 -7 l 3 -8 l -6 4 z"/>'
            '</g>')


def pointed_ears(fill, rim):
    """Ears on the CRT shell. What makes the box read as a vampire."""
    return (f'<g fill="{fill}" stroke="{rim}" stroke-width="6" '
            'stroke-linejoin="round">'
            '<path d="M96 138 l -34 -78 l 76 34 z"/>'
            '<path d="M416 138 l 34 -78 l -76 34 z"/></g>')


def variant_nosferatu():
    """Count Orlok, 1922, public domain. The tell is the pair of central
    incisors and the ears, not the side canines every other vampire has.
    The sunken eyes are lit like two small screens, which is the only
    liberty taken."""
    defs = (
        '<linearGradient id="pallor" x1="0" y1="0" x2="0" y2="1">'
        '<stop offset="0%" stop-color="#c8cbb4"/>'
        '<stop offset="55%" stop-color="#8e9479"/>'
        '<stop offset="100%" stop-color="#4d5340"/></linearGradient>'
        '<radialGradient id="moonglow" cx="0.5" cy="0.28" r="0.75">'
        '<stop offset="0%" stop-color="#9fb4a0" stop-opacity="0.30"/>'
        '<stop offset="100%" stop-color="#9fb4a0" stop-opacity="0"/>'
        '</radialGradient>'
        '<radialGradient id="socket" cx="0.5" cy="0.5" r="0.5">'
        '<stop offset="0%" stop-color="#7ce0b0"/>'
        '<stop offset="55%" stop-color="#1f6b4a"/>'
        '<stop offset="100%" stop-color="#050806"/></radialGradient>'
        '<linearGradient id="wall" x1="0" y1="0" x2="0" y2="1">'
        '<stop offset="0%" stop-color="#151a13"/>'
        '<stop offset="100%" stop-color="#080a07"/></linearGradient>'
    )
    return (
        head(defs)
        + '<rect x="0" y="0" width="512" height="512" fill="url(#wall)"/>'
        + '<rect x="0" y="0" width="512" height="512" fill="url(#moonglow)"/>'
        # the shadow he casts on the wall, offset and cheap on detail
        + '<g fill="#000000" opacity="0.5">'
          '<path d="M330 512 C 322 380 340 300 372 250 '
          'C 386 228 392 196 386 168 L 418 92 L 400 176 '
          'C 424 196 430 240 418 286 C 446 340 452 430 448 512 z"/></g>'
        # shoulders and the high collar
        + '<path d="M112 512 C 118 430 156 388 200 372 '
          'L 312 372 C 356 388 394 430 400 512 z" fill="#10140e"/>'
        + '<path d="M196 376 L 256 424 L 316 376 '
          'C 300 356 292 336 290 312 L 222 312 '
          'C 220 336 212 356 196 376 z" fill="url(#pallor)"/>'
        # the skull: tall, domed, narrowing hard to the jaw
        + '<path d="M256 62 C 316 62 348 110 350 178 '
          'C 352 232 344 276 328 306 C 312 336 288 350 256 350 '
          'C 224 350 200 336 184 306 C 168 276 160 232 162 178 '
          'C 164 110 196 62 256 62 z" fill="url(#pallor)"/>'
        # ears, set high and angled back
        + '<g fill="url(#pallor)">'
          '<path d="M168 156 C 132 126 108 96 100 62 '
          'C 132 78 156 104 170 132 z"/>'
          '<path d="M344 156 C 380 126 404 96 412 62 '
          'C 380 78 356 104 342 132 z"/></g>'
        + '<g fill="#000000" opacity="0.35">'
          '<path d="M168 152 C 142 130 124 108 114 84 '
          'C 138 98 156 118 168 138 z"/>'
          '<path d="M344 152 C 370 130 388 108 398 84 '
          'C 374 98 356 118 344 138 z"/></g>'
        # heavy brow
        + '<path d="M186 176 C 210 162 302 162 326 176 '
          'C 314 190 198 190 186 176 z" fill="#000000" opacity="0.45"/>'
        # the eyes, lit like little screens
        + '<g>'
          '<ellipse cx="212" cy="206" rx="34" ry="26" fill="#050806"/>'
          '<ellipse cx="300" cy="206" rx="34" ry="26" fill="#050806"/>'
          '<ellipse cx="212" cy="206" rx="22" ry="16" fill="url(#socket)"/>'
          '<ellipse cx="300" cy="206" rx="22" ry="16" fill="url(#socket)"/>'
          '<g fill="#0a0d08" opacity="0.5">'
          '<rect x="190" y="196" width="44" height="2.5"/>'
          '<rect x="190" y="206" width="44" height="2.5"/>'
          '<rect x="190" y="216" width="44" height="2.5"/>'
          '<rect x="278" y="196" width="44" height="2.5"/>'
          '<rect x="278" y="206" width="44" height="2.5"/>'
          '<rect x="278" y="216" width="44" height="2.5"/></g></g>'
        # hollow cheeks
        + '<g fill="#000000" opacity="0.28">'
          '<path d="M180 232 C 194 262 200 288 196 312 '
          'C 180 288 174 260 176 234 z"/>'
          '<path d="M332 232 C 318 262 312 288 316 312 '
          'C 332 288 338 260 336 234 z"/></g>'
        # nose
        + '<path d="M256 224 L 268 274 L 256 282 L 244 274 z" '
          'fill="#000000" opacity="0.3"/>'
        # mouth with the two central incisors
        + '<path d="M226 300 C 240 292 272 292 286 300 '
          'C 272 312 240 312 226 300 z" fill="#12150f"/>'
        + '<g fill="#eef2e2">'
          '<path d="M246 302 h11 l-5.5 26 z"/>'
          '<path d="M259 302 h11 l-5.5 26 z"/></g>'
        # a single long-fingered hand reaching up at the edge
        + '<g fill="url(#pallor)" opacity="0.95">'
          '<path d="M60 512 C 56 470 60 440 70 420 '
          'C 76 408 88 404 96 412 C 102 418 102 430 98 442 '
          'L 104 400 C 106 388 118 384 126 390 C 132 395 133 404 131 414 '
          'L 136 396 C 139 384 150 381 157 387 C 163 392 164 401 162 411 '
          'L 166 402 C 169 391 180 389 186 395 C 191 400 192 408 190 417 '
          'C 182 452 176 482 176 512 z"/></g>'
        + scanlines(0, 0, 512, 512, "0.045", 8)
        + '<rect x="0" y="0" width="512" height="512" fill="url(#moonglow)" '
          'opacity="0.35"/>'
        + TAIL
    )


def orlok_head(pallor="url(#pallor)", shade="0.35"):
    """The head, ears and hollows. Shared by every variant in the family."""
    return (
        '<path d="M256 62 C 316 62 348 110 350 178 '
        'C 352 232 344 276 328 306 C 312 336 288 350 256 350 '
        'C 224 350 200 336 184 306 C 168 276 160 232 162 178 '
        f'C 164 110 196 62 256 62 z" fill="{pallor}"/>'
        f'<g fill="{pallor}">'
        '<path d="M168 156 C 132 126 108 96 100 62 '
        'C 132 78 156 104 170 132 z"/>'
        '<path d="M344 156 C 380 126 404 96 412 62 '
        'C 380 78 356 104 342 132 z"/></g>'
        f'<g fill="#000000" opacity="{shade}">'
        '<path d="M168 152 C 142 130 124 108 114 84 '
        'C 138 98 156 118 168 138 z"/>'
        '<path d="M344 152 C 370 130 388 108 398 84 '
        'C 374 98 356 118 344 138 z"/></g>'
        '<path d="M186 176 C 210 162 302 162 326 176 '
        'C 314 190 198 190 186 176 z" fill="#000000" opacity="0.45"/>'
        '<g fill="#000000" opacity="0.28">'
        '<path d="M180 232 C 194 262 200 288 196 312 '
        'C 180 288 174 260 176 234 z"/>'
        '<path d="M332 232 C 318 262 312 288 316 312 '
        'C 332 288 338 260 336 234 z"/></g>'
        '<path d="M256 224 L 268 274 L 256 282 L 244 274 z" '
        'fill="#000000" opacity="0.3"/>'
    )


def screen_eyes(glow="url(#socket)"):
    """His sunken eyes, lit like two small screens."""
    return (
        '<ellipse cx="212" cy="206" rx="34" ry="26" fill="#050806"/>'
        '<ellipse cx="300" cy="206" rx="34" ry="26" fill="#050806"/>'
        f'<ellipse cx="212" cy="206" rx="22" ry="16" fill="{glow}"/>'
        f'<ellipse cx="300" cy="206" rx="22" ry="16" fill="{glow}"/>'
        '<g fill="#0a0d08" opacity="0.5">'
        '<rect x="190" y="196" width="44" height="2.5"/>'
        '<rect x="190" y="206" width="44" height="2.5"/>'
        '<rect x="190" y="216" width="44" height="2.5"/>'
        '<rect x="278" y="196" width="44" height="2.5"/>'
        '<rect x="278" y="206" width="44" height="2.5"/>'
        '<rect x="278" y="216" width="44" height="2.5"/></g>'
    )


def incisors():
    """Two central teeth. The tell that this is Orlok and not a Dracula."""
    return (
        '<path d="M226 300 C 240 292 272 292 286 300 '
        'C 272 312 240 312 226 300 z" fill="#12150f"/>'
        '<g fill="#eef2e2">'
        '<path d="M246 302 h11 l-5.5 26 z"/>'
        '<path d="M259 302 h11 l-5.5 26 z"/></g>'
    )


def collar(fill="#10140e", pallor="url(#pallor)"):
    return (
        f'<path d="M112 512 C 118 430 156 388 200 372 '
        f'L 312 372 C 356 388 394 430 400 512 z" fill="{fill}"/>'
        f'<path d="M196 376 L 256 424 L 316 376 '
        f'C 300 356 292 336 290 312 L 222 312 '
        f'C 220 336 212 356 196 376 z" fill="{pallor}"/>'
    )


PALLOR = ('<linearGradient id="pallor" x1="0" y1="0" x2="0" y2="1">'
          '<stop offset="0%" stop-color="#c8cbb4"/>'
          '<stop offset="55%" stop-color="#8e9479"/>'
          '<stop offset="100%" stop-color="#4d5340"/></linearGradient>')
SOCKET = ('<radialGradient id="socket" cx="0.5" cy="0.5" r="0.5">'
          '<stop offset="0%" stop-color="#7ce0b0"/>'
          '<stop offset="55%" stop-color="#1f6b4a"/>'
          '<stop offset="100%" stop-color="#050806"/></radialGradient>')


def orlok(bg, extras_back="", extras_front="", socket=SOCKET, glow="url(#socket)"):
    """Every icon in the family is the same portrait on a different night."""
    return (
        head(PALLOR + socket + bg[0])
        + bg[1] + extras_back
        + collar() + orlok_head() + screen_eyes(glow) + incisors()
        + extras_front
        + scanlines(0, 0, 512, 512, "0.045", 8)
        + TAIL
    )


def variant_shadow():
    """Him as the shadow on the stairwell, which is how everyone remembers it."""
    bg = ('<linearGradient id="wall" x1="0" y1="0" x2="0" y2="1">'
          '<stop offset="0%" stop-color="#2b2f22"/>'
          '<stop offset="100%" stop-color="#0a0c07"/></linearGradient>',
          '<rect x="0" y="0" width="512" height="512" fill="url(#wall)"/>'
          '<g fill="#000000" opacity="0.42">'
          '<path d="M300 512 C 292 380 312 300 344 250 '
          'C 358 228 364 196 358 168 L 392 88 L 372 176 '
          'C 396 196 402 240 390 286 C 418 340 424 430 420 512 z"/>'
          '<path d="M96 512 C 92 462 98 428 110 406 '
          'C 118 392 132 390 138 400 L 146 380 '
          'C 152 366 166 366 170 378 L 176 366 '
          'C 182 354 194 356 196 368 C 190 412 184 462 184 512 z"/></g>')
    return orlok(bg)


def variant_plague():
    """The ship, the rats and the fever. Sickest of the set."""
    bg = ('<radialGradient id="fever" cx="0.5" cy="0.4" r="0.8">'
          '<stop offset="0%" stop-color="#4e6b2e" stop-opacity="0.5"/>'
          '<stop offset="100%" stop-color="#4e6b2e" stop-opacity="0"/>'
          '</radialGradient>',
          '<rect x="0" y="0" width="512" height="512" fill="#0a0f07"/>'
          '<rect x="0" y="0" width="512" height="512" fill="url(#fever)"/>'
          + bat(88, 96, 0.6, "#1a2410") + bat(424, 128, 0.45, "#1a2410"))
    front = ('<g fill="#0d1208" opacity="0.9">'
             '<ellipse cx="120" cy="474" rx="26" ry="14"/>'
             '<path d="M144 474 c 26 -6 42 4 58 14 c -20 -2 -34 -2 -58 -6 z"/>'
             '<circle cx="100" cy="466" r="3"/>'
             '<ellipse cx="386" cy="486" rx="20" ry="11"/>'
             '<path d="M404 486 c 20 -5 32 3 44 11 c -16 -2 -26 -2 -44 -5 z"/>'
             '</g>')
    return orlok(bg, extras_front=front)


def variant_dawn():
    """First light, which is the only thing that ever stopped him."""
    bg = ('<linearGradient id="dawn" x1="0" y1="1" x2="0" y2="0">'
          '<stop offset="0%" stop-color="#f2c078"/>'
          '<stop offset="30%" stop-color="#c76b4a"/>'
          '<stop offset="62%" stop-color="#4a3352"/>'
          '<stop offset="100%" stop-color="#121026"/></linearGradient>',
          '<rect x="0" y="0" width="512" height="512" fill="url(#dawn)"/>'
          '<circle cx="256" cy="470" r="120" fill="#ffd9a0" opacity="0.55"/>')
    front = ('<g fill="#ffd9a0" opacity="0.30">'
             '<path d="M162 300 C 200 340 312 340 350 300 '
             'C 330 360 300 400 256 420 C 212 400 182 360 162 300 z"/></g>')
    glow = '#e8c07a'
    return orlok(bg, extras_front=front, socket="", glow=glow)


def variant_castle():
    """The place he came from, on the ridge above everything."""
    bg = ('<linearGradient id="ridge" x1="0" y1="0" x2="0" y2="1">'
          '<stop offset="0%" stop-color="#141a1e"/>'
          '<stop offset="100%" stop-color="#050708"/></linearGradient>',
          '<rect x="0" y="0" width="512" height="512" fill="url(#ridge)"/>'
          '<circle cx="384" cy="112" r="66" fill="#cfd8c4" opacity="0.22"/>'
          '<g fill="#02040a" opacity="0.95">'
          '<path d="M0 512 L 0 400 L 40 400 L 40 368 L 60 368 L 60 400 '
          'L 92 400 L 92 340 L 112 340 L 112 400 L 150 400 L 150 512 z"/>'
          '<path d="M362 512 L 362 356 L 380 356 L 380 330 L 396 330 '
          'L 396 356 L 424 356 L 424 300 L 442 300 L 442 356 L 470 356 '
          'L 470 512 z"/></g>'
          + bat(150, 150, 0.5, "#02040a", "0.8"))
    return orlok(bg)


def variant_fangs():
    """The plain nerd face, grown a set of canines."""
    defs = ('<linearGradient id="fsky" x1="0" y1="0" x2="0" y2="1">'
            '<stop offset="0%" stop-color="#1a0510"/>'
            '<stop offset="100%" stop-color="#4a0c22"/></linearGradient>')
    return (
        head(defs)
        + '<rect x="0" y="0" width="512" height="512" fill="url(#fsky)"/>'
        + bat(96, 78, 0.7, "#ff2d55", "0.5") + bat(410, 116, 0.5, "#ff2d55", "0.35")
        + crt("#22101a", "#ff2d55", 8, "#120309")
        + scanlines(116, 146, 280, 168, "0.06")
        + glasses("#120309", "#ff2d55", "#ffd166", "#ffe8ee", "0.5")
        + prompt_mouth("#ff2d55")
        + fangs("#fff5f7", "#c2143c")
        + blood_drip(268, 338)
        + TAIL
    )


def variant_cape():
    """A high collar rising behind the box, crimson on the inside."""
    defs = ('<linearGradient id="lining" x1="0" y1="0" x2="0" y2="1">'
            '<stop offset="0%" stop-color="#e01b3c"/>'
            '<stop offset="100%" stop-color="#6b0618"/></linearGradient>')
    return (
        head(defs)
        + '<rect x="0" y="0" width="512" height="512" fill="#120a14"/>'
        + '<path d="M256 400 C 120 400 44 300 40 96 '
          'C 120 150 180 168 256 172 C 332 168 392 150 472 96 '
          'C 468 300 392 400 256 400 z" fill="url(#lining)"/>'
        + '<path d="M256 396 C 132 396 62 302 58 118 '
          'C 130 168 186 184 256 188 C 326 184 382 168 454 118 '
          'C 450 302 380 396 256 396 z" fill="#0b060d"/>'
        + crt("#241528", "#8a1533", 8, "#0a050c")
        + scanlines(116, 146, 280, 168, "0.05")
        + glasses("#0a050c", "#8a1533", "#e01b3c", "#ffd9e2", "0.45")
        + prompt_mouth("#e01b3c")
        + fangs("#fff0f3", "#8a1533")
        + TAIL
    )


def variant_coffin():
    """The box laid in a six-sided box, by candlelight."""
    defs = ('<radialGradient id="candle" cx="0.5" cy="0.15" r="0.8">'
            '<stop offset="0%" stop-color="#ffb648" stop-opacity="0.55"/>'
            '<stop offset="100%" stop-color="#ffb648" stop-opacity="0"/>'
            '</radialGradient>')
    return (
        head(defs)
        + '<rect x="0" y="0" width="512" height="512" fill="#140f0b"/>'
        + '<rect x="0" y="0" width="512" height="512" fill="url(#candle)"/>'
        + '<path d="M256 26 L 462 150 L 430 486 L 82 486 L 50 150 z" '
          'fill="#3b2a1c" stroke="#6b4a2e" stroke-width="9"/>'
        + '<path d="M256 46 L 442 160 L 412 466 L 100 466 L 70 160 z" '
          'fill="none" stroke="#8a6236" stroke-width="4" opacity="0.7"/>'
        + crt("#241a12", "#c08b4a", 8, "#0d0906")
        + scanlines(116, 146, 280, 168, "0.05")
        + glasses("#0d0906", "#c08b4a", "#ffb648", "#ffe9c2", "0.45")
        + prompt_mouth("#ffb648")
        + fangs("#fff6e6", "#8a2f1a")
        + TAIL
    )


def variant_bloodmoon():
    """Synthwave, but the sun set and something else came up."""
    defs = (SKY.replace('id="sky"', 'id="bsky"')
            .replace("#150733", "#100418").replace("#3a1170", "#2a0722")
            .replace("#7c1a86", "#5c0a24").replace("#e0257f", "#a10f2b")
            .replace("#ff7a3d", "#e02020")
            + BELOW
            + '<linearGradient id="moon" x1="0" y1="0" x2="0" y2="1">'
              '<stop offset="0%" stop-color="#ff6b6b"/>'
              '<stop offset="100%" stop-color="#8a0f22"/></linearGradient>')
    verticals = "".join(f'<line x1="256" y1="342" x2="{x}" y2="512"/>'
                        for x in (-240, -60, 76, 176, 256, 336, 436, 572, 752))
    horizontals = "".join(f'<line x1="0" y1="{y}" x2="512" y2="{y}"/>'
                          for y in (356, 376, 404, 442, 492))
    return (
        head(defs)
        + '<rect x="0" y="0" width="512" height="512" fill="url(#bsky)"/>'
        + '<circle cx="256" cy="196" r="150" fill="url(#moon)" opacity="0.85"/>'
        + bat(112, 112, 0.8, "#1a0208") + bat(392, 88, 0.6, "#1a0208")
        + bat(430, 190, 0.45, "#1a0208", "0.7")
        + '<rect x="0" y="338" width="512" height="5" fill="#ff2d55" opacity="0.9"/>'
        + f'<g clip-path="url(#below)" stroke="#ff2d55" fill="none" opacity="0.55">'
          f'<g stroke-width="3">{verticals}</g>'
          f'<g stroke-width="3.5">{horizontals}</g></g>'
        + crt("#1c0810", "#ff2d55", 8, "#0a0206")
        + scanlines(116, 146, 280, 168, "0.06")
        + glasses("#0a0206", "#ff2d55", "#ffd166", "#ffe8ee", "0.5")
        + prompt_mouth("#ff8fa3")
        + fangs("#fff5f7", "#c2143c")
        + TAIL
    )


def variant_crypt():
    """Cold stone, a gothic arch and a web in the corner."""
    defs = ('<linearGradient id="stone" x1="0" y1="0" x2="0" y2="1">'
            '<stop offset="0%" stop-color="#5b6472"/>'
            '<stop offset="100%" stop-color="#262c36"/></linearGradient>')
    web = ('<g stroke="#8fa3b8" stroke-width="2.5" fill="none" opacity="0.45">'
           '<path d="M10 10 L 150 10"/><path d="M10 10 L 10 150"/>'
           '<path d="M10 10 L 116 116"/><path d="M10 10 L 60 140"/>'
           '<path d="M10 10 L 140 60"/>'
           '<path d="M52 10 C 46 30 30 46 10 52"/>'
           '<path d="M92 10 C 82 46 46 82 10 92"/>'
           '<path d="M132 10 C 118 62 62 118 10 132"/></g>')
    return (
        head(defs)
        + '<rect x="0" y="0" width="512" height="512" fill="#171b22"/>'
        + '<path d="M76 512 L 76 190 A 180 180 0 0 1 436 190 L 436 512 z" '
          'fill="url(#stone)" opacity="0.55"/>'
        + '<g stroke="#0f1319" stroke-width="4" opacity="0.6">'
          '<line x1="76" y1="286" x2="436" y2="286"/>'
          '<line x1="76" y1="380" x2="436" y2="380"/>'
          '<line x1="256" y1="190" x2="256" y2="286"/>'
          '<line x1="166" y1="286" x2="166" y2="380"/>'
          '<line x1="346" y1="286" x2="346" y2="380"/></g>'
        + web
        + bat(430, 66, 0.55, "#0f1319")
        + crt("#1e242e", "#7f8ea3", 8, "#0a0d12")
        + scanlines(116, 146, 280, 168, "0.05")
        + glasses("#0a0d12", "#7f8ea3", "#b9c6d6", "#e6f0fa", "0.4")
        + prompt_mouth("#9fd4ff")
        + fangs("#f2f8ff", "#7a1023")
        + TAIL
    )


VARIANTS = {
    # The Nosferatu family: one portrait, seven nights. Blood Moon is the one
    # the app wears by default. The 80s set below it stays; the point is choice.
    "dawn": variant_dawn,
    "night": variant_nosferatu,
    "shadow": variant_shadow,
    "plague": variant_plague,
    "castle": variant_castle,
    "bloodmoon": variant_bloodmoon,
    "crypt": variant_crypt,
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
