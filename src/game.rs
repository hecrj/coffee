mod r#loop;

pub(crate) use r#loop::Loop;

use crate::graphics::{CursorIcon, Frame, Window, WindowSettings};
use crate::input::{keyboard, Input};
use crate::load::{LoadingScreen, Task};
use crate::{Debug, Result, Timer};

/// The entrypoint of the engine. It describes your game logic.
///
/// Implementors of this trait should hold the game state and any assets
/// necessary for drawing.
///
/// Coffee forces you to decouple your game state from your input state. While
/// this might seem limiting at first, it helps you to keep mutability at bay
/// and forces you to think about the architecture of your game.
pub trait Game {
    /// The input data of your game.
    ///
    /// The built-in [`KeyboardAndMouse`] type can be a good starting point. It
    /// allows you to query the state of the keyboard and the mouse.
    ///
    /// You can also build your custom input type using the [`Input`] trait.
    ///
    /// If your game does not deal with user input, use `()`.
    ///
    /// [`KeyboardAndMouse`]: input/struct.KeyboardAndMouse.html
    /// [`Input`]: input/trait.Input.html
    type Input: Input;

    /// The loading screen that will be used when your game starts.
    ///
    /// The built-in [`ProgressBar`] loading screen is a good choice to get
    /// started. It shows a simple progress bar.
    ///
    /// You can also build your own loading screen type using the
    /// [`LoadingScreen`] trait.
    ///
    /// If you do not want your game to have a loading screen, use `()`.
    ///
    /// [`ProgressBar`]: load/loading_screen/struct.ProgressBar.html
    /// [`LoadingScreen`]: load/loading_screen/trait.LoadingScreen.html
    type LoadingScreen: LoadingScreen;

    /// Defines how many times the [`update`] function should be called per
    /// second.
    ///
    /// By default, it is set to `60`.
    ///
    /// [`update`]: #method.update
    const TICKS_PER_SECOND: u16 = 60;

    /// Defines the key that will be used to toggle the [`debug`] view. Set it to
    /// `None` if you want to disable it.
    ///
    /// By default, it is set to `F12`.
    ///
    /// [`debug`]: #method.debug
    const DEBUG_KEY: Option<keyboard::KeyCode> = Some(keyboard::KeyCode::F12);

    /// Loads the [`Game`].
    ///
    /// Use the [`load`] module to load your assets here.
    ///
    /// [`Game`]: trait.Game.html
    /// [`load`]: load/index.html
    fn load(window: &Window) -> Task<Self>
    where
        Self: Sized;

    /// Draws the [`Game`].
    ///
    /// Check out the [`graphics`] module to learn more about rendering in
    /// Coffee.
    ///
    /// This function will be called once per frame.
    ///
    /// [`Game`]: trait.Game.html
    /// [`graphics`]: graphics/index.html
    /// [`update`]: #method.update
    fn draw(&mut self, frame: &mut Frame<'_>, timer: &Timer);

    /// Consumes [`Input`] to let users interact with the [`Game`].
    ///
    /// Right before an [`update`], input events will be processed and this
    /// function will be called. This reduces latency when multiple updates need
    /// to happen during a single frame.
    ///
    /// If no [`update`] is needed during a frame, it will still be called once,
    /// right after processing input events and before drawing. This allows you
    /// to keep your view updated every frame in order to offer a smooth user
    /// experience independently of the [`TICKS_PER_SECOND`] setting.
    ///
    /// You can access the [`Window`]. For instance, you may want to toggle
    /// fullscreen mode based on some input, or maybe access the [`Gpu`]
    /// to prepare some assets before rendering.
    ///
    /// By default, it does nothing.
    ///
    /// [`Input`]: #associatedtype.Input
    /// [`Game`]: trait.Game.html
    /// [`update`]: #method.update
    /// [`TICKS_PER_SECOND`]: #associatedconstant.TICKS_PER_SECOND
    /// [`Window`]: graphics/struct.Window.html
    /// [`Gpu`]: graphics/struct.Gpu.html
    fn interact(&mut self, _input: &mut Self::Input, _window: &mut Window) {}

    /// Updates the [`Game`].
    ///
    /// All your game logic should live here.
    ///
    /// The [`TICKS_PER_SECOND`] constant defines how many times this function
    /// will be called per second. This function may be called multiple times
    /// per frame if it is necessary.
    ///
    /// Notice that you are also allowed to access [`Window`] data. This can be
    /// useful if your [`Game`] needs to know how much of the world is visible.
    ///
    /// By default, it does nothing.
    ///
    /// [`Game`]: trait.Game.html
    /// [`TICKS_PER_SECOND`]: #associatedconstant.TICKS_PER_SECOND
    /// [`Window`]: graphics/struct.Window.html
    fn update(&mut self, _window: &Window) {}

    /// Defines the cursor icon of the window.
    ///
    /// By default, it returns platform-dependent default cursor.
    fn cursor_icon(&self) -> CursorIcon {
        CursorIcon::Default
    }

    /// Displays debug information.
    ///
    /// This method is called after [`draw`] once per frame when debug has been
    /// toggled using the [`DEBUG_KEY`]. Anything you draw here will be on top.
    ///
    /// Debug code is only called when compiling with `debug_assertions` _or_
    /// the `debug` feature enabled.
    ///
    /// By default, it shows [`Debug`], which displays a brief summary about
    /// game performance in the top left corner.
    ///
    /// [`draw`]: #tymethod.draw
    /// [`DEBUG_KEY`]: #associatedconstant.DEBUG_KEY
    /// [`Debug`]: struct.Debug.html
    fn debug(
        &self,
        _input: &Self::Input,
        frame: &mut Frame<'_>,
        debug: &mut Debug,
    ) {
        debug.draw(frame);
    }

    /// Handles a close request from the operating system to the game window.
    ///
    /// This function should return true to allow the game loop to end,
    /// otherwise false.
    ///
    /// By default, it does nothing and returns true.
    fn on_close_request(&mut self) -> bool {
        true
    }

    /// Returns whether the game is finished or not.
    ///
    /// If this function returns true, the game will be closed gracefully.
    ///
    /// By default, it always returns false.
    fn is_finished(&self) -> bool {
        false
    }

    /// Returns whether the screen should be drawn.
    ///
    /// Use this to limit the amount of drawn frames per second (FPS).
    ///
    /// By default, it always returns true.
    fn should_draw(&self) -> bool {
        true
    }

    /// Runs the [`Game`] with the given [`WindowSettings`].
    ///
    /// You probably want to call this in your `main` function to run your game!
    ///
    /// [`Game`]: trait.Game.html
    /// [`WindowSettings`]: graphics/struct.WindowSettings.html
    fn run(window_settings: WindowSettings) -> Result<()>
    where
        Self: 'static + Sized,
    {
        <r#loop::Default as Loop<Self>>::run(window_settings)
    }
}
