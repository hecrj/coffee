use stretch::{geometry, result};

use crate::ui::Widget;

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

    pub(crate) fn compute_layout(&self) -> result::Layout {
        let node = self.widget.node();

        node.0.compute_layout(geometry::Size::undefined()).unwrap()
    }
}

impl<'a, M, A> std::fmt::Debug for Root<'a, M, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Root")
    }
}
