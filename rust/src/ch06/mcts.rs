use std::sync::Arc;

use crate::ch06::maze_state;
use crate::ch06::monte_carlo;

#[derive(Debug, Clone)]
pub struct MCTSParams {
    c: f32,
    expand_threshold: usize,
}

// a base node
// even node and odd node will be made
// from this
struct Node {
    state: maze_state::SimultaneousMazeState,
    w: f32,
    n: usize,
    child_nodes: Vec<Node>,
    params: MCTSParams,
    player: usize,
    action0: Option<usize>,
}

impl Node {
    pub fn new(
        state: &maze_state::SimultaneousMazeState,
        params: MCTSParams,
        player: usize,
        action0: Option<usize>,
    ) -> Self {
        let child_nodes = Vec::new();
        Node {
            state: state.clone(),
            child_nodes,
            w: 0.0,
            n: 0,
            params,
            player,
            action0,
        }
    }

    fn increment(&mut self, value: f32) {
        self.w += value;
        self.n += 1;
    }

    fn ucb1(&self, t: f32) -> f32 {
        1.0 - self.w / (self.n as f32)
            + self.params.c * (2.0 * t.ln() / (self.n as f32)).sqrt()
    }

    fn t(&self) -> f32 {
        self.child_nodes.iter().map(|p| p.n).sum::<usize>() as f32
    }

    fn next_child_node(&mut self) -> &mut Node {
        // find n == 0
        if let Some((index, _)) = self
            .child_nodes
            .iter()
            .enumerate()
            .find(|(_, child_node)| child_node.n == 0)
        {
            return &mut self.child_nodes[index];
        }

        let t = self.t();
        let best = self
            .child_nodes
            .iter_mut()
            .max_by(|a, b| a.ucb1(t).partial_cmp(&b.ucb1(t)).unwrap())
            .unwrap();
        best
    }

    fn expand_even(&mut self) {
        let legal_actions = self.state.legal_actions(0);
        self.child_nodes.clear();
        for action0 in legal_actions {
            self.child_nodes.push(Node::new(
                &self.state,
                self.params.clone(),
                1,
                Some(action0),
            ));
        }
    }

    fn expand_odd(&mut self) {
        let legal_actions = self.state.legal_actions(1);
        self.child_nodes.clear();
        for action1 in legal_actions {
            let mut state = self.state.clone();
            state.advance(vec![self.action0.unwrap(), action1]);
            self.child_nodes.push(Node::new(
                &state,
                self.params.clone(),
                0,
                None,
            ));
        }
    }

    pub fn explore_even(&mut self) -> f32 {
        // always expand even nodes
        if self.child_nodes.is_empty() {
            self.expand_even();
        }

        let value = 1.0 - self.next_child_node().explore_odd();
        self.increment(value);
        value
    }

    fn explore_odd(&mut self) -> f32 {
        if self.state.is_done() {
            let value = 1.0 - self.state.white_score().score;
            self.increment(value);
            return value;
        }

        if !self.child_nodes.is_empty() {
            let value = 1.0 - self.next_child_node().explore_even();
            self.increment(value);
            return value;
        }
        // no children, return playout
        let value = 1.0 - monte_carlo::Playout::new(&self.state).playout();
        if self.n >= self.params.expand_threshold {
            self.expand_odd();
        }
        self.increment(value);
        value
    }
}

fn mcts(
    state: &maze_state::SimultaneousMazeState,
    params: MCTSParams,
    player_id: usize,
    num_playout: usize,
) -> usize {
    assert!(player_id == 0, "only works for player0");
    let mut node = Node::new(state, params.clone(), 0, None);
    for _ in 0..num_playout {
        node.expand_even();
    }

    let legal_actions = state.legal_actions(0);
    // argmax of child.n
    let best = legal_actions
        .iter()
        .zip(node.child_nodes.iter())
        .map(|(action, child_node)| (action, child_node.n))
        .max_by_key(|(_, n)| n.to_owned())
        .unwrap()
        .0
        .to_owned();
    best
}

pub fn mcts_arc(
    params: MCTSParams,
    num_playout: usize,
) -> maze_state::ActionFunc {
    Arc::new(move |state, player_id| -> usize {
        mcts(state, params.clone(), player_id, num_playout)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Node {
        let state_params = maze_state::MazeParams {
            height: 3,
            width: 3,
            end_turn: 4,
        };
        let state = maze_state::SimultaneousMazeState::new(0, state_params);
        let mcts_params = MCTSParams {
            c: 1.0,
            expand_threshold: 3,
        };
        Node::new(&state, mcts_params, 0, None)
    }

    #[test]
    fn test_mcts_arc() {
        let state_params = maze_state::MazeParams {
            height: 3,
            width: 3,
            end_turn: 4,
        };
        let state = maze_state::SimultaneousMazeState::new(0, state_params);
        let mcts_params = MCTSParams {
            c: 1.0,
            expand_threshold: 3,
        };

        let actual = mcts_arc(mcts_params, 3000)(&state, 0);
        assert_eq!(actual, 3);
    }

    #[test]
    fn test_mcts() {
        let state_params = maze_state::MazeParams {
            height: 3,
            width: 3,
            end_turn: 4,
        };
        let state = maze_state::SimultaneousMazeState::new(0, state_params);
        let mcts_params = MCTSParams {
            c: 1.0,
            expand_threshold: 3,
        };

        let actual = mcts(&state, mcts_params, 0, 3000);
        assert_eq!(actual, 3);
    }

    #[test]
    fn test_explore() {
        let mut node = setup();
        let actual = node.explore_even();
        assert!(actual <= 1.0);
    }

    #[test]
    fn test_expand() {
        let mut node = setup();
        assert!(node.child_nodes.is_empty());
        node.expand_even();
        assert!(!node.child_nodes.is_empty());
        assert_eq!(node.child_nodes[0].player, 1);

        assert!(node.child_nodes[0].child_nodes.is_empty());
        node.child_nodes[0].expand_odd();
        assert!(!node.child_nodes[0].child_nodes.is_empty());
    }

    #[test]
    fn test_t() {
        let node = setup();
        assert_eq!(node.t(), 0.0);
    }

    #[test]
    fn test_increment() {
        let mut node = setup();
        node.increment(0.3);
        assert_eq!(node.n, 1);
        assert_eq!(node.w, 0.3);
    }

    #[test]
    fn test_make_node() {
        let node = setup();
        assert_eq!(node.n, 0);
        assert_eq!(node.player, 0);
    }
}
