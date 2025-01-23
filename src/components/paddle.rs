use crate::game::constants::*;
use ggez::graphics::{self, Color, DrawMode, Mesh, MeshBuilder, Rect};
use ggez::GameResult;

pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub color: Color,
}

impl Paddle {
    pub fn new(x: f32, y: f32) -> Self {
        Paddle {
            x,
            y,
            color: Color::WHITE,
        }
    }

    pub fn move_by(&mut self, amount: f32) {
        self.y += amount;
        self.y = self.y.clamp(0.0, SCREEN_HEIGHT - PADDLE_HEIGHT);
    }

    pub fn get_mesh(&self, ctx: &mut ggez::Context) -> GameResult<Mesh> {
        Mesh::new_rounded_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(self.x, self.y, PADDLE_WIDTH, PADDLE_HEIGHT),
            5.0,
            self.color,
        )
    }
}
