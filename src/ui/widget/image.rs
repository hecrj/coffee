//! Displays image to your users.

use crate::graphics::{
    self, Rectangle, Point,
};
use crate::ui::core:: {
    Style, Node, Element, MouseCursor, Layout, Hasher, Widget,
};

use std::hash::Hash;

/// A widget that displays an image.
/// 
/// It implements [`Widget`] when the associated [`core::Renderer`] implements
/// the [`image::Renderer`] trait.
///
/// [`Widget`]: ../../core/trait.Widget.html
/// [`core::Renderer`]: ../../core/trait.Renderer.html
/// [`image::Renderer`]: trait.Renderer.html
/// # Example
///
/// ```
/// use coffee::graphics;
/// use coffee::ui::Image;
///
/// let image_task = graphics::Image::load("resources/ui.png")
/// 	.map(|image| Image::new(&image));
/// ```
#[derive(Debug)]
pub struct Image {
    image: graphics::Image,
    source: Rectangle<u16>,
    style: Style,
}

impl Image {
    /// Creates a new [`Image`] with given image handle.
    ///
    /// [`Image`]: struct.Image.html
    pub fn new(image: &graphics::Image) -> Self {
        Image {
            image: image.clone(),
            source: Rectangle {
                x: 0,
                y: 0,
                width: image.width(),
                height: image.height(),
            },
            style: Style::default().fill_width().fill_height(),
        }
    }

    /// Sets the portion of the [`Image`] that we want to draw.
    /// 
    /// [`Image`]: struct.Image.html
    pub fn clip(mut self, source: Rectangle<u16>) -> Self {
        self.source = source;
        self
    }

    /// Sets the width of the [`Image`] boundaries in pixels.
    ///
    /// [`Image`]: struct.Image.html
    pub fn width(mut self, width: u32) -> Self {
        self.style = self.style.width(width);
        self
    }

    /// Sets the height of the [`Image`] boundaries in pixels.
    ///
    /// [`Image`]: struct.Image.html
    pub fn height(mut self, height: u32) -> Self {
        self.style = self.style.height(height);
        self
    }
}

impl<Message, Renderer> Widget<Message, Renderer> for Image
where
    Renderer: self::Renderer 
{
    fn node(&self, _renderer: &Renderer) -> Node {
        Node::new(self.style)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        layout: Layout<'_>,
        _cursor_position: Point,
    ) -> MouseCursor {
        renderer.draw(
            layout.bounds(),
            self.image.clone(),
            self.source,
        );

        MouseCursor::OutOfBounds
    }

    fn hash(&self, state: &mut Hasher) {
        self.style.hash(state);
    }
}

/// The renderer of a [`Image`].
///
/// Your [`core::Renderer`] will need to implement this trait before being
/// able to use a [`Image`] in your user interface.
///
/// [`Image`]: struct.Image.html
/// [`core::Renderer`]: ../../core/trait.Renderer.html
pub trait Renderer {
    /// Draws a [`Image`].
    ///
    /// It receives:
    ///   * the bounds of the [`Image`]
    ///   * the handle of the loaded [`Image`]
    ///   * the portion of the image that we wants to draw
    ///   
    /// [`Image`]: struct.Image.html
    fn draw(
        &mut self,
        bounds: Rectangle<f32>,
        image: graphics::Image,
        source: Rectangle<u16>,
    );
}

impl<'a, Message, Renderer> From<Image> for Element<'a, Message, Renderer>
where
    Renderer: self::Renderer,
{
    fn from(image: Image) -> Element<'a, Message, Renderer> {
        Element::new(image)
    }
}
