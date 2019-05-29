use stretch::{geometry, result};

use crate::graphics::Point;
use crate::ui::{Event, Hasher, Layout, MouseCursor, Node, Widget};

pub struct Element<'a, M, R> {
    pub(crate) widget: Box<Widget<Message = M, Renderer = R> + 'a>,
}

impl<'a, M, R> Element<'a, M, R> {
    pub fn new(
        widget: impl Widget<Message = M, Renderer = R> + 'a,
    ) -> Element<'a, M, R> {
        Element {
            widget: Box::new(widget),
        }
    }

    pub fn map<F, B>(self, f: F) -> Element<'a, B, R>
    where
        M: Copy + 'static,
        B: 'static,
        R: 'static,
        F: Fn(M) -> B + 'static,
    {
        Element {
            widget: Box::new(Map::new(self.widget, f)),
        }
    }

    pub(crate) fn compute_layout(&self, renderer: &R) -> result::Layout {
        let node = self.widget.node(renderer);

        node.0.compute_layout(geometry::Size::undefined()).unwrap()
    }

    pub(crate) fn hash(&self, state: &mut Hasher) {
        self.widget.hash(state);
    }
}

pub struct Map<'a, A, B, R> {
    widget: Box<Widget<Message = A, Renderer = R> + 'a>,
    mapper: Box<Fn(A) -> B>,
}

impl<'a, A, B, R> Map<'a, A, B, R> {
    pub fn new<F>(
        widget: Box<Widget<Message = A, Renderer = R> + 'a>,
        mapper: F,
    ) -> Map<'a, A, B, R>
    where
        F: Fn(A) -> B + 'static,
    {
        Map {
            widget,
            mapper: Box::new(mapper),
        }
    }
}

impl<'a, A, B, R> Widget for Map<'a, A, B, R>
where
    A: Copy,
{
    type Message = B;
    type Renderer = R;

    fn node(&self, renderer: &R) -> Node {
        self.widget.node(renderer)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout,
        cursor_position: Point,
        messages: &mut Vec<Self::Message>,
    ) {
        let mut original_messages = Vec::new();

        self.widget.on_event(
            event,
            layout,
            cursor_position,
            &mut original_messages,
        );

        original_messages
            .iter()
            .cloned()
            .for_each(|message| messages.push((self.mapper)(message)));
    }

    fn draw(
        &self,
        renderer: &mut R,
        layout: Layout,
        cursor_position: Point,
    ) -> MouseCursor {
        self.widget.draw(renderer, layout, cursor_position)
    }

    fn hash(&self, state: &mut Hasher) {
        self.widget.hash(state);
    }
}
