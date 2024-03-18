use crate::base::state::{HashableState, SinglePlayerState};
use crate::base::{is_done, state::ActionFunc};
use std::collections::{BinaryHeap, HashSet};
use std::rc::Rc;

pub fn beam_search_hash_box<T: SinglePlayerState + HashableState>(
    beam_width: usize,
    beam_depth: usize,
) -> ActionFunc<T> {
    Box::new(move |state| -> usize {
        beam_search_with_hash(Rc::new(state.clone()), beam_width, is_done::depth_stopper(beam_depth))
    })
}

pub fn beam_search_hash_timed_box<T: SinglePlayerState + HashableState>(
    beam_width: usize,
    time_threshold_ms: u64,
) -> ActionFunc<T> {
    Box::new(move |state| -> usize {
        beam_search_with_hash(
            Rc::new(state.clone()),
            beam_width,
            is_done::time_stopper(time_threshold_ms),
        )
    })
}

fn beam_search_with_hash<T: SinglePlayerState + HashableState>(
    initial_state: Rc<T>,
    beam_width: usize,
    mut stop_condition: is_done::Stopper,
) -> usize {
    let mut best_state = initial_state.clone();
    let mut beam = BinaryHeap::new();
    beam.push(initial_state.clone());
    let mut hash_check = HashSet::new();

    while !stop_condition() {
        let mut next_beam = BinaryHeap::new();

        for _ in 0..beam_width {
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
                next_beam.push(Rc::new(state));
            }
        }

        beam = next_beam;
        best_state = beam.peek().cloned().unwrap_or(best_state);

        if best_state.is_done() {
            break;
        }
    }

    best_state.get_first_action()
}
