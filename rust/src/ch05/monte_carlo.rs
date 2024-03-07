use std::sync::Arc;

use crate::base::playout;
use crate::base::state::State;
use crate::ch05::maze_state;
use crate::ch05::random_action;

// implements playout, because playout needs to own the mutable state
// make it a struct not a function
pub struct Playout {
    state: maze_state::AlternateMazeState,
}

impl Playout {
    pub fn new(state: &maze_state::AlternateMazeState) -> Self {
        Playout {
            state: state.clone(),
        }
    }

    pub fn playout(&mut self) -> f32 {
        if self.state.is_done() {
            return self.state.white_score().score;
        }

        let action = random_action::random_action_arc()(&self.state);
        self.state.advance(action);
        1.0 - self.playout()
    }
}

#[derive(Debug)]
struct ActionScore {
    action: usize,
    score: f32,
}

fn monte_carlo_action(
    initial_state: &maze_state::AlternateMazeState,
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

    // get best action
    let mut action_scores = Vec::new();
    for index in 0..legal_actions.len() {
        let value_mean = values[index] / counts[index] as f32;
        action_scores.push(ActionScore {
            action: legal_actions[index],
            score: value_mean,
        })
    }

    // floatにOrdが無いので変な書き方になるけどただのmax_by_key
    let best = action_scores
        .iter()
        .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
        .unwrap();
    best.action
}

pub fn monte_carlo_action_arc(
    num_playout: usize,
) -> Arc<maze_state::ActionFunc> {
    Arc::new(move |state| -> usize { monte_carlo_action(state, num_playout) })
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
