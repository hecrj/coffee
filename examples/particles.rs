use rand::Rng;
use rayon::prelude::*;
use std::{thread, time};

use coffee::graphics::{
    Batch, Color, Font, Gpu, Image, Point, Rectangle, Result, Sprite, Text,
    Vector, Window, WindowSettings,
};
use coffee::input;
use coffee::load::{loading_screen, Join, LoadingScreen, Task};
use coffee::{Game, Timer};

fn main() {
    coffee::run::<Particles>(WindowSettings {
        title: String::from("Particles - Coffee"),
        size: (1280, 1024),
        resizable: false,
    })
    .unwrap();
}

struct Particles {
    particles: Vec<Particle>,
    gravity_center: Point,
}

impl Particles {
    const AMOUNT: u16 = 50_000;
    const G: f32 = 6.674;
    const MASS: f32 = 200.0;

    fn generate(max_x: f32, max_y: f32) -> Task<Particles> {
        Task::new(move || {
            let rng = &mut rand::thread_rng();

            let particles = (0..Self::AMOUNT)
                .map(|_| Particle::random(max_x, max_y, rng))
                .collect();

            Particles {
                particles,
                gravity_center: Point::new(0.0, 0.0),
            }
        })
    }
}

impl Game for Particles {
    type View = View;
    type Input = Input;

    // Low update rate.
    // This way, graphics interpolation is really noticeable when toggled!
    const TICKS_PER_SECOND: u16 = 20;

    fn new(window: &mut Window) -> (Particles, View, Input) {
        let task = (
            Task::stage(
                "Generating particles...",
                Particles::generate(window.width(), window.height()),
            ),
            Task::stage("Loading assets...", View::load()),
            Task::stage(
                "Showing off the loading screen for a bit...",
                Task::new(|| thread::sleep(time::Duration::from_secs(2))),
            ),
        )
            .join();

        let mut loading_screen = loading_screen::ProgressBar::new(window.gpu());
        let (particles, view, _) = loading_screen.run(task, window);

        (particles, view, Input::new())
    }

    fn on_input(&self, input: &mut Input, event: input::Event) {
        match event {
            input::Event::CursorMoved { x, y } => {
                input.cursor_position = Point::new(x, y);
            }
            input::Event::KeyboardInput {
                key_code,
                state: input::KeyState::Released,
            } => {
                input.released_keys.push(key_code);
            }
            _ => {}
        }
    }

    fn interact(&mut self, input: &mut Input, view: &mut View, _gpu: &mut Gpu) {
        self.gravity_center = input.cursor_position;

        for key in &input.released_keys {
            match key {
                input::KeyCode::I => {
                    view.interpolate = !view.interpolate;
                }
                _ => {}
            }
        }

        input.released_keys.clear();
    }

    fn update(&mut self, _view: &View, _window: &Window) {
        let gravity_center = self.gravity_center.clone();

        // Update particles in parallel! <3 rayon
        self.particles.par_iter_mut().for_each(move |particle| {
            let distance = particle.position - gravity_center;

            particle.acceleration = -((Self::G * Self::MASS)
                * distance.normalize())
                / distance.norm_squared().max(1000.0);

            particle.velocity += particle.acceleration;
            particle.position += particle.velocity;
        });
    }

    fn draw(
        &self,
        view: &mut View,
        window: &mut Window,
        timer: &Timer,
    ) -> Result<()> {
        let mut frame = window.frame();
        frame.clear(Color::BLACK);

        // Draw particles all at once!
        let mut batch = Batch::new(view.palette.clone());

        let delta_factor = if view.interpolate {
            timer.next_tick_proximity()
        } else {
            0.0
        };

        for particle in &self.particles {
            let velocity =
                particle.velocity + particle.acceleration * delta_factor;

            batch.add(Sprite {
                source: Rectangle {
                    x: View::particle_color(velocity),
                    y: 0,
                    width: 1,
                    height: 1,
                },
                position: particle.position + velocity * delta_factor,
            });
        }

        batch.draw(Point::new(0.0, 0.0), &mut frame.as_target());

        // Draw simple text UI
        view.font.add(Text {
            content: String::from("Graphics interpolation:"),
            position: Point::new(10.0, frame.height() - 50.0),
            bounds: (frame.width(), frame.height()),
            size: 20.0,
            color: Color::WHITE,
        });

        view.font.add(Text {
            content: if view.interpolate {
                String::from("ON")
            } else {
                String::from("OFF")
            },
            position: Point::new(250.0, frame.height() - 50.0),
            bounds: (frame.width(), frame.height()),
            size: 20.0,
            color: if view.interpolate {
                Color::new(0.0, 1.0, 0.0, 1.0)
            } else {
                Color::new(1.0, 0.0, 0.0, 1.0)
            },
        });

        view.font.add(Text {
            content: String::from("Press I to toggle."),
            position: Point::new(10.0, frame.height() - 25.0),
            bounds: (frame.width(), frame.height()),
            size: 16.0,
            color: Color::WHITE,
        });

        view.font.draw(&mut frame);

        Ok(())
    }
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

struct View {
    palette: Image,
    font: Font,
    interpolate: bool,
}

impl View {
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

    fn load() -> Task<View> {
        (
            Task::using_gpu(|gpu| {
                Image::from_colors(gpu, &Self::COLORS).unwrap()
            }),
            Font::load(include_bytes!(
                "../resources/font/Inconsolata-Regular.ttf"
            )),
        )
            .join()
            .map(|(palette, font)| View {
                palette,
                font,
                interpolate: true,
            })
    }

    fn particle_color(velocity: Vector) -> u16 {
        ((velocity.norm() * 2.0).round() as usize).min(View::COLORS.len())
            as u16
    }
}

struct Input {
    cursor_position: Point,
    released_keys: Vec<input::KeyCode>,
}

impl Input {
    fn new() -> Input {
        Input {
            cursor_position: Point::new(0.0, 0.0),
            released_keys: Vec::new(),
        }
    }
}
