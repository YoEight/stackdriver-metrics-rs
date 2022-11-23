use chrono::{DateTime, Utc};
use std::time::{Duration, Instant};

pub struct CachedDate {
    time: DateTime<Utc>,
    clock: Instant,
}

impl CachedDate {
    pub fn new() -> Self {
        Self {
            time: Utc::now(),
            clock: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.clock.elapsed()
    }

    pub fn reset(&mut self) {
        self.time = Utc::now();
        self.clock = Instant::now();
    }

    pub fn time(&self) -> DateTime<Utc> {
        self.time
    }
}
