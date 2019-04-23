use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::graphics::gpu::{self, Texture};
use crate::graphics::{Color, Gpu, IntoQuad, Target};
use crate::load::Task;
use crate::Result;

/// A loaded image.
///
/// You can use this to load your spritesheets and draw your sprites!
///
/// Cloning an [`Image`] is cheap, it only clones a handle. It does not
/// create a new copy of the image on the GPU.
///
/// [`Image`]: struct.Image.html
#[derive(Clone)]
pub struct Image {
    pub(super) texture: Texture,
}

impl Image {
    /// Load an [`Image`] from the given path.
    ///
    /// [`Image`]: struct.Image.html
    pub fn new<P: AsRef<Path>>(gpu: &mut Gpu, path: P) -> Result<Image> {
        let image = {
            let mut buf = Vec::new();
            let mut reader = File::open(path)?;
            let _ = reader.read_to_end(&mut buf)?;
            image::load_from_memory(&buf)?
        };

        Image::from_image(gpu, image)
    }

    /// Create a [`Task`] that loads an [`Image`] from the given path.
    ///
    /// [`Task`]: ../load/struct.Task.html
    /// [`Image`]: struct.Image.html
    pub fn load<P: Into<PathBuf>>(path: P) -> Task<Image> {
        let p = path.into();

        Task::using_gpu(move |gpu| Image::new(gpu, &p))
    }

    /// Create an [`Image`] from a [`DynamicImage`] of the [`image` crate].
    ///
    /// [`Image`]: struct.Image.html
    /// [`DynamicImage`]: https://docs.rs/image/0.21.1/image/enum.DynamicImage.html
    /// [`image` crate]: https://docs.rs/image
    pub fn from_image(
        gpu: &mut Gpu,
        image: image::DynamicImage,
    ) -> Result<Image> {
        let texture = gpu.upload_texture(&image);

        Ok(Image { texture })
    }

    /// Create an [`Image`] representing a color palette.
    ///
    /// Each [`Color`] will be a pixel of the image, arranged horizontally.
    ///
    /// [`Image`]: struct.Image.html
    /// [`Color`]: struct.Color.html
    pub fn from_colors(gpu: &mut Gpu, colors: &[Color]) -> Result<Image> {
        let colors: Vec<[u8; 4]> =
            colors.iter().map(|color| color.to_rgba()).collect();

        Self::from_image(
            gpu,
            image::DynamicImage::ImageRgba8(
                image::RgbaImage::from_raw(
                    colors.len() as u32,
                    1,
                    colors.iter().flatten().cloned().collect(),
                )
                .unwrap(),
            ),
        )
    }

    /// Get the width of the [`Image`].
    ///
    /// [`Image`]: struct.Image.html
    pub fn width(&self) -> u16 {
        self.texture.width()
    }

    /// Get the height of the [`Image`].
    ///
    /// [`Image`]: struct.Image.html
    pub fn height(&self) -> u16 {
        self.texture.height()
    }

    /// Draw the [`Image`].
    ///
    /// [`Image`]: struct.Image.html
    #[inline]
    pub fn draw<Q: IntoQuad>(&self, quad: Q, target: &mut Target) {
        target.draw_texture_quads(
            &self.texture,
            &[gpu::Instance::from(quad.into_quad(
                1.0 / self.width() as f32,
                1.0 / self.height() as f32,
            ))],
        );
    }
}
