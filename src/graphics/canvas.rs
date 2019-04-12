use crate::graphics::gpu::{self, texture, Gpu};
use crate::graphics::{DrawParameters, Target};

pub struct Canvas {
    drawable: texture::Drawable,
}

impl Canvas {
    pub fn new(gpu: &mut Gpu, width: u16, height: u16) -> Canvas {
        Canvas {
            drawable: gpu.create_drawable_texture(width, height),
        }
    }

    pub fn as_target<'a>(&mut self, gpu: &'a mut Gpu) -> Target<'a> {
        let texture = self.drawable.texture();

        Target::with_transformation(
            gpu,
            self.drawable.target().clone(),
            texture.width() as f32,
            texture.height() as f32,
            texture::Drawable::render_transformation(),
        )
    }

    pub fn draw(&self, parameters: DrawParameters, target: &mut Target) {
        target.draw_texture_quads(
            &self.drawable.texture(),
            &[gpu::Instance::from_parameters(parameters)],
        );
    }
}
