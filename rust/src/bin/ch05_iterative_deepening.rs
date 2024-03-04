use std::sync::Arc;

use search::ch05::game;
use search::ch05::iterative_deepening;
use search::ch05::maze_state;

fn main() {
    pub const PARAMS: maze_state::MazeParams = maze_state::MazeParams {
        height: 5,
        width: 5,
        end_turn: 10,
    };
    let num_games = 100;
    let print_every = 10;

    struct ActionName {
        action_funcs: Vec<Arc<maze_state::ActionFunc>>,
        name: String,
    }

    let mut action_names = Vec::new();
    let patterns = vec![(1, 1), (2, 1), (10, 1), (100, 1)];
    for (a, b) in patterns {
        action_names.push(ActionName {
            action_funcs: vec![
                iterative_deepening::iterative_deepening_action_arc(a),
                iterative_deepening::iterative_deepening_action_arc(b),
            ],
            name: format!("iterative deepening {a}ms vs. {b} ms"),
        })
    }

    for action_name in action_names {
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
