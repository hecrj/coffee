mod column;
mod layout;
mod node;
mod root;
mod style;
mod widget;

pub mod button;

pub use button::Button;
pub use column::Column;
pub use layout::Layout;
pub use node::Node;
pub use root::Root;
pub use style::Style;
pub use widget::Widget;

use crate::graphics::Window;

pub trait UserInterface {
    type Msg;

    fn new() -> Self;

    fn layout(&mut self, window: &Window) -> Root<Self::Msg>;

    fn update(&mut self, msg: Self::Msg);

    fn draw(&self, window: &mut Window);
}

pub enum Length {
    Px(u32),
    Fill,
    Shrink,
}
