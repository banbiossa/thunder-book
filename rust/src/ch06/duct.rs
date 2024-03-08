use crate::ch06::maze_state;

#[derive(Debug, Clone)]
struct DuctParams {
    c: f32,
    expand_threshold: usize,
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
