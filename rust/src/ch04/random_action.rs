use crate::ch04::maze_state;

use rand::{thread_rng, Rng};

pub fn random_action(
    initial_state: &maze_state::AutoMoveMazeState,
) -> maze_state::AutoMoveMazeState {
    let mut rng = thread_rng();
    let mut state = initial_state.clone();
    for id in 0..state.params.num_characters {
        let y = rng.gen_range(0..state.params.height);
        let x = rng.gen_range(0..state.params.width);

        state.set_character(id, y, x);
    }

    state
}
