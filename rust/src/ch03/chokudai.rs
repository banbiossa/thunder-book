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
        chokudai_search_action(state, beam_width, beam_depth, beam_number)
    })
}

pub fn chokudai_search_timed_factory(
    beam_width: usize,
    beam_depth: usize,
    time_threshold_ms: u64,
) -> Box<dyn Fn(&maze_state::NumberCollectingGame) -> usize> {
    Box::new(move |state| -> usize {
        chokudai_search_timed_action(
            state,
            beam_width,
            beam_depth,
            time_threshold_ms,
        )
    })
}

// returns an action based on chokudai search
fn chokudai_search<F>(
    initial_state: &maze_state::NumberCollectingGame,
    beam_width: usize,
    beam_depth: usize,
    mut stop_condition: F,
) -> usize
where
    F: FnMut() -> bool,
{
    // keep track of all beams
    let mut beams: Vec<BinaryHeap<maze_state::NumberCollectingGame>> =
        vec![BinaryHeap::new(); beam_depth + 1];
    beams[0].push(initial_state.clone());
    assert!(initial_state.first_action.is_none());

    println!("print initial {beams:?}");

    while !stop_condition() {
        for t in 0..beam_depth {
            println!("print beams at depth {t} => {beams:?}");
            // split_at_mut to satisty the borrow checker
            let (left, right) = beams.split_at_mut(t + 1);
            let beam = &mut left[t];
            let next_beam = &mut right[0];

            for _i in 0..beam_width {
                if beam.is_empty() {
                    break;
                }

                let state = beam.peek().unwrap();
                if state.is_done() {
                    break;
                }
                // beam.pop();
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
    } // end beam_number/time

    // count down from the deepest beam to find a state
    for t in (0..=beam_depth).rev() {
        let beam = &mut beams[t];
        if !beam.is_empty() {
            return beam.peek().unwrap().first_action.unwrap();
        }
    }
    panic!("beam should have a value to return");
}

fn chokudai_search_action(
    initial_state: &maze_state::NumberCollectingGame,
    beam_width: usize,
    beam_depth: usize,
    beam_number: usize,
) -> usize {
    let stopper = is_done::depth_stopper(beam_number);
    chokudai_search(initial_state, beam_width, beam_depth, stopper)
}

fn chokudai_search_timed_action(
    initial_state: &maze_state::NumberCollectingGame,
    beam_width: usize,
    beam_depth: usize,
    time_threshold_ms: u64,
) -> usize {
    let stopper = is_done::time_stopper(time_threshold_ms);
    chokudai_search(initial_state, beam_width, beam_depth, stopper)
}

#[cfg(test)]
mod tests {

    use std::{thread, time::Duration};

    use super::*;

    #[test]
    fn test_chokudai_search_factory() {
        let state = maze_state::NumberCollectingGame::new(0);
        let action_func = chokudai_search_factory(1, 3, 1);
        let action = action_func(&state);
        let legal_actions = state.legal_actions();
        assert!(legal_actions.contains(&action));
    }

    #[test]
    fn test_chokudai_search_timed_factory() {
        let state = maze_state::NumberCollectingGame::new(0);
        let action_func = chokudai_search_timed_factory(1, 3, 1);
        let action = action_func(&state);
        let legal_actions = state.legal_actions();
        assert!(legal_actions.contains(&action));
    }

    #[test]
    fn chokudai_search_timed_returns() {
        let mut state = maze_state::NumberCollectingGame::new(0);
        let legal_actions = state.legal_actions();
        let action = chokudai_search_timed_action(&state, 1, 3, 1);
        assert!(legal_actions.contains(&action));

        state.advance(action)
    }

    #[test]
    fn chokudai_search_timed_returns_after_advance() {
        let mut state = maze_state::NumberCollectingGame::new(0);
        let legal_actions = state.legal_actions();
        let action = chokudai_search_timed_action(&state, 1, 3, 1);
        assert!(legal_actions.contains(&action));

        state.advance(action);
        state.evaluate_score();
        assert_eq!(state.turn, 1);
        assert!(state.game_score > 0);

        thread::sleep(Duration::from_millis(10));
        let legal_actions = state.legal_actions();
        let action = chokudai_search_timed_action(&state, 1, 3, 1);
        assert!(legal_actions.contains(&action));
    }

    #[test]
    fn chokudai_search_returns_value() {
        let state = maze_state::NumberCollectingGame::new(0);
        let legal_actions = state.legal_actions();
        let action = chokudai_search_action(&state, 1, 3, 1);
        assert!(legal_actions.contains(&action));
    }
}
