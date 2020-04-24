use crate::graphics::gpu::{self, texture, Gpu};
use crate::graphics::{IntoQuad, Target};
use crate::load::Task;
use crate::Result;

/// An off-screen rendering target.
///
/// It can be used both as a [`Target`] and as a resource.
///
/// [`Target`]: struct.Target.html
#[derive(Clone)]
pub struct Canvas {
    drawable: texture::Drawable,
}

impl Canvas {
    /// Creates a new [`Canvas`] with the given size.
    ///
    /// [`Canvas`]: struct.Canvas.html
    pub fn new(gpu: &mut Gpu, width: u16, height: u16) -> Result<Canvas> {
        Ok(Canvas {
            drawable: gpu.create_drawable_texture(width, height),
        })
    }

    /// Creates a [`Task`] that produces a new [`Canvas`] with the given size.
    ///
    /// [`Task`]: ../load/struct.Task.html
    /// [`Canvas`]: struct.Canvas.html
    pub fn load(width: u16, height: u16) -> Task<Canvas> {
        Task::using_gpu(move |gpu| Canvas::new(gpu, width, height))
    }

    /// Returns the width of the [`Canvas`].
    ///
    /// [`Canvas`]: struct.Canvas.html
    pub fn width(&self) -> u16 {
        self.drawable.texture().width()
    }

    /// Returns the height of the [`Canvas`].
    ///
    /// [`Canvas`]: struct.Canvas.html
    pub fn height(&self) -> u16 {
        self.drawable.texture().height()
    }

    /// Views the [`Canvas`] as a [`Target`].
    ///
    /// [`Canvas`]: struct.Canvas.html
    /// [`Target`]: struct.Target.html
    pub fn as_target<'a>(&'a mut self, gpu: &'a mut Gpu) -> Target<'a> {
        let texture = self.drawable.texture();

        Target::with_transformation(
            gpu,
            self.drawable.target(),
            f32::from(texture.width()),
            f32::from(texture.height()),
            texture::Drawable::render_transformation(),
        )
    }

    /// Renders the [`Canvas`] on the given [`Target`].
    ///
    /// [`Canvas`]: struct.Canvas.html
    /// [`Target`]: struct.Target.html
    pub fn draw<Q: IntoQuad>(&self, quad: Q, target: &mut Target<'_>) {
        target.draw_texture_quads(
            &self.drawable.texture(),
            &[gpu::Quad::from(quad.into_quad(
                1.0 / self.width() as f32,
                1.0 / self.height() as f32,
            ))],
        );
    }

    /// Reads the pixels of the [`Canvas`].
    ///
    /// _Note:_ This is a very slow operation.
    ///
    /// [`Canvas`]: struct.Canvas.html
    pub fn read_pixels(&self, gpu: &mut Gpu) -> image::DynamicImage {
        gpu.read_drawable_texture_pixels(&self.drawable)
    }
}

impl std::fmt::Debug for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Canvas {{ width: {}, height: {} }}",
            self.width(),
            self.height()
        )
    }
}
