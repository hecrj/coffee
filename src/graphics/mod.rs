pub mod color;
mod gpu;
pub mod window;

pub use color::Color;
pub use gpu::Gpu;
pub use window::Window;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {}
