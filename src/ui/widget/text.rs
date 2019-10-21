//! Write some text for your users to read.
use crate::graphics::{Color, HorizontalAlignment, VerticalAlignment};
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
    size: Option<u16>,
    color: Option<Color>,
    style: Style,
    horizontal_alignment: iced::text::HorizontalAlignment,
    vertical_alignment: iced::text::VerticalAlignment,
}

impl Text {
    /// Create a new fragment of [`Text`] with the given contents.
    ///
    /// [`Text`]: struct.Text.html
    pub fn new(label: &str) -> Self {
        Text {
            content: String::from(label),
            size: None,
            color: None,
            style: Style::default().fill_width(),
            horizontal_alignment: iced::text::HorizontalAlignment::Left,
            vertical_alignment: iced::text::VerticalAlignment::Top,
        }
    }

    /// Sets the size of the [`Text`] in pixels.
    ///
    /// [`Text`]: struct.Text.html
    pub fn size(mut self, size: u16) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets the [`Color`] of the [`Text`].
    ///
    /// [`Text`]: struct.Text.html
    /// [`Color`]: ../../../graphics/struct.Color.html
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the width of the [`Text`] boundaries in pixels.
    ///
    /// [`Text`]: struct.Text.html
    pub fn width(mut self, width: u16) -> Self {
        self.style = self.style.width(width);
        self
    }

    /// Sets the height of the [`Text`] boundaries in pixels.
    ///
    /// [`Text`]: struct.Text.html
    pub fn height(mut self, height: u16) -> Self {
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
        self.horizontal_alignment = alignment.into();
        self
    }

    /// Sets the [`VerticalAlignment`] of the [`Text`].
    ///
    /// [`Text`]: struct.Text.html
    /// [`VerticalAlignment`]: ../../../graphics/enum.VerticalAlignment.html
    pub fn vertical_alignment(mut self, alignment: VerticalAlignment) -> Self {
        self.vertical_alignment = alignment.into();
        self
    }
}

impl<Message, Renderer> Widget<Message, Renderer> for Text
where
    Renderer: iced::text::Renderer<Color>,
{
    fn node(&self, renderer: &Renderer) -> Node {
        renderer.node(self.style, &self.content, self.size)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        layout: Layout<'_>,
        _cursor_position: iced::Point,
    ) -> MouseCursor {
        renderer.draw(
            layout.bounds(),
            &self.content,
            self.size,
            self.color,
            self.horizontal_alignment,
            self.vertical_alignment,
        );

        MouseCursor::OutOfBounds
    }

    fn hash_layout(&self, state: &mut Hasher) {
        self.style.hash(state);

        self.content.hash(state);
        self.size.hash(state);
    }
}

impl<'a, Message, Renderer> Into<Element<'a, Message, Renderer>> for Text
where
    Renderer: iced::text::Renderer<Color>,
{
    fn into(self) -> Element<'a, Message, Renderer> {
        Element::new(self)
    }
}
