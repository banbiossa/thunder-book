use search::base::alternate::{ActionFunc, MazeParams};
use search::ch05::game;
use search::ch05::maze_state;
use search::ch05::monte_carlo;
use search::ch05::random_action;

fn main() {
    pub const PARAMS: MazeParams = MazeParams {
        height: 3,
        width: 3,
        end_turn: 4,
    };
    let num_games = 1000;
    let print_every = 100;
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
                monte_carlo::monte_carlo_action_arc(num_playout),
                monte_carlo::monte_carlo_action_arc(30),
            ],
            name: format!("monte_carlo num_playout {num_playout} vs 30"),
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
