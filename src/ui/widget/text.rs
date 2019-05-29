use std::hash::Hash;

use crate::graphics::{Color, Point, Rectangle};
use crate::ui::{Element, Hasher, Layout, MouseCursor, Node, Style, Widget};

pub struct Text<M, R> {
    content: String,
    size: u16,
    color: Color,
    style: Style,
    message: std::marker::PhantomData<M>,
    renderer: std::marker::PhantomData<R>,
}

impl<M, R> Text<M, R> {
    pub fn new(label: &str) -> Self {
        Text {
            content: String::from(label),
            size: 20,
            color: Color::default(),
            style: Style::default(),
            message: std::marker::PhantomData,
            renderer: std::marker::PhantomData,
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
        self.style = self.style.width(width as f32);
        self
    }

    pub fn align_center(mut self) -> Self {
        self.style = self.style.align_center();
        self
    }

    pub fn align_right(mut self) -> Self {
        self.style = self.style.align_right();
        self
    }
}

impl<M, R> Widget for Text<M, R>
where
    R: Renderer,
{
    type Message = M;
    type Renderer = R;

    fn node(&self, renderer: &R) -> Node {
        renderer.node(self.style, &self.content, self.size as f32)
    }

    fn draw(
        &self,
        renderer: &mut R,
        layout: Layout,
        _cursor_position: Point,
    ) -> MouseCursor {
        renderer.draw(
            &self.content,
            self.size as f32,
            self.color,
            layout.bounds(),
        );

        MouseCursor::Default
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
        bounds: Rectangle<f32>,
    );
}

impl<'a, M, R> From<Text<M, R>> for Element<'a, M, R>
where
    R: Renderer + 'static,
    M: 'static,
{
    fn from(text: Text<M, R>) -> Element<'a, M, R> {
        Element::new(text)
    }
}
