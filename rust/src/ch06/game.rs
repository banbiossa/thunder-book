use crate::base::game_result;
use crate::ch06::maze_state;

fn action_to_str(action: usize) -> String {
    let as_str = match action {
        0 => "RIGHT",
        1 => "LEFT",
        2 => "DOWN",
        3 => "UP",
        _ => "",
    };
    as_str.to_string()
}

pub fn play_game(
    params: maze_state::MazeParams,
    action_funcs: Vec<maze_state::ActionFunc>,
    seed: u64,
    print: bool,
) -> game_result::GameResult {
    let mut state = maze_state::SimultaneousMazeState::new(seed, params);
    if print {
        println!("{}", state.to_string());
    }

    while !state.is_done() {
        let actions =
            vec![action_funcs[0](&state, 0), action_funcs[1](&state, 1)];
        if print {
            println!(
                "actions A: {} B: {}",
                action_to_str(actions[0]),
                action_to_str(actions[1])
            );
        }
        state.advance(actions);
        if print {
            println!("{}", state.to_string());
        }
    }
    // return score or winner
    state.white_score()
}

// the signature is the same so should be able to make in base
pub fn average(
    params: maze_state::MazeParams,
    action_funcs: Vec<maze_state::ActionFunc>,
    num_games: usize,
    print_every: usize,
) -> f32 {
    let mut total = 0.0;
    for i in 0..num_games {
        let result =
            play_game(params.clone(), action_funcs.clone(), i as u64, false);
        total += result.score;
        if print_every > 0 && i % print_every == 0 {
            println!("i {i} v {:.2}", total / (i + 1) as f32);
        }
    }

    total / num_games as f32
}

pub fn play_black_white(
    params: maze_state::MazeParams,
    action_funcs: Vec<maze_state::ActionFunc>,
    num_games: usize,
    print_every: usize,
) -> f32 {
    // reverse order
    let action_funcs_bw: Vec<maze_state::ActionFunc> =
        action_funcs.iter().cloned().rev().collect();
    let mut total =
        average(params.clone(), action_funcs, num_games, print_every);
    total +=
        1.0 - average(params.clone(), action_funcs_bw, num_games, print_every);
    total / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ch06::random_action;

    fn setup() -> maze_state::MazeParams {
        let params = maze_state::MazeParams {
            height: 3,
            width: 3,
            end_turn: 4,
        };
        params
    }

    #[test]
    fn test_black_and_white() {
        let params = setup();
        let action_funcs = vec![
            random_action::random_action_arc(),
            random_action::random_action_arc(),
        ];
        let res = play_black_white(params, action_funcs, 100, 10);
        assert!(res <= 0.7);
        // assert!(false);
    }

    #[test]
    fn test_average() {
        let params = setup();
        let action_funcs = vec![
            random_action::random_action_arc(),
            random_action::random_action_arc(),
        ];
        let res = average(params, action_funcs, 100, 10);
        assert!(res <= 0.7);
    }

    #[test]
    fn test_play_game() {
        let params = setup();
        let action_funcs = vec![
            random_action::random_action_arc(),
            random_action::random_action_arc(),
        ];
        let res = play_game(params, action_funcs, 0, true);
        assert!(res.score <= 1.0);
        // look at printed result
        // assert!(false);
    }

    #[test]
    fn test_action_to_str() {
        let actual = action_to_str(0);
        assert_eq!(actual, "RIGHT");
    }
}
