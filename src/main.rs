mod lib;
use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};
use lib::snake::SnakeGame;
use lib::tm_logic::handle_input;
use std::io::stdout;

fn main() -> Result<(), std::io::Error> {
    let mut game = SnakeGame::init();
    let mut stdout = stdout();
    // for consistent refresh rate
    let frame_duration = std::time::Duration::from_millis(50);

    while handle_input(&mut game) && !game.game_over {
        let frame_start = std::time::Instant::now();
        // Clear the screen
        execute!(
            stdout,
            Clear(ClearType::All), // Clear the screen
            cursor::MoveTo(0, 0)   // Move cursor to top-left
        )?;

        game.update();
        game.draw();

        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }

    game.cleanup();
    Ok(())
}
