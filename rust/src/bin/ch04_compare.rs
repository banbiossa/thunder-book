use std::time::Instant;

use search::ch04::game;
use search::ch04::hill_climb;
use search::ch04::maze_state;
use search::ch04::random_action;
use search::ch04::simulated_annealing;
use search::log_and_print;

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
            action_func: random_action::random_action_factory(),
            name: "random action".to_string(),
        },
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

    log_and_print!("| name | score | time |");
    log_and_print!("| ---- | ----- | ---- |");
    for pair in action_funcs {
        println!("do {}", pair.name);
        let start = Instant::now();
        let average_score =
            game::average(PARAMS, &pair.action_func, num_games, print_every);
        println!("average of {} is {average_score}", pair.name);
        let elapsed = start.elapsed().as_secs_f64();
        log_and_print!(
            "| {} | {average_score:.2} | {elapsed:.2}s |",
            pair.name
        );
    }
}
