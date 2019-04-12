#[cfg(feature = "opengl")]
mod backend_gfx;
#[cfg(feature = "opengl")]
use backend_gfx as gpu;

#[cfg(feature = "vulkan")]
mod backend_wgpu;
#[cfg(feature = "vulkan")]
use backend_wgpu as gpu;

mod canvas;
mod color;
mod draw_parameters;
mod font;
mod image;
mod point;
mod rectangle;
mod target;
mod text;
mod transformation;
mod vector;

pub mod sprite_batch;
pub mod texture_array;
pub mod window;

pub use self::image::Image;
pub use canvas::Canvas;
pub use color::Color;
pub use draw_parameters::DrawParameters;
pub use font::Font;
pub use gpu::Gpu;
pub use point::Point;
pub use rectangle::Rectangle;
pub use sprite_batch::SpriteBatch;
pub use target::Target;
pub use text::Text;
pub use texture_array::TextureArray;
pub use transformation::Transformation;
pub use vector::Vector;
pub use window::{Frame, Window};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {}
