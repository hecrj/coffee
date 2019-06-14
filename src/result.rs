use std::error;
use std::fmt;
use std::io;

use crate::graphics::texture_array;

/// A convenient result with a locked [`Error`] type.
///
/// [`Error`]: enum.Error.html
pub type Result<T> = std::result::Result<T, Error>;

/// An error in the engine.
///
/// They are mostly errors that happen during the initialization stage of your
/// game.
#[derive(Debug)]
pub enum Error {
    /// The window creation failed.
    WindowCreation(String),

    /// A texture array failed to load.
    TextureArray(texture_array::Error),

    /// A file failed to load.
    IO(io::Error),

    /// An image failed to load.
    Image(image::ImageError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::WindowCreation(error) => {
                write!(f, "Window creation error: {}", error)
            }
            Error::TextureArray(error) => {
                write!(f, "Texture array error: {}", error)
            }
            Error::IO(error) => write!(f, "IO error: {}", error),
            Error::Image(error) => write!(f, "Image error: {}", error),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::IO(error) => Some(error),
            Error::Image(error) => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error::IO(error)
    }
}

impl From<image::ImageError> for Error {
    fn from(error: image::ImageError) -> Error {
        Error::Image(error)
    }
}
