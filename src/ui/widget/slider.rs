use std::hash::Hash;
use std::ops::Range;

use crate::graphics::{Point, Rectangle};
use crate::input::{ButtonState, MouseButton};
use crate::ui::{
    Element, Event, Hasher, Layout, MouseCursor, Node, Style, Widget,
};

pub struct Slider<'a, M, R> {
    state: &'a mut State,
    range: Range<f32>,
    value: f32,
    on_change: Box<dyn Fn(f32) -> M>,
    style: Style,
    renderer: std::marker::PhantomData<R>,
}

impl<'a, M, R> Slider<'a, M, R> {
    pub fn new<F>(
        state: &'a mut State,
        range: Range<f32>,
        value: f32,
        on_change: F,
    ) -> Self
    where
        F: Fn(f32) -> M + 'static,
    {
        Slider {
            state,
            value: value.max(range.start).min(range.end),
            range,
            on_change: Box::new(on_change),
            style: Style::default().min_width(100.0),
            renderer: std::marker::PhantomData,
        }
    }

    pub fn width(mut self, width: u32) -> Self {
        self.style = self.style.width(width as f32);
        self
    }
}

impl<'a, M, R> Widget for Slider<'a, M, R>
where
    R: Renderer,
    M: Copy,
{
    type Msg = M;
    type Renderer = R;

    fn node(&self, _renderer: &R) -> Node {
        Node::new(self.style.height(25.0).grow(), Vec::new())
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout,
        cursor_position: Point,
        messages: &mut Vec<M>,
    ) {
        let mut change = || {
            let bounds = layout.bounds();

            if cursor_position.x <= bounds.x {
                messages.push((self.on_change)(self.range.start));
            } else if cursor_position.x >= bounds.x + bounds.width {
                messages.push((self.on_change)(self.range.end));
            } else {
                let percent = (cursor_position.x - bounds.x) / bounds.width;
                let value = (self.range.end - self.range.start) * percent
                    + self.range.start;

                messages.push((self.on_change)(value));
            }
        };

        match event {
            Event::MouseInput {
                button: MouseButton::Left,
                state,
            } => match state {
                ButtonState::Pressed => {
                    if layout.bounds().contains(cursor_position) {
                        change();
                        self.state.is_dragging = true;
                    }
                }
                ButtonState::Released => {
                    self.state.is_dragging = false;
                }
            },
            Event::CursorMoved => {
                if self.state.is_dragging {
                    change();
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
            &self.range,
            self.value,
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
        range: &Range<f32>,
        value: f32,
        bounds: Rectangle<f32>,
        cursor_position: Point,
    ) -> MouseCursor;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State {
    is_dragging: bool,
}

impl State {
    pub fn new() -> State {
        State { is_dragging: false }
    }

    pub fn is_dragging(&self) -> bool {
        self.is_dragging
    }
}

impl<'a, M, R> From<Slider<'a, M, R>> for Element<'a, M, R>
where
    R: Renderer + 'static,
    M: Copy + 'static,
{
    fn from(slider: Slider<'a, M, R>) -> Element<'a, M, R> {
        Element::new(slider)
    }
}
