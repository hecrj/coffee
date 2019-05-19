use crate::graphics::{Point, Rectangle};
use crate::ui::{Event, Layout, Map, MouseCursor, Node, Style, Widget};

pub struct Panel<'a, M, R> {
    style: Style,
    content: Box<Widget<'a, Msg = M, Renderer = R> + 'a>,
}

impl<'a, M, R> Panel<'a, M, R> {
    pub fn new(content: impl Widget<'a, Msg = M, Renderer = R> + 'a) -> Self {
        Panel {
            style: Style::default().padding(20),
            content: Box::new(content),
        }
    }

    pub fn width(mut self, width: u32) -> Self {
        self.style = self.style.width(width as f32);
        self
    }

    pub fn max_width(mut self, max_width: u32) -> Self {
        self.style = self.style.max_width(max_width as f32);
        self
    }

    pub fn map<B, F>(self, f: F) -> Map<'a, M, B, R>
    where
        F: Fn(M) -> B + 'static,
        M: Copy + 'static,
        R: Renderer + 'static,
    {
        Map::new(Box::new(self), f)
    }
}

impl<'a, M, R> Widget<'a> for Panel<'a, M, R>
where
    R: Renderer,
{
    type Msg = M;
    type Renderer = R;

    fn node(&self, renderer: &R) -> Node {
        Node::new(self.style, vec![self.content.node(renderer)])
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout,
        cursor_position: Point,
        messages: &mut Vec<Self::Msg>,
    ) {
        [&mut self.content]
            .iter_mut()
            .zip(layout.children())
            .for_each(|(child, layout)| {
                child.on_event(event, layout, cursor_position, messages)
            });
    }

    fn draw(
        &self,
        renderer: &mut Self::Renderer,
        layout: Layout,
        cursor_position: Point,
    ) -> MouseCursor {
        let mut cursor = MouseCursor::Default;
        renderer.draw(layout.bounds());

        [&self.content].iter().zip(layout.children()).for_each(
            |(child, layout)| {
                let new_cursor = child.draw(renderer, layout, cursor_position);

                if new_cursor != MouseCursor::Default {
                    cursor = new_cursor;
                }
            },
        );

        cursor
    }
}

pub trait Renderer {
    fn draw(&mut self, bounds: Rectangle<f32>);
}
