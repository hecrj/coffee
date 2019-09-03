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
//! the definitions of [`Row`], [`Column`], and [`Panel`] found in this module.
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
//! [`Panel`]: struct.Panel.html
//! [`Renderer`]: ../struct.Renderer.html
mod column;
mod row;

pub mod button;
pub mod checkbox;
pub mod image;
pub mod panel;
pub mod progress_bar;
pub mod radio;
pub mod slider;
pub mod text;

pub use self::image::Image;
pub use button::Button;
pub use checkbox::Checkbox;
pub use column::Column;
pub use panel::Panel;
pub use progress_bar::ProgressBar;
pub use radio::Radio;
pub use row::Row;
pub use slider::Slider;
pub use text::Text;
