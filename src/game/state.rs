//! This file contains the main game state and logic for the Pong game

// Import necessary components and modules
use crate::components::{ball::Ball, paddle::Paddle, score::Score}; // Game components
use crate::game::constants::*; // Game constants like speeds, dimensions
use crate::graphics::renderer::GameRenderer; // Rendering functionality
use ggez::event::EventHandler; // Event handling from ggez
use ggez::graphics::{Canvas, Color, DrawParam, Text, TextFragment}; // Graphics components
use ggez::input::keyboard::{KeyCode, KeyInput}; // Keyboard input handling
use rand::Rng; // Random number generation
use std::collections::HashSet; // For tracking pressed keys
use std::time::Instant; // For timing operations

/// Main struct that holds all game state
pub struct GameState {
    pub player1: Paddle,                  // Left paddle (human player)
    pub player2: Paddle,                  // Right paddle (AI player)
    pub ball: Ball,                       // Game ball
    pub score: Score,                     // Game score
    pub game_running: bool,               // Whether game is active
    pub pressed_keys: HashSet<KeyCode>,   // Currently pressed keys
    pub last_winner: Option<u8>,          // Tracks who scored last (1 or 2)
    pub countdown_start: Option<Instant>, // Timer for countdown between points
    pub point_scored: bool,               // Whether a point was just scored
    pub should_exit: bool,                // Flag to indicate if game should exit
}

impl GameState {
    /// Creates a new game state with initial values
    pub fn new() -> Self {
        let mut state = GameState {
            // Initialize player1 paddle on the left side, vertically centered
            player1: Paddle::new(0.0, (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0),

            // Initialize player2 (AI) paddle on the right side, vertically centered
            player2: Paddle::new(
                SCREEN_WIDTH - PADDLE_WIDTH,
                (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0,
            ),
            ball: Ball::new(),
            score: Score::new(),
            game_running: false, // Game starts paused
            pressed_keys: HashSet::new(),
            last_winner: None,
            countdown_start: None,
            point_scored: false,
            should_exit: false,
        };

        // Set initial ball position to center of screen
        state.ball.x = SCREEN_WIDTH / 2.0;
        state.ball.y = SCREEN_HEIGHT / 2.0;
        state.ball.dx = 0.0; // Initial velocity is 0
        state.ball.dy = 0.0;
        state
    }

    /// Handles the countdown timer between points
    /// delta: Time since last frame for smooth movement
    fn handle_countdown(&mut self, countdown_start: Instant, delta: f32) {
        let elapsed = countdown_start.elapsed().as_secs_f32();

        // During countdown, move AI paddle to center position
        let middle_position = (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0;
        let distance_to_middle = middle_position - self.player2.y;

        // Only move if not already centered
        if distance_to_middle.abs() > 1.0 {
            let direction = distance_to_middle.signum();
            self.player2.move_by(direction * AI_PADDLE_SPEED * delta);
        }

        // When countdown finishes, start ball movement
        if elapsed >= COUNTDOWN_DURATION {
            self.countdown_start = None;
            self.start_ball();
        }
    }

    /// Initializes ball movement after countdown
    fn start_ball(&mut self) {
        let mut rng = rand::thread_rng();

        // Set horizontal direction based on who scored last
        self.ball.dx = match self.last_winner {
            Some(1) => -BALL_SPEED, // Ball goes towards player 1
            Some(2) => BALL_SPEED,  // Ball goes towards player 2
            _ => {
                // Random direction for first serve
                if rng.gen_bool(0.5) {
                    BALL_SPEED
                } else {
                    -BALL_SPEED
                }
            }
        };

        // Randomize vertical direction
        self.ball.dy = if rng.gen_bool(0.5) {
            BALL_SPEED
        } else {
            -BALL_SPEED
        };

        self.point_scored = false;
    }

    /// Processes keyboard input for player movement
    fn handle_input(&mut self, delta: f32) {
        // Move paddle up if Up arrow is pressed
        if self.pressed_keys.contains(&KeyCode::Up) {
            self.player1.move_by(-PLAYER_PADDLE_SPEED * delta);
        }
        // Move paddle down if Down arrow is pressed
        if self.pressed_keys.contains(&KeyCode::Down) {
            self.player1.move_by(PLAYER_PADDLE_SPEED * delta);
        }
    }

    /// Handles all collision detection and response
    fn handle_collisions(&mut self) {
        // Ball collision with top and bottom walls
        if self.ball.y - BALL_RADIUS <= COLLISION_TOLERANCE {
            // Hit top wall - bounce down
            self.ball.y = BALL_RADIUS + COLLISION_TOLERANCE;
            self.ball.dy = self.ball.dy.abs();
        } else if self.ball.y + BALL_RADIUS >= SCREEN_HEIGHT - COLLISION_TOLERANCE {
            // Hit bottom wall - bounce up
            self.ball.y = SCREEN_HEIGHT - BALL_RADIUS - COLLISION_TOLERANCE;
            self.ball.dy = -self.ball.dy.abs();
        }

        // Ball collision with player 1's paddle
        if self.ball.x - BALL_RADIUS <= PADDLE_WIDTH
            && self.ball.y >= self.player1.y
            && self.ball.y <= self.player1.y + PADDLE_HEIGHT
        {
            self.ball.dx = self.ball.dx.abs(); // Bounce right
        }

        // Ball collision with player 2's paddle
        if self.ball.x + BALL_RADIUS >= SCREEN_WIDTH - PADDLE_WIDTH
            && self.ball.y >= self.player2.y
            && self.ball.y <= self.player2.y + PADDLE_HEIGHT
        {
            self.ball.dx = -self.ball.dx.abs(); // Bounce left
        }

        // Scoring logic
        if self.ball.x - BALL_RADIUS <= 0.0 {
            // Ball passed left paddle - Player 2 scores
            self.score.increment_player2();
            self.last_winner = Some(2);
            self.reset_ball();
        } else if self.ball.x + BALL_RADIUS >= SCREEN_WIDTH {
            // Ball passed right paddle - Player 1 scores
            self.score.increment_player1();
            self.last_winner = Some(1);
            self.reset_ball();
        }
    }

    /// Resets the ball after a point is scored
    fn reset_ball(&mut self) {
        // Center the ball
        self.ball.x = SCREEN_WIDTH / 2.0;
        self.ball.y = SCREEN_HEIGHT / 2.0;
        // Stop ball movement
        self.ball.dx = 0.0;
        self.ball.dy = 0.0;
        self.point_scored = true;

        // Start countdown only if game is active
        if self.game_running {
            self.countdown_start = Some(Instant::now());
        }
    }

    /// Updates AI paddle movement
    fn update_ai_paddle(&mut self, delta: f32) {
        // Only move if ball is moving towards AI
        if self.ball.dx > 0.0 {
            let paddle_center = self.player2.y + PADDLE_HEIGHT / 2.0;
            let mut rng = rand::thread_rng();

            // Make AI more human-like with delays and errors
            let reaction_speed = AI_PADDLE_SPEED - 10.0;
            let hesitation = if rng.gen_bool(0.08) { 0.0 } else { 1.0 }; // Sometimes hesitate
            let error_margin: f32 = rng.gen_range(-3.0..3.0); // Add some randomness to targeting

            // Move paddle towards ball with error margin
            if self.ball.y + error_margin > paddle_center {
                self.player2.move_by(reaction_speed * hesitation * delta);
            } else if self.ball.y + error_margin < paddle_center {
                self.player2.move_by(-reaction_speed * hesitation * delta);
            }
        }
    }

    /// Draws game instructions at the bottom of the screen
    fn draw_instructions(&self, canvas: &mut Canvas, ctx: &mut ggez::Context) -> ggez::GameResult {
        let instructions = "Press S to Start, R to Reset, E to Exit";

        let text_fragment = TextFragment::new(instructions)
            .scale(14.0)
            .color(Color::WHITE);

        let instruction_text = Text::new(text_fragment);
        let dims = instruction_text.measure(ctx)?;

        // Position text at bottom center of screen
        canvas.draw(
            &instruction_text,
            DrawParam::default().dest([
                SCREEN_WIDTH / 2.0 - dims.x / 2.0, // Center horizontally
                SCREEN_HEIGHT - dims.y - 20.0,     // 20 pixels from bottom
            ]),
        );

        Ok(())
    }
}

/// Implementation of GGEZ's EventHandler trait for game loop control
impl EventHandler for GameState {
    /// Update game state (called each frame)
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        // Check if we should exit the game
        if self.should_exit {
            ctx.request_quit();
            return Ok(());
        }

        let delta = ctx.time.delta().as_secs_f32(); // Time since last frame

        // Handle countdown if active
        if let Some(countdown_start) = self.countdown_start {
            self.handle_countdown(countdown_start, delta);
        }

        // Only process game logic if game is running
        if self.game_running {
            self.handle_input(delta);

            // Only update ball and AI if not in countdown
            if self.countdown_start.is_none() {
                self.ball.update(delta);
                self.handle_collisions();
                self.update_ai_paddle(delta);
            }
        }

        Ok(())
    }

    /// Draw game state (called each frame)
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(30, 30, 30)); // Dark gray background
        let mut renderer = GameRenderer::new(ctx);
        renderer.render(&mut canvas, self)?;
        self.draw_instructions(&mut canvas, ctx)?;
        canvas.finish(ctx)?;
        Ok(())
    }

    /// Handle keyboard key press events
    fn key_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        input: KeyInput,
        _repeat: bool,
    ) -> ggez::GameResult {
        if let Some(keycode) = input.keycode {
            match keycode {
                KeyCode::S => {
                    // Start game if not already running
                    if !self.game_running {
                        self.game_running = true;
                        self.countdown_start = Some(Instant::now());
                    }
                }
                KeyCode::P => {
                    // Pause game
                    self.game_running = false;
                }
                KeyCode::E => {
                    // Set exit flag
                    self.should_exit = true;
                }
                KeyCode::R => {
                    // Reset everything to initial state
                    self.score.reset();
                    self.game_running = false;

                    // Reset ball position
                    self.ball.x = SCREEN_WIDTH / 2.0;
                    self.ball.y = SCREEN_HEIGHT / 2.0;
                    self.ball.dx = 0.0;
                    self.ball.dy = 0.0;

                    // Center both paddles
                    let middle_y = (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0;
                    self.player1.y = middle_y;
                    self.player2.y = middle_y;

                    self.point_scored = false;
                    self.countdown_start = None;
                    self.last_winner = None;
                }
                _ => {
                    // Store any other pressed key
                    self.pressed_keys.insert(keycode);
                }
            }
        }
        Ok(())
    }

    /// Handle keyboard key release events
    fn key_up_event(&mut self, _ctx: &mut ggez::Context, input: KeyInput) -> ggez::GameResult {
        if let Some(keycode) = input.keycode {
            self.pressed_keys.remove(&keycode); // Remove released key from pressed keys set
        }
        Ok(())
    }
}
