use search::ch06::game;
use search::ch06::maze_state;
use search::ch06::mcts;
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

    pub const MCTS_PARAMS: mcts::MCTSParams = mcts::MCTSParams {
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
                mcts::mcts_arc(MCTS_PARAMS, num_playout * 2),
                monte_carlo::monte_carlo_arc(num_playout),
            ],
            name: format!("mcts vs monte carlo {num_playout}"),
        },
        ActionName {
            action_funcs: vec![
                mcts::mcts_arc(MCTS_PARAMS, num_playout),
                random_action::random_action_arc(),
            ],
            name: format!("mcts vs random {num_playout}"),
        },
    ];

    for action_name in action_names.into_iter().rev() {
        println!("{}", action_name.name);
        let result = game::average(
            PARAMS,
            action_name.action_funcs,
            num_games,
            print_every,
        );
        println!("result {result:.2} of {}", action_name.name);
    }
}
