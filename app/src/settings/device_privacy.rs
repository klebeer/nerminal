//! Privacy settings that describe one machine and never leave it.

use settings::macros::define_settings_group;
use settings::{SupportedPlatforms, SyncToCloud};

define_settings_group!(DevicePrivacySettings, settings: [
    secure_keyboard_entry: SecureKeyboardEntry {
        type: bool,
        // Off by default: while it is on this app loses the emoji picker and any
        // input method that runs out of process, which is not a surprise to
        // spring on someone who never asked for it.
        default: false,
        supported_platforms: SupportedPlatforms::MAC,
        sync_to_cloud: SyncToCloud::Never,
        surface: settings::SettingSurfaces::GUI,
        private: false,
        toml_path: "privacy.secure_keyboard_entry",
        description: "Whether to stop other apps from observing keystrokes while this app has focus.",
    },
]);
