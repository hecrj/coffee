use crate::graphics::{Point, Rectangle};
use crate::input::{ButtonState, MouseButton};
use crate::ui::{Event, MouseCursor, Node, Style, Widget};

pub struct Button<'a, M, R> {
    state: &'a mut State,
    label: String,
    style: Style,
    on_click: Option<M>,
    renderer: std::marker::PhantomData<R>,
}

impl<'a, M, R> Button<'a, M, R> {
    pub fn new(state: &'a mut State, label: &str) -> Self {
        Button {
            state,
            label: String::from(label),
            style: Style::default(),
            on_click: None,
            renderer: std::marker::PhantomData,
        }
    }

    pub fn width(mut self, width: u32) -> Self {
        self.style = self.style.width(width as f32);
        self
    }

    pub fn on_click(mut self, msg: M) -> Self {
        self.on_click = Some(msg);
        self
    }
}

impl<'a, M, R> Widget<'a> for Button<'a, M, R>
where
    R: Renderer,
    M: Copy,
{
    type Msg = M;
    type Renderer = R;

    fn node(&self) -> Node {
        Node::new(self.style.height(50.0).grow(), Vec::new())
    }

    fn on_event(
        &mut self,
        event: Event,
        bounds: Rectangle<f32>,
        cursor_position: Point,
    ) -> Option<M> {
        match event {
            Event::MouseInput {
                button: MouseButton::Left,
                state,
            } => match state {
                ButtonState::Pressed => {
                    self.state.is_pressed = bounds.contains(cursor_position);
                }
                ButtonState::Released => {
                    let is_clicked = self.state.is_pressed
                        && bounds.contains(cursor_position);

                    self.state.is_pressed = false;

                    if is_clicked {
                        println!("{}", self.label);
                        return self.on_click;
                    }
                }
            },
            _ => {}
        }

        None
    }

    fn draw(
        &self,
        renderer: &mut R,
        bounds: Rectangle<f32>,
        cursor_position: Point,
    ) -> MouseCursor {
        renderer.draw(self.state, &self.label, bounds, cursor_position)
    }
}

pub trait Renderer {
    fn draw(
        &mut self,
        state: &State,
        label: &str,
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
}
