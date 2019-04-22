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
