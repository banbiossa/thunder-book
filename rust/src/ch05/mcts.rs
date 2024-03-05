use std::sync::Arc;

use crate::ch05::maze_state;
use crate::ch05::monte_carlo;

#[derive(Debug, Clone)]
pub struct MCTSParams {
    c: f32,
    expand_threshold: usize,
}

#[derive(Debug)]
struct Node {
    state: maze_state::AlternateMazeState,
    w: f32,
    n: usize,
    child_nodes: Vec<Node>,
    params: MCTSParams,
}

impl Node {
    fn new(state: &maze_state::AlternateMazeState, params: MCTSParams) -> Self {
        let child_nodes: Vec<Node> = Vec::new();
        Node {
            state: state.clone(),
            w: 0.0,
            n: 0,
            child_nodes,
            params,
        }
    }

    fn expand(&mut self) {
        let legal_actions = self.state.legal_actions();
        self.child_nodes.clear();
        for action in legal_actions {
            let mut node = Node::new(&self.state, self.params.clone());
            node.state.advance(action);
            self.child_nodes.push(node);
        }
    }

    fn ucb1(&self, t: f32) -> f32 {
        // will be called by parent so do 1.0 -
        1.0 - self.w / self.n as f32
            + self.params.c * (2.0 * t.ln() / self.n as f32).sqrt()
    }

    fn t(&self) -> f32 {
        self.child_nodes.iter().map(|p| p.n).sum::<usize>() as f32
    }

    fn next_child_node(&mut self) -> &mut Node {
        assert!(!self.child_nodes.is_empty());
        // find if n == 0
        if let Some((index, _)) = self
            .child_nodes
            .iter()
            .enumerate()
            .find(|(_, child_node)| child_node.n == 0)
        {
            return &mut self.child_nodes[index];
        }

        // compare ucb1 value and get largest
        let t = self.t();
        self.child_nodes
            .iter_mut()
            .max_by(|a, b| a.ucb1(t).partial_cmp(&b.ucb1(t)).unwrap())
            .unwrap()
    }

    fn explore(&mut self) -> f32 {
        if self.state.is_done() {
            let value = self.state.teban_score().score;
            self.increment(value);
            return value;
        }

        if self.child_nodes.is_empty() {
            let value = monte_carlo::Playout::new(&self.state).playout();
            self.increment(value);
            if self.n == self.params.expand_threshold {
                self.expand();
            }
            return value;
        }

        let value = 1.0 - self.next_child_node().explore();
        self.increment(value);
        value
    }

    fn increment(&mut self, value: f32) {
        self.w += value;
        self.n += 1;
    }
}

fn mcts_action(
    state: &maze_state::AlternateMazeState,
    num_playout: usize,
    params: MCTSParams,
) -> usize {
    let mut node = Node::new(state, params);
    node.expand();
    for _ in 0..num_playout {
        node.explore();
    }

    // break into 2 parts for easier debugging
    let action_scores: Vec<(usize, &Node)> = state
        .legal_actions()
        .into_iter()
        .zip(node.child_nodes.iter())
        .collect();

    action_scores
        .iter()
        .max_by_key(|(_, child_node)| child_node.n)
        .unwrap()
        .0
        .to_owned()
}

pub fn mcts_action_arc(
    num_playout: usize,
    params: MCTSParams,
) -> Arc<maze_state::ActionFunc> {
    Arc::new(move |state| -> usize {
        mcts_action(state, num_playout, params.clone())
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    fn setup() -> Node {
        let maze_params = maze_state::MazeParams {
            height: 3,
            width: 3,
            end_turn: 3,
        };

        let state = maze_state::AlternateMazeState::new(0, maze_params);
        let mcts_params = MCTSParams {
            c: 1.0,
            expand_threshold: 3,
        };
        Node::new(&state, mcts_params)
    }

    #[test]
    fn test_mcts_action_arc() {
        let maze_params = maze_state::MazeParams {
            height: 3,
            width: 3,
            end_turn: 3,
        };

        let state = maze_state::AlternateMazeState::new(0, maze_params);
        let mcts_params = MCTSParams {
            c: 1.0,
            expand_threshold: 3,
        };
        let actual = mcts_action_arc(100, mcts_params)(&state);
        let expected = 3;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_mcts_action() {
        let maze_params = maze_state::MazeParams {
            height: 3,
            width: 3,
            end_turn: 3,
        };

        let state = maze_state::AlternateMazeState::new(0, maze_params);
        let mcts_params = MCTSParams {
            c: 1.0,
            expand_threshold: 3,
        };
        let actual = mcts_action(&state, 100, mcts_params);
        let expected = 3;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_ucb1() {
        let mut node = setup();
        node.explore();
        let actual = node.ucb1(1.0);
        // result is random
        assert!(actual <= 1.0);
    }

    #[test]
    fn test_t() {
        let mut node = setup();
        assert_eq!(node.t(), 0.0);
        for _ in 0..4 {
            node.explore();
        }
        let actual = node.t();
        assert_eq!(actual, 1.0);
    }

    #[test]
    fn test_next_child_node() {
        let mut node = setup();
        node.expand();
        let child = node.next_child_node();
        assert_eq!(child.n, 0);
    }

    #[test]
    fn test_explore_expands() {
        let mut node = setup();
        node.explore();
        node.explore();
        assert!(node.child_nodes.is_empty());
        node.explore();
        assert!(!node.child_nodes.is_empty());
    }

    #[test]
    fn test_expand() {
        let mut node = setup();
        assert!(node.child_nodes.is_empty());
        node.expand();
        assert!(!node.child_nodes.is_empty());
        assert!(node.child_nodes[0].child_nodes.is_empty());
    }

    #[test]
    fn test_explore() {
        let mut node = setup();
        node.explore();
        assert_eq!(node.n, 1);
        // result is random
        // assert_eq!(node.w, 0.0);
    }

    #[test]
    fn test_increment() {
        let mut node = setup();
        node.increment(0.5);
        assert_eq!(node.w, 0.5);
        assert_eq!(node.n, 1);
    }

    #[test]
    fn make_empty_node() {
        let node = setup();
        assert!(node.child_nodes.is_empty());
    }
}
