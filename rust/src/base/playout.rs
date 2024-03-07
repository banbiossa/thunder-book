use crate::base::state;

pub struct Playout<S, F>
where
    S: state::State,
    F: Fn(&S) -> S::Action,
{
    state: S,
    random_action: F,
}

impl<S, F> Playout<S, F>
where
    S: state::State,
    F: Fn(&S) -> S::Action,
{
    pub fn new(state: &S, random_action: F) -> Self {
        Playout {
            state: state.clone(),
            random_action,
        }
    }

    pub fn playout(&mut self) -> f32 {
        if self.state.is_done() {
            return self.state.white_score();
        }
        let action = (self.random_action)(&self.state);
        self.state.advance(action);
        1.0 - self.playout()
    }
}
