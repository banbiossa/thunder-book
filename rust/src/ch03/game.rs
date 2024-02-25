use crate::ch03::maze_state;
use crate::ch03::random_action;

pub fn play_game(seed: u8) {
    let mut state = maze_state::NumberCollectingGame::new(seed);
    println!("{}", state.to_string());

    while !state.is_done() {
        state.advance(random_action::random_action(&state));
        println!("{}", state.to_string());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_play_game() {
        play_game(0);
    }
}
