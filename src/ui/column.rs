use crate::ui::{Node, Style, Widget};

pub struct Column<'a, M> {
    style: Style,
    spacing: u32,
    children: Vec<Box<Widget<Msg = M> + 'a>>,
}

impl<'a, M> Column<'a, M> {
    pub fn new() -> Self {
        Column {
            style: Style::default(),
            spacing: 0,
            children: Vec::new(),
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.style = self.style.width(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.style = self.style.height(height);
        self
    }

    pub fn spacing(mut self, px: u32) -> Self {
        self.spacing = px;
        self
    }

    pub fn center_children(mut self) -> Self {
        self.style = self.style.center_children();
        self
    }

    pub fn push(mut self, child: impl Widget<Msg = M> + 'a) -> Column<'a, M> {
        self.children.push(Box::new(child));
        self
    }
}

impl<'a, M> Widget for Column<'a, M> {
    type Msg = M;

    fn node(&self) -> Node {
        let mut children: Vec<Node> = self
            .children
            .iter()
            .map(|child| {
                let mut node = child.node();

                let mut style = node.0.style();
                style.margin.bottom =
                    stretch::style::Dimension::Points(self.spacing as f32);

                node.0.set_style(style);
                node
            })
            .collect();

        if let Some(node) = children.last_mut() {
            let mut style = node.0.style();
            style.margin.bottom = stretch::style::Dimension::Undefined;

            node.0.set_style(style);
        }

        let mut style = self.style;
        style.0.flex_direction = stretch::style::FlexDirection::Column;

        Node::new(style, children)
    }
}
