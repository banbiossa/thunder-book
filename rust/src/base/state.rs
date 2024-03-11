use std::cmp::Ordering;

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
    y: usize, // y coming first is important
    x: usize,
    mark: String,
}

/// base struct holds state of game
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateData {
    character: Character,
    game_score: usize,
    // dims points[H][W]
    points: Vec<Vec<usize>>,
    turn: usize,
    evaluated_score: usize,
    first_action: Option<usize>, // will be set during explore
    params: MazeParams,
}

impl PartialOrd for StateData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StateData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.evaluated_score.cmp(&other.evaluated_score)
    }
}

pub trait SinglePlayerState: Clone + Ord {
    fn new(seed: u64, params: MazeParams) -> Self;
    fn legal_actions(&self) -> Vec<usize>;
    fn advance(&mut self, action: usize);
    fn evaluate_score(&mut self);
    fn is_done(&self) -> bool;
    fn to_string(&self) -> String;
    fn set_first_action(&mut self, action: usize);
    fn get_first_action(&self) -> usize;
    fn get_game_score(&self) -> usize;
    fn get_evaluated_score(&self) -> usize;
    fn get_character(&self) -> &Character;
    fn get_points(&self) -> &Vec<Vec<usize>>;
    fn get_params(&self) -> &MazeParams;
}

pub type ActionFunc<T> = Box<dyn Fn(&T) -> usize>;

pub trait HashableState {
    fn get_hash(&self) -> u64;
}
