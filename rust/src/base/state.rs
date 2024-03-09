pub trait State: Clone {
    type Action;
    fn is_done(&self) -> bool;
    fn white_score(&self) -> f32;
    fn advance(&mut self, action: Self::Action);
}

pub trait SinglePlayerState: Clone + Ord {
    fn legal_actions(&self) -> Vec<usize>;
    fn advance(&mut self, action: usize);
    fn set_first_action(&mut self, action: usize);
    fn get_first_action(&self) -> usize;
    fn evaluate_score(&mut self);
    fn is_done(&self) -> bool;
}
