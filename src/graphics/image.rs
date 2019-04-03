use image;
use log::debug;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::graphics::gpu::{Gpu, Target, Texture};
use crate::graphics::DrawParameters;
use crate::graphics::Vector;

pub struct Image {
    texture: Texture,
}

impl Image {
    pub fn new<P: AsRef<Path>>(gpu: &mut Gpu, path: P) -> Option<Image> {
        let resource_path = Path::new("resources")
            .join(path.as_ref().strip_prefix("/").unwrap());

        debug!("Loading image: {:?}", resource_path);

        let image = {
            let mut buf = Vec::new();
            let mut reader = File::open(resource_path).unwrap();
            let _ = reader.read_to_end(&mut buf).unwrap();
            image::load_from_memory(&buf).unwrap()
        };

        Some(Image {
            texture: gpu.upload_image(&image),
        })
    }

    pub fn width(&self) -> u16 {
        self.texture.width()
    }

    pub fn height(&self) -> u16 {
        self.texture.height()
    }

    pub fn draw(&self, mut parameters: DrawParameters, target: &mut Target) {
        parameters.scale = Vector::new(
            parameters.scale.x * self.width() as f32,
            parameters.scale.y * self.height() as f32,
        );

        target.draw_texture(&self.texture, parameters);
    }
}
