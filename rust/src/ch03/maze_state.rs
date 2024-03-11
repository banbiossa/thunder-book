use crate::base::state::{Character, MazeParams, SinglePlayerState};

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::cmp::Ordering;

/// type for actions to implement
// pub type ActionFunc<T: SinglePlayerState> = Box<dyn Fn(&T) -> usize>;

impl PartialOrd for NumberCollectingGame {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NumberCollectingGame {
    fn cmp(&self, other: &Self) -> Ordering {
        self.evaluated_score.cmp(&other.evaluated_score)
    }
}

/// base struct holds state of game
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberCollectingGame {
    // wraps data, and provides access to it
    character: Character,
    game_score: usize,
    // dims points[H][W]
    points: Vec<Vec<usize>>,
    turn: usize,
    evaluated_score: isize,
    first_action: Option<usize>, // will be set during explore
    params: MazeParams,
}

impl SinglePlayerState for NumberCollectingGame {
    fn new(seed: u64, params: MazeParams) -> NumberCollectingGame {
        let mut rng = StdRng::seed_from_u64(seed);

        // make character
        let character = Character {
            y: rng.gen_range(0..params.height),
            x: rng.gen_range(0..params.width),
            mark: String::from("A"),
        };

        // make points, if character is there skip
        let mut points: Vec<Vec<usize>> =
            vec![vec![0; params.width]; params.height];
        for y in 0..params.height {
            for x in 0..params.width {
                if character.y == y && character.x == x {
                    continue;
                }
                points[y][x] = rng.gen_range(0..=9);
            }
        }

        NumberCollectingGame {
            character,
            game_score: 0,
            points,
            turn: 0,
            evaluated_score: 0,
            first_action: None,
            params,
        }
    }
    /// checks if the game is done
    fn is_done(&self) -> bool {
        self.turn >= self.params.end_turn
    }

    /// evaluate score
    fn evaluate_score(&mut self) {
        self.evaluated_score = self.game_score as isize;
    }

    // /// moves game one action forward
    fn advance(&mut self, action: usize) -> usize {
        let character = &mut self.character;
        character.y =
            (character.y as isize + Self::DY[action] as isize) as usize;
        character.x =
            (character.x as isize + Self::DX[action] as isize) as usize;
        let point = self.points[character.y][character.x];
        self.game_score += point;
        self.points[character.y][character.x] = 0;
        self.turn += 1;
        point
    }

    /// actions that can be taken at that step
    fn legal_actions(&self) -> Vec<usize> {
        let mut actions = Vec::new();
        for action in 0..4 as usize {
            let ty = self.character.y as isize + Self::DY[action] as isize;
            let tx = self.character.x as isize + Self::DX[action] as isize;
            if ty >= 0
                && ty < self.params.height as isize
                && tx >= 0
                && tx < self.params.width as isize
            {
                actions.push(action);
            }
        }
        actions
    }

    fn set_first_action(&mut self, action: usize) {
        if self.first_action.is_none() {
            self.first_action = Some(action);
        }
    }

    fn get_first_action(&self) -> usize {
        self.first_action.unwrap()
    }
    /// utility to show state of game
    fn to_string(&self) -> String {
        let mut ss = String::from("");

        ss += &format!("turn:\t{}\n", self.turn);
        ss += &format!("score:\t{}\n", self.game_score);
        for y in 0..self.params.height {
            ss += "\n";
            for x in 0..self.params.width {
                if self.character.y == y && self.character.x == x {
                    ss += "@";
                } else if self.points[y][x] > 0 {
                    ss += &format!("{}", self.points[y][x]);
                } else {
                    ss += ".";
                }
            }
        }
        ss += "\n";

        ss
    }

    fn get_game_score(&self) -> usize {
        self.game_score
    }

    fn set_game_score(&mut self, score: usize) {
        self.game_score = score;
    }

    fn get_character(&self) -> &Character {
        &self.character
    }

    fn get_evaluated_score(&self) -> isize {
        self.evaluated_score
    }

    fn set_evaluated_score(&mut self, score: isize) {
        self.evaluated_score = score;
    }

    fn get_params(&self) -> &MazeParams {
        &self.params
    }

    fn get_points(&self) -> &Vec<Vec<usize>> {
        &self.points
    }

    fn remove_points(&mut self, y: usize, x: usize) {
        self.points[y][x] = 0
    }

    fn get_turn(&self) -> usize {
        self.turn
    }

    fn set_turn(&mut self, turn: usize) {
        self.turn = turn;
    }
}

impl NumberCollectingGame {
    const DX: [i8; 4] = [1, -1, 0, 0];
    const DY: [i8; 4] = [0, 0, 1, -1];
}

#[cfg(test)]
mod test {
    use super::*;

    // create a state as a fixture
    fn setup() -> NumberCollectingGame {
        let params = MazeParams {
            height: 3,
            width: 4,
            end_turn: 3,
        };
        NumberCollectingGame::new(0, params)
    }

    #[test]
    fn test_ordering() {
        let params = MazeParams {
            height: 3,
            width: 4,
            end_turn: 1,
        };
        let state_small = NumberCollectingGame::new(0, params.clone());
        let mut state_big = NumberCollectingGame::new(1, params);

        state_big.evaluated_score = 3;
        assert!(state_small < state_big);
    }

    #[test]
    fn to_string() {
        pub const PARAMS: MazeParams = MazeParams {
            height: 3,
            width: 4,
            end_turn: 1,
        };
        let state = NumberCollectingGame::new(0, PARAMS);
        let actual = state.to_string();
        let expected = "\
turn:\t0
score:\t0

.227
11.4
492@
";
        assert_eq!(actual, expected);
    }

    #[test]
    fn legal_actions() {
        let state = setup();
        let legal_actions = state.legal_actions();
        assert!(legal_actions.len() > 0);
    }

    #[test]
    fn advance_moves_character() {
        let mut state = setup();
        let legal_actions = state.legal_actions();
        let action = legal_actions[0];
        let character_before = state.character.clone();
        state.advance(action);
        assert_eq!(state.turn, 1);
        assert_ne!(state.character, character_before);
        let actual = state.to_string();
        let expected = "\
turn:\t1
score:\t2

.227
11.4
49@.
";
        assert_eq!(actual, expected);
    }

    #[test]
    fn create_state() {
        let state = setup();
        assert!(state.points[0][0] < 10);
        assert!(state.points.len() == state.params.height);
        assert!(state.points[0].len() == state.params.width);
    }

    #[test]
    fn is_done_works() {
        let mut state = setup();
        assert!(!state.is_done());
        state.turn = state.params.end_turn;
        assert!(state.is_done());
    }

    #[test]
    fn create_character() {
        let character = Character {
            y: 0,
            x: 0,
            mark: "A".to_string(),
        };
        assert!(character.x == 0);
    }
}
