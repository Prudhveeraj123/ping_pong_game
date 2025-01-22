use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Color, DrawParam, Mesh, MeshBuilder, Text};
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;
use std::collections::HashSet;

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
    last_winner: Option<u8>, // 1 for Player 1, 2 for Player 2
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
        }
    }

    fn reset_ball(&mut self) {
        let mut rng = rand::thread_rng();
        self.ball_x = SCREEN_WIDTH / 2.0;
        self.ball_y = SCREEN_HEIGHT / 2.0;

        // Determine ball direction based on the winner of the previous round
        self.ball_dx = match self.last_winner {
            Some(1) => -BALL_SPEED, // Player 1 won, ball moves toward Player 2
            Some(2) => BALL_SPEED,  // Player 2 won, ball moves toward Player 1
            _ => {
                if rng.gen_bool(0.5) {
                    BALL_SPEED
                } else {
                    -BALL_SPEED
                }
            } // Random if no winner yet
        };

        // Randomize the vertical direction
        self.ball_dy = if rng.gen_bool(0.5) {
            BALL_SPEED
        } else {
            -BALL_SPEED
        };
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const FIXED_TIMESTEP: f32 = 1.0 / DESIRED_FPS as f32;

        while ggez::timer::check_update_time(ctx, DESIRED_FPS) {
            if self.game_running {
                // Player 1 paddle movement
                if self.pressed_keys.contains(&KeyCode::Up) {
                    self.player1_y -= PLAYER_PADDLE_SPEED * FIXED_TIMESTEP;
                    self.player1_y = self.player1_y.max(0.0);
                }
                if self.pressed_keys.contains(&KeyCode::Down) {
                    self.player1_y += PLAYER_PADDLE_SPEED * FIXED_TIMESTEP;
                    self.player1_y = self.player1_y.min(SCREEN_HEIGHT - PADDLE_HEIGHT);
                }

                // Ball movement
                self.ball_x += self.ball_dx * FIXED_TIMESTEP;
                self.ball_y += self.ball_dy * FIXED_TIMESTEP;

                // Ball collision with top and bottom
                if self.ball_y - BALL_RADIUS <= COLLISION_TOLERANCE {
                    self.ball_y = BALL_RADIUS + COLLISION_TOLERANCE;
                    self.ball_dy = self.ball_dy.abs();
                } else if self.ball_y + BALL_RADIUS >= SCREEN_HEIGHT - COLLISION_TOLERANCE {
                    self.ball_y = SCREEN_HEIGHT - BALL_RADIUS - COLLISION_TOLERANCE;
                    self.ball_dy = -self.ball_dy.abs();
                }

                // Ball collision with Player 1 paddle
                if self.ball_x - BALL_RADIUS <= PADDLE_WIDTH
                    && self.ball_y >= self.player1_y
                    && self.ball_y <= self.player1_y + PADDLE_HEIGHT
                {
                    self.ball_dx = self.ball_dx.abs();
                }

                // Ball collision with AI paddle
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
                    self.reset_ball();
                } else if self.ball_x + BALL_RADIUS >= SCREEN_WIDTH {
                    self.score1 += 1;
                    self.last_winner = Some(1);
                    self.reset_ball();
                }

                // AI Paddle movement
                if self.ball_dx > 0.0 {
                    let paddle_center = self.player2_y + PADDLE_HEIGHT / 2.0;

                    if self.ball_y > paddle_center + 10.0 {
                        self.player2_y += AI_PADDLE_SPEED * FIXED_TIMESTEP;
                        self.player2_y = self.player2_y.min(SCREEN_HEIGHT - PADDLE_HEIGHT);
                    } else if self.ball_y < paddle_center - 10.0 {
                        self.player2_y -= AI_PADDLE_SPEED * FIXED_TIMESTEP;
                        self.player2_y = self.player2_y.max(0.0);
                    }
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(30, 30, 30));

        let paddle1 = MeshBuilder::new()
            .rounded_rectangle(
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, self.player1_y, PADDLE_WIDTH, PADDLE_HEIGHT),
                5.0,
                Color::from_rgb(0, 255, 0),
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
                Color::from_rgb(0, 0, 255),
            )?
            .build(ctx)?;

        let ball = Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            ggez::mint::Point2 {
                x: self.ball_x,
                y: self.ball_y,
            },
            BALL_RADIUS,
            0.1,
            Color::from_rgb(255, 255, 0),
        )?;

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
