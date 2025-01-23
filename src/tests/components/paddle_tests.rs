#[cfg(test)]
mod tests {
    use crate::{components::paddle::Paddle, game::constants::{PADDLE_HEIGHT, SCREEN_HEIGHT}};

    use approx::assert_relative_eq;
    use ggez::graphics::Color;

    #[test]
    fn test_new_paddle() {
        let paddle = Paddle::new(10.0, 20.0);
        assert_relative_eq!(paddle.x, 10.0);
        assert_relative_eq!(paddle.y, 20.0);
        assert_eq!(paddle.color, Color::WHITE);
    }

    #[test]
    fn test_paddle_move_within_bounds() {
        let mut paddle = Paddle::new(0.0, SCREEN_HEIGHT / 2.0);
        let initial_y = paddle.y;
        
        // Move up
        paddle.move_by(-50.0);
        assert!(paddle.y < initial_y);
        
        // Move to initial position        
        paddle.move_by(50.0);

        //Move down
        paddle.move_by(50.0);
        assert!(paddle.y > initial_y);
    }

    #[test]
    fn test_paddle_upper_boundary() {
        let mut paddle = Paddle::new(0.0, 10.0);
        
        // Try to move past top boundary
        paddle.move_by(-100.0);
        assert_relative_eq!(paddle.y, 0.0);
    }

    #[test]
    fn test_paddle_lower_boundary() {
        let mut paddle = Paddle::new(0.0, SCREEN_HEIGHT - PADDLE_HEIGHT - 10.0);
        
        // Try to move past bottom boundary
        paddle.move_by(100.0);
        assert_relative_eq!(paddle.y, SCREEN_HEIGHT - PADDLE_HEIGHT);
    }
    
}