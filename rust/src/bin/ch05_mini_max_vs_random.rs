use std::sync::Arc;

use search::ch05::game;
use search::ch05::maze_state;
use search::ch05::mini_max;
use search::ch05::random_action;

fn main() {
    pub const PARAMS: maze_state::MazeParams = maze_state::MazeParams {
        height: 3,
        width: 3,
        end_turn: 4,
    };
    let depth = PARAMS.end_turn;
    let num_games = 1000;
    let print_every = 100;

    struct ActionName {
        action_funcs: Vec<Arc<maze_state::ActionFunc>>,
        name: String,
    }

    let action_names = vec![
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
    ];

    for action_name in action_names {
        let result = game::play_black_white(
            PARAMS,
            action_name.action_funcs,
            num_games,
            print_every,
        );
        println!("result {result:.2} of {}", action_name.name);
    }
}
