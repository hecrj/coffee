use crate::graphics::{Point, Rectangle};
use crate::ui::{Event, Node, Style, Widget};

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
            Event::MouseInput { .. } => {
                if bounds.contains(cursor_position) {
                    println!("{}", self.label);
                }
            }
            _ => {}
        }

        None
    }

    fn draw(
        &self,
        renderer: &mut R,
        bounds: Rectangle<f32>,
        cursor_position: Point,
    ) {
        renderer.draw(self.state, &self.label, bounds, cursor_position);
    }
}

pub trait Renderer {
    fn draw(
        &mut self,
        state: &State,
        label: &str,
        bounds: Rectangle<f32>,
        cursor_position: Point,
    );
}

pub struct State {}

impl State {
    pub fn new() -> State {
        State {}
    }
}
