//! A particle gravity simulator that showcases a loading screen, input
//! handling, and graphics interpolation with batched drawing and font
//! rendering. Move the mouse around to attract the particles.
use rand::Rng;
use rayon::prelude::*;
use std::{thread, time};

use coffee::graphics::{
    Batch, Color, Frame, Image, Point, Rectangle, Sprite, Vector, Window,
    WindowSettings,
};
use coffee::input::{keyboard, mouse, KeyboardAndMouse};
use coffee::load::{loading_screen::ProgressBar, Join, Task};
use coffee::ui::{Checkbox, Column, Element, Justify, Renderer, UserInterface};
use coffee::{Game, Result, Timer};

fn main() -> Result<()> {
    <Particles as UserInterface>::run(WindowSettings {
        title: String::from("Particles - Coffee"),
        size: (1280, 1024),
        resizable: false,
        fullscreen: false,
        maximized: false,
        vsync: false,
    })
}

struct Particles {
    particles: Vec<Particle>,
    gravity_centers: Vec<Point>,

    batch: Batch,
    interpolate: bool,
}

impl Particles {
    // Try increasing this value! I (@hecrj) can render 1 MILLION particles at
    // 90 fps on my system (4790k, GTX 980, Windows 7) using Vulkan.
    const AMOUNT: usize = 50_000;

    // Play with these values to alter the way gravity works.
    const G: f32 = 6.674;
    const CENTER_MASS: f32 = 200.0;

    fn generate(max_x: f32, max_y: f32) -> Task<Vec<Particle>> {
        Task::succeed(move || {
            let rng = &mut rand::thread_rng();

            (0..Self::AMOUNT)
                .map(|_| Particle::random(max_x, max_y, rng))
                .collect()
        })
    }

    fn load_palette() -> Task<Image> {
        Task::using_gpu(|gpu| Image::from_colors(gpu, &COLORS))
    }

    fn particle_color(velocity: Vector) -> u16 {
        ((velocity.norm() * 2.0) as usize).min(COLORS.len()) as u16
    }
}

impl Game for Particles {
    type Input = KeyboardAndMouse;
    type LoadingScreen = ProgressBar;

    // Low update rate.
    // This makes graphics interpolation really noticeable when toggled.
    const TICKS_PER_SECOND: u16 = 20;

    fn load(window: &Window) -> Task<Particles> {
        (
            Task::stage(
                "Generating particles...",
                Self::generate(window.width(), window.height()),
            ),
            Task::stage("Loading assets...", Self::load_palette()),
            Task::stage(
                "Showing off the loading screen for a bit...",
                Task::succeed(|| thread::sleep(time::Duration::from_secs(2))),
            ),
        )
            .join()
            .map(|(particles, palette, _)| Particles {
                particles,
                gravity_centers: vec![Point::new(0.0, 0.0)],
                batch: Batch::new(palette),
                interpolate: true,
            })
    }

    fn interact(&mut self, input: &mut KeyboardAndMouse, window: &mut Window) {
        let mouse = input.mouse();
        let keyboard = input.keyboard();

        self.gravity_centers[0] = mouse.cursor_position();
        self.gravity_centers
            .extend(mouse.button_clicks(mouse::Button::Left));

        if keyboard.was_key_released(keyboard::KeyCode::I) {
            self.interpolate = !self.interpolate;
        }

        if keyboard.was_key_released(keyboard::KeyCode::F) {
            window.toggle_fullscreen();
        }
    }

    fn update(&mut self, _window: &Window) {
        let gravity_centers = self.gravity_centers.clone();

        // Update particles in parallel! <3 rayon
        self.particles.par_iter_mut().for_each(move |particle| {
            particle.acceleration = gravity_centers
                .iter()
                .map(|gravity_center| {
                    let distance = particle.position - gravity_center;

                    -((Self::G * Self::CENTER_MASS) * distance.normalize())
                        / distance.norm_squared().max(1000.0)
                })
                .sum();

            particle.velocity += particle.acceleration;
            particle.position += particle.velocity;
        });
    }

    fn draw(&mut self, frame: &mut Frame, timer: &Timer) {
        frame.clear(Color::BLACK);

        // When interpolating, we need to know how close the next tick is
        let delta_factor = if self.interpolate {
            timer.next_tick_proximity()
        } else {
            0.0
        };

        // Generate sprites in parallel! <3 rayon
        let sprites = self.particles.par_iter().map(|particle| {
            let velocity =
                particle.velocity + particle.acceleration * delta_factor;

            Sprite {
                source: Rectangle {
                    x: Self::particle_color(velocity),
                    y: 0,
                    width: 1,
                    height: 1,
                },
                position: particle.position + velocity * delta_factor,
                scale: (1.0, 1.0),
            }
        });

        // Clear batch contents from previous frame
        self.batch.clear();

        // Use the parallel iterator to populate the batch efficiently
        self.batch.par_extend(sprites);

        // Draw particles all at once!
        self.batch.draw(&mut frame.as_target());
    }
}

impl UserInterface for Particles {
    type Message = Message;
    type Renderer = Renderer;

    fn react(&mut self, msg: Message, _window: &mut Window) {
        match msg {
            Message::ToggleInterpolation(interpolate) => {
                self.interpolate = interpolate;
            }
        }
    }

    fn layout(&mut self, window: &Window) -> Element<Message> {
        Column::new()
            .padding(20)
            .spacing(20)
            .width(window.width() as u32)
            .height(window.height() as u32)
            .justify_content(Justify::End)
            .push(Checkbox::new(
                self.interpolate,
                "Graphics interpolation",
                Message::ToggleInterpolation,
            ))
            .into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ToggleInterpolation(bool),
}

#[derive(Clone)]
struct Particle {
    position: Point,
    velocity: Vector,
    acceleration: Vector,
}

impl Particle {
    fn random<R: Rng>(max_x: f32, max_y: f32, rng: &mut R) -> Particle {
        Particle {
            position: Point::new(
                rng.gen_range(0.0, max_x),
                rng.gen_range(0.0, max_y),
            ),
            velocity: Vector::new(0.0, 0.0),
            acceleration: Vector::new(0.0, 0.0),
        }
    }
}

const COLORS: [Color; 7] = [
    Color {
        r: 0.4,
        g: 0.4,
        b: 0.4,
        a: 1.0,
    },
    Color {
        r: 0.5,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    },
    Color {
        r: 0.6,
        g: 0.6,
        b: 0.6,
        a: 1.0,
    },
    Color {
        r: 0.7,
        g: 0.7,
        b: 0.7,
        a: 1.0,
    },
    Color {
        r: 0.8,
        g: 0.8,
        b: 0.8,
        a: 1.0,
    },
    Color {
        r: 0.9,
        g: 0.9,
        b: 0.9,
        a: 1.0,
    },
    Color {
        r: 0.8,
        g: 0.8,
        b: 1.0,
        a: 1.0,
    },
];
