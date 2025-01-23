//! This file defines the ball component for the Pong game, controlling its movement and appearance

// Import necessary modules and types
use crate::game::constants::*; // Game constants for dimensions, speeds, etc.
use ggez::graphics::{
    self,
    Color,    // For ball color
    DrawMode, // For specifying how to draw (filled or outlined)
    Mesh,     // For creating drawable shapes
};
use rand::Rng; // For generating random directions

/// Represents the ball in the game
pub struct Ball {
    pub x: f32,  // Horizontal position of the ball's center
    pub y: f32,  // Vertical position of the ball's center
    pub dx: f32, // Horizontal velocity (speed and direction)
    // Positive = moving right, Negative = moving left
    pub dy: f32, // Vertical velocity (speed and direction)
    // Positive = moving down, Negative = moving up
    pub color: Color, // Ball color
}

impl Ball {
    /// Creates a new ball in the center of the screen
    /// Initially, the ball is stationary (dx and dy are 0)
    pub fn new() -> Self {
        Ball {
            x: SCREEN_WIDTH / 2.0,               // Place ball at horizontal center
            y: SCREEN_HEIGHT / 2.0,              // Place ball at vertical center
            dx: 0.0,                             // No initial horizontal movement
            dy: 0.0,                             // No initial vertical movement
            color: Color::from_rgb(255, 255, 0), // Yellow color
        }
    }

    /// Resets the ball to center and gives it a new direction
    ///
    /// # Parameters
    /// * `last_winner`: Indicates which player scored last (affects initial direction)
    ///   - Some(1): Ball moves towards Player 2
    ///   - Some(2): Ball moves towards Player 1
    ///   - None: Random direction
    pub fn reset(&mut self, last_winner: Option<u8>) {
        // Move ball back to center
        self.x = SCREEN_WIDTH / 2.0;
        self.y = SCREEN_HEIGHT / 2.0;

        // Create random number generator for direction choices
        let mut rng = rand::thread_rng();

        // Determine horizontal direction based on who scored last
        self.dx = match last_winner {
            Some(1) => -BALL_SPEED, // Move towards Player 1
            Some(2) => BALL_SPEED,  // Move towards Player 2
            _ => {
                // Random horizontal direction if no last winner
                if rng.gen_bool(0.5) {
                    BALL_SPEED // Move right
                } else {
                    -BALL_SPEED // Move left
                }
            }
        };

        // Randomly choose vertical direction
        self.dy = if rng.gen_bool(0.5) {
            BALL_SPEED // Move down
        } else {
            -BALL_SPEED // Move up
        };
    }

    /// Updates ball position based on its velocity
    ///
    /// # Parameters
    /// * `delta`: Time since last frame (for smooth movement)
    pub fn update(&mut self, delta: f32) {
        // Move ball horizontally based on speed and time
        self.x += self.dx * delta;
        // Move ball vertically based on speed and time
        self.y += self.dy * delta;
    }

    /// Creates a circular mesh (drawable shape) for the ball
    ///
    /// # Parameters
    /// * `ctx`: The GGEZ graphics context needed for creating meshes
    ///
    /// # Returns
    /// * A Result containing either the ball mesh or an error
    pub fn get_mesh(&self, ctx: &mut ggez::Context) -> ggez::GameResult<Mesh> {
        Mesh::new_circle(
            ctx,
            DrawMode::fill(), // Make the ball filled (not just an outline)
            ggez::mint::Point2 {
                // Center point of the circle
                x: self.x, // Ball's horizontal position
                y: self.y, // Ball's vertical position
            },
            BALL_RADIUS, // Size of the ball
            0.1,         // Tolerance (how smooth the circle looks)
            self.color,  // Ball color (yellow)
        )
    }

    /// Reverses the horizontal direction of the ball
    /// Used when the ball hits a paddle
    pub fn reverse_dx(&mut self) {
        self.dx = -self.dx; // Flip horizontal velocity
    }

    /// Reverses the vertical direction of the ball
    /// Used when the ball hits the top or bottom of the screen
    pub fn reverse_dy(&mut self) {
        self.dy = -self.dy; // Flip vertical velocity
    }

    /// Stops the ball's movement by setting all velocities to 0
    pub fn stop(&mut self) {
        self.dx = 0.0; // No horizontal movement
        self.dy = 0.0; // No vertical movement
    }

    /// Checks if the ball is currently moving
    ///
    /// # Returns
    /// * `true` if the ball has any velocity (moving)
    /// * `false` if the ball is completely still
    pub fn is_moving(&self) -> bool {
        self.dx != 0.0 || self.dy != 0.0 // True if moving in either direction
    }
}
