//! Build a responsive graphical user interface for your game.
//!
//! # Getting started
//! Once you have implemented the [`Game`] trait, you can easily add a user
//! interface to your game by also implementing the [`UserInterface`] trait.
//!
//! Here is an example that will produce an interactive counter:
//!
//! ```
//! use coffee::graphics::{Color, Window};
//! use coffee::ui::{button, Button, Column, Element, Renderer, Text};
//! use coffee::UserInterface;
//! # use coffee::graphics::{Frame, WindowSettings};
//! # use coffee::input::KeyboardAndMouse;
//! # use coffee::load::{loading_screen::ProgressBar, Task};
//! # use coffee::{Game, Result, Timer};
//!
//! struct Counter {
//!     // The counter value
//!     value: i32,
//!
//!     // Local state of the two counter buttons
//!     increment_button: button::State,
//!     decrement_button: button::State,
//! }
//! #
//! # impl Game for Counter {
//! #     type State = ();
//! #     type Input = KeyboardAndMouse;
//! #     type LoadingScreen = ProgressBar;
//! #
//! #     fn load(_window: &Window) -> Task<Counter> {
//! #         Task::new(|| Counter {
//! #             value: 0,
//! #             increment_button: button::State::new(),
//! #             decrement_button: button::State::new(),
//! #         })
//! #     }
//! #
//! #     fn draw(&mut self, _state: &(), frame: &mut Frame, _timer: &Timer) {
//! #         frame.clear(Color::BLACK);
//! #     }
//! # }
//!
//! // The user interactions that we are interested on.
//! #[derive(Debug, Clone, Copy)]
//! pub enum Message {
//!     IncrementPressed,
//!     DecrementPressed,
//! }
//!
//! impl UserInterface for Counter {
//!     // The messages that will be triggered by the counter.
//!     type Message = Message;
//!
//!     // We can simply use the the built-in `Renderer`.
//!     type Renderer = Renderer;
//!
//!     fn update(&mut self, _state: &mut Self::State, message: Message) {
//!         // We update the user interface after an interaction here.
//!         match message {
//!             Message::IncrementPressed => {
//!                 self.value += 1;
//!             }
//!             Message::DecrementPressed => {
//!                 self.value -= 1;
//!             }
//!         }
//!     }
//!
//!     fn layout(&mut self, _state: &Self::State, window: &Window) -> Element<Message> {
//!         // We create the different widgets of our user interface here.
//!         // We use a column so the elements inside are laid out vertically.
//!         Column::new()
//!             .spacing(20) // We set a spacing between elements of 20 pixels
//!             .push(
//!                 Button::new(&mut self.increment_button, "+")
//!                     .on_click(Message::IncrementPressed),
//!             )
//!             .push(Text::new(&self.value.to_string()).size(50))
//!             .push(
//!                 Button::new(&mut self.decrement_button, "-")
//!                     .on_click(Message::DecrementPressed),
//!             )
//!             .into() // We convert the column into a generic `Element`
//!     }
//! }
//! ```
//!
//! The [`Game`] implementation is mostly irrelevant and was omitted in order to
//! keep the example short. You can find the full source code of this example
//! (and other examples too!) in the [`examples` directory on GitHub].
//!
//! [`Game`]: ../trait.Game.html
//! [`UserInterface`]: trait.UserInterface.html
//! [`examples` directory on GitHub]: https://github.com/hecrj/coffee/tree/0.3.0/examples
mod checkbox;
mod panel;
mod radio;
mod renderer;
mod text;

pub mod button;
pub mod core;
pub mod slider;

pub use self::core::{Align, Justify};
pub use button::Button;
pub use checkbox::Checkbox;
pub use panel::Panel;
pub use radio::Radio;
pub use renderer::{Configuration, Renderer};
pub use slider::Slider;
pub use text::Text;

pub type Column<'a, M> = self::core::widget::Column<'a, M, Renderer>;
pub type Row<'a, M> = self::core::widget::Row<'a, M, Renderer>;
pub type Element<'a, M> = self::core::Element<'a, M, Renderer>;

use self::core::{Event, Interface, MouseCursor, Renderer as _};

use crate::game;
use crate::graphics::{window, Window, WindowSettings};
use crate::input::{self, HasCursorPosition, Input};
use crate::load::{Join, LoadingScreen};
use crate::Debug;
use crate::{Game, Result, State, Timer};

/// The user interface of your game.
///
/// Implementors of this trait must also implement [`Game`] and should hold all
/// the state of the user interface.
///
/// Be sure to read the introduction to the [`ui` module] first! It will help
/// you understand the purpose of this trait.
///
/// [`Game`]: ../trait.Game.html
/// [`ui` module]: index.html
pub trait UserInterface: Game {
    /// The type of messages handled by the user interface.
    ///
    /// Messages are produced by user interactions. The runtime feeds these
    /// messages to the [`update`] method, which updates the state of the game
    /// depending on the user interaction.
    ///
    /// The [`Message`] type should normally be an enumeration of different
    /// user interactions. For example:
    ///
    /// ```
    /// enum Message {
    ///     ButtonPressed,
    ///     CheckboxToggled(bool),
    ///     SliderChanged(f32),
    ///     // ...
    /// }
    /// ```
    ///
    /// [`update`]: #tymethod.update
    /// [`Message`]: #associatedtype.Message
    type Message;

    /// The renderer used to draw the user interface.
    ///
    /// If you just want to use the built-in widgets in Coffee, you should
    /// simply use the [`Renderer`] type here.
    ///
    /// If you want to write your own renderer, you will need to implement the
    /// [`core::Renderer`] trait.
    ///
    /// [`Renderer`]: struct.Renderer.html
    /// [`core::Renderer`]: core/trait.Renderer.html
    type Renderer: self::core::Renderer;

    /// Processes a [`Message`], updating game state as needed.
    ///
    /// This method is analogous to [`Game::interact`], but it processes a
    /// [`Message`] instead of [`Game::Input`].
    ///
    /// The logic of your user interface should live here.
    ///
    /// [`Game::interact`]: ../trait.Game.html#method.interact
    /// [`Game::Input`]: ../trait.Game.html#associatedtype.Input
    /// [`Message`]: #associatedtype.Message
    fn update(&mut self, state: &mut Self::State, message: Self::Message);

    /// Produces the layout of the user interface.
    ///
    /// It returns an [`Element`] containing the different widgets that comprise
    /// the user interface.
    ///
    /// This method is called on every frame. The produced layout is rendered
    /// and used by the runtime to allow user interaction.
    ///
    /// [`Element`]: core/struct.Element.html
    fn layout(
        &mut self,
        state: &Self::State,
        window: &Window,
    ) -> self::core::Element<Self::Message, Self::Renderer>;

    /// Builds the renderer configuration for the user interface.
    ///
    /// By default, it returns `Default::default()`.
    fn configuration() -> <Self::Renderer as core::Renderer>::Configuration {
        Default::default()
    }

    /// Runs the [`Game`] with a user interface.
    ///
    /// Call this method instead of [`Game::run`] once you have implemented the
    /// [`UserInterface`] in order to enable it.
    ///
    /// [`Game`]: ../trait.Game.html
    /// [`UserInterface`]: trait.UserInterface.html
    /// [`Game::run`]: ../trait.Game.html#method.run
    fn run(window_settings: WindowSettings) -> Result<()>
    where
        Self: 'static + Sized,
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
        let mut mouse_cursor = MouseCursor::OutOfBounds;
        let mut ui_cache =
            Interface::compute(game.layout(state, window), &renderer).cache();

        while alive {
            debug.frame_started();
            timer.update();

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
                .drain(..)
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
                if new_cursor == MouseCursor::OutOfBounds {
                    input.update(input::Event::CursorReturned);
                } else if mouse_cursor == MouseCursor::OutOfBounds {
                    input.update(input::Event::CursorTaken);
                }

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
