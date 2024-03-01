use crate::ch04::maze_state;

pub fn play_game(
    action_func: Box<maze_state::ActionFunc>,
    seed: u64,
    params: maze_state::MazeParams,
    print: bool,
) -> usize {
    let state = maze_state::AutoMoveMazeState::new(seed, params);
    let state = action_func(&state);
    if print {
        println!("{}", state.to_string());
    }
    let score = state.get_score(print);
    if print {
        println!("score is {score}");
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ch04::random_action;

    #[test]
    #[ignore = "until play_game works"]
    fn can_play_game() {
        let params = maze_state::MazeParams {
            height: 3,
            width: 3,
            end_turn: 10,
            num_characters: 2,
        };
        let score =
            play_game(Box::new(random_action::random_action), 0, params, true);

        assert!(score > 0);
    }
}
