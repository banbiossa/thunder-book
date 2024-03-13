use crate::base::state::{HashableState, SinglePlayerState};
use crate::base::{is_done, state::ActionFunc};
use std::collections::{BinaryHeap, HashSet};

pub fn beam_search_hash_box<T: SinglePlayerState + HashableState>(
    beam_width: usize,
    beam_depth: usize,
) -> ActionFunc<T> {
    Box::new(move |state| -> usize {
        beam_search_with_hash(
            state,
            beam_width,
            is_done::depth_stopper(beam_depth),
        )
    })
}

fn beam_search_with_hash<T: SinglePlayerState + HashableState>(
    initial_state: &T,
    beam_width: usize,
    mut stop_condition: is_done::Stopper,
) -> usize {
    let mut best_state = initial_state.clone();

    let mut beam = BinaryHeap::new();
    beam.push(initial_state.clone());
    let mut hash_check = HashSet::new();

    // stop on max_depth or max_time
    // todo: this needs to be stopper.check_and_increment()
    while !stop_condition() {
        let mut next_beam = BinaryHeap::new();

        for _ in 0..beam_width {
            // we can do match beam.pop() but early return seems more clear
            if beam.is_empty() {
                break;
            }
            let top_state = beam.pop().unwrap();
            let legal_actions = top_state.legal_actions();
            for action in legal_actions {
                let mut state = top_state.clone();
                state.advance(action);
                if hash_check.contains(&state.get_hash()) {
                    continue;
                }
                hash_check.insert(state.get_hash());
                state.evaluate_score();
                state.set_first_action(action);
                next_beam.push(state);
            }
        } // end width

        beam = next_beam;
        // best_state = beam.peek();
        best_state = beam.peek().cloned().unwrap_or(best_state);

        // end based on turn
        if best_state.is_done() {
            break;
        }
    } // end depth/time
    best_state.get_first_action()
}

#[cfg(test)]
mod tests {
    use crate::{base::state::MazeParams, ch07::zobrist_hash::ZobristState};

    use super::*;

    fn setup() -> ZobristState {
        let params = MazeParams {
            height: 5,
            width: 5,
            end_turn: 3,
        };
        let state = ZobristState::new(0, params);
        state
    }

    #[test]
    fn test_beam_search_with_hash() {
        let state = setup();
        let actual = beam_search_hash_box(3, 3)(&state);
        assert_eq!(actual, 1);
    }
}
