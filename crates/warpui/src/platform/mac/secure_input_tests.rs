use std::sync::MutexGuard;

use super::*;

/// Every `apply_to_os` call the code under test made, in order. `true` is an
/// enable, `false` a disable.
static CALLS: Mutex<Vec<bool>> = Mutex::new(Vec::new());
/// Lets a test make the next OS call report a failure.
static FORCED_STATUS: Mutex<Option<i32>> = Mutex::new(None);
/// `STATE` is process-wide, so the tests take turns.
static TEST_LOCK: Mutex<()> = Mutex::new(());

pub(super) fn record_call(hold: bool) -> i32 {
    CALLS.lock().unwrap().push(hold);
    FORCED_STATUS.lock().unwrap().take().unwrap_or(0)
}

struct TestRun(#[allow(dead_code)] MutexGuard<'static, ()>);

fn begin() -> TestRun {
    let guard = TEST_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    *STATE.lock().unwrap() = State::default();
    CALLS.lock().unwrap().clear();
    *FORCED_STATUS.lock().unwrap() = None;
    TestRun(guard)
}

fn calls() -> Vec<bool> {
    CALLS.lock().unwrap().clone()
}

fn fail_next_call() {
    *FORCED_STATUS.lock().unwrap() = Some(-1);
}

fn is_held() -> bool {
    STATE.lock().unwrap().held
}

#[test]
fn the_setting_alone_does_not_take_the_flag() {
    let _run = begin();

    set_requested(true);

    assert!(calls().is_empty(), "nothing should happen while unfocused");
    assert!(!is_held());
}

#[test]
fn focus_alone_does_not_take_the_flag() {
    let _run = begin();

    set_app_active(true);

    assert!(calls().is_empty(), "the setting is still off");
    assert!(!is_held());
}

#[test]
fn the_flag_is_taken_once_both_agree() {
    let _run = begin();

    set_requested(true);
    set_app_active(true);

    assert_eq!(calls(), vec![true]);
    assert!(is_held());
}

#[test]
fn the_hold_follows_focus() {
    let _run = begin();
    set_requested(true);

    set_app_active(true);
    set_app_active(false);
    set_app_active(true);

    assert_eq!(calls(), vec![true, false, true]);
    assert!(is_held());
}

#[test]
fn the_flag_is_never_taken_twice() {
    let _run = begin();
    set_requested(true);

    set_app_active(true);
    set_app_active(true);
    set_requested(true);

    assert_eq!(
        calls(),
        vec![true],
        "an unbalanced enable leaks a hold no disable can clear"
    );
}

#[test]
fn it_is_never_released_when_it_was_never_taken() {
    let _run = begin();

    set_app_active(false);
    set_requested(false);
    release();

    assert!(calls().is_empty());
}

#[test]
fn turning_the_setting_off_releases_while_still_focused() {
    let _run = begin();
    set_requested(true);
    set_app_active(true);

    set_requested(false);

    assert_eq!(calls(), vec![true, false]);
    assert!(!is_held());
}

#[test]
fn release_drops_an_active_hold() {
    let _run = begin();
    set_requested(true);
    set_app_active(true);

    release();

    assert_eq!(calls(), vec![true, false]);
    assert!(!is_held());
}

#[test]
fn regaining_focus_after_release_does_not_re_arm_it() {
    let _run = begin();
    set_requested(true);
    set_app_active(true);
    release();

    set_app_active(true);

    assert_eq!(
        calls(),
        vec![true, false],
        "release also clears the request"
    );
    assert!(!is_held());
}

#[test]
fn a_failed_os_call_leaves_the_hold_unrecorded() {
    let _run = begin();
    set_requested(true);
    fail_next_call();

    set_app_active(true);

    assert_eq!(calls(), vec![true]);
    assert!(!is_held(), "a failed enable must not be recorded as held");
}

#[test]
fn a_failed_enable_is_retried_on_the_next_transition() {
    let _run = begin();
    set_requested(true);
    fail_next_call();
    set_app_active(true);

    set_app_active(false);
    set_app_active(true);

    assert_eq!(
        calls(),
        vec![true, true],
        "the failed enable left nothing to disable, so only enables are attempted"
    );
    assert!(is_held());
}
