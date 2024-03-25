use std::time::Instant;

use search::ch06::duct;
use search::ch06::game;
use search::ch06::maze_state;
use search::ch06::mcts;
use search::ch06::monte_carlo;
use search::ch06::random_action;
use search::log_and_print;

fn main() {
    pub const PARAMS: maze_state::MazeParams = maze_state::MazeParams {
        height: 5,
        width: 5,
        end_turn: 20,
    };
    let num_games = 100;
    let print_every = num_games / 10;
    let num_playout = 1000;

    pub const MCTS_PARAMS: mcts::MCTSParams = mcts::MCTSParams {
        c: 1.0,
        expand_threshold: 5,
    };

    pub const DUCT_PARAMS: duct::DuctParams = duct::DuctParams {
        c: 1.0,
        expand_threshold: 5,
    };

    struct ActionName {
        action_funcs: Vec<maze_state::ActionFunc>,
        name: String,
    }

    let action_names = vec![
        ActionName {
            action_funcs: vec![
                monte_carlo::monte_carlo_arc(num_playout),
                random_action::random_action_arc(),
            ],
            name: format!("monte_carlo num_playout {num_playout} vs random"),
        },
        ActionName {
            action_funcs: vec![
                monte_carlo::monte_carlo_arc(num_playout),
                monte_carlo::monte_carlo_arc(num_playout / 100),
            ],
            name: format!(
                "monte_carlo num_playout {num_playout} vs {}",
                num_playout / 100
            ),
        },
        ActionName {
            action_funcs: vec![
                mcts::mcts_arc(MCTS_PARAMS, num_playout),
                monte_carlo::monte_carlo_arc(num_playout),
            ],
            name: format!("mcts vs monte carlo {num_playout}"),
        },
        // mcts
        ActionName {
            action_funcs: vec![
                mcts::mcts_arc(MCTS_PARAMS, num_playout * 2),
                monte_carlo::monte_carlo_arc(num_playout),
            ],
            name: format!(
                "mcts {} vs monte carlo {}",
                num_playout * 2,
                num_playout
            ),
        },
        ActionName {
            action_funcs: vec![
                mcts::mcts_arc(MCTS_PARAMS, num_playout),
                random_action::random_action_arc(),
            ],
            name: format!("mcts vs random {num_playout}"),
        },
        // duct
        ActionName {
            action_funcs: vec![
                mcts::mcts_arc(MCTS_PARAMS, num_playout * 2),
                duct::duct_arc(DUCT_PARAMS, num_playout),
            ],
            name: format!("mcts vs duct {num_playout}"),
        },
        ActionName {
            action_funcs: vec![
                duct::duct_arc(DUCT_PARAMS, num_playout),
                monte_carlo::monte_carlo_arc(num_playout),
            ],
            name: format!("duct vs. monte carlo {num_playout}"),
        },
        ActionName {
            action_funcs: vec![
                duct::duct_arc(DUCT_PARAMS, num_playout),
                random_action::random_action_arc(),
            ],
            name: format!("duct vs random {num_playout}"),
        },
    ];

    log_and_print!("| name | win rate | time |");
    log_and_print!("| ---- | -------- | ---- |");
    for action_name in action_names {
        println!("{}", action_name.name);
        let start = Instant::now();
        let result = game::average(
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
