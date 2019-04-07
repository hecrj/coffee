pub mod graphics;
pub mod input;

mod timer;

use graphics::window::{self, Window};
use timer::Timer;

pub trait Game {
    type View;
    type Input;

    const TICKS_PER_SECOND: u16;

    fn load(&self, gpu: &mut graphics::Gpu) -> (Self::View, Self::Input);

    fn update(
        &mut self,
        input: &mut Self::Input,
        view: &mut Self::View,
        window: &graphics::Window,
    );

    fn draw(
        &self,
        _input: &Self::Input,
        _view: &mut Self::View,
        _window: &mut graphics::Window,
    ) -> graphics::Result<()> {
        Ok(())
    }

    fn key_down_event(
        &self,
        _input: &mut Self::Input,
        _keycode: input::Keycode,
        _keymod: input::Mod,
        _repeat: bool,
    ) {
    }

    fn key_up_event(
        &self,
        _input: &mut Self::Input,
        _keycode: input::Keycode,
        _keymod: input::Mod,
    ) {
    }
}

pub fn run<G: Game>(
    game: &mut G,
    window_settings: window::Settings,
) -> graphics::Result<()> {
    // Set up window
    let mut event_loop = window::EventLoop::new();
    let window = &mut Window::new(window_settings, &event_loop);

    // Load game
    // (Loading progress support soon!)
    let (view, input) = &mut game.load(window.gpu());

    // Game loop
    let mut timer = Timer::new(G::TICKS_PER_SECOND);
    let mut alive = true;

    while alive {
        event_loop.poll(|event| match event {
            graphics::window::Event::CloseRequested => {
                alive = false;
            }
            graphics::window::Event::Resized(new_size) => {
                window.resize(new_size);
            }
        });

        timer.update();

        while timer.tick() {
            game.update(input, view, window);
        }

        game.draw(input, view, window)?;
    }

    Ok(())
}
