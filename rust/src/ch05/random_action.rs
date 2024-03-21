use crate::base::alternate::{ActionFunc, AlternateState};
use std::sync::Arc;

use rand::{seq::SliceRandom, thread_rng};

pub fn random_action_arc<T: AlternateState>() -> ActionFunc<T> {
    Arc::new(move |state| random_action(state))
}

fn random_action<T: AlternateState>(state: &T) -> usize {
    let mut rng = thread_rng();
    let legal_actions = state.legal_actions();
    legal_actions.choose(&mut rng).unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::alternate::MazeParams;
    use crate::ch05::maze_state;

    fn setup() -> maze_state::AlternateMazeState {
        let params = MazeParams {
            height: 3,
            width: 3,
            end_turn: 4,
        };
        maze_state::AlternateMazeState::new(0, params)
    }

    #[test]
    fn test_random_action() {
        let state = setup();
        let legal_actions = state.legal_actions();
        let action = random_action(&state);
        assert!(legal_actions.contains(&action));
    }
}
