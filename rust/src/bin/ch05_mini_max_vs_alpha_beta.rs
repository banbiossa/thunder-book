use search::base::alternate::{ActionFunc, MazeParams};
use search::ch05::alpha_beta;
use search::ch05::game;
use search::ch05::maze_state;
use search::ch05::mini_max;

fn main() {
    pub const PARAMS: MazeParams = MazeParams {
        height: 3,
        width: 3,
        end_turn: 10,
    };

    let num_states = 100;
    let states = game::sample_states(num_states, 0, PARAMS);

    struct ActionName {
        action_func: ActionFunc<maze_state::AlternateMazeState>,
        name: String,
    }
    let depth = PARAMS.end_turn;

    let action_names = vec![
        ActionName {
            action_func: mini_max::mini_max_arc(depth),
            name: format!("mini max depth {depth}"),
        },
        ActionName {
            action_func: alpha_beta::alpha_beta_arc(depth),
            name: format!("alpa beta depth {depth}"),
        },
    ];

    for action_name in action_names {
        let run_time = game::how_fast(action_name.action_func, &states);
        println!(
            "{} took {run_time} ms for {num_states} states",
            action_name.name
        );
    }
}
