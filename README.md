<p align="center">
  <img alt="Nerminal" src="images/nerminal-banner.png" />
</p>

<p align="center">A fork of Warp, just for fun.</p>

Warp is the best terminal ever made. I just wanted a simpler one, old school:
no AI, no account, no cloud, nothing that talks to a server.

That is the whole idea.

## Install

    brew install --cask klebeer/nerminal/nerminal

Apple Silicon, macOS 11 and later. Signed and notarized.

## What is different

**Nothing of its own goes over the network**, and there is no account to make.

**Themes.** Nosferatu and Nosferatu Light, Dracula, and Catppuccin in all four
flavours. Mocha is the default.

**Secure keyboard entry.** While the window has focus, other applications
cannot observe your keystrokes, the same protection Terminal and iTerm2 offer.
It matters most at a `sudo` prompt, which the system cannot recognise as a
password field. Off by default, under Settings > Privacy, because the emoji
picker stops working while it is on.

**The Nerd Font is embedded, not downloaded.** Glyphs render on a machine with
nothing else installed, and nothing is fetched to make that happen.

**Copying leaves the padding behind.** A program that redraws its own output
clears each row by writing spaces over it. Those spaces no longer follow the
text to the clipboard, so a line that looked blank stops arriving as a hundred
of them.

**A long URL stays one link** even when the program that printed it split the
address across several lines.

**Less on screen.** The buttons that float over a block on hover and the
elapsed time beside each prompt can both be turned off.

## Credit

A fork of [warpdotdev/warp](https://github.com/warpdotdev/warp). All credit for
the terminal belongs to Warp. Want the whole product?
[warp.dev](https://www.warp.dev).

Licensing and what changed: [NOTICE.md](NOTICE.md).
