use crate::base::is_done;
use crate::ch03::maze_state;
use std::collections::BinaryHeap;

/**
 * beam search
 * the actual implementation is in beam_search_action
 * the caller needs a state->usize function to compare between actions
 * so the factory is the public facing method
*/
pub fn beam_search_factory(
    beam_width: usize,
    beam_depth: usize,
) -> Box<dyn Fn(&maze_state::NumberCollectingGame) -> usize> {
    Box::new(move |state| -> usize {
        beam_search_action(state, beam_width, beam_depth)
    })
}

pub fn beam_search_timed_factory(
    beam_width: usize,
    time_threshold_ms: u64,
) -> Box<dyn Fn(&maze_state::NumberCollectingGame) -> usize> {
    Box::new(move |state| -> usize {
        beam_search_action_with_time(state, beam_width, time_threshold_ms)
    })
}

/** a modular beam search that can take a function as the
 stopping condition
 * stop_condition is a function that returns a bool when called
 * currenty depth_stopper and time_stopper can be used
 which mostly work like a while loop and time_keeper
*/
fn beam_search<F>(
    initial_state: &maze_state::NumberCollectingGame,
    beam_width: usize,
    mut stop_condition: F,
) -> usize
where
    F: FnMut() -> bool,
{
    let mut best_state: Option<&maze_state::NumberCollectingGame> = None;

    let mut beam = BinaryHeap::new();
    beam.push(initial_state.clone());

    // stop on max_depth or max_time
    // todo: this needs to be stopper.check_and_increment()
    while !stop_condition() {
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
                if next_state.first_action.is_none() {
                    next_state.first_action = Some(action);
                }
                next_beam.push(next_state);
            }
        } // end width

        beam = next_beam;
        best_state = beam.peek();

        // end based on turn
        if best_state.unwrap().is_done() {
            break;
        }
    } // end depth/time
    best_state.unwrap().first_action.unwrap()
}

// depth based stopping condition
// you can call this max_depth times before it returns true
// basically just a while loop
fn depth_stopper(max_depth: usize) -> Box<dyn FnMut() -> bool> {
    let mut depth = 0;
    Box::new(move || {
        depth += 1;
        depth > max_depth
    })
}

// time_keeper basesd stopping condition
fn time_stopper(time_threshold_ms: u64) -> Box<dyn FnMut() -> bool> {
    //
    let time_keeper = is_done::TimeKeeper::new(time_threshold_ms);
    Box::new(move || time_keeper.is_over())
}

fn beam_search_action(
    initial_state: &maze_state::NumberCollectingGame,
    beam_width: usize,
    beam_depth: usize,
) -> usize {
    // the depth stopper is just a while loop like
    // while count < beam_depth
    // it is used for modularity
    let stopper = depth_stopper(beam_depth);
    beam_search(initial_state, beam_width, stopper)
}

fn beam_search_action_with_time(
    initial_state: &maze_state::NumberCollectingGame,
    beam_width: usize,
    time_threshold_ms: u64,
) -> usize {
    // the time stopper is just a
    // while !time_keeper.is_done()
    // used for modularity
    let stopper = time_stopper(time_threshold_ms);
    beam_search(initial_state, beam_width, stopper)
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::ch03::greedy;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_time_stopper() {
        let mut stopper = time_stopper(0);
        assert_eq!(stopper(), true);

        // this test may be flaky, as it is based on running time
        let mut stopper = time_stopper(10);
        assert_eq!(stopper(), false);
        thread::sleep(Duration::from_millis(10));
        assert_eq!(stopper(), true);
    }

    #[test]
    fn test_depth_stopper() {
        // test that the stopper logic works
        let mut stopper = depth_stopper(0);
        // 0st call is true
        assert_eq!(stopper(), true);
        // 1st call is also true
        assert_eq!(stopper(), true);

        let mut stopper = depth_stopper(2);
        assert_eq!(stopper(), false);
        assert_eq!(stopper(), false);
        assert_eq!(stopper(), true);
    }

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
    fn test_beam_search_with_time() {
        let state = maze_state::NumberCollectingGame::new(0);
        let legal_actions = state.legal_actions();
        let beam_action_timed = beam_search_action_with_time(&state, 10, 1);
        assert!(legal_actions.contains(&beam_action_timed));
    }

    #[test]
    fn test_beam_search_deep_is_ok() {
        // check that a deep and wide action can be taken
        let state = maze_state::NumberCollectingGame::new(0);
        let legal_actions = state.legal_actions();
        let beam_action = beam_search_action(&state, 10, 10);
        assert!(legal_actions.contains(&beam_action));
    }

    #[test]
    fn beam_action_and_beam_action_factory_give_same_results() {
        let state = maze_state::NumberCollectingGame::new(0);

        let beam_width = 10;
        let beam_depth = 10;
        let beam_action = beam_search_action(&state, beam_width, beam_depth);

        let action_f = beam_search_factory(beam_width, beam_depth);
        let factory_action = action_f(&state);

        assert_eq!(beam_action, factory_action);
    }
}
