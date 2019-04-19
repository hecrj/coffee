use crate::graphics::{Point, Quad, Rectangle};

/// A portion of a texture in absolute coordinates.
///
/// Unlike a [`Region`], the coordinates of a [`Sprite`] are absolute. It can be
/// used as a convenient alternative.
pub struct Sprite {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Sprite {
    pub fn into_quad(self, x_unit: f32, y_unit: f32, position: Point) -> Quad {
        Quad {
            source: Rectangle {
                x: self.x as f32 * x_unit,
                y: self.y as f32 * y_unit,
                width: self.width as f32 * x_unit,
                height: self.height as f32 * y_unit,
            },
            position,
            size: (self.width as f32, self.height as f32),
        }
    }
}
