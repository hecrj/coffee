use crate::graphics::point::Point;
use crate::graphics::rectangle::Rectangle;

/// A textured quad.
#[derive(Debug)]
pub struct Quad {
    /// The region of the resource that should be shown on the quad, in relative
    /// coordinates: [0.0, 1.0].
    pub source: Rectangle,

    /// The position of the quad.
    pub position: Point,

    /// The size of the quad.
    pub size: (f32, f32),
}

impl Default for Quad {
    fn default() -> Self {
        Self {
            source: Rectangle {
                x: 0.0,
                y: 0.0,
                width: 1.0,
                height: 1.0,
            },
            position: Point::new(0.0, 0.0),
            size: (1.0, 1.0),
        }
    }
}
