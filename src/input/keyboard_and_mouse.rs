use super::{Event, Input};
use super::mouse::Mouse;
use super::keyboard::Keyboard;

/// A simple keyboard and mouse input tracker.
///
/// You can use this as your [`Game::Input`] directly!
///
/// [`Game::Input`]: ../trait.Game.html#associatedtype.Input
#[derive(Debug)]
pub struct KeyboardAndMouse {
    mouse: Mouse,
    keyboard: Keyboard
}

impl KeyboardAndMouse {
    /// Returns the mouse input tracker.
    pub fn mouse(&self) -> &Mouse {
        &self.mouse
    }

    /// Returns the mouse input tracker.
    pub fn keyboard(&self) -> &Keyboard {
        &self.keyboard
    }
}

impl Input for KeyboardAndMouse {
    fn new() -> KeyboardAndMouse {
        KeyboardAndMouse {
            mouse: Mouse::new(),
            keyboard: Keyboard::new()
        }
    }

    fn update(&mut self, event: Event) {
        self.mouse.update(event);
        self.keyboard.update(event);
    }

    fn clear(&mut self) {
        self.mouse.clear();
        self.keyboard.clear();
    }
}
