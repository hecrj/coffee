use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use log::debug;

use crate::graphics::gpu::{Instance, Texture};
use crate::graphics::{
    Gpu, Point, Quad, Sprite, Target, Transformation, Vector,
};
use crate::load;

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

    /// Add a quad to the batch
    #[inline]
    pub fn add_quad(&mut self, index: &Index, mut quad: Quad) {
        quad.source.x += index.offset.x;
        quad.source.y += index.offset.y;

        let mut instance = Instance::from_quad(quad);

        instance.layer = index.layer.into();

        self.instances.push(instance);
    }

    /// Add a sprite to the batch
    #[inline]
    pub fn add_sprite(
        &mut self,
        index: &Index,
        sprite: Sprite,
        position: Point,
    ) {
        let quad = sprite.into_quad(
            self.texture_array.x_unit,
            self.texture_array.y_unit,
            position,
        );

        self.add_quad(index, quad)
    }

    /// Draw the batch
    pub fn draw(&self, position: Point, target: &mut Target) {
        let mut translated = target.transform(Transformation::translate(
            Vector::new(position.x, position.y),
        ));

        translated.draw_texture_quads(
            &self.texture_array.texture,
            &self.instances[..],
        );
    }
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

/// An index that identifies a texture in a texture array.
#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
struct Offset {
    x: f32,
    y: f32,
}

pub struct Loader {
    width: u16,
    height: u16,
    paths: Vec<PathBuf>,
}

#[derive(Clone, Copy)]
pub struct LazyIndex(usize);

pub struct LoadedIndices(Vec<Index>);

impl LoadedIndices {
    pub fn get(&self, key: LazyIndex) -> Index {
        self.0[key.0]
    }
}

impl Loader {
    pub fn new(width: u16, height: u16) -> Loader {
        Loader {
            width,
            height,
            paths: Vec::new(),
        }
    }

    pub fn add<P: Into<PathBuf>>(&mut self, path: P) -> LazyIndex {
        self.paths.push(path.into());
        LazyIndex(self.paths.len() - 1)
    }

    pub fn finish<F, R>(self, on_completion: F) -> load::Task<R>
    where
        F: 'static + Fn(TextureArray, LoadedIndices) -> R,
    {
        let total_work = self.paths.len() as u32 + 1;

        load::Task::sequence(total_work, move |task| {
            let mut builder = Builder::new(self.width, self.height);
            let mut work_todo = VecDeque::from(self.paths.clone());
            let mut indices = Vec::new();

            while let Some(next) = work_todo.pop_front() {
                indices.push(builder.add(next).unwrap());

                task.notify_progress(1);
            }

            let result = on_completion(
                builder.build(task.gpu()),
                LoadedIndices(indices),
            );

            task.notify_progress(1);

            result
        })
    }
}
