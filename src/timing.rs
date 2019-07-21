//! Provides tools to manage the game's clocks
//! timers, and ticks.
//!
//! Currently, this module just provides a simple
//! clock based on the system clock.

use std::time::{Duration, Instant};

// A timing primitive that keeps track of how much
// time has passed since the last reset.
//
// It can be used to time the duration of a game
// frame update.
//
// ``Clock`` is essentially a virtual stopwatch.
// Upon construction, the clock begins "timing
// automatically". Implmentation-wise, there is
// no actual background work being executed.
// Rather, the clock remembers a unique system
// timestamp and, when queried, calcuates the
// delta from that pinned time.
pub struct Clock {
}
