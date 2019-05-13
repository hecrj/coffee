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
    points_clicked: Vec<Point>,
    released_keys: HashSet<KeyCode>,
}

impl KeyboardAndMouse {
    pub fn cursor_position(&self) -> Point {
        self.cursor_position
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
            points_clicked: Vec::new(),
            released_keys: HashSet::new(),
        }
    }

    fn update(&mut self, event: Event) {
        match event {
            Event::CursorMoved { x, y } => {
                self.cursor_position = Point::new(x, y);
            }
            Event::MouseInput {
                button: MouseButton::Left,
                state: ButtonState::Released,
            } => {
                self.points_clicked.push(self.cursor_position);
            }
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
