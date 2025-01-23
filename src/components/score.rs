use crate::game::constants::*;
use ggez::graphics::{self, Canvas, Color, DrawParam, Drawable, Text, TextFragment};
use ggez::GameResult;
use std::time::Instant;

pub struct Score {
    pub player1: u32,
    pub player2: u32,
    pub flash_winner: Option<u8>,
    pub flash_start: Option<Instant>,
}

impl Score {
    pub fn new() -> Self {
        Score {
            player1: 0,
            player2: 0,
            flash_winner: None,
            flash_start: None,
        }
    }

    pub fn increment_player1(&mut self) {
        self.player1 += 1;
        self.flash_winner = Some(1);
        self.flash_start = Some(Instant::now());
    }

    pub fn increment_player2(&mut self) {
        self.player2 += 1;
        self.flash_winner = Some(2);
        self.flash_start = Some(Instant::now());
    }

    pub fn draw(&self, canvas: &mut Canvas, ctx: &mut ggez::Context) -> GameResult {
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

    fn draw_normal_score(&self, canvas: &mut Canvas, ctx: &mut ggez::Context) -> GameResult {
        let score_text = format!("Player 1: {}     Player 2: {}", self.player1, self.player2);
        let text = Text::new(TextFragment::new(score_text).scale(1.0));
        let dims = text.measure(ctx)?;

        canvas.draw(
            &text,
            DrawParam::default()
                .dest([SCREEN_WIDTH / 2.0 - dims.x / 2.0, 20.0])
                .color(Color::WHITE),
        );
        Ok(())
    }

    fn draw_highlighted_score(&self, canvas: &mut Canvas, ctx: &mut ggez::Context) -> GameResult {
        let (p1_color, p2_color, p1_scale, p2_scale) = match self.flash_winner {
            Some(1) => (Color::GREEN, Color::WHITE, 2.0, 1.0),
            Some(2) => (Color::WHITE, Color::GREEN, 1.0, 2.0),
            _ => (Color::WHITE, Color::WHITE, 1.0, 1.0),
        };

        // Draw Player 1 score
        let p1_text =
            Text::new(TextFragment::new(format!("Player 1: {}", self.player1)).scale(p1_scale));
        let p1_dims = p1_text.measure(ctx)?;

        canvas.draw(
            &p1_text,
            DrawParam::default()
                .dest([SCREEN_WIDTH / 4.0 - p1_dims.x / 2.0, 20.0])
                .color(p1_color),
        );

        // Draw Player 2 score
        let p2_text =
            Text::new(TextFragment::new(format!("Player 2: {}", self.player2)).scale(p2_scale));
        let p2_dims = p2_text.measure(ctx)?;

        canvas.draw(
            &p2_text,
            DrawParam::default()
                .dest([3.0 * SCREEN_WIDTH / 4.0 - p2_dims.x / 2.0, 20.0])
                .color(p2_color),
        );

        Ok(())
    }

    pub fn update(&mut self) {
        if let Some(flash_start) = self.flash_start {
            if flash_start.elapsed().as_secs_f32() >= 3.0 {
                self.flash_winner = None;
                self.flash_start = None;
            }
        }
    }

    pub fn reset(&mut self) {
        self.player1 = 0;
        self.player2 = 0;
        self.flash_winner = None;
        self.flash_start = None;
    }
}
