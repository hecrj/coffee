mod gpu;
pub mod window;

pub use gpu::Gpu;
pub use window::Window;

#[derive(Debug)]
pub enum Error {}
pub type Result<T> = std::result::Result<T, Error>;
