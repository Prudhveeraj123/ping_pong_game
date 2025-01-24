// Import necessary modules and components for the game
use crate::components::{ball::Ball, paddle::Paddle, score::Score};
use crate::game::constants::*;
use crate::graphics::renderer::GameRenderer;
use ggez::event::EventHandler;
use ggez::graphics::{Canvas, Color, DrawParam, Text, TextFragment};
use ggez::input::keyboard::{KeyCode, KeyInput};
use rand::Rng;
use std::collections::HashSet;
use std::time::Instant;

// Main game state structure that holds all game components and their states
pub struct GameState {
    pub player1: Paddle,                  // Left paddle controlled by human player
    pub player2: Paddle,                  // Right paddle controlled by AI
    pub ball: Ball,                       // Ball object with position and movement data
    pub score: Score,                     // Tracks score for both players
    pub game_running: bool,               // Whether game is currently active
    pub pressed_keys: HashSet<KeyCode>,   // Set of currently pressed keyboard keys
    pub last_winner: Option<u8>,          // Stores who won last point (1 or 2)
    pub countdown_start: Option<Instant>, // Timer for countdown between points
    pub point_scored: bool,               // Flag indicating if point was just scored
    pub should_exit: bool,                // Flag to trigger game exit
    pub game_over: bool,                  // Flag indicating if game has ended
    pub winner: Option<u8>,               // Stores the winner of the game (1 or 2)
}

impl GameState {
    // Creates a new game state with default values
    pub fn new() -> Self {
        let mut state = GameState {
            // Initialize player paddles at center height on respective sides
            player1: Paddle::new(0.0, (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0),
            player2: Paddle::new(
                SCREEN_WIDTH - PADDLE_WIDTH,
                (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0,
            ),
            ball: Ball::new(),
            score: Score::new(),
            game_running: false,
            pressed_keys: HashSet::new(),
            last_winner: None,
            countdown_start: None,
            point_scored: false,
            should_exit: false,
            game_over: false,
            winner: None,
        };

        // Place ball at center of screen
        state.ball.x = SCREEN_WIDTH / 2.0;
        state.ball.y = SCREEN_HEIGHT / 2.0;
        state.ball.dx = 0.0; // Initial velocity is 0
        state.ball.dy = 0.0;
        state
    }

    // Manages countdown timer between points and AI paddle centering
    fn handle_countdown(&mut self, countdown_start: Instant, delta: f32) {
        let elapsed = countdown_start.elapsed().as_secs_f32();

        // Center AI paddle during countdown
        let middle_position = (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0;
        let distance_to_middle = middle_position - self.player2.y;

        if distance_to_middle.abs() > 1.0 {
            let direction = distance_to_middle.signum();
            self.player2.move_by(direction * AI_PADDLE_SPEED * delta);
        }

        // Start ball movement when countdown ends
        if elapsed >= COUNTDOWN_DURATION {
            self.countdown_start = None;
            self.start_ball();
        }
    }

    // Initializes ball movement after countdown or point
    fn start_ball(&mut self) {
        let mut rng = rand::thread_rng();

        // Set ball direction based on last point winner
        self.ball.dx = match self.last_winner {
            Some(2) => BALL_SPEED,  // Throw towards Player 2 if they won
            Some(1) => -BALL_SPEED, // Throw towards Player 1 if they won
            Some(_) | None => {
                // Random direction for game start
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

    // Processes keyboard input for player paddle movement
    fn handle_input(&mut self, delta: f32) {
        if self.pressed_keys.contains(&KeyCode::Up) {
            self.player1.move_by(-PLAYER_PADDLE_SPEED * delta); // Move up
        }
        if self.pressed_keys.contains(&KeyCode::Down) {
            self.player1.move_by(PLAYER_PADDLE_SPEED * delta); // Move down
        }
    }

    // Checks if a player has won the game (reached 3 points)
    fn check_winner(&mut self, player: u8) {
        let score = if player == 1 {
            self.score.player1
        } else {
            self.score.player2
        };

        // If score reaches 3, end the game
        if score >= 3 {
            self.game_over = true;
            self.winner = Some(player);
            self.game_running = false;
            self.ball.color = Color::from_rgb(30, 30, 30); // Hide ball
            self.last_winner = None;
        }
    }

    // Handles all collision detection and response
    fn handle_collisions(&mut self) {
        // Ball collision with top and bottom walls
        if self.ball.y - BALL_RADIUS <= COLLISION_TOLERANCE {
            self.ball.y = BALL_RADIUS + COLLISION_TOLERANCE;
            self.ball.dy = self.ball.dy.abs(); // Bounce down
        } else if self.ball.y + BALL_RADIUS >= SCREEN_HEIGHT - COLLISION_TOLERANCE {
            self.ball.y = SCREEN_HEIGHT - BALL_RADIUS - COLLISION_TOLERANCE;
            self.ball.dy = -self.ball.dy.abs(); // Bounce up
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

        // Scoring logic when ball passes paddles
        if self.ball.x - BALL_RADIUS <= 0.0 {
            self.score.increment_player2(); // Player 2 scores
            self.last_winner = Some(2);
            self.check_winner(2);
            self.reset_ball();
        } else if self.ball.x + BALL_RADIUS >= SCREEN_WIDTH {
            self.score.increment_player1(); // Player 1 scores
            self.last_winner = Some(1);
            self.check_winner(1);
            self.reset_ball();
        }
    }

    // Resets ball position after point
    fn reset_ball(&mut self) {
        self.ball.x = SCREEN_WIDTH / 2.0; // Center horizontally
        self.ball.y = SCREEN_HEIGHT / 2.0; // Center vertically
        self.ball.dx = 0.0; // Stop movement
        self.ball.dy = 0.0;
        self.point_scored = true;

        // Start countdown if game is still active
        if self.game_running && !self.game_over {
            self.countdown_start = Some(Instant::now());
        }
    }

    // Controls AI paddle movement with human-like behavior
    fn update_ai_paddle(&mut self, delta: f32) {
        // Only move if ball is coming towards AI
        if self.ball.dx > 0.0 {
            let paddle_center = self.player2.y + PADDLE_HEIGHT / 2.0;
            let mut rng = rand::thread_rng();

            // Add human-like imperfections
            let reaction_speed = AI_PADDLE_SPEED - 10.0;
            let hesitation = if rng.gen_bool(0.08) { 0.0 } else { 1.0 }; // Sometimes hesitate
            let error_margin: f32 = rng.gen_range(-3.0..3.0); // Add targeting error

            // Move paddle towards ball with error margin
            if self.ball.y + error_margin > paddle_center {
                self.player2.move_by(reaction_speed * hesitation * delta);
            } else if self.ball.y + error_margin < paddle_center {
                self.player2.move_by(-reaction_speed * hesitation * delta);
            }
        }
    }

    // Draws game instructions and status messages
    fn draw_instructions(&self, canvas: &mut Canvas, ctx: &mut ggez::Context) -> ggez::GameResult {
        // Show game over message if game ended
        if self.game_over {
            let winner_text = format!("Player {} Won\n\n Game Over", self.winner.unwrap());
            let text_fragment = TextFragment::new(winner_text)
                .scale(24.0)
                .color(Color::WHITE);
            let game_over_text = Text::new(text_fragment);
            let dims = game_over_text.measure(ctx)?;

            canvas.draw(
                &game_over_text,
                DrawParam::default().dest([
                    SCREEN_WIDTH / 2.0 - dims.x / 2.0,
                    SCREEN_HEIGHT / 2.0 - dims.y,
                ]),
            );
        }
        // Show initial game instructions
        else if !self.game_running {
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

        // Show control instructions
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

// Implementation of GGEZ's EventHandler for game loop
impl EventHandler for GameState {
    // Updates game state each frame
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if self.should_exit {
            ctx.request_quit();
            return Ok(());
        }

        let delta = ctx.time.delta().as_secs_f32();

        // Handle countdown between points
        if let Some(countdown_start) = self.countdown_start {
            self.handle_countdown(countdown_start, delta);
        }

        // Update game elements if game is running
        if self.game_running {
            self.handle_input(delta);

            if self.countdown_start.is_none() {
                self.ball.update(delta);
                self.handle_collisions();
                self.update_ai_paddle(delta);
            }
        }

        Ok(())
    }

    // Renders game state each frame
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(30, 30, 30));
        let mut renderer = GameRenderer::new(ctx);
        renderer.render(&mut canvas, self)?;
        self.draw_instructions(&mut canvas, ctx)?;
        canvas.finish(ctx)?;
        Ok(())
    }

    // Handles keyboard key press events
    fn key_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        input: KeyInput,
        _repeat: bool,
    ) -> ggez::GameResult {
        if let Some(keycode) = input.keycode {
            match keycode {
                KeyCode::S => {
                    // Start game if not running and not over
                    if !self.game_running && !self.game_over {
                        self.game_running = true;
                        self.countdown_start = Some(Instant::now());
                    }
                }
                KeyCode::P => {
                    self.game_running = false; // Pause game
                }
                KeyCode::E => {
                    self.should_exit = true; // Exit game
                }
                KeyCode::R => {
                    // Reset game state
                    self.score.reset();
                    self.game_running = false;
                    self.game_over = false;
                    self.winner = None;

                    // Reset ball
                    self.ball.x = SCREEN_WIDTH / 2.0;
                    self.ball.y = SCREEN_HEIGHT / 2.0;
                    self.ball.dx = 0.0;
                    self.ball.dy = 0.0;
                    self.ball.color = Color::from_rgb(255, 255, 0);

                    // Reset paddles
                    let middle_y = (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0;
                    self.player1.y = middle_y;
                    self.player2.y = middle_y;

                    self.point_scored = false;
                    self.countdown_start = None;
                    self.last_winner = None;
                }
                _ => {
                    self.pressed_keys.insert(keycode); // Store other pressed keys
                }
            }
        }
        Ok(())
    }

    // Handles keyboard key release events
    fn key_up_event(&mut self, _ctx: &mut ggez::Context, input: KeyInput) -> ggez::GameResult {
        if let Some(keycode) = input.keycode {
            self.pressed_keys.remove(&keycode); // Remove released key from set
        }
        Ok(())
    }
}
