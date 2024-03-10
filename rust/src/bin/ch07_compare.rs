use search::base::state::{ActionFunc, MazeParams};
use search::ch03::game;
use search::ch03::{beam_search, random_action};
use search::ch07::maze_state::WallMazeState;

/** compare random, greedy, beam_search
 *
 */

struct ActionNamePair {
    action_func: ActionFunc<WallMazeState>,
    name: String,
}

fn main() {
    pub const PARAMS: MazeParams = MazeParams {
        height: 7,
        width: 7,
        end_turn: 49,
    };
    let num_games = 100;

    let beam_width = 10;
    let beam_depth = PARAMS.end_turn;

    let action_funcs = vec![
        ActionNamePair {
            action_func: random_action::random_action_box(),
            name: "random".to_string(),
        },
        ActionNamePair {
            action_func: beam_search::beam_search_factory(
                beam_width, beam_depth,
            ),
            name: format!(
                "beam_search - width: {beam_width}, depth: {beam_depth} "
            ),
        },
    ];

    for pair in action_funcs.into_iter().rev() {
        println!("do {}", pair.name);
        let average = game::average(PARAMS, pair.action_func, num_games, 10);
        println!(
            "average {average} of {} over num_games {num_games}\n",
            pair.name,
        );
    }
}
