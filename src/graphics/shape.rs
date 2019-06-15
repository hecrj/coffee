use crate::graphics::{Point, Rectangle};

/// A geometric figure.
#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    /// A rectangle
    Rectangle(Rectangle<f32>),

    /// A circle
    Circle {
        /// The center of the circle
        center: Point,

        /// The radius of the circle
        radius: f32,
    },

    /// An ellipse
    Ellipse {
        /// The center of the ellipse
        center: Point,

        /// The horizontal radius of the ellipse
        horizontal_radius: f32,

        /// The vertical radius of the ellipse
        vertical_radius: f32,

        /// The rotation of the ellipse in radians
        rotation: f32,
    },

    /// A polyline
    Polyline {
        /// The points of the polyline
        points: Vec<Point>,
    },
}
