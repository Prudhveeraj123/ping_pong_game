// Imports needed for scoring
use crate::game::constants::*;
use ggez::graphics::{Canvas, Color, DrawParam, Text, TextFragment};
use ggez::GameResult;
use std::time::Instant;

// Track scores and scoring visuals
pub struct Score {
    pub player1: u32,                 // Left player score
    pub player2: u32,                 // Right player score
    pub flash_winner: Option<u8>,     // Last scorer (1 or 2)
    pub flash_start: Option<Instant>, // When scored
}

impl Score {
    // New game with 0-0 score
    pub fn new() -> Self {
        Score {
            player1: 0,
            player2: 0,
            flash_winner: None,
            flash_start: None,
        }
    }

    // Add point for player 1 with visual effect
    pub fn increment_player1(&mut self) {
        self.player1 += 1;
        self.flash_winner = Some(1);
        self.flash_start = Some(Instant::now());
    }

    // Add point for player 2 with visual effect
    pub fn increment_player2(&mut self) {
        self.player2 += 1;
        self.flash_winner = Some(2);
        self.flash_start = Some(Instant::now());
    }

    // Draw score display
    pub fn draw(&self, canvas: &mut Canvas, ctx: &mut ggez::Context) -> GameResult {
        // Show special effect for 3 seconds after scoring
        if let Some(flash_start) = self.flash_start {
            if flash_start.elapsed().as_secs_f32() < 3.0 {
                self.draw_highlighted_score(canvas, ctx)?;
            } else {
                self.draw_normal_score(canvas, ctx)?;
            }
        } else {
            self.draw_normal_score(canvas, ctx)?;
        }
        Ok(())
    }

    // Draw basic score display
    fn draw_normal_score(&self, canvas: &mut Canvas, ctx: &mut ggez::Context) -> GameResult {
        // Create score text
        let score_text = format!("Player 1: {}  |  Player 2: {}", self.player1, self.player2);
        let text = Text::new(
            TextFragment::new(score_text)
                .scale(16.0)
                .color(Color::WHITE),
        );
        let dims = text.measure(ctx)?;

        // Center at top of screen
        canvas.draw(
            &text,
            DrawParam::default().dest([SCREEN_WIDTH / 2.0 - dims.x / 2.0, 20.0]),
        );
        Ok(())
    }

    // Draw fancy score display after point
    fn draw_highlighted_score(&self, canvas: &mut Canvas, ctx: &mut ggez::Context) -> GameResult {
        // Get winner info
        let (winner_score, other_score, winner_x, other_x) = match self.flash_winner {
            Some(1) => (
                self.player1,
                self.player2,
                SCREEN_WIDTH / 4.0,
                3.0 * SCREEN_WIDTH / 4.0,
            ),
            Some(2) => (
                self.player2,
                self.player1,
                3.0 * SCREEN_WIDTH / 4.0,
                SCREEN_WIDTH / 4.0,
            ),
            _ => return Ok(()),
        };

        // Create green text for winner
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
            .scale(18.0)
            .color(Color::GREEN),
        );
        let winner_dims = winner_text.measure(ctx)?;

        // Create white text for other player
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
            .scale(16.0)
            .color(Color::WHITE),
        );
        let other_dims = other_text.measure(ctx)?;

        // Draw both scores
        canvas.draw(
            &winner_text,
            DrawParam::default().dest([winner_x - winner_dims.x / 2.0, 20.0]),
        );
        canvas.draw(
            &other_text,
            DrawParam::default().dest([other_x - other_dims.x / 2.0, 20.0]),
        );

        // Draw big green score for winner
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

    // Reset all scores to 0
    pub fn reset(&mut self) {
        self.player1 = 0;
        self.player2 = 0;
        self.flash_winner = None;
        self.flash_start = None;
    }
}
