use crate::graphics::Point;
use crate::ui::{Event, Hasher, Layout, MouseCursor, Node};

pub trait Widget<'a> {
    type Msg;
    type Renderer;

    fn node(&self, renderer: &Self::Renderer) -> Node;

    fn on_event(
        &mut self,
        _event: Event,
        _layout: Layout,
        _cursor_position: Point,
        _messages: &mut Vec<Self::Msg>,
    ) {
    }

    fn draw(
        &self,
        renderer: &mut Self::Renderer,
        layout: Layout,
        cursor_position: Point,
    ) -> MouseCursor;

    fn hash(&self, state: &mut Hasher);
}
