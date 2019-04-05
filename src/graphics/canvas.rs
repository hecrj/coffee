use crate::graphics::gpu::texture;
use crate::graphics::gpu::{self, Gpu, Target};
use crate::graphics::{DrawParameters, Vector};

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

        gpu::Target::new(
            gpu,
            self.drawable.target().clone(),
            texture.width() as f32,
            texture.height() as f32,
        )
    }

    pub fn draw(&self, mut parameters: DrawParameters, target: &mut Target) {
        let texture = self.drawable.texture();

        parameters.scale = Vector::new(
            parameters.scale.x * texture.width() as f32,
            parameters.scale.y * texture.height() as f32,
        );

        target.draw_texture(&texture, parameters);
    }
}
