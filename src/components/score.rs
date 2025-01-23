//! This file handles the scoring system and score display for the Pong game

// Import necessary modules and types
use crate::game::constants::*; // Game constants like screen dimensions
use ggez::graphics::{Canvas, Color, DrawParam, Text, TextFragment}; // For rendering text
use ggez::GameResult; // Type for error handling
use std::time::Instant; // For timing the score highlight effect

/// Structure to keep track of game scores and score display effects
pub struct Score {
    pub player1: u32,                 // Player 1's score (left player)
    pub player2: u32,                 // Player 2's score (right player)
    pub flash_winner: Option<u8>,     // Which player just scored (1 or 2), None if no recent score
    pub flash_start: Option<Instant>, // When the score highlight effect started
}

impl Score {
    /// Creates a new Score instance with initial values
    pub fn new() -> Self {
        Score {
            player1: 0,         // Start with 0 points
            player2: 0,         // Start with 0 points
            flash_winner: None, // No one has scored yet
            flash_start: None,  // No highlight effect active
        }
    }

    /// Increases Player 1's score and triggers the highlight effect
    pub fn increment_player1(&mut self) {
        self.player1 += 1; // Add one point
        self.flash_winner = Some(1); // Mark Player 1 as recent scorer
        self.flash_start = Some(Instant::now()); // Start the highlight timer
    }

    /// Increases Player 2's score and triggers the highlight effect
    pub fn increment_player2(&mut self) {
        self.player2 += 1; // Add one point
        self.flash_winner = Some(2); // Mark Player 2 as recent scorer
        self.flash_start = Some(Instant::now()); // Start the highlight timer
    }

    /// Main drawing function that decides whether to show normal or highlighted score
    pub fn draw(&self, canvas: &mut Canvas, ctx: &mut ggez::Context) -> GameResult {
        if let Some(flash_start) = self.flash_start {
            // If someone just scored (within last 3 seconds), show highlighted score
            if flash_start.elapsed().as_secs_f32() < 3.0 {
                self.draw_highlighted_score(canvas, ctx)?;
            } else {
                // After 3 seconds, show normal score
                self.draw_normal_score(canvas, ctx)?;
            }
        } else {
            // If no one just scored, show normal score
            self.draw_normal_score(canvas, ctx)?;
        }
        Ok(())
    }

    /// Draws the score normally (both scores same size and color)
    fn draw_normal_score(&self, canvas: &mut Canvas, ctx: &mut ggez::Context) -> GameResult {
        // Create score text with both players' scores
        let score_text = format!("Player 1: {}  |  Player 2: {}", self.player1, self.player2);

        // Create text object with white color and size 16
        let text = Text::new(
            TextFragment::new(score_text)
                .scale(16.0)
                .color(Color::WHITE),
        );

        // Calculate text dimensions for centering
        let dims = text.measure(ctx)?;

        // Draw text centered at top of screen
        canvas.draw(
            &text,
            DrawParam::default().dest([SCREEN_WIDTH / 2.0 - dims.x / 2.0, 20.0]),
        );
        Ok(())
    }

    /// Draws highlighted score when someone just scored (special effect)
    fn draw_highlighted_score(&self, canvas: &mut Canvas, ctx: &mut ggez::Context) -> GameResult {
        // Determine which scores and positions to use based on who scored
        let (winner_score, other_score, winner_x, other_x) = match self.flash_winner {
            Some(1) => (
                self.player1,             // Winner's score
                self.player2,             // Other player's score
                SCREEN_WIDTH / 4.0,       // Winner's X position (left side)
                3.0 * SCREEN_WIDTH / 4.0, // Other player's X position (right side)
            ),
            Some(2) => (
                self.player2,             // Winner's score
                self.player1,             // Other player's score
                3.0 * SCREEN_WIDTH / 4.0, // Winner's X position (right side)
                SCREEN_WIDTH / 4.0,       // Other player's X position (left side)
            ),
            _ => return Ok(()), // Should never happen, but handle it safely
        };

        // Create highlighted text for the player who just scored
        let winner_text = Text::new(
            TextFragment::new(format!(
                "Player {}: {}",
                if self.flash_winner == Some(1) {
                    "1"
                } else {
                    "2"
                },
                winner_score
            ))
            .scale(18.0) // Larger text for emphasis
            .color(Color::GREEN), // Green color for winner
        );
        let winner_dims = winner_text.measure(ctx)?;

        // Create normal text for the other player
        let other_text = Text::new(
            TextFragment::new(format!(
                "Player {}: {}",
                if self.flash_winner == Some(1) {
                    "2"
                } else {
                    "1"
                },
                other_score
            ))
            .scale(16.0) // Normal size text
            .color(Color::WHITE), // White color for non-winner
        );
        let other_dims = other_text.measure(ctx)?;

        // Draw winner's score text
        canvas.draw(
            &winner_text,
            DrawParam::default().dest([winner_x - winner_dims.x / 2.0, 20.0]),
        );

        // Draw other player's score text
        canvas.draw(
            &other_text,
            DrawParam::default().dest([other_x - other_dims.x / 2.0, 20.0]),
        );

        // Create and draw an extra-large score number for dramatic effect
        let big_score = Text::new(
            TextFragment::new(winner_score.to_string())
                .scale(22.0) // Extra large size
                .color(Color::GREEN), // Green to match winner's text
        );
        let big_dims = big_score.measure(ctx)?;

        // Draw the big score number below the regular score
        canvas.draw(
            &big_score,
            DrawParam::default().dest([winner_x - big_dims.x / 2.0, 80.0]),
        );

        Ok(())
    }

    /// Resets all scores and effects to initial state
    pub fn reset(&mut self) {
        self.player1 = 0; // Reset player 1 score to 0
        self.player2 = 0; // Reset player 2 score to 0
        self.flash_winner = None; // Clear any highlight effect
        self.flash_start = None; // Clear highlight timer
    }
}
