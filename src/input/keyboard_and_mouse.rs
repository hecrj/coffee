use super::keyboard::Keyboard;
use super::mouse::Mouse;
use super::{Event, Input};

/// A simple keyboard and mouse input tracker.
///
/// You can use this as your [`Game::Input`] directly!
///
/// [`Game::Input`]: ../trait.Game.html#associatedtype.Input
#[derive(Debug, Clone)]
pub struct KeyboardAndMouse {
    mouse: Mouse,
    keyboard: Keyboard,
}

impl KeyboardAndMouse {
    /// Returns the [`Mouse`] input.
    ///
    /// [`Mouse`]: mouse/struct.Mouse.html
    pub fn mouse(&self) -> &Mouse {
        &self.mouse
    }

    /// Returns the [`Keyboard`] input.
    ///
    /// [`Keyboard`]: keyboard/struct.Keyboard.html
    pub fn keyboard(&self) -> &Keyboard {
        &self.keyboard
    }
}

impl Input for KeyboardAndMouse {
    fn new() -> KeyboardAndMouse {
        KeyboardAndMouse {
            mouse: Mouse::new(),
            keyboard: Keyboard::new(),
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
