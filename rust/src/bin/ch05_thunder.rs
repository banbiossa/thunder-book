use std::sync::Arc;

use search::ch05::game;
use search::ch05::iterative_deepening;
use search::ch05::maze_state;
use search::ch05::mcts;
use search::ch05::thunder;

fn main() {
    pub const PARAMS: maze_state::MazeParams = maze_state::MazeParams {
        height: 5,
        width: 5,
        end_turn: 50,
    };
    let num_games = 100;
    let print_every = 100;
    let num_playout = 300;

    struct ActionName {
        action_funcs: Vec<Arc<maze_state::ActionFunc>>,
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

    for action_name in action_names.into_iter().rev() {
        println!("{}", action_name.name);
        let result = game::play_black_white(
            PARAMS,
            action_name.action_funcs,
            num_games,
            print_every,
        );
        println!("result {result:.2} of {}", action_name.name);
    }
}
