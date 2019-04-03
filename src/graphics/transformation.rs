use nalgebra;
use std::ops::Mul;

use crate::graphics::vector::Vector;

#[derive(Clone, Copy)]
pub struct Transformation(nalgebra::Matrix3<f32>);

impl Transformation {
    pub fn identity() -> Transformation {
        Transformation(nalgebra::Matrix3::identity())
    }

    pub fn translate(translation: Vector) -> Transformation {
        Transformation(nalgebra::Matrix3::new_translation(&translation))
    }

    pub fn scale(scale: f32) -> Transformation {
        Transformation(nalgebra::Matrix3::new_scaling(scale))
    }
}

impl Mul for Transformation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Transformation(self.0 * rhs.0)
    }
}
