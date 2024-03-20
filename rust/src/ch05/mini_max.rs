use std::sync::Arc;

use crate::base::alternate::AlternateState;
use crate::ch05::maze_state;

// utility to track score and action
#[derive(Debug, Clone)]
struct ScoreAction {
    score: isize,
    action: usize,
}

fn mini_max_score(
    initial_state: &maze_state::AlternateMazeState,
    depth: usize,
    print: bool,
) -> isize {
    if print {
        println!(
            "depth:\t{depth}\nchar:\t{}\n{}",
            initial_state.characters[0].mark,
            initial_state.to_string()
        );
    }
    if initial_state.is_done() || depth == 0 {
        if print {
            println!(
                "done, eval is {} for {}\n",
                initial_state.evaluation(),
                initial_state.characters[0].mark
            );
        }
        return initial_state.evaluation();
    }

    let legal_actions = initial_state.legal_actions();
    if legal_actions.is_empty() {
        if print {
            println!(
                "no action to take, eval is {} for {}",
                initial_state.evaluation(),
                initial_state.characters[0].mark
            );
        }
        return initial_state.evaluation();
    }

    let mut score_actions = Vec::new();

    for action in legal_actions {
        if print {
            println!(
                "{} takes action: {action}",
                initial_state.characters[0].mark
            );
        }
        let mut state = initial_state.clone();
        state.advance(action);
        let score = -mini_max_score(&state, depth - 1, print);
        score_actions.push(ScoreAction { score, action });
    }

    if print {
        println!(
            "best was {:?} for {}",
            score_actions.iter().max_by_key(|p| p.score).unwrap(),
            initial_state.characters[0].mark
        );
    }

    let best = score_actions.iter().max_by_key(|p| p.score).unwrap();
    best.score
}

fn mini_max_action(
    initial_state: &maze_state::AlternateMazeState,
    depth: usize,
    print: bool,
) -> usize {
    let mut score_actions: Vec<ScoreAction> = Vec::new();

    let legal_actions = initial_state.legal_actions();
    for action in legal_actions {
        let mut state = initial_state.clone();
        state.advance(action);
        let score = -mini_max_score(&state, depth, print);
        score_actions.push(ScoreAction { score, action });
    }

    let best = score_actions.iter().max_by_key(|p| p.score).unwrap();
    best.action
}

pub fn mini_max_arc(depth: usize) -> Arc<maze_state::ActionFunc> {
    Arc::new(move |state| -> usize { mini_max_action(state, depth, false) })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::alternate::MazeParams;

    fn setup() -> maze_state::AlternateMazeState {
        let params = MazeParams {
            height: 3,
            width: 3,
            end_turn: 3,
        };
        maze_state::AlternateMazeState::new(0, params)
    }

    #[test]
    fn test_mini_max_action() {
        let state = setup();
        let actual = mini_max_action(&state, 2, true);
        let expected = 0;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_mini_max_score() {
        let state = setup();
        let actual = mini_max_score(&state, 3, true);
        let expected = 3;
        assert_eq!(actual, expected);
    }
}
