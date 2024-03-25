use std::time::Instant;

use search::base::alternate::{ActionFunc, MazeParams};
use search::ch05::game;
use search::ch05::maze_state;
use search::ch05::mcts;
use search::ch05::monte_carlo;
use search::log_and_print;

fn main() {
    pub const PARAMS: MazeParams = MazeParams {
        height: 5,
        width: 5,
        end_turn: 50,
    };
    let num_games = 100;
    let print_every = 10;
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
    ];

    log_and_print!("| name | win_rate | time |");
    log_and_print!("| ---- | ----- | ---- |");
    for action_name in action_names.into_iter().rev() {
        println!("{}", action_name.name);
        let start = Instant::now();
        let result = game::play_black_white(
            PARAMS,
            action_name.action_funcs,
            num_games,
            print_every,
        );
        println!("result {result:.2} of {}", action_name.name);
        let elapsed = start.elapsed().as_secs_f32();
        log_and_print!(
            "| {} | {:.2}% | {:.1}s | ",
            action_name.name,
            result * 100.0,
            elapsed
        );
    }
}
