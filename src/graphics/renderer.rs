// Bring in needed functions and types for drawing the game
use crate::game::constants::*;
use crate::game::state::GameState;
use ggez::graphics::{Canvas, Color, DrawParam, Text, TextFragment};
use std::time::Instant;

// This struct handles all the drawing in the game
pub struct GameRenderer<'a> {
    ctx: &'a mut ggez::Context, // Stores drawing tools
}

impl<'a> GameRenderer<'a> {
    // Create new renderer with drawing tools
    pub fn new(ctx: &'a mut ggez::Context) -> Self {
        GameRenderer { ctx }
    }

    // Main function that draws everything in the game
    pub fn render(&mut self, canvas: &mut Canvas, state: &GameState) -> ggez::GameResult {
        // First draw the paddles
        self.draw_paddles(canvas, state)?;

        // Draw ball except during countdown
        if !state.game_running || state.countdown_start.is_none() {
            self.draw_ball(canvas, state)?;
        }

        // Draw the score at the top
        state.score.draw(canvas, self.ctx)?;

        // Show countdown if game is running
        if state.game_running {
            if let Some(countdown_start) = state.countdown_start {
                self.draw_countdown(canvas, countdown_start)?;
            }
        }

        Ok(())
    }

    // Draw both player paddles - green for left, blue for right
    fn draw_paddles(&mut self, canvas: &mut Canvas, state: &GameState) -> ggez::GameResult {
        // Create and draw left paddle in green
        let paddle1_mesh = state.player1.get_mesh(self.ctx)?;
        canvas.draw(
            &paddle1_mesh,
            DrawParam::default().color(Color::from_rgb(0, 255, 0)),
        );

        // Create and draw right paddle in blue
        let paddle2_mesh = state.player2.get_mesh(self.ctx)?;
        canvas.draw(
            &paddle2_mesh,
            DrawParam::default().color(Color::from_rgb(0, 0, 255)),
        );

        Ok(())
    }

    // Draw the ball in yellow
    fn draw_ball(&mut self, canvas: &mut Canvas, state: &GameState) -> ggez::GameResult {
        let ball_mesh = state.ball.get_mesh(self.ctx)?;
        canvas.draw(&ball_mesh, DrawParam::default().color(Color::YELLOW));
        Ok(())
    }

    // Draw countdown numbers in different colors
    fn draw_countdown(
        &mut self,
        canvas: &mut Canvas,
        countdown_start: Instant,
    ) -> ggez::GameResult {
        // Get time passed since countdown started
        let elapsed = countdown_start.elapsed().as_secs_f32();

        // Show different numbers based on time:
        // 0-1 sec: show 3
        // 1-2 sec: show 2
        // 2-3 sec: show 1
        let count = if elapsed < 1.0 {
            3
        } else if elapsed < 2.0 {
            2
        } else if elapsed < 3.0 {
            1
        } else {
            0
        };

        // Only show numbers 3,2,1
        if count > 0 {
            // Pick color for current number:
            // 3 = red, 2 = yellow, 1 = green
            let color = match count {
                3 => Color::RED,
                2 => Color::YELLOW,
                1 => Color::GREEN,
                _ => Color::WHITE,
            };

            // Create number text with chosen color
            let fragment = TextFragment::new(count.to_string())
                .scale(35.0)
                .color(color);

            // Set up text for drawing
            let countdown_text = Text::new(fragment);
            let dims = countdown_text.measure(self.ctx)?;

            // Draw number in center of screen
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
