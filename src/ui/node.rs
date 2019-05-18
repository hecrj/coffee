use stretch::node;

use crate::ui::{Number, Size, Style};

pub struct Node(pub(super) node::Node);

impl Node {
    pub fn new(style: Style, children: Vec<Node>) -> Node {
        Node(node::Node::new(
            style.0,
            children.iter().map(|c| &c.0).collect(),
        ))
    }

    pub fn new_leaf<F>(style: Style, measure: F) -> Node
    where
        F: Fn(Size<Number>) -> Size<f32> + 'static,
    {
        Node(node::Node::new_leaf(
            style.0,
            Box::new(move |size| Ok(measure(size))),
        ))
    }
}
