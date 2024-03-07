pub trait State: Clone {
    type Action;
    fn is_done(&self) -> bool;
    fn white_score(&self) -> f32;
    fn advance(&mut self, action: Self::Action);
}
