use crate::ch03::maze_state;

/// keep track of the action-score tuple
#[derive(Debug)]
struct ActionScore {
    action: usize,
    score: usize,
}

pub fn greedy_action(
    initial_state: &maze_state::NumberCollectingGame,
) -> usize {
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

    // create a state as a fixture
    fn setup() -> maze_state::NumberCollectingGame {
        let params = maze_state::MazeParams {
            height: 3,
            width: 4,
            end_turn: 3,
        };
        maze_state::NumberCollectingGame::new(0, params)
    }

    #[test]
    fn test_greedy_action() {
        let state = setup();
        let legal_actions = state.legal_actions();
        let action = greedy_action(&state);
        assert!(legal_actions.contains(&action));
    }

    #[test]
    fn test_greedy_is_greedy() {
        // アホだけど1回 print して greedy になっていることを保証する
        let state = setup();
        let actual = state.to_string();
        let expected = "\
turn:\t0
score:\t0

.227
11.4
492@
";
        assert_eq!(actual, expected);

        let legal_actions = state.legal_actions();
        assert_eq!(legal_actions, vec![1, 3]); // 左と上

        let action = greedy_action(&state);
        assert_eq!(action, 3); // 上に行くのが正解
    }
}
