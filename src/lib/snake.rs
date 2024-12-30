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
    pub direction2: Direction,
    growing: bool,
    growing2: bool,
    food: Point,
    height: u16,
    width: u16,
    score: u16,
    score2: u16,
    pub winner: u16,
    pub game_over: bool,
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
            direction2: Direction::Left,
            food: food,
            score: 0,
            score2: 0,
            game_over: false,
            growing: false,
            growing2: false,
            winner: 0,
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
    pub fn cleanup(self, players: u8) {
        tm_logic::cleanup_terminal(
            self.score,
            self.score2,
            self.width,
            self.height,
            players,
            self.winner,
        );
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
    fn multiplayer_new(width: u16, height: u16) -> Self;
    fn multiplayer_update(&mut self, border: bool);
    fn multiplayer_draw(&mut self);
    fn multiplayer_move_snake(&mut self, border: bool);
    fn collision(&mut self);
}

impl Multiplayer for SnakeGame {
    fn multiplayer_new(width: u16, height: u16) -> Self {
        let mut snake = VecDeque::new();
        snake.push_back(Point {
            x: width - 2,
            y: height / 2,
        });
        snake.push_back(Point {
            x: width - 1,
            y: height / 2,
        });
        snake.push_back(Point {
            x: width,
            y: height / 2,
        });

        let new_food = Point {
            x: rand::thread_rng().gen_range(1..width - 1),
            y: rand::thread_rng().gen_range(1..height - 1),
        };

        let mut snake2 = VecDeque::new();
        snake2.push_back(Point {
            x: width / 2,
            y: height / 2,
        });
        snake2.push_back(Point {
            x: width / 2 + 1,
            y: height / 2,
        });
        snake2.push_back(Point {
            x: width / 2 + 2,
            y: height / 2,
        });

        Self {
            player1: snake,
            player2: Some(snake2),
            direction: Direction::Left,
            direction2: Direction::Left,
            food: new_food,
            score: 0,
            score2: 0,
            game_over: false,
            growing: false,
            growing2: false,
            winner: 0,
            height: height,
            width: width,
        }
    }

    fn multiplayer_update(&mut self, border: bool) {
        self.move_snake(border);
        self.multiplayer_move_snake(border);
        let snake1_head = self.player1[0];
        let snake2_head = self.player2.as_ref().unwrap()[0];

        // check for collision
        self.collision();

        // ate the apple
        if snake1_head == self.food {
            self.score += 1;
            let last = self.player1.back().unwrap(); // this should never be None
            self.player1.push_back(Point {
                x: last.x + 1,
                y: last.y,
            });

            self.growing = true;
            self.gen_fruit();
        }

        if snake2_head == self.food {
            self.score2 += 1;
            let player2 = self.player2.as_mut().unwrap();
            let last = player2.back().unwrap(); // this should never be None
            player2.push_back(Point {
                x: last.x + 1,
                y: last.y,
            });

            self.growing2 = true;
            self.gen_fruit();
        }

        if self.score == self.width * self.height || self.score2 == self.width * self.height {
            self.game_over = true;
        }
    }

    fn multiplayer_draw(&mut self) {
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

        if let Some(player2) = &self.player2 {
            for (i, segment) in player2.iter().enumerate() {
                if i == 0 {
                    // head
                    let _ = execute!(stdout, cursor::MoveTo(segment.x, segment.y), Print("$"));
                } else {
                    let _ = execute!(stdout, cursor::MoveTo(segment.x, segment.y), Print("o"));
                }
            }
        }

        // draw the food
        let _ = execute!(stdout, cursor::MoveTo(self.food.x, self.food.y), Print("a"));

        // score
        let _ = execute!(
            stdout,
            cursor::MoveTo(0, 0),
            Print(format!("P1: {}", self.score)),
            cursor::MoveTo(self.width - 20, 0),
            Print(format!("P2: {}", self.score2))
        );
    }

    fn multiplayer_move_snake(&mut self, border: bool) {
        let new_head = {
            let head = self.player2.as_mut().unwrap().front().unwrap();
            match self.direction2 {
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
        self.player2.as_mut().unwrap().push_front(new_head);

        if !self.growing2 {
            self.player2.as_mut().unwrap().pop_back();
        } else {
            self.growing2 = false;
        }
    }

    // returns true if the game is over, the snakes have collided
    fn collision(&mut self) {
        let snake1_head = self.player1[0];
        let snake2_head = self.player2.as_ref().unwrap()[0];

        // check for collision
        if self
            .player1
            .iter()
            .skip(1)
            .any(|segment| *segment == snake1_head)
            || self
                .player2
                .as_ref()
                .unwrap()
                .iter()
                .skip(1)
                .any(|segment| *segment == snake2_head)
        {
            self.game_over = true;
            self.winner = if self
                .player1
                .iter()
                .skip(1)
                .any(|segment| *segment == snake1_head)
            {
                2
            } else {
                1
            };
        }

        if self
            .player2
            .as_ref()
            .unwrap()
            .iter()
            .any(|segment| *segment == snake1_head)
            || self.player1.iter().any(|segment| *segment == snake2_head)
        {
            self.game_over = true;
            self.winner = if self
                .player2
                .as_ref()
                .unwrap()
                .iter()
                .any(|segment| *segment == snake1_head)
            {
                2
            } else {
                1
            };
        }

        // they collide head on, so then the person with higher score wins
        if snake1_head == snake2_head {
            self.winner = if self.score > self.score2 {
                1
            } else if self.score > self.score2 {
                2
            } else {
                // draw
                3
            }
        }
    }
}
