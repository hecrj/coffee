//! A particle gravity simulator that showcases a loading screen, input
//! handling, and graphics interpolation with batched drawing and font
//! rendering. Move the mouse around to attract the particles.
use rand::Rng;
use rayon::prelude::*;

use coffee::graphics::{
    Batch, Color, Frame, Image, Point, Rectangle, Sprite, Vector, Window,
    WindowSettings,
};
use coffee::input::{KeyCode, KeyboardAndMouse};
use coffee::load::{loading_screen::ProgressBar, Task};
use coffee::ui::{Checkbox, Column, Element, Justify, Renderer};
use coffee::{Game, Result, Timer, UserInterface};

// Try increasing this value! I (@hecrj) can render 1 MILLION particles at
// 90 fps on my system (4790k, GTX 980, Windows 7) using Vulkan.
const AMOUNT: u32 = 50_000;

// Play with these values to alter the way gravity works.
const G: f32 = 6.674;
const CENTER_MASS: f32 = 200.0;

fn main() -> Result<()> {
    <Particles as UserInterface>::run(WindowSettings {
        title: String::from("Particles - Coffee"),
        size: (1280, 1024),
        resizable: false,
        fullscreen: false,
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
    interpolate: bool,
}

impl Particles {
    fn load() -> Task<Particles> {
        Task::using_gpu(|gpu| Image::from_colors(gpu, &COLORS)).map(|palette| {
            Particles {
                batch: Batch::new(palette),
                interpolate: true,
            }
        })
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
        window: &mut Window,
    ) {
        state.gravity_centers[0] = input.cursor_position();

        for point in input.clicks() {
            state.gravity_centers.push(*point);
        }

        if input.was_key_released(&KeyCode::F) {
            window.toggle_fullscreen();
        }
    }

    fn draw(&mut self, state: &State, frame: &mut Frame, timer: &Timer) {
        frame.clear(Color::BLACK);

        // When interpolating, we need to know how close the next tick is
        let delta_factor = if self.interpolate {
            timer.next_tick_proximity()
        } else {
            0.0
        };

        // Generate sprites in parallel! <3 rayon
        let sprites = state.particles.par_iter().map(|particle| {
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
        self.batch
            .draw(Point::new(0.0, 0.0), &mut frame.as_target());
    }
}

impl UserInterface for Particles {
    type Message = Message;
    type Renderer = Renderer;

    fn update(&mut self, _state: &mut State, msg: Message) {
        match msg {
            Message::ToggleInterpolation(interpolate) => {
                self.interpolate = interpolate;
            }
        }
    }

    fn layout(&mut self, _state: &State, window: &Window) -> Element<Message> {
        Column::new()
            .padding(20)
            .spacing(20)
            .width(window.width())
            .height(window.height())
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
