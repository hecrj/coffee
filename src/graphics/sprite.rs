use crate::graphics::{IntoQuad, Point, Quad, Rectangle};

/// A quad describing the portion of a resource in absolute coordinates.
///
/// Unlike a [`Quad`], the `source` coordinates of a [`Sprite`] are absolute. It
/// can be used as a convenient alternative.
///
/// [`Quad`]: struct.Quad.html
/// [`Sprite`]: struct.Sprite.html
#[derive(Debug, PartialEq, Clone)]
pub struct Sprite {
    /// The portion of a resource that contains the sprite, in absolute
    /// coordinates.
    pub source: Rectangle<u16>,

    /// The position where the sprite should be drawn.
    pub position: Point,

    /// The scale to apply to the sprite.
    pub scale: (f32, f32),
}

impl Default for Sprite {
    #[inline]
    fn default() -> Sprite {
        Sprite {
            source: Rectangle {
                x: 0,
                y: 0,
                width: 1,
                height: 1,
            },
            position: Point::new(0.0, 0.0),
            scale: (1.0, 1.0),
        }
    }
}

impl IntoQuad for Sprite {
    fn into_quad(self, x_unit: f32, y_unit: f32) -> Quad {
        Quad {
            source: Rectangle {
                x: self.source.x as f32 * x_unit,
                y: self.source.y as f32 * y_unit,
                width: self.source.width as f32 * x_unit,
                height: self.source.height as f32 * y_unit,
            },
            position: self.position,
            size: (
                self.source.width as f32 * self.scale.0,
                self.source.height as f32 * self.scale.1,
            ),
        }
    }
}
