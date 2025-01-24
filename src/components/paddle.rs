// This file contains the code for the paddles that players control in the game

// Import needed tools and settings for our game
use crate::game::constants::*;
use ggez::graphics::{
    Color,    // For setting paddle colors
    DrawMode, // For choosing how shapes are drawn
    Mesh,     // For creating shapes we can draw
    Rect,     // For making rectangular shapes
};
use ggez::GameResult;

// Define what makes up a paddle
pub struct Paddle {
    pub x: f32,       // Position from left side of screen
    pub y: f32,       // Position from top of screen
    pub color: Color, // Paddle's color
}

// Define what a paddle can do
impl Paddle {
    // Create a new paddle at a specific position
    pub fn new(x: f32, y: f32) -> Self {
        Paddle {
            x,                   // Set left position
            y,                   // Set top position
            color: Color::WHITE, // Make it white
        }
    }

    // Move the paddle up or down while keeping it on screen
    pub fn move_by(&mut self, amount: f32) {
        // Update paddle position
        self.y += amount;

        // Keep paddle within screen boundaries
        self.y = self.y.clamp(0.0, SCREEN_HEIGHT - PADDLE_HEIGHT);
    }

    // Create the actual shape that will be drawn on screen
    pub fn get_mesh(&self, ctx: &mut ggez::Context) -> GameResult<Mesh> {
        // Make a rounded rectangle for the paddle
        Mesh::new_rounded_rectangle(
            ctx,
            DrawMode::fill(), // Make it solid, not hollow
            Rect::new(
                self.x,        // Left edge
                self.y,        // Top edge
                PADDLE_WIDTH,  // How wide
                PADDLE_HEIGHT, // How tall
            ),
            5.0,        // How rounded the corners are
            self.color, // What color to use
        )
    }
}
