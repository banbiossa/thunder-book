use search::ch04::game;
use search::ch04::maze_state;
use search::ch04::random_action;

fn main() {
    pub const PARAMS: maze_state::MazeParams = maze_state::MazeParams {
        height: 4,
        width: 4,
        num_characters: 3,
        end_turn: 4,
    };

    game::play_game(PARAMS, Box::new(random_action::random_action), 0, true);
}
