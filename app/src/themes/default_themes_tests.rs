use warp_core::ui::color::contrast::relative_luminance;

use super::*;

/// WCAG AA for body text. Terminal text is small, dense and read for hours, so
/// the floor is applied at each end of the gradient rather than to its average:
/// a colour that only passes at the darker end vanishes in the last rows of a
/// long listing.
const CONTRAST_FLOOR: f32 = 4.5;

/// https://www.w3.org/TR/WCAG20/#contrast-ratiodef
fn contrast_ratio(one: ColorU, other: ColorU) -> f32 {
    let one = relative_luminance(one) + 0.05;
    let other = relative_luminance(other) + 0.05;
    if one > other { one / other } else { other / one }
}

/// The two ends a foreground can land on. Rows between them sit on interpolated
/// values, so the endpoints bound the whole window.
fn background_stops(theme: &WarpTheme) -> Vec<(&'static str, ColorU)> {
    match theme.background() {
        Fill::Solid(color) => vec![("background", color)],
        Fill::VerticalGradient(gradient) => vec![
            ("background top", gradient.top()),
            ("background bottom", gradient.bottom()),
        ],
        Fill::HorizontalGradient(_) => panic!("no bundled theme uses a horizontal gradient"),
    }
}

/// Everything the theme can draw text in: the foreground, the accent, and both
/// ANSI banks. The background stops are what these are measured against.
fn foreground_slots(theme: &WarpTheme) -> Vec<(String, ColorU)> {
    let mut slots = vec![
        ("foreground".to_string(), theme.foreground().into_solid()),
        ("accent".to_string(), theme.accent().into_solid()),
    ];

    let colors = theme.terminal_colors();
    for (bank, ansi) in [("", &colors.normal), ("bright ", &colors.bright)] {
        for (name, color) in [
            ("black", ansi.black),
            ("red", ansi.red),
            ("green", ansi.green),
            ("yellow", ansi.yellow),
            ("blue", ansi.blue),
            ("magenta", ansi.magenta),
            ("cyan", ansi.cyan),
            ("white", ansi.white),
        ] {
            slots.push((format!("{bank}{name}"), color.into()));
        }
    }

    slots
}

fn assert_meets_contrast_floor(theme: &WarpTheme) {
    let stops = background_stops(theme);

    let failures: Vec<String> = foreground_slots(theme)
        .into_iter()
        .flat_map(|(slot, color)| {
            stops.iter().filter_map(move |(stop, background)| {
                let ratio = contrast_ratio(color, *background);
                (ratio < CONTRAST_FLOOR).then(|| format!("{slot} on {stop}: {ratio:.2}:1"))
            })
        })
        .collect();

    assert!(
        failures.is_empty(),
        "{} slots below {CONTRAST_FLOOR}:1 -\n  {}",
        theme.name().unwrap_or_default(),
        failures.join("\n  ")
    );
}

/// The accent in both themes is far enough from the foreground's lightness that
/// drawing one over the other fails. Nothing may fill with the accent and then
/// draw `foreground()` on top; the helper that picks a readable pair has to be
/// used instead.
fn assert_accent_fill_gets_a_readable_text_color(theme: &WarpTheme) {
    let accent = theme.accent().into_solid();

    assert!(
        contrast_ratio(theme.foreground().into_solid(), accent) < CONTRAST_FLOOR,
        "{}: the raw foreground now passes on the accent, so this guard is stale",
        theme.name().unwrap_or_default()
    );

    let picked = theme.main_text_color(theme.accent()).into_solid();
    let ratio = contrast_ratio(picked, accent);
    assert!(
        ratio >= CONTRAST_FLOOR,
        "{}: main_text_color picked {picked:?} on the accent: {ratio:.2}:1",
        theme.name().unwrap_or_default()
    );
}

#[test]
fn nosferatu_meets_contrast_floor() {
    assert_meets_contrast_floor(&nosferatu());
}

#[test]
fn nosferatu_light_meets_contrast_floor() {
    assert_meets_contrast_floor(&nosferatu_light());
}

#[test]
fn nosferatu_accent_fill_gets_a_readable_text_color() {
    assert_accent_fill_gets_a_readable_text_color(&nosferatu());
}

#[test]
fn nosferatu_light_accent_fill_gets_a_readable_text_color() {
    assert_accent_fill_gets_a_readable_text_color(&nosferatu_light());
}

/// On a light background the emphatic version of a colour has to gain contrast,
/// which means going darker. This inverts the usual ANSI intuition, so it is
/// worth pinning: a well-meaning "fix" that brightens the bright bank would make
/// bold text weaker than plain text.
#[test]
fn nosferatu_light_brights_are_darker_than_their_normals() {
    let theme = nosferatu_light();
    let colors = theme.terminal_colors();

    for (name, normal, bright) in [
        ("black", colors.normal.black, colors.bright.black),
        ("red", colors.normal.red, colors.bright.red),
        ("green", colors.normal.green, colors.bright.green),
        ("yellow", colors.normal.yellow, colors.bright.yellow),
        ("blue", colors.normal.blue, colors.bright.blue),
        ("magenta", colors.normal.magenta, colors.bright.magenta),
        ("cyan", colors.normal.cyan, colors.bright.cyan),
        ("white", colors.normal.white, colors.bright.white),
    ] {
        let (normal, bright) = (normal.into(), bright.into());
        assert!(
            relative_luminance(bright) < relative_luminance(normal),
            "bright {name} is lighter than normal {name}"
        );
    }
}
