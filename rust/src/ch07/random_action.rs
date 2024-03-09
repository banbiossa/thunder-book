use crate::base::state::SinglePlayerState;
use std::sync::Arc;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::ch07::maze_state;

pub fn random_action(state: &maze_state::WallMazeState) -> usize {
    let mut rng = thread_rng();
    let legal_actions = state.legal_actions();
    legal_actions.choose(&mut rng).unwrap().to_owned()
}

pub fn random_action_arc() -> maze_state::ActionFunc {
    Arc::new(move |state| -> usize { random_action(state) })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> maze_state::WallMazeState {
        let params = maze_state::MazeParams {
            height: 5,
            width: 5,
            end_turn: 3,
        };
        maze_state::WallMazeState::new(0, params)
    }

    #[test]
    fn test_random_action_arc() {
        let state = setup();
        let legal_actions = state.legal_actions();
        let actual = random_action_arc()(&state);
        assert!(legal_actions.contains(&actual));
    }

    #[test]
    fn test_random_action() {
        let state = setup();
        let legal_actions = state.legal_actions();
        let actual = random_action(&state);
        assert!(legal_actions.contains(&actual));
    }
}
