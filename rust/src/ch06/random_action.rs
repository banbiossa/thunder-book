use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::ch06::maze_state;

pub fn random_action(
    state: &maze_state::SimultaneousMazeState,
    player_id: usize,
) -> usize {
    let mut rng = thread_rng();
    let legal_actions = state.legal_actions(player_id);
    legal_actions.choose(&mut rng).unwrap().to_owned()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn setup() -> maze_state::SimultaneousMazeState {
        let params = maze_state::MazeParams {
            height: 3,
            width: 3,
            end_turn: 4,
        };
        maze_state::SimultaneousMazeState::new(0, params)
    }

    #[test]
    fn test_random() {
        let mut state = setup();
        let action0 = random_action(&state, 0);
        let action1 = random_action(&state, 1);
        state.advance(action0, action1);
    }
}
