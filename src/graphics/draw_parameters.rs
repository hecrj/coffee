use crate::graphics::point::Point;
use crate::graphics::rectangle::Rectangle;
use crate::graphics::vector::Vector;

#[derive(Debug)]
pub struct DrawParameters {
    pub source: Rectangle,
    pub position: Point,
    pub scale: Vector,
}

impl Default for DrawParameters {
    fn default() -> Self {
        Self {
            source: Rectangle {
                x: 0.0,
                y: 0.0,
                width: 1.0,
                height: 1.0,
            },
            position: Point::new(0.0, 0.0),
            scale: Vector::new(1.0, 1.0),
        }
    }
}
