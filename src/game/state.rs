// Importing modules and components from the crate
// Ball, Paddle, and Score are game components likely representing game objects
use crate::components::{ball::Ball, paddle::Paddle, score::Score};

// Importing constants used in the game logic, such as screen dimensions or movement speeds
use crate::game::constants::*;

// Importing the renderer for rendering game graphics
use crate::graphics::renderer::GameRenderer;

// Importing traits and types required for event handling
use ggez::event::EventHandler;

// Importing graphics utilities from the ggez library
use ggez::graphics::{Canvas, Color, DrawParam, Text, TextFragment};

// Importing keyboard input utilities for capturing player actions
use ggez::input::keyboard::{KeyCode, KeyInput};

// Importing random number generation functionality
use rand::Rng;

// Importing HashSet for managing collections of unique items
use std::collections::HashSet;

// Importing Instant for handling timing-related operations
use std::time::Instant;

// Struct to represent the state of the game
pub struct GameState {
    pub player1: Paddle,                // Player 1's paddle (controlled by the user)
    pub player2: Paddle,                // Player 2's paddle (controlled by AI)
    pub ball: Ball,                     // The ball used in the game
    pub score: Score,                   // Tracks the scores of both players
    pub game_running: bool,             // Indicates if the game is currently running
    pub pressed_keys: HashSet<KeyCode>, // Stores the keys currently being pressed
    pub last_winner: Option<u8>,        // The last player to score a point (1 or 2)
    pub countdown_start: Option<Instant>, // Timer for the countdown before starting a new point
    pub point_scored: bool,             // Indicates if a point was scored
    pub should_exit: bool,              // Flag to indicate if the game should exit
    pub game_over: bool,                // Indicates if the game is over
    pub winner: Option<u8>,             // Stores the winner of the game (1 or 2)
}

impl GameState {
    // Constructor to create a new game state
    pub fn new() -> Self {
        // Initialize the game state with default values
        let mut state = GameState {
            player1: Paddle::new(0.0, (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0), // Left paddle at center
            player2: Paddle::new(
                SCREEN_WIDTH - PADDLE_WIDTH,
                (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0,
            ), // Right paddle at center
            ball: Ball::new(),                                                // Initialize the ball
            score: Score::new(),          // Initialize the score tracker
            game_running: false,          // Game is not running initially
            pressed_keys: HashSet::new(), // No keys are pressed initially
            last_winner: None,            // No points scored yet
            countdown_start: None,        // Countdown timer is not active
            point_scored: false,          // No points scored initially
            should_exit: false,           // Game should not exit initially
            game_over: false,             // Game is not over initially
            winner: None,                 // No winner initially
        };

        // Set the ball's initial position and velocity
        state.ball.x = SCREEN_WIDTH / 2.0;
        state.ball.y = SCREEN_HEIGHT / 2.0;
        state.ball.dx = 0.0;
        state.ball.dy = 0.0;
        state
    }

    // Handle the countdown timer before starting a new point
    fn handle_countdown(&mut self, countdown_start: Instant, delta: f32) {
        let elapsed = countdown_start.elapsed().as_secs_f32(); // Calculate elapsed time

        // Move the AI paddle towards the middle of the screen during the countdown
        let middle_position = (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0;
        let distance_to_middle = middle_position - self.player2.y;

        if distance_to_middle.abs() > 1.0 {
            let direction = distance_to_middle.signum(); // Determine direction to move
            self.player2.move_by(direction * AI_PADDLE_SPEED * delta);
        }

        // Start the ball movement after the countdown ends
        if elapsed >= COUNTDOWN_DURATION {
            self.countdown_start = None; // Reset countdown timer
            self.start_ball(); // Launch the ball
        }
    }

    // Start the ball movement in a random direction
    fn start_ball(&mut self) {
        let mut rng = rand::thread_rng(); // Random number generator

        // Set the horizontal direction of the ball based on the last winner
        self.ball.dx = match self.last_winner {
            Some(2) => BALL_SPEED,  // If Player 2 scored, move right
            Some(1) => -BALL_SPEED, // If Player 1 scored, move left
            Some(_) | None => {
                // Randomize the direction if no prior winner
                if rng.gen_bool(0.5) {
                    BALL_SPEED
                } else {
                    -BALL_SPEED
                }
            }
        };

        // Set the vertical direction of the ball randomly
        self.ball.dy = if rng.gen_bool(0.5) {
            BALL_SPEED
        } else {
            -BALL_SPEED
        };
        self.point_scored = false; // Reset the point scored flag
    }

    // Handle user input to move the paddle
    fn handle_input(&mut self, delta: f32) {
        if self.pressed_keys.contains(&KeyCode::Up) {
            // Move paddle up if 'Up' key is pressed
            self.player1.move_by(-PLAYER_PADDLE_SPEED * delta);
        }
        if self.pressed_keys.contains(&KeyCode::Down) {
            // Move paddle down if 'Down' key is pressed
            self.player1.move_by(PLAYER_PADDLE_SPEED * delta);
        }
    }

    // Check if any player has won the game
    fn check_winner(&mut self, player: u8) {
        let score = if player == 1 {
            self.score.player1
        } else {
            self.score.player2
        };

        // Declare the game over if the score reaches the winning threshold
        if score >= 3 {
            self.game_over = true;
            self.winner = Some(player); // Set the winner
            self.game_running = false; // Stop the game
            self.ball.color = Color::from_rgb(30, 30, 30); // Dim the ball color
            self.last_winner = None; // Reset last winner
        }
    }

    // Handle collisions between the ball and game objects (walls, paddles)
    fn handle_collisions(&mut self) {
        // Ball bounces off the top wall
        if self.ball.y - BALL_RADIUS <= COLLISION_TOLERANCE {
            self.ball.y = BALL_RADIUS + COLLISION_TOLERANCE;
            self.ball.dy = self.ball.dy.abs();
        }
        // Ball bounces off the bottom wall
        else if self.ball.y + BALL_RADIUS >= SCREEN_HEIGHT - COLLISION_TOLERANCE {
            self.ball.y = SCREEN_HEIGHT - BALL_RADIUS - COLLISION_TOLERANCE;
            self.ball.dy = -self.ball.dy.abs();
        }

        // Ball hits Player 1's paddle
        if self.ball.x - BALL_RADIUS <= PADDLE_WIDTH
            && self.ball.y >= self.player1.y
            && self.ball.y <= self.player1.y + PADDLE_HEIGHT
        {
            self.ball.dx = self.ball.dx.abs();
        }

        // Ball hits Player 2's paddle
        if self.ball.x + BALL_RADIUS >= SCREEN_WIDTH - PADDLE_WIDTH
            && self.ball.y >= self.player2.y
            && self.ball.y <= self.player2.y + PADDLE_HEIGHT
        {
            self.ball.dx = -self.ball.dx.abs();
        }

        // Ball goes past Player 1 (Player 2 scores)
        if self.ball.x - BALL_RADIUS <= 0.0 {
            self.score.increment_player2();
            self.last_winner = Some(2);
            self.check_winner(2);
            self.reset_ball();
        }
        // Ball goes past Player 2 (Player 1 scores)
        else if self.ball.x + BALL_RADIUS >= SCREEN_WIDTH {
            self.score.increment_player1();
            self.last_winner = Some(1);
            self.check_winner(1);
            self.reset_ball();
        }
    }

    // Reset the ball to the center of the screen after a point
    fn reset_ball(&mut self) {
        self.ball.x = SCREEN_WIDTH / 2.0;
        self.ball.y = SCREEN_HEIGHT / 2.0;
        self.ball.dx = 0.0;
        self.ball.dy = 0.0;
        self.point_scored = true;

        // Start a countdown for the next point if the game is not over
        if self.game_running && !self.game_over {
            self.countdown_start = Some(Instant::now());
        }
    }

    // Update AI paddle position to follow the ball
    fn update_ai_paddle(&mut self, delta: f32) {
        if self.ball.dx > 0.0 {
            let paddle_center = self.player2.y + PADDLE_HEIGHT / 2.0; // Get the center of the AI paddle
            let mut rng = rand::thread_rng(); // Random number generator

            let reaction_speed = AI_PADDLE_SPEED - 10.0; // Adjust reaction speed
            let hesitation = if rng.gen_bool(0.08) { 0.0 } else { 1.0 }; // Simulate hesitation
            let error_margin: f32 = rng.gen_range(-3.0..3.0); // Add random error to movement

            // Move the AI paddle up or down based on the ball's position
            if self.ball.y + error_margin > paddle_center {
                self.player2.move_by(reaction_speed * hesitation * delta);
            } else if self.ball.y + error_margin < paddle_center {
                self.player2.move_by(-reaction_speed * hesitation * delta);
            }
        }
    }

    // Display game instructions and messages
    fn draw_instructions(&self, canvas: &mut Canvas, ctx: &mut ggez::Context) -> ggez::GameResult {
        if self.game_over {
            // Display "Game Over" message based on who won
            let winner_text = if self.winner.unwrap() == 1 {
                "You Won!\n\nGame Over".to_string()
            } else {
                "You Lost!\n\nGame Over".to_string()
            };

            let text_fragment = TextFragment::new(winner_text)
                .scale(24.0)
                .color(Color::WHITE);
            let game_over_text = Text::new(text_fragment);
            let dims = game_over_text.measure(ctx)?;

            // Center the "Game Over" text on the screen
            canvas.draw(
                &game_over_text,
                DrawParam::default().dest([
                    SCREEN_WIDTH / 2.0 - dims.x / 2.0,
                    SCREEN_HEIGHT / 2.0 - dims.y,
                ]),
            );
        } else if !self.game_running {
            // Display the "First to score 3 wins" message when game is not running
            let start_text = "First to score 3 wins";
            let text_fragment = TextFragment::new(start_text)
                .scale(24.0)
                .color(Color::WHITE);
            let start_game_text = Text::new(text_fragment);
            let dims = start_game_text.measure(ctx)?;

            canvas.draw(
                &start_game_text,
                DrawParam::default().dest([
                    SCREEN_WIDTH / 2.0 - dims.x / 2.0,
                    SCREEN_HEIGHT / 2.0 - dims.y / 0.5,
                ]),
            );
        }

        // Display game instructions (dynamic based on game state)
        let instructions = if !self.game_over {
            "Press S to Start, R to Reset, E to Exit"
        } else {
            "Press R to Restart, E to Exit"
        };

        let text_fragment = TextFragment::new(instructions)
            .scale(14.0)
            .color(Color::WHITE);
        let instruction_text = Text::new(text_fragment);
        let dims = instruction_text.measure(ctx)?;

        // Position the instructions near the bottom of the screen
        canvas.draw(
            &instruction_text,
            DrawParam::default().dest([
                SCREEN_WIDTH / 2.0 - dims.x / 2.0,
                SCREEN_HEIGHT - dims.y - 20.0,
            ]),
        );

        Ok(())
    }
}

impl EventHandler for GameState {
    // Update game state for each frame
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if self.should_exit {
            ctx.request_quit(); // Exit the game if the exit flag is set
            return Ok(());
        }

        let delta = ctx.time.delta().as_secs_f32(); // Time since last frame

        // Handle countdown if it is active
        if let Some(countdown_start) = self.countdown_start {
            self.handle_countdown(countdown_start, delta);
        }

        if self.game_running {
            self.handle_input(delta); // Process user input

            if self.countdown_start.is_none() {
                self.ball.update(delta); // Move the ball
                self.handle_collisions(); // Check for collisions
                self.update_ai_paddle(delta); // Update AI paddle movement
            }
        }

        Ok(())
    }

    // Draw the game state on the screen
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(30, 30, 30)); // Clear the screen with a dark background
        let mut renderer = GameRenderer::new(ctx); // Initialize the renderer
        renderer.render(&mut canvas, self)?; // Render the game objects
        self.draw_instructions(&mut canvas, ctx)?; // Draw game instructions
        canvas.finish(ctx)?; // Display the frame
        Ok(())
    }

    // Handle key press events
    fn key_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        input: KeyInput,
        _repeat: bool,
    ) -> ggez::GameResult {
        if let Some(keycode) = input.keycode {
            match keycode {
                KeyCode::S => {
                    // Start the game when 'S' is pressed
                    if !self.game_running && !self.game_over {
                        self.game_running = true;
                        self.countdown_start = Some(Instant::now());
                    }
                }
                KeyCode::P => {
                    // Pause the game when 'P' is pressed
                    self.game_running = false;
                }
                KeyCode::E => {
                    // Exit the game when 'E' is pressed
                    self.should_exit = true;
                }
                KeyCode::R => {
                    // Reset the game when 'R' is pressed
                    self.score.reset();
                    self.game_running = false;
                    self.game_over = false;
                    self.winner = None;

                    // Reset ball and paddle positions
                    self.ball.x = SCREEN_WIDTH / 2.0;
                    self.ball.y = SCREEN_HEIGHT / 2.0;
                    self.ball.dx = 0.0;
                    self.ball.dy = 0.0;
                    self.ball.color = Color::from_rgb(255, 255, 0);

                    let middle_y = (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0;
                    self.player1.y = middle_y;
                    self.player2.y = middle_y;

                    self.point_scored = false;
                    self.countdown_start = None;
                    self.last_winner = None;
                }
                _ => {
                    // Add any other pressed key to the set of active keys
                    self.pressed_keys.insert(keycode);
                }
            }
        }
        Ok(())
    }

    // Handle key release events
    fn key_up_event(&mut self, _ctx: &mut ggez::Context, input: KeyInput) -> ggez::GameResult {
        if let Some(keycode) = input.keycode {
            self.pressed_keys.remove(&keycode); // Remove the released key from the set
        }
        Ok(())
    }
}
