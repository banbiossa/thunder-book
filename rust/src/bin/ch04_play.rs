use search::ch04::game;
use search::ch04::hill_climb;
use search::ch04::maze_state;
use search::ch04::random_action;
use search::ch04::simulated_annealing;

fn main() {
    pub const PARAMS: maze_state::MazeParams = maze_state::MazeParams {
        height: 5,
        width: 5,
        num_characters: 5,
        end_turn: 4,
    };

    println!("play random");
    game::play_game(PARAMS, &random_action::random_action_factory(), 0, true);

    println!("play hillclimb");
    game::play_game(PARAMS, &hill_climb::hill_climb_factory(100, 0), 0, true);

    println!("play simulated annealing");
    game::play_game(
        PARAMS,
        &simulated_annealing::simulated_annealing_factory(100, 500.0, 10.0, 0),
        0,
        true,
    );
}
