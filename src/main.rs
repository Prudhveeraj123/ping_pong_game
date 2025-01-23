//! This is the entry point for the Pong game. It sets up the game window and starts the game loop.

// Import the game state struct from our game module
use crate::game::state::GameState;

// Import necessary components from the ggez game framework
use ggez::{event, ContextBuilder};

// Declare the modules our game uses
mod components; // Contains game objects like ball, paddle, score
mod game; // Contains game state and constants
mod graphics; // Contains rendering code

/// The main function - entry point of our game
fn main() -> ggez::GameResult {
    // Create a new game window and context
    let (ctx, event_loop) = ContextBuilder::new(
        "Ping Pong",        // Internal name of the game
        "Prudhveraj Botta", // Author name
    )
    .window_setup(
        ggez::conf::WindowSetup::default().title("Ping Pong Game"), // Window title shown at the top
    )
    .window_mode(ggez::conf::WindowMode::default().dimensions(
        game::constants::SCREEN_WIDTH,  // Window width from our constants
        game::constants::SCREEN_HEIGHT, // Window height from our constants
    ))
    .build()?; // Create the window, '?' handles any errors

    // Create a new instance of our game state
    let game = GameState::new();

    // Start the game loop!
    // This runs forever until the game is closed, handling:
    // - Drawing each frame
    // - Processing input
    // - Updating game state
    event::run(ctx, event_loop, game)
}

// Note: This marks the completion of basic module structure
// The game is organized into:
// - components/: Individual game pieces (ball, paddles, etc.)
// - game/: Game logic and state management
// - graphics/: Visual rendering code
