//! `coffee` is an opinionated 2D game engine focused on simplicity,
//! explicitness, and safety.
mod debug;
mod timer;

pub mod graphics;
pub mod input;
pub mod loader;
pub mod loading_screen;

pub use debug::Debug;
pub use loader::Loader;
pub use loading_screen::LoadingScreen;

use graphics::window::{self, Window};
use timer::Timer;

/// A Coffee game has a bunch of related types, constants, and methods.
///
/// Implementors of this trait should hold the game state.
///
/// Coffee forces you to decouple your game state from your view and input
/// state. While this might seem limiting at first, it helps to keep mutability
/// at bay and forces you to think about the architecture of your game.
///
/// Ideally, your game state should be an opaque type with a meaningful API with
/// clear boundaries. External code (like draw code or input code) can rely on
/// this API to do its job.
///
/// This is the main reason why Coffee is _opinionated_.
pub trait Game {
    /// The view data of your game.
    ///
    /// This type should hold all the assets and state necessary to render your
    /// game and UI.
    type View;

    /// The input data of your game.
    ///
    /// You can start by simply using a `HashSet` here to see which keys are
    /// pressed at any given time. Then, you can iterate when needed (e.g. if
    /// you need to know which keys were _just_ released, etc.).
    type Input;

    /// Defines how many times the `update` function should be called per
    /// second.
    ///
    /// A common value is `60`.
    const TICKS_PER_SECOND: u16;

    /// Defines the key that will be used to toggle the debug view. Set it to
    /// `None` if you want to disable it.
    ///
    /// By default, it is set to `F12`.
    const DEBUG_KEY: Option<input::KeyCode> = Some(input::KeyCode::F12);

    /// Create your game here.
    ///
    /// You need to return your initial game state, view state, and input state.
    ///
    /// It is recommended to load your game assets right here. You can use
    /// the `Loader` abstraction to declaratively describe how to load your
    /// assets and easily get a _consistent_ `LoadingScreen` for free!
    fn new(window: &mut graphics::Window) -> (Self, Self::View, Self::Input)
    where
        Self: Sized;

    /// Consume user input to control your view state here.
    ///
    /// Unlike `update`, this function is guaranteed to be called once per
    /// frame, after event processing. This allows you to keep your UI updated
    /// every frame in order to offer a smooth user experience independently of
    /// the `TICKS_PER_SECOND` setting.
    ///
    /// You can access the GPU in case you need to prepare some new assets
    /// before rendering.
    fn control(
        &self,
        input: &mut Self::Input,
        view: &mut Self::View,
        gpu: &mut graphics::Gpu,
    );

    /// Consume input and update your game state here.
    ///
    /// You should probably translate input into game actions based on the state
    /// of your UI! For instance, if a text box is currently focused, you
    /// probably do not want to perform player movement actions when keys are
    /// pressed.
    ///
    /// The `TICKS_PER_SECOND` constant defines how many times this function
    /// will be called per second. This function may be called multiple times
    /// per frame if it is necessary.
    ///
    /// Notice that you are also allowed to access window information. This can
    /// be useful if your game state needs to know how much of the world is
    /// visible.
    fn update(
        &mut self,
        input: &mut Self::Input,
        view: &Self::View,
        window: &graphics::Window,
    );

    /// Here is where you draw your game!
    ///
    /// You probably want to get a `window.frame()` and clear it first:
    ///
    /// ```
    /// use coffee::graphics::{Result, Window, Color};
    /// # struct Game { state: () }
    /// # struct View;
    ///
    /// # impl Game {
    /// fn draw(&self, view: &mut View, window: &mut Window, _was_updated: bool) -> Result<()> {
    ///     let frame = &mut window.frame();
    ///
    ///     frame.clear(Color::BLACK);
    ///     // ...
    ///
    ///     Ok(())
    /// }
    /// # }
    /// ```
    ///
    /// `was_updated` tells you whether the `update` function was called during
    /// this frame. This allows you to perform optimizations.
    ///
    /// This function will be called once per frame, after `update`.
    fn draw(
        &self,
        view: &mut Self::View,
        window: &mut graphics::Window,
        was_updated: bool,
    ) -> graphics::Result<()>;

    /// Implement this function to display debug information.
    ///
    /// It is called after `draw` once per frame when debug has been toggled
    /// using the `DEBUG_KEY`. Anything you draw here will be on top. Debug code
    /// is only called when compiling with `debug_assertions` enabled.
    ///
    /// By default, it just calls `Debug::draw` which displays a brief summary
    /// about game performance in the top left corner.
    fn debug(
        &self,
        _input: &Self::Input,
        _view: &Self::View,
        window: &mut graphics::Window,
        debug: &mut Debug,
    ) -> graphics::Result<()> {
        debug.draw(&mut window.frame())
    }

    /// Process an input event and apply it to your `Input` type.
    ///
    /// This function may be called multiple times during event processing,
    /// right before `play`.
    ///
    /// It does nothing by default.
    fn on_input(&self, _input: &mut Self::Input, _event: input::Event) {}
}

pub fn run<G: Game>(window_settings: window::Settings) -> graphics::Result<()> {
    // Set up window
    let mut event_loop = window::EventLoop::new();
    let window = &mut Window::new(window_settings, &event_loop);

    // Debug
    let mut debug = Debug::new(window.gpu(), G::TICKS_PER_SECOND);

    // Load game
    // (Loading progress support soon!)
    debug.loading_started();
    let (game, view, input) = &mut G::new(window);
    debug.loading_finished();

    // Game loop
    let mut timer = Timer::new(G::TICKS_PER_SECOND);
    let mut alive = true;
    let mut was_updated = true;

    while alive {
        debug.frame_started();

        debug.event_loop_started();
        event_loop.poll(|event| match event {
            window::Event::Input(input_event) => {
                game.on_input(input, input_event);

                if cfg!(debug_assertions) {
                    match input_event {
                        input::Event::KeyboardInput {
                            state: input::KeyState::Released,
                            key_code,
                        } if Some(key_code) == G::DEBUG_KEY => {
                            debug.toggle();
                        }
                        _ => {}
                    }
                }
            }
            window::Event::CloseRequested => {
                alive = false;
            }
            window::Event::Resized(new_size) => {
                window.resize(new_size);
            }
        });
        game.control(input, view, window.gpu());
        debug.event_loop_finished();

        // Update loop
        timer.update();

        while timer.tick() {
            debug.update_started();
            game.update(input, view, window);
            debug.update_finished();

            was_updated = true;
        }

        debug.draw_started();
        game.draw(view, window, was_updated)?;
        debug.draw_finished();

        game.debug(input, view, window, &mut debug).unwrap();
        window.swap_buffers();

        debug.frame_finished();

        was_updated = false;
    }

    Ok(())
}
