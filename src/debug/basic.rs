use std::time;

use crate::graphics;

/// A bunch of performance information about your game. It can be drawn!
///
/// ![Debug information][debug]
///
/// This is the default debug information that will be shown when the
/// [`Game::DEBUG_KEY`] is pressed.
///
/// Overriding [`Game::debug`] gives you access to this struct, allowing you to
/// implement your own debug view.
///
/// [`Game`]: trait.Game.html
/// [debug]: https://github.com/hecrj/coffee/blob/50c9a857e476266d8bd37f705266bd66b77c0f2d/images/debug.png?raw=true
/// [`Game::DEBUG_KEY`]: trait.Game.html#associatedconstant.DEBUG_KEY
/// [`Game::debug`]: trait.Game.html#method.debug
pub struct Debug {
    font: graphics::Font,
    enabled: bool,
    load_start: time::Instant,
    load_duration: time::Duration,
    frame_start: time::Instant,
    frame_durations: TimeBuffer,
    interact_start: time::Instant,
    interact_duration: time::Duration,
    update_start: time::Instant,
    update_durations: TimeBuffer,
    draw_start: time::Instant,
    draw_durations: TimeBuffer,
    ui_start: time::Instant,
    ui_durations: TimeBuffer,
    debug_start: time::Instant,
    debug_durations: TimeBuffer,
    text: Vec<(String, String)>,
    draw_rate: u16,
    frames_until_refresh: u16,
}

impl Debug {
    pub(crate) fn new(gpu: &mut graphics::Gpu) -> Self {
        let now = time::Instant::now();

        Self {
            font: graphics::Font::from_bytes(gpu, graphics::Font::DEFAULT)
                .expect("Load debug font"),
            enabled: cfg!(feature = "debug"),
            load_start: now,
            load_duration: time::Duration::from_secs(0),
            frame_start: now,
            frame_durations: TimeBuffer::new(200),
            interact_start: now,
            interact_duration: time::Duration::from_secs(0),
            update_start: now,
            update_durations: TimeBuffer::new(200),
            draw_start: now,
            draw_durations: TimeBuffer::new(200),
            ui_start: now,
            ui_durations: TimeBuffer::new(200),
            debug_start: now,
            debug_durations: TimeBuffer::new(200),
            text: Vec::new(),
            draw_rate: 10,
            frames_until_refresh: 0,
        }
    }

    pub(crate) fn loading_started(&mut self) {
        self.load_start = time::Instant::now();
    }

    pub(crate) fn loading_finished(&mut self) {
        self.load_duration = time::Instant::now() - self.load_start;
    }

    /// Returns the time spent loading your [`Game`].
    ///
    /// [`Game`]: trait.Game.html
    pub fn load_duration(&self) -> time::Duration {
        self.load_duration
    }

    pub(crate) fn frame_started(&mut self) {
        self.frame_start = time::Instant::now();
    }
    pub(crate) fn frame_finished(&mut self) {
        self.frame_durations
            .push(time::Instant::now() - self.frame_start);
    }

    /// Returns the average time spent per frame.
    ///
    /// It includes time spent on V-Sync, if enabled.
    pub fn frame_duration(&self) -> time::Duration {
        self.frame_durations.average()
    }

    pub(crate) fn interact_started(&mut self) {
        self.interact_start = time::Instant::now();
    }

    pub(crate) fn interact_finished(&mut self) {
        self.interact_duration = time::Instant::now() - self.interact_start;
    }

    /// Returns the average time spent processing events and running
    /// [`Game::interact`].
    ///
    /// [`Game::interact`]: trait.Game.html#method.interact
    pub fn interact_duration(&self) -> time::Duration {
        self.interact_duration
    }

    pub(crate) fn update_started(&mut self) {
        self.update_start = time::Instant::now();
    }

    pub(crate) fn update_finished(&mut self) {
        self.update_durations
            .push(time::Instant::now() - self.update_start);
    }

    /// Returns the average time spent running [`Game::update`].
    ///
    /// [`Game::update`]: trait.Game.html#tymethod.update
    pub fn update_duration(&self) -> time::Duration {
        self.update_durations.average()
    }

    pub(crate) fn draw_started(&mut self) {
        self.draw_start = time::Instant::now();
    }

    pub(crate) fn draw_finished(&mut self) {
        let duration = time::Instant::now() - self.draw_start;

        if duration.subsec_micros() > 0 {
            self.draw_durations.push(duration);
        }
    }

    /// Returns the average time spent running [`Game::draw`].
    ///
    /// [`Game::draw`]: trait.Game.html#tymethod.draw
    pub fn draw_duration(&self) -> time::Duration {
        self.draw_durations.average()
    }

    pub(crate) fn ui_started(&mut self) {
        self.ui_start = time::Instant::now();
    }

    pub(crate) fn ui_finished(&mut self) {
        self.ui_durations.push(time::Instant::now() - self.ui_start);
    }

    /// Returns the average time spent rendering the [`UserInterface`].
    ///
    /// [`UserInterface`]: ui/trait.UserInterface.html
    pub fn ui_duration(&self) -> time::Duration {
        self.ui_durations.average()
    }

    pub(crate) fn toggle(&mut self) {
        self.enabled = !self.enabled;
        self.frames_until_refresh = 0;
    }

    pub(crate) fn debug_started(&mut self) {
        self.debug_start = time::Instant::now();
    }

    pub(crate) fn debug_finished(&mut self) {
        self.debug_durations
            .push(time::Instant::now() - self.debug_start);
    }

    /// Returns the average time spent running [`Game::debug`].
    ///
    /// [`Game::debug`]: trait.Game.html#tymethod.debug
    pub fn debug_duration(&self) -> time::Duration {
        self.debug_durations.average()
    }

    pub(crate) fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Draws the [`Debug`] information.
    ///
    /// [`Debug`]: struct.Debug.html
    pub fn draw(&mut self, frame: &mut graphics::Frame<'_>) {
        if self.frames_until_refresh <= 0 {
            self.text.clear();
            self.refresh_text();
            self.frames_until_refresh = self.draw_rate.max(1);
        }

        self.draw_text(frame);
        self.frames_until_refresh -= 1;
    }

    const MARGIN: f32 = 20.0;
    const ROW_HEIGHT: f32 = 25.0;
    const TITLE_WIDTH: f32 = 150.0;
    const SHADOW_OFFSET: f32 = 2.0;

    fn refresh_text(&mut self) {
        let frame_duration = self.frame_durations.average();
        let frame_micros = (frame_duration.as_secs() as u32 * 1_000_000
            + frame_duration.subsec_micros())
        .max(1);

        let fps = (1_000_000.0 / frame_micros as f32).round() as u32;
        let rows = [
            ("Load:", self.load_duration, None),
            ("Interact:", self.interact_duration, None),
            ("Update:", self.update_duration(), None),
            ("Draw:", self.draw_duration(), None),
            ("UI:", self.ui_duration(), None),
            ("Debug:", self.debug_duration(), None),
            ("Frame:", frame_duration, Some(fps.to_string() + " fps")),
        ];

        for (title, duration, extra) in rows.iter() {
            let formatted_duration = match extra {
                Some(string) => format_duration(duration) + " (" + string + ")",
                None => format_duration(duration),
            };

            self.text.push((String::from(*title), formatted_duration));
        }
    }

    fn draw_text(&mut self, frame: &mut graphics::Frame<'_>) {
        for (row, (key, value)) in self.text.iter().enumerate() {
            let y = row as f32 * Self::ROW_HEIGHT;

            self.font.add(graphics::Text {
                content: key,
                position: graphics::Point::new(
                    Self::MARGIN + Self::SHADOW_OFFSET,
                    Self::MARGIN + y + Self::SHADOW_OFFSET,
                ),
                size: 20.0,
                color: graphics::Color::BLACK,
                ..graphics::Text::default()
            });

            self.font.add(graphics::Text {
                content: key,
                position: graphics::Point::new(Self::MARGIN, Self::MARGIN + y),
                size: 20.0,
                color: graphics::Color::WHITE,
                ..graphics::Text::default()
            });

            self.font.add(graphics::Text {
                content: value,
                position: graphics::Point::new(
                    Self::MARGIN + Self::TITLE_WIDTH + Self::SHADOW_OFFSET,
                    Self::MARGIN + y + Self::SHADOW_OFFSET,
                ),
                size: 20.0,
                color: graphics::Color::BLACK,
                ..graphics::Text::default()
            });

            self.font.add(graphics::Text {
                content: value,
                position: graphics::Point::new(
                    Self::MARGIN + Self::TITLE_WIDTH,
                    Self::MARGIN + y,
                ),
                size: 20.0,
                color: graphics::Color::WHITE,
                ..graphics::Text::default()
            });
        }

        self.font.draw(&mut frame.as_target());
    }
}

fn format_duration(duration: &time::Duration) -> String {
    let seconds = duration.as_secs();

    if seconds > 0 {
        seconds.to_string()
            + "."
            + &format!("{:03}", duration.subsec_millis())
            + " s"
    } else {
        let millis = duration.subsec_millis();

        if millis > 0 {
            millis.to_string()
                + "."
                + &format!("{:03}", (duration.subsec_micros() - millis * 1000))
                + " ms"
        } else {
            duration.subsec_micros().to_string() + " µs"
        }
    }
}

impl std::fmt::Debug for Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Debug {{ load: {:?}, interact: {:?}, update: {:?}, draw: {:?}, frame: {:?} }}",
            self.load_duration(),
            self.interact_duration(),
            self.update_duration(),
            self.draw_duration(),
            self.frame_duration(),
        )
    }
}

struct TimeBuffer {
    head: usize,
    size: usize,
    contents: Vec<time::Duration>,
}

impl TimeBuffer {
    fn new(capacity: usize) -> TimeBuffer {
        TimeBuffer {
            head: 0,
            size: 0,
            contents: vec![time::Duration::from_secs(0); capacity],
        }
    }

    fn push(&mut self, duration: time::Duration) {
        self.head = (self.head + 1) % self.contents.len();
        self.contents[self.head] = duration;
        self.size = (self.size + 1).min(self.contents.len());
    }

    fn average(&self) -> time::Duration {
        let sum: time::Duration = if self.size == self.contents.len() {
            self.contents[..].iter().sum()
        } else {
            self.contents[..self.size].iter().sum()
        };

        sum / self.size.max(1) as u32
    }
}
