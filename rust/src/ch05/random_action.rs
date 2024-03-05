use std::sync::Arc;

use rand::{seq::SliceRandom, thread_rng};

use crate::ch05::maze_state;

pub fn random_action_arc() -> Arc<maze_state::ActionFunc> {
    Arc::new(move |state| random_action(state))
}

fn random_action(state: &maze_state::AlternateMazeState) -> usize {
    let mut rng = thread_rng();
    let legal_actions = state.legal_actions();
    legal_actions.choose(&mut rng).unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> maze_state::AlternateMazeState {
        let params = maze_state::MazeParams {
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
