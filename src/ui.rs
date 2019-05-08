mod widget;

pub mod button;

pub use button::Button;
pub use widget::Widget;

use crate::graphics;

pub trait UserInterface {
    type Msg;

    fn new() -> Self;

    fn layout(&mut self) -> Layout<Self::Msg>;

    fn update(&mut self, msg: Self::Msg);

    fn draw(&self, window: &mut graphics::Window);
}

pub struct Layout<'a, M> {
    root: Box<Widget<Msg = M> + 'a>,
}

impl<'a, M> Layout<'a, M> {
    pub fn new(root: impl Widget<Msg = M> + 'a) -> Layout<'a, M> {
        Layout {
            root: Box::new(root),
        }
    }
}

pub enum Length {
    Px(u32),
    Fill,
    Shrink,
}

pub struct Column<'a, M> {
    children: Vec<Box<Widget<Msg = M> + 'a>>,
}

impl<'a, M> Column<'a, M> {
    pub fn new() -> Self {
        Column {
            children: Vec::new(),
        }
    }

    pub fn center_x(self) -> Self {
        self
    }

    pub fn center_y(self) -> Self {
        self
    }

    pub fn width(self, length: Length) -> Self {
        self
    }

    pub fn spacing(self, px: u32) -> Self {
        self
    }

    pub fn push(mut self, child: impl Widget<Msg = M> + 'a) -> Column<'a, M> {
        self.children.push(Box::new(child));
        self
    }
}

impl<'a, M> Widget for Column<'a, M> {
    type Msg = M;
}
