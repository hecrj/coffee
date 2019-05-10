use crate::ui::{Node, Style, Widget};

pub struct Button<'a, M> {
    state: &'a mut State,
    label: String,
    style: Style,
    on_click: Option<M>,
}

impl<'a, M> Button<'a, M> {
    pub fn new(state: &'a mut State, label: &str) -> Self {
        Button {
            state,
            label: String::from(label),
            style: Style::default(),
            on_click: None,
        }
    }

    pub fn width(mut self, width: u32) -> Self {
        self.style = self.style.width(width as f32);
        self
    }

    pub fn on_click(mut self, msg: M) -> Self {
        self.on_click = Some(msg);
        self
    }
}

impl<'a, M> Widget for Button<'a, M> {
    type Msg = M;

    fn node(&self) -> Node {
        Node::new(self.style.height(40.0).grow(), Vec::new())
    }
}

pub struct State {}

impl State {
    pub fn new() -> State {
        State {}
    }
}
