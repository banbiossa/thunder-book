use crate::base::game_result;
use crate::ch05::maze_state;

pub fn play_game(
    params: maze_state::MazeParams,
    action_funcs: Vec<Box<maze_state::ActionFunc>>,
    seed: u64,
    print: bool,
) -> game_result::GameResult {
    let mut state = maze_state::AlternateMazeState::new(seed, params);
    if print {
        println!("{}", state.to_string());
    }

    let mut player = 0;
    while !state.is_done() {
        if print {
            println!("p{player} --------------------------");
            let action = action_funcs[player](&state);
            state.advance(action);
            if print {
                println!("{}", state.to_string());
            }
        }
        player ^= 1;
    }

    let result = state.white_score();

    if print {
        println!("{}", result.display());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ch05::maze_state;
    use crate::ch05::random_action;

    #[test]
    fn test_play_game() {
        let params = maze_state::MazeParams {
            height: 3,
            width: 3,
            end_turn: 3,
        };
        let action_funcs = vec![
            random_action::random_action_factory(),
            random_action::random_action_factory(),
        ];
        let result = play_game(params, action_funcs, 0, true);
        println!("{:?}", result);
        // result is random
        // assert_eq!(result.score, 1.0);
        // assert_eq!(result.points, 7);
    }
}
