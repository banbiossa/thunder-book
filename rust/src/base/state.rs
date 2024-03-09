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

pub trait SinglePlayerState: Clone + Ord {
    fn new(seed: u64, params: MazeParams) -> Self;
    fn legal_actions(&self) -> Vec<usize>;
    fn advance(&mut self, action: usize);
    fn set_first_action(&mut self, action: usize);
    fn get_first_action(&self) -> usize;
    fn evaluate_score(&mut self);
    fn is_done(&self) -> bool;
}
