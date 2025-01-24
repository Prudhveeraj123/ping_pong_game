// Import required modules for graphics, game constants, and timing
use crate::game::constants::*;
use ggez::graphics::{Canvas, Color, DrawParam, Text, TextFragment};
use ggez::GameResult;
use std::time::Instant;

// Main structure to handle game scoring and score display
pub struct Score {
    pub player1: u32,                 // Store your (left player) score
    pub player2: u32,                 // Store AI (right player) score
    pub flash_winner: Option<u8>,     // Store who just scored (1=you, 2=AI)
    pub flash_start: Option<Instant>, // Timer for score highlight animation
}

impl Score {
    // Create new score board with all scores at 0
    pub fn new() -> Self {
        Score {
            player1: 0,
            player2: 0,
            flash_winner: None,
            flash_start: None,
        }
    }

    // Increase your score by 1 and trigger highlight animation
    pub fn increment_player1(&mut self) {
        self.player1 += 1;
        self.flash_winner = Some(1);
        self.flash_start = Some(Instant::now());
    }

    // Increase AI's score by 1 and trigger highlight animation
    pub fn increment_player2(&mut self) {
        self.player2 += 1;
        self.flash_winner = Some(2);
        self.flash_start = Some(Instant::now());
    }

    // Main draw function that decides whether to show normal or highlighted score
    pub fn draw(&self, canvas: &mut Canvas, ctx: &mut ggez::Context) -> GameResult {
        // Check if highlight animation is active (lasts 3 seconds)
        if let Some(flash_start) = self.flash_start {
            if flash_start.elapsed().as_secs_f32() < 3.0 {
                self.draw_highlighted_score(canvas, ctx)?; // Show fancy animation
            } else {
                self.draw_normal_score(canvas, ctx)?; // Show regular score
            }
        } else {
            self.draw_normal_score(canvas, ctx)?; // Show regular score
        }
        Ok(())
    }

    // Draw regular score display at top of screen
    fn draw_normal_score(&self, canvas: &mut Canvas, ctx: &mut ggez::Context) -> GameResult {
        // Create score text with both scores
        let score_text = format!(
            "Your Score: {}  |  Player 2 Score: {}",
            self.player1, self.player2
        );

        // Setup text style (white, medium size)
        let text = Text::new(
            TextFragment::new(score_text)
                .scale(16.0)
                .color(Color::WHITE),
        );

        // Get text dimensions for centering
        let dims = text.measure(ctx)?;

        // Draw centered at top of screen
        canvas.draw(
            &text,
            DrawParam::default().dest([SCREEN_WIDTH / 2.0 - dims.x / 2.0, 20.0]),
        );
        Ok(())
    }

    // Draw animated score display when someone scores
    fn draw_highlighted_score(&self, canvas: &mut Canvas, ctx: &mut ggez::Context) -> GameResult {
        // Setup positions based on who scored
        let (winner_score, other_score, winner_x, other_x) = match self.flash_winner {
            Some(1) => (
                // You scored
                self.player1,             // Winner score is yours
                self.player2,             // Other score is AI's
                SCREEN_WIDTH / 4.0,       // Your score position
                3.0 * SCREEN_WIDTH / 4.0, // AI score position
            ),
            Some(2) => (
                // AI scored
                self.player2,             // Winner score is AI's
                self.player1,             // Other score is yours
                3.0 * SCREEN_WIDTH / 4.0, // AI score position
                SCREEN_WIDTH / 4.0,       // Your score position
            ),
            _ => return Ok(()),
        };

        // Create highlighted text for scorer (green, larger)
        let winner_text = Text::new(
            TextFragment::new(format!(
                "{}: {}",
                if self.flash_winner == Some(1) {
                    "Your Score"
                } else {
                    "Player 2 Score"
                },
                winner_score
            ))
            .scale(18.0)
            .color(Color::GREEN),
        );
        let winner_dims = winner_text.measure(ctx)?;

        // Create normal text for other player (white, regular size)
        let other_text = Text::new(
            TextFragment::new(format!(
                "{}: {}",
                if self.flash_winner == Some(1) {
                    "Player 2 Score"
                } else {
                    "Your Score"
                },
                other_score
            ))
            .scale(16.0)
            .color(Color::WHITE),
        );
        let other_dims = other_text.measure(ctx)?;

        // Draw both score texts
        canvas.draw(
            &winner_text,
            DrawParam::default().dest([winner_x - winner_dims.x / 2.0, 20.0]),
        );
        canvas.draw(
            &other_text,
            DrawParam::default().dest([other_x - other_dims.x / 2.0, 20.0]),
        );

        // Draw big score number for scorer
        let big_score = Text::new(
            TextFragment::new(winner_score.to_string())
                .scale(22.0)
                .color(Color::GREEN),
        );
        let big_dims = big_score.measure(ctx)?;
        canvas.draw(
            &big_score,
            DrawParam::default().dest([winner_x - big_dims.x / 2.0, 80.0]),
        );

        Ok(())
    }

    // Reset all scores and animations to starting state
    pub fn reset(&mut self) {
        self.player1 = 0; // Your score to 0
        self.player2 = 0; // AI score to 0
        self.flash_winner = None; // Clear winner highlight
        self.flash_start = None; // Clear animation timer
    }
}
