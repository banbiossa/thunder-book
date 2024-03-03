use crate::base::game_result;
use crate::ch05::maze_state;

pub fn play_game(
    params: maze_state::MazeParams,
    action_funcs: &mut Vec<Box<maze_state::ActionFunc>>,
    seed: u64,
    print: bool,
    play_black: bool,
) -> game_result::GameResult {
    let mut state = maze_state::AlternateMazeState::new(seed, params);
    if print {
        println!("{}", state.to_string());
    }

    if play_black {
        action_funcs.swap(0, 1);
    }

    let mut player = 0;
    while !state.is_done() {
        if print {
            println!("p{player} --------------------------");
        }
        let action = action_funcs[player](&state);
        state.advance(action);
        if print {
            println!("{}", state.to_string());
        }
        player ^= 1;
    }

    let result = state.white_score();

    if print {
        println!("{}", result.display());
    }

    result
}

fn average(
    params: maze_state::MazeParams,
    action_funcs: &mut Vec<Box<maze_state::ActionFunc>>,
    num_games: usize,
    print_every: usize,
    play_black: bool,
) -> f32 {
    let mut total = 0.0;
    for i in 0..num_games {
        let result = play_game(
            params.clone(),
            action_funcs,
            i as u64,
            false,
            play_black,
        );
        total += result.score;
        if print_every > 0 && i % print_every == 0 {
            println!("i {i} v {}", total / (i + 1) as f32);
        }
    }

    total / num_games as f32
}

pub fn play_black_white(
    params: maze_state::MazeParams,
    action_funcs: &mut Vec<Box<maze_state::ActionFunc>>,
    num_games: usize,
    print_every: usize,
) -> f32 {
    let mut total =
        average(params.clone(), action_funcs, num_games, print_every, false);
    total +=
        average(params.clone(), action_funcs, num_games, print_every, true);
    total / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ch05::maze_state;
    use crate::ch05::mini_max;
    use crate::ch05::random_action;

    #[test]
    fn test_average_ch05() {
        let params = maze_state::MazeParams {
            height: 3,
            width: 3,
            end_turn: 3,
        };
        {
            let mut action_funcs = vec![
                random_action::random_action_factory(),
                mini_max::mini_max_action_factory(3),
            ];
            let result =
                average(params.clone(), &mut action_funcs, 100, 10, false);
            println!("{:?}", result);
            // assert_eq!(result, 0.5);
        }
        {
            let mut action_funcs = vec![
                random_action::random_action_factory(),
                random_action::random_action_factory(),
            ];
            let result =
                average(params.clone(), &mut action_funcs, 100, 10, true);
            println!("{:?}", result);
            // assert_eq!(result, 0.5);
        }
    }

    #[test]
    fn test_play_game() {
        let params = maze_state::MazeParams {
            height: 3,
            width: 3,
            end_turn: 3,
        };
        let mut action_funcs = vec![
            random_action::random_action_factory(),
            random_action::random_action_factory(),
        ];
        let result =
            play_game(params.clone(), &mut action_funcs, 0, true, false);
        println!("{:?}", result);
        // result is random
        // assert_eq!(result.score, 1.0);
        // assert_eq!(result.points, 7);
    }

    #[test]
    fn test_mini_max_vs_random() {
        let params = maze_state::MazeParams {
            height: 3,
            width: 3,
            end_turn: 3,
        };
        {
            let mut action_funcs = vec![
                mini_max::mini_max_action_factory(3),
                random_action::random_action_factory(),
            ];
            let result =
                play_game(params.clone(), &mut action_funcs, 0, true, true);
            println!("{:?}", result);
            // result is random
            // assert_eq!(result.score, 1.0);
            // assert_eq!(result.points, 7);
        }

        {
            let mut action_funcs = vec![
                random_action::random_action_factory(),
                mini_max::mini_max_action_factory(3),
            ];
            let result = play_game(params, &mut action_funcs, 0, true, false);
            println!("{:?}", result);
            // result is random
            // assert_eq!(result.score, 1.0);
            // assert_eq!(result.points, 7);
        }
    }
}
