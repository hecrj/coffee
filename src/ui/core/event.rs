use crate::graphics::window::winit;
use crate::input;

pub use winit::ElementState as ButtonState;
pub use winit::MouseButton;
pub use winit::VirtualKeyCode as KeyCode;

/// A user interface event.
///
/// This is a subset of [`input::Event`].
///
/// [`input::Event`]: ../../input/enum.Event.html
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

    /// The mouse cursor was moved.
    CursorMoved,

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
}

impl Event {
    pub(crate) fn from_input(event: input::Event) -> Option<Event> {
        match event {
            input::Event::KeyboardInput { state, key_code } => {
                Some(Event::KeyboardInput { state, key_code })
            }
            input::Event::TextInput { character } => {
                Some(Event::TextInput { character })
            }
            input::Event::CursorMoved { .. } => Some(Event::CursorMoved),
            input::Event::MouseInput { state, button } => {
                Some(Event::MouseInput { state, button })
            }
            input::Event::MouseWheel { delta_x, delta_y } => {
                Some(Event::MouseWheel { delta_x, delta_y })
            }
            _ => None,
        }
    }
}
