use crate::graphics::Point;
use crate::ui::core::{Event, Hasher, Layout, MouseCursor, Node};

pub trait Widget<Message, Renderer> {
    fn node(&self, renderer: &Renderer) -> Node;

    fn on_event(
        &mut self,
        _event: Event,
        _layout: Layout,
        _cursor_position: Point,
        _messages: &mut Vec<Message>,
    ) {
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        layout: Layout,
        cursor_position: Point,
    ) -> MouseCursor;

    fn hash(&self, state: &mut Hasher);
}
