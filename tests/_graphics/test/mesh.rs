use coffee::graphics::{Canvas, Gpu};
use coffee::load::Task;

pub struct Mesh {}

impl Mesh {
    pub fn load() -> Task<Mesh> {
        Task::succeed(|| Mesh {})
    }

    pub fn draw(self, gpu: &mut Gpu) -> Canvas {
        let canvas = Canvas::new(gpu, 400, 400).expect("Canvas creation");

        // TODO: Draw some meshes here

        canvas
    }
}
