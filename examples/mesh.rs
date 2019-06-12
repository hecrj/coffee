use coffee::graphics::{
    Color, Frame, HorizontalAlignment, Mesh, Point, Shape, Window,
    WindowSettings,
};
use coffee::load::Task;
use coffee::ui::{
    slider, Align, Column, Element, Justify, Radio, Renderer, Row, Slider, Text,
};
use coffee::{Game, Result, Timer, UserInterface};

use std::ops::RangeInclusive;

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
    color: Color,
    radius: f32,
    stroke_width: u16,

    radius_slider: slider::State,
    color_sliders: [slider::State; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShapeOption {
    Circle,
}

enum ModeOption {
    Fill,
    Stroke,
}

impl Game for Example {
    type Input = ();
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<Example> {
        Task::new(move || Example {
            shape: ShapeOption::Circle,
            mode: ModeOption::Fill,
            color: Color::WHITE,
            radius: 100.0,
            stroke_width: 2,

            radius_slider: slider::State::new(),
            color_sliders: [slider::State::new(); 3],
        })
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
            ShapeOption::Circle => Shape::Circle {
                center: Point::new(frame.width() / 2.0, frame.height() / 2.0),
                radius: self.radius,
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
            Message::RadiusChanged(radius) => {
                self.radius = radius;
            }
            Message::ColorChanged(color) => {
                self.color = color;
            }
            Message::ShapeSelected(shape) => {
                self.shape = shape;
            }
        }
    }

    fn layout(&mut self, window: &Window) -> Element<Message> {
        let mut controls = Column::new()
            .max_width(500)
            .spacing(20)
            .push(shape_selector(self.shape));

        match self.shape {
            ShapeOption::Circle => {
                controls = controls
                    .push(radius_slider(&mut self.radius_slider, self.radius));
            }
        }

        controls =
            controls.push(color_sliders(&mut self.color_sliders, self.color));

        Column::new()
            .width(window.width() as u32)
            .height(window.height() as u32)
            .padding(20)
            .align_items(Align::End)
            .justify_content(Justify::End)
            .push(controls)
            .into()
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RadiusChanged(f32),
    ColorChanged(Color),
    ShapeSelected(ShapeOption),
}

fn shape_selector(current: ShapeOption) -> Element<'static, Message> {
    let options = [ShapeOption::Circle].iter().cloned().fold(
        Column::new().padding(10).spacing(10),
        |container, shape| {
            container.push(Radio::new(
                shape,
                &format!("{:?}", shape),
                Some(current),
                Message::ShapeSelected,
            ))
        },
    );

    Column::new()
        .spacing(10)
        .push(Text::new("Shape:"))
        .push(options)
        .into()
}

fn radius_slider(state: &mut slider::State, radius: f32) -> Element<Message> {
    slider_with_label(
        "Radius:",
        state,
        50.0..=200.0,
        radius,
        &format!("{:.*} px", 2, radius),
        Message::RadiusChanged,
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
