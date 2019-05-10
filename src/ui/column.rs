use crate::ui::{Length, Widget};

pub struct Column<'a, M> {
    center_x: bool,
    center_y: bool,
    width: Length,
    spacing: u32,
    children: Vec<Box<Widget<Msg = M> + 'a>>,
}

impl<'a, M> Column<'a, M> {
    pub fn new() -> Self {
        Column {
            center_x: false,
            center_y: false,
            width: Length::Shrink,
            spacing: 0,
            children: Vec::new(),
        }
    }

    pub fn center_x(mut self) -> Self {
        self.center_x = true;
        self
    }

    pub fn center_y(mut self) -> Self {
        self.center_y = true;
        self
    }

    pub fn width(mut self, length: Length) -> Self {
        self.width = length;
        self
    }

    pub fn spacing(mut self, px: u32) -> Self {
        self.spacing = px;
        self
    }

    pub fn push(mut self, child: impl Widget<Msg = M> + 'a) -> Column<'a, M> {
        self.children.push(Box::new(child));
        self
    }
}

impl<'a, M> Widget for Column<'a, M> {
    type Msg = M;

    fn node(&self) -> stretch::node::Node {
        let mut children: Vec<stretch::node::Node> = self
            .children
            .iter()
            .map(|child| {
                let mut node = child.node();

                let mut style = node.style();
                style.margin.bottom =
                    stretch::style::Dimension::Points(self.spacing as f32);

                node.set_style(style);
                node
            })
            .collect();

        if let Some(node) = children.last_mut() {
            let mut style = node.style();
            style.margin.bottom = stretch::style::Dimension::Auto;

            node.set_style(style);
        }

        let mut style = stretch::style::Style::default();
        style.flex_direction = stretch::style::FlexDirection::Column;

        match self.width {
            Length::Shrink => {}
            Length::Fill => {
                style.flex_grow = 1.0;
            }
            Length::Px(width) => {
                style.size.width =
                    stretch::style::Dimension::Points(width as f32);
            }
        }

        stretch::node::Node::new(style, children.iter().collect())
    }
}
