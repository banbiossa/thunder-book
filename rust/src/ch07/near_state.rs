use crate::base::state::{
    single_player_state_portrait, Character, MazeParams, SinglePlayerState,
    Wall,
};
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
pub struct NearPointState {
    pub state: WallMazeState,
}

impl PartialOrd for NearPointState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NearPointState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_evaluated_score().cmp(&other.get_evaluated_score())
    }
}

#[portrait::fill(portrait::delegate(WallMazeState; self.state))]
impl SinglePlayerState for NearPointState {
    fn new(seed: u64, params: crate::base::state::MazeParams) -> Self {
        let state = WallMazeState::new(seed, params);
        NearPointState { state }
    }

    fn evaluate_score(&mut self) {
        let evaluated_score = (self.get_game_score()
            * self.get_params().height
            * self.get_params().width) as isize
            - self.get_distance_to_nearest_point() as isize;
        self.set_evaluated_score(evaluated_score);
    }
}

impl Wall for NearPointState {
    fn get_walls(&self) -> &Vec<Vec<usize>> {
        &self.state.get_walls()
    }
}

impl NearPointState {
    fn get_distance_to_nearest_point(&self) -> usize {
        let mut check = vec![
            vec![false; self.get_params().width];
            self.get_params().height
        ];
        let mut que = VecDeque::new();
        que.push_back(DistanceCoord::from_character(&self.get_character()));

        while !que.is_empty() {
            let pawn = que.pop_front().unwrap();
            if self.get_points()[pawn.y][pawn.x] > 0 {
                return pawn.distance;
            }

            check[pawn.y][pawn.x] = true;
            for action in 0..4 {
                let ty = pawn.y as isize + WallMazeState::DY[action];
                let tx = pawn.x as isize + WallMazeState::DX[action];
                if ty >= 0
                    && (ty as usize) < self.get_params().height
                    && tx >= 0
                    && (tx as usize) < self.get_params().width
                    && self.get_walls()[ty as usize][tx as usize] == 0
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
        self.get_params().height * self.get_params().width
    }
}

#[cfg(test)]
mod tests {
    use crate::base::state;

    use super::*;

    fn setup() -> NearPointState {
        let params = state::MazeParams {
            height: 5,
            width: 5,
            end_turn: 3,
        };
        NearPointState::new(0, params)
    }

    #[test]
    fn test_evaluated_score() {
        let mut state = setup();
        state.advance(1);
        state.evaluate_score();
        let actual = state.get_evaluated_score();
        let expected = 2 * 5 * 5 - 2;
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

2@711
.#4##
51825
6#9##
6#735
";
        assert_eq!(actual, expected);
    }
}
