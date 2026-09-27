use turbo::*;

use crate::config;

// --- Client Timer ---

#[turbo::serialize]
pub struct ClientTimer {
    pub interval_ms: u64,
    pub last_reset_time_ms: u64,
}

impl ClientTimer {
    pub fn new(interval_ms: u64) -> Self {
        Self {
            interval_ms,
            last_reset_time_ms: time::now(),
        }
    }

    pub fn ready(&self) -> bool {
        self.elapsed_ms() >= self.interval_ms
    }

    pub fn reset(&mut self) {
        self.last_reset_time_ms = time::now();
    }

    pub fn check_and_reset(&mut self) -> bool {
        if self.ready() {
            self.reset();
            return true;
        }

        false
    }

    // Helpers
    pub fn set_interval(&mut self, interval_ms: u64) {
        self.interval_ms = interval_ms;
    }

    pub fn elapsed_ms(&self) -> u64 {
        time::now() - self.last_reset_time_ms
    }
}

// --- Server Timer ---

#[turbo::serialize]
pub struct ServerTimer {
    tick_count: u64,
    last_reset_tick: u64,
    interval_ticks: u64,
}

impl ServerTimer {
    pub fn new(interval_ms: u64) -> Self {
        let interval_ticks = interval_ms.div_ceil(config::SERVER_INTERVAL_RATE_MS as u64);
        Self {
            tick_count: 0,
            last_reset_tick: 0,
            interval_ticks,
        }
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
    }

    pub fn ready(&self) -> bool {
        self.elapsed_ticks() >= self.interval_ticks
    }

    pub fn reset(&mut self) {
        self.last_reset_tick = self.tick_count;
    }

    pub fn check_and_reset(&mut self) -> bool {
        if self.ready() {
            self.reset();
            return true;
        }
        false
    }

    // Helpers
    pub fn set_interval(&mut self, interval_ms: u64) {
        self.interval_ticks = interval_ms.div_ceil(config::SERVER_INTERVAL_RATE_MS as u64);
    }

    pub fn elapsed_ticks(&self) -> u64 {
        self.tick_count - self.last_reset_tick
    }

    pub fn elapsed_ms(&self) -> u64 {
        self.elapsed_ticks() * config::SERVER_INTERVAL_RATE_MS as u64
    }
}
