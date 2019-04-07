use std::time;

pub struct Timer {
    target_delta: time::Duration,
    last_tick: time::Instant,
    accumulated_delta: time::Duration,
}

impl Timer {
    pub fn new(ticks_per_second: u16) -> Timer {
        let target = 1.0 / ticks_per_second as f64;
        let target_seconds = target.trunc();
        let target_nanos = target.fract() * 1e9;

        Timer {
            target_delta: time::Duration::new(
                target_seconds as u64,
                target_nanos as u32,
            ),
            last_tick: time::Instant::now(),
            accumulated_delta: time::Duration::from_secs(0),
        }
    }

    pub fn update(&mut self) {
        let now = time::Instant::now();
        let diff = now - self.last_tick;

        self.last_tick = now;
        self.accumulated_delta += diff;
    }

    pub fn tick(&mut self) -> bool {
        if self.accumulated_delta >= self.target_delta {
            self.accumulated_delta -= self.target_delta;
            true
        } else {
            false
        }
    }
}
