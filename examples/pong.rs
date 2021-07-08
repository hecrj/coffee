extern crate coffee;

use coffee::graphics::{Color, Font, Frame, Mesh, Point, Rectangle, Shape, Text, Window, WindowSettings};
use coffee::input::keyboard::{Keyboard, KeyCode};
use coffee::load::Task;
use coffee::{Game, Timer};

// Load game with stated parameters
fn main() {
    PongGame::run(WindowSettings {
        title: String::from("Pong"),
        size: (900, 600),
        resizable: false,
        fullscreen: false,
        maximized: false,
    })
    .expect("An error occured while starting the game");
}

// Define the paddles
struct Paddle {
    pos: (f32, f32)
}

impl Paddle {
    fn new_l() -> Paddle { 
        Paddle {pos: (50.0,225.0)}
    }

    fn new_r() -> Paddle {
        Paddle {pos: (830.0,225.0)}
    }

    // Define movement for the left paddle
    fn move_l(p: &Paddle, key: KeyCode) -> Paddle {
        let mut new_p = Paddle{pos: (p.pos.0, p.pos.1)};
        if p.pos.1 > 0.0 && p.pos.1 < 450.0 {
            if key == KeyCode::W {
                new_p.pos.1 -= 5.0;
            } else if key == KeyCode::S {
                new_p.pos.1 += 5.0;
            }
        } else if p.pos.1 == 0.0 {
            if key == KeyCode::S {
                new_p.pos.1 += 5.0;
            }
        } else if p.pos.1 == 450.0 {
            if key == KeyCode::W {
                new_p.pos.1 -= 5.0;
            }
        }
        new_p 
    }

    // Define movement for the right paddle
    fn move_r(p: &Paddle, key: KeyCode) -> Paddle {
        let mut new_p = Paddle{pos: (p.pos.0, p.pos.1)};
        if p.pos.1 > 0.0 && p.pos.1 < 450.0 {
            if key == KeyCode::Up {
                new_p.pos.1 -= 5.0;
            } else if key == KeyCode::Down {
                new_p.pos.1 += 5.0;
            }
        } else if p.pos.1 == 0.0 {
            if key == KeyCode::Down {
                new_p.pos.1 += 5.0;
            }
        } else if p.pos.1 == 450.0 {
            if key == KeyCode::Up {
                new_p.pos.1 -= 5.0;
            }
        }
        new_p 
    }
}

// Define movement for the ball
struct Ball {
    pos: (f32, f32),
    fly: (f32, f32),
    speed: f32,
}

impl Ball {
    fn new() -> Ball {
        Ball {
            pos: (440.0, 290.0),
            fly: (-1.0, 0.0),
            speed: 3.0,
        }
    }

    // Describes movement of the ball
    fn update(b: &Ball) -> Ball {
        let new_b = Ball {
            pos: ((b.pos.0 + b.fly.0 * b.speed), (b.pos.1 + b.fly.1 * b.speed)),
            fly: (-1.0, 0.0), // TODO update this based on whether there was a collision
            speed: 3.0, // TODO update this based on paddle collision
        };
        new_b
    }
}

// How to keep score, game ends a player gets to 10 points
struct Score {
    l_score: i8,
    r_score: i8,
}

impl Score {
    fn new(l: i8, r: i8) -> Score {
        Score {
            l_score: l,
            r_score: r,
        }
    }

    fn update(l_winner: bool, score: &Score) -> Score {
        let mut new_score = Score {
            l_score: score.l_score,
            r_score: score.r_score,
        };
        if l_winner {
            new_score = Score {
            l_score: score.l_score + 1,
            r_score: score.r_score,
            };
        } else {
            new_score = Score {
            l_score: score.l_score,
            r_score: score.r_score + 1,
            };
        }
        new_score
    }
}

struct PongGame {
    l_paddle: Paddle,
    r_paddle: Paddle,
    ball: Ball,
    score: Score,
}

impl Game for PongGame {
    type Input = Keyboard;
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<PongGame> {
        let mut l_paddle = Paddle::new_l();
        let mut r_paddle = Paddle::new_r();
        let mut ball = Ball::new();
        let mut score = Score::new(0,0);
        Task::succeed(|| PongGame {
            l_paddle: l_paddle,
            r_paddle: r_paddle,
            ball: ball,
            score: score,
        })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        // Clear the frame
        frame.clear(Color::BLACK);
        // Load the font
        let mut font = Font::from_bytes(
            frame.gpu(),
            include_bytes!("../resources/font/Inconsolata-Regular.ttf"),
        )
        .expect("Font has failed to load");

        // Write score
        let score_text = format!(
            "{}|{}",
            self.score.l_score,
            self.score.r_score,
        );
        font.add(Text{
            content: &score_text,
            position: Point::new(415.0, 10.0),
            size: 50.0,
            color: Color::WHITE,
            ..Text::default()
        });
        font.draw(&mut frame.as_target());

        // Draw left paddle
        let mut lp_mesh = Mesh::new();
        lp_mesh.fill(
            Shape::Rectangle(Rectangle {
                x: self.l_paddle.pos.0,
                y: self.l_paddle.pos.1,
                width: 20.0,
                height: 150.0,
            }),
            Color::WHITE,
        );
        lp_mesh.draw(&mut frame.as_target());

        // Draw right paddle
        let mut rp_mesh = Mesh::new();
        rp_mesh.fill(
            Shape::Rectangle(Rectangle {
                x: self.r_paddle.pos.0,
                y: self.r_paddle.pos.1,
                width: 20.0,
                height: 150.0,
            }),
            Color::WHITE,
        );
        rp_mesh.draw(&mut frame.as_target());

        // Draw ball
        let mut b_mesh = Mesh::new();
        b_mesh.fill(
            Shape::Rectangle(Rectangle {
                x: self.ball.pos.0,
                y: self.ball.pos.1,
                width: 20.0,
                height: 20.0,
            }),
            Color::WHITE,
        );
        b_mesh.draw(&mut frame.as_target());
    }
    
    fn interact(&mut self, input: &mut Self::Input, _window: &mut Window) {
        if input.is_key_pressed(KeyCode::W) {
            self.l_paddle = Paddle::move_l(&self.l_paddle, KeyCode::W);
        } else if input.is_key_pressed(KeyCode::S) {
            self.l_paddle = Paddle::move_l(&self.l_paddle, KeyCode::S);
        } else if input.is_key_pressed(KeyCode::Up) {
            self.r_paddle = Paddle::move_r(&self.r_paddle, KeyCode::Up);
        } else if input.is_key_pressed(KeyCode::Down) {
            self.r_paddle = Paddle::move_r(&self.r_paddle, KeyCode::Down);
        }
    }

    fn update(&mut self, _window: &Window) {
        self.ball = Ball::update(&self.ball);

        // Update score if it goes off the edges and reset ball
        if self.ball.pos.0 < 0.0 {
            self.ball = Ball::new();
            self.score = Score::update(true, &self.score)
        } else if self.ball.pos.0 > 600.0 {
            self.ball = Ball::new();
            self.score = Score::update(false, &self.score)
        }
    }
}