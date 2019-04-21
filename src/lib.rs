//! `coffee` is an opinionated 2D game engine focused on simplicity,
//! explicitness, and safety.
mod debug;
mod timer;

pub mod graphics;
pub mod input;
pub mod load;

pub use debug::Debug;
pub use timer::Timer;

use graphics::window::{self, Window};

/// The entrypoint of the engine. It describes your game logic.
///
/// Implementors of this trait should hold the game state.
///
/// Coffee forces you to decouple your game state from your view and input
/// state. While this might seem limiting at first, it helps you to keep
/// mutability at bay and forces you to think about the architecture of your
/// game.
///
/// Ideally, your game state should be an opaque type with a meaningful API with
/// clear boundaries. External code (like draw code or input code) should rely
/// on this API to do its job.
pub trait Game {
    /// The view data of your game.
    ///
    /// This type should hold all the assets and state necessary to render your
    /// game and UI.
    type View;

    /// The input data of your game.
    ///
    /// For instance, you could start by simply using a `HashSet` here to track
    /// which keys are pressed at any given time.
    type Input;

    /// Defines how many times the [`update`] function should be called per
    /// second.
    ///
    /// A common value is `60`.
    ///
    /// [`update`]: #tymethod.update
    const TICKS_PER_SECOND: u16;

    /// Defines the key that will be used to toggle the [`debug`] view. Set it to
    /// `None` if you want to disable it.
    ///
    /// By default, it is set to `F12`.
    ///
    /// [`debug`]: #method.debug
    const DEBUG_KEY: Option<input::KeyCode> = Some(input::KeyCode::F12);

    /// Create your game here.
    ///
    /// You need to return your initial game state, view state, and input state.
    ///
    /// It is recommended to load your game assets right here. You can use
    /// the [`load`] module to declaratively describe how to load your
    /// assets and easily get a _consistent_ loading screen for free!
    ///
    /// [`load`]: load/index.html
    fn new(window: &mut graphics::Window) -> (Self, Self::View, Self::Input)
    where
        Self: Sized;

    /// Consume your [`Input`] to let users interact with your game here.
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
    /// You can access the GPU if, as a consequence of the interaction, you need
    /// to prepare some assets before rendering.
    ///
    /// [`Input`]: #associatedtype.Input
    /// [`update`]: #tymethod.update
    /// [`TICKS_PER_SECOND`]: #associatedconstant.TICKS_PER_SECOND
    fn interact(
        &mut self,
        input: &mut Self::Input,
        view: &mut Self::View,
        gpu: &mut graphics::Gpu,
    );

    /// Update your game state here.
    ///
    /// The [`TICKS_PER_SECOND`] constant defines how many times this function
    /// will be called per second. This function may be called multiple times
    /// per frame if it is necessary.
    ///
    /// Notice that you are also allowed to access view and window data. This
    /// can be useful if your game state needs to know how much of the world is
    /// visible.
    ///
    /// [`TICKS_PER_SECOND`]: #associatedconstant.TICKS_PER_SECOND
    /// [`View`]: #associatedtype.View
    fn update(&mut self, view: &Self::View, window: &Window);

    /// Here is where you draw your game!
    ///
    /// Check out the [`graphics`] module to learn more about rendering in
    /// Coffee.
    ///
    /// This function will be called once per frame.
    ///
    /// [`graphics`]: graphics/index.html
    /// [`update`]: #tymethod.update
    fn draw(
        &self,
        view: &mut Self::View,
        window: &mut graphics::Window,
        timer: &Timer,
    ) -> graphics::Result<()>;

    /// Process an input event and keep track of it in your [`Input`] type.
    ///
    /// This function may be called multiple times during event processing,
    /// before [`interact`].
    ///
    /// By default, it does nothing.
    ///
    /// [`Input`]: #associatedtype.Input
    /// [`interact`]: #tymethod.interact
    fn on_input(&self, _input: &mut Self::Input, _event: input::Event) {}

    /// Implement this function to display debug information.
    ///
    /// It is called after `draw` once per frame when debug has been toggled
    /// using the [`DEBUG_KEY`]. Anything you draw here will be on top. Debug
    /// code is only called when compiling with `debug_assertions` enabled.
    ///
    /// By default, it shows [`Debug`], which displays a brief summary about
    /// game performance in the top left corner.
    ///
    /// [`DEBUG_KEY`]: #associatedconstant.DEBUG_KEY
    /// [`Debug`]: struct.Debug.html
    fn debug(
        &self,
        _input: &Self::Input,
        _view: &Self::View,
        window: &mut graphics::Window,
        debug: &mut Debug,
    ) -> graphics::Result<()> {
        debug.draw(&mut window.frame())
    }
}

pub fn run<G: Game>(
    window_settings: graphics::WindowSettings,
) -> graphics::Result<()> {
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

    fn process_events<G: Game>(
        game: &mut G,
        input: &mut G::Input,
        view: &mut G::View,
        debug: &mut Debug,
        window: &mut Window,
        event_loop: &mut window::EventLoop,
        alive: &mut bool,
    ) {
        debug.event_loop_started();
        event_loop.poll(|event| match event {
            window::Event::Input(input_event) => {
                game.on_input(input, input_event);

                if cfg!(any(debug_assertions, feature = "debug")) {
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
            window::Event::CursorMoved(logical_position) => {
                let position = logical_position.to_physical(window.dpi());

                game.on_input(
                    input,
                    input::Event::CursorMoved {
                        x: position.x as f32,
                        y: position.y as f32,
                    },
                )
            }
            window::Event::CloseRequested => {
                *alive = false;
            }
            window::Event::Resized(new_size) => {
                window.resize(new_size);
            }
        });
        game.interact(input, view, window.gpu());
        debug.event_loop_finished();
    }

    while alive {
        debug.frame_started();

        // Update loop
        timer.update();

        while timer.tick() {
            process_events(
                game,
                input,
                view,
                &mut debug,
                window,
                &mut event_loop,
                &mut alive,
            );

            debug.update_started();
            game.update(view, window);
            debug.update_finished();

            was_updated = true;
        }

        if !was_updated {
            process_events(
                game,
                input,
                view,
                &mut debug,
                window,
                &mut event_loop,
                &mut alive,
            );
        }

        debug.draw_started();
        game.draw(view, window, &timer)?;
        debug.draw_finished();

        game.debug(input, view, window, &mut debug).unwrap();
        window.swap_buffers();

        debug.frame_finished();

        was_updated = false;
    }

    Ok(())
}
