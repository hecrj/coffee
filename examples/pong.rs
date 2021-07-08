extern crate coffee;

use coffee::graphics::{Color, Font, Frame, Mesh, Point, Rectangle, Shape, Text, Window, WindowSettings};
use coffee::input::keyboard::Keyboard;
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

struct Position(f32, f32);

// Define the paddles
struct Paddle {
    pos: [Position; 2]
}

impl Paddle {
    fn new_l() -> Paddle { 
        Paddle {pos: [Position(1.0,10.0), Position(1.0,20.0)]}
    }
    fn new_r() -> Paddle {
        Paddle {pos: [Position(50.0,10.0), Position(50.0,20.0)]}
    }
}

// Define movement for the ball
struct Ball {
    pos: Position,
    speed: i8,
}

impl Ball {
    fn new() -> Ball {
        Ball {
            pos: Position(30.0,15.0),
            speed: 3,
        }
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
            position: Point::new(450.0, 50.0),
            size: 50.0,
            color: Color::WHITE,
            ..Text::default()
        });
        font.draw(&mut frame.as_target());
        )
    }
}