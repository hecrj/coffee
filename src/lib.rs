pub mod graphics;
pub mod input;

mod timer;

use timer::Timer;

pub trait Game {
    type View;
    type Input;

    const TICKS_PER_SECOND: u16;

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
    input: &mut G::Input,
    view: &mut G::View,
    window: &mut graphics::Window,
) -> graphics::Result<()> {
    let mut timer = Timer::new(G::TICKS_PER_SECOND);
    let mut alive = true;

    while alive {
        window.poll_events(|event| match event {
            graphics::window::Event::CloseRequested => {
                alive = false;
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
