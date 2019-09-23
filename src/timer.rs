use std::time;

/// The timer of your game state.
///
/// A [`Timer`] is updated once per frame, and it ticks [`Game::TICKS_PER_SECOND`]
/// times every second. When the timer ticks, your game is updated.
///
/// [`Timer`]: struct.Timer.html
/// [`Game::TICKS_PER_SECOND`]: trait.Game.html#associatedconstant.TICKS_PER_SECOND
#[derive(Debug)]
pub struct Timer {
    target_ticks: u16,
    target_delta: time::Duration,
    last_tick: time::Instant,
    accumulated_delta: time::Duration,
    has_ticked: bool,
}

impl Timer {
    pub(crate) fn new(ticks_per_second: u16) -> Timer {
        let (target_seconds, target_nanos) = match ticks_per_second {
            0 => (std::u64::MAX, 0),
            1 => (1, 0),
            _ => (0, ((1.0 / ticks_per_second as f64) * 1e9) as u32),
        };

        Timer {
            target_ticks: ticks_per_second,
            target_delta: time::Duration::new(target_seconds, target_nanos),
            last_tick: time::Instant::now(),
            accumulated_delta: time::Duration::from_secs(0),
            has_ticked: false,
        }
    }

    pub(crate) fn update(&mut self) {
        let now = time::Instant::now();
        let diff = now - self.last_tick;

        self.last_tick = now;
        self.accumulated_delta += diff;
        self.has_ticked = false;
    }

    pub(crate) fn tick(&mut self) -> bool {
        if self.accumulated_delta >= self.target_delta {
            self.accumulated_delta -= self.target_delta;
            self.has_ticked = true;

            true
        } else {
            false
        }
    }

    /// Returns `true` if the [`Timer`] has ticked since its last update.
    ///
    /// This tells you whether your game has been updated or not during a frame.
    ///
    /// You can use this to avoid computations during [`Game::draw`] when your
    /// game has not been updated during a particular frame.
    ///
    /// [`Timer`]: struct.Timer.html
    /// [`Game::draw`]: trait.Game.html#tymethod.draw
    pub fn has_ticked(&self) -> bool {
        self.has_ticked
    }

    /// Returns how close the next tick is.
    ///
    /// The returned value is in the `[0.0, 1.0]` interval. You should use this
    /// value in your [`Game::draw`] function to perform _graphics
    /// interpolation_. You can read more about it in [this excellent article].
    ///
    /// [`Game::draw`]: trait.Game.html#tymethod.draw
    /// [this excellent article]: http://web.archive.org/web/20190506030345/https://gafferongames.com/post/fix_your_timestep/
    pub fn next_tick_proximity(&self) -> f32 {
        let delta = self.accumulated_delta;

        self.target_ticks as f32
            * (delta.as_secs() as f32
                + (delta.subsec_micros() as f32 / 1_000_000.0))
    }
}
