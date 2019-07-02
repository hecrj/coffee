use coffee::graphics::{
    Color, Frame, HorizontalAlignment, Mesh, Point, Rectangle, Shape, Window,
    WindowSettings,
};
use coffee::input::KeyboardAndMouse;
use coffee::load::{loading_screen::ProgressBar, Task};
use coffee::ui::{
    slider, Align, Column, Element, Justify, Radio, Renderer, Row, Slider,
    Text, UserInterface,
};
use coffee::{Game, Result, Timer};

use std::ops::RangeInclusive;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    #[cfg(debug_assertions)]
    {
        use log::Level;
        console_log::init_with_level(Level::Trace).expect("error initializing log");
    }

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    main().expect("Run main");
}

fn main() -> Result<()> {
    <Example as UserInterface>::run(WindowSettings {
        title: String::from("Mesh - Coffee"),
        size: (1280, 1024),
        resizable: false,
        fullscreen: false,
    })
}

struct Example {
    shape: ShapeOption,
    mode: ModeOption,
    stroke_width: u16,
    radius: f32,
    vertical_radius: f32,
    color: Color,
    polyline_points: Vec<Point>,

    stroke_width_slider: slider::State,
    radius_slider: slider::State,
    vertical_radius_slider: slider::State,
    color_sliders: [slider::State; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShapeOption {
    Rectangle,
    Circle,
    Ellipse,
    Polyline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModeOption {
    Fill,
    Stroke,
}

impl Game for Example {
    type Input = KeyboardAndMouse;
    type LoadingScreen = ProgressBar;

    fn load(_window: &Window) -> Task<Example> {
        Task::new(move || Example {
            shape: ShapeOption::Rectangle,
            mode: ModeOption::Fill,
            color: Color::WHITE,
            radius: 100.0,
            vertical_radius: 50.0,
            stroke_width: 2,
            polyline_points: Vec::new(),

            stroke_width_slider: slider::State::new(),
            radius_slider: slider::State::new(),
            vertical_radius_slider: slider::State::new(),
            color_sliders: [slider::State::new(); 3],
        })
    }

    fn interact(&mut self, input: &mut KeyboardAndMouse, _window: &mut Window) {
        match self.shape {
            ShapeOption::Polyline => {
                self.polyline_points.extend(input.left_clicks());
            }
            _ => {}
        }
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color {
            r: 0.3,
            g: 0.3,
            b: 0.6,
            a: 1.0,
        });

        let mut mesh = Mesh::new();

        let shape = match self.shape {
            ShapeOption::Rectangle => Shape::Rectangle(Rectangle {
                x: frame.width() / 4.0 - 100.0,
                y: frame.height() / 2.0 - 50.0,
                width: 200.0,
                height: 100.0,
            }),
            ShapeOption::Circle => Shape::Circle {
                center: Point::new(frame.width() / 4.0, frame.height() / 2.0),
                radius: self.radius,
            },
            ShapeOption::Ellipse => Shape::Ellipse {
                center: Point::new(frame.width() / 4.0, frame.height() / 2.0),
                horizontal_radius: self.radius,
                vertical_radius: self.vertical_radius,
                rotation: 0.0,
            },
            ShapeOption::Polyline => Shape::Polyline {
                points: self.polyline_points.clone(),
            },
        };

        match self.mode {
            ModeOption::Fill => {
                mesh.fill(shape, self.color);
            }
            ModeOption::Stroke => {
                mesh.stroke(shape, self.color, self.stroke_width);
            }
        }

        mesh.draw(&mut frame.as_target());
    }
}

impl UserInterface for Example {
    type Message = Message;
    type Renderer = Renderer;

    fn react(&mut self, msg: Message) {
        match msg {
            Message::ShapeSelected(shape) => {
                self.shape = shape;
            }
            Message::ModeSelected(mode) => {
                self.mode = mode;
            }
            Message::StrokeWidthChanged(stroke_width) => {
                self.stroke_width = stroke_width;
            }
            Message::RadiusChanged(radius) => {
                self.radius = radius;
            }
            Message::VerticalRadiusChanged(radius) => {
                self.vertical_radius = radius;
            }
            Message::ColorChanged(color) => {
                self.color = color;
            }
        }
    }

    fn layout(&mut self, window: &Window) -> Element<Message> {
        let mut shape_and_mode = Column::new()
            .max_width(500)
            .spacing(20)
            .push(shape_selector(self.shape))
            .push(mode_selector(self.mode));

        match self.mode {
            ModeOption::Fill => {}
            ModeOption::Stroke => {
                shape_and_mode = shape_and_mode.push(stroke_width_slider(
                    &mut self.stroke_width_slider,
                    self.stroke_width,
                ));
            }
        }

        let mut controls = Column::new().max_width(500).spacing(20);

        match self.shape {
            ShapeOption::Rectangle => {}
            ShapeOption::Circle => {
                controls = controls.push(radius_slider(
                    "Radius:",
                    &mut self.radius_slider,
                    self.radius,
                    Message::RadiusChanged,
                ));
            }
            ShapeOption::Ellipse => {
                controls = controls
                    .push(radius_slider(
                        "Horizontal radius:",
                        &mut self.radius_slider,
                        self.radius,
                        Message::RadiusChanged,
                    ))
                    .push(radius_slider(
                        "Vertical radius:",
                        &mut self.vertical_radius_slider,
                        self.vertical_radius,
                        Message::VerticalRadiusChanged,
                    ));
            }
            ShapeOption::Polyline => {
                controls = controls.push(
                    Text::new("Click to draw!")
                        .size(40)
                        .horizontal_alignment(HorizontalAlignment::Center),
                )
            }
        }

        controls =
            controls.push(color_sliders(&mut self.color_sliders, self.color));

        Column::new()
            .width(window.width() as u32)
            .height(window.height() as u32)
            .padding(20)
            .align_items(Align::End)
            .justify_content(Justify::SpaceBetween)
            .push(shape_and_mode)
            .push(controls)
            .into()
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ShapeSelected(ShapeOption),
    ModeSelected(ModeOption),
    StrokeWidthChanged(u16),
    RadiusChanged(f32),
    VerticalRadiusChanged(f32),
    ColorChanged(Color),
}

fn shape_selector(current: ShapeOption) -> Element<'static, Message> {
    let options = [
        ShapeOption::Rectangle,
        ShapeOption::Circle,
        ShapeOption::Ellipse,
        ShapeOption::Polyline,
    ]
    .iter()
    .cloned()
    .fold(Column::new().padding(10).spacing(10), |container, shape| {
        container.push(Radio::new(
            shape,
            &format!("{:?}", shape),
            Some(current),
            Message::ShapeSelected,
        ))
    });

    Column::new()
        .spacing(10)
        .push(Text::new("Shape:"))
        .push(options)
        .into()
}

fn mode_selector(current: ModeOption) -> Element<'static, Message> {
    let options = [ModeOption::Fill, ModeOption::Stroke].iter().cloned().fold(
        Row::new().padding(10).spacing(10),
        |container, mode| {
            container.push(Radio::new(
                mode,
                &format!("{:?}", mode),
                Some(current),
                Message::ModeSelected,
            ))
        },
    );

    Column::new()
        .spacing(10)
        .push(Text::new("Mode:"))
        .push(options)
        .into()
}

fn radius_slider<'a>(
    label: &str,
    state: &'a mut slider::State,
    radius: f32,
    on_change: fn(f32) -> Message,
) -> Element<'a, Message> {
    slider_with_label(
        label,
        state,
        50.0..=200.0,
        radius,
        &format!("{:.*} px", 2, radius),
        on_change,
    )
}

fn stroke_width_slider(
    state: &mut slider::State,
    stroke_width: u16,
) -> Element<Message> {
    slider_with_label(
        "Stroke width:",
        state,
        1.0..=20.0,
        stroke_width as f32,
        &format!("{:.*} px", 2, stroke_width),
        |width| Message::StrokeWidthChanged(width.round() as u16),
    )
}

fn color_sliders(
    [red, green, blue]: &mut [slider::State; 3],
    color: Color,
) -> Element<Message> {
    Column::new()
        .spacing(10)
        .push(Text::new("Color:"))
        .push(
            Row::new()
                .spacing(10)
                .push(Slider::new(red, 0.0..=1.0, color.r, move |r| {
                    Message::ColorChanged(Color { r, ..color })
                }))
                .push(Slider::new(green, 0.0..=1.0, color.g, move |g| {
                    Message::ColorChanged(Color { g, ..color })
                }))
                .push(Slider::new(blue, 0.0..=1.0, color.b, move |b| {
                    Message::ColorChanged(Color { b, ..color })
                })),
        )
        .push(
            Text::new(&format!(
                "({:.*}, {:.*}, {:.*})",
                2, color.r, 2, color.g, 2, color.b
            ))
            .horizontal_alignment(HorizontalAlignment::Center),
        )
        .into()
}

fn slider_with_label<'a>(
    label: &str,
    state: &'a mut slider::State,
    range: RangeInclusive<f32>,
    value: f32,
    format: &str,
    on_change: fn(f32) -> Message,
) -> Element<'a, Message> {
    Column::new()
        .spacing(10)
        .push(Text::new(label))
        .push(
            Row::new()
                .spacing(10)
                .push(Slider::new(state, range, value, on_change))
                .push(
                    Text::new(format)
                        .width(150)
                        .height(50)
                        .horizontal_alignment(HorizontalAlignment::Center),
                ),
        )
        .into()
}
