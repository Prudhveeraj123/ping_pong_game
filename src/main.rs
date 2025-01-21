use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Color, DrawParam, Mesh, Rect};
use ggez::{Context, ContextBuilder, GameResult};

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 100.0;
const BALL_SIZE: f32 = 20.0;
const PADDLE_SPEED: f32 = 500.0;
const BALL_SPEED: f32 = 300.0;

struct GameState {
    player1_y: f32,
    player2_y: f32,
    ball_x: f32,
    ball_y: f32,
    ball_dx: f32,
    ball_dy: f32,
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
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let delta_time = ggez::timer::delta(ctx).as_secs_f32();

        // Move ball
        self.ball_x += self.ball_dx * delta_time;
        self.ball_y += self.ball_dy * delta_time;

        // Ball collision with top and bottom
        if self.ball_y <= 0.0 || self.ball_y + BALL_SIZE >= SCREEN_HEIGHT {
            self.ball_dy = -self.ball_dy;
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
        if self.ball_x <= 0.0 || self.ball_x + BALL_SIZE >= SCREEN_WIDTH {
            self.ball_dx = -self.ball_dx;
            self.ball_x = SCREEN_WIDTH / 2.0 - BALL_SIZE / 2.0;
            self.ball_y = SCREEN_HEIGHT / 2.0 - BALL_SIZE / 2.0;
        }

        // Player 2 AI
        if self.ball_y > self.player2_y + PADDLE_HEIGHT / 2.0 {
            self.player2_y += PADDLE_SPEED * delta_time;
        } else if self.ball_y < self.player2_y + PADDLE_HEIGHT / 2.0 {
            self.player2_y -= PADDLE_SPEED * delta_time;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);

        // Draw paddles
        let paddle1 = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(0.0, self.player1_y, PADDLE_WIDTH, PADDLE_HEIGHT),
            Color::WHITE,
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
            Color::WHITE,
        )?;
        let ball = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(self.ball_x, self.ball_y, BALL_SIZE, BALL_SIZE),
            Color::WHITE,
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
            KeyCode::Up => self.player1_y -= PADDLE_SPEED / 60.0,
            KeyCode::Down => self.player1_y += PADDLE_SPEED / 60.0,
            _ => {}
        }
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
