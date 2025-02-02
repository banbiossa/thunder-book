use std::sync::Arc;

use crate::base::alternate::{ActionFunc, AlternateState, Evaluatable};
use crate::base::is_done;

#[derive(Debug)]
struct Node<T: AlternateState + Evaluatable> {
    state: T,
    w: f32,
    n: usize,
    child_nodes: Vec<Node<T>>,
}

impl<T: AlternateState + Evaluatable> Node<T> {
    fn new(state: &T) -> Self {
        let child_nodes: Vec<Node<T>> = Vec::new();
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

    fn next_child_node(&mut self) -> &mut Node<T> {
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
            let value = self.state.teban_score();
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

fn thunder_search<T: AlternateState + Evaluatable>(
    state: &T,
    mut stop_condition: is_done::Stopper,
    print: bool,
) -> usize {
    let mut node = Node::new(state);
    node.expand();
    while !stop_condition() {
        node.explore();
    }

    if print {
        println!("{}", node.print(0));
    }

    // break into 2 parts for easier debugging
    let action_scores: Vec<(usize, &Node<T>)> = state
        .legal_actions()
        .into_iter()
        .zip(node.child_nodes.iter())
        .collect();

    action_scores
        .iter()
        .max_by_key(|(_, b)| b.n)
        // .max_by(|(_, a), (_, b)| {
        //     a.thunder_value().partial_cmp(&b.thunder_value()).unwrap()
        // })
        .unwrap()
        .0
        .to_owned()
}

pub fn thunder_search_arc<T: AlternateState + Evaluatable>(
    num_playout: usize,
) -> ActionFunc<T> {
    Arc::new(move |state| -> usize {
        let for_loop = is_done::depth_stopper(num_playout);
        thunder_search(state, for_loop, false)
    })
}

pub fn thunder_timebound_arc<T: AlternateState + Evaluatable>(
    time_threshold_ms: u64,
) -> ActionFunc<T> {
    Arc::new(move |state| -> usize {
        let time_stopper = is_done::time_stopper(time_threshold_ms);
        thunder_search(state, time_stopper, false)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::alternate::MazeParams;
    use crate::ch05::maze_state;

    fn setup() -> Node<maze_state::AlternateMazeState> {
        let params = MazeParams {
            height: 3,
            width: 3,
            end_turn: 3,
        };
        let state = maze_state::AlternateMazeState::new(0, params);
        Node::new(&state)
    }

    #[test]
    fn test_thunder_timebound() {
        let params = MazeParams {
            height: 3,
            width: 3,
            end_turn: 3,
        };
        let state = maze_state::AlternateMazeState::new(0, params);
        let actual = thunder_timebound_arc(1)(&state);
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_thunder_arc() {
        let params = MazeParams {
            height: 3,
            width: 3,
            end_turn: 3,
        };
        let state = maze_state::AlternateMazeState::new(0, params);
        let actual = thunder_search_arc(1000)(&state);
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_thunder_search_deep() {
        let params = MazeParams {
            height: 3,
            width: 3,
            end_turn: 6,
        };
        let state = maze_state::AlternateMazeState::new(0, params);
        let for_loop = is_done::depth_stopper(100);
        let actual = thunder_search(&state, for_loop, true);
        // to see the print
        // assert_eq!(actual, 4);
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_thunder_search() {
        let params = MazeParams {
            height: 3,
            width: 3,
            end_turn: 6,
        };
        let state = maze_state::AlternateMazeState::new(0, params);
        let for_loop = is_done::depth_stopper(100);
        let actual = thunder_search(&state, for_loop, true);
        // to see the print
        // assert_eq!(actual, 4);
        assert_eq!(actual, 0);
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
