use crate::graphics::window::winit;

pub use winit::ElementState as KeyState;
pub use winit::VirtualKeyCode as KeyCode;

#[derive(PartialEq, Clone, Copy)]
pub enum Event {
    KeyboardInput { state: KeyState, key_code: KeyCode },
    CursorMoved { x: f32, y: f32 },
}

pub enum Mod {}
