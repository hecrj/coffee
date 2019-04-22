//! Allow players to interact with your game.
use crate::graphics::window::winit;

pub use winit::ElementState as KeyState;
pub use winit::VirtualKeyCode as KeyCode;

/// An input event.
///
/// There are many events still missing here! Controllers are also not supported
/// _yet_!
///
/// Feel free to [open an issue] if you need a particular event.
/// [PRs are also appreciated!]
///
/// [open an issue]: https://github.com/hecrj/coffee/issues
/// [PRS are also appreciated!]: https://github.com/hecrj/coffee/pulls
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Event {
    /// A keyboard key was pressed or released.
    KeyboardInput { state: KeyState, key_code: KeyCode },

    /// The mouse cursor was moved.
    CursorMoved { x: f32, y: f32 },
}
