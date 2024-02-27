use crate::ch03::maze_state;
use std::collections::BinaryHeap;

/**
 * beam search
*/
pub fn beam_search_action(
    initial_state: &maze_state::NumberCollectingGame,
    beam_width: usize,
    beam_depth: usize,
) -> usize {
    let mut best_state: Option<&maze_state::NumberCollectingGame> = None;

    let mut beam = BinaryHeap::new();
    beam.push(initial_state.clone());

    for t in 0..beam_depth {
        let mut next_beam = BinaryHeap::new();

        for _ in 0..beam_width {
            // we can do match beam.pop() but early return seems more clear
            if beam.is_empty() {
                break;
            }
            let state = beam.pop().unwrap();
            let legal_actions = state.legal_actions();
            for action in legal_actions {
                let mut next_state = state.clone();
                next_state.advance(action);
                next_state.evaluate_score();
                if t == 0 {
                    next_state.first_action = Some(action);
                }
                next_beam.push(next_state);
            }
        } // end width

        beam = next_beam;
        best_state = beam.peek();

        if best_state.unwrap().is_done() {
            break;
        }
    } // end depth
    best_state.unwrap().first_action.unwrap()
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::ch03::greedy;

    #[test]
    fn test_beam_search_depth_1_is_greedy() {
        // beam search of depth 1, width max should be equal to greedy
        let state = maze_state::NumberCollectingGame::new(0);
        let legal_actions = state.legal_actions();
        let beam_action = beam_search_action(&state, legal_actions.len(), 1);
        let greedy_action = greedy::greedy_action(&state);
        assert_eq!(beam_action, greedy_action);
    }

    #[test]
    fn test_beam_search_deep_is_ok() {
        // check that a deep and wide action can be taken
        let state = maze_state::NumberCollectingGame::new(0);
        let legal_actions = state.legal_actions();
        let beam_action = beam_search_action(&state, 10, 10);
        assert!(legal_actions.contains(&beam_action));
    }
}
