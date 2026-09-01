use enum_iterator::{Sequence, all};
use serde::{Deserialize, Serialize};
use warp_core::settings::macros::define_settings_group;
use warp_core::settings::{SupportedPlatforms, SyncToCloud};

/// The app icon to use (mac-only).
///
/// Three families: the CRT in one palette or another, the portrait, and the
/// record sleeves. To add one, drop a 512x512 PNG in `app/assets/bundled/png/`
/// and wire it up in [`AppIcon::asset_path`].
#[derive(
    Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Sequence, schemars::JsonSchema,
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
    #[schemars(description = "Spiral")]
    Spiral,
    #[schemars(description = "Aura")]
    Aura,
    #[schemars(description = "Ash")]
    Ash,
    #[schemars(description = "Ribcage")]
    Ribcage,
    #[schemars(description = "Wings")]
    Wings,
    #[schemars(description = "Totem")]
    Totem,
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
            AppIcon::Spiral => "bundled/png/nerminal-spiral.png",
            AppIcon::Aura => "bundled/png/nerminal-aura.png",
            AppIcon::Ash => "bundled/png/nerminal-ash.png",
            AppIcon::Ribcage => "bundled/png/nerminal-ribcage.png",
            AppIcon::Wings => "bundled/png/nerminal-wings.png",
            AppIcon::Totem => "bundled/png/nerminal-totem.png",
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
            AppIcon::Spiral => "Spiral",
            AppIcon::Aura => "Aura",
            AppIcon::Ash => "Ash",
            AppIcon::Ribcage => "Ribcage",
            AppIcon::Wings => "Wings",
            AppIcon::Totem => "Totem",
        }
    }

    /// The name this variant carries in the settings file.
    ///
    /// Snake case, which is what the `SettingsValue` derive produced before
    /// this became a manual impl. Editing one of these strings orphans every
    /// settings file that already holds it.
    fn file_name(self) -> &'static str {
        match self {
            AppIcon::Ruthven => "ruthven",
            AppIcon::Catppuccin => "catppuccin",
            AppIcon::BloodMoon => "blood_moon",
            AppIcon::Dawn => "dawn",
            AppIcon::Night => "night",
            AppIcon::Shadow => "shadow",
            AppIcon::Plague => "plague",
            AppIcon::Castle => "castle",
            AppIcon::Crypt => "crypt",
            AppIcon::Classic => "classic",
            AppIcon::Spiral => "spiral",
            AppIcon::Aura => "aura",
            AppIcon::Ash => "ash",
            AppIcon::Ribcage => "ribcage",
            AppIcon::Wings => "wings",
            AppIcon::Totem => "totem",
        }
    }
}

impl std::fmt::Display for AppIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.display_name())
    }
}

/// Icon names that shipped once and no longer name anything.
///
/// Dropping a variant is not free: a settings file still holding one of these
/// fails to parse, and the settings layer answers that by inhibiting writes to
/// the key as well as falling back to the default. The dock would show the
/// default and the picker would go quiet, saving nothing, until the file was
/// edited by hand. Reading them as the default instead keeps the key writable,
/// so the next pick rewrites the stale line.
const RETIRED: &[&str] = &["metal", "arcade", "cartridge", "mixtape", "neon", "vhs"];

impl settings_value::SettingsValue for AppIcon {
    fn to_file_value(&self) -> serde_json::Value {
        serde_json::Value::String(self.file_name().to_owned())
    }

    fn from_file_value(value: &serde_json::Value) -> Option<Self> {
        let name = value.as_str()?;
        all::<AppIcon>()
            .find(|icon| icon.file_name() == name)
            .or_else(|| RETIRED.contains(&name).then_some(AppIcon::default()))
    }
}

#[cfg(test)]
#[path = "app_icon_tests.rs"]
mod tests;

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
