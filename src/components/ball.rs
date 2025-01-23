use crate::game::constants::*;
use ggez::graphics::{self, Color, DrawMode, Mesh};
use rand::Rng;

pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub color: Color,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            x: SCREEN_WIDTH / 2.0,
            y: SCREEN_HEIGHT / 2.0,
            dx: 0.0,
            dy: 0.0,
            color: Color::from_rgb(255, 255, 0),
        }
    }

    pub fn reset(&mut self, last_winner: Option<u8>) {
        self.x = SCREEN_WIDTH / 2.0;
        self.y = SCREEN_HEIGHT / 2.0;

        let mut rng = rand::thread_rng();
        self.dx = match last_winner {
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
        self.dy = if rng.gen_bool(0.5) {
            BALL_SPEED
        } else {
            -BALL_SPEED
        };
    }

    pub fn update(&mut self, delta: f32) {
        self.x += self.dx * delta;
        self.y += self.dy * delta;
    }

    pub fn get_mesh(&self, ctx: &mut ggez::Context) -> ggez::GameResult<Mesh> {
        Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            ggez::mint::Point2 {
                x: self.x,
                y: self.y,
            },
            BALL_RADIUS,
            0.1,
            self.color,
        )
    }

    pub fn reverse_dx(&mut self) {
        self.dx = -self.dx;
    }

    pub fn reverse_dy(&mut self) {
        self.dy = -self.dy;
    }

    pub fn stop(&mut self) {
        self.dx = 0.0;
        self.dy = 0.0;
    }

    pub fn is_moving(&self) -> bool {
        self.dx != 0.0 || self.dy != 0.0
    }
}
