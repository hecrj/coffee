mod column;
mod event;
mod layout;
mod node;
mod root;
mod style;
mod widget;

pub mod button;
pub mod renderer;

pub use button::Button;
pub use column::Column;
pub use event::Event;
pub use layout::Layout;
pub use node::Node;
pub use renderer::Renderer;
pub use root::Root;
pub use style::Style;
pub use widget::Widget;

use crate::graphics::Window;

pub trait UserInterface {
    type Msg;
    type Renderer: Renderer;

    fn new(window: &mut Window) -> (Self, Self::Renderer)
    where
        Self: Sized;

    fn layout(&mut self, window: &Window) -> Root<Self::Msg, Self::Renderer>;

    fn update(&mut self, msg: Self::Msg);
}

pub enum Length {
    Px(u32),
    Fill,
    Shrink,
}
