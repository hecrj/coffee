use std::f32;

use crate::graphics::{Color, Point};

/// A section of text.
#[derive(Clone, PartialEq, Debug)]
pub struct Text<'a> {
    /// Text content.
    pub content: &'a str,

    /// Text position.
    pub position: Point,

    /// Text bounds, in screen coordinates.
    pub bounds: (f32, f32),

    /// Text size.
    pub size: f32,

    /// Text color.
    pub color: Color,
}

impl Default for Text<'static> {
    fn default() -> Text<'static> {
        Text {
            content: "",
            position: Point::new(0.0, 0.0),
            bounds: (f32::INFINITY, f32::INFINITY),
            size: 16.0,
            color: Color::BLACK,
        }
    }
}
