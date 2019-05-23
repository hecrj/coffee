mod column;
mod event;
mod hasher;
mod interface;
mod layout;
mod map;
mod mouse_cursor;
mod node;
mod root;
mod row;
mod style;
mod widget;

pub mod button;
pub mod panel;
pub mod renderer;
pub mod text;

pub use stretch::geometry::Size;
pub use stretch::number::Number;

pub use button::Button;
pub use column::Column;
pub use event::Event;
pub use hasher::Hasher;
pub use layout::Layout;
pub use map::Map;
pub use mouse_cursor::MouseCursor;
pub use node::Node;
pub use panel::Panel;
pub use renderer::Renderer;
pub use root::Root;
pub use row::Row;
pub use style::Style;
pub use text::Text;
pub use widget::Widget;

use crate::game;
use crate::graphics::{window, Window, WindowSettings};
use crate::input::{HasCursorPosition, Input};
use crate::load::{Join, LoadingScreen};
use crate::Debug;
use crate::{Game, Result, State, Timer};
use interface::Interface;

pub trait UserInterface: Game {
    type Event;
    type Renderer: Renderer;

    fn update(&mut self, state: &mut Self::State, event: Self::Event);

    fn layout(
        &mut self,
        state: &Self::State,
        window: &Window,
    ) -> Root<Self::Event, Self::Renderer>;

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
            Self::Renderer::load(),
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
            game.draw(state, window, &timer);
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
}
