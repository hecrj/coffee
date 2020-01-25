//! Build, load, and use texture arrays.
mod batch;
mod builder;
mod loader;

pub use batch::Batch;
pub use builder::Builder;
pub use loader::{Indices, Key, Loader};

use std::fmt;
use std::path::PathBuf;

use crate::graphics::gpu::Texture;

/// A collection of different textures with the same size.
///
/// If you want to use different images to render multiple sprites efficiently,
/// a [`TextureArray`] can do the job.
///
/// You need to use a [`Builder`] or a [`Loader`] to create one. Use a [`Batch`]
/// to draw it.
///
/// Cloning a [`TextureArray`] is cheap, it only clones a handle. It does not
/// create new copy of the texture on the GPU.
///
/// [`TextureArray`]: struct.TextureArray.html
/// [`Builder`]: struct.Builder.html
/// [`Loader`]: struct.Loader.html
/// [`Batch`]: struct.Batch.html
#[derive(Debug, Clone)]
pub struct TextureArray {
    texture: Texture,
    x_unit: f32,
    y_unit: f32,
}

/// An index that identifies a texture in a [`TextureArray`].
///
/// You will need this in order to draw using a [`Batch`].
///
/// [`TextureArray`]: struct.TextureArray.html
/// [`Batch`]: struct.Batch.html
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Index {
    layer: u16,
    offset: Offset,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Offset {
    x: f32,
    y: f32,
}

/// A texture array loading error.
#[derive(Debug, Clone)]
pub enum Error {
    /// A texture array [`Index`] could not be found for the given [`Key`].
    ///
    /// [`Key`]: struct.Key.html
    /// [`Index`]: struct.Index.html
    KeyNotFound(usize),

    /// A provided image did not fit in a texture array layer.
    ImageIsTooBig(PathBuf),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::KeyNotFound(key) => write!(f, "Key not found: {}", key),
            Error::ImageIsTooBig(path) => {
                write!(f, "Image is too big: {}", path.display())
            }
        }
    }
}
