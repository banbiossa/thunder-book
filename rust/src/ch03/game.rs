use crate::base::state::{self, SinglePlayerState};
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

/// play 1 game and return the score
pub fn play_game<T: SinglePlayerState>(
    params: state::MazeParams,
    action_func: state::ActionFunc<T>,
    seed: u64,
    print: bool,
) -> usize {
    let mut state = T::new(seed, params);
    if print {
        println!("{}", state.to_string());
    }

    while !state.is_done() {
        state.advance(action_func(&state));
        if print {
            println!("{}", state.to_string());
        }
    }

    state.get_game_score()
}

// take an average score on num_games
pub fn average<T: SinglePlayerState>(
    params: state::MazeParams,
    action_func: state::ActionFunc<T>,
    num_games: usize,
    print_every: usize,
) -> f64 {
    //
    let mut total_score = 0;
    let mut rng = StdRng::seed_from_u64(0);
    for i in 0..num_games {
        let mut state = T::new(rng.gen(), params.clone());
        while !state.is_done() {
            state.advance(action_func(&state))
        }
        total_score += state.get_game_score();

        if print_every > 0 && i % print_every == 0 {
            println!(
                "i: {i} score: {:.2}",
                total_score as f64 / (i + 1) as f64
            );
        }
    }
    total_score as f64 / num_games as f64
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ch03::{maze_state::NumberCollectingGame, random_action};

    // create params as a fixture
    fn setup() -> state::MazeParams {
        state::MazeParams {
            height: 3,
            width: 4,
            end_turn: 3,
        }
    }

    #[test]
    fn test_play_game() {
        let params = setup();
        let action_func: state::ActionFunc<NumberCollectingGame> =
            random_action::random_action_box();
        let score = play_game(params, action_func, 0, true);
        assert!(score > 0);
    }

    #[test]
    fn test_average() {
        let params = setup();
        let action_func: state::ActionFunc<NumberCollectingGame> =
            random_action::random_action_box();
        let score = average(params, action_func, 3, 4);
        assert!(score > 0.0);
    }
}
