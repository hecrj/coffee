use coffee::graphics::Gpu;
use coffee::load::Task;

mod mesh;

pub use mesh::Mesh;

pub enum Test {
    Mesh(Mesh),
}

impl Test {
    pub fn all() -> Vec<Task<Test>> {
        vec![Mesh::load().map(Test::Mesh)]
    }

    pub fn draw(self, gpu: &mut Gpu) -> Drawing {
        let name = self.name();

        let canvas = match self {
            Test::Mesh(mesh) => mesh.draw(gpu),
        };

        // TODO: Save canvas to image here

        Drawing { name }
    }

    fn name(&self) -> &'static str {
        match self {
            Test::Mesh(_) => "mesh",
        }
    }
}

pub struct Drawing {
    name: &'static str,
}

impl Drawing {
    pub fn diff(&self) -> Result {
        // TODO: Perform image diffing
        Result::Failed
    }
}

pub enum Result {
    Passed,
    Failed,
}
