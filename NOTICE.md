# Notice

Nerminal is a modified version of Warp, published by Denver Technologies, Inc.
at https://github.com/warpdotdev/warp.

Modification began on 20 August 2026, from upstream commit
`092c1dce952f29264e8d281298e4574355fc7102`. This notice and its date exist to
satisfy section 5(a) of the GNU Affero General Public License, which requires a
modified work to state that it was changed and when.

Nerminal is not affiliated with, endorsed by, or sponsored by Denver
Technologies, Inc.

## What was changed

Nerminal ships the terminal and nothing that depends on a Warp account.

- The agent, MCP, Warp Drive, cloud agents, code review, codebase indexing and
  the onboarding wizard are disabled for this build. See
  `OSS_DISABLED_FLAGS` in `app/src/features.rs`.
- The Warp server, Oz and RTC URLs point at the loopback discard port, so no
  request can leave the machine even if a code path is missed. See
  `WarpServerConfig::offline` in `crates/warp_core/src/channel/config.rs`.
- The app starts straight into a terminal with no account and no login wall.
- Telemetry, crash reporting and autoupdate are off, as they already were on
  the upstream OSS channel.
- The product is renamed: binary `nerminal`, bundle `com.klebeer.Nerminal`,
  URL scheme `nerminal://`, config directory `~/.nerminal`. Warp's branding is
  not used, and its logo files were removed from this tree.
- Warp's GitHub Actions workflows, issue and pull request templates and
  internal ownership files were removed. They are wired to Warp's own secrets,
  runners and review bots.

## Licensing

Warp is licensed under AGPL-3.0-only, except for the `crates/warpui` and
`crates/warpui_core` crates, which are MIT. Nerminal keeps that arrangement
unchanged; see `LICENSE-AGPL` and `LICENSE-MIT`.

The complete corresponding source for any Nerminal binary is this repository,
at the commit the binary was built from.

## Trademarks

Warp, Oz and the Warp logo are marks of Denver Technologies, Inc. The AGPL
grants copyright permissions and no trademark rights; section 7(e) of the
licence contemplates exactly that reservation. Warp is named here only to
identify the upstream project this work derives from.
