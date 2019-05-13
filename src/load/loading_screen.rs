//! Build consistent loading screens that run tasks.
//!
//! Any [`Task`] allows querying its total amount of work without running it. A
//! loading screen uses this feature to keep track of the overall progress
//! consistently.
//!
//! If you want to implement your own loading screen, check out the
//! [`LoadingScreen`] trait.
//!
//! If you just want a simple placeholder, you can try out the built-in
//! [`ProgressBar`] loading screen.
//!
//! [`Task`]: ../struct.Task.html
//! [`LoadingScreen`]: trait.LoadingScreen.html
//! [`ProgressBar`]: struct.ProgressBar.html
use super::{Progress, Task};
use crate::graphics;
use crate::Result;

/// A loading screen keeps track of the progress of a task and provides feedback
/// to the user.
///
/// # Usage
/// If you have a [`LoadingScreen`], you can use it in your [`Game::new`] method
/// easily. Let's say we want to use the [`ProgressBar`] loading screen in our
/// game:
///
/// ```
/// use coffee::{Game, Result};
/// use coffee::load::{Task, Join, LoadingScreen};
/// use coffee::load::loading_screen::ProgressBar;
/// use coffee::graphics::Window;
/// # use coffee::Timer;
/// # use coffee::graphics::Gpu;
/// #
/// # struct State;
/// # impl State {
/// # fn load() -> Task<State> { Task::new(|| State) }
/// # }
/// # struct View;
/// # impl View {
/// # fn load() -> Task<View> { Task::new(|| View) }
/// # }
/// # struct Input;
/// # impl Input {
/// # fn new() -> Input { Input }
/// # }
///
/// struct MyGame {
///     state: State,
///     // ...
/// }
///
/// impl Game for MyGame {
/// #   type View = View;
/// #   type Input = Input;
/// #
/// #   const TICKS_PER_SECOND: u16 = 60;
/// #
///     // ...
///
///     fn new(window: &mut Window) -> Result<(MyGame, View, Input)> {
///         let load =
///             (
///                 Task::stage("Loading state...", State::load()),
///                 Task::stage("Loading assets...", View::load()),
///             )
///                 .join();
///
///         // Create the loading screen and use `run`
///         let mut progress_bar = ProgressBar::new(window.gpu());
///         let (state, view) = progress_bar.run(load, window)?;
///
///         Ok((MyGame { state }, view, Input::new()))
///     }
///
///     // ...
///     # fn interact(&mut self, _input: &mut Self::Input,
///     #             _view: &mut Self::View, _gpu: &mut Gpu) {}
///     # fn update(&mut self, _view: &View, window: &Window) {}
///     # fn draw(&self, _view: &mut Self::View, _window: &mut Window,
///     #         _timer: &Timer) {}
/// }
/// ```
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
/// [`Game::new`]: ../../trait.Game.html#tymethod.new
/// [create an issue]: https://github.com/hecrj/coffee/issues
/// [open a pull request]: https://github.com/hecrj/coffee/pulls
pub trait LoadingScreen {
    fn new(gpu: &mut graphics::Gpu) -> Result<Self>
    where
        Self: Sized;

    /// React to task progress.
    ///
    /// You should provide feedback to the user here. You can draw on the given
    /// [`Window`], like in [`Game::draw`].
    ///
    /// [`Window`]: ../../graphics/struct.Window.html
    /// [`Game::draw`]: ../../trait.Game.html#tymethod.draw
    fn on_progress(
        &mut self,
        progress: &Progress,
        window: &mut graphics::Window,
    );

    /// Run the loading screen with a task and obtain its result.
    ///
    /// By default, it runs the task and refreshes the window when there is
    /// progress.
    fn run<T>(
        &mut self,
        task: Task<T>,
        window: &mut graphics::Window,
    ) -> Result<T> {
        task.run(window, |progress, window| {
            self.on_progress(progress, window);
            window.swap_buffers();
        })
    }
}

/// A simple loading screen showing a progress bar and the current stage.
///
/// ![The ProgressBar loading screen][progress_bar]
///
/// See [`LoadingScreen`] for a detailed example on how to use it.
///
/// [progress_bar]: https://github.com/hecrj/coffee/blob/e079e7205a53f92ac6614382b5cdd250fed64a98/images/loading_screen/progress_bar.png?raw=true
/// [`LoadingScreen`]: trait.LoadingScreen.html
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

    fn on_progress(
        &mut self,
        progress: &Progress,
        window: &mut graphics::Window,
    ) {
        let mut frame = window.frame();

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
                content: stage.clone(),
                position: graphics::Point::new(
                    50.0,
                    frame.height() / 2.0 - 80.0,
                ),
                size: 30.0,
                bounds: (frame.width(), frame.height()),
                color: graphics::Color::WHITE,
            });
        }

        self.font.add(graphics::Text {
            content: format!("{:.0}", progress.percentage()) + "%",
            position: graphics::Point::new(50.0, frame.height() / 2.0 + 50.0),
            size: 30.0,
            bounds: (frame.width(), frame.height()),
            color: graphics::Color::WHITE,
        });

        self.font.draw(&mut frame.as_target());
    }
}
