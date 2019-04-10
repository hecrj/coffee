use winit;

pub use winit::VirtualKeyCode as KeyCode;

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Event {
    KeyboardInput { state: KeyState, key_code: KeyCode },
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum KeyState {
    Pressed,
    Released,
}

pub enum Mod {}
