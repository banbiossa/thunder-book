use crate::ch03::maze_state;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn random_action(state: maze_state::NumberCollectingGame) -> usize {
    let mut rng = thread_rng();

    let legal_actions = state.legal_actions();
    let choice = legal_actions.choose(&mut rng).unwrap();

    choice.clone()
}
