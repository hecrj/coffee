use std::hash::Hash;

use crate::graphics::{Color, Point, Rectangle};
use crate::input::{ButtonState, MouseButton};
use crate::ui::{
    column, text, Column, Element, Event, Hasher, Layout, MouseCursor, Node,
    Row, Text, Widget,
};

pub struct Checkbox<M, R> {
    is_checked: bool,
    on_toggle: Box<Fn(bool) -> M>,
    label: String,
    renderer: std::marker::PhantomData<R>,
}

impl<M, R> Checkbox<M, R> {
    pub fn new<F>(is_checked: bool, label: &str, f: F) -> Self
    where
        F: Fn(bool) -> M + 'static,
    {
        Checkbox {
            is_checked,
            on_toggle: Box::new(f),
            label: String::from(label),
            renderer: std::marker::PhantomData,
        }
    }
}

impl<'a, M, R> Widget<'a> for Checkbox<M, R>
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
                let mouse_over = layout
                    .children()
                    .any(|child| child.bounds().contains(cursor_position));

                if mouse_over {
                    messages.push((self.on_toggle)(!self.is_checked));
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
            self.is_checked,
            children[0].bounds(),
            text_bounds,
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
        is_checked: bool,
        bounds: Rectangle<f32>,
        label_bounds: Rectangle<f32>,
        cursor_position: Point,
    ) -> MouseCursor;
}

impl<'a, M, R> From<Checkbox<M, R>> for Element<'a, M, R>
where
    R: Renderer + column::Renderer + text::Renderer + 'static,
    M: Copy + 'static,
{
    fn from(checkbox: Checkbox<M, R>) -> Element<'a, M, R> {
        Element::new(checkbox)
    }
}
