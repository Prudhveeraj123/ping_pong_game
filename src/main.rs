use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Color, DrawParam, Mesh, MeshBuilder, Text};
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;
use std::collections::HashSet;
use std::time::Instant;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const PADDLE_WIDTH: f32 = 15.0;
const PADDLE_HEIGHT: f32 = 100.0;
const BALL_RADIUS: f32 = 10.0;
const PLAYER_PADDLE_SPEED: f32 = 500.0;
const AI_PADDLE_SPEED: f32 = 300.0;
const BALL_SPEED: f32 = 300.0;
const DESIRED_FPS: u32 = 60;
const COLLISION_TOLERANCE: f32 = 1.0;
const COUNTDOWN_DURATION: f32 = 3.0;

struct GameState {
    player1_y: f32,
    player2_y: f32,
    ball_x: f32,
    ball_y: f32,
    ball_dx: f32,
    ball_dy: f32,
    score1: u32,
    score2: u32,
    game_running: bool,
    pressed_keys: HashSet<KeyCode>,
    last_winner: Option<u8>,
    countdown_start: Option<Instant>,
    point_scored: bool,
    score_flash_winner: Option<u8>,
    score_flash_start: Option<Instant>,
}

impl GameState {
    fn new() -> Self {
        GameState {
            player1_y: (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0,
            player2_y: (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0,
            ball_x: SCREEN_WIDTH / 2.0,
            ball_y: SCREEN_HEIGHT / 2.0,
            ball_dx: BALL_SPEED,
            ball_dy: BALL_SPEED,
            score1: 0,
            score2: 0,
            game_running: false,
            pressed_keys: HashSet::new(),
            last_winner: None,
            countdown_start: None,
            point_scored: false,
            score_flash_winner: None,
            score_flash_start: None,
        }
    }

    fn reset_ball(&mut self) {
        self.ball_x = SCREEN_WIDTH / 2.0;
        self.ball_y = SCREEN_HEIGHT / 2.0;
        self.point_scored = true;
        self.countdown_start = Some(Instant::now());
        self.score_flash_start = Some(Instant::now());

        // Ball direction will be set after countdown
        self.ball_dx = 0.0;
        self.ball_dy = 0.0;
    }

    fn start_ball(&mut self) {
        let mut rng = rand::thread_rng();
        self.ball_dx = match self.last_winner {
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
        self.ball_dy = if rng.gen_bool(0.5) {
            BALL_SPEED
        } else {
            -BALL_SPEED
        };
        self.point_scored = false;
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let delta = ggez::timer::delta(ctx).as_secs_f32();

        // Handle countdown timer
        if let Some(countdown_start) = self.countdown_start {
            let elapsed = countdown_start.elapsed().as_secs_f32();
            if elapsed >= COUNTDOWN_DURATION {
                self.countdown_start = None;
                self.start_ball();
            }
        }

        // Reset score flash effect after 3 seconds
        if let Some(flash_start) = self.score_flash_start {
            if flash_start.elapsed().as_secs_f32() >= 3.0 {
                self.score_flash_winner = None;
                self.score_flash_start = None;
            }
        }

        if self.game_running && self.countdown_start.is_none() {
            // Player 1 paddle movement
            if self.pressed_keys.contains(&KeyCode::Up) {
                self.player1_y -= PLAYER_PADDLE_SPEED * delta;
                self.player1_y = self.player1_y.max(0.0);
            }
            if self.pressed_keys.contains(&KeyCode::Down) {
                self.player1_y += PLAYER_PADDLE_SPEED * delta;
                self.player1_y = self.player1_y.min(SCREEN_HEIGHT - PADDLE_HEIGHT);
            }

            // Ball movement
            self.ball_x += self.ball_dx * delta;
            self.ball_y += self.ball_dy * delta;

            // Handle ball collision with walls
            if self.ball_y - BALL_RADIUS <= COLLISION_TOLERANCE {
                self.ball_y = BALL_RADIUS + COLLISION_TOLERANCE;
                self.ball_dy = self.ball_dy.abs();
            } else if self.ball_y + BALL_RADIUS >= SCREEN_HEIGHT - COLLISION_TOLERANCE {
                self.ball_y = SCREEN_HEIGHT - BALL_RADIUS - COLLISION_TOLERANCE;
                self.ball_dy = -self.ball_dy.abs();
            }

            // Ball collision with paddles
            if self.ball_x - BALL_RADIUS <= PADDLE_WIDTH
                && self.ball_y >= self.player1_y
                && self.ball_y <= self.player1_y + PADDLE_HEIGHT
            {
                self.ball_dx = self.ball_dx.abs();
            }

            if self.ball_x + BALL_RADIUS >= SCREEN_WIDTH - PADDLE_WIDTH
                && self.ball_y >= self.player2_y
                && self.ball_y <= self.player2_y + PADDLE_HEIGHT
            {
                self.ball_dx = -self.ball_dx.abs();
            }

            // Ball out of bounds
            if self.ball_x - BALL_RADIUS <= 0.0 {
                self.score2 += 1;
                self.last_winner = Some(2);
                self.score_flash_winner = Some(2);
                self.reset_ball();
            } else if self.ball_x + BALL_RADIUS >= SCREEN_WIDTH {
                self.score1 += 1;
                self.last_winner = Some(1);
                self.score_flash_winner = Some(1);
                self.reset_ball();
            }

            // AI Paddle movement
            if self.ball_dx > 0.0 {
                let paddle_center = self.player2_y + PADDLE_HEIGHT / 2.0;

                let reaction_speed = AI_PADDLE_SPEED - 10.0;
                let mut rng = rand::thread_rng();
                let hesitation = if rng.gen_bool(0.08) { 0.0 } else { 1.0 };
                let error_margin: f32 = rng.gen_range(-3.0..3.0);

                if self.ball_y + error_margin > paddle_center {
                    self.player2_y += reaction_speed * hesitation * delta;
                    self.player2_y = self.player2_y.min(SCREEN_HEIGHT - PADDLE_HEIGHT);
                } else if self.ball_y + error_margin < paddle_center {
                    self.player2_y -= reaction_speed * hesitation * delta;
                    self.player2_y = self.player2_y.max(0.0);
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(30, 30, 30));

        // Draw countdown if active
        if let Some(countdown_start) = self.countdown_start {
            let elapsed = countdown_start.elapsed().as_secs_f32();
            let count = (COUNTDOWN_DURATION - elapsed).ceil() as i32;
            if count > 0 {
                let countdown_text = Text::new(count.to_string());
                let text_dims = countdown_text.dimensions(ctx);
                graphics::draw(
                    ctx,
                    &countdown_text,
                    DrawParam::default()
                        .dest([
                            SCREEN_WIDTH / 2.0 - text_dims.w / 2.0,
                            SCREEN_HEIGHT / 2.0 - text_dims.h / 2.0,
                        ])
                        .scale([3.0, 3.0])
                        .color(Color::WHITE),
                )?;
            }
        }

        // Draw paddles with normal colors
        let paddle1_color = Color::from_rgb(0, 255, 0);
        let paddle2_color = Color::from_rgb(0, 0, 255);

        let paddle1 = MeshBuilder::new()
            .rounded_rectangle(
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, self.player1_y, PADDLE_WIDTH, PADDLE_HEIGHT),
                5.0,
                paddle1_color,
            )?
            .build(ctx)?;

        let paddle2 = MeshBuilder::new()
            .rounded_rectangle(
                graphics::DrawMode::fill(),
                graphics::Rect::new(
                    SCREEN_WIDTH - PADDLE_WIDTH,
                    self.player2_y,
                    PADDLE_WIDTH,
                    PADDLE_HEIGHT,
                ),
                5.0,
                paddle2_color,
            )?
            .build(ctx)?;

        // Draw ball only when countdown is not active
        let ball = if self.countdown_start.is_none() {
            Some(Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                ggez::mint::Point2 {
                    x: self.ball_x,
                    y: self.ball_y,
                },
                BALL_RADIUS,
                0.1,
                Color::from_rgb(255, 255, 0),
            )?)
        } else {
            None
        };

        // Draw scores separately to handle different sizes
        match self.score_flash_winner {
            Some(1) => {
                let winner = Text::new(format!("Player 1: {}", self.score1));
                let other = Text::new(format!("Player 2: {}", self.score2));
                graphics::draw(
                    ctx,
                    &winner,
                    (
                        ggez::mint::Point2 {
                            x: SCREEN_WIDTH / 4.0 - 50.0,
                            y: 20.0,
                        },
                        Color::from_rgb(0, 255, 0),
                    ),
                )?;
                graphics::draw(
                    ctx,
                    &other,
                    (
                        ggez::mint::Point2 {
                            x: 3.0 * SCREEN_WIDTH / 4.0 - 50.0,
                            y: 20.0,
                        },
                        Color::WHITE,
                    ),
                )?;
                // Draw bigger winning score
                let big_score = Text::new(self.score1.to_string());
                graphics::draw(
                    ctx,
                    &big_score,
                    DrawParam::default()
                        .dest(ggez::mint::Point2 {
                            x: SCREEN_WIDTH / 4.0 - 20.0,
                            y: 50.0,
                        })
                        .scale([3.0, 3.0])
                        .color(Color::from_rgb(0, 255, 0)),
                )?;
            }
            Some(2) => {
                let other = Text::new(format!("Player 1: {}", self.score1));
                let winner = Text::new(format!("Player 2: {}", self.score2));
                graphics::draw(
                    ctx,
                    &other,
                    (
                        ggez::mint::Point2 {
                            x: SCREEN_WIDTH / 4.0 - 50.0,
                            y: 20.0,
                        },
                        Color::WHITE,
                    ),
                )?;
                graphics::draw(
                    ctx,
                    &winner,
                    (
                        ggez::mint::Point2 {
                            x: 3.0 * SCREEN_WIDTH / 4.0 - 50.0,
                            y: 20.0,
                        },
                        Color::from_rgb(0, 255, 0),
                    ),
                )?;
                // Draw bigger winning score
                let big_score = Text::new(self.score2.to_string());
                graphics::draw(
                    ctx,
                    &big_score,
                    DrawParam::default()
                        .dest(ggez::mint::Point2 {
                            x: 3.0 * SCREEN_WIDTH / 4.0 - 20.0,
                            y: 50.0,
                        })
                        .scale([3.0, 3.0])
                        .color(Color::from_rgb(0, 255, 0)),
                )?;
            }
            _ => {
                let score_text = format!("Player 1: {}  |  Player 2: {}", self.score1, self.score2);
                let score_display = Text::new(score_text);
                graphics::draw(
                    ctx,
                    &score_display,
                    (
                        ggez::mint::Point2 {
                            x: SCREEN_WIDTH / 2.0 - 100.0,
                            y: 20.0,
                        },
                        Color::WHITE,
                    ),
                )?;
            }
        };

        // Draw instructions
        let instructions = Text::new("Press S to Start, Press P to Pause");
        graphics::draw(
            ctx,
            &instructions,
            (
                ggez::mint::Point2 {
                    x: SCREEN_WIDTH / 2.0 - 100.0,
                    y: SCREEN_HEIGHT - 40.0,
                },
                Color::WHITE,
            ),
        )?;

        graphics::draw(ctx, &paddle1, DrawParam::default())?;
        graphics::draw(ctx, &paddle2, DrawParam::default())?;
        if let Some(ball) = ball {
            graphics::draw(ctx, &ball, DrawParam::default())?;
        }
        graphics::present(ctx)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::S => self.game_running = true,
            KeyCode::P => self.game_running = false,
            _ => {
                self.pressed_keys.insert(keycode);
            }
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        self.pressed_keys.remove(&keycode);
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("Ping Pong", "Prudhveraj Botta")
        .window_setup(ggez::conf::WindowSetup::default().title("Ping Pong Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()?;

    let game = GameState::new();
    event::run(ctx, event_loop, game)
}

// added highlighting scores and countdown
