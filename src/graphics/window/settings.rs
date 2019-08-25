use super::winit;

/// A window configuration.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Settings {
    /// A title for the window.
    pub title: String,

    /// A target size for the window.
    pub size: (u32, u32),

    /// Defines whether or not the window should be resizable.
    pub resizable: bool,

    /// Defines whether or not the window should start in fullscreen mode.
    pub fullscreen: bool,
}

impl Settings {
    pub(super) fn into_builder(
        self,
        events_loop: &winit::event_loop::EventLoop<()>,
    ) -> winit::window::WindowBuilder {
        let monitor = if self.fullscreen {
            Some(events_loop.primary_monitor())
        } else {
            None
        };

        winit::window::WindowBuilder::new()
            .with_title(self.title)
            .with_inner_size(winit::dpi::LogicalSize {
                width: self.size.0 as f64,
                height: self.size.1 as f64,
            })
            .with_resizable(self.resizable)
            .with_fullscreen(monitor.map(winit::window::Fullscreen::Borderless))
    }
}
