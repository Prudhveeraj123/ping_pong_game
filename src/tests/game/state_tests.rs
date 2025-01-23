#[cfg(test)]
mod tests {
    use crate::game::{constants::{SCREEN_HEIGHT, SCREEN_WIDTH}, state::GameState};

    #[test]
    fn test_new_game_state() {
        let state = GameState::new();
        assert!(!state.game_running);
        assert_eq!(state.ball.x, SCREEN_WIDTH / 2.0);
        assert_eq!(state.ball.y, SCREEN_HEIGHT / 2.0);
        assert_eq!(state.ball.dx, 0.0);
        assert_eq!(state.ball.dy, 0.0);
        assert_eq!(state.score.player1, 0);
        assert_eq!(state.score.player2, 0);
    }

}