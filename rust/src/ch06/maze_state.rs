use rand::{rngs::StdRng, SeedableRng};

struct MazeParams {
    height: usize,
    width: usize,
    end_turn: usize,
}

struct Character {
    y: usize,
    x: usize,
    mark: String,
}

struct SimultaneousMazeState {
    points: Vec<Vec<usize>>,
    turn: usize,
    characters: Vec<Character>,
    params: MazeParams,
}

impl SimultaneousMazeState {
    pub fn new(seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
    }
    fn is_done(&self) -> bool {}
    fn advance(&self, action0: usize, action1: usize) {}
    fn legal_actions(&self, player_id: usize) -> Vec<usize> {}
    fn to_string(&self) -> String {}
}

#[cfg(test)]
mod tests {
    use super::*;

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
