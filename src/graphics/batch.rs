use rayon::prelude::*;

use crate::graphics::gpu;
use crate::graphics::{Image, IntoQuad, Target};

/// A collection of quads that will be drawn all at once using the same
/// [`Image`].
///
/// [`Image`]: struct.Image.html
pub struct Batch {
    image: Image,
    instances: Vec<gpu::Quad>,
    x_unit: f32,
    y_unit: f32,
}

impl Batch {
    /// Creates a new [`Batch`] using the given [`Image`].
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

    /// Adds a quad to the [`Batch`].
    ///
    /// [`Batch`]: struct.Batch.html
    #[inline]
    pub fn add<Q: IntoQuad>(&mut self, quad: Q) {
        let instance =
            gpu::Quad::from(quad.into_quad(self.x_unit, self.y_unit));

        self.instances.push(instance);
    }

    /// Draws the [`Batch`] on the given [`Target`].
    ///
    /// [`Batch`]: struct.Batch.html
    /// [`Target`]: struct.Target.html
    pub fn draw(&self, target: &mut Target<'_>) {
        target.draw_texture_quads(&self.image.texture, &self.instances[..]);
    }

    /// Clears the [`Batch`] contents.
    ///
    /// This is useful to avoid creating a new batch every frame and
    /// reallocating the same memory.
    ///
    /// [`Batch`]: struct.Batch.html
    pub fn clear(&mut self) {
        self.instances.clear();
    }
}

impl std::fmt::Debug for Batch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Batch {{ image: {:?} }}", self.image,)
    }
}

impl<Q: IntoQuad> Extend<Q> for Batch {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Q>,
    {
        let iter = iter.into_iter();
        let x_unit = self.x_unit;
        let y_unit = self.y_unit;

        self.instances.extend(
            iter.map(|quad| gpu::Quad::from(quad.into_quad(x_unit, y_unit))),
        );
    }
}

/// Extends the [`Batch`] using a parallel iterator from [`rayon`].
///
/// If you are dealing with many thousands of quads, `par_extend` can help you
/// speed up your drawing by using multiple threads to populate a [`Batch`].
///
/// [`Batch`]: struct.Batch.html
/// [`rayon`]: https://docs.rs/rayon/1.0/rayon/
impl<Q: IntoQuad + Send> ParallelExtend<Q> for Batch {
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = Q>,
    {
        let par_iter = par_iter.into_par_iter();
        let x_unit = self.x_unit;
        let y_unit = self.y_unit;

        self.instances.par_extend(
            par_iter
                .map(|quad| gpu::Quad::from(quad.into_quad(x_unit, y_unit))),
        );
    }
}
