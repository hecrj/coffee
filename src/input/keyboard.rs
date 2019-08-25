//! Listen to keyboard events.

mod event;

pub use crate::graphics::window::winit::event::VirtualKeyCode as KeyCode;
pub use event::Event;

use super::{ButtonState, Event as InputEvent, Input};

use std::collections::HashSet;

/// A simple keyboard input tracker.
///
/// You can use this as your [`Game::Input`] directly!
///
/// [`Game::Input`]: ../trait.Game.html#associatedtype.Input
#[derive(Debug, Clone)]
pub struct Keyboard {
    pressed_keys: HashSet<KeyCode>,
    released_keys: HashSet<KeyCode>,
}

impl Keyboard {
    /// Returns true if the given key is currently pressed.
    pub fn is_key_pressed(&self, key_code: KeyCode) -> bool {
        self.pressed_keys.contains(&key_code)
    }

    /// Returns true if the given key was released during the last interaction.
    pub fn was_key_released(&self, key_code: KeyCode) -> bool {
        self.released_keys.contains(&key_code)
    }
}

impl Input for Keyboard {
    fn new() -> Keyboard {
        Keyboard {
            pressed_keys: HashSet::new(),
            released_keys: HashSet::new(),
        }
    }

    fn update(&mut self, event: InputEvent) {
        match event {
            InputEvent::Mouse { .. } => {
                // Ignore mouse events...
            }
            InputEvent::Keyboard(keyboard_event) => match keyboard_event {
                Event::Input { key_code, state } => {
                    match state {
                        ButtonState::Pressed => {
                            let _ = self.pressed_keys.insert(key_code);
                        }
                        ButtonState::Released => {
                            let _ = self.pressed_keys.remove(&key_code);
                            let _ = self.released_keys.insert(key_code);
                        }
                    };
                }
                Event::TextEntered { .. } => {}
            },
            InputEvent::Gamepad { .. } => {
                // Ignore gamepad events...
            }
            InputEvent::Window { .. } => {
                // Ignore window events...
            }
        }
    }

    fn clear(&mut self) {
        self.released_keys.clear();
    }
}
