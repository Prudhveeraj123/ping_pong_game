#[cfg(test)]
mod tests {
    use crate::{
        components::ball::Ball,
        game::constants::{BALL_SPEED, SCREEN_HEIGHT, SCREEN_WIDTH},
    };

    use approx::assert_relative_eq;

    #[test]
    fn test_new_ball() {
        let ball = Ball::new();
        assert_eq!(ball.x, SCREEN_WIDTH / 2.0);
        assert_eq!(ball.y, SCREEN_HEIGHT / 2.0);
        assert_eq!(ball.dx, 0.0);
        assert_eq!(ball.dy, 0.0);
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
}
