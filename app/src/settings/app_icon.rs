use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use warp_core::settings::macros::define_settings_group;
use warp_core::settings::{SupportedPlatforms, SyncToCloud};

/// The app icon to use (mac-only).
///
/// Every variant is the same face in a different era. To add one, drop a
/// 512x512 PNG in `app/assets/bundled/png/` and wire it up in
/// [`AppIcon::asset_path`].
#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Serialize,
    Deserialize,
    Sequence,
    schemars::JsonSchema,
    settings_value::SettingsValue,
)]
#[schemars(
    description = "The app icon displayed in the dock.",
    rename_all = "snake_case"
)]
pub enum AppIcon {
    #[default]
    #[schemars(description = "Ruthven")]
    Ruthven,
    #[schemars(description = "Catppuccin")]
    Catppuccin,
    #[schemars(description = "Blood Moon")]
    BloodMoon,
    #[schemars(description = "Dawn")]
    Dawn,
    #[schemars(description = "Night")]
    Night,
    #[schemars(description = "Shadow")]
    Shadow,
    #[schemars(description = "Plague")]
    Plague,
    #[schemars(description = "Castle")]
    Castle,
    #[schemars(description = "Crypt")]
    Crypt,
    #[schemars(description = "Classic")]
    Classic,
    #[schemars(description = "Metal")]
    Metal,
    #[schemars(description = "Arcade")]
    Arcade,
    #[schemars(description = "Cartridge")]
    Cartridge,
    #[schemars(description = "Mixtape")]
    Mixtape,
    #[schemars(description = "Neon")]
    Neon,
    #[schemars(description = "VHS")]
    Vhs,
}

impl AppIcon {
    pub fn asset_path(self) -> &'static str {
        match self {
            AppIcon::Ruthven => "bundled/png/nerminal-ruthven.png",
            AppIcon::Catppuccin => "bundled/png/nerminal-catppuccin.png",
            AppIcon::BloodMoon => "bundled/png/nerminal-bloodmoon.png",
            AppIcon::Dawn => "bundled/png/nerminal-dawn.png",
            AppIcon::Night => "bundled/png/nerminal-night.png",
            AppIcon::Shadow => "bundled/png/nerminal-shadow.png",
            AppIcon::Plague => "bundled/png/nerminal-plague.png",
            AppIcon::Castle => "bundled/png/nerminal-castle.png",
            AppIcon::Crypt => "bundled/png/nerminal-crypt.png",
            AppIcon::Classic => "bundled/png/nerminal-classic.png",
            AppIcon::Metal => "bundled/png/nerminal-metal.png",
            AppIcon::Arcade => "bundled/png/nerminal-arcade.png",
            AppIcon::Cartridge => "bundled/png/nerminal-cartridge.png",
            AppIcon::Mixtape => "bundled/png/nerminal-mixtape.png",
            AppIcon::Neon => "bundled/png/nerminal-neon.png",
            AppIcon::Vhs => "bundled/png/nerminal-vhs.png",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            AppIcon::Ruthven => "Ruthven",
            AppIcon::Catppuccin => "Catppuccin",
            AppIcon::BloodMoon => "Blood Moon",
            AppIcon::Dawn => "Dawn",
            AppIcon::Night => "Night",
            AppIcon::Shadow => "Shadow",
            AppIcon::Plague => "Plague",
            AppIcon::Castle => "Castle",
            AppIcon::Crypt => "Crypt",
            AppIcon::Classic => "Classic",
            AppIcon::Metal => "Metal",
            AppIcon::Arcade => "Arcade",
            AppIcon::Cartridge => "Cartridge",
            AppIcon::Mixtape => "Mixtape",
            AppIcon::Neon => "Neon",
            AppIcon::Vhs => "VHS",
        }
    }
}

impl std::fmt::Display for AppIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.display_name())
    }
}

define_settings_group!(AppIconSettings, settings: [
    app_icon: AppIconState {
        type: AppIcon,
        default: AppIcon::Ruthven,
        supported_platforms: SupportedPlatforms::MAC,
        sync_to_cloud: SyncToCloud::Never,
        surface: settings::SettingSurfaces::GUI,
        private: false,
        storage_key: "AppIcon",
        toml_path: "appearance.icon.app_icon",
        description: "The app icon displayed in the dock.",
    },
    show_dock_icon: ShowDockIconState {
        type: bool,
        default: true,
        supported_platforms: SupportedPlatforms::MAC,
        sync_to_cloud: SyncToCloud::Never,
        surface: settings::SettingSurfaces::GUI,
        private: false,
        storage_key: "ShowDockIcon",
        toml_path: "appearance.icon.show_dock_icon",
        description: "Whether Nerminal is shown in the macOS Dock and Cmd-Tab switcher.",
    },
]);
