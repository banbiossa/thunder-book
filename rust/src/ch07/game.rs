use crate::base::state::SinglePlayerState;
use crate::ch07::maze_state;

pub fn play_game(
    seed: u64,
    params: maze_state::MazeParams,
    action_func: maze_state::ActionFunc,
    print: bool,
) -> usize {
    let mut state = maze_state::WallMazeState::new(seed, params);
    if print {
        println!("{}", state.to_string());
    }
    while !state.is_done() {
        state.advance(action_func(&state));
        if print {
            println!("{}", state.to_string());
        }
    }
    state.game_score
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ch07::random_action;

    #[test]
    fn test_game() {
        let params = maze_state::MazeParams {
            height: 5,
            width: 5,
            end_turn: 3,
        };
        play_game(0, params, random_action::random_action_arc(), true);
        // panic!("panic to look at game")
    }
}
