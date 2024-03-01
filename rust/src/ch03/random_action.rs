use crate::ch03::maze_state;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn random_action(state: &maze_state::NumberCollectingGame) -> usize {
    let mut rng = thread_rng();

    let legal_actions = state.legal_actions();
    let choice = legal_actions.choose(&mut rng).unwrap();

    choice.clone()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_random_action() {
        let params = maze_state::MazeParams {
            height: 3,
            width: 4,
            end_turn: 3,
        };
        let state = maze_state::NumberCollectingGame::new(0, params);
        let action = random_action(&state);
        let legal_actions = state.legal_actions();
        assert!(legal_actions.contains(&action));
    }
}
