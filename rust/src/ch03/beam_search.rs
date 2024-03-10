use crate::base::is_done;
use crate::base::state::SinglePlayerState;
use std::collections::BinaryHeap;

/**
 * beam search
 * the actual implementation is in beam_search_action
 * the caller needs a state->usize function to compare between actions
 * so the factory is the public facing method
*/
pub fn beam_search_factory<T: SinglePlayerState>(
    beam_width: usize,
    beam_depth: usize,
) -> Box<dyn Fn(&T) -> usize> {
    Box::new(move |state| -> usize {
        beam_search_action(state, beam_width, beam_depth)
    })
}

pub fn beam_search_timed_factory<T: SinglePlayerState>(
    beam_width: usize,
    time_threshold_ms: u64,
) -> Box<dyn Fn(&T) -> usize> {
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
fn beam_search<T: SinglePlayerState>(
    initial_state: &T,
    beam_width: usize,
    mut stop_condition: is_done::Stopper,
) -> usize {
    let mut best_state: Option<&T> = None;

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
                next_state.set_first_action(action);
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
    best_state.unwrap().get_first_action()
}

fn beam_search_action<T: SinglePlayerState>(
    initial_state: &T,
    beam_width: usize,
    beam_depth: usize,
) -> usize {
    // the depth stopper is just a while loop like
    // while count < beam_depth
    // it is used for modularity
    let stopper = is_done::depth_stopper(beam_depth);
    beam_search(initial_state, beam_width, stopper)
}

fn beam_search_action_with_time<T: SinglePlayerState>(
    initial_state: &T,
    beam_width: usize,
    time_threshold_ms: u64,
) -> usize {
    // the time stopper is just a
    // while !time_keeper.is_done()
    // used for modularity
    let stopper = is_done::time_stopper(time_threshold_ms);
    beam_search(initial_state, beam_width, stopper)
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::base::state;
    use crate::ch03::greedy;
    use crate::ch03::maze_state;

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
    fn test_beam_search_depth_1_is_greedy() {
        // beam search of depth 1, width max should be equal to greedy
        let state = setup();
        let legal_actions = state.legal_actions();
        let beam_action = beam_search_action(&state, legal_actions.len(), 1);
        let greedy_action = greedy::greedy_action(&state);
        assert_eq!(beam_action, greedy_action);
    }

    #[test]
    fn test_beam_search_with_time() {
        let state = setup();
        let legal_actions = state.legal_actions();
        let beam_action_timed = beam_search_action_with_time(&state, 10, 1);
        assert!(legal_actions.contains(&beam_action_timed));
    }

    #[test]
    fn test_beam_search_deep_is_ok() {
        // check that a deep and wide action can be taken
        let state = setup();
        let legal_actions = state.legal_actions();
        let beam_action = beam_search_action(&state, 10, 10);
        assert!(legal_actions.contains(&beam_action));
    }

    #[test]
    fn beam_action_and_beam_action_factory_give_same_results() {
        let state = setup();

        let beam_width = 10;
        let beam_depth = 10;
        let beam_action = beam_search_action(&state, beam_width, beam_depth);

        let action_f = beam_search_factory(beam_width, beam_depth);
        let factory_action = action_f(&state);

        assert_eq!(beam_action, factory_action);
    }
}
