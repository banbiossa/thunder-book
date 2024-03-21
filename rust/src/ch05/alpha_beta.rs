use crate::base::alternate::{ActionFunc, AlternateState, Evaluatable};
use std::sync::Arc;

fn alpha_beta_score<T: AlternateState + Evaluatable>(
    initial_state: &T,
    mut alpha: isize,
    beta: isize,
    depth: usize,
) -> isize {
    if initial_state.is_done() || depth == 0 {
        return initial_state.evaluation();
    }

    let legal_actions = initial_state.legal_actions();
    if legal_actions.is_empty() {
        return initial_state.evaluation();
    }

    for action in legal_actions {
        let mut state = initial_state.clone();
        state.advance(action);
        let score = -alpha_beta_score(&state, -beta, -alpha, depth - 1);
        if score > alpha {
            alpha = score;
        }
        if alpha >= beta {
            return alpha;
        }
    }

    alpha
}

// utility to track score and action
#[derive(Debug, Clone)]
struct ScoreAction {
    score: isize,
    action: usize,
}

fn alpha_beta_action<T: AlternateState + Evaluatable>(
    initial_state: &T,
    depth: usize,
) -> usize {
    let mut score_actions = Vec::new();
    // to prevent overflow, make 1 smaller than limit
    let alpha = isize::MIN + 1;
    let beta = isize::MAX - 1;
    let legal_actions = initial_state.legal_actions();
    for action in legal_actions {
        let mut state = initial_state.clone();
        state.advance(action);
        let score = -alpha_beta_score(&state, -beta, -alpha, depth);
        score_actions.push(ScoreAction { score, action });
    }

    let best = score_actions.iter().max_by_key(|p| p.score).unwrap();
    best.action
}

pub fn alpha_beta_arc<T: AlternateState + Evaluatable>(
    depth: usize,
) -> ActionFunc<T> {
    Arc::new(move |state| -> usize { alpha_beta_action(state, depth) })
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
    fn test_alpha_beta_arc() {
        let state = setup();
        let actual = alpha_beta_arc(4)(&state);
        let expected = 0;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_alpha_beta_action() {
        let state = setup();
        let actual = alpha_beta_action(&state, 4);
        let expected = 0;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_alpha_beta_score() {
        let state = setup();
        let alpha = isize::MIN + 1;
        let beta = isize::MAX - 1;
        let actual = alpha_beta_score(&state, alpha, beta, 3);
        let expected = 3;
        assert_eq!(actual, expected);
    }
}
