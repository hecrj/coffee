use std::time;

use crate::graphics;

#[cfg(debug_assertions)]
pub struct Debug {
    font: graphics::Font,
    update_start: time::Instant,
    update_durations: TimeBuffer,
    draw_start: time::Instant,
    draw_durations: TimeBuffer,
}

#[cfg(debug_assertions)]
impl Debug {
    pub fn new(gpu: &mut graphics::Gpu) -> Self {
        let now = time::Instant::now();

        Self {
            font: graphics::Font::from_bytes(
                gpu,
                include_bytes!("font/Inconsolata-Regular.ttf"),
            ),
            update_start: now,
            update_durations: TimeBuffer::new(200),
            draw_start: now,
            draw_durations: TimeBuffer::new(200),
        }
    }

    pub(super) fn loading_started(&mut self) {}

    pub(super) fn loading_finished(&mut self) {}

    pub(super) fn frame_started(&mut self) {}
    pub(super) fn frame_finished(&mut self) {}

    pub(super) fn event_loop_started(&mut self) {}
    pub(super) fn event_loop_finished(&mut self) {}

    pub(super) fn update_started(&mut self) {
        self.update_start = time::Instant::now();
    }

    pub(super) fn update_finished(&mut self) {
        self.update_durations
            .push(time::Instant::now() - self.update_start);
    }

    pub(super) fn draw_started(&mut self) {
        self.draw_start = time::Instant::now();
    }

    pub(super) fn draw_finished(&mut self) {
        let duration = time::Instant::now() - self.draw_start;

        if duration.subsec_micros() > 0 {
            self.draw_durations.push(duration);
        }
    }

    pub fn draw(
        &mut self,
        frame: &mut graphics::Frame,
    ) -> graphics::Result<()> {
        self.font.add(graphics::Text {
            content: String::from("Update:"),
            position: graphics::Vector::new(2.0, 2.0),
            size: 20.0,
            color: graphics::Color::new(0.0, 0.0, 0.0, 1.0),
        });

        self.font.add(graphics::Text {
            content: String::from("Update:"),
            position: graphics::Vector::new(0.0, 0.0),
            size: 20.0,
            color: graphics::Color::new(1.0, 1.0, 1.0, 1.0),
        });

        self.font.add(graphics::Text {
            content: self
                .update_durations
                .average()
                .subsec_micros()
                .to_string()
                + " us",
            position: graphics::Vector::new(100.0, 0.0),
            size: 20.0,
            color: graphics::Color::new(1.0, 1.0, 1.0, 1.0),
        });

        self.font.add(graphics::Text {
            content: String::from("Draw:"),
            position: graphics::Vector::new(2.0, 32.0),
            size: 20.0,
            color: graphics::Color::new(0.0, 0.0, 0.0, 1.0),
        });

        self.font.add(graphics::Text {
            content: String::from("Draw:"),
            position: graphics::Vector::new(0.0, 30.0),
            size: 20.0,
            color: graphics::Color::new(1.0, 1.0, 1.0, 1.0),
        });

        self.font.add(graphics::Text {
            content: self.draw_durations.average().subsec_micros().to_string()
                + " us",
            position: graphics::Vector::new(100.0, 30.0),
            size: 20.0,
            color: graphics::Color::new(1.0, 1.0, 1.0, 1.0),
        });

        self.font.draw(frame);

        Ok(())
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

// Null debug implementation
#[cfg(not(debug_assertions))]
pub struct Debug {}

#[cfg(not(debug_assertions))]
impl Debug {
    pub fn new() -> Self {
        Self {}
    }

    pub(super) fn loading_started(&mut self) {}

    pub(super) fn loading_finished(&mut self) {}
}
