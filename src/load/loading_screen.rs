//! Build consistent loading screens that run tasks.
//!
//! Any [`Task`] allows querying its total amount of work without running it. A
//! loading screen uses this feature to keep track of the overall progress
//! consistently.
//!
//! If you want to implement your own loading screen, check out the
//! [`LoadingScreen`] trait.
//!
//! If you want a simple placeholder, you can try out the built-in
//! [`ProgressBar`] loading screen.
//!
//! [`Task`]: ../struct.Task.html
//! [`LoadingScreen`]: trait.LoadingScreen.html
//! [`ProgressBar`]: struct.ProgressBar.html
mod progress_bar;

pub use progress_bar::ProgressBar;

use crate::graphics;
use crate::load::{Progress, Task};
use crate::Result;

/// A loading screen keeps track of the progress of a task and provides feedback
/// to the user.
///
/// # Usage
/// If you have a [`LoadingScreen`], set it as your [`Game::LoadingScreen`]
/// associated type. Coffee will automatically use it when your game starts!
///
/// # Future plans
/// As of now, Coffee only ships with the [`ProgressBar`] loading screen. In the
/// near future, the plan is to add more interesting (and configurable!) loading
/// screens. If you make a cool loading screen or have an interesting idea and
/// you would like to share it, feel free to [create an issue] or
/// [open a pull request]!
///
/// [`Task`]: ../struct.Task.html
/// [`LoadingScreen`]: trait.LoadingScreen.html
/// [`ProgressBar`]: struct.ProgressBar.html
/// [`Game::LoadingScreen`]: ../../trait.Game.html#associatedtype.LoadingScreen
/// [create an issue]: https://github.com/hecrj/coffee/issues
/// [open a pull request]: https://github.com/hecrj/coffee/pulls
pub trait LoadingScreen {
    /// Creates the [`LoadingScreen`].
    ///
    /// You can use the provided [`Gpu`] to load the assets necessary to show
    /// the loading screen.
    ///
    /// [`LoadingScreen`]: trait.LoadingScreen.html
    fn new(gpu: &mut graphics::Gpu) -> Result<Self>
    where
        Self: Sized;

    /// Draws the [`LoadingScreen`] with the given [`Progress`].
    ///
    /// You should provide feedback to the user here. You can draw on the given
    /// [`Frame`], like in [`Game::draw`].
    ///
    /// [`LoadingScreen`]: trait.LoadingScreen.html
    /// [`Progress`]: ../struct.Progress.html
    /// [`Frame`]: ../../graphics/struct.Frame.html
    /// [`Game::draw`]: ../../trait.Game.html#tymethod.draw
    fn draw(&mut self, progress: &Progress, frame: &mut graphics::Frame<'_>);

    /// Runs the [`LoadingScreen`] with a task and obtain its result.
    ///
    /// By default, it runs the task and refreshes the window when there is
    /// progress.
    ///
    /// [`LoadingScreen`]: trait.LoadingScreen.html
    fn run<T>(
        &mut self,
        task: Task<T>,
        window: &mut graphics::Window,
    ) -> Result<T> {
        task.run_with_window(window, |progress, window| {
            self.draw(progress, &mut window.frame());
            window.swap_buffers();
        })
    }
}

impl LoadingScreen for () {
    fn new(_gpu: &mut graphics::Gpu) -> Result<Self> {
        Ok(())
    }

    fn draw(&mut self, _progress: &Progress, _frame: &mut graphics::Frame<'_>) {
    }
}
