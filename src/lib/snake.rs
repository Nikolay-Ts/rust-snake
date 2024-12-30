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
    player1: VecDeque<Point>,
    player2: Option<VecDeque<Point>>,
    pub direction: Direction,
    pub direction2: Option<Direction>,
    pub game_over: bool,
    score: u16,
    growing: bool,
    food: Point,
    height: u16,
    width: u16,
}

// single player
impl SnakeGame {
    pub fn new(width: u16, height: u16) -> Self {
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
            player1: snake,
            player2: None,
            direction: Direction::Left,
            direction2: None,
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
    pub fn update(&mut self, border: bool) {
        self.move_snake(border);
        let head = self.player1[0];

        // check for collision
        self.game_over = self.player1.iter().skip(1).any(|segment| *segment == head);

        // ate the apple
        if head == self.food {
            self.score += 1;
            let last = self.player1.back().unwrap(); // this should never be None
            self.player1.push_back(Point {
                x: last.x + 1,
                y: last.y,
            });

            self.growing = true;
            self.gen_fruit();
        }

        if self.score == self.width * self.height {
            self.game_over = true;
        }
    }

    // draws it based on the coordinates
    // all coordinates are positive
    // head is drawn using a star
    pub fn draw(&self /*_score: Point*/) {
        let mut stdout = stdout();

        let _ = execute!(stdout, Clear(ClearType::All));

        for (i, segment) in self.player1.iter().enumerate() {
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
    // also depending on the gamemode checks if the head has collided with the wall
    // if there are no walls % width / height to teleport the snake to the opposite side of screen
    fn move_snake(&mut self, border: bool) {
        let new_head = {
            let head = self.player1.front().unwrap();
            match self.direction {
                Direction::Up => {
                    if border && head.y == 0 {
                        self.game_over = true;
                        head.clone()
                    } else {
                        Point {
                            x: head.x,
                            y: if border {
                                head.y - 1
                            } else {
                                (head.y + self.height - 1) % self.height
                            },
                        }
                    }
                }
                Direction::Down => {
                    if border && head.y == self.height - 1 {
                        self.game_over = true;
                        head.clone()
                    } else {
                        Point {
                            x: head.x,
                            y: if border {
                                head.y + 1
                            } else {
                                (head.y + 1) % self.height
                            },
                        }
                    }
                }
                Direction::Left => {
                    if border && head.x == 0 {
                        self.game_over = true;
                        head.clone()
                    } else {
                        Point {
                            x: if border {
                                head.x - 1
                            } else {
                                (head.x + self.width - 1) % self.width
                            },
                            y: head.y,
                        }
                    }
                }
                Direction::Right => {
                    if border && head.x == self.width - 1 {
                        self.game_over = true;
                        head.clone()
                    } else {
                        Point {
                            x: if border {
                                head.x + 1
                            } else {
                                (head.x + 1) % self.width
                            },
                            y: head.y,
                        }
                    }
                }
            }
        };

        // prepend the new head
        self.player1.push_front(new_head);

        if !self.growing {
            self.player1.pop_back();
        } else {
            self.growing = false;
        }
    }

    // random x y for the fruit
    // recursivley called if the food is spawned in the snake thats not the head
    fn gen_fruit(&mut self) {
        self.food.x = rand::thread_rng().gen_range(1..self.width - 1);
        self.food.y = rand::thread_rng().gen_range(1..self.height - 1);

        if self
            .player1
            .iter()
            .skip(1)
            .any(|segment| *segment == self.food)
        {
            self.gen_fruit();
        }
    }
}

pub trait Multiplayer {
    fn multiplayer_new() -> Self;
    fn multiplayer_update(&mut self, border: bool);
}
