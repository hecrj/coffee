use super::{LoadingScreen, Progress};
use crate::graphics;
use crate::Result;

/// A simple loading screen showing a progress bar and the current stage.
///
/// ![The ProgressBar loading screen][progress_bar]
///
/// # Usage
/// Set [`ProgressBar`] as your [`Game::LoadingScreen`] associated type.
///
/// [progress_bar]: https://github.com/hecrj/coffee/blob/e079e7205a53f92ac6614382b5cdd250fed64a98/images/loading_screen/progress_bar.png?raw=true
/// [`LoadingScreen`]: trait.LoadingScreen.html
/// [`ProgressBar`]: struct.ProgressBar.html
/// [`Game::LoadingScreen`]: ../../trait.Game.html#associatedtype.LoadingScreen
#[allow(missing_debug_implementations)]
pub struct ProgressBar {
    font: graphics::Font,
    pencil: graphics::Image,
}

impl LoadingScreen for ProgressBar {
    /// Create the loading screen.
    fn new(gpu: &mut graphics::Gpu) -> Result<Self> {
        Ok(Self {
            font: graphics::Font::from_bytes(gpu, graphics::Font::DEFAULT)?,
            pencil: graphics::Image::from_colors(
                gpu,
                &[graphics::Color::WHITE],
            )?,
        })
    }

    fn draw(&mut self, progress: &Progress, frame: &mut graphics::Frame<'_>) {
        frame.clear(graphics::Color::BLACK);

        self.pencil.draw(
            graphics::Quad {
                position: graphics::Point::new(
                    50.0,
                    frame.height() / 2.0 - 25.0,
                ),
                size: (
                    (frame.width() - 100.0) * (progress.percentage() / 100.0),
                    50.0,
                ),
                ..Default::default()
            },
            &mut frame.as_target(),
        );

        if let Some(stage) = progress.stage() {
            self.font.add(graphics::Text {
                content: stage,
                position: graphics::Point::new(
                    50.0,
                    frame.height() / 2.0 - 80.0,
                ),
                size: 30.0,
                color: graphics::Color::WHITE,
                ..graphics::Text::default()
            });
        }

        self.font.add(graphics::Text {
            content: &(format!("{:.0}", progress.percentage()) + "%"),
            position: graphics::Point::new(50.0, frame.height() / 2.0 + 50.0),
            size: 30.0,
            color: graphics::Color::WHITE,
            ..graphics::Text::default()
        });

        self.font.draw(&mut frame.as_target());
    }
}
