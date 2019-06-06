use std::hash::Hash;

use crate::graphics::{
    Color, HorizontalAlignment, Point, Rectangle, VerticalAlignment,
};
use crate::input::{ButtonState, MouseButton};
use crate::ui::core::widget::{text, Column, Row, Text};
use crate::ui::core::{
    Align, Element, Event, Hasher, Layout, MouseCursor, Node, Widget,
};

pub struct Checkbox<Message> {
    is_checked: bool,
    on_toggle: Box<Fn(bool) -> Message>,
    label: String,
}

impl<Message> Checkbox<Message> {
    pub fn new<F>(is_checked: bool, label: &str, f: F) -> Self
    where
        F: 'static + Fn(bool) -> Message,
    {
        Checkbox {
            is_checked,
            on_toggle: Box::new(f),
            label: String::from(label),
        }
    }
}

impl<Message, Renderer> Widget<Message, Renderer> for Checkbox<Message>
where
    Renderer: self::Renderer + text::Renderer,
{
    fn node(&self, renderer: &Renderer) -> Node {
        Row::<(), Renderer>::new()
            .spacing(15)
            .align_items(Align::Center)
            .push(Column::new().width(28).height(28))
            .push(Text::new(&self.label))
            .node(renderer)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout,
        cursor_position: Point,
        messages: &mut Vec<Message>,
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
        renderer: &mut Renderer,
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
            HorizontalAlignment::Left,
            VerticalAlignment::Top,
            text_bounds,
        );

        (renderer as &mut self::Renderer).draw(
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

impl<'a, Message, Renderer> From<Checkbox<Message>>
    for Element<'a, Message, Renderer>
where
    Renderer: self::Renderer + text::Renderer,
    Message: 'static,
{
    fn from(checkbox: Checkbox<Message>) -> Element<'a, Message, Renderer> {
        Element::new(checkbox)
    }
}
