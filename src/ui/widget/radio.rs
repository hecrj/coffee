use std::hash::Hash;

use crate::graphics::{Color, Point, Rectangle};
use crate::input::{ButtonState, MouseButton};
use crate::ui::{
    column, text, Column, Element, Event, Hasher, Layout, MouseCursor, Node,
    Row, Text, Widget,
};

pub struct Radio<M, R> {
    is_selected: bool,
    on_click: M,
    label: String,
    renderer: std::marker::PhantomData<R>,
}

impl<M, R> Radio<M, R> {
    pub fn new<F, V>(value: V, label: &str, selected: Option<V>, f: F) -> Self
    where
        V: Eq + Copy,
        F: Fn(V) -> M + 'static,
    {
        Radio {
            is_selected: Some(value) == selected,
            on_click: f(value),
            label: String::from(label),
            renderer: std::marker::PhantomData,
        }
    }
}

impl<'a, M, R> Widget<'a> for Radio<M, R>
where
    R: Renderer + column::Renderer + text::Renderer + 'static,
    M: Copy,
{
    type Msg = M;
    type Renderer = R;

    fn node(&self, renderer: &R) -> Node {
        Row::<(), R>::new()
            .spacing(15)
            .push(Column::new().width(28.0).height(28.0).align_center())
            .push(Text::new(&self.label).align_center())
            .node(renderer)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout,
        cursor_position: Point,
        messages: &mut Vec<M>,
    ) {
        match event {
            Event::MouseInput {
                button: MouseButton::Left,
                state: ButtonState::Pressed,
            } => {
                if layout.bounds().contains(cursor_position) {
                    messages.push(self.on_click);
                }
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        renderer: &mut R,
        layout: Layout,
        cursor_position: Point,
    ) -> MouseCursor {
        let children: Vec<_> = layout.children().collect();

        let mut text_bounds = children[1].bounds();
        text_bounds.y -= 2.0;

        (renderer as &mut text::Renderer).draw(
            &self.label,
            20.0,
            Color::WHITE,
            text_bounds,
        );

        (renderer as &mut Renderer).draw(
            self.is_selected,
            children[0].bounds(),
            layout.bounds(),
            cursor_position,
        )
    }

    fn hash(&self, state: &mut Hasher) {
        self.label.hash(state);
    }
}

pub trait Renderer {
    fn draw(
        &mut self,
        is_selected: bool,
        bounds: Rectangle<f32>,
        label_bounds: Rectangle<f32>,
        cursor_position: Point,
    ) -> MouseCursor;
}

impl<'a, M, R> From<Radio<M, R>> for Element<'a, M, R>
where
    R: Renderer + column::Renderer + text::Renderer + 'static,
    M: Copy + 'static,
{
    fn from(checkbox: Radio<M, R>) -> Element<'a, M, R> {
        Element::new(checkbox)
    }
}
