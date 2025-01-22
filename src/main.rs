use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Color, DrawParam, Mesh, Rect, Text};
use ggez::{Context, ContextBuilder, GameResult};
use std::collections::HashSet;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 100.0;
const BALL_SIZE: f32 = 20.0;
const PADDLE_SPEED: f32 = 500.0;
const BALL_SPEED: f32 = 300.0;
const DESIRED_FPS: u32 = 60; // Target frame rate
const COLLISION_TOLERANCE: f32 = 1.0; // Tolerance for edge collisions

struct GameState {
    player1_y: f32,
    player2_y: f32,
    ball_x: f32,
    ball_y: f32,
    ball_dx: f32,
    ball_dy: f32,
    score1: u32,
    score2: u32,
    game_running: bool,             // Tracks whether the game is running
    pressed_keys: HashSet<KeyCode>, // Tracks the keys currently pressed
}

impl GameState {
    fn new() -> Self {
        GameState {
            player1_y: (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0,
            player2_y: (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0,
            ball_x: SCREEN_WIDTH / 2.0 - BALL_SIZE / 2.0,
            ball_y: SCREEN_HEIGHT / 2.0 - BALL_SIZE / 2.0,
            ball_dx: BALL_SPEED,
            ball_dy: BALL_SPEED,
            score1: 0,
            score2: 0,
            game_running: false, // Start with the game stopped
            pressed_keys: HashSet::new(),
        }
    }

    fn reset_ball(&mut self) {
        self.ball_x = SCREEN_WIDTH / 2.0 - BALL_SIZE / 2.0;
        self.ball_y = SCREEN_HEIGHT / 2.0 - BALL_SIZE / 2.0;
        self.ball_dx = BALL_SPEED;
        self.ball_dy = BALL_SPEED;
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const FIXED_TIMESTEP: f32 = 1.0 / DESIRED_FPS as f32;

        while ggez::timer::check_update_time(ctx, DESIRED_FPS) {
            if self.game_running {
                // Move player 1 paddle based on key presses
                if self.pressed_keys.contains(&KeyCode::Up) {
                    self.player1_y -= PADDLE_SPEED * FIXED_TIMESTEP;
                    self.player1_y = self.player1_y.max(0.0); // Prevent going off top
                }
                if self.pressed_keys.contains(&KeyCode::Down) {
                    self.player1_y += PADDLE_SPEED * FIXED_TIMESTEP;
                    self.player1_y = self.player1_y.min(SCREEN_HEIGHT - PADDLE_HEIGHT);
                    // Prevent going off bottom
                }

                // Ball movement
                self.ball_x += self.ball_dx * FIXED_TIMESTEP;
                self.ball_y += self.ball_dy * FIXED_TIMESTEP;

                // Ball collision with top and bottom
                if self.ball_y <= COLLISION_TOLERANCE {
                    self.ball_y = COLLISION_TOLERANCE; // Snap to edge
                    self.ball_dy = self.ball_dy.abs(); // Reflect downwards
                } else if self.ball_y + BALL_SIZE >= SCREEN_HEIGHT - COLLISION_TOLERANCE {
                    self.ball_y = SCREEN_HEIGHT - BALL_SIZE - COLLISION_TOLERANCE; // Snap to edge
                    self.ball_dy = -self.ball_dy.abs(); // Reflect upwards
                }

                // Ball collision with paddles
                if (self.ball_x <= PADDLE_WIDTH
                    && self.ball_y + BALL_SIZE >= self.player1_y
                    && self.ball_y <= self.player1_y + PADDLE_HEIGHT)
                    || (self.ball_x + BALL_SIZE >= SCREEN_WIDTH - PADDLE_WIDTH
                        && self.ball_y + BALL_SIZE >= self.player2_y
                        && self.ball_y <= self.player2_y + PADDLE_HEIGHT)
                {
                    self.ball_dx = -self.ball_dx;
                }

                // Ball out of bounds
                if self.ball_x <= 0.0 {
                    self.score2 += 1;
                    self.reset_ball();
                } else if self.ball_x + BALL_SIZE >= SCREEN_WIDTH {
                    self.score1 += 1;
                    self.reset_ball();
                }

                // Player 2 AI
                if self.ball_y > self.player2_y + PADDLE_HEIGHT / 2.0 {
                    self.player2_y += PADDLE_SPEED * FIXED_TIMESTEP;
                    self.player2_y = self.player2_y.min(SCREEN_HEIGHT - PADDLE_HEIGHT);
                // Prevent going off bottom
                } else if self.ball_y < self.player2_y + PADDLE_HEIGHT / 2.0 {
                    self.player2_y -= PADDLE_SPEED * FIXED_TIMESTEP;
                    self.player2_y = self.player2_y.max(0.0); // Prevent going off top
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(30, 30, 30)); // Dark background

        // Draw paddles
        let paddle1 = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(0.0, self.player1_y, PADDLE_WIDTH, PADDLE_HEIGHT),
            Color::from_rgb(0, 255, 0), // Green paddle
        )?;
        let paddle2 = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(
                SCREEN_WIDTH - PADDLE_WIDTH,
                self.player2_y,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),
            Color::from_rgb(0, 0, 255), // Blue paddle
        )?;
        let ball = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(self.ball_x, self.ball_y, BALL_SIZE, BALL_SIZE),
            Color::from_rgb(255, 255, 0), // Yellow ball
        )?;

        // Draw score
        let score_display = Text::new(format!(
            "Player 1: {}  |  Player 2: {}",
            self.score1, self.score2
        ));
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

        graphics::draw(ctx, &paddle1, DrawParam::default())?;
        graphics::draw(ctx, &paddle2, DrawParam::default())?;
        graphics::draw(ctx, &ball, DrawParam::default())?;
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
            KeyCode::S => self.game_running = true,  // Start game
            KeyCode::P => self.game_running = false, // Stop game
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
