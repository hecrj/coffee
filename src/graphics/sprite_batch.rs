use crate::graphics::gpu;
use crate::graphics::{
    DrawParameters, Image, Point, Rectangle, Target, Transformation, Vector,
};

pub struct SpriteBatch {
    image: Image,
    sprites: Vec<gpu::Instance>,
    x_unit: f32,
    y_unit: f32,
}

impl SpriteBatch {
    pub fn new(image: Image) -> Self {
        let x_unit = 1.0 / image.width() as f32;
        let y_unit = 1.0 / image.height() as f32;

        Self {
            image,
            sprites: Vec::new(),
            x_unit,
            y_unit,
        }
    }

    pub fn add(&mut self, sprite: Sprite, position: Point) {
        let instance = gpu::Instance::from_parameters(DrawParameters {
            source: Rectangle {
                x: (sprite.column * sprite.width) as f32 * self.x_unit,
                y: (sprite.row * sprite.height) as f32 * self.y_unit,
                width: sprite.width as f32 * self.x_unit,
                height: sprite.height as f32 * self.y_unit,
            },
            position,
            scale: Vector::new(sprite.width as f32, sprite.height as f32),
        });

        self.sprites.push(instance);
    }

    pub fn draw(&self, position: Point, target: &mut Target) {
        let mut translated = target.transform(Transformation::translate(
            Vector::new(position.x, position.y),
        ));

        translated.draw_texture_quads(&self.image.texture(), &self.sprites[..]);
    }
}

/// Represents a sprite
#[derive(Debug)]
pub struct Sprite {
    /// Sprite row
    pub row: u32,
    /// Sprite column
    pub column: u32,
    /// Sprite width
    pub width: u32,
    /// Sprite height
    pub height: u32,
}
