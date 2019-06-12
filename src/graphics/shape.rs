use crate::graphics::Point;

/// A geometric figure.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    /// A circle.
    Circle {
        /// The center of the circle.
        center: Point,

        /// The radius of the circle.
        radius: f32,
    },
}
