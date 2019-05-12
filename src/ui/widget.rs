use crate::graphics::Point;
use crate::ui::{Event, Node};

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
        _position: Point,
        _width: f32,
        _height: f32,
        _cursor_position: Point,
    ) -> Option<Self::Msg> {
        None
    }

    fn draw(
        &self,
        renderer: &mut Self::Renderer,
        location: Point,
        width: f32,
        height: f32,
    );
}
