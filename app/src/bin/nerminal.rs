// On Windows, we don't want to display a console window when the application is running in release
// builds. See https://doc.rust-lang.org/reference/runtime.html#the-windows_subsystem-attribute.
#![cfg_attr(feature = "release_bundle", windows_subsystem = "windows")]

use anyhow::Result;
use warp_core::AppId;
use warp_core::channel::{Channel, ChannelConfig, ChannelState, OzConfig, WarpServerConfig};

// Entry point for Nerminal, this fork's build of the OSS channel.
//
// It ships the terminal only. The agent, MCP, Warp Drive and the account that
// backs them are disabled (`warp::features::OSS_DISABLED_FLAGS`), and the
// server URLs point at the loopback discard port so nothing can reach Warp's
// backend even if a code path is missed.
fn main() -> Result<()> {
    let mut state = ChannelState::new(
        Channel::Oss,
        ChannelConfig {
            app_id: AppId::new("com", "klebeer", "Nerminal"),
            logfile_name: "nerminal.log".into(),
            server_config: WarpServerConfig::offline(),
            oz_config: OzConfig::offline(),
            telemetry_config: None,
            crash_reporting_config: None,
            autoupdate_config: None,
            mcp_static_config: None,
        },
    )
    .with_disabled_features(warp::features::OSS_DISABLED_FLAGS);
    if cfg!(debug_assertions) {
        state = state.with_additional_features(warp_core::features::DEBUG_FLAGS);
    }
    ChannelState::set(state);

    warp::run()
}

// If we're not using an external plist, embed the following as the Info.plist.
#[cfg(all(not(feature = "extern_plist"), target_os = "macos"))]
embed_plist::embed_info_plist_bytes!(r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <!DOCTYPE plist PUBLIC "-//Apple Computer//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
    <plist version="1.0">
    <dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>English</string>
    <key>CFBundleDisplayName</key>
    <string>Nerminal</string>
    <key>CFBundleExecutable</key>
    <string>nerminal</string>
    <key>CFBundleIdentifier</key>
    <string>com.klebeer.Nerminal</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>Nerminal</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.developer-tools</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>UIDesignRequiresCompatibility</key>
    <true/>
    <key>CFBundleURLTypes</key>
    <array><dict><key>CFBundleURLName</key><string>Custom App</string><key>CFBundleURLSchemes</key><array><string>nerminal</string></array></dict></array>
    <key>NSHumanReadableCopyright</key>
    <string>© 2026 Kleber Ayala. Based on Warp, © 2020-2026 Denver Technologies, Inc. Licensed under AGPL-3.0-only.</string>
    </dict>
    </plist>
"#.as_bytes());
