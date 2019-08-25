//! Write some text for your users to read.
use crate::graphics::{
    Color, HorizontalAlignment, Point, Rectangle, VerticalAlignment,
};
use crate::ui::core::{
    Element, Hasher, Layout, MouseCursor, Node, Style, Widget,
};

use std::hash::Hash;

/// A fragment of text.
///
/// It implements [`Widget`] when the associated [`core::Renderer`] implements
/// the [`text::Renderer`] trait.
///
/// [`Widget`]: ../../core/trait.Widget.html
/// [`core::Renderer`]: ../../core/trait.Renderer.html
/// [`text::Renderer`]: trait.Renderer.html
///
/// # Example
///
/// ```
/// use coffee::graphics::Color;
/// use coffee::ui::Text;
///
/// Text::new("I <3 coffee!")
///     .size(40)
///     .color(Color::BLUE);
/// ```
///
/// ![Text drawn by the built-in renderer](https://github.com/hecrj/coffee/blob/bda9818f823dfcb8a7ad0ff4940b4d4b387b5208/images/ui/text.png?raw=true)
#[derive(Debug, Clone)]
pub struct Text {
    content: String,
    size: u16,
    color: Color,
    style: Style,
    horizontal_alignment: HorizontalAlignment,
    vertical_alignment: VerticalAlignment,
}

impl Text {
    /// Create a new fragment of [`Text`] with the given contents.
    ///
    /// [`Text`]: struct.Text.html
    pub fn new(label: &str) -> Self {
        Text {
            content: String::from(label),
            size: 20,
            color: Color::WHITE,
            style: Style::default().fill_width(),
            horizontal_alignment: HorizontalAlignment::Left,
            vertical_alignment: VerticalAlignment::Top,
        }
    }

    /// Sets the size of the [`Text`] in pixels.
    ///
    /// [`Text`]: struct.Text.html
    pub fn size(mut self, size: u16) -> Self {
        self.size = size;
        self
    }

    /// Sets the [`Color`] of the [`Text`].
    ///
    /// [`Text`]: struct.Text.html
    /// [`Color`]: ../../../graphics/struct.Color.html
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Sets the width of the [`Text`] boundaries in pixels.
    ///
    /// [`Text`]: struct.Text.html
    pub fn width(mut self, width: u32) -> Self {
        self.style = self.style.width(width);
        self
    }

    /// Sets the height of the [`Text`] boundaries in pixels.
    ///
    /// [`Text`]: struct.Text.html
    pub fn height(mut self, height: u32) -> Self {
        self.style = self.style.height(height);
        self
    }

    /// Sets the [`HorizontalAlignment`] of the [`Text`].
    ///
    /// [`Text`]: struct.Text.html
    /// [`HorizontalAlignment`]: ../../../graphics/enum.HorizontalAlignment.html
    pub fn horizontal_alignment(
        mut self,
        alignment: HorizontalAlignment,
    ) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    /// Sets the [`VerticalAlignment`] of the [`Text`].
    ///
    /// [`Text`]: struct.Text.html
    /// [`VerticalAlignment`]: ../../../graphics/enum.VerticalAlignment.html
    pub fn vertical_alignment(mut self, alignment: VerticalAlignment) -> Self {
        self.vertical_alignment = alignment;
        self
    }
}

impl<Message, Renderer> Widget<Message, Renderer> for Text
where
    Renderer: self::Renderer,
{
    fn node(&self, renderer: &Renderer) -> Node {
        renderer.node(self.style, &self.content, self.size as f32)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        layout: Layout<'_>,
        _cursor_position: Point,
    ) -> MouseCursor {
        renderer.draw(
            layout.bounds(),
            &self.content,
            self.size as f32,
            self.color,
            self.horizontal_alignment,
            self.vertical_alignment,
        );

        MouseCursor::OutOfBounds
    }

    fn hash(&self, state: &mut Hasher) {
        self.style.hash(state);

        self.content.hash(state);
        self.size.hash(state);
    }
}

/// The renderer of a [`Text`] fragment.
///
/// Your [`core::Renderer`] will need to implement this trait before being
/// able to use a [`Text`] in your user interface.
///
/// [`Text`]: struct.Text.html
/// [`core::Renderer`]: ../../core/trait.Renderer.html
pub trait Renderer {
    /// Creates a [`Node`] with the given [`Style`] for the provided [`Text`]
    /// contents and size.
    ///
    /// You should probably use [`Node::with_measure`] to allow [`Text`] to
    /// adapt to the dimensions of its container.
    ///
    /// [`Node`]: ../../core/struct.Node.html
    /// [`Style`]: ../../core/struct.Style.html
    /// [`Text`]: struct.Text.html
    /// [`Node::with_measure`]: ../../core/struct.Node.html#method.with_measure
    fn node(&self, style: Style, content: &str, size: f32) -> Node;

    /// Draws a [`Text`] fragment.
    ///
    /// It receives:
    ///   * the bounds of the [`Text`]
    ///   * the contents of the [`Text`]
    ///   * the size of the [`Text`]
    ///   * the color of the [`Text`]
    ///   * the [`HorizontalAlignment`] of the [`Text`]
    ///   * the [`VerticalAlignment`] of the [`Text`]
    ///
    /// [`Text`]: struct.Text.html
    /// [`HorizontalAlignment`]: ../../../graphics/enum.HorizontalAlignment.html
    /// [`VerticalAlignment`]: ../../../graphics/enum.VerticalAlignment.html
    fn draw(
        &mut self,
        bounds: Rectangle<f32>,
        content: &str,
        size: f32,
        color: Color,
        horizontal_alignment: HorizontalAlignment,
        vertical_alignment: VerticalAlignment,
    );
}

impl<'a, Message, Renderer> From<Text> for Element<'a, Message, Renderer>
where
    Renderer: self::Renderer,
{
    fn from(text: Text) -> Element<'a, Message, Renderer> {
        Element::new(text)
    }
}
