use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use log::debug;

use crate::graphics;
use crate::graphics::gpu::{Instance, Texture};
use crate::graphics::transformation::Transformation;
use crate::loader;

/// A texture array
#[derive(Debug, Clone)]
pub struct TextureArray {
    texture: Texture,
    x_unit: f32,
    y_unit: f32,
}

impl TextureArray {
    /// Obtain a batch for the texture array
    pub fn batch(&self) -> Batch {
        Batch::new(self.clone())
    }
}

/// Represents a batch of sprites that can be drawn with a texture array all at once.
#[derive(Debug)]
pub struct Batch {
    texture_array: TextureArray,
    instances: Vec<Instance>,
}

impl Batch {
    fn new(texture_array: TextureArray) -> Batch {
        Batch {
            texture_array,
            instances: Vec::new(),
        }
    }

    /// Add a sprite to the batch
    pub fn add(
        &mut self,
        index: &Index,
        sprite: Sprite,
        position: graphics::Point,
    ) {
        let mut instance =
            Instance::from_parameters(graphics::DrawParameters {
                source: graphics::Rectangle {
                    x: (index.offset.x + sprite.column * sprite.width) as f32
                        * self.texture_array.x_unit,
                    y: (index.offset.y + sprite.row * sprite.height) as f32
                        * self.texture_array.y_unit,
                    width: sprite.width as f32 * self.texture_array.x_unit,
                    height: sprite.height as f32 * self.texture_array.y_unit,
                },
                position,
                scale: graphics::Vector::new(
                    sprite.width as f32,
                    sprite.height as f32,
                ),
            });

        instance.layer = index.layer.into();

        self.instances.push(instance);
    }

    /// Draw the batch
    pub fn draw(
        &self,
        position: graphics::Point,
        target: &mut graphics::Target,
    ) {
        let mut translated = target.transform(Transformation::translate(
            graphics::Vector::new(position.x, position.y),
        ));

        translated.draw_texture_quads(
            &self.texture_array.texture,
            &self.instances[..],
        );
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

/// A texture array builder
#[derive(Debug)]
pub struct Builder {
    width: u32,
    height: u32,
    layers: Vec<Layer>,
    current: Layer,
}

impl Builder {
    /// Create a new texture array builder
    pub fn new(width: u16, height: u16) -> Builder {
        Builder {
            width: width as u32,
            height: height as u32,
            layers: Vec::new(),
            current: Layer::new(width, height),
        }
    }

    /// Add a new texture.
    pub fn add<P: AsRef<Path>>(&mut self, path: P) -> Option<Index> {
        let resource_path = Path::new("resources")
            .join(path.as_ref().strip_prefix("/").unwrap());

        debug!("Loading image: {:?}", resource_path);

        let img = {
            let mut buf = Vec::new();
            let mut reader = File::open(resource_path).unwrap();
            reader.read_to_end(&mut buf).unwrap();
            let rgba = image::load_from_memory(&buf).unwrap().to_rgba();
            Arc::new(rgba)
        };

        if img.width() > self.width || img.height() > self.height {
            None
        //Err(GameError::ResourceLoadError(String::from(
        //    "Image is too big",
        //)))
        } else {
            let offset = self.current.add(img.clone());

            match offset {
                Some(offset) => Some(Index {
                    layer: self.layers.len() as u16,
                    offset,
                }),
                None => {
                    self.layers.push(self.current.clone());
                    self.current =
                        Layer::new(self.width as u16, self.height as u16);

                    Some(Index {
                        layer: self.layers.len() as u16,
                        offset: self.current.add(img).unwrap(),
                    })
                }
            }
        }
    }

    /// Build the texture array
    pub fn build(mut self, gpu: &mut graphics::Gpu) -> TextureArray {
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

/// An index that identifies a texture in a texture array.
#[derive(Debug)]
pub struct Index {
    layer: u16,
    offset: Offset,
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
        let current_row_width =
            self.current_row.iter().map(|i| i.width()).sum();

        if current_row_width + image.width() <= self.max_width {
            if self.current_height() + image.height() <= self.max_height {
                self.current_row.push(image);

                Some(Offset {
                    x: current_row_width,
                    y: self.current_height(),
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
                    x: 0,
                    y: self.current_height(),
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
        .unwrap();

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

#[derive(Debug)]
struct Offset {
    x: u32,
    y: u32,
}

pub struct Loader<T> {
    width: u16,
    height: u16,
    paths: Vec<PathBuf>,
    on_completion: Box<FnOnce(TextureArray, LoadedIndices) -> Result<T, ()>>,
}

#[derive(Clone, Copy)]
pub struct LazyIndex(u16);

pub struct LoadedIndices {}

impl LoadedIndices {
    pub fn get(&self, key: LazyIndex) -> Result<Index, ()> {
        Ok(Index {
            layer: 0,
            offset: Offset { x: 0, y: 0 },
        })
    }
}

impl Loader<()> {
    pub fn new(width: u16, height: u16) -> Loader<()> {
        Loader {
            width,
            height,
            paths: Vec::new(),
            on_completion: Box::new(|_, _| Ok(())),
        }
    }

    pub fn add<P: Into<PathBuf>>(&mut self, path: P) -> LazyIndex {
        self.paths.push(path.into());
        LazyIndex(self.paths.len() as u16 - 1)
    }

    pub fn finish<F, R>(self, on_completion: F) -> Loader<R>
    where
        F: 'static + FnOnce(TextureArray, LoadedIndices) -> Result<R, ()>,
    {
        Loader {
            width: self.width,
            height: self.height,
            paths: self.paths,
            on_completion: Box::new(on_completion),
        }
    }
}

impl<T> loader::Loader<T> for Loader<T> {
    fn total_work(&self) -> u32 {
        self.paths.len() as u32 + 1
    }

    fn load(&mut self, gpu: &mut graphics::Gpu) -> loader::Progress<T> {
        loader::Progress::Loading { work_completed: 0 }
    }
}
