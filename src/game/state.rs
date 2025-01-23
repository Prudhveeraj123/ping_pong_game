use crate::components::{ball::Ball, paddle::Paddle, score::Score};
use crate::game::constants::*;
use crate::graphics::renderer::GameRenderer;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Canvas, Color};
use ggez::input::keyboard::{KeyCode, KeyInput};
use rand::Rng;
use std::collections::HashSet;
use std::time::Instant;

pub struct GameState {
    pub player1: Paddle,
    pub player2: Paddle,
    pub ball: Ball,
    pub score: Score,
    pub game_running: bool,
    pub pressed_keys: HashSet<KeyCode>,
    pub last_winner: Option<u8>,
    pub countdown_start: Option<Instant>,
    pub point_scored: bool,
}

impl GameState {
    pub fn new() -> Self {
        let mut state = GameState {
            player1: Paddle::new(0.0, (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0),
            player2: Paddle::new(
                SCREEN_WIDTH - PADDLE_WIDTH,
                (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0,
            ),
            ball: Ball::new(),
            score: Score::new(),
            game_running: false,
            pressed_keys: HashSet::new(),
            last_winner: None,
            countdown_start: None,
            point_scored: false,
        };
        state.reset_ball();
        state
    }

    fn handle_countdown(&mut self, countdown_start: Instant, delta: f32) {
        let elapsed = countdown_start.elapsed().as_secs_f32();

        // Move AI paddle to middle during countdown
        let middle_position = (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0;
        let distance_to_middle = middle_position - self.player2.y;

        if distance_to_middle.abs() > 1.0 {
            let direction = distance_to_middle.signum();
            self.player2.move_by(direction * AI_PADDLE_SPEED * delta);
        }

        if elapsed >= COUNTDOWN_DURATION {
            self.countdown_start = None;
            println!("Countdown finished, starting ball movement");
            self.start_ball();
        }
    }

    fn start_ball(&mut self) {
        let mut rng = rand::thread_rng();
        self.ball.dx = match self.last_winner {
            Some(1) => -BALL_SPEED,
            Some(2) => BALL_SPEED,
            _ => {
                if rng.gen_bool(0.5) {
                    BALL_SPEED
                } else {
                    -BALL_SPEED
                }
            }
        };
        self.ball.dy = if rng.gen_bool(0.5) {
            BALL_SPEED
        } else {
            -BALL_SPEED
        };
        self.point_scored = false;
        println!(
            "Ball started with velocity: ({}, {})",
            self.ball.dx, self.ball.dy
        );
    }

    fn handle_input(&mut self, delta: f32) {
        if self.pressed_keys.contains(&KeyCode::Up) {
            self.player1.move_by(-PLAYER_PADDLE_SPEED * delta);
        }
        if self.pressed_keys.contains(&KeyCode::Down) {
            self.player1.move_by(PLAYER_PADDLE_SPEED * delta);
        }
    }

    fn handle_collisions(&mut self) {
        // Wall collisions
        if self.ball.y - BALL_RADIUS <= COLLISION_TOLERANCE {
            self.ball.y = BALL_RADIUS + COLLISION_TOLERANCE;
            self.ball.dy = self.ball.dy.abs();
        } else if self.ball.y + BALL_RADIUS >= SCREEN_HEIGHT - COLLISION_TOLERANCE {
            self.ball.y = SCREEN_HEIGHT - BALL_RADIUS - COLLISION_TOLERANCE;
            self.ball.dy = -self.ball.dy.abs();
        }

        // Paddle collisions
        if self.ball.x - BALL_RADIUS <= PADDLE_WIDTH
            && self.ball.y >= self.player1.y
            && self.ball.y <= self.player1.y + PADDLE_HEIGHT
        {
            self.ball.dx = self.ball.dx.abs();
        }

        if self.ball.x + BALL_RADIUS >= SCREEN_WIDTH - PADDLE_WIDTH
            && self.ball.y >= self.player2.y
            && self.ball.y <= self.player2.y + PADDLE_HEIGHT
        {
            self.ball.dx = -self.ball.dx.abs();
        }

        // Scoring
        if self.ball.x - BALL_RADIUS <= 0.0 {
            println!("Player 2 scored!");
            self.score.increment_player2();
            self.last_winner = Some(2);
            self.reset_ball();
        } else if self.ball.x + BALL_RADIUS >= SCREEN_WIDTH {
            println!("Player 1 scored!");
            self.score.increment_player1();
            self.last_winner = Some(1);
            self.reset_ball();
        }
    }

    fn reset_ball(&mut self) {
        println!("Resetting ball position");
        self.ball.x = SCREEN_WIDTH / 2.0;
        self.ball.y = SCREEN_HEIGHT / 2.0;
        self.ball.dx = 0.0;
        self.ball.dy = 0.0;
        self.point_scored = true;
        self.countdown_start = Some(Instant::now());
    }

    fn update_ai_paddle(&mut self, delta: f32) {
        if self.ball.dx > 0.0 {
            let paddle_center = self.player2.y + PADDLE_HEIGHT / 2.0;
            let mut rng = rand::thread_rng();

            let reaction_speed = AI_PADDLE_SPEED - 10.0;
            let hesitation = if rng.gen_bool(0.08) { 0.0 } else { 1.0 };
            let error_margin: f32 = rng.gen_range(-3.0..3.0);

            if self.ball.y + error_margin > paddle_center {
                self.player2.move_by(reaction_speed * hesitation * delta);
            } else if self.ball.y + error_margin < paddle_center {
                self.player2.move_by(-reaction_speed * hesitation * delta);
            }
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let delta = ctx.time.delta().as_secs_f32();

        if let Some(countdown_start) = self.countdown_start {
            self.handle_countdown(countdown_start, delta);
        }

        if self.game_running {
            self.handle_input(delta);

            if self.countdown_start.is_none() {
                self.ball.update(delta);
                self.handle_collisions();
                self.update_ai_paddle(delta);
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(30, 30, 30));
        let mut renderer = GameRenderer::new(ctx);
        renderer.render(&mut canvas, self)?;
        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        input: KeyInput,
        _repeat: bool,
    ) -> ggez::GameResult {
        if let Some(keycode) = input.keycode {
            println!("Key pressed: {:?}", keycode);
            match keycode {
                KeyCode::S => {
                    println!("Starting game...");
                    self.game_running = true;
                    if self.countdown_start.is_none() {
                        self.countdown_start = Some(Instant::now());
                    }
                }
                KeyCode::P => {
                    println!("Pausing game...");
                    self.game_running = false;
                }
                KeyCode::R => {
                    println!("Resetting game...");
                    self.score.reset();
                    self.game_running = false;
                    self.reset_ball();
                }
                _ => {
                    self.pressed_keys.insert(keycode);
                }
            }
        }
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut ggez::Context, input: KeyInput) -> ggez::GameResult {
        if let Some(keycode) = input.keycode {
            self.pressed_keys.remove(&keycode);
        }
        Ok(())
    }
}
