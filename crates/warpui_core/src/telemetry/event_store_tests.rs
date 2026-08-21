use std::borrow::Cow;

use chrono::{TimeZone, Utc};

use crate::telemetry::event_store::EventStore;
use crate::telemetry::{
    flush_events, record_app_active_event, record_event, record_identify_user_event,
};
use crate::time::test_offset_time;

#[test]
fn test_initialize_session() {
    test_offset_time(5);
    let event_store = EventStore::new();
    assert_eq!(
        event_store.current_session_created_at,
        Utc.timestamp_opt(5, 0).unwrap()
    );
}

// The recording entry points are deliberately empty in this build. These pin
// that: whatever the several hundred emit sites across the tree call, nothing
// reaches the queue, so there is never anything to write to a file or send to a
// server.
#[test]
fn test_recording_an_event_stores_nothing() {
    let timestamp = Utc.timestamp_opt(1, 0).unwrap();
    record_event(
        Some("user123".to_string()),
        "anon-user-xyz".to_string(),
        Cow::Borrowed("SomeEvent"),
        None,
        false,
        timestamp,
    );
    assert!(flush_events().is_empty());
}

#[test]
fn test_identify_user_stores_nothing() {
    record_identify_user_event(
        "user123".to_string(),
        "anon-user-xyz".to_string(),
        Utc.timestamp_opt(1, 0).unwrap(),
    );
    assert!(flush_events().is_empty());
}

#[test]
fn test_app_active_stores_nothing() {
    record_app_active_event(
        Some("user123".to_string()),
        "anon-user-xyz".to_string(),
        Utc.timestamp_opt(1, 0).unwrap(),
    );
    assert!(flush_events().is_empty());
}
