// Telemetry is not collected in this build. These macros keep their shape so
// the several hundred call sites across the tree still compile and still type-
// check their event payloads, but there is no client left to send to and no
// store to queue into.

#[macro_export]
macro_rules! send_telemetry_sync_from_ctx {
    ($event:expr_2021, $ctx:expr_2021) => {
        let _ = (&$event, &$ctx);
    };
}

#[macro_export]
macro_rules! send_telemetry_sync_from_app_ctx {
    ($event:expr_2021, $app_ctx:expr_2021) => {
        let _ = (&$event, &$app_ctx);
    };
}

#[macro_export]
macro_rules! send_telemetry_on_executor {
    ($auth_state: expr_2021, $event:expr_2021, $executor:expr_2021) => {
        let _ = (&$auth_state, &$event, &$executor);
    };
}
