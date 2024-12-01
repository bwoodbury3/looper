use std::time::{Duration, Instant};

/// A timer which can be started and stopped.
pub struct Timer {
    /// The start time of the timer.
    monotonic_start: Instant,

    /// The pause duration of the timer.
    pause_duration: Duration,

    /// The last pause time.
    last_pause_time: Instant,

    /// Whether the timer is paused.
    paused: bool,

    /// Whether the timer is stopped.
    stopped: bool,
}

impl Timer {
    /// Start a timer.
    pub fn start() -> Self {
        Timer {
            monotonic_start: Instant::now(),
            last_pause_time: Instant::now(),
            pause_duration: Duration::ZERO,
            paused: false,
            stopped: false,
        }
    }

    /// Pause a timer.
    pub fn pause(&mut self) {
        if self.stopped {
            panic!("Dead timer may not be paused.");
        }

        if !self.paused {
            self.paused = true;
            self.last_pause_time = Instant::now();
        }
    }

    /// Resume a timer. Returns the pause duration.
    pub fn resume(&mut self) -> u128 {
        if self.stopped {
            panic!("Dead timer may not be resumed.");
        }
        if !self.paused {
            panic!("Cannot resume a timer that was not paused.");
        }

        self.paused = false;
        let pause_duration = self.last_pause_time.elapsed();
        self.pause_duration += pause_duration;

        return pause_duration.as_millis();
    }

    /// Stop a timer.
    pub fn stop(&mut self) -> u128 {
        if self.stopped {
            panic!("Dead timer may not be stopped.");
        }

        self.stopped = true;
        let total = self.monotonic_start.elapsed() - self.pause_duration;
        return total.as_millis();
    }
}
