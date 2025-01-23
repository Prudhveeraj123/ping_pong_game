//! This file defines the paddle component used by players in the Pong game.

// Import necessary modules and constants
use crate::game::constants::*; // Game constants for dimensions and speeds
use ggez::graphics::{
    self,
    Color,       // For paddle color
    DrawMode,    // For specifying how to draw (filled or outlined)
    Mesh,        // For creating drawable shapes
    MeshBuilder, // For building complex shapes
    Rect,        // For defining rectangular shapes
};
use ggez::GameResult; // For error handling

/// Represents a paddle in the game
/// Each player controls a paddle to hit the ball
pub struct Paddle {
    pub x: f32,       // Horizontal position of the paddle (left edge)
    pub y: f32,       // Vertical position of the paddle (top edge)
    pub color: Color, // Color of the paddle (defaults to white)
}

impl Paddle {
    /// Creates a new paddle at the specified position
    ///
    /// # Parameters
    /// * `x`: The horizontal position for the paddle
    /// * `y`: The vertical position for the paddle
    ///
    /// # Returns
    /// A new Paddle instance positioned at (x, y)
    pub fn new(x: f32, y: f32) -> Self {
        Paddle {
            x,                   // Set initial x position
            y,                   // Set initial y position
            color: Color::WHITE, // Default color is white
        }
    }

    /// Moves the paddle up or down by the specified amount
    /// Keeps the paddle within the screen boundaries
    ///
    /// # Parameters
    /// * `amount`: Distance to move (positive = down, negative = up)
    ///            This will typically be PLAYER_PADDLE_SPEED * delta_time
    pub fn move_by(&mut self, amount: f32) {
        // Add the movement amount to current y position
        self.y += amount;

        // Clamp the paddle position to keep it on screen
        // If y < 0, sets to 0
        // If y > screen_height - paddle_height, sets to that value
        // This prevents the paddle from moving off the top or bottom of the screen
        self.y = self.y.clamp(0.0, SCREEN_HEIGHT - PADDLE_HEIGHT);
    }

    /// Creates a mesh (drawable shape) for the paddle
    /// This mesh is what actually gets drawn to the screen
    ///
    /// # Parameters
    /// * `ctx`: The GGEZ graphics context needed for creating meshes
    ///
    /// # Returns
    /// * A Result containing either the paddle mesh or an error
    pub fn get_mesh(&self, ctx: &mut ggez::Context) -> GameResult<Mesh> {
        // Create a new rounded rectangle mesh for the paddle
        Mesh::new_rounded_rectangle(
            ctx,
            DrawMode::fill(), // Make the paddle filled (not just an outline)
            Rect::new(
                self.x,        // Left edge position
                self.y,        // Top edge position
                PADDLE_WIDTH,  // Width from constants
                PADDLE_HEIGHT, // Height from constants
            ),
            5.0,        // Corner radius (makes the corners rounded)
            self.color, // Paddle color (white by default)
        )
    }
}
