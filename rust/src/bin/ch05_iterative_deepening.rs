use search::base::alternate::MazeParams;
use search::ch05::game;
use search::ch05::iterative_deepening;
use search::ch05::maze_state;

fn main() {
    pub const PARAMS: MazeParams = MazeParams {
        height: 5,
        width: 5,
        end_turn: 50,
    };
    let num_games = 100;
    let print_every = 10;

    struct ActionName {
        action_funcs:
            Vec<maze_state::ActionFunc<maze_state::AlternateMazeState>>,
        name: String,
    }

    // compare long and short run time
    let mut action_names = Vec::new();
    let patterns = vec![(1, 1), (2, 1), (10, 1), (100, 1)];
    for (long, short) in patterns {
        action_names.push(ActionName {
            action_funcs: vec![
                iterative_deepening::iterative_deepening_action_arc(long),
                iterative_deepening::iterative_deepening_action_arc(short),
            ],
            name: format!("iterative deepening {long}ms vs. {short} ms"),
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
