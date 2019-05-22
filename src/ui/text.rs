use crate::graphics::{Color, Point, Rectangle};
use crate::ui::{Event, Layout, MouseCursor, Node, Style, Widget};

pub struct Text<M, R> {
    content: String,
    size: f32,
    color: Color,
    style: Style,
    message: std::marker::PhantomData<M>,
    renderer: std::marker::PhantomData<R>,
}

impl<M, R> Text<M, R> {
    pub fn new(label: &str) -> Self {
        Text {
            content: String::from(label),
            size: 20.0,
            color: Color::default(),
            style: Style::default(),
            message: std::marker::PhantomData,
            renderer: std::marker::PhantomData,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
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
        renderer.node(self.style, &self.content, self.size)
    }

    fn draw(
        &self,
        renderer: &mut R,
        layout: Layout,
        cursor_position: Point,
    ) -> MouseCursor {
        renderer.draw(
            &self.content,
            self.size,
            self.color,
            layout.bounds(),
            cursor_position,
        )
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
