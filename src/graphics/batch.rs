use crate::graphics::gpu;
use crate::graphics::{Image, IntoQuad, Point, Target, Transformation, Vector};

/// A collection of quads that will be drawn all at once using the same
/// [`Image`].
///
/// [`Image`]: struct.Image.html
pub struct Batch {
    image: Image,
    instances: Vec<gpu::Instance>,
    x_unit: f32,
    y_unit: f32,
}

impl Batch {
    /// Create a new [`Batch`] using the given [`Image`].
    ///
    /// [`Batch`]: struct.Batch.html
    /// [`Image`]: struct.Image.html
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

    /// Add a quad to the [`Batch`].
    ///
    /// [`Batch`]: struct.Batch.html
    #[inline]
    pub fn add<Q: IntoQuad>(&mut self, quad: Q) {
        let instance =
            gpu::Instance::from_quad(quad.into_quad(self.x_unit, self.y_unit));

        self.instances.push(instance);
    }

    /// Draw the [`Batch`] at the given position.
    ///
    /// [`Batch`]: struct.Batch.html
    pub fn draw(&self, position: Point, target: &mut Target) {
        let mut translated = target.transform(Transformation::translate(
            Vector::new(position.x, position.y),
        ));

        translated.draw_texture_quads(&self.image.texture, &self.instances[..]);
    }
}
