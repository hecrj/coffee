pub mod graphics;
pub mod input;

pub trait Game {
    type View;
    type Input;

    fn update(&mut self, view: &mut Self::View, window: &graphics::Window);

    fn draw(
        &self,
        _view: &mut Self::View,
        _window: &mut graphics::Window,
        _was_updated: bool,
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

pub trait Renderer {}

pub fn run<G: Game>(
    game: &mut G,
    view: &mut G::View,
    window: &mut graphics::Window,
    ticks_per_second: u32,
) -> graphics::Result<()> {
    let mut alive = true;

    while alive {
        window.poll_events(|event| match event {
            graphics::window::Event::CloseRequested => {
                alive = false;
            }
        });

        game.update(view, window);
        game.draw(view, window, true)?;
    }

    Ok(())
}
