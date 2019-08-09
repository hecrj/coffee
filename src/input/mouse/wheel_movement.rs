/// Movement of a mouse wheel
#[derive(Debug, Copy, Clone)]
pub struct WheelMovement {
    /// The number of horizontal lines scrolled
    pub horizontal: f32,

    /// The number of vertical lines scrolled
    pub vertical: f32,
}

impl WheelMovement {
    /// Creates a new WheelMovement
    pub fn new(horizontal: f32, vertical: f32) -> Self {
        WheelMovement {
            horizontal,
            vertical,
        }
    }
}
