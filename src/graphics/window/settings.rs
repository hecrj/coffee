use super::winit;

/// A window configuration.
pub struct Settings {
    /// A title for the window.
    pub title: String,

    /// A target size for the window.
    pub size: (u32, u32),

    /// Defines whether if the window should be resizable.
    pub resizable: bool,
}

impl Settings {
    pub(super) fn into_builder(self) -> winit::WindowBuilder {
        winit::WindowBuilder::new()
            .with_title(self.title)
            .with_dimensions(winit::dpi::LogicalSize {
                width: self.size.0 as f64,
                height: self.size.1 as f64,
            })
            .with_resizable(self.resizable)
    }
}
