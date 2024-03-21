use std::sync::Arc;

use crate::base::alternate::{AlternateState, Evaluatable};
use crate::base::is_done;
use crate::ch05::alpha_beta;
use crate::ch05::maze_state;

// tracks the final result of the deepening
#[derive(Debug)]
struct DeepeningResult {
    action: usize,
    _depth: usize,
}

// (state, depth) -> action
// type Deepenable = dyn Fn(&maze_state::AlternateMazeState, usize) -> usize;

// do alpha-beta, keep deeper until time is up
// it doesn't early stop when time is up during iteration
// but won't use the result so isn't cheating.
// 早めに終わる実装をやってもいいけど alpha-beta には手を入れたくないのでこのままで
// is_time_up decorator を作って各loopも関数にして、見たいのでやればできる気はする
fn iterative_deepening_action<T: AlternateState + Evaluatable>(
    state: &T,
    time_threshold_ms: u64,
    // todo: action: Deepenable にして alpha beta 側に実装する
) -> DeepeningResult {
    let mut best: Option<usize> = None;
    let mut depth = 1;

    let mut is_time_up = is_done::time_stopper(time_threshold_ms);
    loop {
        // call first because timer starts from first call
        if is_time_up() {
            break;
        }
        // todo: action_func(&state) にする
        let action = alpha_beta::alpha_beta_arc(depth)(state);
        best = Some(action);
        depth += 1;
    }

    DeepeningResult {
        action: best.unwrap(),
        _depth: depth,
    }
}

pub fn iterative_deepening_action_arc<T: AlternateState + Evaluatable>(
    time_threshold_ms: u64,
) -> maze_state::ActionFunc<T> {
    Arc::new(move |state| -> usize {
        iterative_deepening_action(state, time_threshold_ms).action
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::alternate::MazeParams;

    fn setup() -> maze_state::AlternateMazeState {
        let params = MazeParams {
            height: 3,
            width: 3,
            end_turn: 4,
        };
        maze_state::AlternateMazeState::new(0, params)
    }

    #[test]
    fn test_iterative_deepening_arc() {
        let state = setup();
        let action = iterative_deepening_action_arc(1)(&state);
        assert_eq!(action, 0);
    }

    #[test]
    fn test_iterative_deepening() {
        let state = setup();
        let short = iterative_deepening_action(&state, 1);
        let long = iterative_deepening_action(&state, 2);

        assert!(short._depth < long._depth);
        // assert_eq!(short._depth, long._depth);
        assert_eq!(short.action, 0);
        assert_eq!(long.action, 0);
    }
}
