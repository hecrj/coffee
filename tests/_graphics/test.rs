use coffee::graphics::{Canvas, Gpu};

mod mesh;

use mesh::Mesh;

#[derive(Clone, Copy)]
pub enum Test {
    Mesh,
}

impl Test {
    pub fn all() -> Vec<Test> {
        vec![Test::Mesh]
    }

    pub fn run(&self, gpu: &mut Gpu) -> Execution {
        let draw = match self {
            Test::Mesh => Mesh::draw(),
        };

        Execution {
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

pub struct Execution {
    test: Test,
    canvas: Canvas,
}

impl Execution {
    pub fn canvas(&self) -> &Canvas {
        &self.canvas
    }

    pub fn store(self, gpu: &mut Gpu) -> Output {
        Output {
            test: self.test,
            image: self.canvas.read_pixels(gpu),
        }
    }
}

pub struct Output {
    test: Test,
    image: image::DynamicImage,
}

impl Output {
    pub fn diff(&self) -> Result {
        // TODO: Perform image diffing
        Result::Failed
    }
}

pub enum Result {
    Passed,
    Failed,
}
