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

    pub horizontal_alignment: HorizontalAlignment,

    pub vertical_alignment: VerticalAlignment,
}

impl Default for Text<'static> {
    #[inline]
    fn default() -> Text<'static> {
        Text {
            content: "",
            position: Point::new(0.0, 0.0),
            bounds: (f32::INFINITY, f32::INFINITY),
            size: 16.0,
            color: Color::BLACK,
            horizontal_alignment: HorizontalAlignment::Left,
            vertical_alignment: VerticalAlignment::Top,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}
