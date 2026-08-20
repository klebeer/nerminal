<p align="center">
  <img width="128" alt="Nerminal" src="app/channels/oss/icon/no-padding/512x512.png" />
</p>

<h1 align="center">Nerminal</h1>

<p align="center">A terminal with glasses. No agent, no account, no network.</p>

## What this is

Nerminal is a build of [Warp](https://github.com/warpdotdev/warp) with the
agent, the account and the cloud taken out. What is left is the part people
actually came for: blocks, the tab sidebar, panes, search, the theme system and
the settings.

It does not talk to anything. Launch it, open tabs, run commands, split panes,
change settings, and it opens no socket to any host. The only listener is a
loopback one it uses to talk to its own worker processes.

> [!NOTE]
> This is a personal fork, not a product. It is not affiliated with, endorsed by
> or sponsored by Denver Technologies, Inc. See [NOTICE.md](NOTICE.md) for what
> was changed and why.

## What was removed

| Removed | Kept |
| --- | --- |
| Warp Agent, agent view, conversations | Blocks and block actions |
| MCP servers | Tab sidebar, panes, vertical tabs |
| Warp Drive, session sharing, teams | Global search, command palette |
| Cloud agents, Oz, environments | Themes, fonts, appearance |
| Codebase indexing and embedding | Workflows, keybindings, vim mode |
| Agent-driven code review | Warpify, SSH, shell integration |
| Account, login, billing, referrals | Settings and `settings.toml` |
| Telemetry, crash reporting, autoupdate | The terminal |

## Building

Requires macOS with Xcode.app (not just the Command Line Tools), the Metal
toolchain, `protoc`, `jq` and the Rust toolchain pinned in
`rust-toolchain.toml`.

```bash
xcodebuild -downloadComponent MetalToolchain   # once, ~700 MB
brew install protobuf jq

cargo build --bin nerminal --features gui      # build
./script/macos/run                             # build and run as a real .app
./script/macos/run --release                   # the one you want day to day
```

A debug build carries an on-screen grid/size overlay and runs slowly. Use
`--release` for daily use.

## Configuration

| What | Where |
| --- | --- |
| Settings | `~/.nerminal/settings.toml` |
| Themes and workflows | `~/.nerminal/` |
| Application state | `~/Library/Application Support/com.klebeer.Nerminal` |
| Logs | `~/Library/Logs/nerminal.log` |

Coming from Warp? Copy `~/.warp/settings.toml` to `~/.nerminal/settings.toml`.
Theme, font, spacing and notification preferences carry over as they are.

## Licence

AGPL-3.0-only, except `crates/warpui` and `crates/warpui_core`, which are MIT.
Both texts are in this repository as `LICENSE-AGPL` and `LICENSE-MIT`.

This repository is the complete corresponding source for any Nerminal binary.
If you distribute a modified build, the same obligation passes to you.

## Upstream

Nerminal tracks [warpdotdev/warp](https://github.com/warpdotdev/warp). All
credit for the terminal itself belongs there.

```bash
git remote add upstream https://github.com/warpdotdev/warp.git
git fetch upstream
```
