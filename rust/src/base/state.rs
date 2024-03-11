use rand::{rngs::StdRng, Rng, SeedableRng};

pub trait State: Clone {
    type Action;
    fn is_done(&self) -> bool;
    fn white_score(&self) -> f32;
    fn advance(&mut self, action: Self::Action);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MazeParams {
    pub height: usize,
    pub width: usize,
    pub end_turn: usize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Character {
    pub y: usize, // y coming first is important
    pub x: usize,
    pub mark: String,
}

impl Character {
    pub fn new(params: &MazeParams, seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        Character {
            y: rng.gen_range(0..params.height),
            x: rng.gen_range(0..params.width),
            mark: "A".to_string(),
        }
    }
}

pub trait SinglePlayerState: Clone + Ord {
    fn new(seed: u64, params: MazeParams) -> Self;
    fn legal_actions(&self) -> Vec<usize>;
    fn advance(&mut self, action: usize) -> usize;
    fn evaluate_score(&mut self);
    fn is_done(&self) -> bool;
    fn to_string(&self) -> String;
    fn set_first_action(&mut self, action: usize);
    fn get_first_action(&self) -> usize;
    fn get_game_score(&self) -> usize;
    fn set_game_score(&mut self, score: usize);
    fn get_evaluated_score(&self) -> isize;
    fn set_evaluated_score(&mut self, score: isize);
    fn get_character(&self) -> &Character;
    fn get_points(&self) -> &Vec<Vec<usize>>;
    fn remove_points(&mut self, y: usize, x: usize);
    fn get_params(&self) -> &MazeParams;
    fn get_turn(&self) -> usize;
    fn set_turn(&mut self, turn: usize);
}

pub type ActionFunc<T> = Box<dyn Fn(&T) -> usize>;

pub trait HashableState {
    fn get_hash(&self) -> u64;
    fn set_hash(&mut self, hash: u64);
}
