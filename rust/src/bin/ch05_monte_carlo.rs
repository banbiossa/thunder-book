use std::time::Instant;

use search::base::alternate::{ActionFunc, MazeParams};
use search::ch05::game;
use search::ch05::maze_state;
use search::ch05::monte_carlo;
use search::ch05::random_action;

use search::log_and_print;

fn main() {
    pub const PARAMS: MazeParams = MazeParams {
        height: 3,
        width: 3,
        end_turn: 4,
    };
    let num_games = 100;
    let print_every = num_games / 10;
    let num_playout = 3000;

    struct ActionName {
        action_funcs: Vec<ActionFunc<maze_state::AlternateMazeState>>,
        name: String,
    }

    let action_names = vec![
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
    ];

    log_and_print!("| name | win_rate | time |");
    log_and_print!("| ---- | -------- | ---- |");

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
