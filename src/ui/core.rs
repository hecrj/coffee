//! Customize your user interface with your own widgets and renderers.
//!
//!   * The [`Widget`] trait allows you to build custom widgets.
//!   * The [`Renderer`] trait can be used to build your own renderer.
//!
//! [`Widget`]: trait.Widget.html
//! [`Renderer`]: trait.Renderer.html
mod renderer;

#[doc(no_inline)]
pub use iced::{
    renderer::Debugger, Align, Element, Event, Hasher, Justify, Layout,
    MouseCursor, Node, Number, Size, Style, Widget,
};

pub use renderer::Renderer;
