use crate::graphics::window::winit;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum MouseCursor {
    Default,
    Pointer,
    Working,
}

impl From<MouseCursor> for winit::MouseCursor {
    fn from(mouse_cursor: MouseCursor) -> winit::MouseCursor {
        match mouse_cursor {
            MouseCursor::Default => winit::MouseCursor::Default,
            MouseCursor::Pointer => winit::MouseCursor::Hand,
            MouseCursor::Working => winit::MouseCursor::Progress,
        }
    }
}
