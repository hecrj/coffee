use crate::input::{gamepad, keyboard, mouse, window};

use std::time::SystemTime;

/// An input event.
///
/// Input events in your [`Game`] are processed by the [`Game::Input`] associated
/// type.
///
/// You can use your own input handler by implementing the [`Input`] trait.
///
/// [`Game`]: ../trait.Game.html
/// [`Game::Input`]: ../trait.Game.html#associatedtype.Input
/// [`Input`]: trait.Input.html
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Event {
    /// A keyboard event
    Keyboard(keyboard::Event),

    /// A mouse event
    Mouse(mouse::Event),

    /// A gamepad event
    Gamepad {
        /// The gamepad identifier
        id: gamepad::Id,

        /// The gamepad event
        event: gamepad::Event,

        /// The time of the event
        time: SystemTime,
    },

    /// A window event
    Window(window::Event),
}
