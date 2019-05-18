use crate::graphics::{Point, Rectangle};
use crate::ui::{Event, Layout, MouseCursor, Node, Style, Widget};

pub struct Text<M, R> {
    content: String,
    style: Style,
    message: std::marker::PhantomData<M>,
    renderer: std::marker::PhantomData<R>,
}

impl<M, R> Text<M, R> {
    pub fn new(label: &str) -> Self {
        Text {
            content: String::from(label),
            style: Style::default(),
            message: std::marker::PhantomData,
            renderer: std::marker::PhantomData,
        }
    }

    pub fn width(mut self, width: u32) -> Self {
        self.style = self.style.width(width as f32);
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
        renderer.node(self.style, &self.content)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout,
        cursor_position: Point,
        messages: &mut Vec<M>,
    ) {
    }

    fn draw(
        &self,
        renderer: &mut R,
        layout: Layout,
        cursor_position: Point,
    ) -> MouseCursor {
        renderer.draw(&self.content, layout.bounds(), cursor_position)
    }
}

pub trait Renderer {
    fn node(&self, style: Style, content: &str) -> Node;

    fn draw(
        &mut self,
        content: &str,
        bounds: Rectangle<f32>,
        cursor_position: Point,
    ) -> MouseCursor;
}
