use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::base::state::{
    Character, HashableState, MazeParams, SinglePlayerState,
};
use crate::ch07::maze_state::WallMazeState;
use crate::ch07::near_state::NeatPointState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZobristHash {
    points: Vec<Vec<Vec<u64>>>,
    character: Vec<Vec<u64>>,
    params: MazeParams,
}

impl ZobristHash {
    pub fn new(params: MazeParams) -> Self {
        let mut rng = StdRng::seed_from_u64(0);
        let mut points: Vec<Vec<Vec<u64>>> =
            vec![vec![vec![0; 10]; params.width]; params.height];
        let mut character: Vec<Vec<u64>> =
            vec![vec![0; params.width]; params.height];

        for y in 0..params.height {
            for x in 0..params.width {
                for p in 1..=9 {
                    points[y][x][p] = rng.gen();
                }
                character[y][x] = rng.gen();
            }
        }

        ZobristHash {
            points,
            character,
            params,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZobristState {
    pub state: NeatPointState,
    pub zobrist: ZobristHash,
    pub hash: u64,
}

impl HashableState for ZobristState {
    fn get_hash(&self) -> u64 {
        self.hash
    }
}

impl PartialOrd for ZobristState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ZobristState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.state
            .state
            .evaluated_score
            .cmp(&other.state.state.evaluated_score)
    }
}

impl SinglePlayerState for ZobristState {
    fn new(seed: u64, params: MazeParams) -> Self {
        let state = NeatPointState::new(seed, params.clone());
        let zobrist = ZobristHash::new(params.clone());

        // state.state はキモいので
        // get_character, get_point を実装したほうがベター
        let character = &state.state.character;
        let points = &state.state.points;

        // calculate hash
        let mut hash = 0;
        hash ^= zobrist.character[character.y][character.x];
        for y in 0..params.height {
            for x in 0..params.width {
                let point = points[y][x];
                if point > 0 {
                    hash ^= zobrist.points[y][x][point];
                }
            }
        }

        ZobristState {
            state,
            hash,
            zobrist,
        }
    }

    fn advance(&mut self, action: usize) {
        let character = &mut self.state.state.character;
        let points = &mut self.state.state.points;
        // let game_score = &mut self.state.state.game_score;

        self.hash ^= self.zobrist.character[character.y][character.x];

        character.y =
            (character.y as isize + WallMazeState::DY[action]) as usize;
        character.x =
            (character.x as isize + WallMazeState::DX[action]) as usize;
        let point = points[character.y][character.x];

        self.hash ^= self.zobrist.character[character.y][character.x];
        if point > 0 {
            self.hash ^= self.zobrist.points[character.y][character.x][point];
            // *game_score += point;
            self.state.state.game_score += point;
            points[character.y][character.x] = 0;
        }
        self.state.state.turn += 1;
    }

    // leeches on to NeatPointState which leeches on to WallMazeState
    fn evaluate_score(&mut self) {
        self.state.evaluate_score()
    }

    fn get_first_action(&self) -> usize {
        self.state.get_first_action()
    }

    fn get_game_score(&self) -> usize {
        self.state.get_game_score()
    }

    fn is_done(&self) -> bool {
        self.state.is_done()
    }

    fn legal_actions(&self) -> Vec<usize> {
        self.state.legal_actions()
    }

    fn set_first_action(&mut self, action: usize) {
        self.state.set_first_action(action)
    }

    fn to_string(&self) -> String {
        self.state.to_string()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advance() {
        let params = MazeParams {
            height: 3,
            width: 3,
            end_turn: 3,
        };
        let mut state = ZobristState::new(0, params);
        let hash_before = state.hash;
        let action = 1;
        assert!(state.legal_actions().contains(&action));
        state.advance(action);
        assert_ne!(hash_before, state.hash);
    }

    #[test]
    fn test_xor() {
        let mut hash = 0;
        let value: u64 = 17873746660756568148;
        hash ^= value;
        assert_eq!(hash, value);
        hash ^= value;
        assert_eq!(hash, 0);
    }

    #[test]
    fn test_make_zobrist() {
        let params = MazeParams {
            height: 3,
            width: 3,
            end_turn: 3,
        };
        let zobrist = ZobristHash::new(params);
        let actual = zobrist.character;
        let expected = [
            [
                17873746660756568148,
                10317859705648061693,
                18204917730788549397,
            ],
            [
                12283475703381346152,
                18016506439157543517,
                1541945846486539002,
            ],
            [
                333050225838729137,
                15011644926525101193,
                6662742835366122624,
            ],
        ];
        assert_eq!(actual, expected);
    }
}
