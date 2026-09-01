#!/usr/bin/env python3
"""Generate the Nerminal alternate app icons.

Three families. The CRT: a tube whose screen is a nerd face, with thick taped
glasses for eyes and a shell prompt for a mouth, in one palette or another. The
portrait: Orlok, 1922, on a different night each time. The sleeves: no box and
no face at all, one figure per record alone on a dark ground. Homage to a
genre, never to anyone's marks or characters.
"""

import math
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


# Catppuccin Mocha, from the project's published palette. Its own colours only:
# base and crust for the ground, mauve for the accent, pink, blue and green
# where the family usually puts neon.
CATPPUCCIN = dict(
    crust="#11111B", base="#1E1E2E", surface="#313244", overlay="#6C7086",
    text="#CDD6F4", mauve="#CBA6F7", pink="#F5C2E7", blue="#89B4FA",
    green="#A6E3A1", peach="#FAB387", red="#F38BA8",
)


def variant_catppuccin():
    """The same CRT, lit in Catppuccin Mocha instead of neon."""
    c = CATPPUCCIN
    defs = (
        '<linearGradient id="csky" x1="0" y1="0" x2="0" y2="1">'
        f'<stop offset="0%" stop-color="{c["crust"]}"/>'
        f'<stop offset="45%" stop-color="{c["base"]}"/>'
        f'<stop offset="78%" stop-color="{c["surface"]}"/>'
        f'<stop offset="100%" stop-color="{c["mauve"]}"/></linearGradient>'
        + BELOW
        + '<radialGradient id="cmoon" cx="0.5" cy="0.4" r="0.6">'
        f'<stop offset="0%" stop-color="{c["pink"]}"/>'
        f'<stop offset="100%" stop-color="{c["mauve"]}"/></radialGradient>'
    )
    verticals = "".join(f'<line x1="256" y1="342" x2="{x}" y2="512"/>'
                        for x in (-240, -60, 76, 176, 256, 336, 436, 572, 752))
    horizontals = "".join(f'<line x1="0" y1="{y}" x2="512" y2="{y}"/>'
                          for y in (356, 376, 404, 442, 492))
    return (
        head(defs)
        + '<rect x="0" y="0" width="512" height="512" fill="url(#csky)"/>'
        + '<circle cx="256" cy="196" r="150" fill="url(#cmoon)" opacity="0.55"/>'
        + bat(112, 112, 0.8, c["crust"]) + bat(392, 88, 0.6, c["crust"])
        + bat(430, 190, 0.45, c["crust"], "0.7")
        + f'<rect x="0" y="338" width="512" height="5" fill="{c["mauve"]}" opacity="0.9"/>'
        + f'<g clip-path="url(#below)" stroke="{c["mauve"]}" fill="none" opacity="0.5">'
          f'<g stroke-width="3">{verticals}</g>'
          f'<g stroke-width="3.5">{horizontals}</g></g>'
        + crt(c["base"], c["mauve"], 8, c["crust"])
        + scanlines(116, 146, 280, 168, "0.06")
        + glasses(c["crust"], c["mauve"], c["peach"], c["text"], "0.5")
        + prompt_mouth(c["green"])
        + fangs(c["text"], c["pink"])
        + TAIL
    )


def monocle(lens_fill, rim, glint, chain):
    """The aristocrat's eyepiece: one lens, one bare socket, a hanging chain.

    Polidori's Ruthven is read by his face rather than by fangs, so the eyes do
    the work here: a gold-rimmed monocle on one side and the dead grey eye on
    the other, where every other variant has a matched pair of lenses.
    """
    return f'''
      <g>
        <circle cx="190" cy="204" r="44" fill="{lens_fill}"/>
        <circle cx="190" cy="204" r="44" fill="none" stroke="{rim}"
                stroke-width="4" opacity="0.35"/>
        <circle cx="190" cy="204" r="15" fill="{glint}" opacity="0.75"/>
        <circle cx="322" cy="204" r="48" fill="{lens_fill}"/>
        <g fill="none" stroke="{rim}">
          <circle cx="322" cy="204" r="48" stroke-width="10"/>
          <circle cx="322" cy="204" r="40" stroke-width="3" opacity="0.55"/>
        </g>
        <path d="M364 230 q18 30 4 60 q-10 22 10 40" fill="none" stroke="{chain}"
              stroke-width="5" stroke-linecap="round" opacity="0.8"/>
        <g stroke="{glint}" stroke-width="9" stroke-linecap="round" opacity="0.45">
          <line x1="302" y1="190" x2="326" y2="172"/>
        </g>
      </g>'''


def cravat(linen, knot):
    """Court dress at the throat, on the stand every variant already has.

    It starts below y=340 because the canines hang to y=334 and a collar drawn
    any higher reads as a bib rather than as dress.
    """
    return f'''
      <g>
        <path d="M234 346 q22 18 44 0 l9 34 q-31 12 -62 0 z" fill="{linen}"
              opacity="0.9"/>
        <path d="M256 362 l-9 20 h18 z" fill="{knot}" opacity="0.85"/>
      </g>'''


def variant_ruthven():
    """Polidori, 1819: the first aristocrat to be a vampire, and the reason
    every vampire since has had manners.

    Catppuccin Mocha throughout, exactly as `variant_catppuccin` lights it. The
    only thing that changes is the face: a monocle and a collar where the rest
    of the family wears a matched pair of lenses.
    """
    c = CATPPUCCIN
    defs = (
        '<linearGradient id="rsky" x1="0" y1="0" x2="0" y2="1">'
        f'<stop offset="0%" stop-color="{c["crust"]}"/>'
        f'<stop offset="45%" stop-color="{c["base"]}"/>'
        f'<stop offset="78%" stop-color="{c["surface"]}"/>'
        f'<stop offset="100%" stop-color="{c["mauve"]}"/></linearGradient>'
        + BELOW
        + '<radialGradient id="rmoon" cx="0.5" cy="0.4" r="0.6">'
        f'<stop offset="0%" stop-color="{c["pink"]}"/>'
        f'<stop offset="100%" stop-color="{c["mauve"]}"/></radialGradient>'
    )
    verticals = "".join(f'<line x1="256" y1="342" x2="{x}" y2="512"/>'
                        for x in (-240, -60, 76, 176, 256, 336, 436, 572, 752))
    horizontals = "".join(f'<line x1="0" y1="{y}" x2="512" y2="{y}"/>'
                          for y in (356, 376, 404, 442, 492))
    return (
        head(defs)
        + '<rect x="0" y="0" width="512" height="512" fill="url(#rsky)"/>'
        + '<circle cx="256" cy="196" r="150" fill="url(#rmoon)" opacity="0.55"/>'
        + bat(112, 112, 0.8, c["crust"]) + bat(392, 88, 0.6, c["crust"])
        + bat(430, 190, 0.45, c["crust"], "0.7")
        + f'<rect x="0" y="338" width="512" height="5" fill="{c["mauve"]}" opacity="0.9"/>'
        + f'<g clip-path="url(#below)" stroke="{c["mauve"]}" fill="none" opacity="0.5">'
          f'<g stroke-width="3">{verticals}</g>'
          f'<g stroke-width="3.5">{horizontals}</g></g>'
        + crt(c["base"], c["mauve"], 8, c["crust"])
        + scanlines(116, 146, 280, 168, "0.06")
        + monocle(c["crust"], c["mauve"], c["text"], c["peach"])
        + prompt_mouth(c["green"])
        + fangs(c["text"], c["pink"])
        + cravat(c["text"], c["surface"])
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


# ---------------------------------------------------------------------------
# The record-sleeve family
#
# No tube and no face. Six sleeves from one shelf of progressive metal, each
# reduced to the one figure it is remembered by: the spiral, the radiant body,
# the smoke, the ribs, the winged eye, the ornament. Drawn from scratch in the
# same bone line and single accent, so the six read as one set.
#
# The vocabulary is old and unowned. Nothing here traces a photograph, a
# painting, a sleeve or a logo, and the variants are named for what they draw
# rather than for anybody's record.
# ---------------------------------------------------------------------------

VOID = "#05060a"
BONE = "#ded5c0"
DIM = "#3f3c31"


def ink(accent, extra_defs="", ground=VOID, glow="0.30"):
    """Open a sleeve variant: the ground and the accent haze behind it."""
    defs = (
        '<radialGradient id="aura" cx="0.5" cy="0.5" r="0.62">'
        f'<stop offset="0%" stop-color="{accent}" stop-opacity="{glow}"/>'
        f'<stop offset="55%" stop-color="{accent}" stop-opacity="0.07"/>'
        f'<stop offset="100%" stop-color="{accent}" stop-opacity="0"/>'
        '</radialGradient>'
        '<radialGradient id="vignette" cx="0.5" cy="0.5" r="0.72">'
        '<stop offset="50%" stop-color="#000000" stop-opacity="0"/>'
        '<stop offset="100%" stop-color="#000000" stop-opacity="0.55"/>'
        '</radialGradient>' + extra_defs
    )
    return (
        head(defs)
        + f'<rect x="0" y="0" width="512" height="512" fill="{ground}"/>'
        + '<rect x="0" y="0" width="512" height="512" fill="url(#aura)"/>'
    )


def ink_close(halo):
    """Close a geometry variant with the shared ring and the vignette."""
    return (
        f'<g fill="none" stroke="{halo}">'
        '<circle cx="256" cy="256" r="234" stroke-width="3" opacity="0.28"/>'
        '<circle cx="256" cy="256" r="223" stroke-width="1.5" opacity="0.16"/>'
        '</g>'
        '<rect x="0" y="0" width="512" height="512" fill="url(#vignette)"/>'
        + TAIL
    )


def line_pair(paths, accent, w=11, glow=2.1):
    """A figure drawn twice: once wide in the accent for the bleed a thin line
    cannot give, once at weight in bone. Every construction here uses it, which
    is what makes the six look struck by the same hand."""
    body = "".join(paths)
    return (
        f'<g fill="none" stroke="{accent}" stroke-width="{w * glow:.1f}" '
        f'stroke-linecap="round" stroke-linejoin="round" opacity="0.30">{body}</g>'
        f'<g fill="none" stroke="{BONE}" stroke-width="{w}" '
        f'stroke-linecap="round" stroke-linejoin="round">{body}</g>'
    )


def _ring_points(cx, cy, r, n, rot=-90.0):
    return [
        (cx + r * math.cos(math.radians(rot + i * 360.0 / n)),
         cy + r * math.sin(math.radians(rot + i * 360.0 / n)))
        for i in range(n)
    ]


def _points(pts):
    return " ".join(f"{x:.1f},{y:.1f}" for x, y in pts)


# The corner each square's quarter-circle turns about, by the side the square
# was added on. Getting this wrong gives a chain of arcs that meet but do not
# stay tangent, which reads as a wobble rather than as a spiral.
_PIVOT = {
    "above": lambda x, y, s: (x, y + s),
    "left": lambda x, y, s: (x + s, y + s),
    "below": lambda x, y, s: (x + s, y),
    "right": lambda x, y, s: (x, y),
}


def _fib_tiling(count):
    """The Fibonacci tiling in unit squares, y down, plus its bounding box."""
    fib = [1, 1]
    while len(fib) < count:
        fib.append(fib[-1] + fib[-2])
    x0, y0, x1, y1 = 0, 0, 1, 1
    squares = [(0, 0, 1, "above")]
    for i in range(1, count):
        s = fib[i]
        side = ("left", "below", "right", "above")[(i - 1) % 4]
        if side == "left":
            x0 -= s
            squares.append((x0, y0, s, side))
        elif side == "below":
            squares.append((x0, y1, s, side))
            y1 += s
        elif side == "right":
            squares.append((x1, y0, s, side))
            x1 += s
        else:
            y0 -= s
            squares.append((x0, y0, s, side))
    return squares, (x0, y0, x1, y1)


def variant_spiral():
    """The golden rectangle taken apart into squares, and the curve that falls
    out of it once you join their corners."""
    accent = "#b8722e"
    squares, (ux0, uy0, ux1, uy1) = _fib_tiling(8)
    span, rise = ux1 - ux0, uy1 - uy0
    u = 434.0 / span
    ox = (512 - span * u) / 2 - ux0 * u
    oy = (512 - rise * u) / 2 - uy0 * u

    cells, arcs = [], []
    for x, y, s, side in squares:
        cells.append(
            f'<rect x="{ox + x * u:.1f}" y="{oy + y * u:.1f}" '
            f'width="{s * u:.1f}" height="{s * u:.1f}"/>'
        )
        px, py = _PIVOT[side](x, y, s)
        corners = [(x, y), (x + s, y), (x, y + s), (x + s, y + s)]
        (ax, ay), (bx, by) = [c for c in corners if (c[0] == px) != (c[1] == py)]
        turn = (ax - px) * (by - py) - (ay - py) * (bx - px)
        arcs.append(
            f'<path d="M{ox + ax * u:.1f} {oy + ay * u:.1f} '
            f'A{s * u:.1f} {s * u:.1f} 0 0 {1 if turn > 0 else 0} '
            f'{ox + bx * u:.1f} {oy + by * u:.1f}"/>'
        )

    return (
        ink(accent)
        + f'<g fill="none" stroke="{DIM}" stroke-width="2.5">{"".join(cells)}</g>'
        + f'<rect x="{ox + ux0 * u:.1f}" y="{oy + uy0 * u:.1f}" '
        f'width="{span * u:.1f}" height="{rise * u:.1f}" fill="none" '
        f'stroke="{accent}" stroke-width="4" opacity="0.55"/>'
        + line_pair(arcs, accent, 12)
        + ink_close(DIM)
    )


# One path, not a head sitting on a pair of shoulders: the join has to be the
# trapezius line or the figure comes out a chess pawn. Small enough that the
# innermost band of the field still clears the crown.
BUST = (
    "M256 170 C303 170 331 205 331 250 C331 288 318 318 300 336 "
    "C296 344 294 352 294 366 C294 386 302 396 320 404 "
    "C366 422 398 466 402 512 L110 512 "
    "C114 466 146 422 192 404 C210 396 218 386 218 366 "
    "C218 352 216 344 212 336 C194 318 181 288 181 250 "
    "C181 205 209 170 256 170 Z"
)


def variant_aura():
    """The body drawn as light: a bust against the banded field it gives off,
    lit from the point between the eyes and open enough to see through."""
    teal, gold, violet = "#2fa39a", "#d9a441", "#7d5fb8"
    ground = "#04090c"
    cx, cy = 256, 254

    defs = (
        f'<clipPath id="bust"><path d="{BUST}"/></clipPath>'
        '<linearGradient id="flesh" x1="0" y1="0" x2="0" y2="1">'
        '<stop offset="0%" stop-color="#0d3b3c"/>'
        '<stop offset="55%" stop-color="#072024"/>'
        '<stop offset="100%" stop-color="#03080b"/></linearGradient>'
    )
    bands = "".join(
        f'<circle cx="{cx}" cy="{cy}" r="{r}" fill="none" '
        f'stroke="{(teal, gold, violet, "#cfe8dd")[i % 4]}" '
        f'stroke-width="{3 + (i % 2) * 5}" '
        f'opacity="{max(0.10, 0.70 - i * 0.06):.2f}"/>'
        for i, r in enumerate((132, 156, 182, 210, 240, 272, 306))
    )
    rays = "".join(
        f'<line x1="{cx + 120 * math.cos(math.radians(a)):.1f}" '
        f'y1="{cy + 120 * math.sin(math.radians(a)):.1f}" '
        f'x2="{cx + 310 * math.cos(math.radians(a)):.1f}" '
        f'y2="{cy + 310 * math.sin(math.radians(a)):.1f}"/>'
        for a in range(0, 360, 6)
    )
    # The sleeve trick this is after: every layer of a body shown at once. Here
    # that is contour lines across the figure and a column of vertebrae, both
    # cut to the silhouette so they stop at the skin.
    # Only across the torso. Ruling the head too turns it into a striped egg.
    contours = "".join(
        f'<line x1="100" y1="{y}" x2="412" y2="{y}"/>' for y in range(414, 512, 22)
    )
    spine = "".join(
        f'<circle cx="{cx}" cy="{y}" r="{8 - i * 0.6:.1f}"/>'
        for i, y in enumerate(range(376, 500, 22))
    )

    return (
        ink(teal, defs, ground=ground, glow="0.20")
        + f'<g fill="none" stroke="{gold}" stroke-width="1.6" opacity="0.12">{rays}</g>'
        + bands
        + f'<path d="{BUST}" fill="url(#flesh)"/>'
        + f'<g clip-path="url(#bust)">'
        f'<g fill="none" stroke="{teal}" stroke-width="1.8" opacity="0.24">'
        f'{contours}</g>'
        f'<g fill="{gold}" opacity="0.7">{spine}</g></g>'
        + f'<path d="{BUST}" fill="none" stroke="{teal}" stroke-width="5"/>'
        + f'<circle cx="{cx}" cy="232" r="30" fill="{gold}" opacity="0.22"/>'
        + f'<circle cx="{cx}" cy="232" r="12" fill="{gold}"/>'
        + ink_close(teal)
    )


def variant_ash():
    """A face surfacing in smoke and going under again, behind the fine vertical
    ruling of a lenticular print."""
    ochre, ember = "#b08a52", "#e6c07a"
    defs = (
        '<filter id="fog" x="0%" y="0%" width="100%" height="100%">'
        '<feTurbulence type="fractalNoise" baseFrequency="0.007" numOctaves="5" '
        'seed="17" result="n"/>'
        '<feColorMatrix in="n" type="matrix" '
        'values="0 0 0 0 0.69  0 0 0 0 0.54  0 0 0 0 0.32  1.1 0.7 0 0 -0.55"/>'
        '</filter>'
        '<filter id="soft" x="-30%" y="-30%" width="160%" height="160%">'
        '<feGaussianBlur stdDeviation="9"/></filter>'
        '<radialGradient id="lit" cx="0.5" cy="0.42" r="0.62">'
        f'<stop offset="0%" stop-color="{ember}" stop-opacity="0.95"/>'
        f'<stop offset="55%" stop-color="{ochre}" stop-opacity="0.55"/>'
        f'<stop offset="100%" stop-color="{ochre}" stop-opacity="0"/>'
        '</radialGradient>'
    )
    # The ruling is what tells the eye it is looking at a print and not at
    # weather, so it goes over everything else.
    ruling = "".join(
        f'<rect x="{x}" y="0" width="3" height="512"/>' for x in range(0, 512, 12)
    )
    face = ('M256 116 C314 116 350 160 350 224 C350 288 324 342 298 364 '
            'C284 376 268 382 256 382 C244 382 228 376 214 364 '
            'C188 342 162 288 162 224 C162 160 198 116 256 116 Z')
    # Drawn sharp rather than blurred. Smoke around a face is atmosphere; smoke
    # across the features is a smudge, and at 32px a smudge is nothing at all.
    eyes = ('<path d="M180 218 C200 194 230 194 246 218 C230 240 200 240 180 218 Z '
            'M266 218 C282 194 312 194 332 218 C312 240 282 240 266 218 Z"/>')
    return (
        ink(ochre, defs, ground="#070605", glow="0.16")
        + '<rect x="0" y="0" width="512" height="512" filter="url(#fog)" '
        'opacity="0.34"/>'
        + f'<path d="{face}" fill="url(#lit)"/>'
        + f'<path d="{face}" fill="none" stroke="{ember}" stroke-width="3.5" '
        'opacity="0.55"/>'
        + f'<g fill="#050403">{eyes}</g>'
        + f'<g fill="none" stroke="{ember}" stroke-width="4" opacity="0.75">{eyes}</g>'
        + f'<g fill="#050403" opacity="0.55">'
        '<path d="M256 240 L270 296 L256 304 L242 296 Z"/></g>'
        + '<path d="M212 336 C238 328 274 328 300 336" fill="none" '
        'stroke="#050403" stroke-width="10" stroke-linecap="round" opacity="0.85"/>'
        + '<rect x="0" y="0" width="512" height="512" filter="url(#fog)" '
        'opacity="0.20"/>'
        + f'<g fill="#000000" opacity="0.18">{ruling}</g>'
        + ink_close(ochre)
    )


def variant_ribcage():
    """A ribcage cast in plaster, taken to the waterline and left there."""
    accent = "#8f4a2a"
    # Each rib leaves the sternum and only ever descends. Letting one rise before
    # it falls closes the pair into an arch, and eight nested arches are a
    # basket, not a ribcage.
    ribs = []
    for i in range(8):
        y = 134 + i * 30
        reach = 58 + 70 * math.sin(math.pi * (i + 0.9) / 9.4)
        drop = 44 + i * 11
        for sign in (-1, 1):
            ribs.append(
                f'<path d="M{256 + sign * 20} {y} '
                f'C{256 + sign * reach * 0.58:.0f} {y + drop * 0.06:.0f} '
                f'{256 + sign * reach * 0.97:.0f} {y + drop * 0.44:.0f} '
                f'{256 + sign * reach:.0f} {y + drop:.0f}"/>'
            )
    spine = "".join(
        f'<rect x="243" y="{y}" width="26" height="19" rx="8"/>'
        for y in range(120, 384, 27)
    )
    # The undertow itself: the drag of water under the line it left on the cast.
    current = "".join(
        f'<path d="M-20 {y} C 120 {y - 16} 200 {y + 18} 340 {y - 6} '
        f'C 420 {y - 16} 480 {y + 8} 532 {y}"/>'
        for y in (398, 428, 458, 488)
    )
    return (
        ink(accent, ground="#0a0908", glow="0.24")
        + f'<g fill="none" stroke="{accent}" stroke-width="3" opacity="0.35">'
        f'{current}</g>'
        + f'<rect x="0" y="392" width="512" height="120" fill="{accent}" '
        'opacity="0.10"/>'
        + line_pair(ribs, accent, 11, 2.0)
        + f'<g fill="{BONE}" opacity="0.92">{spine}</g>'
        + f'<g fill="none" stroke="{accent}" stroke-width="3.5" opacity="0.7">'
        '<line x1="0" y1="392" x2="512" y2="392"/></g>'
        + ink_close(accent)
    )


def variant_wings():
    """A winged eye, which is the oldest figure on this shelf by about four
    thousand years."""
    gold, violet = "#d9a441", "#8f7ad1"
    cx, cy = 256, 246
    # Feathers as leaves rotated about a shared root. Drawn as separate strokes
    # they scatter; overlapping bodies fanned from one point are what reads as a
    # wing at the size a dock icon actually gets.
    feathers = []
    for sign in (-1, 1):
        for k in range(8):
            length = 138 - k * 9
            angle = -20 + k * 9
            feathers.append(
                f'<ellipse cx="{length / 2:.0f}" cy="0" rx="{length / 2:.0f}" '
                f'ry="{13 - k * 0.9:.1f}" opacity="{0.95 - k * 0.05:.2f}" '
                f'transform="translate({cx + sign * 44} {cy}) '
                f'rotate({angle if sign > 0 else 180 - angle})"/>'
            )
    stars = "".join(
        f'<circle cx="{x}" cy="{y}" r="{r}"/>'
        for x, y, r in ((88, 96, 3), (150, 62, 2), (402, 88, 3.4),
                        (452, 148, 2.2), (330, 52, 2), (206, 44, 2.6))
    )
    lens = f'<path d="M{cx - 92} {cy} Q{cx} {cy - 66} {cx + 92} {cy} ' \
           f'Q{cx} {cy + 66} {cx - 92} {cy} Z"/>'
    return (
        ink(violet, ground="#070512", glow="0.30")
        + f'<g fill="{BONE}" opacity="0.7">{stars}</g>'
        + f'<circle cx="{cx}" cy="{cy}" r="150" fill="none" stroke="{violet}" '
        'stroke-width="3" opacity="0.35"/>'
        + f'<g fill="{gold}" stroke="#070512" stroke-width="2.5">'
        f'{"".join(feathers)}</g>'
        + f'<g fill="#070512">{lens}</g>'
        + f'<g fill="none" stroke="{BONE}" stroke-width="9" '
        f'stroke-linejoin="round">{lens}</g>'
        + f'<circle cx="{cx}" cy="{cy}" r="36" fill="{violet}"/>'
        + f'<circle cx="{cx}" cy="{cy}" r="36" fill="none" stroke="{gold}" '
        'stroke-width="7"/>'
        + f'<circle cx="{cx}" cy="{cy}" r="14" fill="#070512"/>'
        + ink_close(violet)
    )


def variant_totem():
    """Ornament stacked ring on ring until it stops being decoration and starts
    being a thing that looks back."""
    green, gold = "#1f7a63", "#c9a24a"

    def petals(count, radius, rx, ry, fill, opacity, rot=0.0):
        return "".join(
            f'<ellipse cx="0" cy="{-radius}" rx="{rx}" ry="{ry}" fill="{fill}" '
            f'opacity="{opacity}" transform="translate(256 256) '
            f'rotate({rot + i * 360.0 / count:.1f})"/>'
            for i in range(count)
        )

    hairs = "".join(
        f'<line x1="{256 + 118 * math.cos(math.radians(a)):.1f}" '
        f'y1="{256 + 118 * math.sin(math.radians(a)):.1f}" '
        f'x2="{256 + 224 * math.cos(math.radians(a)):.1f}" '
        f'y2="{256 + 224 * math.sin(math.radians(a)):.1f}"/>'
        for a in range(0, 360, 9)
    )
    studs = "".join(
        f'<circle cx="{256 + 232 * math.cos(math.radians(a)):.1f}" '
        f'cy="{256 + 232 * math.sin(math.radians(a)):.1f}" r="4"/>'
        for a in range(0, 360, 15)
    )
    return (
        ink(green, ground="#050b09", glow="0.28")
        + f'<g stroke="{gold}" stroke-width="1.4" opacity="0.16">{hairs}</g>'
        + petals(24, 216, 9, 26, gold, "0.35", 7.5)
        + petals(16, 186, 17, 46, green, "0.55")
        + f'<g fill="none" stroke="{gold}" stroke-width="2.5" opacity="0.6">'
        '<circle cx="256" cy="256" r="204"/><circle cx="256" cy="256" r="148"/>'
        '<circle cx="256" cy="256" r="96"/></g>'
        + petals(12, 126, 20, 42, "#e0bd66", "0.62", 15)
        + petals(8, 66, 22, 40, green, "0.75")
        + f'<g fill="{gold}" opacity="0.85">{studs}</g>'
        + f'<circle cx="256" cy="256" r="34" fill="#050b09"/>'
        + f'<circle cx="256" cy="256" r="34" fill="none" stroke="{gold}" '
        'stroke-width="8"/>'
        + f'<circle cx="256" cy="256" r="13" fill="{gold}"/>'
        + ink_close(gold)
    )


VARIANTS = {
    # The tube and the portrait first, then the sleeves. Ruthven is the one the
    # app wears by default; the rest are there because the point is choice.
    "catppuccin": variant_catppuccin,
    "ruthven": variant_ruthven,
    "dawn": variant_dawn,
    "night": variant_nosferatu,
    "shadow": variant_shadow,
    "plague": variant_plague,
    "castle": variant_castle,
    "bloodmoon": variant_bloodmoon,
    "crypt": variant_crypt,
    "spiral": variant_spiral,
    "aura": variant_aura,
    "ash": variant_ash,
    "ribcage": variant_ribcage,
    "wings": variant_wings,
    "totem": variant_totem,
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
