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
