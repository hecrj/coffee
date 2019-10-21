use coffee::graphics::{Canvas, Gpu, Image};

mod mesh;

use mesh::Mesh;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Test {
    Mesh,
}

impl Test {
    pub fn all() -> Vec<Test> {
        vec![Test::Mesh]
    }

    pub fn run(&self, gpu: &mut Gpu) -> Drawing {
        let draw = match self {
            Test::Mesh => Mesh::draw(),
        };

        Drawing {
            test: *self,
            canvas: draw
                .run(gpu)
                .expect(&format!("Run test \"{}\"", self.to_string())),
        }
    }
}

impl std::string::ToString for Test {
    fn to_string(&self) -> String {
        let name = match self {
            Test::Mesh => "mesh",
        };

        String::from(name)
    }
}

pub struct Drawing {
    test: Test,
    canvas: Canvas,
}

impl Drawing {
    pub fn test(&self) -> Test {
        self.test
    }

    pub fn canvas(&self) -> &Canvas {
        &self.canvas
    }

    pub fn save_as_model(&self, gpu: &mut Gpu) {
        let model_path = self.model_path();
        let model_directory = model_path.parent().expect("Get model directory");
        let image = self.canvas.read_pixels(gpu);

        std::fs::create_dir_all(model_directory)
            .expect("Create model directory");

        image
            .to_rgba()
            .save(self.model_path())
            .expect(&format!("Save \"{:?}\" drawing", self.test));
    }

    pub fn differences(
        &self,
        gpu: &mut Gpu,
    ) -> Result<Option<Differences>, Error> {
        let model = {
            let mut buf = Vec::new();
            let mut reader = File::open(self.model_path())?;
            let _ = reader.read_to_end(&mut buf)?;
            image::load_from_memory(&buf)?
        };

        let image = self.canvas.read_pixels(gpu);

        let model_rgba = model.to_rgba();
        let image_rgba = image.to_rgba();

        if model_rgba
            .pixels()
            .zip(image_rgba.pixels())
            .all(|(a, b)| a == b)
        {
            Ok(None)
        } else {
            let differences: Vec<u8> = model_rgba
                .pixels()
                .zip(image_rgba.pixels())
                .flat_map(|(a, b)| {
                    if a == b {
                        &[0, 0, 0, 0]
                    } else {
                        &[255, 0, 0, 255]
                    }
                })
                .cloned()
                .collect();

            let image = image::RgbaImage::from_raw(
                self.canvas.width() as u32,
                self.canvas.height() as u32,
                differences,
            )
            .expect("Create diff image");

            let image =
                Image::from_image(gpu, &image::DynamicImage::ImageRgba8(image))
                    .expect("Upload diff image");

            Ok(Some(Differences {
                test: self.test,
                canvas: self.canvas.clone(),
                image,
            }))
        }
    }

    fn model_path(&self) -> PathBuf {
        let mut path = PathBuf::new();

        path.push("tests");
        path.push("_graphics");
        path.push("models");
        path.push(self.test.to_string());
        path.set_extension("png");

        path
    }
}

#[derive(Debug)]
pub struct Differences {
    test: Test,
    canvas: Canvas,
    image: Image,
}

impl Differences {
    pub fn test(&self) -> Test {
        self.test
    }

    pub fn canvas(&self) -> &Canvas {
        &self.canvas
    }

    pub fn image(&self) -> &Image {
        &self.image
    }
}

#[derive(Debug)]
pub enum Error {
    ModelImageNotFound(std::io::Error),
    ModelImageIsInvalid(image::ImageError),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error::ModelImageNotFound(error)
    }
}

impl From<image::ImageError> for Error {
    fn from(error: image::ImageError) -> Error {
        Error::ModelImageIsInvalid(error)
    }
}
