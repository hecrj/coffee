use crate::graphics::window::winit;
use std::convert::TryFrom;

/// Describes the appearance of the mouse cursor.
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

impl TryFrom<CursorIcon> for winit::window::CursorIcon {
    type Error = ();

    fn try_from(
        cursor_icon: CursorIcon,
    ) -> Result<winit::window::CursorIcon, ()> {
        match cursor_icon {
            CursorIcon::Default => Ok(winit::window::CursorIcon::Default),
            CursorIcon::Crosshair => Ok(winit::window::CursorIcon::Crosshair),
            CursorIcon::Hand => Ok(winit::window::CursorIcon::Hand),
            CursorIcon::Hidden => Err(()),
            CursorIcon::Move => Ok(winit::window::CursorIcon::Move),
        }
    }
}
