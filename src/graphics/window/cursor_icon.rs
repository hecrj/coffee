use crate::graphics::window::winit;

/// TODO
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CursorIcon {
    /// The platform-dependent default cursor.
    Default,
    /// A simple crosshair.
    Crosshair,
    /// A hand (often used to indicate links in web browsers).
    Hand,
    /// Hides the cursor.
    Hidden,
    /// Indicates something is to be moved.
    Move,
}

impl Default for CursorIcon {
    fn default() -> Self {
        Self::Default
    }
}

impl From<CursorIcon> for winit::window::CursorIcon {
    fn from(cursor_icon: CursorIcon) -> winit::window::CursorIcon {
        match cursor_icon {
            // If the cursor is hidden, it doesn't matter which type it is, so the default makes
            // the most sense.
            CursorIcon::Default | CursorIcon::Hidden => {
                winit::window::CursorIcon::Default
            }
            CursorIcon::Crosshair => winit::window::CursorIcon::Crosshair,
            CursorIcon::Hand => winit::window::CursorIcon::Hand,
            CursorIcon::Move => winit::window::CursorIcon::Move,
        }
    }
}
