use crate::graphics::Window;
use crate::load::Task;

pub trait Renderer {
    fn load() -> Task<Self>
    where
        Self: Sized;

    fn flush(&mut self, window: &mut Window);
}
