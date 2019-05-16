use crate::graphics::Point;
use crate::ui::{Event, Layout, MouseCursor, Node, Widget};

pub struct Map<'a, A, B, R> {
    widget: Box<Widget<'a, Msg = A, Renderer = R> + 'a>,
    mapper: Box<Fn(A) -> B>,
}

impl<'a, A, B, R> Map<'a, A, B, R> {
    pub fn new<F>(
        widget: Box<Widget<'a, Msg = A, Renderer = R> + 'a>,
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

impl<'a, A, B, R> Widget<'a> for Map<'a, A, B, R>
where
    A: Copy,
{
    type Msg = B;
    type Renderer = R;

    fn node(&self) -> Node {
        self.widget.node()
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout,
        cursor_position: Point,
        messages: &mut Vec<Self::Msg>,
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
}
