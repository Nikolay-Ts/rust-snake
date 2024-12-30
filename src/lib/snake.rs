use super::tm_logic;
use crossterm::{
    cursor, execute,
    style::Print,
    terminal::{Clear, ClearType},
};
use rand::Rng;
use std::collections::VecDeque;
use std::io::stdout;

#[derive(PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Clone, Copy)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl PartialEq for Point {
    // overload for ==
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub struct SnakeGame {
    snake: VecDeque<Point>,
    pub direction: Direction,
    pub game_over: bool,
    score: u16,
    growing: bool,
    food: Point,
    height: u16,
    width: u16,
}

impl SnakeGame {
    pub fn init() -> Self {
        let (width, height) = tm_logic::init_terminal();

        let mut snake = VecDeque::new();
        snake.push_back(Point {
            x: width / 2,
            y: height / 2,
        });
        snake.push_back(Point {
            x: width / 2 + 1,
            y: height / 2,
        });
        snake.push_back(Point {
            x: width / 2 + 2,
            y: height / 2,
        });

        let food = Point {
            x: rand::thread_rng().gen_range(1..width - 1),
            y: rand::thread_rng().gen_range(1..height - 1),
        };

        Self {
            snake,
            direction: Direction::Left,
            food,
            score: 0,
            game_over: false,
            growing: false,
            height: height,
            width: width,
        }
    }

    // updates the location
    // checks for collision and if the snake ate food
    pub fn update(&mut self) {
        self.move_snake();
        let head = self.snake[0];

        // check for collision
        self.game_over = self.snake.iter().skip(1).any(|segment| *segment == head);

        // ate the apple
        if head == self.food {
            self.score += 1;
            let last = self.snake.back().unwrap(); // this should never be None
            self.snake.push_back(Point {
                x: last.x + 1,
                y: last.y,
            });

            self.growing = true;
            self.gen_fruit();
        }
    }

    // draws it based on the coordinates
    // all coordinates are positive
    // head is drawn using a star
    pub fn draw(&self) {
        let mut stdout = stdout();

        let _ = execute!(stdout, Clear(ClearType::All));

        for (i, segment) in self.snake.iter().enumerate() {
            if i == 0 {
                // head
                let _ = execute!(stdout, cursor::MoveTo(segment.x, segment.y), Print("*"));
            } else {
                let _ = execute!(stdout, cursor::MoveTo(segment.x, segment.y), Print("o"));
            }
        }

        // draw the food
        let _ = execute!(stdout, cursor::MoveTo(self.food.x, self.food.y), Print("a"));

        // score
        let _ = execute!(
            stdout,
            cursor::MoveTo(0, 0),
            Print(format!("Score: {}", self.score))
        );
    }

    // wrapper func that resets the terminal to the state at which it was before the game
    pub fn cleanup(self) {
        tm_logic::cleanup_terminal(self.score, self.width, self.height);
    }

    // updates the location by poping the tail and prepending the head to the new location
    fn move_snake(&mut self) {
        let new_head = {
            let head = self.snake.front().unwrap();
            match self.direction {
                Direction::Up => Point {
                    x: head.x,
                    y: (head.y + self.height - 1) % self.height,
                },
                Direction::Down => Point {
                    x: head.x,
                    y: (head.y + 1) % self.height,
                },
                Direction::Left => Point {
                    x: (head.x + self.width - 1) % self.width,
                    y: head.y,
                },
                Direction::Right => Point {
                    x: (head.x + 1) % self.width,
                    y: head.y,
                },
            }
        };

        // Insert the new head at the front
        self.snake.push_front(new_head);

        // Remove the tail unless the snake is growing
        if !self.growing {
            self.snake.pop_back();
        } else {
            self.growing = false; // Reset growth flag
        }
    }

    // random x y for the fruit
    fn gen_fruit(&mut self) {
        self.food.x = rand::thread_rng().gen_range(1..self.width - 1);
        self.food.y = rand::thread_rng().gen_range(1..self.height - 1);
    }
}
