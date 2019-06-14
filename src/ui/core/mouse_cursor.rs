use crate::graphics::window::winit;

/// The state of the mouse cursor.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum MouseCursor {
    /// The cursor is out of the bounds of the user interface.
    OutOfBounds,

    /// The cursor is over a non-interactive widget.
    Idle,

    /// The cursor is over a clickable widget.
    Pointer,

    /// The cursor is over a busy widget.
    Working,

    /// The cursor is over a grabbable widget.
    Grab,

    /// The cursor is grabbing a widget.
    Grabbing,
}

#[doc(hidden)]
impl From<MouseCursor> for winit::MouseCursor {
    fn from(mouse_cursor: MouseCursor) -> winit::MouseCursor {
        match mouse_cursor {
            MouseCursor::OutOfBounds => winit::MouseCursor::Default,
            MouseCursor::Idle => winit::MouseCursor::Default,
            MouseCursor::Pointer => winit::MouseCursor::Hand,
            MouseCursor::Working => winit::MouseCursor::Progress,
            MouseCursor::Grab => winit::MouseCursor::Grab,
            MouseCursor::Grabbing => winit::MouseCursor::Grabbing,
        }
    }
}
