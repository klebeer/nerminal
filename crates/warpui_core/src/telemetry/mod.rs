mod event_store;

use std::borrow::Cow;

use chrono::{DateTime, Utc};
use event_store::*;
pub use event_store::{Event, EventPayload};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use serde_json::Value;

lazy_static! {
    static ref TELEMETRY: Mutex<EventStore> = Mutex::new(EventStore::new());
}

#[macro_export]
macro_rules! record_telemetry_from_ctx {
    ($user_id: expr, $anonymous_id: expr, $name:expr, $payload: expr, $contains_ugc: expr, $ctx: expr) => {{
        let timestamp = $crate::time::get_current_time();
        $ctx.background_executor()
            .spawn(async move {
                $crate::telemetry::record_event(
                    $user_id,
                    $anonymous_id,
                    $name,
                    $payload,
                    $contains_ugc,
                    timestamp,
                )
            })
            .detach();
    }};
}

#[macro_export]
macro_rules! record_telemetry_on_executor {
    ($user_id: expr, $anonymous_id: expr, $name:expr, $payload: expr, $contains_ugc: expr, $executor: expr) => {{
        let timestamp = $crate::time::get_current_time();
        let _ = $executor
            .spawn(async move {
                $crate::telemetry::record_event(
                    $user_id,
                    $anonymous_id,
                    $name,
                    $payload,
                    $contains_ugc,
                    timestamp,
                )
            })
            .detach();
    }};
}

/// Creates a new `Event`, but does not record it. It is up to the caller to determine when, and
/// how, the event should be recorded.
pub fn create_event(
    user_id: Option<String>,
    anonymous_id: String,
    name: Cow<'static, str>,
    payload: Option<Value>,
    contains_ugc: bool,
    timestamp: DateTime<Utc>,
) -> Event {
    let mut telemetry = TELEMETRY.lock();
    telemetry.create_event(
        user_id,
        anonymous_id,
        name,
        payload,
        contains_ugc,
        timestamp,
    )
}

// The three recording entry points are deliberately empty. Several hundred call
// sites across the tree emit events, and the signatures are kept so they keep
// compiling, but nothing is ever stored: the queue stays empty, so there is
// nothing to flush to a file or a server no matter what the rest of the
// pipeline decides to do.
pub fn record_event(
    _user_id: Option<String>,
    _anonymous_id: String,
    _name: Cow<'static, str>,
    _payload: Option<Value>,
    _contains_ugc: bool,
    _timestamp: DateTime<Utc>,
) {
}

pub fn record_identify_user_event(
    _user_id: String,
    _anonymous_id: String,
    _timestamp: DateTime<Utc>,
) {
}

pub fn record_app_active_event(
    _user_id: Option<String>,
    _anonymous_id: String,
    _timestamp: DateTime<Utc>,
) {
}

pub fn flush_events() -> Vec<Event> {
    TELEMETRY.lock().events.drain(..).collect()
}
