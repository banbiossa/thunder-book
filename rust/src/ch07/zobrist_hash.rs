use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::base::state::{
    single_player_state_portrait, Character, HashableState, MazeParams,
    SinglePlayerState,
};
use crate::ch07::near_state::NearPointState;

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
    pub state: NearPointState,
    pub zobrist: ZobristHash,
    pub hash: u64,
}

impl HashableState for ZobristState {
    fn get_hash(&self) -> u64 {
        self.hash
    }
    fn set_hash(&mut self, hash: u64) {
        self.hash = hash
    }
}

impl PartialOrd for ZobristState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ZobristState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_evaluated_score().cmp(&other.get_evaluated_score())
    }
}

impl ZobristState {
    fn update_point_hash(&mut self, point: usize) {
        // used for delete point (after move)
        // 動いた後は、盤面の点数が消えるのでupdateする
        // 盤面作成時のポイントは　new で考慮している
        if point == 0 {
            return;
        }
        let character = self.get_character();
        self.hash ^= self.zobrist.points[character.y][character.x][point];
    }

    fn update_character_hash(&mut self) {
        // used for both delete and add (before and after move)
        let character = self.get_character();
        self.hash ^= self.zobrist.character[character.y][character.x];
    }
}

#[portrait::fill(portrait::delegate(NearPointState; self.state))]
impl SinglePlayerState for ZobristState {
    fn new(seed: u64, params: MazeParams) -> Self {
        let state = NearPointState::new(seed, params.clone());
        let zobrist = ZobristHash::new(params.clone());

        // calculate hash
        let mut hash = 0;

        // can't get 2 borrows so borrow character first
        let character = state.get_character();
        hash ^= zobrist.character[character.y][character.x];

        let points = state.get_points();
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

    /// mutable を常に１つしか持たない制約により、関数を分割している
    fn advance(&mut self, action: usize) -> usize {
        // remove character hash
        self.update_character_hash();
        let point = self.state.advance(action);
        self.update_point_hash(point);
        self.update_character_hash();
        point
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
