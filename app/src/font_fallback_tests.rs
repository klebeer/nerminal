use warpui::AssetProvider as _;

use super::*;
use crate::ASSETS;

/// Glyphs a powerline prompt draws. The separator is the interesting one: the
/// bundled monospace font has it, so a theme can look almost right while every
/// icon beside it is blank.
const PROMPT_GLYPHS: [(char, &str); 4] = [
    ('\u{E0B0}', "powerline separator"),
    ('\u{F07B}', "folder"),
    ('\u{F115}', "folder open"),
    ('\u{E712}', "git branch"),
];

#[test]
fn prompt_glyphs_route_to_the_bundled_icon_font() {
    for (ch, name) in PROMPT_GLYPHS {
        let family = fallback_font_fn(ch)
            .unwrap_or_else(|| panic!("{name} (U+{:04X}) has no fallback family", ch as u32));
        assert_eq!(family.name, NERD_FONT_NAME, "{name} routed elsewhere");
    }
}

/// The family is served from the bundle. An entry here with a URL would mean a
/// glyph could depend on the network, which this build has none of.
#[test]
fn the_icon_font_is_never_fetched() {
    let family = fallback_font_fn('\u{F07B}').expect("folder icon should have a fallback family");
    assert!(
        family.font_urls.is_empty(),
        "the icon font declared URLs: {:?}",
        family.font_urls
    );
}

#[test]
fn every_bundled_icon_font_file_is_present() {
    for path in NERD_FONT_FILES {
        let font = ASSETS
            .get(path)
            .unwrap_or_else(|error| panic!("{path}: {error}"));
        assert!(
            font.len() > 100_000,
            "{path} is {} bytes, far too small to be the font",
            font.len()
        );
    }
}

/// Latin text must keep using the font the user chose. Routing it to the icon
/// font would silently restyle everything they read.
#[test]
fn ordinary_text_is_left_to_the_chosen_font() {
    for ch in ['a', 'Z', '0', ' ', '{', 'é'] {
        assert!(
            fallback_font_fn(ch).is_none(),
            "{ch:?} was routed to a fallback font"
        );
    }
}
