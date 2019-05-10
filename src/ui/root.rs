use crate::ui::{Layout, Widget};

pub struct Root<'a, M> {
    root: Box<Widget<Msg = M> + 'a>,
}

impl<'a, M> Root<'a, M> {
    pub fn new(root: impl Widget<Msg = M> + 'a) -> Root<'a, M> {
        Root {
            root: Box::new(root),
        }
    }

    pub fn layout(&self) -> Layout {
        self.root.node().layout()
    }
}
