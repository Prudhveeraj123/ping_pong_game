use crate::game::constants::*;
use crate::game::state::GameState;
use ggez::graphics::{self, Canvas, Color, DrawParam, Drawable, Text, TextFragment};
use ggez::mint::Point2;
use std::time::Instant;

pub struct GameRenderer<'a> {
    ctx: &'a mut ggez::Context,
}

impl<'a> GameRenderer<'a> {
    pub fn new(ctx: &'a mut ggez::Context) -> Self {
        GameRenderer { ctx }
    }

    pub fn render(&mut self, canvas: &mut Canvas, state: &GameState) -> ggez::GameResult {
        // Draw paddles
        self.draw_paddles(canvas, state)?;

        // Draw ball only if not in countdown
        if state.countdown_start.is_none() {
            self.draw_ball(canvas, state)?;
        }

        // Draw score
        self.draw_score(canvas, state)?;

        // Draw countdown if active
        if let Some(countdown_start) = state.countdown_start {
            self.draw_countdown(canvas, countdown_start)?;
        }

        // Draw instructions
        self.draw_instructions(canvas)?;

        // Draw pause screen if game is not running
        if !state.game_running {
            self.draw_pause_screen(canvas)?;
        }

        Ok(())
    }

    fn draw_paddles(&mut self, canvas: &mut Canvas, state: &GameState) -> ggez::GameResult {
        // Draw player 1's paddle (left)
        let paddle1_mesh = state.player1.get_mesh(self.ctx)?;
        canvas.draw(
            &paddle1_mesh,
            DrawParam::default().color(Color::from_rgb(0, 255, 0)),
        );

        // Draw player 2's paddle (right)
        let paddle2_mesh = state.player2.get_mesh(self.ctx)?;
        canvas.draw(
            &paddle2_mesh,
            DrawParam::default().color(Color::from_rgb(0, 0, 255)),
        );

        Ok(())
    }

    fn draw_ball(&mut self, canvas: &mut Canvas, state: &GameState) -> ggez::GameResult {
        let ball_mesh = state.ball.get_mesh(self.ctx)?;
        canvas.draw(&ball_mesh, DrawParam::default().color(Color::YELLOW));
        Ok(())
    }

    fn draw_score(&mut self, canvas: &mut Canvas, state: &GameState) -> ggez::GameResult {
        // If there's a scoring flash, draw highlighted scores
        if state.score.flash_start.is_some() {
            let (winner_score, winner_pos_x) = if state.score.flash_winner == Some(1) {
                (state.score.player1, SCREEN_WIDTH / 4.0)
            } else {
                (state.score.player2, 3.0 * SCREEN_WIDTH / 4.0)
            };

            // Draw large winning score
            let winner_frag = TextFragment::new(format!("{}", winner_score))
                .scale(8.0)
                .color(Color::GREEN);

            let winner_text = Text::new(winner_frag);
            let dims = winner_text.measure(self.ctx)?;

            canvas.draw(
                &winner_text,
                DrawParam::default().dest([
                    winner_pos_x - dims.x / 2.0,
                    SCREEN_HEIGHT / 4.0 - dims.y / 2.0,
                ]),
            );
        }

        // Draw normal scores with larger text
        let score_frag = TextFragment::new(format!(
            "Player 1: {}     Player 2: {}",
            state.score.player1, state.score.player2
        ))
        .scale(2.5);

        let score_text = Text::new(score_frag);
        let dims = score_text.measure(self.ctx)?;

        canvas.draw(
            &score_text,
            DrawParam::default()
                .dest([SCREEN_WIDTH / 2.0 - dims.x / 2.0, 40.0])
                .color(Color::WHITE),
        );

        Ok(())
    }

    fn draw_countdown(
        &mut self,
        canvas: &mut Canvas,
        countdown_start: Instant,
    ) -> ggez::GameResult {
        let elapsed = countdown_start.elapsed().as_secs_f32();
        let count = (COUNTDOWN_DURATION - elapsed).ceil() as i32;

        if count > 0 {
            // Choose color based on count
            let color = match count {
                3 => Color::RED,
                2 => Color::YELLOW,
                1 => Color::GREEN,
                _ => Color::WHITE,
            };

            let fragment = TextFragment::new(count.to_string()).scale(8.0).color(color);

            let countdown_text = Text::new(fragment);
            let dims = countdown_text.measure(self.ctx)?;

            // Draw a semi-transparent background circle for better visibility
            let circle = graphics::Mesh::new_circle(
                self.ctx,
                graphics::DrawMode::fill(),
                Point2 {
                    x: SCREEN_WIDTH / 2.0,
                    y: SCREEN_HEIGHT / 2.0,
                },
                dims.y / 1.5,
                0.1,
                Color::new(0.0, 0.0, 0.0, 0.5),
            )?;
            canvas.draw(&circle, DrawParam::default());

            // Draw the countdown number
            canvas.draw(
                &countdown_text,
                DrawParam::default().dest([
                    SCREEN_WIDTH / 2.0 - dims.x / 2.0,
                    SCREEN_HEIGHT / 2.0 - dims.y / 2.0,
                ]),
            );
        }
        Ok(())
    }

    fn draw_instructions(&mut self, canvas: &mut Canvas) -> ggez::GameResult {
        let instructions = [
            ("Press S to Start", 120.0),
            ("Press P to Pause", 90.0),
            ("Press R to Reset", 60.0),
            ("Use Up/Down arrows to move paddle", 30.0),
        ];

        for (text, y_offset) in instructions.iter() {
            let text_fragment = TextFragment::new(*text)
                .scale(1.5)
                .color(Color::new(1.0, 1.0, 1.0, 0.8));

            let instruction_text = Text::new(text_fragment);
            let dims = instruction_text.measure(self.ctx)?;

            canvas.draw(
                &instruction_text,
                DrawParam::default()
                    .dest([SCREEN_WIDTH / 2.0 - dims.x / 2.0, SCREEN_HEIGHT - *y_offset]),
            );
        }

        Ok(())
    }

    fn draw_pause_screen(&mut self, canvas: &mut Canvas) -> ggez::GameResult {
        // Create semi-transparent overlay
        let overlay = graphics::Mesh::new_rectangle(
            self.ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT),
            Color::new(0.0, 0.0, 0.0, 0.5),
        )?;

        canvas.draw(&overlay, DrawParam::default());

        // Draw "PAUSED" text with larger size
        let pause_fragment = TextFragment::new("PAUSED").scale(6.0).color(Color::WHITE);

        let pause_text = Text::new(pause_fragment);
        let dims = pause_text.measure(self.ctx)?;

        canvas.draw(
            &pause_text,
            DrawParam::default().dest([
                SCREEN_WIDTH / 2.0 - dims.x / 2.0,
                SCREEN_HEIGHT / 2.0 - dims.y / 2.0,
            ]),
        );

        Ok(())
    }

    pub fn draw_game_over(&mut self, canvas: &mut Canvas, state: &GameState) -> ggez::GameResult {
        // Create dark overlay
        let overlay = graphics::Mesh::new_rectangle(
            self.ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT),
            Color::new(0.0, 0.0, 0.0, 0.8),
        )?;

        canvas.draw(&overlay, DrawParam::default());

        let winner = if state.score.player1 > state.score.player2 {
            "Player 1"
        } else {
            "Player 2"
        };

        // Draw Game Over text with larger size
        let game_over = TextFragment::new("GAME OVER")
            .scale(6.0)
            .color(Color::WHITE);

        let winner_text = TextFragment::new(format!("{} Wins!", winner))
            .scale(4.0)
            .color(Color::GREEN);

        let score_text = TextFragment::new(format!(
            "Final Score: {} - {}",
            state.score.player1, state.score.player2
        ))
        .scale(3.0)
        .color(Color::WHITE);

        let game_over_text = Text::new(game_over);
        let winner_display = Text::new(winner_text);
        let score_display = Text::new(score_text);

        let game_over_dims = game_over_text.measure(self.ctx)?;
        let winner_dims = winner_display.measure(self.ctx)?;
        let score_dims = score_display.measure(self.ctx)?;

        canvas.draw(
            &game_over_text,
            DrawParam::default().dest([
                SCREEN_WIDTH / 2.0 - game_over_dims.x / 2.0,
                SCREEN_HEIGHT / 2.0 - 120.0,
            ]),
        );

        canvas.draw(
            &winner_display,
            DrawParam::default().dest([
                SCREEN_WIDTH / 2.0 - winner_dims.x / 2.0,
                SCREEN_HEIGHT / 2.0 - 20.0,
            ]),
        );

        canvas.draw(
            &score_display,
            DrawParam::default().dest([
                SCREEN_WIDTH / 2.0 - score_dims.x / 2.0,
                SCREEN_HEIGHT / 2.0 + 60.0,
            ]),
        );

        Ok(())
    }
}
