use super::winit;

pub struct Settings {
    pub title: String,
    pub size: (u32, u32),
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
