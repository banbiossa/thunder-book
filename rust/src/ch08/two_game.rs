use crate::base::alternate::{ActionFunc, AlternateState, MazeParams};

pub fn play_game<T, W>(
    params: MazeParams,
    action_funcs: (ActionFunc<T>, ActionFunc<W>),
    print: bool,
) -> f32
where
    T: AlternateState,
    W: AlternateState,
    // actually ActionFunc<W>, but we can't use traits here
{
    let mut states = (T::new(0, params.clone()), W::new(0, params.clone()));
    let mut player = 0;

    if print {
        println!("{}", states.0.to_string());
    }

    while !states.0.is_done() {
        let action = match player {
            0 => action_funcs.0(&states.0),
            _ => action_funcs.1(&states.1),
        };

        states.0.advance(action);
        states.1.advance(action);

        player ^= 1;

        if print {
            println!("{}", states.0.to_string());
        }
    }

    states.0.white_score()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ch05::mcts::{mcts_action_arc, MCTSParams};
    use crate::ch08::bitstate::BitsetConnectFour;
    use crate::ch08::maze_state::ConnectFourState;

    #[test]
    fn test_play_game() {
        let params = MazeParams {
            height: 2,
            width: 4,
            end_turn: 0,
        };
        let mcts_params = MCTSParams {
            c: 1.0,
            expand_threshold: 3,
        };
        let action_funcs = (
            mcts_action_arc::<ConnectFourState>(10, mcts_params.clone()),
            mcts_action_arc::<BitsetConnectFour>(10, mcts_params.clone()),
        );
        let actual = play_game(params, action_funcs, true);
        assert!(actual <= 1.0);
        // see print
        // assert!(false);
    }
}
