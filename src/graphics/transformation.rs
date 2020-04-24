use nalgebra::Matrix3;
use std::ops::Mul;

use crate::graphics::Vector;

/// A 2D transformation matrix.
///
/// It can be used to apply a transformation to a [`Target`].
///
/// [`Target`]: struct.Target.html
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transformation(Matrix3<f32>);

impl Transformation {
    /// Get the identity transformation.
    pub fn identity() -> Transformation {
        Transformation(Matrix3::identity())
    }

    /// Creates an orthographic projection.
    ///
    /// You should rarely need this. On creation, a [`Target`] is automatically
    /// set up with the correct orthographic projection.
    ///
    /// [`Target`]: struct.Target.html
    #[rustfmt::skip]
    pub fn orthographic(width: f32, height: f32) -> Transformation {
        Transformation(nalgebra::Matrix3::new(
            2.0 / width, 0.0, -1.0,
            0.0, -2.0 / height, 1.0,
            0.0, 0.0, 1.0
        ))
    }

    /// Creates a translate transformation.
    ///
    /// You can use this to pan your camera, for example.
    pub fn translate(translation: Vector) -> Transformation {
        Transformation(Matrix3::new_translation(&Vector::new(
            translation.x,
            translation.y,
        )))
    }

    /// Creates a uniform scale transformation.
    ///
    /// You can use this to zoom your camera, for example.
    pub fn scale(scale: f32) -> Transformation {
        Transformation(Matrix3::new_scaling(scale))
    }

    /// Creates a non-uniform scale transformation.
    ///
    /// It allows you to scale each axis independently. You should rarely need
    /// this.
    pub fn nonuniform_scale(scale: Vector) -> Transformation {
        Transformation(Matrix3::new_nonuniform_scaling(&scale))
    }

    /// Creates a rotation transformation (in radians).
    ///
    /// You can use this to rotate your camera, for example.
    pub fn rotate(rotation: f32) -> Transformation {
        Transformation(Matrix3::new_rotation(rotation))
    }
}

impl Mul for Transformation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Transformation(self.0 * rhs.0)
    }
}

impl From<Transformation> for [[f32; 4]; 4] {
    #[rustfmt::skip]
    fn from(t: Transformation) -> [[f32; 4]; 4] {
        [
            [t.0[0], t.0[1], 0.0, t.0[2]],
            [t.0[3], t.0[4], 0.0, t.0[5]],
            [0.0, 0.0, -1.0, 0.0],
            [t.0[6], t.0[7], 0.0, t.0[8]],
        ]
    }
}

impl From<Transformation> for [f32; 16] {
    #[rustfmt::skip]
    fn from(t: Transformation) -> [f32; 16] {
        [
            t.0[0], t.0[1], 0.0, t.0[2],
            t.0[3], t.0[4], 0.0, t.0[5],
            0.0, 0.0, -1.0, 0.0,
            t.0[6], t.0[7], 0.0, t.0[8]
        ]
    }
}

impl From<Matrix3<f32>> for Transformation {
    fn from(matrix: Matrix3<f32>) -> Self {
        Transformation(matrix)
    }
}

impl Into<Matrix3<f32>> for Transformation {
    fn into(self) -> Matrix3<f32> {
        self.0
    }
}
