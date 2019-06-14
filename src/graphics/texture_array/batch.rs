use super::{Index, TextureArray};
use crate::graphics::{gpu, IntoQuad, Point, Target, Transformation, Vector};

/// A collection of quads that can be drawn with a [`TextureArray`] all at once.
///
/// [`TextureArray`]: struct.TextureArray.html
#[derive(Debug)]
pub struct Batch {
    texture_array: TextureArray,
    instances: Vec<gpu::Quad>,
}

impl Batch {
    /// Create a new [`Batch`] from a [`TextureArray`].
    ///
    /// [`Batch`]: struct.Batch.html
    /// [`TextureArray`]: struct.TextureArray.html
    pub fn new(texture_array: TextureArray) -> Batch {
        Batch {
            texture_array,
            instances: Vec::new(),
        }
    }

    /// Add a quad to the [`Batch`] that will be rendered using the texture
    /// represented by the given [`Index`].
    ///
    /// [`Batch`]: struct.Batch.html
    /// [`Index`]: struct.Index.html
    #[inline]
    pub fn add<Q: IntoQuad>(&mut self, index: &Index, quad: Q) {
        let mut quad = quad
            .into_quad(self.texture_array.x_unit, self.texture_array.y_unit);

        quad.source.x += index.offset.x;
        quad.source.y += index.offset.y;

        let mut instance = gpu::Quad::from(quad);

        instance.layer = index.layer.into();

        self.instances.push(instance);
    }

    /// Draw the [`Batch`] at the given position.
    ///
    /// [`Batch`]: struct.Batch.html
    pub fn draw(&self, position: Point, target: &mut Target<'_>) {
        let mut translated = target.transform(Transformation::translate(
            Vector::new(position.x, position.y),
        ));

        translated.draw_texture_quads(
            &self.texture_array.texture,
            &self.instances[..],
        );
    }
}
