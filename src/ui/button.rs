use crate::graphics::{Point, Rectangle};
use crate::input::{ButtonState, MouseButton};
use crate::ui::{Event, Layout, MouseCursor, Node, Style, Widget};

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
            style: Style::default().min_width(100.0),
            on_click: None,
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
        let bounds = layout.bounds();

        match event {
            Event::MouseInput {
                button: MouseButton::Left,
                state,
            } => {
                if let Some(on_click) = self.on_click {
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
                                println!("{}", self.label);
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
        renderer.draw(self.state, &self.label, layout.bounds(), cursor_position)
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

    pub fn is_pressed(&self) -> bool {
        self.is_pressed
    }
}
