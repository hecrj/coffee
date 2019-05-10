mod column;
mod layout;
mod widget;

pub mod button;

pub use button::Button;
pub use column::Column;
pub use layout::Layout;
pub use widget::Widget;

use crate::graphics;

pub trait UserInterface {
    type Msg;

    fn new() -> Self;

    fn layout(&mut self) -> Layout<Self::Msg>;

    fn update(&mut self, msg: Self::Msg);

    fn draw(&self, window: &mut graphics::Window);
}

pub enum Length {
    Px(u32),
    Fill,
    Shrink,
}
