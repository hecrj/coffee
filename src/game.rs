use crate::graphics::window;
use crate::graphics::{Frame, Window, WindowSettings};
use crate::input;
use crate::load::{Join, LoadingScreen, Task};
use crate::{Debug, Input, Result, State, Timer};

/// The entrypoint of the engine. It attaches user interaction and graphics
/// to your game [`State`].
///
/// Implementors of this trait should hold the game assets and view data
/// necessary for drawing.
///
/// Coffee forces you to decouple your game state from your view and input
/// state. While this might seem limiting at first, it helps you to keep
/// mutability at bay and forces you to think about the architecture of your
/// game.
///
/// [`State`]: trait.State.html
pub trait Game {
    /// The state of your game, holding all the game data.
    ///
    /// It needs to implement the [`State`] trait.
    ///
    /// Ideally, your game state should be an opaque type with a meaningful API
    /// with clear boundaries. External code (like draw code or input code) should
    /// rely on this API to do its job.
    ///
    /// If your game has no state, use `()`.
    ///
    /// [`State`]: trait.State.html
    type State: State;

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

    /// Defines the key that will be used to toggle the [`debug`] view. Set it to
    /// `None` if you want to disable it.
    ///
    /// By default, it is set to `F12`.
    ///
    /// [`debug`]: #method.debug
    const DEBUG_KEY: Option<input::KeyCode> = Some(input::KeyCode::F12);

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
    /// [`update`]: #tymethod.update
    fn draw(&mut self, state: &Self::State, frame: &mut Frame, timer: &Timer);

    /// Handles a close request from the operating system to the game window.
    ///
    /// This function should return true to allow the game loop to end,
    /// otherwise false.
    ///
    /// By default, it does nothing and returns true.
    fn on_close_request(&mut self) -> bool {
        true
    }

    /// Consumes [`Input`] to let users interact with the [`Game`].
    ///
    /// Right before a [`State::update`], input events will be processed and this
    /// function will be called. This reduces latency when multiple updates need
    /// to happen during a single frame.
    ///
    /// If no [`State::update`] is needed during a frame, it will still be called once,
    /// right after processing input events and before drawing. This allows you
    /// to keep your view updated every frame in order to offer a smooth user
    /// experience independently of the [`State::TICKS_PER_SECOND`] setting.
    ///
    /// You can access the [`Window`]. For instance, you may want to toggle
    /// fullscreen mode based on some input, or maybe access the [`Gpu`]
    /// to prepare some assets before rendering.
    ///
    /// By default, it does nothing.
    ///
    /// [`Input`]: #associatedtype.Input
    /// [`State::update`]: trait.State.html#tymethod.update
    /// [`State::TICKS_PER_SECOND`]: trait.State.html#associatedconstant.TICKS_PER_SECOND
    /// [`Window`]: graphics/struct.Window.html
    /// [`Gpu`]: graphics/struct.Gpu.html
    fn interact(
        &mut self,
        _input: &mut Self::Input,
        _state: &mut Self::State,
        _window: &mut Window,
    ) {
    }

    /// Displays debug information.
    ///
    /// This method is called after `draw` once per frame when debug has been
    /// toggled using the [`DEBUG_KEY`]. Anything you draw here will be on top.
    ///
    /// Debug code is only called when compiling with `debug_assertions` _or_
    /// the `debug` feature enabled.
    ///
    /// By default, it shows [`Debug`], which displays a brief summary about
    /// game performance in the top left corner.
    ///
    /// [`DEBUG_KEY`]: #associatedconstant.DEBUG_KEY
    /// [`Debug`]: struct.Debug.html
    fn debug(
        &self,
        _input: &Self::Input,
        _state: &Self::State,
        window: &mut Window,
        debug: &mut Debug,
    ) {
        debug.draw(&mut window.frame())
    }

    /// Runs the [`Game`] with the given [`WindowSettings`].
    ///
    /// [`Game`]: trait.Game.html
    /// [`WindowSettings`]: graphics/struct.WindowSettings.html
    fn run(window_settings: WindowSettings) -> Result<()>
    where
        Self: Sized + 'static,
    {
        // Set up window
        let mut event_loop = window::EventLoop::new();
        let window = &mut Window::new(window_settings, &event_loop)?;
        let mut debug = Debug::new(window.gpu(), Self::State::TICKS_PER_SECOND);

        // Load game
        debug.loading_started();
        let mut loading_screen = Self::LoadingScreen::new(window.gpu())?;
        let load = (Self::load(window), Self::State::load(window)).join();
        let (game, state) = &mut loading_screen.run(load, window)?;
        let input = &mut Self::Input::new();
        debug.loading_finished();

        // Game loop
        let mut timer = Timer::new(Self::State::TICKS_PER_SECOND);
        let mut alive = true;

        while alive {
            debug.frame_started();
            timer.update();

            while timer.tick() {
                interact(
                    game,
                    input,
                    state,
                    &mut debug,
                    window,
                    &mut event_loop,
                    &mut alive,
                );

                debug.update_started();
                state.update();
                debug.update_finished();
            }

            if !timer.has_ticked() {
                interact(
                    game,
                    input,
                    state,
                    &mut debug,
                    window,
                    &mut event_loop,
                    &mut alive,
                );
            }

            debug.draw_started();
            game.draw(state, &mut window.frame(), &timer);
            debug.draw_finished();

            if debug.is_enabled() {
                debug.debug_started();
                game.debug(input, state, window, &mut debug);
                debug.debug_finished();
            }

            window.swap_buffers();
            debug.frame_finished();
        }

        Ok(())
    }
}

fn interact<G: Game>(
    game: &mut G,
    input: &mut G::Input,
    state: &mut G::State,
    debug: &mut Debug,
    window: &mut Window,
    event_loop: &mut window::EventLoop,
    alive: &mut bool,
) {
    debug.interact_started();

    event_loop
        .poll(|event| process_event(game, input, debug, window, alive, event));

    game.interact(input, state, window);
    input.clear();

    debug.interact_finished();
}

pub(crate) fn process_event<G: Game, I: Input>(
    game: &mut G,
    input: &mut I,
    debug: &mut Debug,
    window: &mut Window,
    alive: &mut bool,
    event: window::Event,
) {
    match event {
        window::Event::Input(input_event) => {
            input.update(input_event);

            #[cfg(any(debug_assertions, feature = "debug"))]
            match input_event {
                input::Event::KeyboardInput {
                    state: input::ButtonState::Released,
                    key_code,
                } if Some(key_code) == G::DEBUG_KEY => {
                    debug.toggle();
                }
                _ => {}
            }
        }
        window::Event::CursorMoved(logical_position) => {
            let position = logical_position.to_physical(window.dpi());
            let event = input::Event::CursorMoved {
                x: position.x as f32,
                y: position.y as f32,
            };

            input.update(event);
        }
        window::Event::Moved(logical_position) => {
            let position = logical_position.to_physical(window.dpi());

            input.update(input::Event::WindowMoved {
                x: position.x as f32,
                y: position.y as f32,
            })
        }
        window::Event::CloseRequested => {
            if game.on_close_request() {
                *alive = false;
            }
        }
        window::Event::Resized(new_size) => {
            window.resize(new_size);
        }
    };
}
