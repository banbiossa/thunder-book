use std::sync::Arc;

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
    fn teban_score(&self) -> f32;
}

pub trait Evaluatable {
    // these 2 will be uninplemented in ch05
    // so should be another trait
    fn evaluation(&self) -> isize;
    fn evaluation_rate(&self) -> f32;
}

pub type ActionFunc<T> = Arc<dyn Fn(&T) -> usize>;
