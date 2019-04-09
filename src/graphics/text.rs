use crate::graphics::{Color, Vector};

#[derive(Clone)]
pub struct Text {
    pub content: String,
    pub position: Vector,
    pub bounds: (f32, f32),
    pub size: f32,
    pub color: Color,
}
