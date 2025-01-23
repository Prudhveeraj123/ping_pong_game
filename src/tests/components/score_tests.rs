#[cfg(test)]
mod tests {

    use crate::components::score::Score;

    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_new_score() {
        let score = Score::new();
        assert_eq!(score.player1, 0);
        assert_eq!(score.player2, 0);
        assert!(score.flash_winner.is_none());
        assert!(score.flash_start.is_none());
    }

    #[test]
    fn test_increment_player1() {
        let mut score = Score::new();
        score.increment_player1();
        assert_eq!(score.player1, 1);
        assert_eq!(score.player2, 0);
        assert_eq!(score.flash_winner, Some(1));
        assert!(score.flash_start.is_some());
    }

    #[test]
    fn test_increment_player2() {
        let mut score = Score::new();
        score.increment_player2();
        assert_eq!(score.player1, 0);
        assert_eq!(score.player2, 1);
        assert_eq!(score.flash_winner, Some(2));
        assert!(score.flash_start.is_some());
    }

    #[test]
    fn test_score_reset() {
        let mut score = Score::new();
        score.increment_player1();
        score.increment_player2();
        score.reset();

        assert_eq!(score.player1, 0);
        assert_eq!(score.player2, 0);
        assert!(score.flash_winner.is_none());
        assert!(score.flash_start.is_none());
    }

    #[test]
    fn test_score_update_flash_timeout() {
        let mut score = Score::new();
        score.increment_player1();

        assert!(score.flash_winner.is_some());
        assert!(score.flash_start.is_some());
    }
}
