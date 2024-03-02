use search::ch04::game;
use search::ch04::hill_climb;
use search::ch04::maze_state;
use search::ch04::random_action;

fn main() {
    pub const PARAMS: maze_state::MazeParams = maze_state::MazeParams {
        height: 4,
        width: 4,
        num_characters: 3,
        end_turn: 4,
    };

    println!("play random");
    game::play_game(PARAMS, Box::new(random_action::random_action), 0, true);

    println!("play hillclimb");
    game::play_game(PARAMS, hill_climb::hill_climb_factory(100, 0), 0, true);
}
