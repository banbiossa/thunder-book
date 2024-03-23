use std::time::Instant;

use search::base::state::{ActionFunc, MazeParams};
use search::ch03::beam_search;
use search::ch03::chokudai;
use search::ch03::game;
use search::ch03::greedy;
use search::ch03::maze_state::NumberCollectingGame;
use search::ch03::random_action;
use search::log_and_print;

/** compare random, greedy, beam_search
 *
 */

struct ActionNamePair {
    action_func: ActionFunc<NumberCollectingGame>,
    name: String,
}

fn main() {
    pub const PARAMS: MazeParams = MazeParams {
        height: 30,
        width: 30,
        end_turn: 100,
    };
    let num_games = 100;

    // let beam_width = 10;
    let beam_depth = PARAMS.end_turn;

    let action_funcs = vec![
        ActionNamePair {
            action_func: Box::new(random_action::random_action),
            name: "random".to_string(),
        },
        ActionNamePair {
            action_func: Box::new(greedy::greedy_action),
            name: "greedy".to_string(),
        },
        ActionNamePair {
            action_func: beam_search::beam_search_factory(2, beam_depth),
            name: format!("beam_search - width: 2, depth: {beam_depth} "),
        },
        ActionNamePair {
            action_func: beam_search::beam_search_timed_factory(5, 1),
            name: format!("beam search - width: 5, time: 1ms"),
        },
        ActionNamePair {
            action_func: beam_search::beam_search_timed_factory(5, 10),
            name: format!("beam search - width: 5, time: 10ms"),
        },
        ActionNamePair {
            action_func: chokudai::chokudai_search_factory(
                1,
                PARAMS.end_turn,
                2,
            ),
            name: format!("chokudai search - width: 1, 2 beams"),
        },
        ActionNamePair {
            action_func: chokudai::chokudai_search_timed_factory(
                1,
                PARAMS.end_turn,
                1,
            ),
            name: format!("chokudai search - width: 1, 1ms"),
        },
        ActionNamePair {
            action_func: chokudai::chokudai_search_timed_factory(
                1,
                PARAMS.end_turn,
                10,
            ),
            name: format!("chokudai search - width: 1, 10ms"),
        },
    ];

    log_and_print!("| name | score | time |");
    log_and_print!("| ---- | ----- | ---- |");
    for pair in action_funcs {
        println!("do {}", pair.name);
        let start = Instant::now();
        let average = game::average(PARAMS, pair.action_func, num_games, 10);
        let elapsed = start.elapsed().as_secs_f32();
        log_and_print!("| {} | {} | {:.2} |", pair.name, average, elapsed);
    }
}
