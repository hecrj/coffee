pub trait Widget {
    type Msg;

    fn node(&self) -> stretch::node::Node;
}
