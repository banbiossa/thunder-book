use crate::base::is_done;
use crate::ch03::maze_state;
use std::collections::BinaryHeap;

/// the public API of chokudai_search is a factory to pass to
/// game(*)
pub fn chokudai_search_factory(
    beam_width: usize,
    beam_depth: usize,
    beam_number: usize,
) -> Box<dyn Fn(&maze_state::NumberCollectingGame) -> usize> {
    Box::new(move |state| -> usize {
        chokudai_search(
            state,
            beam_width,
            beam_depth,
            beam_number,
            is_done::no_stop(),
        )
    })
}

pub fn chokudai_search_timed_factory(beam_width: usize) {}

// returns an action based on chokudai search
fn chokudai_search<F>(
    initial_state: &maze_state::NumberCollectingGame,
    beam_width: usize,
    beam_depth: usize,
    beam_number: usize,
    mut stop_condition: F,
) -> usize
where
    F: FnMut() -> bool,
{
    // keep track of all beams
    let mut beams: Vec<BinaryHeap<maze_state::NumberCollectingGame>> =
        vec![BinaryHeap::new(); beam_depth + 1];
    beams[0].push(initial_state.clone());

    for _cnt in 0..beam_number {
        for t in 0..beam_depth {
            // split_at_mut to satisty the borrow checker
            let (left, right) = beams.split_at_mut(t + 1);
            let beam = &mut left[t];
            let next_beam = &mut right[0];

            for _i in 0..beam_width {
                if beam.is_empty() {
                    break;
                }
                let state = beam.pop().unwrap();
                if state.is_done() {
                    break;
                }
                let legal_actions = state.legal_actions();
                for action in legal_actions {
                    let mut next_state = state.clone();
                    next_state.advance(action);
                    next_state.evaluate_score();
                    if next_state.first_action.is_none() {
                        next_state.first_action = Some(action);
                    }
                    next_beam.push(next_state);
                }
            } // end beam_width
        } // end beam_depth
        if stop_condition() {
            break;
        }
    } // end beam_number

    // count down from the deepest beam to find a state
    for t in (0..=beam_depth).rev() {
        let beam = &mut beams[t];
        if !beam.is_empty() {
            return beam.peek().unwrap().first_action.unwrap();
        }
    }
    panic!("beam should have a value to return");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn chokudai_search_returns_value() {
        let state = maze_state::NumberCollectingGame::new(0);
        let legal_actions = state.legal_actions();
        let action = chokudai_search(&state, 1, 3, 1, is_done::no_stop());
        assert!(legal_actions.contains(&action));
    }
}
