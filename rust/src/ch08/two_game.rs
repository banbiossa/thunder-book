use crate::base::alternate::{ActionFunc, AlternateState, MazeParams};

fn play_game<T, W>(
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

fn play_many<T, W>(
    params: MazeParams,
    action_funcs: (ActionFunc<T>, ActionFunc<W>),
    num_games: usize,
    print_every: usize,
) -> f32
where
    T: AlternateState,
    W: AlternateState,
{
    let mut total = 0.0;
    for i in 0..num_games {
        total += play_game(params.clone(), action_funcs.clone(), false);
        if print_every > 0 && (i + 1) % print_every == 0 {
            println!("{} {:.2}", i + 1, total / (i + 1) as f32);
        }
    }
    total / num_games as f32
}

pub fn play_black_and_white<T, W>(
    params: MazeParams,
    action_funcs: (ActionFunc<T>, ActionFunc<W>),
    num_games: usize,
    print_every: usize,
) -> f32
where
    T: AlternateState,
    W: AlternateState,
{
    let mut total =
        play_many(params.clone(), action_funcs.clone(), num_games, print_every);
    let actions_bw = (action_funcs.1, action_funcs.0);
    total += 1.0 - play_many(params, actions_bw, num_games, print_every);
    total / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ch05::mcts::{mcts_action_arc, MCTSParams};
    use crate::ch08::bitstate::BitsetConnectFour;
    use crate::ch08::maze_state::ConnectFourState;

    #[test]
    fn test_black_and_white() {
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
        let actual = play_black_and_white(params, action_funcs, 10, 3);
        assert!(actual <= 1.0);
        // assert!(false);
    }

    #[test]
    fn test_play_many() {
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
        let actual = play_many(params, action_funcs, 10, 3);
        assert!(actual <= 1.0);
        // assert!(false);
    }

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
