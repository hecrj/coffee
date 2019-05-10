use crate::ui::Widget;

pub struct Layout<'a, M> {
    root: Box<Widget<Msg = M> + 'a>,
    center_x: bool,
    center_y: bool,
}

impl<'a, M> Layout<'a, M> {
    pub fn new(root: impl Widget<Msg = M> + 'a) -> Layout<'a, M> {
        Layout {
            root: Box::new(root),
            center_x: false,
            center_y: true,
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

    pub fn compute(&self, width: f32, height: f32) -> stretch::result::Layout {
        let mut style = stretch::style::Style::default();
        style.flex_direction = stretch::style::FlexDirection::Column;
        style.size.width = stretch::style::Dimension::Points(width);
        style.size.height = stretch::style::Dimension::Points(height);

        if self.center_x {
            style.align_items = stretch::style::AlignItems::Center;
        }

        if self.center_y {
            style.justify_content = stretch::style::JustifyContent::Center;
        }

        let root = self.root.node();

        stretch::node::Node::new(style, vec![&root])
            .compute_layout(stretch::geometry::Size {
                width: stretch::number::Number::Defined(width),
                height: stretch::number::Number::Defined(height),
            })
            .unwrap()
    }
}
