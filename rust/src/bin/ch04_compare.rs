use search::ch04::game;
use search::ch04::hill_climb;
use search::ch04::maze_state;
use search::ch04::simulated_annealing;

struct ActionNamePair {
    action_func: Box<maze_state::ActionFunc>,
    name: String,
}
fn main() {
    pub const PARAMS: maze_state::MazeParams = maze_state::MazeParams {
        height: 5,
        width: 5,
        end_turn: 4,
        num_characters: 5,
    };
    let num_games = 100;
    let print_every = num_games / 10;
    let num_iter = 10000;
    let seed = 0;
    let start_temp = 100.0;
    let end_temp = 0.0;

    let action_funcs = vec![
        ActionNamePair {
            action_func: hill_climb::hill_climb_factory(num_iter, seed),
            name: format!("hill climb"),
        },
        ActionNamePair {
            action_func: simulated_annealing::simulated_annealing_factory(
                num_iter, start_temp, end_temp, seed,
            ),
            name: format!("simulated annealing {start_temp} to {end_temp}"),
        },
    ];

    for pair in action_funcs.into_iter().rev() {
        println!("do {}", pair.name);
        let average_score =
            game::average(PARAMS, &pair.action_func, num_games, print_every);
        println!("average of {} is {average_score}", pair.name);
    }
}
