use crate::graphics::window::winit;

pub use gilrs::ev::state::GamepadState;
pub use gilrs::ev::EventType as GamepadEvent;
pub use gilrs::GamepadId;
pub use winit::ElementState as ButtonState;
pub use winit::MouseButton;
pub use winit::VirtualKeyCode as KeyCode;

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
    /// A keyboard key was pressed or released.
    KeyboardInput {
        /// The state of the key
        state: ButtonState,

        /// The key identifier
        key_code: KeyCode,
    },

    /// Text was entered.
    TextInput {
        /// The character entered
        character: char,
    },

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
    /// [`UserInterface`]: ../ui/trait.UserInterface.html
    CursorTaken,

    /// The mouse cursor has been returned and is no longer in use.
    CursorReturned,

    /// A mouse button was pressed or released.
    MouseInput {
        /// The state of the button
        state: ButtonState,

        /// The button identifier
        button: MouseButton,
    },

    /// The mouse wheel was scrolled.
    MouseWheel {
        /// The number of horizontal lines scrolled
        delta_x: f32,

        /// The number of vertical lines scrolled
        delta_y: f32,
    },

    /// An event from a gamepad was emitted.
    Gamepad {
        /// Thw id of the gamepad that emitted the event.
        id: GamepadId,

        /// The gamepad event.
        event: GamepadEvent,
    },

    /// The game window gained focus.
    WindowFocused,

    /// The game window lost focus.
    WindowUnfocused,

    /// The game window was moved.
    WindowMoved {
        /// The new X coordinate of the window
        x: f32,

        /// The new Y coordinate of the window
        y: f32,
    },
}
