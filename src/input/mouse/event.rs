use super::Button;
use crate::input::ButtonState;

/// A mouse event.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    /// The mouse cursor was moved
    CursorMoved {
        /// The X coordinate of the mouse position
        x: f32,

        /// The Y coordinate of the mouse position
        y: f32,
    },

    /// The mouse cursor entered the game window.
    CursorEntered,

    /// The mouse cursor left the game window.
    CursorLeft,

    /// The mouse cursor has been taken and is in use.
    ///
    /// This event is fired when the cursor is hovering or interacting with a
    /// [`UserInterface`].
    ///
    /// [`UserInterface`]: ../../ui/trait.UserInterface.html
    CursorTaken,

    /// The mouse cursor has been returned and is no longer in use.
    CursorReturned,

    /// A mouse button was pressed or released.
    Input {
        /// The state of the button
        state: ButtonState,

        /// The button identifier
        button: Button,
    },

    /// The mouse wheel was scrolled.
    WheelScrolled {
        /// The number of horizontal lines scrolled
        delta_x: f32,

        /// The number of vertical lines scrolled
        delta_y: f32,
    },
}

#[doc(hidden)]
impl From<Event> for Option<iced::input::mouse::Event> {
    fn from(event: Event) -> Option<iced::input::mouse::Event> {
        match event {
            Event::CursorEntered => {
                Some(iced::input::mouse::Event::CursorEntered)
            }
            Event::CursorLeft => Some(iced::input::mouse::Event::CursorEntered),
            Event::CursorMoved { x, y } => {
                Some(iced::input::mouse::Event::CursorMoved { x, y })
            }
            Event::Input { state, button } => {
                Some(iced::input::mouse::Event::Input {
                    state: state.into(),
                    button: button.into(),
                })
            }
            Event::WheelScrolled { delta_x, delta_y } => {
                Some(iced::input::mouse::Event::WheelScrolled {
                    delta_x,
                    delta_y,
                })
            }
            Event::CursorTaken => None,
            Event::CursorReturned => None,
        }
    }
}
