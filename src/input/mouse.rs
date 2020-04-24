//! Listen to mouse events.

mod event;
mod wheel_movement;

pub use crate::graphics::window::winit::event::MouseButton as Button;
pub use event::Event;
pub use wheel_movement::WheelMovement;

use super::{ButtonState, Event as InputEvent, Input};
use crate::graphics::Point;

use std::collections::{HashMap, HashSet};

/// A simple mouse input tracker.
///
/// You can use this as your [`Game::Input`] directly!
///
/// [`Game::Input`]: ../trait.Game.html#associatedtype.Input
#[derive(Debug, Clone)]
pub struct Mouse {
    cursor_position: Point,
    wheel_movement: WheelMovement,
    is_cursor_taken: bool,
    is_cursor_within_window: bool,
    button_clicks: HashMap<Button, Vec<Point>>,
    pressed_buttons: HashSet<Button>,
}

impl Mouse {
    /// Returns the current cursor position.
    pub fn cursor_position(&self) -> Point {
        self.cursor_position
    }

    /// Returns the wheel movements during the last interaction.
    pub fn wheel_movement(&self) -> WheelMovement {
        self.wheel_movement
    }

    /// Returns true if the cursor is currently not available.
    ///
    /// This mostly happens when the cursor is currently over a
    /// [`UserInterface`].
    ///
    /// [`UserInterface`]: ../../ui/trait.UserInterface.html
    pub fn is_cursor_taken(&self) -> bool {
        self.is_cursor_taken
    }

    /// Returns true if the cursor is currently within the [`Window`].
    ///
    /// [`Window`]: ../../graphics/struct.Window.html
    pub fn is_cursor_within_window(&self) -> bool {
        self.is_cursor_within_window
    }

    /// Returns true if the given button is currently pressed.
    pub fn is_button_pressed(&self, button: Button) -> bool {
        self.pressed_buttons.contains(&button)
    }

    /// Returns the positions of the clicks during the last interaction.
    ///
    /// Clicks performed while the cursor is not available are automatically
    /// ignored.
    pub fn button_clicks(&self, button: Button) -> &[Point] {
        self.button_clicks
            .get(&button)
            .map(|v| &v[..])
            .unwrap_or(&[])
    }
}

impl Input for Mouse {
    fn new() -> Mouse {
        Mouse {
            cursor_position: Point::new(0.0, 0.0),
            wheel_movement: WheelMovement::new(0.0, 0.0),
            is_cursor_taken: false,
            is_cursor_within_window: false,
            button_clicks: HashMap::new(),
            pressed_buttons: HashSet::new(),
        }
    }

    fn update(&mut self, event: InputEvent) {
        match event {
            InputEvent::Mouse(mouse_event) => match mouse_event {
                Event::CursorMoved { x, y } => {
                    self.cursor_position = Point::new(x, y);
                }
                Event::CursorTaken => {
                    self.is_cursor_taken = true;
                }
                Event::CursorReturned => {
                    self.is_cursor_taken = false;
                }
                Event::Input { state, button } => {
                    match state {
                        ButtonState::Pressed => {
                            if !self.is_cursor_taken {
                                let _ = self.pressed_buttons.insert(button);
                            }
                        }
                        ButtonState::Released => {
                            if !self.is_cursor_taken
                                && self.is_button_pressed(button)
                            {
                                self.button_clicks
                                    .entry(button)
                                    .or_insert_with(Vec::new)
                                    .push(self.cursor_position);
                            }

                            let _ = self.pressed_buttons.remove(&button);
                        }
                    };
                }
                Event::CursorEntered => {
                    self.is_cursor_within_window = true;
                }
                Event::CursorLeft => {
                    self.is_cursor_within_window = false;
                }
                Event::WheelScrolled { delta_x, delta_y } => {
                    self.wheel_movement.horizontal += delta_x;
                    self.wheel_movement.vertical += delta_y;
                }
            },
            InputEvent::Keyboard { .. } => {
                // Ignore keyboard events...
            }
            InputEvent::Gamepad { .. } => {
                // Ignore gamepad events...
            }
            InputEvent::Window { .. } => {
                // Ignore window events...
            }
        }
    }

    fn clear(&mut self) {
        self.button_clicks.values_mut().for_each(Vec::clear);
        self.wheel_movement.horizontal = 0.0;
        self.wheel_movement.vertical = 0.0;
    }
}
