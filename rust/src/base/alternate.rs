use crate::base::game_result;

#[derive(Debug, Clone)]
pub struct MazeParams {
    pub height: usize,
    pub width: usize,
    pub end_turn: usize,
}

pub trait AlternateState: Clone {
    fn new(seed: u64, params: MazeParams) -> Self;
    fn is_done(&self) -> bool;
    fn advance(&mut self, action: usize);
    fn legal_actions(&self) -> Vec<usize>;
    fn to_string(&self) -> String;
    fn white_score(&self) -> f32;
}
