//! macOS secure event input.
//!
//! While the flag is held the WindowServer stops handing keystrokes to processes
//! watching the event monitor target, so keyloggers, text expanders and input
//! methods no longer see what is typed. macOS protects native password fields on
//! its own, but a `sudo` prompt is just a shell reading stdin with the echo off,
//! which the system cannot recognise as a password.
//!
//! The flag is session-wide rather than per-window, so it is held only while this
//! app is frontmost: Apple's guidance is to release it as soon as the application
//! becomes inactive, otherwise every other app loses those services too.

use std::sync::Mutex;

#[cfg(not(test))]
unsafe extern "C" {
    fn EnableSecureEventInput() -> i32;
    fn DisableSecureEventInput() -> i32;
}

#[derive(Default)]
struct State {
    /// The user turned the setting on.
    requested: bool,
    /// This app is frontmost.
    app_active: bool,
    /// This process is holding the flag right now. `EnableSecureEventInput` is
    /// reference counted, so an unbalanced call leaks a hold that nothing but
    /// process death will clear.
    held: bool,
}

static STATE: Mutex<State> = Mutex::new(State {
    requested: false,
    app_active: false,
    held: false,
});

/// Turns the OS flag on or off. Split out so the tests can record the calls
/// instead of putting the whole machine into secure input.
#[cfg(not(test))]
fn apply_to_os(hold: bool) -> i32 {
    // SAFETY: neither call takes arguments or returns an owned resource. Both are
    // documented as not thread safe, and every caller below reaches this through
    // `STATE`'s lock.
    unsafe {
        if hold {
            EnableSecureEventInput()
        } else {
            DisableSecureEventInput()
        }
    }
}

#[cfg(test)]
fn apply_to_os(hold: bool) -> i32 {
    tests::record_call(hold)
}

fn sync(update: impl FnOnce(&mut State)) {
    let mut state = STATE
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    update(&mut state);

    let should_hold = state.requested && state.app_active;
    if should_hold == state.held {
        return;
    }

    let status = apply_to_os(should_hold);
    if status != 0 {
        log::warn!(
            "secure event input {} failed with status {status}",
            if should_hold { "enable" } else { "disable" }
        );
        return;
    }
    state.held = should_hold;
}

/// Records whether the user wants secure input, applying it if the app is already
/// frontmost.
pub(crate) fn set_requested(requested: bool) {
    sync(|state| state.requested = requested);
}

/// The activation callbacks only fire on a change, so callers that flip the
/// setting while the app is already frontmost have to seed this themselves.
pub(crate) fn set_app_active(app_active: bool) {
    sync(|state| state.app_active = app_active);
}

/// Drops the hold on the way out. The WindowServer also releases it when the
/// process dies, so this only tidies up the orderly quit.
pub(crate) fn release() {
    sync(|state| {
        state.requested = false;
        state.app_active = false;
    });
}

#[cfg(test)]
#[path = "secure_input_tests.rs"]
mod tests;
