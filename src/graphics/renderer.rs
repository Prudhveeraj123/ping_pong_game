// Import necessary modules and types from other parts of the project and dependencies
use crate::game::constants::*; // Game constants like screen dimensions, speeds, etc.
use crate::game::state::GameState; // Main game state struct
use ggez::graphics::{Canvas, Color, DrawParam, Text, TextFragment}; // Graphics components from ggez
use std::time::Instant; // For timing operations

/// GameRenderer struct handles all the drawing operations for the game
/// It takes a mutable reference to the ggez Context which is needed for rendering
pub struct GameRenderer<'a> {
    ctx: &'a mut ggez::Context,
}

impl<'a> GameRenderer<'a> {
    /// Creates a new GameRenderer instance
    /// Parameters:
    ///   - ctx: The ggez graphics context used for rendering
    pub fn new(ctx: &'a mut ggez::Context) -> Self {
        GameRenderer { ctx }
    }

    /// Main render function that draws all game elements
    /// Parameters:
    ///   - canvas: The canvas to draw on
    ///   - state: Current game state containing all game objects
    pub fn render(&mut self, canvas: &mut Canvas, state: &GameState) -> ggez::GameResult {
        // Draw the paddles first
        self.draw_paddles(canvas, state)?;

        // Draw ball in these conditions:
        // 1. Before game starts (game_running is false)
        // 2. During gameplay when there's no countdown
        if !state.game_running || state.countdown_start.is_none() {
            self.draw_ball(canvas, state)?;
        }

        // Draw the score
        state.score.draw(canvas, self.ctx)?;

        // Only show countdown during gameplay (when game_running is true)
        if state.game_running {
            if let Some(countdown_start) = state.countdown_start {
                self.draw_countdown(canvas, countdown_start)?;
            }
        }

        Ok(())
    }

    /// Draws both player paddles
    /// Parameters:
    ///   - canvas: The canvas to draw on
    ///   - state: Current game state containing paddle information
    fn draw_paddles(&mut self, canvas: &mut Canvas, state: &GameState) -> ggez::GameResult {
        // Draw player 1's paddle (left side) in green
        let paddle1_mesh = state.player1.get_mesh(self.ctx)?;
        canvas.draw(
            &paddle1_mesh,
            DrawParam::default().color(Color::from_rgb(0, 255, 0)),
        );

        // Draw player 2's paddle (right side) in blue
        let paddle2_mesh = state.player2.get_mesh(self.ctx)?;
        canvas.draw(
            &paddle2_mesh,
            DrawParam::default().color(Color::from_rgb(0, 0, 255)),
        );

        Ok(())
    }

    /// Draws the ball
    /// Parameters:
    ///   - canvas: The canvas to draw on
    ///   - state: Current game state containing ball information
    fn draw_ball(&mut self, canvas: &mut Canvas, state: &GameState) -> ggez::GameResult {
        // Create the ball mesh and draw it in yellow
        let ball_mesh = state.ball.get_mesh(self.ctx)?;
        canvas.draw(&ball_mesh, DrawParam::default().color(Color::YELLOW));
        Ok(())
    }

    /// Draws the countdown numbers (3,2,1) before each point starts
    /// Parameters:
    ///   - canvas: The canvas to draw on
    ///   - countdown_start: The instant when countdown started
    fn draw_countdown(
        &mut self,
        canvas: &mut Canvas,
        countdown_start: Instant,
    ) -> ggez::GameResult {
        // Calculate how much time has passed since countdown started
        let elapsed = countdown_start.elapsed().as_secs_f32();

        // Determine which number to show based on elapsed time
        // Each number shows for 1 second
        let count = if elapsed < 1.0 {
            3
        } else if elapsed < 2.0 {
            2
        } else if elapsed < 3.0 {
            1
        } else {
            0 // Countdown is finished
        };

        // Only draw numbers 3,2,1 (not 0)
        if count > 0 {
            // Choose color based on the current number
            let color = match count {
                3 => Color::RED,    // 3 is shown in red
                2 => Color::YELLOW, // 2 is shown in yellow
                1 => Color::GREEN,  // 1 is shown in green
                _ => Color::WHITE,  // Fallback color (shouldn't occur)
            };

            // Create the text with the countdown number
            let fragment = TextFragment::new(count.to_string())
                .scale(35.0) // Size of the countdown text
                .color(color);

            let countdown_text = Text::new(fragment);

            // Calculate dimensions to center the text
            let dims = countdown_text.measure(self.ctx)?;

            // Draw the countdown text centered on screen
            canvas.draw(
                &countdown_text,
                DrawParam::default().dest([
                    SCREEN_WIDTH / 2.0 - dims.x / 2.0,  // Center horizontally
                    SCREEN_HEIGHT / 2.0 - dims.y / 2.0, // Center vertically
                ]),
            );
        }
        Ok(())
    }
}
