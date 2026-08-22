use asset_macro::bundled_or_fetched_asset;
use pathfinder_color::ColorU;
use warp_core::ui::color::OPAQUE;
use warp_core::ui::theme::{
    AnsiColor, AnsiColors, Details, Fill, Image, TerminalColors, VerticalGradient, WarpTheme,
};

const DARK_MODE_NORMAL_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x616161FF),
    AnsiColor::from_u32(0xFF8272FF),
    AnsiColor::from_u32(0xB4FA72FF),
    AnsiColor::from_u32(0xFEFDC2FF),
    AnsiColor::from_u32(0xA5D5FEFF),
    AnsiColor::from_u32(0xFF8FFDFF),
    AnsiColor::from_u32(0xD0D1FEFF),
    AnsiColor::from_u32(0xF1F1F1FF),
);
const DARK_MODE_BRIGHT_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x8E8E8EFF),
    AnsiColor::from_u32(0xFFC4BDFF),
    AnsiColor::from_u32(0xD6FCB9FF),
    AnsiColor::from_u32(0xFEFDD5FF),
    AnsiColor::from_u32(0xC1E3FEFF),
    AnsiColor::from_u32(0xFFB1FEFF),
    AnsiColor::from_u32(0xE5E6FEFF),
    AnsiColor::from_u32(0xFEFFFFFF),
);

const LIGHT_MODE_NORMAL_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x212121FF),
    AnsiColor::from_u32(0xC30771FF),
    AnsiColor::from_u32(0x10A778FF),
    AnsiColor::from_u32(0xA89C14FF),
    AnsiColor::from_u32(0x008EC4FF),
    AnsiColor::from_u32(0x523C79FF),
    AnsiColor::from_u32(0x20A5BAFF),
    AnsiColor::from_u32(0xE0E0E0FF),
);
const LIGHT_MODE_BRIGHT_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x212121FF),
    AnsiColor::from_u32(0xFB007AFF),
    AnsiColor::from_u32(0x5FD7AFFF),
    AnsiColor::from_u32(0xF3E430FF),
    AnsiColor::from_u32(0x20BBFCFF),
    AnsiColor::from_u32(0x6855DEFF),
    AnsiColor::from_u32(0x4FB8CCFF),
    AnsiColor::from_u32(0xF1F1F1FF),
);

/// Taken from the Dawn icon: the last of the night at the top of the sky, the
/// first warmth at the bottom, his pallor for green and blood for red.
///
/// Every slot here clears 4.5:1 against both ends of the background gradient,
/// which `nosferatu_meets_contrast_floor` enforces. Blue is the one colour that
/// does not come from the icon: every lilac readable at that floor collides
/// with magenta, and directories are the most-read colour in a listing.
const NOSFERATU_NORMAL_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x9C8AA4FF),
    AnsiColor::from_u32(0xF45F68FF),
    AnsiColor::from_u32(0x8CBF69FF),
    AnsiColor::from_u32(0xF2C87EFF),
    AnsiColor::from_u32(0x9FB6F0FF),
    AnsiColor::from_u32(0xD673B4FF),
    AnsiColor::from_u32(0x84C1BCFF),
    AnsiColor::from_u32(0xF2E3C8FF),
);
const NOSFERATU_BRIGHT_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0xBFAEC2FF),
    AnsiColor::from_u32(0xFF8F8FFF),
    AnsiColor::from_u32(0xB6DC90FF),
    AnsiColor::from_u32(0xFFE9C0FF),
    AnsiColor::from_u32(0xC6D2FFFF),
    AnsiColor::from_u32(0xF0A0D8FF),
    AnsiColor::from_u32(0xADE0DAFF),
    AnsiColor::from_u32(0xFFFCF5FF),
);

const PHENOMENON_NORMAL_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x121212FF),
    AnsiColor::from_u32(0xD22D1EFF),
    AnsiColor::from_u32(0x1CA05AFF),
    AnsiColor::from_u32(0xE5A01AFF),
    AnsiColor::from_u32(0x3780E9FF),
    AnsiColor::from_u32(0xBF409DFF),
    AnsiColor::from_u32(0x799C92FF),
    AnsiColor::from_u32(0xFAF9F6FF),
);
const PHENOMENON_BRIGHT_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x292929FF),
    AnsiColor::from_u32(0xAE756FFF),
    AnsiColor::from_u32(0x789B88FF),
    AnsiColor::from_u32(0xBD9F65FF),
    AnsiColor::from_u32(0x6F839FFF),
    AnsiColor::from_u32(0xA57899FF),
    AnsiColor::from_u32(0xBFC5C3FF),
    AnsiColor::from_u32(0xFFFFFFFF),
);

const ADEBERRY_NORMAL_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x121212FF),
    AnsiColor::from_u32(0xC76156FF),
    AnsiColor::from_u32(0x57C78AFF),
    AnsiColor::from_u32(0xC8A35AFF),
    AnsiColor::from_u32(0x5785C7FF),
    AnsiColor::from_u32(0xC756A9FF),
    AnsiColor::from_u32(0x57C7C3FF),
    AnsiColor::from_u32(0xEEEDEBFF),
);
const ADEBERRY_BRIGHT_COLORS: AnsiColors = AnsiColors::new(
    AnsiColor::from_u32(0x292929FF),
    AnsiColor::from_u32(0xD22D1EFF),
    AnsiColor::from_u32(0x1CA05AFF),
    AnsiColor::from_u32(0xE5A01AFF),
    AnsiColor::from_u32(0x1458B8FF),
    AnsiColor::from_u32(0xA43787FF),
    AnsiColor::from_u32(0x4D9989FF),
    AnsiColor::from_u32(0xFFFFFFFF),
);

pub(super) fn light_mode_colors() -> TerminalColors {
    TerminalColors::new(LIGHT_MODE_NORMAL_COLORS, LIGHT_MODE_BRIGHT_COLORS)
}

pub(super) fn dark_mode_colors() -> TerminalColors {
    TerminalColors::new(DARK_MODE_NORMAL_COLORS, DARK_MODE_BRIGHT_COLORS)
}

pub(super) fn nosferatu_colors() -> TerminalColors {
    TerminalColors::new(NOSFERATU_NORMAL_COLORS, NOSFERATU_BRIGHT_COLORS)
}

pub(super) fn phenomenon_colors() -> TerminalColors {
    TerminalColors::new(PHENOMENON_NORMAL_COLORS, PHENOMENON_BRIGHT_COLORS)
}

pub(super) fn adeberry_colors() -> TerminalColors {
    TerminalColors::new(ADEBERRY_NORMAL_COLORS, ADEBERRY_BRIGHT_COLORS)
}

/// Default bundled themes
pub fn dark_theme() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0x050505FF)),
        ColorU::from_u32(0xffffffff),
        Fill::Solid(ColorU::from_u32(0x19AAD8FF)),
        None,
        Some(Details::Darker),
        dark_mode_colors(),
        None,
        Some("Dark".to_string()),
    )
}

pub fn light_theme() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::white()),
        ColorU::new(17, 17, 17, OPAQUE),
        Fill::Solid(ColorU::from_u32(0x00c2ffff)),
        None,
        Some(Details::Lighter),
        light_mode_colors(),
        None,
        Some("Light".to_string()),
    )
}

pub(super) fn nosferatu() -> WarpTheme {
    WarpTheme::new(
        // The sky in the Dawn icon, read top to bottom: night giving way to the
        // first warmth. The two stops span 2.6x in luminance; widening that gap
        // costs the bottom rows more contrast than any palette can absorb.
        Fill::VerticalGradient(VerticalGradient::new(
            ColorU::from_u32(0x1E1028FF),
            ColorU::from_u32(0x3E1D28FF),
        )),
        ColorU::from_u32(0xF2E3C8FF),
        // The sun itself, brighter than anything in the background so the
        // cursor and the UI details stay findable. Foreground over this reads
        // at 1.75:1, so anything filling with it needs `main_text_color`.
        Fill::Solid(ColorU::from_u32(0xF5993FFF)),
        None,
        Some(Details::Darker),
        nosferatu_colors(),
        None,
        Some("Nosferatu".to_string()),
    )
}

pub(super) fn phenomenon() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0x121212FF)),
        ColorU::from_u32(0xFAF9F6FF),
        Fill::Solid(ColorU::from_u32(0x2E5D9EFF)),
        None,
        Some(Details::Darker),
        phenomenon_colors(),
        Some(Image {
            source: bundled_or_fetched_asset!("jpg/phenomenon_bg.jpg"),
            opacity: 100,
        }),
        Some("Phenomenon".to_string()),
    )
}

pub(super) fn adeberry() -> WarpTheme {
    WarpTheme::new(
        Fill::Solid(ColorU::from_u32(0x1D2022FF)),
        ColorU::from_u32(0xE4EEF5FF),
        Fill::Solid(ColorU::from_u32(0x6C96B4FF)),
        None,
        Some(Details::Darker),
        adeberry_colors(),
        None,
        Some("Adeberry".to_string()),
    )
}

#[cfg(test)]
#[path = "default_themes_tests.rs"]
mod tests;
