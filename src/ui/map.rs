use crate::ui::{Node, Widget};

pub struct Map<'a, O, M, R> {
    widget: Box<Widget<'a, Msg = O, Renderer = R> + 'a>,
    mapper: Fn(O) -> M,
}
