use crate::graphics::{Point, Rectangle};
use crate::ui::{Event, MouseCursor, Node};

pub trait Widget<'a> {
    type Msg;
    type Renderer;

    fn node(&self) -> Node;

    fn children(
        &mut self,
    ) -> Option<
        &mut Vec<
            Box<Widget<'a, Msg = Self::Msg, Renderer = Self::Renderer> + 'a>,
        >,
    > {
        None
    }

    fn on_event(
        &mut self,
        _event: Event,
        _bounds: Rectangle<f32>,
        _cursor_position: Point,
    ) -> Option<Self::Msg> {
        None
    }

    fn draw(
        &self,
        renderer: &mut Self::Renderer,
        bounds: Rectangle<f32>,
        cursor_position: Point,
    ) -> MouseCursor;
}
