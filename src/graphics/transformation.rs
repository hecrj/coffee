use nalgebra;
use std::ops::Mul;

use crate::graphics::vector::Vector;

/// A 2D transformation matrix.
///
/// It can be used to apply a transformation to a [`Target`].
///
/// [`Target`]: struct.Target.html
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transformation(nalgebra::Matrix4<f32>);

impl Transformation {
    /// Get the identity transformation.
    pub fn identity() -> Transformation {
        Transformation(nalgebra::Matrix4::identity())
    }

    /// Obtain an orthographic projection.
    ///
    /// You should rarely need this. On creation, a [`Target`] is automatically
    /// set up with the correct orthographic projection.
    ///
    /// [`Target`]: struct.Target.html
    pub fn orthographic(width: f32, height: f32) -> Transformation {
        Transformation(nalgebra::Matrix4::new_orthographic(
            0.0, width, 0.0, height, -1.0, 1.0,
        ))
    }

    /// Obtain a translate transformation.
    ///
    /// You can use this to pan your camera, for example.
    pub fn translate(translation: Vector) -> Transformation {
        Transformation(nalgebra::Matrix4::new_translation(
            &nalgebra::Vector3::new(translation.x, translation.y, 0.0),
        ))
    }

    /// Obtain a uniform scale transformation.
    ///
    /// You can use this to zoom your camera, for example.
    pub fn scale(scale: f32) -> Transformation {
        Transformation(nalgebra::Matrix4::new_scaling(scale))
    }

    /// Obtain a non-uniform scale transformation.
    ///
    /// It allows you to scale each axis independently. You should rarely need
    /// this.
    pub fn nonuniform_scale(x: f32, y: f32) -> Transformation {
        Transformation(nalgebra::Matrix4::new_nonuniform_scaling(
            &nalgebra::Vector3::new(x, y, 1.0),
        ))
    }

    /// Obtain a rotation transformation.
    ///
    /// You can use this to rotate your camera, for example.
    /// Note: Rotation is in radians.
    pub fn rotate(rotation: f32) -> Transformation {
        Transformation(nalgebra::Matrix4::new_rotation(nalgebra::Vector3::new(
            0.0, 0.0, rotation,
        )))
    }
}

impl Mul for Transformation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Transformation(self.0 * rhs.0)
    }
}

impl From<Transformation> for [[f32; 4]; 4] {
    fn from(transformation: Transformation) -> [[f32; 4]; 4] {
        transformation.0.into()
    }
}

impl From<Transformation> for [f32; 16] {
    fn from(t: Transformation) -> [f32; 16] {
        [
            t.0[0], t.0[1], t.0[2], t.0[3], t.0[4], t.0[5], t.0[6], t.0[7],
            t.0[8], t.0[9], t.0[10], t.0[11], t.0[12], t.0[13], t.0[14],
            t.0[15],
        ]
    }
}
