//! Module that builds a static context to attach to each of our events that are sent to Rudderstack.
//! This is needed so we know the backing operating system and version of each telemetry event.

use std::sync::OnceLock;

use serde::Serialize;
use serde_json::{Value, json};
use warp_errors::report_error;
#[cfg(target_family = "wasm")]
use warpui::platform::wasm;

use crate::server::OperatingSystemInfo;

static TELEMETRY_CONTEXT: OnceLock<TelemetryContext> = OnceLock::new();

#[derive(Serialize)]
struct TelemetryContextInfo {
    /// Info about the operating system of the client.
    #[serde(skip_serializing_if = "Option::is_none")]
    os: Option<&'static OperatingSystemInfo>,
    /// The user agent provided by the browser, if running on Web. If not on
    /// Web, this is always `None`.
    #[serde(rename = "userAgent", skip_serializing_if = "Option::is_none")]
    user_agent: Option<String>,
}

/// Newtype representing a [`Value`] with a serialized version of the context that we send to
/// Rudderstack.
/// See https://www.rudderstack.com/docs/event-spec/standard-events/common-fields/#contextual-fields.
pub struct TelemetryContext(Value);

impl TelemetryContext {
    pub fn as_value(&self) -> Value {
        self.0.clone()
    }
}

impl TelemetryContext {
    fn new() -> Self {
        let context = TelemetryContextInfo {
            os: OperatingSystemInfo::get().ok(),
            user_agent: user_agent(),
        };

        match serde_json::to_value(context) {
            Ok(value) => Self(value),
            Err(e) => {
                report_error!(
                    anyhow::Error::new(e)
                        .context("Failed to serialize telemetry context info to JSON value")
                );
                Self(json!({}))
            }
        }
    }
}

fn user_agent() -> Option<String> {
    cfg_if::cfg_if! {
        if #[cfg(target_family = "wasm")] {
            wasm::user_agent()
        } else {
            None
        }
    }
}

pub fn telemetry_context() -> &'static TelemetryContext {
    TELEMETRY_CONTEXT.get_or_init(TelemetryContext::new)
}
