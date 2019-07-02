//! Customize your user interface with your own widgets and renderers.
//!
//!   * The [`Widget`] trait allows you to build custom widgets.
//!   * The [`Renderer`] trait can be used to build your own renderer.
//!
//! [`Widget`]: trait.Widget.html
//! [`Renderer`]: trait.Renderer.html
mod element;
mod event;
mod hasher;
mod interface;
mod layout;
mod mouse_cursor;
mod node;
mod renderer;
mod style;
mod widget;

#[doc(no_inline)]
pub use stretch::{geometry::Size, number::Number};

pub use element::Element;
pub use event::Event;
pub use hasher::Hasher;
pub(crate) use interface::{Cache, Interface};
pub use layout::Layout;
pub use mouse_cursor::MouseCursor;
pub use node::Node;
pub use renderer::Renderer;
pub use style::{Align, Justify, Style};
pub use widget::Widget;
