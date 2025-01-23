//! This file defines all the constant values used throughout the Pong game.
//! These values control everything from game dimensions to speeds and timings.

/// Width of the game window in pixels
/// 800 pixels gives a good width for most screens
pub const SCREEN_WIDTH: f32 = 800.0;

/// Height of the game window in pixels
/// 600 pixels provides a 4:3 aspect ratio with the width
pub const SCREEN_HEIGHT: f32 = 600.0;

/// Width of each paddle in pixels
/// Thin enough to make the game challenging but not too difficult
pub const PADDLE_WIDTH: f32 = 15.0;

/// Height of each paddle in pixels
/// Large enough to make the game playable but small enough to be challenging
pub const PADDLE_HEIGHT: f32 = 100.0;

/// Radius of the ball in pixels
/// The ball is drawn as a circle, and this determines its size
/// 10 pixels makes it visible but not too large
pub const BALL_RADIUS: f32 = 10.0;

/// Speed at which the player's paddle moves in pixels per second
/// 500 pixels/second gives responsive movement while staying controllable
pub const PLAYER_PADDLE_SPEED: f32 = 500.0;

/// Speed at which the AI paddle moves in pixels per second
/// Slightly slower than player speed to make the game fair
pub const AI_PADDLE_SPEED: f32 = 300.0;

/// Speed at which the ball moves in pixels per second
/// This is the base speed for both horizontal and vertical movement
/// 300 pixels/second makes the game challenging but not impossible
pub const BALL_SPEED: f32 = 300.0;

/// Target frames per second for the game
/// 60 FPS is standard for smooth gameplay on most displays
pub const DESIRED_FPS: u32 = 60;

/// Small value used for collision detection to prevent rounding errors
/// Helps avoid the ball getting stuck in paddles or walls
pub const COLLISION_TOLERANCE: f32 = 1.0;

/// Duration of the countdown timer in seconds
/// Shows "3", "2", "1" before each point starts
pub const COUNTDOWN_DURATION: f32 = 3.0;
