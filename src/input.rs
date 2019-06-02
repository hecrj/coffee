//! Allow players to interact with your game.

mod event;

pub use event::{ButtonState, Event, KeyCode, MouseButton};

use std::collections::HashSet;

use crate::graphics::Point;

pub trait Input {
    fn new() -> Self;

    /// Process an input event and keep track of it in your [`Input`] type.
    ///
    /// This function may be called multiple times during event processing,
    /// before [`Game::interact`].
    ///
    /// [`Input`]: #associatedtype.Input
    /// [`interact`]: #method.interact
    fn update(&mut self, event: Event);

    fn clear(&mut self);
}

impl Input for () {
    fn new() -> () {
        ()
    }

    fn update(&mut self, _event: Event) {}

    fn clear(&mut self) {}
}

pub trait HasCursorPosition {
    fn cursor_position(&self) -> Point;
}

pub struct KeyboardAndMouse {
    cursor_position: Point,
    is_cursor_taken: bool,
    is_mouse_pressed: bool,
    points_clicked: Vec<Point>,
    released_keys: HashSet<KeyCode>,
}

impl KeyboardAndMouse {
    pub fn cursor_position(&self) -> Point {
        self.cursor_position
    }

    pub fn is_cursor_taken(&self) -> bool {
        self.is_cursor_taken
    }

    pub fn clicks(&self) -> &Vec<Point> {
        &self.points_clicked
    }

    pub fn was_key_released(&self, key_code: &KeyCode) -> bool {
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
            Event::KeyboardInput {
                key_code,
                state: ButtonState::Released,
            } => {
                let _ = self.released_keys.insert(key_code);
            }
            _ => {}
        }
    }

    fn clear(&mut self) {
        self.points_clicked.clear();
        self.released_keys.clear();
    }
}

impl HasCursorPosition for KeyboardAndMouse {
    fn cursor_position(&self) -> Point {
        self.cursor_position
    }
}
