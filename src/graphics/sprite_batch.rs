use crate::graphics::gpu;
use crate::graphics::{
    Image, Point, Quad, Sprite, Target, Transformation, Vector,
};

pub struct SpriteBatch {
    image: Image,
    instances: Vec<gpu::Instance>,
    x_unit: f32,
    y_unit: f32,
}

impl SpriteBatch {
    pub fn new(image: Image) -> Self {
        let x_unit = 1.0 / image.width() as f32;
        let y_unit = 1.0 / image.height() as f32;

        Self {
            image,
            instances: Vec::new(),
            x_unit,
            y_unit,
        }
    }

    #[inline]
    pub fn add_quad(&mut self, quad: Quad) {
        let instance = gpu::Instance::from_quad(quad);

        self.instances.push(instance);
    }

    #[inline]
    pub fn add_sprite(&mut self, sprite: Sprite, position: Point) {
        let quad = sprite.into_quad(self.x_unit, self.y_unit, position);

        self.add_quad(quad)
    }

    pub fn draw(&self, position: Point, target: &mut Target) {
        let mut translated = target.transform(Transformation::translate(
            Vector::new(position.x, position.y),
        ));

        translated.draw_texture_quads(&self.image.texture, &self.instances[..]);
    }
}
