use crate::ui::Style;

pub struct Node(pub(super) stretch::node::Node);

impl Node {
    pub fn new(style: Style, children: Vec<Node>) -> Node {
        Node(stretch::node::Node::new(
            style.0,
            children.iter().map(|c| &c.0).collect(),
        ))
    }
}
