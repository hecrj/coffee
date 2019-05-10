use crate::ui::Node;

pub trait Widget {
    type Msg;

    fn node(&self) -> Node;
}
