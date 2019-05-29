use std::hash::Hash;

use crate::graphics::{Point, Rectangle};
use crate::input::{ButtonState, MouseButton};
use crate::ui::{
    Element, Event, Hasher, Layout, MouseCursor, Node, Style, Widget,
};

pub struct Button<'a, M, R> {
    state: &'a mut State,
    label: String,
    type_: Type,
    on_click: Option<M>,
    style: Style,
    renderer: std::marker::PhantomData<R>,
}

impl<'a, M, R> Button<'a, M, R> {
    pub fn new(state: &'a mut State, label: &str) -> Self {
        Button {
            state,
            label: String::from(label),
            type_: Type::Primary,
            on_click: None,
            style: Style::default().min_width(100.0),
            renderer: std::marker::PhantomData,
        }
    }

    pub fn width(mut self, width: u32) -> Self {
        self.style = self.style.width(width as f32);
        self
    }

    pub fn align_right(mut self) -> Self {
        self.style = self.style.align_right();
        self
    }

    pub fn r#type(mut self, type_: Type) -> Self {
        self.type_ = type_;
        self
    }

    pub fn on_click(mut self, msg: M) -> Self {
        self.on_click = Some(msg);
        self
    }
}

impl<'a, M, R> Widget for Button<'a, M, R>
where
    R: Renderer,
    M: Copy,
{
    type Message = M;
    type Renderer = R;

    fn node(&self, _renderer: &R) -> Node {
        Node::new(self.style.height(50.0).grow(), Vec::new())
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
                state,
            } => {
                if let Some(on_click) = self.on_click {
                    let bounds = layout.bounds();

                    match state {
                        ButtonState::Pressed => {
                            self.state.is_pressed =
                                bounds.contains(cursor_position);
                        }
                        ButtonState::Released => {
                            let is_clicked = self.state.is_pressed
                                && bounds.contains(cursor_position);

                            self.state.is_pressed = false;

                            if is_clicked {
                                messages.push(on_click);
                            }
                        }
                    }
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
        renderer.draw(
            self.state,
            &self.label,
            self.type_,
            layout.bounds(),
            cursor_position,
        )
    }

    fn hash(&self, state: &mut Hasher) {
        self.style.hash(state);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Primary,
    Secondary,
    Positive,
}

pub trait Renderer {
    fn draw(
        &mut self,
        state: &State,
        label: &str,
        type_: Type,
        bounds: Rectangle<f32>,
        cursor_position: Point,
    ) -> MouseCursor;
}

pub struct State {
    is_pressed: bool,
}

impl State {
    pub fn new() -> State {
        State { is_pressed: false }
    }

    pub fn is_pressed(&self) -> bool {
        self.is_pressed
    }
}

impl<'a, M, R> From<Button<'a, M, R>> for Element<'a, M, R>
where
    R: Renderer + 'static,
    M: Copy + 'static,
{
    fn from(button: Button<'a, M, R>) -> Element<'a, M, R> {
        Element::new(button)
    }
}
