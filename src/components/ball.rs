//! This is the main file that controls how the ball works in our Pong game
//! It handles everything about the ball - where it is, how it moves, and how it looks

// First, we need to bring in some useful tools from other parts of our code
use crate::game::constants::*; // This gives us access to important game settings like screen size
use ggez::graphics::{
    Color,    // Lets us set colors (like making the ball yellow)
    DrawMode, // Helps us choose if shapes should be filled in or just outlined
    Mesh,     // Lets us create shapes we can draw on the screen
};

// This is like a blueprint for our ball - it stores all the important information about it
pub struct Ball {
    pub x: f32,  // Where the ball is from left to right (higher = more right)
    pub y: f32,  // Where the ball is from top to bottom (higher = more down)
    pub dx: f32, // How fast the ball is moving left or right
    // (positive = moving right, negative = moving left)
    pub dy: f32, // How fast the ball is moving up or down
    // (positive = moving down, negative = moving up)
    pub color: Color, // What color the ball should be
}

// Here we define all the things our ball can do
impl Ball {
    // This function creates a new ball in the middle of the screen
    pub fn new() -> Self {
        Ball {
            x: SCREEN_WIDTH / 2.0,               // Put the ball in the middle horizontally
            y: SCREEN_HEIGHT / 2.0,              // Put the ball in the middle vertically
            dx: 0.0,                             // Start with the ball not moving left or right
            dy: 0.0,                             // Start with the ball not moving up or down
            color: Color::from_rgb(255, 255, 0), // Make the ball yellow
        }
    }

    // This function moves the ball based on how much time has passed
    pub fn update(&mut self, delta: f32) {
        // Move the ball horizontally:
        // New position = current position + (speed × time passed)
        self.x += self.dx * delta;

        // Move the ball vertically:
        // New position = current position + (speed × time passed)
        self.y += self.dy * delta;
    }

    // This function creates the actual circle shape that will be drawn on the screen
    pub fn get_mesh(&self, ctx: &mut ggez::Context) -> ggez::GameResult<Mesh> {
        // Create a new circle shape with these settings:
        Mesh::new_circle(
            ctx,
            DrawMode::fill(), // Make it a solid circle (not just an outline)
            ggez::mint::Point2 {
                x: self.x, // Place it at the ball's current horizontal position
                y: self.y, // Place it at the ball's current vertical position
            },
            BALL_RADIUS, // Make it this big (size comes from our game settings)
            0.1,         // How smooth to make the circle (lower = smoother)
            self.color,  // Color it yellow (or whatever color we set)
        )
    }
}
