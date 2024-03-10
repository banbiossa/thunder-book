use crate::base::state::{ActionFunc, SinglePlayerState};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn random_action<T: SinglePlayerState>(state: &T) -> usize {
    let mut rng = thread_rng();

    let legal_actions = state.legal_actions();
    let choice = legal_actions.choose(&mut rng).unwrap();

    choice.clone()
}

pub fn random_action_box<T: SinglePlayerState>() -> ActionFunc<T> {
    Box::new(move |state| -> usize { random_action(state) })
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::base::state;
    use crate::ch03::maze_state::{self, NumberCollectingGame};

    fn setup() -> state::MazeParams {
        state::MazeParams {
            height: 3,
            width: 4,
            end_turn: 3,
        }
    }

    #[test]
    fn test_random_action_box() {
        let params = setup();
        let state = maze_state::NumberCollectingGame::new(0, params);
        let legal_actions = state.legal_actions();
        let action_func: state::ActionFunc<NumberCollectingGame> =
            random_action_box();
        let action = action_func(&state);
        assert!(legal_actions.contains(&action));
    }

    #[test]
    fn test_random_action() {
        let params = setup();
        let state = maze_state::NumberCollectingGame::new(0, params);
        let action = random_action(&state);
        let legal_actions = state.legal_actions();
        assert!(legal_actions.contains(&action));
    }
}
