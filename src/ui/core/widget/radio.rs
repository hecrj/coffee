use std::hash::Hash;

use crate::graphics::{
    Color, HorizontalAlignment, Point, Rectangle, VerticalAlignment,
};
use crate::input::{ButtonState, MouseButton};
use crate::ui::core::widget::{text, Column, Row, Text};
use crate::ui::core::{
    Align, Element, Event, Hasher, Layout, MouseCursor, Node, Widget,
};

pub struct Radio<Message> {
    is_selected: bool,
    on_click: Message,
    label: String,
}

impl<Message> Radio<Message> {
    pub fn new<F, V>(value: V, label: &str, selected: Option<V>, f: F) -> Self
    where
        V: Eq + Copy,
        F: 'static + Fn(V) -> Message,
    {
        Radio {
            is_selected: Some(value) == selected,
            on_click: f(value),
            label: String::from(label),
        }
    }
}

impl<Message, Renderer> Widget<Message, Renderer> for Radio<Message>
where
    Renderer: self::Renderer + text::Renderer,
    Message: Copy,
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
                if layout.bounds().contains(cursor_position) {
                    messages.push(self.on_click);
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

impl<'a, Message, Renderer> From<Radio<Message>>
    for Element<'a, Message, Renderer>
where
    Renderer: self::Renderer + text::Renderer,
    Message: 'static + Copy,
{
    fn from(checkbox: Radio<Message>) -> Element<'a, Message, Renderer> {
        Element::new(checkbox)
    }
}
