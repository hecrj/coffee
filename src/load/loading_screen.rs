use super::{Progress, Task};
use crate::graphics;

pub trait LoadingScreen {
    fn on_progress(
        &mut self,
        progress: &Progress,
        window: &mut graphics::Window,
    ) -> graphics::Result<()>;

    fn run<T>(&mut self, task: Task<T>, window: &mut graphics::Window) -> T {
        task.run(window, |progress, window| {
            self.on_progress(progress, window).unwrap();
            window.swap_buffers();
        })
    }
}

pub struct ProgressBar {
    font: graphics::Font,
    pencil: graphics::Image,
}

impl ProgressBar {
    pub fn new(gpu: &mut graphics::Gpu) -> Self {
        Self {
            font: graphics::Font::from_bytes(
                gpu,
                include_bytes!("../debug/font/Inconsolata-Regular.ttf"),
            ),
            pencil: graphics::Image::from_colors(
                gpu,
                &[graphics::Color::WHITE],
            )
            .unwrap(),
        }
    }
}

impl LoadingScreen for ProgressBar {
    fn on_progress(
        &mut self,
        progress: &Progress,
        window: &mut graphics::Window,
    ) -> graphics::Result<()> {
        let mut frame = window.frame();

        frame.clear(graphics::Color::BLACK);

        self.pencil.draw(
            graphics::DrawParameters {
                position: graphics::Point::new(
                    50.0,
                    frame.height() / 2.0 - 25.0,
                ),
                scale: graphics::Vector::new(
                    (frame.width() - 100.0) * (progress.percentage() / 100.0),
                    50.0,
                ),
                ..Default::default()
            },
            &mut frame.as_target(),
        );

        if let Some(stage) = progress.stage() {
            self.font.add(graphics::Text {
                content: stage.clone(),
                position: graphics::Vector::new(
                    50.0,
                    frame.height() / 2.0 - 80.0,
                ),
                size: 30.0,
                bounds: (frame.width(), frame.height()),
                color: graphics::Color::WHITE,
            });
        }

        self.font.add(graphics::Text {
            content: format!("{:.2}", progress.percentage()) + "%",
            position: graphics::Vector::new(50.0, frame.height() / 2.0 + 50.0),
            size: 30.0,
            bounds: (frame.width(), frame.height()),
            color: graphics::Color::WHITE,
        });

        self.font.draw(&mut frame);

        Ok(())
    }
}
