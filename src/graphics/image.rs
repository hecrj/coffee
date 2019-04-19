use image;
use log::debug;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::graphics::gpu::{self, Texture};
use crate::graphics::{Color, Gpu, Quad, Target};
use crate::load;

#[derive(Clone)]
pub struct Image {
    pub(super) texture: Texture,
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

        Image::from_image(gpu, image)
    }

    pub fn load<P: Into<PathBuf>>(path: P) -> load::Task<Image> {
        let p = path.into();

        load::Task::using_gpu(move |gpu| Image::new(gpu, &p).unwrap())
    }

    pub fn from_image(
        gpu: &mut Gpu,
        image: image::DynamicImage,
    ) -> Option<Image> {
        let texture = gpu.upload_texture(&image);

        Some(Image { texture })
    }

    pub fn from_colors(gpu: &mut Gpu, colors: &[Color]) -> Option<Image> {
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

    pub fn width(&self) -> u16 {
        self.texture.width()
    }

    pub fn height(&self) -> u16 {
        self.texture.height()
    }

    #[inline]
    pub fn draw(&self, quad: Quad, target: &mut Target) {
        target.draw_texture_quads(
            &self.texture,
            &[gpu::Instance::from_quad(quad)],
        );
    }
}
