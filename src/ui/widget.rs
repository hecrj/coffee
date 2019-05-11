use crate::graphics::Point;
use crate::ui::Node;

pub trait Widget<'a> {
    type Msg;
    type Renderer;

    fn node(&self) -> Node;

    fn children(
        &self,
    ) -> Option<
        &Vec<Box<Widget<'a, Msg = Self::Msg, Renderer = Self::Renderer> + 'a>>,
    >;

    fn draw(
        &self,
        renderer: &mut Self::Renderer,
        location: Point,
        width: f32,
        height: f32,
    );
}
