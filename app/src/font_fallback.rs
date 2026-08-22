use std::sync::Arc;

use lazy_static::lazy_static;
use warpui::fonts::ExternalFontFamily;

/// Named after the font's own typographic family, so the font picker shows
/// something a person recognises.
pub const NERD_FONT_NAME: &str = "JetBrainsMono Nerd Font Mono";

/// Bundled asset paths for the family. Loaded in `appearance.rs`.
pub const NERD_FONT_FILES: [&str; 4] = [
    "bundled/fonts/jetbrains-mono-nerd/JetBrainsMonoNerdFontMono-Regular.ttf",
    "bundled/fonts/jetbrains-mono-nerd/JetBrainsMonoNerdFontMono-Bold.ttf",
    "bundled/fonts/jetbrains-mono-nerd/JetBrainsMonoNerdFontMono-Italic.ttf",
    "bundled/fonts/jetbrains-mono-nerd/JetBrainsMonoNerdFontMono-BoldItalic.ttf",
];

lazy_static! {
    /// Nerd Font icons live in Unicode's private use area. Whether the platform
    /// cascade reaches them depends on the machine having a patched font
    /// installed separately, so without this the same prompt renders icons on
    /// one machine and blank cells on another. Carrying the family makes that
    /// answer the same everywhere.
    ///
    /// The URL list is empty on purpose. The family is loaded from the app
    /// bundle at startup, and a family already loaded is never requested, so
    /// nothing is ever fetched.
    static ref NERD_FONT: ExternalFontFamily = ExternalFontFamily {
        name: NERD_FONT_NAME,
        font_urls: Arc::new(Vec::new()),
    };
}

/// The ranges upstream also routed to downloaded Noto families -- emoji, CJK,
/// Arabic, Indic -- are real script with real script tags, so macOS substitutes
/// them from its own fonts and they are left to the platform cascade.
pub fn fallback_font_fn(ch: char) -> Option<ExternalFontFamily> {
    match ch {
        '\u{01A4}'..='\u{01A4}'
        | '\u{01E6}'..='\u{01E7}'
        | '\u{03F4}'..='\u{03F4}'
        | '\u{03F6}'..='\u{03F6}'
        | '\u{051A}'..='\u{051D}'
        | '\u{0531}'..='\u{0556}'
        | '\u{0559}'..='\u{055F}'
        | '\u{0561}'..='\u{0587}'
        | '\u{0589}'..='\u{058A}'
        | '\u{0E3F}'..='\u{0E3F}'
        | '\u{10D0}'..='\u{10FC}'
        | '\u{2012}'..='\u{2012}'
        | '\u{2016}'..='\u{2016}'
        | '\u{201F}'..='\u{201F}'
        | '\u{2023}'..='\u{2024}'
        | '\u{202F}'..='\u{202F}'
        | '\u{2031}'..='\u{2031}'
        | '\u{2034}'..='\u{2037}'
        | '\u{203D}'..='\u{203F}'
        | '\u{2045}'..='\u{2049}'
        | '\u{204B}'..='\u{204B}'
        | '\u{205F}'..='\u{205F}'
        | '\u{2070}'..='\u{2070}'
        | '\u{2075}'..='\u{207E}'
        | '\u{208A}'..='\u{208E}'
        | '\u{20A0}'..='\u{20A2}'
        | '\u{20A5}'..='\u{20A5}'
        | '\u{20AD}'..='\u{20B0}'
        | '\u{20B2}'..='\u{20B5}'
        | '\u{20B7}'..='\u{20B8}'
        | '\u{2150}'..='\u{2151}'
        | '\u{2153}'..='\u{215A}'
        | '\u{215F}'..='\u{215F}'
        | '\u{2190}'..='\u{21DD}'
        | '\u{21E0}'..='\u{21E6}'
        | '\u{21E8}'..='\u{21E9}'
        | '\u{21EB}'..='\u{2201}'
        | '\u{2203}'..='\u{2205}'
        | '\u{2207}'..='\u{220E}'
        | '\u{2210}'..='\u{2210}'
        | '\u{2213}'..='\u{2213}'
        | '\u{2215}'..='\u{2215}'
        | '\u{2217}'..='\u{2219}'
        | '\u{221B}'..='\u{221D}'
        | '\u{221F}'..='\u{2220}'
        | '\u{2223}'..='\u{2223}'
        | '\u{2227}'..='\u{222A}'
        | '\u{222C}'..='\u{222D}'
        | '\u{2234}'..='\u{223D}'
        | '\u{2241}'..='\u{2247}'
        | '\u{2249}'..='\u{225F}'
        | '\u{2261}'..='\u{2263}'
        | '\u{2266}'..='\u{2269}'
        | '\u{226D}'..='\u{228B}'
        | '\u{228D}'..='\u{22A4}'
        | '\u{22B2}'..='\u{22B5}'
        | '\u{22B8}'..='\u{22B8}'
        | '\u{22C2}'..='\u{22C6}'
        | '\u{22CD}'..='\u{22D1}'
        | '\u{22DA}'..='\u{22E9}'
        | '\u{22EF}'..='\u{22EF}'
        | '\u{2304}'..='\u{2304}'
        | '\u{2308}'..='\u{230B}'
        | '\u{2310}'..='\u{2310}'
        | '\u{2320}'..='\u{2321}'
        | '\u{239B}'..='\u{23AE}'
        | '\u{23FB}'..='\u{23FE}'
        | '\u{2500}'..='\u{25C9}'
        | '\u{25CB}'..='\u{25FF}'
        | '\u{2630}'..='\u{2630}'
        | '\u{2665}'..='\u{2665}'
        | '\u{266A}'..='\u{266A}'
        | '\u{26A1}'..='\u{26A1}'
        | '\u{2756}'..='\u{2756}'
        | '\u{2768}'..='\u{2775}'
        | '\u{2794}'..='\u{2794}'
        | '\u{2798}'..='\u{27AF}'
        | '\u{27B1}'..='\u{27BE}'
        | '\u{27C2}'..='\u{27C2}'
        | '\u{27C5}'..='\u{27C6}'
        | '\u{27DC}'..='\u{27DC}'
        | '\u{27E0}'..='\u{27E0}'
        | '\u{27E6}'..='\u{27EB}'
        | '\u{27F5}'..='\u{27F7}'
        | '\u{2987}'..='\u{2988}'
        | '\u{2997}'..='\u{2998}'
        | '\u{29EB}'..='\u{29EB}'
        | '\u{29FA}'..='\u{29FB}'
        | '\u{2A00}'..='\u{2A00}'
        | '\u{2A2F}'..='\u{2A2F}'
        | '\u{2A6A}'..='\u{2A6B}'
        | '\u{2B05}'..='\u{2B0D}'
        | '\u{2B16}'..='\u{2B1A}'
        | '\u{2B58}'..='\u{2B58}'
        | '\u{2E18}'..='\u{2E18}'
        | '\u{2E1F}'..='\u{2E1F}'
        | '\u{2E22}'..='\u{2E25}'
        | '\u{2E2E}'..='\u{2E2E}'
        | '\u{E000}'..='\u{E00A}'
        | '\u{E0A0}'..='\u{E0A3}'
        | '\u{E0B0}'..='\u{E0C8}'
        | '\u{E0CA}'..='\u{E0CA}'
        | '\u{E0CC}'..='\u{E0D2}'
        | '\u{E0D4}'..='\u{E0D4}'
        | '\u{E0D6}'..='\u{E0D7}'
        | '\u{E200}'..='\u{E2A9}'
        | '\u{E300}'..='\u{E3E3}'
        | '\u{E5FA}'..='\u{E6B5}'
        | '\u{E700}'..='\u{E7C5}'
        | '\u{EA60}'..='\u{EA88}'
        | '\u{EA8A}'..='\u{EA8C}'
        | '\u{EA8F}'..='\u{EAC7}'
        | '\u{EAC9}'..='\u{EAC9}'
        | '\u{EACC}'..='\u{EB09}'
        | '\u{EB0B}'..='\u{EB4E}'
        | '\u{EB50}'..='\u{EC1E}'
        | '\u{ED00}'..='\u{EDFF}'
        | '\u{EE0C}'..='\u{EFCE}'
        | '\u{F000}'..='\u{F375}'
        | '\u{F400}'..='\u{F533}'
        | '\u{F0001}'..='\u{F1AF0}' => Some(NERD_FONT.clone()),
        _ => None,
    }
}

#[cfg(test)]
#[path = "font_fallback_tests.rs"]
mod tests;
