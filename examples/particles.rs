//! A particle gravity simulator that showcases a loading screen, input
//! handling, and graphics interpolation with batched drawing and font
//! rendering. Move the mouse around to attract the particles.
use rand::Rng;
use rayon::prelude::*;

use coffee::graphics::{
    Batch, Color, Font, Gpu, Image, Point, Rectangle, Sprite, Text, Vector,
    Window, WindowSettings,
};
use coffee::input::{KeyCode, KeyboardAndMouse};
use coffee::load::{loading_screen::ProgressBar, Join, Task};
use coffee::{Game, Result, Timer};

// Try increasing this value! I (@hecrj) can render 350k particles at 100fps
// on my system. I have not tried going above that, yet...
const AMOUNT: u32 = 50_000;

// Play with these values to alter the way gravity works.
const G: f32 = 6.674;
const CENTER_MASS: f32 = 200.0;

fn main() -> Result<()> {
    Particles::run(WindowSettings {
        title: String::from("Particles - Coffee"),
        size: (1280, 1024),
        resizable: false,
    })
}

struct State {
    particles: Vec<Particle>,
    gravity_centers: Vec<Point>,
}

impl State {
    fn generate(max_x: f32, max_y: f32) -> Task<State> {
        Task::new(move || {
            let rng = &mut rand::thread_rng();

            let particles = (0..AMOUNT)
                .map(|_| Particle::random(max_x, max_y, rng))
                .collect();

            State {
                particles,
                gravity_centers: vec![Point::new(0.0, 0.0)],
            }
        })
    }
}

impl coffee::State for State {
    // Low update rate.
    // This makes graphics interpolation really noticeable when toggled.
    const TICKS_PER_SECOND: u16 = 20;

    fn load(window: &Window) -> Task<State> {
        Task::stage(
            "Generating particles...",
            State::generate(window.width(), window.height()),
        )
    }

    fn update(&mut self) {
        let gravity_centers = self.gravity_centers.clone();

        // Update particles in parallel! <3 rayon
        self.particles.par_iter_mut().for_each(move |particle| {
            particle.acceleration = gravity_centers
                .iter()
                .map(|gravity_center| {
                    let distance = particle.position - gravity_center;
                    -((G * CENTER_MASS) * distance.normalize())
                        / distance.norm_squared().max(1000.0)
                })
                .sum();

            particle.velocity += particle.acceleration;
            particle.position += particle.velocity;
        });
    }
}

struct Particles {
    batch: Batch,
    font: Font,
    interpolate: bool,
}

impl Particles {
    fn load() -> Task<Particles> {
        (Self::load_palette(), Self::load_font()).join().map(
            |(palette, font)| Particles {
                batch: Batch::new(palette),
                font,
                interpolate: true,
            },
        )
    }

    fn load_palette() -> Task<Image> {
        Task::using_gpu(|gpu| Image::from_colors(gpu, &COLORS))
    }

    fn load_font() -> Task<Font> {
        Font::load(include_bytes!("../resources/font/Inconsolata-Regular.ttf"))
    }

    fn particle_color(velocity: Vector) -> u16 {
        ((velocity.norm() * 2.0) as usize).min(COLORS.len()) as u16
    }
}

impl Game for Particles {
    type State = State;
    type Input = KeyboardAndMouse;
    type LoadingScreen = ProgressBar;

    fn load(_window: &Window) -> Task<Particles> {
        Task::stage("Loading assets...", Particles::load())
    }

    fn interact(
        &mut self,
        input: &mut KeyboardAndMouse,
        state: &mut State,
        _gpu: &mut Gpu,
    ) {
        state.gravity_centers[0] = input.cursor_position();

        for point in input.clicks() {
            state.gravity_centers.push(*point);
        }

        if input.was_key_released(&KeyCode::I) {
            self.interpolate = !self.interpolate;
        }
    }

    fn draw(&mut self, state: &State, window: &mut Window, timer: &Timer) {
        let mut frame = window.frame();
        frame.clear(Color::BLACK);

        // Draw particles all at once!
        self.batch.clear();

        let delta_factor = if self.interpolate {
            timer.next_tick_proximity()
        } else {
            0.0
        };

        for particle in &state.particles {
            let velocity =
                particle.velocity + particle.acceleration * delta_factor;

            self.batch.add(Sprite {
                source: Rectangle {
                    x: Self::particle_color(velocity),
                    y: 0,
                    width: 1,
                    height: 1,
                },
                position: particle.position + velocity * delta_factor,
            });
        }

        self.batch
            .draw(Point::new(0.0, 0.0), &mut frame.as_target());

        // Draw simple text UI
        self.font.add(Text {
            content: "Graphics interpolation:",
            position: Point::new(10.0, frame.height() - 50.0),
            bounds: (frame.width(), frame.height()),
            size: 20.0,
            color: Color::WHITE,
        });

        self.font.add(Text {
            content: if self.interpolate { "ON" } else { "OFF" },
            position: Point::new(250.0, frame.height() - 50.0),
            bounds: (frame.width(), frame.height()),
            size: 20.0,
            color: if self.interpolate {
                Color::new(0.0, 1.0, 0.0, 1.0)
            } else {
                Color::new(1.0, 0.0, 0.0, 1.0)
            },
        });

        self.font.add(Text {
            content: "Press I to toggle.",
            position: Point::new(10.0, frame.height() - 25.0),
            bounds: (frame.width(), frame.height()),
            size: 16.0,
            color: Color::WHITE,
        });

        self.font.draw(&mut frame.as_target());
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
