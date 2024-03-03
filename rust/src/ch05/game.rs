use crate::ch05::maze_state;

pub fn play_game(
    params: maze_state::MazeParams,
    action_func: &Box<maze_state::ActionFunc>,
    seed: u64,
    print: bool,
) -> f32 {
    let mut state = maze_state::AlternateMazeState::new(seed, params);
    if print {
        println!("{}", state.to_string());
    }

    let mut player = 0;
    while !state.is_done() {
        if print {
            println!("p{player} --------------------------");
            let action = action_func(&state);
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

    result.score
}
