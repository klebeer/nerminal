//! OSS-channel `warp-tui` binary and `default-run` target.
//!
//! This is what bare `cargo run -p warp_tui` builds, so it hand-builds its
//! config and needs no internal `warp-channel-config` generator (mirrors
//! `app/src/bin/nerminal.rs`). Like the GUI binary it is offline: the server
//! URLs point at the loopback discard port, so nothing here reaches a
//! backend either. It is a console application (no GUI window,
//! no app bundle), so unlike the GUI binaries it sets no `windows_subsystem`
//! attribute and embeds no `Info.plist`.

use anyhow::Result;
use warp_core::AppId;
use warp_core::channel::{Channel, ChannelConfig, ChannelState, OzConfig, WarpServerConfig};

fn main() -> Result<()> {
    let mut state = ChannelState::new(
        Channel::Oss,
        ChannelConfig {
            app_id: AppId::new("com", "klebeer", "NerminalTui"),
            logfile_name: "nerminal-tui.log".into(),
            server_config: WarpServerConfig::offline(),
            oz_config: OzConfig::offline(),
            telemetry_config: None,
            crash_reporting_config: None,
            autoupdate_config: None,
            mcp_static_config: None,
        },
    );
    if cfg!(debug_assertions) {
        state = state.with_additional_features(warp_core::features::DEBUG_FLAGS);
    }
    ChannelState::set(state);

    warp_tui::run()
}
