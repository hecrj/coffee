use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::{Index, Offset, TextureArray};
use crate::graphics::Gpu;
use crate::{Error, Result};

/// A [`TextureArray`] builder.
///
/// [`TextureArray`]: struct.TextureArray.html
#[derive(Debug)]
pub struct Builder {
    width: u32,
    height: u32,
    layers: Vec<Layer>,
    current: Layer,
}

impl Builder {
    /// Creates a new [`Builder`] of a [`TextureArray`] of the given size.
    ///
    /// [`Builder`]: struct.Builder.html
    /// [`TextureArray`]: struct.TextureArray.html
    pub fn new(width: u16, height: u16) -> Builder {
        Builder {
            width: width as u32,
            height: height as u32,
            layers: Vec::new(),
            current: Layer::new(width, height),
        }
    }

    /// Loads a new image from the given path and adds it to the produced
    /// [`TextureArray`].
    ///
    /// The [`Builder`] will try to fit multiple images in the same layer of the
    /// array. For example, if you are building a texture array of `2048x2048`
    /// pixels and you add 4 images of `1024x1024` pixels, they will all share
    /// the same layer.
    ///
    /// As of now, the [`Builder`] uses a very naive placement algorithm. It
    /// simply places images in rows as they are added if there is any space left
    /// in the current layer. If there is not, it creates a new layer.
    ///
    /// [`TextureArray`]: struct.TextureArray.html
    /// [`Builder`]: struct.Builder.html
    pub fn add<P: AsRef<Path>>(&mut self, path: P) -> Result<Index> {
        let img = {
            let mut buf = Vec::new();
            let mut reader = File::open(&path)?;
            let _ = reader.read_to_end(&mut buf)?;
            let rgba = image::load_from_memory(&buf)?.to_rgba();
            Arc::new(rgba)
        };

        if img.width() > self.width || img.height() > self.height {
            Err(Error::TextureArray(super::Error::ImageIsTooBig(
                PathBuf::from(path.as_ref()),
            )))
        } else {
            let offset = self.current.add(img.clone());

            match offset {
                Some(offset) => Ok(Index {
                    layer: self.layers.len() as u16,
                    offset,
                }),
                None => {
                    self.layers.push(self.current.clone());
                    self.current =
                        Layer::new(self.width as u16, self.height as u16);

                    Ok(Index {
                        layer: self.layers.len() as u16,
                        offset: self
                            .current
                            .add(img)
                            .expect("Image should fit layer"),
                    })
                }
            }
        }
    }

    /// Builds the [`TextureArray`].
    ///
    /// [`TextureArray`]: struct.TextureArray.html
    pub fn build(&mut self, gpu: &mut Gpu) -> TextureArray {
        if !self.current.is_empty() {
            self.layers.push(self.current.clone());
            self.current = Layer::new(0, 0);
        }

        let images: Vec<image::DynamicImage> = self
            .layers
            .iter()
            .map(|layer| {
                image::DynamicImage::ImageRgba8(layer.to_owned().to_rgba())
            })
            .collect();

        let texture = gpu.upload_texture_array(&images[..]);

        TextureArray {
            texture,
            x_unit: 1.0 / self.width as f32,
            y_unit: 1.0 / self.height as f32,
        }
    }
}

#[derive(Debug, Clone)]
struct Layer {
    images: Vec<Vec<Arc<image::RgbaImage>>>,
    current_row: Vec<Arc<image::RgbaImage>>,
    max_width: u32,
    max_height: u32,
}

impl Layer {
    fn new(max_width: u16, max_height: u16) -> Layer {
        Layer {
            images: Vec::new(),
            current_row: Vec::new(),
            max_width: max_width as u32,
            max_height: max_height as u32,
        }
    }

    fn current_height(&self) -> u32 {
        self.images
            .iter()
            .map(|row| row.iter().map(|i| i.height()).max().unwrap_or(0))
            .sum()
    }

    fn is_empty(&self) -> bool {
        self.images.is_empty() && self.current_row.is_empty()
    }

    fn add(&mut self, image: Arc<image::RgbaImage>) -> Option<Offset> {
        let current_row_width: u32 =
            self.current_row.iter().map(|i| i.width()).sum();

        if current_row_width + image.width() <= self.max_width {
            if self.current_height() + image.height() <= self.max_height {
                self.current_row.push(image);

                Some(Offset {
                    x: current_row_width as f32 / self.max_width as f32,
                    y: self.current_height() as f32 / self.max_height as f32,
                })
            } else {
                None
            }
        } else {
            let current_row_height = self
                .current_row
                .iter()
                .map(|i| i.height())
                .max()
                .unwrap_or(0);

            if self.current_height() + current_row_height + image.height()
                <= self.max_height
            {
                self.images.push(self.current_row.clone());
                self.current_row = vec![image];

                Some(Offset {
                    x: 0.0,
                    y: self.current_height() as f32 / self.max_height as f32,
                })
            } else {
                None
            }
        }
    }

    fn to_rgba(mut self) -> image::RgbaImage {
        let mut values = Vec::new();
        values.resize((self.max_width * self.max_height * 4) as usize, 0 as u8);

        let mut texture = image::ImageBuffer::from_raw(
            self.max_width,
            self.max_height,
            values,
        )
        .expect("Image buffer creation");

        if !self.current_row.is_empty() {
            self.images.push(self.current_row.clone());
            self.current_row = Vec::new();
        }

        let mut y = 0;

        for row in self.images {
            let mut x = 0;
            let mut row_height = 0;

            for image in row {
                image::imageops::overlay(&mut texture, &image, x, y);

                x += image.width();
                row_height = row_height.max(image.height());
            }

            y += row_height;
        }

        texture
    }
}
