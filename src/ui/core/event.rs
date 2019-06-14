use crate::input::{self, gamepad, keyboard, mouse};

/// A user interface event.
///
/// This is a subset of [`input::Event`].
///
/// [`input::Event`]: ../../input/enum.Event.html
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
    },
}

impl Event {
    pub(crate) fn from_input(event: input::Event) -> Option<Event> {
        match event {
            input::Event::Keyboard(keyboard_event) => {
                Some(Event::Keyboard(keyboard_event))
            }
            input::Event::Mouse(mouse_event) => Some(Event::Mouse(mouse_event)),
            input::Event::Gamepad { id, event, .. } => {
                Some(Event::Gamepad { id, event })
            }
            _ => None,
        }
    }
}
