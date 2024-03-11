use crate::base::state::{Character, MazeParams, SinglePlayerState};
use crate::ch07::maze_state::WallMazeState;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
struct DistanceCoord {
    y: usize,
    x: usize,
    distance: usize,
}

impl DistanceCoord {
    fn from_character(character: &Character) -> Self {
        DistanceCoord {
            y: character.y,
            x: character.x,
            distance: 0,
        }
    }
}

/// "leeches" on to WallMazeState,
/// and tracks distance to nearest point
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeatPointState {
    pub state: WallMazeState,
}

impl PartialOrd for NeatPointState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NeatPointState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.state.evaluated_score.cmp(&other.state.evaluated_score)
    }
}

impl SinglePlayerState for NeatPointState {
    fn new(seed: u64, params: crate::base::state::MazeParams) -> Self {
        let state = WallMazeState::new(seed, params);
        NeatPointState { state }
    }

    fn evaluate_score(&mut self) {
        self.state.evaluated_score = (self.state.game_score
            * self.state.params.height
            * self.state.params.width)
            as isize
            - self.get_distance_to_nearest_point() as isize;
    }

    // same as underlying state
    fn legal_actions(&self) -> Vec<usize> {
        self.state.legal_actions()
    }

    fn advance(&mut self, action: usize) {
        self.state.advance(action)
    }

    fn set_first_action(&mut self, action: usize) {
        self.state.set_first_action(action)
    }

    fn get_first_action(&self) -> usize {
        self.state.get_first_action()
    }

    fn is_done(&self) -> bool {
        self.state.is_done()
    }

    fn to_string(&self) -> String {
        self.state.to_string()
    }

    fn get_game_score(&self) -> usize {
        self.state.get_game_score()
    }

    fn get_character(&self) -> &Character {
        self.state.get_character()
    }

    fn get_evaluated_score(&self) -> isize {
        self.state.get_evaluated_score()
    }

    fn get_params(&self) -> &MazeParams {
        self.state.get_params()
    }

    fn get_points(&self) -> &Vec<Vec<usize>> {
        self.state.get_points()
    }

    fn get_turn(&self) -> usize {
        self.state.get_turn()
    }
}

impl NeatPointState {
    fn get_distance_to_nearest_point(&self) -> usize {
        let mut check = vec![
            vec![false; self.state.params.width];
            self.state.params.height
        ];
        let mut que = VecDeque::new();
        que.push_back(DistanceCoord::from_character(&self.state.character));

        while !que.is_empty() {
            let pawn = que.pop_front().unwrap();
            if self.state.points[pawn.y][pawn.x] > 0 {
                return pawn.distance;
            }

            check[pawn.y][pawn.x] = true;
            for action in 0..4 {
                let ty = pawn.y as isize + WallMazeState::DY[action];
                let tx = pawn.x as isize + WallMazeState::DX[action];
                if ty >= 0
                    && (ty as usize) < self.state.params.height
                    && tx >= 0
                    && (tx as usize) < self.state.params.width
                    && self.state.walls[ty as usize][tx as usize] == 0
                    && !check[ty as usize][tx as usize]
                {
                    que.push_back(DistanceCoord {
                        y: ty as usize,
                        x: tx as usize,
                        distance: pawn.distance + 1,
                    });
                }
            }
        }

        // return max if no early return
        self.state.params.height * self.state.params.width
    }
}

#[cfg(test)]
mod tests {
    use crate::base::state;

    use super::*;

    fn setup() -> NeatPointState {
        let params = state::MazeParams {
            height: 5,
            width: 5,
            end_turn: 3,
        };
        NeatPointState::new(0, params)
    }

    #[test]
    fn test_evaluated_score() {
        let mut state = setup();
        state.advance(1);
        state.evaluate_score();
        let actual = state.state.evaluated_score;
        let expected = 7 * 5 * 5 - 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_nearest_point() {
        let state = setup();
        let actual = state.get_distance_to_nearest_point();
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn make_state() {
        let state = setup();
        let actual = state.to_string();
        let expected = "\
turn:\t0
score:\t0

7@.22
7#1##
49251
8#5##
9#665
";
        assert_eq!(actual, expected);
    }
}
