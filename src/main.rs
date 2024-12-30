mod lib;
use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};
use lib::tm_logic::{self, handle_input};
use lib::{snake::SnakeGame, tm_logic::GameMode};
use std::io::stdout;

fn main() -> Result<(), std::io::Error> {
    let (width, height) = tm_logic::init_terminal();
    let mut game = SnakeGame::new(width, height);
    let mut stdout = stdout();
    let mut game_mode = GameMode::new();

    // for consistent refresh rate
    let frame_duration = std::time::Duration::from_millis(50);

    game_mode.welcome_screen(width, height);

    if game_mode.players == 1 {
        while handle_input(&mut game) && !game.game_over {
            let frame_start = std::time::Instant::now();
            // Clear the screen
            execute!(
                stdout,
                Clear(ClearType::All), // Clear the screen
                cursor::MoveTo(0, 0)   // Move cursor to top-left
            )?;

            game.update(game_mode.borders);
            game.draw();

            let elapsed = frame_start.elapsed();
            if elapsed < frame_duration {
                std::thread::sleep(frame_duration - elapsed);
            }
        }
    }

    game.cleanup();
    Ok(())
}
