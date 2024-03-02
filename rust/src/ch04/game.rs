use crate::ch04::maze_state;

pub fn play_game(
    params: maze_state::MazeParams,
    action_func: &Box<maze_state::ActionFunc>,
    seed: u64,
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

pub fn play_many(
    params: maze_state::MazeParams,
    action_func: &Box<maze_state::ActionFunc>,
    num_games: usize,
    print_every: usize,
) -> f32 {
    let mut total = 0;
    for i in 0..num_games {
        total += play_game(params.clone(), &action_func, i as u64, false);

        if print_every > 0 && i % print_every == 0 {
            println!("i:{i} v: {}", total as f32 / (i + 1) as f32);
        }
    }

    total as f32 / num_games as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ch04::random_action;

    fn setup() -> maze_state::MazeParams {
        maze_state::MazeParams {
            height: 3,
            width: 3,
            end_turn: 10,
            num_characters: 2,
        }
    }

    #[test]
    fn test_play_many() {
        let params = setup();
        let average =
            play_many(params, &random_action::random_action_factory(), 10, 1);
        assert!(average > 0.0);
    }

    #[test]
    fn can_play_game() {
        let params = setup();
        let score =
            play_game(params, &random_action::random_action_factory(), 0, true);

        assert!(score > 0);
    }
}
