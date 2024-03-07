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
