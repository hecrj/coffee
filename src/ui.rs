mod checkbox;
mod panel;
mod radio;
mod text;

pub mod button;
pub mod core;
pub mod renderer;
pub mod slider;

pub use self::core::{Align, Justify};
pub use button::Button;
pub use checkbox::Checkbox;
pub use panel::Panel;
pub use radio::Radio;
pub use renderer::Renderer;
pub use slider::Slider;
pub use text::Text;

pub type Column<'a, M> = self::core::widget::Column<'a, M, Renderer>;
pub type Row<'a, M> = self::core::widget::Row<'a, M, Renderer>;
pub type Element<'a, M> = self::core::Element<'a, M, Renderer>;

use self::core::{Event, Interface, MouseCursor, Renderer as _};

use crate::game;
use crate::graphics::{window, Window, WindowSettings};
use crate::input::{HasCursorPosition, Input};
use crate::load::{Join, LoadingScreen};
use crate::Debug;
use crate::{Game, Result, State, Timer};

pub trait UserInterface: Game {
    type Message;
    type Renderer: self::core::Renderer;

    fn update(&mut self, state: &mut Self::State, message: Self::Message);

    fn layout(
        &mut self,
        state: &Self::State,
        window: &Window,
    ) -> self::core::Element<Self::Message, Self::Renderer>;

    fn run(window_settings: WindowSettings) -> Result<()>
    where
        Self: Sized + 'static,
        Self::Input: HasCursorPosition,
    {
        // Set up window
        let mut event_loop = window::EventLoop::new();
        let window = &mut Window::new(window_settings, &event_loop)?;
        let mut debug = Debug::new(window.gpu(), Self::State::TICKS_PER_SECOND);

        // Load game
        debug.loading_started();
        let mut loading_screen = Self::LoadingScreen::new(window.gpu())?;
        let load = (
            Self::load(window),
            Self::State::load(window),
            Self::Renderer::load(Self::configuration()),
        )
            .join();
        let (game, state, renderer) = &mut loading_screen.run(load, window)?;
        let input = &mut Self::Input::new();
        debug.loading_finished();

        // Game loop
        let mut timer = Timer::new(Self::State::TICKS_PER_SECOND);
        let mut alive = true;
        let events = &mut Vec::new();
        let triggered_events = &mut Vec::new();
        let mut mouse_cursor = MouseCursor::Default;
        let mut ui_cache =
            Interface::compute(game.layout(state, window), &renderer).cache();

        while alive {
            debug.frame_started();
            timer.update();
            events.clear();

            while timer.tick() {
                game::process_events(
                    game,
                    input,
                    state,
                    &mut debug,
                    window,
                    &mut event_loop,
                    &mut alive,
                    Some(events),
                );

                debug.update_started();
                state.update();
                debug.update_finished();
            }

            if !timer.has_ticked() {
                game::process_events(
                    game,
                    input,
                    state,
                    &mut debug,
                    window,
                    &mut event_loop,
                    &mut alive,
                    Some(events),
                );
            }

            debug.draw_started();
            game.draw(state, &mut window.frame(), &timer);
            debug.draw_finished();

            debug.ui_started();
            let mut interface = Interface::compute_with_cache(
                game.layout(state, window),
                &renderer,
                ui_cache,
            );

            events
                .iter()
                .cloned()
                .filter_map(Event::from_input)
                .for_each(|event| {
                    interface.on_event(
                        event,
                        input.cursor_position(),
                        triggered_events,
                    )
                });

            let new_cursor =
                interface.draw(renderer, window, input.cursor_position());

            ui_cache = interface.cache();

            if new_cursor != mouse_cursor {
                window.update_cursor(new_cursor.into());
                mouse_cursor = new_cursor;
            }

            for event in triggered_events.drain(..) {
                game.update(state, event);
            }
            debug.ui_finished();

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

    fn configuration() -> <Self::Renderer as core::Renderer>::Configuration {
        <Self::Renderer as core::Renderer>::Configuration::default()
    }
}
