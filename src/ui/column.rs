use crate::graphics::{Point, Rectangle};
use crate::ui::{Event, Layout, Map, MouseCursor, Node, Style, Widget};

pub struct Column<'a, M, R> {
    style: Style,
    spacing: u32,
    children: Vec<Box<Widget<'a, Msg = M, Renderer = R> + 'a>>,
}

impl<'a, M, R> Column<'a, M, R> {
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

    pub fn max_width(mut self, max_width: f32) -> Self {
        self.style = self.style.max_width(max_width);
        self
    }

    pub fn spacing(mut self, px: u32) -> Self {
        self.spacing = px;
        self
    }

    pub fn padding(mut self, px: u32) -> Self {
        self.style = self.style.padding(px);
        self
    }

    pub fn center_children(mut self) -> Self {
        self.style = self.style.center_children();
        self
    }

    pub fn push(
        mut self,
        child: impl Widget<'a, Msg = M, Renderer = R> + 'a,
    ) -> Column<'a, M, R> {
        self.children.push(Box::new(child));
        self
    }

    pub fn map<B, F>(self, f: F) -> Map<'a, M, B, R>
    where
        F: Fn(M) -> B + 'static,
        M: Copy + 'static,
        R: Renderer + 'static,
    {
        Map::new(Box::new(self), f)
    }
}

impl<'a, M, R> Widget<'a> for Column<'a, M, R>
where
    R: Renderer,
{
    type Msg = M;
    type Renderer = R;

    fn node(&self, renderer: &R) -> Node {
        let mut children: Vec<Node> = self
            .children
            .iter()
            .map(|child| {
                let mut node = child.node(renderer);

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

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout,
        cursor_position: Point,
        messages: &mut Vec<Self::Msg>,
    ) {
        self.children.iter_mut().zip(layout.children()).for_each(
            |(child, layout)| {
                child.on_event(event, layout, cursor_position, messages)
            },
        );
    }

    fn draw(
        &self,
        renderer: &mut Self::Renderer,
        layout: Layout,
        cursor_position: Point,
    ) -> MouseCursor {
        let mut cursor = MouseCursor::Default;

        renderer.draw(layout.bounds());

        self.children.iter().zip(layout.children()).for_each(
            |(child, layout)| {
                let new_cursor = child.draw(renderer, layout, cursor_position);

                if new_cursor != MouseCursor::Default {
                    cursor = new_cursor;
                }
            },
        );

        cursor
    }
}

pub trait Renderer {
    fn draw(&mut self, bounds: Rectangle<f32>);
}
