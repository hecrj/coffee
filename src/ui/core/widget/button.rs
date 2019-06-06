//! Allow your users to perform actions by pressing a button.
//!
//! A [`Button`] has some local [`State`] and a [`Class`].
//!
//! [`Button`]: struct.Button.html
//! [`State`]: struct.State.html
//! [`Class`]: enum.Class.html

use crate::graphics::{Point, Rectangle};
use crate::input::{ButtonState, MouseButton};
use crate::ui::core::{
    Align, Element, Event, Hasher, Layout, MouseCursor, Node, Style, Widget,
};

use std::hash::Hash;

/// A generic widget that produces a message when clicked.
///
/// It implements [`Widget`] when the associated [`Widget::Renderer`] implements
/// the [`button::Renderer`] trait.
///
/// [`Widget`]: ../trait.Widget.html
/// [`Widget::Renderer`]: ../trait.Widget.html#associatedtype.Renderer
/// [`button::Renderer`]: trait.Renderer.html
pub struct Button<'a, M, R> {
    state: &'a mut State,
    label: String,
    class: Class,
    on_click: Option<M>,
    style: Style,
    renderer: std::marker::PhantomData<R>,
}

impl<'a, M, R> Button<'a, M, R> {
    /// Creates a new [`Button`] with some local [`State`] and the given label.
    ///
    /// The default [`Class`] of a new [`Button`] is [`Class::Primary`].
    ///
    /// [`Button`]: struct.Button.html
    /// [`State`]: struct.State.html
    /// [`Class`]: enum.Class.html
    /// [`Class::Primary`]: enum.Class.html#variant.Primary
    pub fn new(state: &'a mut State, label: &str) -> Self {
        Button {
            state,
            label: String::from(label),
            class: Class::Primary,
            on_click: None,
            style: Style::default().min_width(100),
            renderer: std::marker::PhantomData,
        }
    }

    /// Sets the width of the [`Button`] in pixels.
    ///
    /// [`Button`]: struct.Button.html
    pub fn width(mut self, width: u32) -> Self {
        self.style = self.style.width(width);
        self
    }

    /// Makes the [`Button`] fill the horizontal space of its container.
    ///
    /// [`Button`]: struct.Button.html
    pub fn fill_width(mut self) -> Self {
        self.style = self.style.fill_width();
        self
    }

    /// Sets the alignment of the [`Button`] itself.
    ///
    /// This is useful if you want to override the default alignment given by
    /// the parent container.
    ///
    /// [`Button`]: struct.Button.html
    pub fn align_self(mut self, align: Align) -> Self {
        self.style = self.style.align_self(align);
        self
    }

    /// Sets the [`Class`] of the [`Button`].
    ///
    ///
    /// [`Button`]: struct.Button.html
    /// [`Class`]: enum.Class.html
    pub fn class(mut self, class: Class) -> Self {
        self.class = class;
        self
    }

    /// Sets the message that will be produced when the [`Button`] is clicked.
    ///
    /// [`Button`]: struct.Button.html
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
        Node::new(self.style.height(50))
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
            cursor_position,
            layout.bounds(),
            self.state,
            &self.label,
            self.class,
        )
    }

    fn hash(&self, state: &mut Hasher) {
        self.style.hash(state);
    }
}

/// The local state of a [`Button`].
///
/// [`Button`]: struct.Button.html
#[derive(Debug)]
pub struct State {
    is_pressed: bool,
}

impl State {
    /// Creates a new [`State`].
    ///
    /// [`State`]: struct.State.html
    pub fn new() -> State {
        State { is_pressed: false }
    }

    /// Returns whether the associated [`Button`] is currently being pressed or
    /// not.
    ///
    /// [`Button`]: struct.Button.html
    pub fn is_pressed(&self) -> bool {
        self.is_pressed
    }
}

/// The type of a [`Button`].
///
/// [`Button`]: struct.Button.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    /// The [`Button`] performs an important action.
    ///
    /// [`Button`]: struct.Button.html
    Primary,

    /// The [`Button`] performs an alternative action.
    ///
    /// [`Button`]: struct.Button.html
    Secondary,

    /// The [`Button`] performs a productive action.
    ///
    /// [`Button`]: struct.Button.html
    Positive,
}

/// The renderer of a [`Button`].
///
/// Your [`core::Renderer`] will need to implement this trait before being
/// able to use a [`Button`] in your user interface.
///
/// [`Button`]: struct.Button.html
/// [`core::Renderer`]: ../../trait.Renderer.html
pub trait Renderer {
    /// Draws a [`Button`].
    ///
    /// It receives the current cursor position and the bounds,
    /// [`State`], label, and [`Class`] of the [`Button`].
    ///
    /// [`Button`]: struct.Button.html
    /// [`State`]: struct.State.html
    /// [`Class`]: enum.Class.html
    fn draw(
        &mut self,
        cursor_position: Point,
        bounds: Rectangle<f32>,
        state: &State,
        label: &str,
        class: Class,
    ) -> MouseCursor;
}

impl<'a, M, R> From<Button<'a, M, R>> for Element<'a, M, R>
where
    R: 'static + Renderer,
    M: 'static + Copy,
{
    fn from(button: Button<'a, M, R>) -> Element<'a, M, R> {
        Element::new(button)
    }
}
