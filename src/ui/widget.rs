//! Use the built-in widgets in your user interface.
//!
//! # Re-exports
//! The contents of this module are re-exported in the [`ui` module]. Therefore,
//! you can directly type:
//!
//! ```
//! use coffee::ui::{button, Button};
//! ```
//!
//! However, if you want to use a custom renderer, you will need to work with
//! the definitions of [`Row`] and [`Column`] found in this module.
//!
//! # Customization
//! Every drawable widget has its own module with a `Renderer` trait that must
//! be implemented by a custom renderer before being able to use the
//! widget.
//!
//! The built-in [`Renderer`] supports all the widgets in this module!
//!
//! [`ui` module]: ../index.html
//! [`Row`]: struct.Row.html
//! [`Column`]: struct.Column.html
//! [`Renderer`]: ../struct.Renderer.html

use crate::graphics::Color;

mod text;

pub mod image;
pub mod progress_bar;

pub use self::image::Image;
pub use progress_bar::ProgressBar;
pub use text::Text;

#[doc(no_inline)]
pub use iced::{button, slider, Button, Column, Row, Slider};

/// A checkbox.
pub type Checkbox<Message> = iced::Checkbox<Color, Message>;

/// A radio button.
pub type Radio<Message> = iced::Radio<Color, Message>;
