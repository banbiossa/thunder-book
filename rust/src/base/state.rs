use crate::base::game_result;

pub trait State: Clone {
    type Action;
    fn is_done(&self) -> bool;
    fn white_score(&self) -> game_result::GameResult;
    fn advance(&mut self, action: Self::Action);
}
