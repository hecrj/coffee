//! Draw your game with an explicit 2D graphics API.
//!
//! Graphics in Coffee focus on simplicity while __reducing global state__.
//! Operations like matrix transformations, off-screen rendering and draw calls
//! have always an explicit scope. In Coffee, you do not have to remember to pop
//! a transformation from the matrix stack, reset the render target, reset the
//! screen coordinates, etc.
//!
//! To achieve this, the graphics module revolves around three concepts:
//! [graphics processors], [targets], and [resources].
//!
//! # Graphics processors
//! A [`Gpu`] represents a link between your game and a graphics processor. It
//! is necessary to perform any kind of graphical operation, like loading
//! resources and drawing.
//!
//! As of now, you will only have one [`Gpu`] available at a given time.
//! However, in the future, the graphics module may allow recording graphical
//! operations concurrently.
//!
//! # Targets
//! A [`Target`] represents a drawable target on a specific [`Gpu`]. A
//! [`Transformation`] can be applied to them.
//!
//! Any kind of draw operation needs an explicit [`Target`]. For example,
//! [`Image::draw`] needs a reference to a [`Target`] as the last argument.
//!
//! Currently, there are two ways to obtain a [`Target`]: either from a
//! [`Frame`] or a [`Canvas`], using the `as_target` method.
//!
//! # Resources
//! A resource is the source of some drawable object. In Coffee, there is no
//! `Resource` or `Drawable` type/trait. Resources are represented by different
//! types like [`Image`], [`Font`], [`TextureArray`], etc.
//!
//! [graphics processors]: #graphics-processors
//! [targets]: #targets
//! [resources]: #resources
//! [`Gpu`]: struct.Gpu.html
//! [`Target`]: struct.Target.html
//! [`Transformation`]: struct.Transformation.html
//! [`Frame`]: struct.Frame.html
//! [`Canvas`]: struct.Canvas.html
//! [`Image`]: struct.Image.html
//! [`Image::draw`]: struct.Image.html#method.draw
//! [`TextureArray`]: texture_array/struct.TextureArray.html
//! [`Font`]: struct.Font.html
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
mod font;
mod image;
mod point;
mod quad;
mod rectangle;
mod sprite;
mod sprite_batch;
mod target;
mod text;
mod transformation;
mod vector;

pub(crate) mod window;

pub mod texture_array;

pub use self::image::Image;
pub use canvas::Canvas;
pub use color::Color;
pub use font::Font;
pub use gpu::Gpu;
pub use point::Point;
pub use quad::Quad;
pub use rectangle::Rectangle;
pub use sprite::Sprite;
pub use sprite_batch::SpriteBatch;
pub use target::Target;
pub use text::Text;
pub use texture_array::TextureArray;
pub use transformation::Transformation;
pub use vector::Vector;
pub use window::{Frame, Settings as WindowSettings, Window};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {}
