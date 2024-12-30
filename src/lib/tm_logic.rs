use super::snake;
use crossterm::event::{self, Event, KeyCode};
use crossterm::{
    cursor, execute,
    style::Print,
    terminal,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetSize},
};

use std::io::stdout;

pub struct GameMode {
    pub players: u8,
    pub borders: bool,
}

impl GameMode {
    pub fn new() -> Self {
        GameMode {
            players: 1,
            borders: false,
        }
    }

    pub fn welcome_screen(&mut self, width: u16, height: u16) {
        let mut stdout = stdout();

        let h1 = "Press 1 for single player";
        let h2 = "Press 2 for multiplayer";

        let _ = execute!(
            stdout,
            Clear(ClearType::All),
            cursor::MoveTo((width / 2) - (h1.len() as u16 / 2), (height / 2) - 1),
            Print(h1),
            cursor::MoveTo((width / 2) - (h2.len() as u16 / 2), (height / 2) + 1),
            Print(h2),
        );

        loop {
            if event::poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap() {
                    match key_event.code {
                        KeyCode::Char('1') => {
                            self.players = 1;
                            break;
                        }
                        KeyCode::Char('2') => {
                            self.players = 2;
                            break;
                        }
                        KeyCode::Char('q') | KeyCode::Char('Q') => {
                            self.players = 0;
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }

        let noborders = "Press 1 for no borders";
        let borders = "Press 2 for borders";

        let _ = execute!(
            stdout,
            Clear(ClearType::All),
            cursor::MoveTo((width / 2) - (noborders.len() as u16 / 2), (height / 2) - 1),
            Print(noborders),
            cursor::MoveTo((width / 2) - (borders.len() as u16 / 2), (height / 2) + 1),
            Print(borders),
        );

        loop {
            if event::poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap() {
                    match key_event.code {
                        KeyCode::Char('1') => {
                            self.borders = false;
                            break;
                        }
                        KeyCode::Char('2') => {
                            self.borders = true;
                            break;
                        }
                        KeyCode::Char('q') | KeyCode::Char('Q') => {
                            self.players = 0;
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

// hides cursros and sets temrinal to Raw for better user handling
pub fn init_terminal() -> (u16, u16) {
    match terminal::enable_raw_mode() {
        Ok(_) => {}
        Err(e) => panic!("Could not init raw mode because: {}", e),
    }

    let (x, y) = terminal::size().unwrap_or((80, 24));

    let mut stdout = stdout();

    let h1 = "My Scuffed Snake";
    let h2 = "By Nik Tsonev";

    let rc = execute!(
        stdout,
        EnterAlternateScreen,
        SetSize(x, y),
        Clear(ClearType::All),
        cursor::Hide,
        cursor::MoveTo((x / 2) - (h1.len() as u16 / 2), (y / 2) - 1),
        Print(h1),
        cursor::MoveTo((x / 2) - (h2.len() as u16 / 2), (y / 2) + 1),
        Print(h2),
    );

    std::thread::sleep(std::time::Duration::from_secs(2));

    match rc {
        Ok(()) => {}
        Err(e) => panic!("Error: {}", e),
    }

    terminal::size().unwrap_or((80, 24))
}

pub fn clear_screan() {
    let mut stdout = stdout();

    let _ = execute!(stdout, Clear(ClearType::All));
}

pub fn cleanup_terminal(
    score1: u16,
    score2: u16,
    width: u16,
    height: u16,
    players: u8,
    winner: u16,
) {
    let mut stdout = stdout();

    if players == 1 {
        let h1 = "thank you for playing!";
        let h2 = format!("Your score: {}!", score1);

        let _ = execute!(
            stdout,
            cursor::MoveTo((width / 2) - (h1.len() as u16 / 2), (height / 2) - 1),
            Print(h1),
            cursor::MoveTo((width / 2) - (h2.len() as u16 / 2), (height / 2) + 1),
            Print(h2),
        );
    }

    if players == 2 {
        let winner_msg: String = if winner == 1 {
            format!("Player {} Won with {} points!", winner, score1)
        } else {
            format!("Player {} Won with {} points!", winner, score2)
        };

        let _ = execute!(
            stdout,
            cursor::MoveTo(
                (width / 2) - (winner_msg.len() as u16 / 2),
                (height / 2) - 1
            ),
            Print(winner_msg),
        );
    }

    if players == 0 {
        clear_screan();
        let goodbey = "Sad to see you go";
        let _ = execute!(
            stdout,
            cursor::MoveTo((width / 2) - (goodbey.len() as u16 / 2), (height / 2) - 1),
            Print(goodbey),
        );
    }

    std::thread::sleep(std::time::Duration::from_secs(2));

    let _ = execute!(
        stdout,
        Clear(ClearType::All),
        LeaveAlternateScreen,
        cursor::Show
    );

    let _ = terminal::disable_raw_mode();
}

// non blocking event func that converts arrows to Direction Enum
// must be none blocking otherwise snake does not move on its own
// change the from_millis to dictate the speed
pub fn handle_input(game: &mut snake::SnakeGame) -> bool {
    if event::poll(std::time::Duration::from_millis(50)).unwrap() {
        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event.code {
                KeyCode::Up if game.direction != snake::Direction::Down => {
                    game.direction = snake::Direction::Up;
                }
                KeyCode::Down if game.direction != snake::Direction::Up => {
                    game.direction = snake::Direction::Down;
                }
                KeyCode::Left if game.direction != snake::Direction::Right => {
                    game.direction = snake::Direction::Left;
                }
                KeyCode::Right if game.direction != snake::Direction::Left => {
                    game.direction = snake::Direction::Right;
                }
                KeyCode::Char('q') => {
                    println!("Exiting...");
                    return false;
                }
                _ => {}
            }
        }
    }
    true
}

pub fn multiplayer_handle_input(game: &mut snake::SnakeGame) -> bool {
    if event::poll(std::time::Duration::from_millis(50)).unwrap() {
        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event.code {
                KeyCode::Up if game.direction != snake::Direction::Down => {
                    game.direction = snake::Direction::Up;
                }
                KeyCode::Down if game.direction != snake::Direction::Up => {
                    game.direction = snake::Direction::Down;
                }
                KeyCode::Left if game.direction != snake::Direction::Right => {
                    game.direction = snake::Direction::Left;
                }
                KeyCode::Right if game.direction != snake::Direction::Left => {
                    game.direction = snake::Direction::Right;
                }
                KeyCode::Char('w') if game.direction2 != snake::Direction::Down => {
                    game.direction2 = snake::Direction::Up;
                }
                KeyCode::Char('s') if game.direction2 != snake::Direction::Up => {
                    game.direction2 = snake::Direction::Down;
                }
                KeyCode::Char('a') if game.direction2 != snake::Direction::Right => {
                    game.direction2 = snake::Direction::Left;
                }
                KeyCode::Char('d') if game.direction2 != snake::Direction::Left => {
                    game.direction2 = snake::Direction::Right;
                }
                KeyCode::Char('q') => {
                    println!("Exiting...");
                    return false;
                }
                _ => {}
            }
        }
    }
    true
}
