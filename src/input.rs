//! Allow players to interact with your game.
use crate::graphics::window::winit;

pub use winit::ElementState as ButtonState;
pub use winit::MouseButton;
pub use winit::VirtualKeyCode as KeyCode;

pub use gilrs::ev::EventType;
pub use gilrs::GamepadId;
use std::time::SystemTime;

/// An input event.
///
/// You can listen to this type of events by implementing [`Game::on_input`].
///
/// There are many events still missing here!
///
/// Feel free to [open an issue] if you need a particular event.
/// [PRs are also appreciated!]
///
/// [`Game::on_input`]: ../trait.Game.html#method.on_input
/// [open an issue]: https://github.com/hecrj/coffee/issues
/// [PRS are also appreciated!]: https://github.com/hecrj/coffee/pulls
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Event {
    /// A keyboard key was pressed or released.
    KeyboardInput {
        /// The state of the key
        state: ButtonState,

        /// The key identifier
        key_code: KeyCode,
    },

    /// The mouse cursor was moved
    CursorMoved {
        /// The X coordinate of the mouse position
        x: f32,

        /// The Y coordinate of the mouse position
        y: f32,
    },

    /// A mouse button was pressed or released.
    MouseInput {
        /// The state of the button
        state: ButtonState,

        /// The button identifier
        button: MouseButton,
    },
    /// An event from a gamepad was emitted.
    /// Maps to [gilrs::ev::Event](https://docs.rs/gilrs/0.7.1/gilrs/ev/struct.Event.html)
    GamepadEvent {
        /// Id of gamepad.
        id: GamepadId,
        /// Event's data.
        event: EventType,
        /// Time when event was emitted.
        time: SystemTime,
    },
}
