use crate::base::state::SinglePlayerState;
use crate::ch07::maze_state::{Character, WallMazeState};
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct NeatPointState {
    state: WallMazeState,
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
        self.state.evaluated_score = self.state.game_score
            * self.state.params.height
            * self.state.params.width
            - self.get_distance_to_nearest_point();
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
