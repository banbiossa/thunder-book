use crate::ch04::maze_state;

pub fn hill_climb_factory(
    num_iter: usize,
    seed: u64,
) -> Box<maze_state::ActionFunc> {
    Box::new(move |state| -> maze_state::AutoMoveMazeState {
        hill_climb(state, num_iter, seed)
    })
}

fn hill_climb(
    initial_state: &maze_state::AutoMoveMazeState,
    num_iter: usize,
    seed: u64,
) -> maze_state::AutoMoveMazeState {
    let mut best_state = initial_state.clone();
    best_state.shuffle_characters(seed);
    let mut best_score = best_state.get_score(false);
    for _ in 0..num_iter {
        let mut state = best_state.clone();
        state.transition();
        let score = state.get_score(false);
        if score > best_score {
            best_score = score;
            best_state = state;
        }
    }
    best_state
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> maze_state::AutoMoveMazeState {
        let params = maze_state::MazeParams {
            height: 3,
            width: 3,
            end_turn: 1,
            num_characters: 2,
        };
        maze_state::AutoMoveMazeState::new(0, params)
    }

    #[test]
    fn test_hill_climb() {
        let state = setup();
        let best_state = hill_climb(&state, 100, 0);

        let state_score = state.get_score(false);
        let best_score = best_state.get_score(false);

        assert!(best_score > state_score);
    }
}
