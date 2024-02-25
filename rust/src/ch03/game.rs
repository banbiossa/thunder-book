use crate::ch03::maze_state;
use crate::ch03::random_action;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

/// play 1 game and return the score
pub fn play_game(
    action_func: maze_state::ActionFunc,
    seed: u64,
    print: bool,
) -> usize {
    let mut state = maze_state::NumberCollectingGame::new(seed);
    if print {
        println!("{}", state.to_string());
    }

    while !state.is_done() {
        state.advance(action_func(&state));
        if print {
            println!("{}", state.to_string());
        }
    }

    state.game_score
}

pub fn play_random(seed: u64) -> usize {
    play_game(random_action::random_action, seed, true)
}

// take an average score on num_games
pub fn average(
    action_func: maze_state::ActionFunc,
    num_games: usize,
    print_every: usize,
) -> f64 {
    //
    let mut total_score = 0;
    let mut rng = StdRng::seed_from_u64(0);
    for i in 0..num_games {
        let mut state = maze_state::NumberCollectingGame::new(rng.gen());
        while !state.is_done() {
            state.advance(action_func(&state))
        }
        total_score += state.game_score;

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

    #[test]
    fn test_play_game() {
        let score = play_game(random_action::random_action, 0, true);
        assert!(score > 0);
    }

    #[test]
    fn test_average() {
        let score = average(random_action::random_action, 3, 4);
        assert!(score > 0.0);
    }
}
