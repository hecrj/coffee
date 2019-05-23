use std::hash::Hash;

use crate::graphics::{Color, Point, Rectangle};
use crate::ui::{Hasher, Layout, MouseCursor, Node, Style, Widget};

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
}

impl<'a, M, R> Widget<'a> for Text<M, R>
where
    R: Renderer,
    M: Copy,
{
    type Msg = M;
    type Renderer = R;

    fn node(&self, renderer: &R) -> Node {
        renderer.node(self.style, &self.content, self.size as f32)
    }

    fn draw(
        &self,
        renderer: &mut R,
        layout: Layout,
        cursor_position: Point,
    ) -> MouseCursor {
        renderer.draw(
            &self.content,
            self.size as f32,
            self.color,
            layout.bounds(),
            cursor_position,
        )
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
        cursor_position: Point,
    ) -> MouseCursor;
}
