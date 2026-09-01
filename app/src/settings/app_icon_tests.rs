use enum_iterator::all;
use settings_value::SettingsValue;

use super::{AppIcon, RETIRED};

#[test]
fn every_variant_survives_a_trip_through_the_settings_file() {
    for icon in all::<AppIcon>() {
        let written = icon.to_file_value();
        assert_eq!(
            AppIcon::from_file_value(&written),
            Some(icon),
            "{icon} did not read back"
        );
    }
}

#[test]
fn file_names_are_the_snake_case_the_derive_used_to_write() {
    assert_eq!(AppIcon::Ruthven.file_name(), "ruthven");
    assert_eq!(AppIcon::BloodMoon.file_name(), "blood_moon");
    assert_eq!(AppIcon::Totem.file_name(), "totem");
}

#[test]
fn file_names_are_unique() {
    let mut names: Vec<_> = all::<AppIcon>().map(AppIcon::file_name).collect();
    let total = names.len();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), total, "two variants share a file name");
}

#[test]
fn a_retired_icon_reads_as_the_default() {
    for name in RETIRED {
        let stored = serde_json::Value::String((*name).to_owned());
        assert_eq!(
            AppIcon::from_file_value(&stored),
            Some(AppIcon::default()),
            "{name} should fall back rather than fail to parse"
        );
    }
}

#[test]
fn a_retired_name_never_collides_with_a_live_one() {
    for icon in all::<AppIcon>() {
        assert!(
            !RETIRED.contains(&icon.file_name()),
            "{icon} reuses a retired name, so it would be shadowed by the fallback"
        );
    }
}

#[test]
fn an_unknown_name_still_fails_to_parse() {
    let stored = serde_json::Value::String("laserdisc".to_owned());
    assert_eq!(AppIcon::from_file_value(&stored), None);
}
