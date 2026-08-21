use std::borrow::Cow;

use chrono::{DateTime, Utc};
use serde_json::Value;

// Telemetry is not collected in this build. The recording entry points are kept
// so the several hundred call sites across the tree still compile, but there is
// no store behind them and no code left that could write or send an event.
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
