use super::{keyboard, mouse, ButtonState, Event, Input};
use crate::graphics::Point;

use std::collections::HashSet;

/// A simple keyboard and mouse input tracker.
///
/// You can use this as your [`Game::Input`] directly!
///
/// [`Game::Input`]: ../trait.Game.html#associatedtype.Input
#[derive(Debug)]
pub struct KeyboardAndMouse {
    cursor_position: Point,
    is_cursor_taken: bool,
    is_mouse_pressed: bool,
    left_clicks: Vec<Point>,
    pressed_keys: HashSet<keyboard::KeyCode>,
    released_keys: HashSet<keyboard::KeyCode>,
}

impl KeyboardAndMouse {
    /// Returns the current cursor position.
    pub fn cursor_position(&self) -> Point {
        self.cursor_position
    }

    /// Returns true if the cursor is currently not available.
    ///
    /// This mostly happens when the cursor is currently over a
    /// [`UserInterface`].
    ///
    /// [`UserInterface`]: ../ui/trait.UserInterface.html
    pub fn is_cursor_taken(&self) -> bool {
        self.is_cursor_taken
    }

    /// Returns the positions of the mouse clicks during the last interaction.
    ///
    /// Clicks performed while the mouse cursor is not available are
    /// automatically ignored.
    pub fn left_clicks(&self) -> &[Point] {
        &self.left_clicks
    }

    /// Returns true if the given key is currently pressed.
    pub fn is_key_pressed(&self, key_code: keyboard::KeyCode) -> bool {
        self.pressed_keys.contains(&key_code)
    }

    /// Returns true if the given key was released during the last interaction.
    pub fn was_key_released(&self, key_code: keyboard::KeyCode) -> bool {
        self.released_keys.contains(&key_code)
    }
}

impl Input for KeyboardAndMouse {
    fn new() -> KeyboardAndMouse {
        KeyboardAndMouse {
            cursor_position: Point::new(0.0, 0.0),
            is_cursor_taken: false,
            is_mouse_pressed: false,
            left_clicks: Vec::new(),
            pressed_keys: HashSet::new(),
            released_keys: HashSet::new(),
        }
    }

    fn update(&mut self, event: Event) {
        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::CursorMoved { x, y } => {
                    self.cursor_position = Point::new(x, y);
                }
                mouse::Event::CursorTaken => {
                    self.is_cursor_taken = true;
                }
                mouse::Event::CursorReturned => {
                    self.is_cursor_taken = false;
                }
                mouse::Event::Input {
                    button: mouse::Button::Left,
                    state,
                } => match state {
                    ButtonState::Pressed => {
                        self.is_mouse_pressed = !self.is_cursor_taken;
                    }
                    ButtonState::Released => {
                        if !self.is_cursor_taken && self.is_mouse_pressed {
                            self.left_clicks.push(self.cursor_position);
                        }

                        self.is_mouse_pressed = false;
                    }
                },
                mouse::Event::Input { .. } => {
                    // TODO: Track other buttons!
                }
                mouse::Event::CursorEntered => {
                    // TODO: Track it!
                }
                mouse::Event::CursorLeft => {
                    // TODO: Track it!
                }
                mouse::Event::WheelScrolled { .. } => {
                    // TODO: Track it!
                }
            },
            Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::Input { key_code, state } => {
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
                keyboard::Event::TextEntered { .. } => {}
            },
            Event::Gamepad { .. } => {
                // Ignore gamepad events...
            }
            Event::Window(_) => {
                // Ignore window events...
            }
        }
    }

    fn clear(&mut self) {
        self.left_clicks.clear();
        self.released_keys.clear();
    }
}
