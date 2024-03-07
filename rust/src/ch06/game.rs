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
) {
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
    fn test_play_game() {
        let params = setup();
        let action_funcs = vec![
            random_action::random_action_arc(),
            random_action::random_action_arc(),
        ];
        play_game(params, action_funcs, 0, true);
        // look at printed result
        // assert!(false);
    }

    #[test]
    fn test_action_to_str() {
        let actual = action_to_str(0);
        assert_eq!(actual, "RIGHT");
    }
}
