use std::sync::Arc;

use crate::ch06::maze_state;

use super::monte_carlo;

#[derive(Debug, Clone)]
pub struct DuctParams {
    pub c: f32,
    pub expand_threshold: usize,
}

#[derive(Debug, Clone)]
struct Node {
    n: usize,
    w: f32,
    child_nodeses: Vec<Vec<Node>>,
    state: maze_state::SimultaneousMazeState,
    params: DuctParams,
}

impl Node {
    fn new(
        state: &maze_state::SimultaneousMazeState,
        params: DuctParams,
    ) -> Self {
        let child_nodeses = Vec::new();
        Node {
            w: 0.0,
            n: 0,
            state: state.clone(),
            params,
            child_nodeses,
        }
    }

    fn increment(&mut self, value: f32) {
        self.w += value;
        self.n += 1;
    }

    fn t(&self) -> f32 {
        self.child_nodeses
            .iter()
            .flatten()
            .map(|node| node.n as f32)
            .sum()
    }

    fn ucb1(&self, w: f32, n: f32) -> f32 {
        w / n + self.params.c * (self.t().ln() / n).sqrt()
    }

    fn expand(&mut self) {
        self.child_nodeses.clear();
        let legal_actions0 = self.state.legal_actions(0);
        let legal_actions1 = self.state.legal_actions(1);
        for action0 in legal_actions0 {
            let mut child_nodes = Vec::new();
            for action1 in &legal_actions1 {
                let mut state = self.state.clone();
                state.advance(vec![action0, action1.to_owned()]);
                child_nodes.push(Node::new(&state, self.params.clone()));
            }
            self.child_nodeses.push(child_nodes);
        }
    }

    fn action0(&self) -> usize {
        // iterate i and sum j, than take ucb1
        let row_sums: Vec<f32> = self
            .child_nodeses
            .iter()
            .map(|row| {
                let w = row.iter().map(|node| node.w).sum::<f32>();
                let n = row.iter().map(|node| node.n).sum::<usize>() as f32;
                self.ucb1(w, n)
            })
            .collect();

        // get argmax
        row_sums
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(index, _)| index)
            .unwrap()
    }

    fn action1(&self) -> usize {
        // transpose
        let transposed: Vec<Vec<&Node>> = (0..self.child_nodeses[0].len())
            .map(|i| self.child_nodeses.iter().map(|row| &row[i]).collect())
            .collect();

        // take col sum
        let col_sums: Vec<f32> = transposed
            .iter()
            .map(|col| {
                let w = 1.0 - col.iter().map(|node| node.w).sum::<f32>();
                let n = col.iter().map(|node| node.n).sum::<usize>() as f32;
                self.ucb1(w, n)
            })
            .collect();

        // get argmax
        col_sums
            .iter()
            .enumerate()
            .max_by(|&(_, a), &(_, b)| {
                a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(index, _)| index)
            .unwrap()
    }

    fn next_child_node(&mut self) -> &mut Node {
        // find if n == 0
        for (row, child_nodes) in self.child_nodeses.iter().enumerate() {
            for (col, node) in child_nodes.iter().enumerate() {
                if node.n == 0 {
                    return &mut self.child_nodeses[row][col];
                }
            }
        }

        let best_i = self.action0();
        let best_j = self.action1();
        &mut self.child_nodeses[best_i][best_j]
    }

    pub fn explore(&mut self) -> f32 {
        if self.state.is_done() {
            let value = self.state.white_score().score;
            self.increment(value);
            return value;
        }
        if self.child_nodeses.is_empty() {
            let value = monte_carlo::Playout::new(&self.state).playout();
            self.increment(value);
            if self.n >= self.params.expand_threshold {
                self.expand();
            }
            return value;
        }
        // children
        let value = self.next_child_node().explore();
        self.increment(value);
        value
    }

    pub fn best_j(&self) -> usize {
        // transpose
        let transposed: Vec<Vec<&Node>> = (0..self.child_nodeses[0].len())
            .map(|i| self.child_nodeses.iter().map(|row| &row[i]).collect())
            .collect();

        transposed
            .iter()
            .map(|childs| childs.iter().map(|node| node.n).sum::<usize>())
            .enumerate()
            .max_by_key(|(_, n)| n.to_owned())
            .map(|(index, _)| index)
            .unwrap()
    }

    pub fn best_i(&self) -> usize {
        self.child_nodeses
            .iter()
            .map(|childs| childs.iter().map(|node| node.n).sum::<usize>())
            .enumerate()
            .max_by_key(|(_, n)| n.to_owned())
            .map(|(index, _)| index)
            .unwrap()
    }
}

fn duct(
    state: &maze_state::SimultaneousMazeState,
    params: DuctParams,
    player_id: usize,
    num_playout: usize,
) -> usize {
    let mut node = Node::new(state, params);
    node.expand();
    for _ in 0..num_playout {
        node.explore();
    }

    let legal_actions = state.legal_actions(player_id);
    if player_id == 0 {
        let best_i = node.best_i();
        legal_actions[best_i]
    } else {
        let best_j = node.best_j();
        legal_actions[best_j]
    }
}

pub fn duct_arc(
    params: DuctParams,
    num_playout: usize,
) -> maze_state::ActionFunc {
    Arc::new(move |state, player_id| -> usize {
        duct(state, params.clone(), player_id, num_playout)
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
        let params = DuctParams {
            c: 1.0,
            expand_threshold: 3,
        };
        Node::new(&state, params)
    }

    #[test]
    fn test_duct_arc() {
        let state_params = maze_state::MazeParams {
            height: 3,
            width: 3,
            end_turn: 4,
        };
        let state = maze_state::SimultaneousMazeState::new(0, state_params);
        let params = DuctParams {
            c: 1.0,
            expand_threshold: 3,
        };
        let actual = duct_arc(params, 1000)(&state, 1);
        assert_eq!(actual, 3);
    }

    #[test]
    fn test_duct() {
        let state_params = maze_state::MazeParams {
            height: 3,
            width: 3,
            end_turn: 4,
        };
        let state = maze_state::SimultaneousMazeState::new(0, state_params);
        let params = DuctParams {
            c: 1.0,
            expand_threshold: 3,
        };
        let actual = duct(&state, params, 0, 1000);
        assert_eq!(actual, 3);
    }

    #[test]
    fn test_best_j() {
        let mut node = setup();
        node.expand();
        node.child_nodeses[0][0].n = 0;
        node.child_nodeses[1][1].n = 1;
        node.child_nodeses[2][2].n = 2;
        assert_eq!(node.best_j(), 2);
        assert_eq!(node.best_i(), 2);
    }

    #[test]
    fn test_explore() {
        let mut node = setup();
        let actual = node.explore();
        assert!(actual <= 1.0);
    }

    #[test]
    fn test_next_child_node() {
        let mut node = setup();
        node.expand();
        let actual = node.next_child_node();
        assert_eq!(actual.n, 0);

        for i in 0..3 {
            for j in 0..3 {
                node.child_nodeses[i][j].n = 1;
            }
        }
        node.child_nodeses[0][0].w = 0.5;

        let actual = node.next_child_node();
        assert_eq!(actual.n, 1);
    }

    #[test]
    fn test_action_1() {
        let mut node = setup();
        node.expand();
        node.child_nodeses[0][0].n = 1;
        node.child_nodeses[0][0].w = 0.5;
        node.child_nodeses[1][1].n = 1;
        node.child_nodeses[1][1].w = 0.5;
        node.child_nodeses[2][2].n = 1;
        node.child_nodeses[2][2].w = 0.7;
        assert_eq!(node.action1(), 1);
    }

    #[test]
    fn test_action_0() {
        let mut node = setup();
        node.expand();
        node.child_nodeses[0][0].n = 1;
        node.child_nodeses[0][0].w = 0.5;
        node.child_nodeses[1][1].n = 1;
        node.child_nodeses[2][2].n = 1;
        assert_eq!(node.action0(), 0);
    }

    #[test]
    fn test_expand() {
        let mut node = setup();
        assert!(node.child_nodeses.is_empty());
        node.expand();
        assert!(!node.child_nodeses.is_empty());
        assert!(!node.child_nodeses[0].is_empty());
    }

    #[test]
    fn test_ucb1() {
        let mut node = setup();
        node.n = 1;
        let child_nodes = vec![node.clone(); 3];
        let child_nodeses = vec![child_nodes; 3];
        node.child_nodeses = child_nodeses;
        let actual = node.ucb1(2.0, 3.0);
        assert_eq!(actual, 1.5224752);
    }

    #[test]
    fn test_t() {
        let mut node = setup();
        node.n = 1;
        let child_nodes = vec![node.clone(); 3];
        let child_nodeses = vec![child_nodes; 3];
        node.child_nodeses = child_nodeses;
        let actual = node.t();
        assert_eq!(actual, 9.0);
    }

    #[test]
    fn test_increment() {
        let mut node = setup();
        node.increment(0.5);
        assert_eq!(node.n, 1);
        assert_eq!(node.w, 0.5);
    }

    #[test]
    fn make_node() {
        let node = setup();
        assert!(node.child_nodeses.is_empty());
    }
}
