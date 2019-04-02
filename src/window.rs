use winit;

pub struct Builder(winit::WindowBuilder);

impl Builder {
    pub fn new() -> Builder {
        Builder(winit::WindowBuilder::new())
    }

    pub fn title<S: Into<String>>(self, title: S) -> Builder {
        Builder(self.0.with_title(title))
    }

    pub fn dimensions(self, width: u32, height: u32) -> Builder {
        Builder(self.0.with_dimensions(winit::dpi::LogicalSize {
            width: width as f64,
            height: height as f64,
        }))
    }

    pub fn resizable(self, resizable: bool) -> Builder {
        Builder(self.0.with_resizable(resizable))
    }

    pub fn open(self) -> Window {
        let events_loop = winit::EventsLoop::new();
        let window = self.0.build(&events_loop).unwrap();

        Window {
            events_loop,
            window,
        }
    }
}

pub struct Window {
    window: winit::Window,
    events_loop: winit::EventsLoop,
}

impl Window {
    pub fn poll_events<F>(&mut self, mut f: F)
    where
        F: FnMut(Event),
    {
        self.events_loop.poll_events(|event| {
            match event {
                winit::Event::WindowEvent {
                    event: winit::WindowEvent::CloseRequested,
                    ..
                } => f(Event::CloseRequested),
                _ => (),
            };
        });
    }
}

pub enum Event {
    CloseRequested,
}
