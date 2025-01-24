// Import what we need to run the game
use crate::game::state::GameState;
use ggez::{event, ContextBuilder};

// Organize our code into folders
mod components; // Game pieces (ball, paddles)
mod game; // Core game logic
mod graphics; // Drawing code
mod tests; // Testing code

fn main() -> ggez::GameResult {
    // Set up game window
    let (ctx, event_loop) = ContextBuilder::new("Ping Pong", "Prudhveraj Botta")
        .window_setup(ggez::conf::WindowSetup::default().title("Ping Pong Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(
            game::constants::SCREEN_WIDTH,
            game::constants::SCREEN_HEIGHT,
        ))
        .build()?;

    // Create new game and start running it
    let game = GameState::new();
    event::run(ctx, event_loop, game)
}
