use crate::game::state::GameState;
use ggez::{event, ContextBuilder};

mod components;
mod game;
mod graphics;

fn main() -> ggez::GameResult {
    let (ctx, event_loop) = ContextBuilder::new("Ping Pong", "Prudhveraj Botta")
        .window_setup(ggez::conf::WindowSetup::default().title("Ping Pong Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(
            game::constants::SCREEN_WIDTH,
            game::constants::SCREEN_HEIGHT,
        ))
        .build()?;

    let game = GameState::new();
    event::run(ctx, event_loop, game)
}
// done with basic moduling
