// same as beam_search_hash but use std::rc::Rc
use crate::base::state::{HashableState, SinglePlayerState};
use crate::base::{is_done, state::ActionFunc};
use std::cell::RefCell;
use std::collections::{BinaryHeap, HashSet};
use std::rc::Rc;

pub fn beam_search_rc_action<T: SinglePlayerState + HashableState>(
    beam_width: usize,
    beam_depth: usize,
) -> ActionFunc<T> {
    Box::new(move |state| -> usize {
        beam_search_hash_rc(
            Rc::new(RefCell::new(state.clone())),
            beam_width,
            is_done::depth_stopper(beam_depth),
        )
    })
}

pub fn beam_search_timed_rc_action<T: SinglePlayerState + HashableState>(
    beam_width: usize,
    time_threshold_ms: u64,
) -> ActionFunc<T> {
    Box::new(move |state| -> usize {
        beam_search_hash_rc(
            Rc::new(RefCell::new(state.clone())),
            beam_width,
            is_done::time_stopper(time_threshold_ms),
        )
    })
}

fn beam_search_hash_rc<T: SinglePlayerState + HashableState>(
    initial_state: Rc<RefCell<T>>,
    beam_width: usize,
    mut stop_condition: is_done::Stopper,
) -> usize {
    let mut best_state = Rc::clone(&initial_state);
    let mut beam = BinaryHeap::new();
    beam.push(Rc::clone(&initial_state));

    let mut hash_check = HashSet::new();

    while !stop_condition() {
        let mut next_beam = BinaryHeap::new();

        for _ in 0..beam_width {
            if beam.is_empty() {
                break;
            }

            let top_state = beam.pop().unwrap();
            let legal_actions = top_state.borrow().legal_actions();

            for action in legal_actions {
                let mut state = top_state.borrow().clone();
                state.advance(action);

                if hash_check.contains(&state.get_hash()) {
                    continue;
                }

                hash_check.insert(state.get_hash());
                state.evaluate_score();
                state.set_first_action(action);

                next_beam.push(Rc::new(RefCell::new(state)));
            }
        }

        beam = next_beam;

        if let Some(top_state) = beam.peek() {
            if top_state.borrow().get_evaluated_score()
                > best_state.borrow().get_evaluated_score()
            {
                best_state = Rc::clone(top_state);
            }
        }

        if best_state.borrow().is_done() {
            break;
        }
    }

    let x = best_state.borrow().get_first_action();
    x
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{base::state::MazeParams, ch07::zobrist_hash::ZobristState};

    fn setup() -> ZobristState {
        let params = MazeParams {
            height: 5,
            width: 5,
            end_turn: 3,
        };
        let state = ZobristState::new(0, params);
        state
        // Rc::new(RefCell::new(state))
    }

    #[test]
    fn test_beam_search_timed() {
        let state = setup();
        let actual = beam_search_timed_rc_action(3, 1)(&state);
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_beam_search_with_hash() {
        let state = setup();
        let actual = beam_search_rc_action(3, 3)(&state);
        assert_eq!(actual, 0);
    }
}
