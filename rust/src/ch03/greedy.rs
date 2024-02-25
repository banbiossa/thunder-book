use crate::ch03::maze_state::NumberCollectingGame;

/// keep track of the action-score tuple
#[derive(Debug)]
struct ActionScore {
    action: usize,
    score: usize,
}

pub fn greedy_action(initial_state: &NumberCollectingGame) -> usize {
    let legal_actions = initial_state.legal_actions();
    let mut action_scores = Vec::new();
    for action in legal_actions {
        //
        let mut state = initial_state.clone();
        state.advance(action);
        state.evaluate_score();
        action_scores.push(ActionScore {
            action,
            score: state.evaluated_score,
        })
    }
    // sort action score and take largest one
    action_scores.sort_by(|a, b| {
        b.score.cmp(&a.score).then_with(|| a.action.cmp(&b.action))
    });

    action_scores[0].action
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greedy_action() {
        let state = NumberCollectingGame::new(0);
        let legal_actions = state.legal_actions();
        let action = greedy_action(&state);
        assert!(legal_actions.contains(&action));
    }
}
