use coffee::graphics::Canvas;
use coffee::load::Task;

mod mesh;

pub use mesh::Mesh;

pub struct Test {
    name: Name,
    output: Canvas,
}

#[derive(Clone, Copy)]
pub enum Name {
    Mesh,
}

impl Test {
    pub fn all() -> Vec<Task<Test>> {
        vec![(Name::Mesh, Mesh::draw)]
            .iter()
            .cloned()
            .map(|(name, draw)| draw().map(move |output| Test { name, output }))
            .collect()
    }

    pub fn output(&self) -> &Canvas {
        &self.output
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
