use crate::ui::{Layout, Style};

pub struct Node(pub(super) stretch::node::Node);

impl Node {
    pub fn new(style: Style, children: Vec<Node>) -> Node {
        Node(stretch::node::Node::new(
            style.0,
            children.iter().map(|c| &c.0).collect(),
        ))
    }

    pub(crate) fn layout(&self) -> Layout {
        Layout::new(
            self.0
                .compute_layout(stretch::geometry::Size::undefined())
                .unwrap(),
        )
    }
}
