use winit;

pub use winit::VirtualKeyCode as KeyCode;

#[derive(Eq, PartialEq)]
pub enum KeyState {
    Pressed,
    Released,
}

pub enum Mod {}
