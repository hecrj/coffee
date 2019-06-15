use crate::graphics::point::Point;
use crate::graphics::rectangle::Rectangle;

/// A textured quad.
#[derive(Debug, PartialEq, Clone)]
pub struct Quad {
    /// The region of the resource that should be shown on the quad, in relative
    /// coordinates: [0.0, 1.0].
    pub source: Rectangle<f32>,

    /// The position where the quad should be drawn.
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

/// Turn a type into a quad.
///
/// Most methods accept generic types that can be turned into quads. This allows
/// you to use your own quad-based type.
pub trait IntoQuad {
    /// Turns the implementor into a quad.
    ///
    /// `x_unit` and `y_unit` are conversion factors for the [`source`] field.
    /// Use them to convert absolute resource coordinates into relative
    /// coordinates.
    ///
    /// [`source`]: struct.Quad.html#structfield.source
    fn into_quad(self, x_unit: f32, y_unit: f32) -> Quad;
}

impl IntoQuad for Quad {
    fn into_quad(self, _x_unit: f32, _y_unit: f32) -> Quad {
        self
    }
}
