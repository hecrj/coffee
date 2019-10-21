use super::KeyCode;
use crate::input::ButtonState;

#[derive(Debug, Clone, Copy, PartialEq)]
/// A keyboard event.
pub enum Event {
    /// A keyboard key was pressed or released.
    Input {
        /// The state of the key
        state: ButtonState,

        /// The key identifier
        key_code: KeyCode,
    },

    /// Text was entered.
    TextEntered {
        /// The character entered
        character: char,
    },
}

#[doc(hidden)]
impl From<Event> for iced::input::keyboard::Event {
    fn from(event: Event) -> iced::input::keyboard::Event {
        match event {
            Event::Input { state, key_code } => {
                iced::input::keyboard::Event::Input {
                    state: state.into(),
                    key_code: key_code.into(),
                }
            }

            Event::TextEntered { character } => {
                iced::input::keyboard::Event::CharacterReceived(character)
            }
        }
    }
}
