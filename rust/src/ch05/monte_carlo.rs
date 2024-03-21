use std::sync::Arc;

use crate::base::alternate::{ActionFunc, AlternateState};
use crate::ch05::random_action;

// implements playout, because playout needs to own the mutable state
// make it a struct not a function
pub struct Playout<T: AlternateState> {
    state: T,
}

impl<T: AlternateState> Playout<T> {
    pub fn new(state: &T) -> Self {
        Playout {
            state: state.clone(),
        }
    }

    pub fn playout(&mut self) -> f32 {
        if self.state.is_done() {
            return self.state.white_score();
        }

        let action = random_action::random_action_arc()(&self.state);
        self.state.advance(action);
        1.0 - self.playout()
    }
}

fn monte_carlo_action<T: AlternateState>(
    initial_state: &T,
    num_playout: usize,
) -> usize {
    let legal_actions = initial_state.legal_actions();
    let mut values = vec![0.0; legal_actions.len()];
    let mut counts: Vec<usize> = vec![0; legal_actions.len()];
    for count in 0..num_playout {
        let index = count % legal_actions.len();
        let mut state = initial_state.clone();
        state.advance(legal_actions[index]);
        // 2回　clone することになるので無駄っぽいけど、
        // mutate するバグを以前踏んだのでこのまま
        values[index] += 1.0 - Playout::new(&state).playout();
        counts[index] += 1;
    }

    // Calculate the action-score pairs
    let action_scores: Vec<(usize, f32)> = legal_actions
        .iter()
        .zip(values.iter().zip(counts.iter()))
        .map(|(&action, (&value, &count))| (action, value / count as f32))
        .collect();

    // Find the best action based on the scores
    action_scores
        .iter()
        .max_by(|(_, score1), (_, score2)| score1.partial_cmp(&score2).unwrap())
        .unwrap()
        .0
        .to_owned()
}

pub fn monte_carlo_action_arc<T: AlternateState>(
    num_playout: usize,
) -> ActionFunc<T> {
    Arc::new(move |state| -> usize { monte_carlo_action(state, num_playout) })
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
    fn test_monte_carlo() {
        let state = setup();
        let actual = monte_carlo_action(&state, 100);
        let expected = 0;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_monte_carlo_arc() {
        let state = setup();
        let actual = monte_carlo_action_arc(100)(&state);
        let expected = 0;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_playout() {
        let state = setup();
        let mut playout = Playout::new(&state);
        let score = playout.playout();
        // check that state isn't mutated
        assert_eq!(state.turn, 0);
        // result is random
        assert!(score <= 1.0);
    }
}
