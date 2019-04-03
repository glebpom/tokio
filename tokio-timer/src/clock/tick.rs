//! An optimized source of time, which calculates only once per tick

use std::cell::RefCell;
use std::time::Instant;

use crate::clock;

/// Return the time which is calculated only once per tick
pub fn tick() -> Instant {
    TICK_CLOCK.with(|state| {
        let tc = &mut *state.borrow_mut();
        match tc {
            TickClockState::Retrieved(instant) => {
                return *instant;
            }
            TickClockState::Disallowed => panic!("attempt to set on uninitialized tick_clock"),
            _ => {}
        };
        let now = clock::now();
        *tc = TickClockState::Retrieved(now);
        now
    })
}

/// Remove current `Instant` from `TICK_CLOCK`
pub fn clear_tick_clock() {
    TICK_CLOCK.with(|tc| {
        *tc.borrow_mut() = TickClockState::Empty;
    })
}

/// Mark `TICK_CLOCK` as disallowed. Should be called after exiting from the main loop.
/// Calling `tick` function will lead to panic in this state
pub fn disallow_tick_clock() {
    TICK_CLOCK.with(|tc| {
        *tc.borrow_mut() = TickClockState::Disallowed;
    })
}

enum TickClockState {
    Disallowed,
    Empty,
    Retrieved(Instant),
}

thread_local! {
    static TICK_CLOCK: RefCell<TickClockState> = RefCell::new(TickClockState::Disallowed);
}