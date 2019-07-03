//! Build a responsive graphical user interface for your game.
//!
//! # Basic concepts
//! The user interface runtime in Coffee is heavily inspired by [Elm] and
//! [The Elm Architecture].
//!
//! Basically, user interfaces in Coffee are split into four different concepts:
//!
//!   * __state__ — data owned by the implementor of [`UserInterface`]
//!   * __messages__ — user interactions or meaningful events that you care
//!   about
//!   * __update logic__ — a way to react to __messages__ and update your
//!   __state__
//!   * __layout logic__ — a way to transform your __state__ into [widgets] that
//!   may produce __messages__ on user interaction
//!
//! # Getting started
//! Once you have implemented the [`Game`] trait, you can easily add a user
//! interface to your game by implementing the [`UserInterface`] trait.
//!
//! Let's take a look at a simple example with basic user interaction: an
//! interactive counter that can be incremented and decremented using two
//! different buttons.
//!
//! ```
//! use coffee::graphics::{Color, Window};
//! use coffee::ui::{button, Button, Column, Element, Renderer, Text, UserInterface};
//! # use coffee::graphics::{Frame, WindowSettings};
//! # use coffee::input::KeyboardAndMouse;
//! # use coffee::load::{loading_screen::ProgressBar, Task};
//! # use coffee::{Game, Result, Timer};
//!
//! // The state of our user interface
//! struct Counter {
//!     // The counter value
//!     value: i32,
//!
//!     // Local state of the two counter buttons
//!     // This is internal widget state that may change outside our update
//!     // logic
//!     increment_button: button::State,
//!     decrement_button: button::State,
//! }
//!
//! # impl Game for Counter {
//! #     type Input = KeyboardAndMouse;
//! #     type LoadingScreen = ProgressBar;
//! #
//! #     fn load(_window: &Window) -> Task<Counter> {
//! #         Task::succeed(|| Counter {
//! #             value: 0,
//! #             increment_button: button::State::new(),
//! #             decrement_button: button::State::new(),
//! #         })
//! #     }
//! #
//! #     fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
//! #         frame.clear(Color::BLACK);
//! #     }
//! # }
//! #
//! // The messages, user interactions that we are interested on
//! #[derive(Debug, Clone, Copy)]
//! pub enum Message {
//!     IncrementPressed,
//!     DecrementPressed,
//! }
//!
//! impl UserInterface for Counter {
//!     // We use the message enum we just defined
//!     type Message = Message;
//!
//!     // We can use the the built-in `Renderer`
//!     type Renderer = Renderer;
//!
//!     // The update logic, called when a message is produced
//!     fn react(&mut self, message: Message) {
//!         // We update the counter value after an interaction here
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
//!     // The layout logic, describing the different components of the user interface
//!     fn layout(&mut self, window: &Window) -> Element<Message> {
//!         // We use a column so the elements inside are laid out vertically
//!         Column::new()
//!             .push(
//!                 // The increment button. We tell it to produce an
//!                 // `IncrementPressed` message when pressed
//!                 Button::new(&mut self.increment_button, "+")
//!                     .on_press(Message::IncrementPressed),
//!             )
//!             .push(
//!                 // We show the value of the counter here
//!                 Text::new(&self.value.to_string()).size(50),
//!             )
//!             .push(
//!                 // The decrement button. We tell it to produce a
//!                 // `DecrementPressed` message when pressed
//!                 Button::new(&mut self.decrement_button, "-")
//!                     .on_press(Message::DecrementPressed),
//!             )
//!             .into() // We need to return a generic `Element`
//!     }
//! }
//! ```
//!
//! _The [`Game`] implementation is mostly irrelevant and was omitted in order to
//! keep the example short. You can find the full source code of this example
//! (and other examples too!) in the [`examples` directory on GitHub]._
//!
//! Notice how [`UserInterface::react`] focuses on processing messages and
//! updating state. On the other hand, [`UserInterface::layout`] only focuses on
//! building the user interface from the current state. This separation of
//! concerns will help you build composable user interfaces that are easy to
//! debug and test!
//!
//! # Customization
//! Coffee provides some [widgets] and a [`Renderer`] out-of-the-box. However,
//! you can build your own! Check out the [`core`] module to learn more!
//!
//! [Elm]: https://elm-lang.org
//! [The Elm Architecture]: https://guide.elm-lang.org/architecture/
//! [`UserInterface`]: trait.UserInterface.html
//! [`UserInterface::react`]: trait.UserInterface.html#tymethod.react
//! [`UserInterface::layout`]: trait.UserInterface.html#tymethod.layout
//! [`UserInterface::Message`]: trait.UserInterface.html#associatedtype.Message
//! [widgets]: widget/index.html
//! [`Button`]: widget/button/struct.Button.html
//! [`Game`]: ../trait.Game.html
//! [`examples` directory on GitHub]: https://github.com/hecrj/coffee/tree/master/examples
//! [`Renderer`]: struct.Renderer.html
//! [`core`]: core/index.html
pub mod core;
mod renderer;
pub mod widget;

#[doc(no_inline)]
pub use self::core::{Align, Justify};
pub use renderer::{Configuration, Renderer};
pub use widget::{button, slider, Button, Checkbox, Radio, Slider, Text};

/// A [`Column`] using the built-in [`Renderer`].
///
/// [`Column`]: widget/struct.Column.html
/// [`Renderer`]: struct.Renderer.html
pub type Column<'a, Message> = widget::Column<'a, Message, Renderer>;

/// A [`Row`] using the built-in [`Renderer`].
///
/// [`Row`]: widget/struct.Row.html
/// [`Renderer`]: struct.Renderer.html
pub type Row<'a, Message> = widget::Row<'a, Message, Renderer>;

/// An [`Element`] using the built-in [`Renderer`].
///
/// [`Element`]: core/struct.Element.html
/// [`Renderer`]: struct.Renderer.html
pub type Element<'a, Message> = self::core::Element<'a, Message, Renderer>;

use crate::game;
use crate::graphics::{window, Point, Window, WindowSettings};
use crate::input::{self, gamepad, mouse, Input as _};
use crate::load::{Join, LoadingScreen};
use crate::ui::core::{Event, Interface, MouseCursor, Renderer as _};
use crate::{Debug, Game, Result, Timer};

/// The user interface of your game.
///
/// Implementors of this trait must also implement [`Game`] and should hold all
/// the state of the user interface.
///
/// Be sure to read the introduction of the [`ui` module] first! It will help
/// you understand the purpose of this trait.
///
/// [`Game`]: ../trait.Game.html
/// [`ui` module]: index.html
pub trait UserInterface: Game {
    /// The type of messages handled by the user interface.
    ///
    /// Messages are produced by user interactions. The runtime feeds these
    /// messages to the [`react`] method, which updates the state of the game
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
    /// [`react`]: #tymethod.react
    /// [`Message`]: #associatedtype.Message
    type Message;

    /// The renderer used to draw the user interface.
    ///
    /// If you just want to use the built-in widgets in Coffee, you should
    /// use the built-in [`Renderer`] type here.
    ///
    /// If you want to write your own renderer, you will need to implement the
    /// [`core::Renderer`] trait.
    ///
    /// [`Renderer`]: struct.Renderer.html
    /// [`core::Renderer`]: core/trait.Renderer.html
    type Renderer: self::core::Renderer;

    /// Reacts to a [`Message`], updating game state as needed.
    ///
    /// This method is analogous to [`Game::interact`], but it processes a
    /// [`Message`] instead of [`Game::Input`].
    ///
    /// The logic of your user interface should live here.
    ///
    /// [`Game::interact`]: ../trait.Game.html#method.interact
    /// [`Game::Input`]: ../trait.Game.html#associatedtype.Input
    /// [`Message`]: #associatedtype.Message
    fn react(&mut self, message: Self::Message);

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
        window: &Window,
    ) -> self::core::Element<'_, Self::Message, Self::Renderer>;

    /// Builds the renderer configuration for the user interface.
    ///
    /// By default, it returns `Default::default()`.
    fn configuration() -> <Self::Renderer as core::Renderer>::Configuration {
        Default::default()
    }

    /// Runs the [`Game`] with a user interface.
    ///
    /// Call this method instead of [`Game::run`] once you have implemented the
    /// [`UserInterface`].
    ///
    /// [`Game`]: ../trait.Game.html
    /// [`UserInterface`]: trait.UserInterface.html
    /// [`Game::run`]: ../trait.Game.html#method.run
    fn run(window_settings: WindowSettings) -> Result<()>
    where
        Self: 'static + Sized,
    {
        // Set up window
        let event_loop = &mut window::EventLoop::new();
        let window = &mut Window::new(window_settings, &event_loop)?;
        let mut debug = Debug::new(window.gpu());

        // Load game
        debug.loading_started();
        let mut loading_screen = Self::LoadingScreen::new(window.gpu())?;
        let load = (
            Self::load(window),
            Self::Renderer::load(Self::configuration()),
        )
            .join();
        let (game, renderer) = &mut loading_screen.run(load, window)?;
        let input = &mut Input::new();
        let mut gamepads = gamepad::Tracker::new();
        debug.loading_finished();

        // Game loop
        let mut timer = Timer::new(Self::TICKS_PER_SECOND);
        let mut alive = true;
        let messages = &mut Vec::new();
        let mut mouse_cursor = MouseCursor::OutOfBounds;
        let mut ui_cache =
            Interface::compute(game.layout(window), &renderer).cache();

        while alive {
            debug.frame_started();
            timer.update();

            while timer.tick() {
                interact(
                    game,
                    input,
                    &mut debug,
                    window,
                    event_loop,
                    gamepads.as_mut(),
                    &mut alive,
                );

                debug.update_started();
                game.update(window);
                debug.update_finished();
            }

            if !timer.has_ticked() {
                interact(
                    game,
                    input,
                    &mut debug,
                    window,
                    event_loop,
                    gamepads.as_mut(),
                    &mut alive,
                );
            }

            debug.draw_started();
            game.draw(&mut window.frame(), &timer);
            debug.draw_finished();

            debug.ui_started();
            let mut interface = Interface::compute_with_cache(
                game.layout(window),
                &renderer,
                ui_cache,
            );

            let cursor_position = input.cursor_position;
            input.ui_events.drain(..).for_each(|event| {
                interface.on_event(event, cursor_position, messages)
            });

            let new_cursor = interface.draw(
                renderer,
                &mut window.frame(),
                input.cursor_position,
            );

            ui_cache = interface.cache();

            if new_cursor != mouse_cursor {
                if new_cursor == MouseCursor::OutOfBounds {
                    input.update(input::Event::Mouse(
                        mouse::Event::CursorReturned,
                    ));
                } else if mouse_cursor == MouseCursor::OutOfBounds {
                    input
                        .update(input::Event::Mouse(mouse::Event::CursorTaken));
                }

                window.update_cursor(new_cursor.into());
                mouse_cursor = new_cursor;
            }

            for message in messages.drain(..) {
                game.react(message);
            }
            debug.ui_finished();

            if debug.is_enabled() {
                debug.debug_started();
                game.debug(
                    &mut input.game_input,
                    &mut window.frame(),
                    &mut debug,
                );
                debug.debug_finished();
            }

            window.swap_buffers();
            debug.frame_finished();
        }

        Ok(())
    }
}

struct Input<I: input::Input> {
    game_input: I,
    cursor_position: Point,
    ui_events: Vec<Event>,
}

impl<I: input::Input> input::Input for Input<I> {
    fn new() -> Input<I> {
        Input {
            game_input: I::new(),
            cursor_position: Point::new(0.0, 0.0),
            ui_events: Vec::new(),
        }
    }

    fn update(&mut self, event: input::Event) {
        self.game_input.update(event);

        match event {
            input::Event::Mouse(mouse::Event::CursorMoved { x, y }) => {
                self.cursor_position = Point::new(x, y);
            }
            _ => {}
        };

        if let Some(ui_event) = Event::from_input(event) {
            self.ui_events.push(ui_event);
        }
    }

    fn clear(&mut self) {
        self.game_input.clear();
    }
}

fn interact<G: Game>(
    game: &mut G,
    input: &mut Input<G::Input>,
    debug: &mut Debug,
    window: &mut Window,
    event_loop: &mut window::EventLoop,
    gamepads: Option<&mut gamepad::Tracker>,
    alive: &mut bool,
) {
    debug.interact_started();

    event_loop.poll(|event| {
        game::process_window_event(game, input, debug, window, alive, event)
    });

    game::process_gamepad_events(gamepads, input);

    game.interact(&mut input.game_input, window);
    input.clear();

    debug.interact_finished();
}
