pub mod debug;
pub mod graphics;
pub mod input;

mod timer;

pub use debug::Debug;
use graphics::window::{self, Window};
use timer::Timer;

pub trait Game {
    type View;
    type Input;

    const TICKS_PER_SECOND: u16;
    const DEBUG_KEY: Option<input::KeyCode> = Some(input::KeyCode::F12);

    fn load(&self, gpu: &mut graphics::Gpu) -> (Self::View, Self::Input);

    fn update(
        &mut self,
        input: &mut Self::Input,
        view: &mut Self::View,
        window: &graphics::Window,
    );

    fn draw(
        &self,
        _view: &mut Self::View,
        _window: &mut graphics::Window,
    ) -> graphics::Result<()> {
        Ok(())
    }

    fn debug(
        &self,
        _input: &Self::Input,
        _view: &Self::View,
        window: &mut graphics::Window,
        debug: &mut Debug,
    ) -> graphics::Result<()> {
        debug.draw(&mut window.frame())
    }

    fn on_input(&self, _input: &mut Self::Input, _event: input::Event) {}
}

pub fn run<G: Game>(
    game: &mut G,
    window_settings: window::Settings,
) -> graphics::Result<()> {
    // Set up window
    let mut event_loop = window::EventLoop::new();
    let window = &mut Window::new(window_settings, &event_loop);

    // Debug
    let mut debug = Debug::new(window.gpu(), G::TICKS_PER_SECOND);

    // Load game
    // (Loading progress support soon!)
    debug.loading_started();
    let (view, input) = &mut game.load(window.gpu());
    debug.loading_finished();

    // Game loop
    let mut timer = Timer::new(G::TICKS_PER_SECOND);
    let mut alive = true;

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
        debug.event_loop_finished();

        timer.update();

        while timer.tick() {
            debug.update_started();
            game.update(input, view, window);
            debug.update_finished();
        }

        debug.draw_started();
        game.draw(view, window)?;
        debug.draw_finished();

        game.debug(input, view, window, &mut debug).unwrap();
        window.swap_buffers();

        debug.frame_finished();
    }

    Ok(())
}
