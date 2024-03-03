use std::sync::Arc;

use crate::ch05::maze_state;

// utility to track score and action
#[derive(Debug, Clone)]
struct ScoreAction {
    action: usize,
    score: isize,
}

fn mini_max_score(
    state: &maze_state::AlternateMazeState,
    depth: usize,
    print: bool,
) -> isize {
    if print {
        println!(
            "depth:\t{depth}\nchar:\t{}\n{}",
            state.characters[0].mark,
            state.to_string()
        );
    }
    if state.is_done() || depth == 0 {
        if print {
            println!(
                "done, eval is {} for {}\n",
                state.evaluation(),
                state.characters[0].mark
            );
        }
        return state.evaluation();
    }

    let legal_actions = state.legal_actions();
    if legal_actions.is_empty() {
        if print {
            println!(
                "no action to take, eval is {} for {}",
                state.evaluation(),
                state.characters[0].mark
            );
        }
        return state.evaluation();
    }

    let mut best: Option<ScoreAction> = None;

    for action in legal_actions {
        if print {
            println!("{} takes action: {action}", state.characters[0].mark);
        }
        let mut next_state = state.clone();
        next_state.advance(action);
        let score = -mini_max_score(&next_state, depth - 1, print);
        if best.is_none() || score > best.as_ref().unwrap().score {
            best = Some(ScoreAction { score, action });
        }
    }

    if print {
        println!(
            "best was {:?} for {}",
            best.as_ref().unwrap(),
            state.characters[0].mark
        );
    }

    best.unwrap().score
}

fn mini_max_action(
    state: &maze_state::AlternateMazeState,
    depth: usize,
) -> usize {
    let mut score_actions: Vec<ScoreAction> = Vec::new();

    for action in state.legal_actions() {
        let mut next_state = state.clone();
        next_state.advance(action);
        let score = -mini_max_score(state, depth, false);
        score_actions.push(ScoreAction { score, action });
    }

    score_actions.iter().max_by_key(|p| p.score).unwrap().action
}

pub fn mini_max_arc(depth: usize) -> Arc<maze_state::ActionFunc> {
    Arc::new(move |state| -> usize { mini_max_action(state, depth) })
}

pub fn mini_max_action_factory(depth: usize) -> Box<maze_state::ActionFunc> {
    Box::new(move |state| -> usize { mini_max_action(state, depth) })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> maze_state::AlternateMazeState {
        let params = maze_state::MazeParams {
            height: 3,
            width: 3,
            end_turn: 3,
        };
        maze_state::AlternateMazeState::new(0, params)
    }

    #[test]
    fn test_mini_max_action() {
        let state = setup();
        let actual = mini_max_action(&state, 2);
        let expected = 3;
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
