use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::ch04::maze_state;

pub fn simulated_annealing_factory(
    num_iter: usize,
    start_temp: f32,
    end_temp: f32,
    seed: u64,
) -> Box<maze_state::ActionFunc> {
    Box::new(move |state| -> maze_state::AutoMoveMazeState {
        simulated_annealing(state, num_iter, start_temp, end_temp, seed)
    })
}

fn simulated_annealing(
    initial_state: &maze_state::AutoMoveMazeState,
    num_iter: usize,
    start_temp: f32,
    end_temp: f32,
    seed: u64,
) -> maze_state::AutoMoveMazeState {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut best_state = initial_state.clone();
    best_state.shuffle_characters(seed);
    let mut best_score = best_state.get_score(false);
    let mut score = best_score;
    // keep track of state to allow 遷移
    let mut state = best_state.clone();

    for i in 0..num_iter {
        let mut next_state = state.clone();
        next_state.transition();
        let next_score = next_state.get_score(false);
        let temp = start_temp + (end_temp - start_temp) * (i / num_iter) as f32;
        let prob = ((next_score as f32 - score as f32) / temp).exp();
        let is_force_next = prob > rng.gen();
        if next_score > score || is_force_next {
            score = next_score;
            state = next_state.clone();
        }
        if next_score > best_score {
            best_score = next_score;
            best_state = next_state;
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
    fn test_factory() {
        let state = setup();
        let action_func = simulated_annealing_factory(10, 0.0, 100.0, 0);
        let best = action_func(&state);
        assert!(best.get_score(false) > state.get_score(false));
    }

    #[test]
    fn test_simulated_annealing() {
        let state = setup();
        let best_state = simulated_annealing(&state, 10, 0.0, 100.0, 0);

        let state_score = state.get_score(false);
        let best_score = best_state.get_score(false);

        assert!(best_score > state_score);
    }
}
