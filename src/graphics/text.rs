use std::f32;

use crate::graphics::{Color, Point};

/// A section of text.
#[derive(Clone, PartialEq, Debug)]
pub struct Text {
    /// Text content.
    pub content: String,

    /// Text position.
    pub position: Point,

    /// Text bounds, in screen coordinates.
    pub bounds: (f32, f32),

    /// Text size.
    pub size: f32,

    /// Text color.
    pub color: Color,
}

impl Default for Text {
    fn default() -> Text {
        Text {
            content: String::from(""),
            position: Point::new(0.0, 0.0),
            bounds: (f32::INFINITY, f32::INFINITY),
            size: 16.0,
            color: Color::BLACK,
        }
    }
}
