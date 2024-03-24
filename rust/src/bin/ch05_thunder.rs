use std::time::Instant;

use search::base::alternate::{ActionFunc, MazeParams};
use search::ch05::game;
use search::ch05::iterative_deepening;
use search::ch05::maze_state;
use search::ch05::mcts;
use search::ch05::thunder;
use search::log_and_print;

fn main() {
    pub const PARAMS: MazeParams = MazeParams {
        height: 5,
        width: 5,
        end_turn: 10,
    };
    let num_games = 100;
    let print_every = num_games / 10;
    let num_playout = 300;

    struct ActionName {
        action_funcs: Vec<ActionFunc<maze_state::AlternateMazeState>>,
        name: String,
    }

    pub const MCTS_PARAMS: mcts::MCTSParams = mcts::MCTSParams {
        c: 1.0,
        expand_threshold: 10,
    };

    let action_names = vec![
        ActionName {
            action_funcs: vec![
                thunder::thunder_search_arc(num_playout),
                mcts::mcts_action_arc(num_playout, MCTS_PARAMS),
            ],
            name: format!("thunder vs. mcts num_playout {num_playout}"),
        },
        ActionName {
            action_funcs: vec![
                thunder::thunder_timebound_arc(1),
                mcts::mcts_timebound_arc(1, MCTS_PARAMS),
            ],
            name: format!("thunder vs. mcts 1ms"),
        },
        ActionName {
            action_funcs: vec![
                thunder::thunder_timebound_arc(1),
                iterative_deepening::iterative_deepening_action_arc(1),
            ],
            name: format!("thunder vs. iterative deepening 1ms"),
        },
    ];

    log_and_print!("|name|score|time|");
    log_and_print!("|----|-----|----|");

    for action_name in action_names.into_iter().rev() {
        println!("{}", action_name.name);
        let start = Instant::now();
        let result = game::play_black_white(
            PARAMS,
            action_name.action_funcs,
            num_games,
            print_every,
        );
        let elapsed = start.elapsed().as_secs_f32();
        println!("result {result:.2} of {}", action_name.name);
        log_and_print!(
            "| {} | {:.2}% | {:.2}s |",
            action_name.name,
            result * 100.0,
            elapsed
        );
    }
}
