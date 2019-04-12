use crate::graphics::gpu::{Gpu, Instance, TargetView, Texture};
use crate::graphics::{Color, Transformation};

pub struct Target<'a> {
    gpu: &'a mut Gpu,
    view: TargetView,
    transformation: Transformation,
}

impl<'a> Target<'a> {
    pub(super) fn new(
        gpu: &mut Gpu,
        view: TargetView,
        width: f32,
        height: f32,
    ) -> Target {
        Target {
            gpu,
            view,
            transformation: Transformation::orthographic(width, height),
        }
    }

    pub(super) fn with_transformation(
        gpu: &mut Gpu,
        view: TargetView,
        width: f32,
        height: f32,
        transformation: Transformation,
    ) -> Target {
        let mut target = Self::new(gpu, view, width, height);
        target.transformation = transformation * target.transformation;
        target
    }

    pub fn transform(&mut self, new_transformation: Transformation) -> Target {
        Target {
            gpu: self.gpu,
            view: self.view.clone(),
            transformation: self.transformation * new_transformation,
        }
    }

    pub fn clear(&mut self, color: Color) {
        self.gpu.clear(&self.view, color);
    }

    pub(super) fn draw_texture_quads(
        &mut self,
        texture: &Texture,
        instances: &[Instance],
    ) {
        self.gpu.draw_texture_quads(
            texture,
            instances,
            &self.view,
            &self.transformation,
        );
    }
}
