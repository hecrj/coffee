use std::hash::Hash;

use crate::graphics::{
    Color, HorizontalAlignment, Point, Rectangle, VerticalAlignment,
};
use crate::ui::core::{
    Element, Hasher, Layout, MouseCursor, Node, Style, Widget,
};

pub struct Text {
    content: String,
    size: u16,
    color: Color,
    style: Style,
    horizontal_alignment: HorizontalAlignment,
    vertical_alignment: VerticalAlignment,
}

impl Text {
    pub fn new(label: &str) -> Self {
        Text {
            content: String::from(label),
            size: 20,
            color: Color::default(),
            style: Style::default().fill_width(),
            horizontal_alignment: HorizontalAlignment::Left,
            vertical_alignment: VerticalAlignment::Top,
        }
    }

    pub fn size(mut self, size: u16) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.style = self.style.width(width);
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.style = self.style.height(height);
        self
    }

    pub fn horizontal_alignment(
        mut self,
        alignment: HorizontalAlignment,
    ) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

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
        layout: Layout,
        _cursor_position: Point,
    ) -> MouseCursor {
        renderer.draw(
            &self.content,
            self.size as f32,
            self.color,
            self.horizontal_alignment,
            self.vertical_alignment,
            layout.bounds(),
        );

        MouseCursor::OutOfBounds
    }

    fn hash(&self, state: &mut Hasher) {
        self.style.hash(state);

        self.content.hash(state);
        self.size.hash(state);
    }
}

pub trait Renderer {
    fn node(&self, style: Style, content: &str, size: f32) -> Node;

    fn draw(
        &mut self,
        content: &str,
        size: f32,
        color: Color,
        horizontal_alignment: HorizontalAlignment,
        vertical_alignment: VerticalAlignment,
        bounds: Rectangle<f32>,
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
