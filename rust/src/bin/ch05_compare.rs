use std::time::Instant;

use search::base::alternate::{ActionFunc, MazeParams};
use search::ch05::alpha_beta;
use search::ch05::game;
use search::ch05::iterative_deepening;
use search::ch05::maze_state;
use search::ch05::mcts;
use search::ch05::mini_max;
use search::ch05::monte_carlo;
use search::ch05::random_action;
use search::ch05::thunder;
use search::log_and_print;

fn main() {
    pub const PARAMS: MazeParams = MazeParams {
        height: 5,
        width: 5,
        end_turn: 10,
    };
    let depth = PARAMS.end_turn;
    let num_games = 100;
    let print_every = num_games / 10;
    let num_playout = 3000;

    struct ActionName {
        action_funcs: Vec<ActionFunc<maze_state::AlternateMazeState>>,
        name: String,
    }

    pub const MCTS_PARAMS: mcts::MCTSParams = mcts::MCTSParams {
        c: 1.0,
        expand_threshold: 10,
    };

    let action_names = vec![
        // minimax, alpha beta
        ActionName {
            action_funcs: vec![
                mini_max::mini_max_arc(depth),
                random_action::random_action_arc(),
            ],
            name: format!("mini max depth {depth} vs. random"),
        },
        ActionName {
            action_funcs: vec![
                random_action::random_action_arc(),
                random_action::random_action_arc(),
            ],
            name: format!("random vs random"),
        },
        ActionName {
            action_funcs: vec![
                mini_max::mini_max_arc(depth),
                mini_max::mini_max_arc(depth),
            ],
            name: format!("mini max depth {depth} vs. mini max depth {depth}"),
        },
        ActionName {
            action_funcs: vec![
                mini_max::mini_max_arc(depth),
                alpha_beta::alpha_beta_arc(depth),
            ],
            name: format!(
                "mini max depth {depth} vs. alpha-beta depth {depth}"
            ),
        },
        // iterative deepening
        ActionName {
            action_funcs: vec![
                iterative_deepening::iterative_deepening_action_arc(1),
                iterative_deepening::iterative_deepening_action_arc(1),
            ],
            name: format!("iterative deepening 1ms vs. 1ms"),
        },
        ActionName {
            action_funcs: vec![
                iterative_deepening::iterative_deepening_action_arc(2),
                iterative_deepening::iterative_deepening_action_arc(1),
            ],
            name: format!("iterative deepening 2ms vs. 1ms"),
        },
        ActionName {
            action_funcs: vec![
                iterative_deepening::iterative_deepening_action_arc(5),
                iterative_deepening::iterative_deepening_action_arc(1),
            ],
            name: format!("iterative deepening 5ms vs. 1ms"),
        },
        // monte_carlo
        ActionName {
            action_funcs: vec![
                monte_carlo::monte_carlo_action_arc(num_playout),
                random_action::random_action_arc(),
            ],
            name: format!("monte_carlo num_playout {num_playout} vs random"),
        },
        ActionName {
            action_funcs: vec![
                monte_carlo::monte_carlo_action_arc(30),
                random_action::random_action_arc(),
            ],
            name: format!("monte_carlo num_playout 30 vs random"),
        },
        ActionName {
            action_funcs: vec![
                monte_carlo::monte_carlo_action_arc(num_playout),
                monte_carlo::monte_carlo_action_arc(30),
            ],
            name: format!("monte_carlo num_playout {num_playout} vs 30"),
        },
        // mcts
        ActionName {
            action_funcs: vec![
                mcts::mcts_action_arc(num_playout, MCTS_PARAMS),
                monte_carlo::monte_carlo_action_arc(num_playout),
            ],
            name: format!(
                "mcts {num_playout} vs monte_carlo num_playout {num_playout}"
            ),
        },
        ActionName {
            action_funcs: vec![
                mcts::mcts_action_arc(num_playout, MCTS_PARAMS),
                mcts::mcts_action_arc(30, MCTS_PARAMS),
            ],
            name: format!("mcts num_playout {num_playout} vs 30"),
        },
        // thunder
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

    log_and_print!("|name|win_rate%|time|");
    log_and_print!("|----|-----|----|");

    for action_name in action_names {
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
