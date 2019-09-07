//! Provides tools to manage the game's clocks
//! timers, and ticks.
//!
//! Currently, this module just provides a simple
//! clock based on the system clock.

use std::time::{Duration, Instant};

/// A timing primitive that keeps track of how much
/// time has passed since the last reset.
///
/// It can be used to time the duration of a game
/// frame update.
///
/// ``Clock`` is essentially a virtual stopwatch.
/// Upon construction, the clock begins "timing
/// automatically". Implmentation-wise, there is
/// no actual background work being executed.
/// Rather, the clock remembers a unique system
/// timestamp and, when queried, calcuates the
/// delta from that pinned time.
pub struct Clock {
    last_restart: Instant,
}

impl Clock {
    /// Create a new clock and start counting
    /// immediately.
    pub fn begin() -> Clock {
        Clock {
            last_restart: Instant::now(),
        }
    }

    /// Return the amount of time that has
    /// elapsed since the clock was created
    /// or last restarted.
    pub fn elapsed(&self) -> Duration {
        Instant::now() - self.last_restart
    }

    /// Return the amount of time in seconds
    /// that has elapsed since the clock was
    /// created or last restarted.
    pub fn elapsed_seconds(&self) -> f64 {
        dur_sec(self.elapsed())
    }

    /// Reset the clock's elapsed time to
    /// zero, returning the time that had
    /// elapsed before the reset.
    pub fn restart(&mut self) -> Duration {
        let now = Instant::now();
        let elapsed = now - self.last_restart;

        self.last_restart = now;

        elapsed
    }

    /// Reset the clock's elapsed time to
    /// zero, returning the time that had
    /// elapsed before the reset in seconds.
    pub fn restart_seconds(&mut self) -> f64 {
        dur_sec(self.restart())
    }
}

/// Return the number of fractional seconds
/// represented by the ``Duration``.
///
/// This function is limited in precision by
/// the floating point representation, and is
/// less precise than the ``Duration``
/// struct, which represents lengths of time as
/// whole units.
pub fn dur_sec(dur: Duration) -> f64 {
    dur.as_secs() as f64 + dur.subsec_nanos() as f64 * 1e-9
}
