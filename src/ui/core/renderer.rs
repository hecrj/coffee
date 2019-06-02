use crate::graphics::Window;
use crate::load::Task;

pub trait Renderer {
    type Configuration: Default;

    fn load(config: Self::Configuration) -> Task<Self>
    where
        Self: Sized;

    fn flush(&mut self, window: &mut Window);
}
