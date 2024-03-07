use std::sync::Arc;

use crate::ch06::maze_state;
use crate::ch06::random_action;

pub struct Playout {
    state: maze_state::SimultaneousMazeState,
}

impl Playout {
    pub fn new(state: &maze_state::SimultaneousMazeState) -> Self {
        Playout {
            state: state.clone(),
        }
    }

    pub fn playout(&mut self) -> f32 {
        if self.state.is_done() {
            return self.state.white_score().score;
        }

        let actions = vec![
            random_action::random_action_arc()(&self.state, 0),
            random_action::random_action_arc()(&self.state, 1),
        ];
        self.state.advance(actions);
        1.0 - self.playout()
    }
}

struct ActionScore {
    action: usize,
    score: f32,
}

fn monte_carlo(
    initial_state: &maze_state::SimultaneousMazeState,
    player_id: usize,
    num_playout: usize,
) -> usize {
    let opp_player_id = player_id ^ 1;
    let legal_actions = initial_state.legal_actions(player_id);

    let mut action_scores: Vec<ActionScore> = Vec::new();

    for action in legal_actions {
        let mut score = 0.0;
        for _ in 0..num_playout {
            let mut actions = vec![0, 0];
            actions[player_id] = action;
            actions[opp_player_id] =
                random_action::random_action(&initial_state, opp_player_id);
            let mut state = initial_state.clone();
            state.advance(actions);
            score += Playout::new(&state).playout();
        }
        // reverse score for player 1
        if player_id == 1 {
            score = 1. - score;
        }
        action_scores.push(ActionScore { action, score });
    }

    action_scores
        .iter()
        .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
        .unwrap()
        .action
}

pub fn monte_carlo_arc(num_playout: usize) -> maze_state::ActionFunc {
    Arc::new(move |state, player_id| -> usize {
        monte_carlo(state, player_id, num_playout)
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    fn setup() -> maze_state::SimultaneousMazeState {
        let params = maze_state::MazeParams {
            height: 3,
            width: 3,
            end_turn: 4,
        };
        maze_state::SimultaneousMazeState::new(0, params)
    }

    #[test]
    fn test_monte_carlo_arc() {
        let state = setup();
        let actual = monte_carlo_arc(100)(&state, 0);
        assert!(actual <= 3);
    }

    #[test]
    fn test_monte_carlo() {
        let state = setup();
        let actual = monte_carlo(&state, 0, 100);
        assert!(actual <= 3);
    }

    #[test]
    fn test_or() {
        let p0: usize = 0;
        let p1 = p0 ^ 1;
        assert_eq!(p1, 1);

        assert_eq!(p1 ^ 1, 0);
    }

    #[test]
    fn test_playout() {
        let state = setup();
        let mut playout = Playout::new(&state);
        let score = playout.playout();
        // check that state isn't mutated
        assert_eq!(state.turn, 0);
        // result is random
        assert!(score <= 1.0);
    }
}
