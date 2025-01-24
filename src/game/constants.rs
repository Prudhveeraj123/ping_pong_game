// This file contains all the main settings that control how the game works

// Screen dimensions (in pixels)
pub const SCREEN_WIDTH: f32 = 900.0; // How wide the game window is
pub const SCREEN_HEIGHT: f32 = 600.0; // How tall the game window is

// Paddle settings
pub const PADDLE_WIDTH: f32 = 15.0; // How wide each paddle is
pub const PADDLE_HEIGHT: f32 = 100.0; // How tall each paddle is

// Ball settings
pub const BALL_RADIUS: f32 = 10.0; // How big the ball is

// Movement speeds (pixels per second)
pub const PLAYER_PADDLE_SPEED: f32 = 500.0; // How fast player's paddle moves
pub const AI_PADDLE_SPEED: f32 = 300.0; // How fast computer's paddle moves
pub const BALL_SPEED: f32 = 300.0; // How fast the ball moves

// Game mechanics
pub const COLLISION_TOLERANCE: f32 = 1.0; // Helps prevent ball from getting stuck
pub const COUNTDOWN_DURATION: f32 = 3.0; // How long the "3,2,1" countdown lasts
