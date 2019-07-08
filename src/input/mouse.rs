//! Listen to mouse events.

mod event;
mod wheel_movement;

pub use crate::graphics::window::winit::MouseButton as Button;
pub use event::Event;
pub use wheel_movement::WheelMovement;
