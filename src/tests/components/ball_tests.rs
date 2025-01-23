#[cfg(test)]
mod tests {
    use crate::{components::ball::Ball, game::constants::{BALL_SPEED, SCREEN_HEIGHT, SCREEN_WIDTH}};

    use approx::assert_relative_eq;

    #[test]
    fn test_new_ball() {
        let ball = Ball::new();
        assert_eq!(ball.x, SCREEN_WIDTH / 2.0);
        assert_eq!(ball.y, SCREEN_HEIGHT / 2.0);
        assert_eq!(ball.dx, 0.0);
        assert_eq!(ball.dy, 0.0);
        assert!(!ball.is_moving());
    }

    #[test]
    fn test_ball_movement() {
        let mut ball = Ball::new();
        ball.dx = BALL_SPEED;
        ball.dy = BALL_SPEED;
        let initial_x = ball.x;
        let initial_y = ball.y;
        
        ball.update(1.0);
        
        assert_relative_eq!(ball.x, initial_x + BALL_SPEED);
        assert_relative_eq!(ball.y, initial_y + BALL_SPEED);
    }

    #[test]
    fn test_ball_reset() {
        let mut ball = Ball::new();
        
        // Test reset towards player 1
        ball.reset(Some(2));
        assert_eq!(ball.x, SCREEN_WIDTH / 2.0, "x position incorrect after reset(Some(2))");
        assert_eq!(ball.y, SCREEN_HEIGHT / 2.0, "y position incorrect after reset(Some(2))");
        assert_eq!(ball.dx, BALL_SPEED, "dx incorrect after reset(Some(2))");
        assert!(ball.dy.abs() == BALL_SPEED, "dy magnitude incorrect after reset(Some(2))");
        
        // Test reset towards player 2
        ball.reset(Some(1));
        assert_eq!(ball.x, SCREEN_WIDTH / 2.0, "x position incorrect after reset(Some(1))");
        assert_eq!(ball.y, SCREEN_HEIGHT / 2.0, "y position incorrect after reset(Some(1))");
        assert_eq!(ball.dx, -BALL_SPEED, "dx incorrect after reset(Some(1))");
        assert!(ball.dy.abs() == BALL_SPEED, "dy magnitude incorrect after reset(Some(1))");
    }

    #[test]
    fn test_direction_reversal() {
        let mut ball = Ball::new();
        ball.dx = BALL_SPEED;
        ball.dy = BALL_SPEED;
        
        ball.reverse_dx();
        assert_eq!(ball.dx, -BALL_SPEED);
        
        ball.reverse_dy();
        assert_eq!(ball.dy, -BALL_SPEED);
    }

    #[test]
    fn test_ball_stop() {
        let mut ball = Ball::new();
        ball.dx = BALL_SPEED;
        ball.dy = BALL_SPEED;
        assert!(ball.is_moving());
        
        ball.stop();
        assert_eq!(ball.dx, 0.0);
        assert_eq!(ball.dy, 0.0);
        assert!(!ball.is_moving());
    }

    #[test]
    fn test_random_reset() {
        let mut ball = Ball::new();
        ball.reset(None);
        assert!(ball.is_moving());
        assert!(ball.dx.abs() == BALL_SPEED);
        assert!(ball.dy.abs() == BALL_SPEED);
    }

}