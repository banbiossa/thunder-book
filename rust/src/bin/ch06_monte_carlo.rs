use search::ch06::game;
use search::ch06::maze_state;
use search::ch06::monte_carlo;
use search::ch06::random_action;

fn main() {
    pub const PARAMS: maze_state::MazeParams = maze_state::MazeParams {
        height: 5,
        width: 5,
        end_turn: 20,
    };
    let num_games = 500;
    let print_every = num_games / 10;
    let num_playout = 1000;

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
