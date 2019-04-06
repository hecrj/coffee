use nalgebra;
use std::ops::Mul;

use crate::graphics::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Transformation(nalgebra::Matrix4<f32>);

impl Transformation {
    pub fn identity() -> Transformation {
        Transformation(nalgebra::Matrix4::identity())
    }

    pub fn orthographic(width: f32, height: f32) -> Transformation {
        Transformation(nalgebra::Matrix4::new_orthographic(
            0.0, width, 0.0, height, -1.0, 1.0,
        ))
    }

    pub fn translate(translation: Vector) -> Transformation {
        Transformation(nalgebra::Matrix4::new_translation(
            &nalgebra::Vector3::new(translation.x, translation.y, 0.0),
        ))
    }

    pub fn scale(scale: f32) -> Transformation {
        Transformation(nalgebra::Matrix4::new_scaling(scale))
    }

    pub fn nonuniform_scale(x: f32, y: f32) -> Transformation {
        Transformation(nalgebra::Matrix4::new_nonuniform_scaling(
            &nalgebra::Vector3::new(x, y, 1.0),
        ))
    }
}

impl Mul for Transformation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Transformation(self.0 * rhs.0)
    }
}

impl Into<[[f32; 4]; 4]> for Transformation {
    fn into(self) -> [[f32; 4]; 4] {
        self.0.into()
    }
}
