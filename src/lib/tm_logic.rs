use super::snake;
use crossterm::event::{self, Event, KeyCode};
use crossterm::{
    cursor, execute,
    style::Print,
    terminal,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetSize},
};
use std::fmt::format;
use std::io::stdout;

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
        SetSize(x, y), // Resize the terminal
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

pub fn cleanup_terminal(score: u16, width: u16, height: u16) {
    let mut stdout = stdout();
    let h1 = "thank you for playing!";
    let h2 = format!("Your score: {}!", score);

    let _ = execute!(
        stdout,
        cursor::MoveTo((width / 2) - (h1.len() as u16 / 2), (height / 2) - 1),
        Print(h1),
        cursor::MoveTo((width / 2) - (h2.len() as u16 / 2), (height / 2) + 1),
        Print(h2),
    );

    std::thread::sleep(std::time::Duration::from_secs(1));

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
    // Check if an event is available within the timeout duration
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
