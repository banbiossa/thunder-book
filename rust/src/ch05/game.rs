use std::time::Instant;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::base::alternate::{AlternateState, MazeParams};
use crate::ch05::maze_state;
use crate::ch05::random_action;

// how fast in milliseconds
pub fn how_fast(
    action_func: maze_state::ActionFunc<maze_state::AlternateMazeState>,
    states: &Vec<maze_state::AlternateMazeState>,
) -> u128 {
    let start = Instant::now();
    for state in states {
        action_func(&state);
    }
    start.elapsed().as_millis()
}

pub fn sample_states(
    num_states: usize,
    seed: u64,
    params: MazeParams,
) -> Vec<maze_state::AlternateMazeState> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut states: Vec<maze_state::AlternateMazeState> = Vec::new();
    let random_action_f = random_action::random_action_arc();

    for i in 0..num_states {
        let mut state =
            maze_state::AlternateMazeState::new(i as u64, params.clone());
        let turn = rng.gen_range(0..params.end_turn);
        for _ in 0..turn {
            state.advance(random_action_f(&state));
        }
        states.push(state);
    }

    states
}

pub fn play_game<T: AlternateState>(
    params: MazeParams,
    action_funcs: Vec<maze_state::ActionFunc<T>>,
    seed: u64,
    print: bool,
) -> f32 {
    let mut state = T::new(seed, params);
    if print {
        println!("{}", state.to_string());
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

    state.white_score()
}

pub fn average<T: AlternateState>(
    params: MazeParams,
    action_funcs: Vec<maze_state::ActionFunc<T>>,
    num_games: usize,
    print_every: usize,
) -> f32 {
    let mut total = 0.0;
    for i in 0..num_games {
        let result =
            play_game(params.clone(), action_funcs.clone(), i as u64, false);
        total += result;
        if print_every > 0 && i % print_every == 0 {
            println!("i {i} v {:.2}", total / (i + 1) as f32);
        }
    }

    total / num_games as f32
}

pub fn play_black_white<T: AlternateState>(
    params: MazeParams,
    action_funcs: Vec<maze_state::ActionFunc<T>>,
    num_games: usize,
    print_every: usize,
) -> f32 {
    // reverse order
    let action_funcs_bw: Vec<maze_state::ActionFunc<T>> =
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
    use crate::ch05::mini_max;
    use crate::ch05::random_action;

    fn setup() -> MazeParams {
        MazeParams {
            height: 3,
            width: 3,
            end_turn: 3,
        }
    }

    #[test]
    fn test_how_fast() {
        let params = setup();
        let states = sample_states(100, 0, params);
        let actual = how_fast(mini_max::mini_max_arc(2), &states);
        // take a few milliseconds
        assert!(actual < 100);
    }

    #[test]
    fn test_sample_states() {
        let params = setup();
        let actual = sample_states(2, 0, params);
        assert_eq!(actual.len(), 2);
        let state0 = &actual[0];
        assert_eq!(state0.turn, 2);
    }

    #[test]
    fn test_black_and_white() {
        let params = setup();
        let action_funcs = vec![
            random_action::random_action_arc::<maze_state::AlternateMazeState>(
            ),
            random_action::random_action_arc(),
        ];
        let actual = play_black_white(params.clone(), action_funcs, 100, 10);
        // is random
        println!("{actual}");
        // assert_eq!(actual, 0.3);
    }

    #[test]
    fn test_average_ch05() {
        let params = setup();
        {
            let action_funcs = vec![
                random_action::random_action_arc(),
                mini_max::mini_max_arc(3),
            ];
            let result = average(params.clone(), action_funcs, 100, 10);
            println!("{:?}", result);
            // assert_eq!(result, 0.5);
        }
        {
            let action_funcs = vec![
                random_action::random_action_arc::<
                    maze_state::AlternateMazeState,
                >(),
                random_action::random_action_arc(),
            ];
            let result = average(params.clone(), action_funcs, 100, 10);
            println!("{:?}", result);
            // assert_eq!(result, 0.5);
        }
    }

    #[test]
    fn test_play_game() {
        let params = setup();
        let action_funcs = vec![
            random_action::random_action_arc::<maze_state::AlternateMazeState>(
            ),
            random_action::random_action_arc(),
        ];
        let result = play_game(params.clone(), action_funcs, 0, true);
        println!("{:?}", result);
        // result is random
        // assert_eq!(result.score, 1.0);
        // assert_eq!(result.points, 7);
    }

    #[test]
    fn test_mini_max_vs_random() {
        let params = setup();
        {
            let action_funcs = vec![
                mini_max::mini_max_arc(3),
                random_action::random_action_arc(),
            ];
            let result = play_game(params.clone(), action_funcs, 0, true);
            println!("{:?}", result);
            // result is random
            // assert_eq!(result.score, 1.0);
            // assert_eq!(result.points, 7);
        }

        {
            let action_funcs = vec![
                random_action::random_action_arc(),
                mini_max::mini_max_arc(3),
            ];
            let result = play_game(params, action_funcs, 0, true);
            println!("{:?}", result);
            // result is random
            // assert_eq!(result.score, 1.0);
            // assert_eq!(result.points, 7);
        }
    }
}
