# JetBrainsMono Nerd Font Mono

The icon fallback font. Prompts like powerlevel10k and starship draw folder,
branch and status icons from Unicode's private use area, and that area carries
no script tag, so the platform font cascade will not substitute into it. A glyph
missing there is a blank cell rather than a substituted one, which is why this
font is carried in the bundle instead of being left to the system.

`app/src/font_fallback.rs` maps the private-use ranges to it and
`app/src/appearance.rs` loads it at startup. Nothing fetches it.

## Which variant

The `Mono` variant, which constrains every icon to a single cell. The plain and
`Propo` variants leave double-width icons that break the terminal grid.

## Where it came from

    gh release download v3.5.1 --repo ryanoasis/nerd-fonts --pattern JetBrainsMono.tar.xz

    JetBrainsMono.tar.xz  sha256 04d5e8f903693f9dd13e16f867e994834e681eb3c72c0d337a770dcda09010cf

Four faces were taken from that archive unmodified: Regular, Bold, Italic and
BoldItalic.

## Licensing

Both halves are the SIL Open Font License 1.1, which permits bundling and
redistribution with software.

- `OFL.txt` covers JetBrains Mono, the base font.
  Copyright 2020 The JetBrains Mono Project Authors. No Reserved Font Name is
  declared, which is what lets the patched font keep the JetBrainsMono name.
- `LICENSE-nerd-fonts.txt` covers the patched icon glyphs.
  Copyright (c) 2014 Ryan L McIntyre.

The licence requires that these files ship alongside the font, that it is not
sold on its own, and that a modified version is not passed off under the same
name. Keep the faces byte-identical to the release above; patch nothing here.
