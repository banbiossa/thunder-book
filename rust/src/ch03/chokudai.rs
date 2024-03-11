use crate::base::is_done;
use crate::base::state::SinglePlayerState;
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

    while !stop_condition() {
        for t in 0..beam_depth {
            // split_at_mut to satisty the borrow checker
            let (left, right) = beams.split_at_mut(t + 1);
            let beam = &mut left[t];
            let next_beam = &mut right[0];

            for _i in 0..beam_width {
                if beam.is_empty() {
                    break;
                }

                // only peek because holding on to the state won't compile
                if let Some(state) = beam.peek() {
                    if state.is_done() {
                        break;
                    }
                }
                let state = beam.pop().unwrap();

                let legal_actions = state.legal_actions();
                for action in legal_actions {
                    let mut next_state = state.clone();
                    next_state.advance(action);
                    next_state.evaluate_score();
                    next_state.set_first_action(action);
                    next_beam.push(next_state);
                }
            } // end beam_width
        } // end beam_depth
    } // end beam_number/time

    // count down from the deepest beam to find a state
    for t in (0..=beam_depth).rev() {
        let beam = &mut beams[t];
        if !beam.is_empty() {
            return beam.peek().unwrap().get_first_action();
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

    use crate::base::state;
    use std::{thread, time::Duration};

    use super::*;

    // create a state as a fixture
    fn setup() -> maze_state::NumberCollectingGame {
        let params = state::MazeParams {
            height: 3,
            width: 4,
            end_turn: 3,
        };
        maze_state::NumberCollectingGame::new(0, params)
    }

    #[test]
    fn test_chokudai_search_factory() {
        let state = setup();
        let action_func = chokudai_search_factory(1, 3, 1);
        let action = action_func(&state);
        let legal_actions = state.legal_actions();
        assert!(legal_actions.contains(&action));
    }

    #[test]
    fn test_chokudai_search_timed_factory() {
        let state = setup();
        let action_func = chokudai_search_timed_factory(1, 3, 1);
        let action = action_func(&state);
        let legal_actions = state.legal_actions();
        assert!(legal_actions.contains(&action));
    }

    #[test]
    fn chokudai_search_timed_returns() {
        let mut state = setup();
        let legal_actions = state.legal_actions();
        let action = chokudai_search_timed_action(&state, 1, 3, 1);
        assert!(legal_actions.contains(&action));
        state.advance(action);
    }

    #[test]
    fn chokudai_search_timed_returns_after_advance() {
        let mut state = setup();
        let legal_actions = state.legal_actions();
        let action = chokudai_search_timed_action(&state, 1, 3, 1);
        assert!(legal_actions.contains(&action));

        state.advance(action);
        state.evaluate_score();
        assert_eq!(state.get_turn(), 1);
        assert!(state.get_game_score() > 0);

        thread::sleep(Duration::from_millis(10));
        let legal_actions = state.legal_actions();
        let action = chokudai_search_timed_action(&state, 1, 3, 1);
        assert!(legal_actions.contains(&action));
    }

    #[test]
    fn chokudai_search_returns_value() {
        let state = setup();
        let legal_actions = state.legal_actions();
        let action = chokudai_search_action(&state, 1, 3, 1);
        assert!(legal_actions.contains(&action));
    }
}
