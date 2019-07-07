use super::{keyboard, mouse, ButtonState, Event, Input};
use crate::graphics::Point;

use std::collections::{HashMap, HashSet};

/// A simple keyboard and mouse input tracker.
///
/// You can use this as your [`Game::Input`] directly!
///
/// [`Game::Input`]: ../trait.Game.html#associatedtype.Input
#[derive(Debug)]
pub struct KeyboardAndMouse {
    cursor_position: Point,
    mouse_wheel: Point,
    is_cursor_taken: bool,
    is_cursor_within_game_window: bool,
    button_clicks: HashMap<mouse::Button, Vec<Point>>,
    pressed_keys: HashSet<keyboard::KeyCode>,
    released_keys: HashSet<keyboard::KeyCode>,
    pressed_buttons: HashSet<mouse::Button>,
}

impl KeyboardAndMouse {
    /// Returns the current cursor position.
    pub fn cursor_position(&self) -> Point {
        self.cursor_position
    }

    /// Returns the mouse wheel movements during the last interaction.
    pub fn mouse_wheel(&self) -> Point {
        self.mouse_wheel
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

    /// Returns true if the cursor is currently within the game window.
    pub fn is_cursor_within_game_window(&self) -> bool {
        self.is_cursor_within_game_window
    }

    /// Returns true if the given key is currently pressed.
    pub fn is_key_pressed(&self, key_code: keyboard::KeyCode) -> bool {
        self.pressed_keys.contains(&key_code)
    }

    /// Returns true if the given key was released during the last interaction.
    pub fn was_key_released(&self, key_code: keyboard::KeyCode) -> bool {
        self.released_keys.contains(&key_code)
    }

    /// Returns true if the given button is currently pressed.
    pub fn is_button_pressed(&self, button: mouse::Button) -> bool {
        self.pressed_buttons.contains(&button)
    }

    /// Returns the positions of the mouse clicks during the last interaction.
    ///
    /// Clicks performed while the mouse cursor is not available are
    /// automatically ignored.
    pub fn button_clicks(&self, button: mouse::Button) -> &[Point] {
        self.button_clicks
            .get(&button)
            .map(|v| &v[..])
            .unwrap_or(&[])
    }
}

impl Input for KeyboardAndMouse {
    fn new() -> KeyboardAndMouse {
        KeyboardAndMouse {
            cursor_position: Point::new(0.0, 0.0),
            mouse_wheel: Point::new(0.0, 0.0),
            is_cursor_taken: false,
            is_cursor_within_game_window: false,
            button_clicks: HashMap::new(),
            pressed_keys: HashSet::new(),
            released_keys: HashSet::new(),
            pressed_buttons: HashSet::new(),
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
                mouse::Event::Input { state, button } => {
                    match state {
                        ButtonState::Pressed => {
                            if self.is_cursor_taken {
                                let _ = self.pressed_buttons.insert(button);
                            }
                        }
                        ButtonState::Released => {
                            if self.is_cursor_taken
                                && self.is_button_pressed(button)
                            {
                                self.button_clicks
                                    .entry(button)
                                    .or_insert_with(|| Vec::new())
                                    .push(self.cursor_position);
                            }

                            let _ = self.pressed_buttons.remove(&button);
                        }
                    };
                }
                mouse::Event::CursorEntered => {
                    self.is_cursor_within_game_window = true;
                }
                mouse::Event::CursorLeft => {
                    self.is_cursor_within_game_window = false;
                }
                mouse::Event::WheelScrolled { delta_x, delta_y } => {
                    self.mouse_wheel.x += delta_x;
                    self.mouse_wheel.y += delta_y;
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
        self.button_clicks.values_mut().for_each(|v| v.clear());
        self.released_keys.clear();
        self.mouse_wheel.x = 0.0;
        self.mouse_wheel.y = 0.0;
    }
}
