use std::sync::Arc;

use crate::ch05::maze_state;

#[derive(Debug)]
struct Node {
    state: maze_state::AlternateMazeState,
    w: f32,
    n: usize,
    child_nodes: Vec<Node>,
}

impl Node {
    fn new(state: &maze_state::AlternateMazeState) -> Self {
        let child_nodes: Vec<Node> = Vec::new();
        Node {
            state: state.clone(),
            child_nodes,
            w: 0.0,
            n: 0,
        }
    }

    fn increment(&mut self, value: f32) {
        self.w += value;
        self.n += 1;
    }

    fn expand(&mut self) {
        let legal_actions = self.state.legal_actions();
        self.child_nodes.clear();
        for action in legal_actions {
            let mut node = Node::new(&self.state);
            node.state.advance(action);
            self.child_nodes.push(node);
        }
    }

    fn thunder_value(&self) -> f32 {
        // win rate, but from the parent perspective
        1. - self.w / (self.n as f32)
    }

    fn win_rate(&self) -> f32 {
        // simple win rate to use in search
        self.w / (self.n as f32)
    }

    fn next_child_node(&mut self) -> &mut Node {
        assert!(!self.child_nodes.is_empty());
        // find n==0
        if let Some((index, _)) = self
            .child_nodes
            .iter()
            .enumerate()
            .find(|(_, child_node)| child_node.n == 0)
        {
            return &mut self.child_nodes[index];
        }

        self.child_nodes
            .iter_mut()
            .max_by(|a, b| {
                a.thunder_value().partial_cmp(&b.thunder_value()).unwrap()
            })
            .unwrap()
    }

    fn explore(&mut self) -> f32 {
        if self.state.is_done() {
            let value = self.state.teban_score().score;
            self.increment(value);
            return value;
        }
        if self.child_nodes.is_empty() {
            let value = self.state.evaluation_rate();
            self.increment(value);
            self.expand();
            return value;
        }
        // has child nodes
        let value = 1. - self.next_child_node().explore();
        self.increment(value);
        value
    }

    // must pass in "" as initial string
    fn print(&self, depth: usize) -> String {
        let mut ss = String::from("");
        for i in 0..self.child_nodes.len() {
            let child_node = &self.child_nodes[i];
            for _ in 0..depth {
                ss += "__ ";
            }
            ss += &format!("{i}({})\n", child_node.n);
            if !child_node.child_nodes.is_empty() {
                ss += &child_node.print(depth + 1);
            }
        }
        ss
    }
}

fn thunder_search(
    state: &maze_state::AlternateMazeState,
    num_playout: usize,
    print: bool,
) -> usize {
    let mut node = Node::new(state);
    node.expand();
    for _ in 0..num_playout {
        node.explore();
    }

    if print {
        node.print(0);
    }

    // break into 2 parts for easier debugging
    let action_scores: Vec<(usize, &Node)> = state
        .legal_actions()
        .into_iter()
        .zip(node.child_nodes.iter())
        .collect();

    action_scores
        .iter()
        .max_by(|(_, a), (_, b)| {
            a.win_rate().partial_cmp(&b.win_rate()).unwrap()
        })
        .unwrap()
        .0
        .to_owned()
}

pub fn thunder_search_arc(num_playout: usize) -> Arc<maze_state::ActionFunc> {
    Arc::new(move |state| -> usize {
        thunder_search(state, num_playout, false)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Node {
        let params = maze_state::MazeParams {
            height: 3,
            width: 3,
            end_turn: 3,
        };
        let state = maze_state::AlternateMazeState::new(0, params);
        Node::new(&state)
    }

    #[test]
    fn test_print() {
        let mut node = setup();
        let actual = node.print(0);
        let expected = "";
        assert_eq!(actual, expected);

        node.explore();
        let actual = node.print(0);
        let expected = "\
0(0)
1(0)
2(0)
";
        assert_eq!(actual, expected);

        node.explore();
        let actual = node.print(0);
        let expected = "\
0(1)
__ 0(0)
__ 1(0)
__ 2(0)
1(0)
2(0)
";
        assert_eq!(actual, expected);

        node.explore();
        node.explore();
        node.explore();
        let actual = node.print(0);
        let expected = "\
0(1)
__ 0(0)
__ 1(0)
__ 2(0)
1(1)
__ 0(0)
__ 1(0)
__ 2(0)
2(2)
__ 0(1)
__ __ 0(0)
__ __ 1(0)
__ 1(0)
__ 2(0)
";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_next_child_node_value() {
        let mut node = setup();
        node.explore();
        node.explore();
        node.explore();
        node.explore();
        let child = node.next_child_node();
        assert!(child.n > 0);
        // assert_eq!(node.w, 0.0);
    }

    #[test]
    fn test_explore() {
        let mut node = setup();
        node.explore();
        assert_eq!(node.n, 1);
        // result is random
        assert!(node.w <= 1.0);
    }

    #[test]
    fn test_next_child_node_zero() {
        let mut node = setup();
        node.expand();
        let child = node.next_child_node();
        assert_eq!(child.n, 0);
    }

    #[test]
    fn test_expand() {
        let mut node = setup();
        assert!(node.child_nodes.is_empty());
        node.expand();
        assert!(!node.child_nodes.is_empty());
    }

    #[test]
    fn test_thunder_value() {
        let mut node = setup();
        node.increment(1.0);
        assert_eq!(node.thunder_value(), 0.0);
        assert_eq!(node.win_rate(), 1.0);
    }

    #[test]
    fn test_increment() {
        let mut node = setup();
        assert_eq!(node.n, 0);
        node.increment(1.0);
        assert_eq!(node.n, 1);
        assert_eq!(node.w, 1.0);
    }

    #[test]
    fn test_make_node() {
        let node = setup();
        assert_eq!(node.n, 0);
        assert_eq!(node.w, 0.0);
    }
}
