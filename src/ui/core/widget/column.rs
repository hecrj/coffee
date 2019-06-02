use std::hash::Hash;

use crate::graphics::{Point, Rectangle};
use crate::ui::core::{
    Align, Element, Event, Hasher, Justify, Layout, MouseCursor, Node, Style,
    Widget,
};

pub struct Column<'a, M, R> {
    style: Style,
    spacing: u16,
    children: Vec<Element<'a, M, R>>,
}

impl<'a, M, R> Column<'a, M, R> {
    pub fn new() -> Self {
        let mut style = Style::default();
        style.0.flex_direction = stretch::style::FlexDirection::Column;

        Column {
            style,
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

    pub fn fill_width(mut self) -> Self {
        self.style = self.style.fill_width();
        self
    }

    pub fn spacing(mut self, px: u16) -> Self {
        self.spacing = px;
        self
    }

    pub fn padding(mut self, px: u32) -> Self {
        self.style = self.style.padding(px);
        self
    }

    pub fn align_self(mut self, align: Align) -> Self {
        self.style = self.style.align_self(align);
        self
    }

    pub fn align_items(mut self, align: Align) -> Self {
        self.style = self.style.align_items(align);
        self
    }

    pub fn justify_content(mut self, justify: Justify) -> Self {
        self.style = self.style.justify_content(justify);
        self
    }

    pub fn push<E>(mut self, child: E) -> Column<'a, M, R>
    where
        E: Into<Element<'a, M, R>>,
    {
        self.children.push(child.into());
        self
    }
}

impl<'a, M, R> Widget for Column<'a, M, R>
where
    R: Renderer,
{
    type Message = M;
    type Renderer = R;

    fn node(&self, renderer: &R) -> Node {
        let mut children: Vec<Node> = self
            .children
            .iter()
            .map(|child| {
                let mut node = child.widget.node(renderer);

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

        Node::new(self.style, children)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout,
        cursor_position: Point,
        messages: &mut Vec<Self::Message>,
    ) {
        self.children.iter_mut().zip(layout.children()).for_each(
            |(child, layout)| {
                child
                    .widget
                    .on_event(event, layout, cursor_position, messages)
            },
        );
    }

    fn draw(
        &self,
        renderer: &mut Self::Renderer,
        layout: Layout,
        cursor_position: Point,
    ) -> MouseCursor {
        let mut cursor = MouseCursor::OutOfBounds;

        renderer.draw(layout.bounds());

        self.children.iter().zip(layout.children()).for_each(
            |(child, layout)| {
                let new_cursor =
                    child.widget.draw(renderer, layout, cursor_position);

                if new_cursor != MouseCursor::OutOfBounds {
                    cursor = new_cursor;
                }
            },
        );

        cursor
    }

    fn hash(&self, state: &mut Hasher) {
        self.style.hash(state);
        self.spacing.hash(state);

        for child in &self.children {
            child.widget.hash(state);
        }
    }
}

pub trait Renderer {
    fn draw(&mut self, bounds: Rectangle<f32>);
}

impl<'a, M, R> From<Column<'a, M, R>> for Element<'a, M, R>
where
    R: Renderer + 'static,
    M: 'static,
{
    fn from(column: Column<'a, M, R>) -> Element<'a, M, R> {
        Element::new(column)
    }
}
