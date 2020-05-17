extern crate coffee;

use coffee::graphics::{
    Color, Font, Frame, Mesh, Point, Rectangle, Shape, Text, Window,
    WindowSettings,
};
use coffee::input::keyboard::KeyCode;
use coffee::input::{self, keyboard, Input};

use coffee::load::Task;
use coffee::{Game, Timer};

use rand::seq::IteratorRandom;

fn main() {
    SnakeGame::run(WindowSettings {
        title: String::from("Snake"),
        size: (900, 600),
        resizable: false,
        maximized: false,
        fullscreen: false,
        vsync: false,
    })
    .expect("An error occured while starting the game");
}

fn new_random_pos() -> (f32, f32) {
    let mut rng = rand::thread_rng();
    let x = (0..900).step_by(30).choose(&mut rng).unwrap() as f32;
    let y = (0..600).step_by(30).choose(&mut rng).unwrap() as f32;
    return (x, y);
}
#[derive(Debug, Clone, Copy, PartialEq)]
struct Position(f32, f32);

struct Snake {
    square_pos: Vec<Position>,
    direction: Option<KeyCode>,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            square_pos: vec![],
            direction: None,
        }
    }
    fn create_snake(&mut self) {
        for i in 1..6 {
            let x = (30.0 * i as f32) + 30.0;
            let y = 90.0;
            self.square_pos.push(Position(x, y));
        }
    }
    fn draw_snake(&mut self, frame: &mut Frame) {
        let mut mesh = Mesh::new();
        for pos in &self.square_pos {
            mesh.fill(
                Shape::Rectangle(Rectangle {
                    x: pos.0,
                    y: pos.1,
                    width: 30.0,
                    height: 30.0,
                }),
                Color::RED,
            );
        }
        mesh.draw(&mut frame.as_target());
    }
    fn add_queue(&mut self) {
        match self.direction {
            Some(KeyCode::Right) => {
                let x = self.square_pos.last().unwrap().0 + 30.0;
                let y = self.square_pos.last().unwrap().1;
                self.square_pos.push(Position(x, y));
            }
            Some(KeyCode::Left) => {
                let x = self.square_pos.last().unwrap().0 - 30.0;
                let y = self.square_pos.last().unwrap().1;
                self.square_pos.push(Position(x, y));
            }
            Some(KeyCode::Down) => {
                let x = self.square_pos.last().unwrap().0;
                let y = self.square_pos.last().unwrap().1 + 30.0;
                self.square_pos.push(Position(x, y));
            }
            Some(KeyCode::Up) => {
                let x = self.square_pos.last().unwrap().0;
                let y = self.square_pos.last().unwrap().1 - 30.0;
                self.square_pos.push(Position(x, y));
            }
            _ => (),
        }
    }
    fn move_right(&mut self) {
        self.square_pos.remove(0);
        let head = self.square_pos.last().unwrap().clone();
        if head.0 != 870.0 {
            self.square_pos.push(Position(head.0 + 30.0, head.1));
        } else {
            self.square_pos.push(Position(0.0, head.1));
        }
    }
    fn move_left(&mut self) {
        self.square_pos.remove(0);
        let head = self.square_pos.last().unwrap().clone();
        if head.0 != 0.0 {
            self.square_pos.push(Position(head.0 - 30.0, head.1));
        } else {
            self.square_pos.push(Position(870.0, head.1));
        }
    }
    fn move_bottom(&mut self) {
        self.square_pos.remove(0);
        let head = self.square_pos.last().unwrap().clone();
        if head.1 != 570.0 {
            self.square_pos.push(Position(head.0, head.1 + 30.0));
        } else {
            self.square_pos.push(Position(head.0, 0.0));
        }
    }
    fn move_top(&mut self) {
        self.square_pos.remove(0);
        let head = self.square_pos.last().unwrap().clone();
        if head.1 != 0.0 {
            self.square_pos.push(Position(head.0, head.1 - 30.0));
        } else {
            self.square_pos.push(Position(head.0, 570.0));
        }
    }
    fn move_to(&mut self, keycode: Option<KeyCode>) {
        self.direction = keycode;
        match keycode {
            Some(KeyCode::Right) => {
                self.move_right();
            }
            Some(KeyCode::Left) => {
                self.move_left();
            }
            Some(KeyCode::Down) => {
                self.move_bottom();
            }
            Some(KeyCode::Up) => {
                self.move_top();
            }
            _ => (),
        }
    }
    fn ate_apple(&self, apple_pos: Position) -> bool {
        if apple_pos == *self.square_pos.last().unwrap() {
            return true;
        }
        false
    }
    fn ate_himself(&self) -> bool {
        for i in &self.square_pos[..self.square_pos.len() - 1] {
            if *i == *self.square_pos.last().unwrap() {
                return true;
            }
        }
        false
    }
}

struct Apple {
    pub pos: Position,
    eaten: bool,
}

impl Apple {
    fn new() -> Apple {
        let (x, y) = new_random_pos();
        Apple {
            pos: Position(x, y),
            eaten: false,
        }
    }
    fn draw(&mut self, frame: &mut Frame) {
        let mut mesh = Mesh::new();
        mesh.fill(
            Shape::Rectangle(Rectangle {
                x: self.get_pos().0,
                y: self.get_pos().1,
                width: 30.0,
                height: 30.0,
            }),
            Color::GREEN,
        );
        mesh.draw(&mut frame.as_target());
    }
    fn get_pos(&mut self) -> Position {
        if self.eaten {
            let (x, y) = new_random_pos();
            self.pos = Position(x, y);
            self.eaten = false;
        }
        self.pos
    }
}
struct SnakeGame {
    snake: Snake,
    last_key: Option<KeyCode>,
    apple: Apple,
    score: u16,
    ticks: u8,
    speed: u8,
}
pub enum ButtonState {
    PlayAgain,
}
impl Game for SnakeGame {
    const TICKS_PER_SECOND: u16 = 60;
    type Input = CustomInput;
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<SnakeGame> {
        let mut snake = Snake::new();
        snake.create_snake();
        Task::succeed(|| SnakeGame {
            snake,
            score: 0,
            apple: Apple::new(),
            ticks: 0,
            speed: 20,
            last_key: None,
        })
    }
    fn update(&mut self, _window: &Window) {}
    fn interact(&mut self, input: &mut CustomInput, _window: &mut Window) {
        if input.keys_pressed.len() != 0 {
            let key = input.keys_pressed[0];
            match key {
                KeyCode::Right => {
                    self.last_key = Some(key);
                }
                KeyCode::Left => {
                    self.last_key = Some(key);
                }
                KeyCode::Down => {
                    self.last_key = Some(key);
                }
                KeyCode::Up => {
                    self.last_key = Some(key);
                }
                _ => (),
            }
        }
    }
    fn draw(&mut self, frame: &mut Frame, timer: &Timer) {
        if timer.has_ticked() && !self.snake.ate_himself() {
            self.ticks += 1;
            if self.ticks == self.speed {
                // We don't want our snake to move 60 times per seconds.
                self.snake.move_to(self.last_key);
                if self.snake.ate_apple(self.apple.get_pos()) {
                    self.apple.eaten = true;
                    self.score += 1;
                    self.snake.add_queue();
                    if self.score % 2 == 0 {
                        self.speed -= 1; // Every two apples, the snake go faster !
                    }
                }
                self.ticks = 0;
            }
        }
        if !self.snake.ate_himself() {
            frame.clear(Color::new(0.5, 0.5, 0.5, 1.0));
            self.snake.draw_snake(frame);
            self.apple.draw(frame);
        } else {
            frame.clear(Color::from_rgb(255, 0, 0));
            let mut font = Font::from_bytes(
                frame.gpu(),
                include_bytes!("../resources/font/Inconsolata-Regular.ttf"),
            )
            .expect("Failed to load font");

            font.add(Text {
                content: "You lost",
                position: Point::new(330.0, 300.0),
                size: 50.0,
                color: Color::WHITE,
                ..Text::default()
            });
            font.add(Text {
                content: format!("Your score is: {}", self.score).as_str(),
                position: Point::new(270.0, 200.0),
                size: 50.0,
                color: Color::WHITE,
                ..Text::default()
            });
            font.draw(&mut frame.as_target());
        }
    }
}

struct CustomInput {
    keys_pressed: Vec<KeyCode>,
}

impl Input for CustomInput {
    fn new() -> CustomInput {
        CustomInput {
            keys_pressed: vec![],
        }
    }

    fn update(&mut self, event: input::Event) {
        match event {
            input::Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::Input { key_code, state } => match state {
                    input::ButtonState::Pressed => {
                        self.keys_pressed.push(key_code);
                    }
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
    }

    fn clear(&mut self) {
        self.keys_pressed.clear();
    }
}
