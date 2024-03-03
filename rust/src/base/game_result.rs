#[derive(Debug, PartialEq)]
pub enum GameStatus {
    Win,
    Draw,
    Lose,
}

#[derive(Debug)]
pub struct GameResult {
    pub status: GameStatus,
    pub score: f32,
    pub points: isize,
    pub message: String,
}

impl GameResult {
    // 白番目線での結果
    // ここまでやる必要もないけど win = 1.0 なのを表現したかったので
    pub fn new(points: isize) -> Self {
        if points > 0 {
            GameResult {
                status: GameStatus::Win,
                score: 1.0,
                points,
                message: "A wins".to_string(),
            }
        } else if points == 0 {
            GameResult {
                status: GameStatus::Draw,
                score: 0.5,
                points,
                message: "DRAW".to_string(),
            }
        } else {
            GameResult {
                status: GameStatus::Lose,
                score: 0.0,
                points,
                message: "B wins".to_string(),
            }
        }
    }

    // Method to display the game result
    pub fn display(&self) -> String {
        format!(
            "Status: {:?}, Score: {}, Points: {}, Message: {}",
            self.status, self.score, self.points, self.message
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status() {
        let result = GameResult::new(0);
        assert_eq!(result.message, "DRAW");
        assert_eq!(result.status, GameStatus::Draw);

        let result = GameResult::new(10);
        assert_eq!(result.message, "A wins");
        assert_eq!(result.status, GameStatus::Win);

        let result = GameResult::new(-3);
        assert_eq!(result.message, "B wins");
        assert_eq!(result.status, GameStatus::Lose);
        assert_eq!(
            result.display(),
            "Status: Lose, Score: 0, Points: -3, Message: B wins"
        )
    }
}
