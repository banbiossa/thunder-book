use std::sync::Arc;

use crate::base::game_result;
use rand::{rngs::StdRng, Rng, SeedableRng};

#[derive(Debug, Clone)]
pub struct MazeParams {
    pub height: usize,
    pub width: usize,
    pub end_turn: usize,
}

#[derive(Debug, Clone)]
struct Character {
    y: usize,
    x: usize,
    mark: String,
    game_score: usize,
}

impl Character {
    fn new(y: usize, x: usize, mark: String) -> Self {
        Character {
            y,
            x,
            mark,
            game_score: 0,
        }
    }
}

// takes state and player_id, returns action
pub type ActionFunc = Arc<dyn Fn(&SimultaneousMazeState, usize) -> usize>;

#[derive(Debug, Clone)]
pub struct SimultaneousMazeState {
    points: Vec<Vec<usize>>,
    pub turn: usize,
    characters: Vec<Character>,
    params: MazeParams,
}

impl SimultaneousMazeState {
    const DY: [isize; 4] = [0, 0, 1, -1];
    const DX: [isize; 4] = [1, -1, 0, 0];

    pub fn new(seed: u64, params: MazeParams) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let characters = vec![
            Character::new(
                params.height / 2,
                params.width / 2 - 1,
                "A".to_string(),
            ),
            Character::new(
                params.height / 2,
                params.width / 2 + 1,
                "B".to_string(),
            ),
        ];
        let mut points = vec![vec![0; params.width]; params.height];
        for y in 0..params.height {
            for x in 0..(params.width / 2 + 1) {
                if characters[0].y == y && characters[0].x == x {
                    continue;
                }
                if characters[1].y == y && characters[0].x == x {
                    continue;
                }
                let ty = y;
                let mut tx = x;
                let point = rng.gen_range(0..=9);
                points[ty][tx] = point;
                tx = params.width - 1 - x;
                points[ty][tx] = point;
            }
        }

        SimultaneousMazeState {
            points,
            turn: 0,
            params,
            characters,
        }
    }

    pub fn is_done(&self) -> bool {
        self.turn >= self.params.end_turn
    }

    pub fn advance(&mut self, actions: Vec<usize>) {
        for player in 0..=1 {
            let character = &mut self.characters[player];
            let action = actions[player];
            character.y = (character.y as isize + Self::DY[action]) as usize;
            character.x = (character.x as isize + Self::DX[action]) as usize;
            let point = self.points[character.y][character.x];
            character.game_score += point;
        }
        for character in &self.characters {
            self.points[character.y][character.x] = 0;
        }
        self.turn += 1;
    }

    pub fn legal_actions(&self, player_id: usize) -> Vec<usize> {
        let mut actions = Vec::new();
        let character = &self.characters[player_id];
        for action in 0..4 {
            let ty = character.y as isize + Self::DY[action];
            let tx = character.x as isize + Self::DX[action];
            if ty >= 0
                && (ty as usize) < self.params.height
                && tx >= 0
                && (tx as usize) < self.params.width
            {
                actions.push(action);
            }
        }
        actions
    }

    pub fn to_string(&self) -> String {
        let mut ss = String::from("");
        ss += &format!("turn:\t{}\n", self.turn);
        ss += &format!(
            "score:\t{}:{} {}:{}",
            self.characters[0].mark,
            self.characters[0].game_score,
            self.characters[1].mark,
            self.characters[1].game_score
        );
        for y in 0..self.params.height {
            ss += "\n";
            for x in 0..self.params.width {
                let mut is_written = false;
                // both A and B
                if self.characters[0].y == y
                    && self.characters[0].x == x
                    && self.characters[1].y == y
                    && self.characters[1].x == x
                {
                    ss += "@";
                    continue;
                }
                for character in &self.characters {
                    if character.y == y && character.x == x {
                        ss += &character.mark;
                        is_written = true;
                    }
                }
                if !is_written {
                    let point = self.points[y][x];
                    if point > 0 {
                        ss += &format!("{point}");
                    } else {
                        ss += ".";
                    }
                }
            }
        }
        ss += "\n";
        ss
    }

    pub fn white_score(&self) -> game_result::GameResult {
        let point_diff = self.characters[0].game_score as isize
            - self.characters[1].game_score as isize;
        game_result::GameResult::new(point_diff)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    fn setup() -> SimultaneousMazeState {
        let params = MazeParams {
            height: 3,
            width: 3,
            end_turn: 4,
        };
        SimultaneousMazeState::new(0, params)
    }

    #[test]
    fn test_white_score() {
        let mut state = setup();
        let actual = state.white_score();
        assert_eq!(actual.score, 0.5);

        state.advance(vec![0, 3]);
        let actual = state.white_score();
        assert_eq!(actual.score, 0.0);
    }

    #[test]
    fn test_to_string() {
        let mut state = setup();
        let actual = state.to_string();
        let expected = "\
turn:\t0
score:\tA:0 B:0
7.7
A2B
272
";
        assert_eq!(actual, expected);

        state.advance(vec![0, 3]);
        let actual = state.to_string();
        let expected = "\
turn:\t1
score:\tA:2 B:7
7.B
.A.
272
";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_legal_actions() {
        let state = setup();
        let legal_actions0 = state.legal_actions(0);
        assert_eq!(legal_actions0, vec![0, 2, 3]);
    }

    #[test]
    fn test_advance() {
        let mut state = setup();
        assert_eq!(state.turn, 0);
        state.advance(vec![0, 1]);
        assert_eq!(state.turn, 1);
        assert_eq!(state.characters[0].game_score, 2);
        assert_eq!(state.characters[0].game_score, 2);
    }

    #[test]
    fn test_is_done() {
        let mut state = setup();
        assert_eq!(state.is_done(), false);
        state.turn = state.params.end_turn;
        assert_eq!(state.is_done(), true);
    }

    #[test]
    fn test_make_state() {
        let state = setup();
        assert_eq!(state.characters.len(), 2);
        assert!(state.points[0][0] <= 9);
        assert_eq!(state.characters[0].mark, "A");
    }

    #[test]
    fn make_params() {
        let params = MazeParams {
            height: 3,
            width: 3,
            end_turn: 4,
        };
        assert_eq!(params.height, 3);
    }
}
