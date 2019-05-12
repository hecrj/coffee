use crate::ui::{Node, Widget};

pub struct Root<'a, M, R> {
    pub(crate) widget: Box<Widget<'a, Msg = M, Renderer = R> + 'a>,
}

impl<'a, M, R> Root<'a, M, R> {
    pub fn new(
        widget: impl Widget<'a, Msg = M, Renderer = R> + 'a,
    ) -> Root<'a, M, R> {
        Root {
            widget: Box::new(widget),
        }
    }

    pub fn node(&self) -> Node {
        self.widget.node()
    }
}

impl<'a, M, A> std::fmt::Debug for Root<'a, M, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Root")
    }
}
