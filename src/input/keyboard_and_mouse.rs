use super::{ButtonState, Event, Input, KeyCode, MouseButton};
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
    points_clicked: Vec<Point>,
    pressed_keys: HashSet<KeyCode>,
    released_keys: HashSet<KeyCode>,
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
    pub fn clicks(&self) -> &[Point] {
        &self.points_clicked
    }

    /// Returns true if the given key is currently pressed.
    pub fn is_key_pressed(&self, key_code: KeyCode) -> bool {
        self.pressed_keys.contains(&key_code)
    }

    /// Returns true if the given key was released during the last interaction.
    pub fn was_key_released(&self, key_code: KeyCode) -> bool {
        self.released_keys.contains(&key_code)
    }
}

impl Input for KeyboardAndMouse {
    fn new() -> KeyboardAndMouse {
        KeyboardAndMouse {
            cursor_position: Point::new(0.0, 0.0),
            is_cursor_taken: false,
            is_mouse_pressed: false,
            points_clicked: Vec::new(),
            pressed_keys: HashSet::new(),
            released_keys: HashSet::new(),
        }
    }

    fn update(&mut self, event: Event) {
        match event {
            Event::CursorMoved { x, y } => {
                self.cursor_position = Point::new(x, y);
            }
            Event::CursorTaken => {
                self.is_cursor_taken = true;
            }
            Event::CursorReturned => {
                self.is_cursor_taken = false;
            }
            Event::MouseInput {
                button: MouseButton::Left,
                state,
            } => match state {
                ButtonState::Pressed => {
                    self.is_mouse_pressed = !self.is_cursor_taken;
                }
                ButtonState::Released => {
                    if !self.is_cursor_taken && self.is_mouse_pressed {
                        self.points_clicked.push(self.cursor_position);
                    }

                    self.is_mouse_pressed = false;
                }
            },
            Event::KeyboardInput { key_code, state } => {
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
            _ => {}
        }
    }

    fn clear(&mut self) {
        self.points_clicked.clear();
        self.released_keys.clear();
    }
}
