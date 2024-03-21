use crate::base::alternate::{AlternateState, MazeParams};
use std::{collections::VecDeque, mem::swap};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Stone {
    x: usize,
    y: usize,
}

// remake enum in python
enum D {
    UP,
    DOWN,
    STAY,
}

impl D {
    fn value(&self) -> Vec<isize> {
        match self {
            D::STAY => vec![0, 0],
            D::DOWN => vec![-1, 1],
            D::UP => vec![1, -1],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Status {
    ONGOING,
    // todo: comment out the below with comments
    // WIN is never made, all advance ends on LOSE or DRAW
    WIN,
    LOSE,
    DRAW,
}

#[derive(Debug, Clone)]
struct ConnectFourState {
    is_first: bool,
    my_board: Vec<Vec<bool>>,
    enemy_board: Vec<Vec<bool>>,
    status: Status,
    params: MazeParams,
}

impl AlternateState for ConnectFourState {
    // seed is not used but necessary for keeping the same interface
    fn new(_: u64, params: MazeParams) -> Self {
        ConnectFourState {
            is_first: true,
            my_board: vec![vec![false; params.width]; params.height],
            enemy_board: vec![vec![false; params.width]; params.height],
            status: Status::ONGOING,
            params: params.clone(),
        }
    }

    fn legal_actions(&self) -> Vec<usize> {
        let mut actions = vec![];
        for x in 0..self.params.width {
            for y in 0..self.params.height {
                if !self.my_board[y][x] && !self.enemy_board[y][x] {
                    actions.push(x);
                    break;
                }
            }
        }
        actions
    }
    fn to_string(&self) -> String {
        let mut ss = format!("is_first: {}\n", self.is_first);
        for y in (0..self.params.height).rev() {
            ss += "\n";
            for x in 0..self.params.width {
                if self.my_board[y][x] {
                    ss.push_str(if self.is_first { "X" } else { "O" });
                } else if self.enemy_board[y][x] {
                    ss.push_str(if self.is_first { "O" } else { "X" });
                } else {
                    ss += ".";
                }
            }
        }
        ss += "\n";
        ss
    }

    fn teban_score(&self) -> f32 {
        match self.status {
            Status::DRAW => 0.5,
            Status::LOSE => 0.0,
            Status::WIN => 1.0,
            Status::ONGOING => panic!("shouldn't call teban_score"),
        }
    }

    fn white_score(&self) -> f32 {
        let score = self.teban_score();
        if !self.is_first {
            return 1.0 - score;
        }
        score
    }
    fn is_done(&self) -> bool {
        self.status != Status::ONGOING
    }

    fn advance(&mut self, action: usize) {
        let stone = self.place_stone(action);

        // -
        self.check_connection(&stone, D::UP, D::STAY);
        // /
        if !self.is_done() {
            self.check_connection(&stone, D::UP, D::UP);
        }
        // \
        if !self.is_done() {
            self.check_connection(&stone, D::UP, D::DOWN);
        }
        // |
        if !self.is_done() {
            self.check_connection(&stone, D::STAY, D::UP);
        }

        // swap
        swap(&mut self.my_board, &mut self.enemy_board);
        self.is_first = !self.is_first;

        if !self.is_done() && self.legal_actions().len() == 0 {
            self.status = Status::DRAW;
        }
    }
}

impl ConnectFourState {
    fn place_stone(&mut self, action: usize) -> Stone {
        for y in 0..self.params.height {
            if !self.my_board[y][action] && !self.enemy_board[y][action] {
                self.my_board[y][action] = true;
                return Stone { x: action, y };
            }
        }
        panic!("no stone to place");
    }

    fn check_connection(&mut self, first_stone: &Stone, dx: D, dy: D) -> bool {
        let mut que = VecDeque::new();
        que.push_back(first_stone.clone());
        let mut check =
            vec![vec![false; self.params.width]; self.params.height];
        let mut count = 0;
        while !que.is_empty() {
            let stone = que.pop_front().unwrap();
            count += 1;
            if count >= 4 {
                self.status = Status::LOSE;
                return true;
            }
            check[stone.y][stone.x] = true;

            for action in 0..2 {
                let ty = stone.y as isize + dy.value()[action];
                let tx = stone.x as isize + dx.value()[action];
                if ty >= 0
                    && (ty as usize) < self.params.height
                    && tx >= 0
                    && (tx as usize) < self.params.width
                    && self.my_board[ty as usize][tx as usize]
                    && !check[ty as usize][tx as usize]
                {
                    que.push_back(Stone {
                        x: tx as usize,
                        y: ty as usize,
                    });
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ch05::mcts::{mcts_action_arc, MCTSParams};

    fn setup() -> ConnectFourState {
        let params = MazeParams {
            height: 2,
            width: 4,
            end_turn: 0,
        };
        ConnectFourState::new(0, params)
    }

    #[test]
    fn test_use_ch05() {
        let state = setup();
        let params = MCTSParams {
            c: 1.0,
            expand_threshold: 3,
        };
        let actual = mcts_action_arc(10, params)(&state);
        let legal_actions = state.legal_actions();
        assert!(legal_actions.contains(&actual));
    }

    #[test]
    fn test_advance_to_end_game() {
        let mut state = setup();
        state.advance(0);
        state.advance(0);
        state.advance(1);
        state.advance(1);
        state.advance(2);
        state.advance(2);
        assert_eq!(state.is_done(), false);
        state.advance(3);
        assert_eq!(state.is_done(), true);
        assert_eq!(state.white_score(), 1.0);
        assert_eq!(state.teban_score(), 0.0);

        let actual = state.to_string();
        let expected = "\
is_first: false

OOO.
XXXX
";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_advance() {
        let mut state = setup();
        state.advance(0);
        assert_eq!(state.is_first, false);
        assert_eq!(state.my_board, vec![vec![false; 4]; 2]);
        assert_eq!(
            state.enemy_board,
            vec![vec![true, false, false, false], vec![false; 4]]
        );
    }

    #[test]
    fn test_is_done() {
        let mut state = setup();
        assert_eq!(state.is_done(), false);
        state.status = Status::DRAW;
        assert_eq!(state.is_done(), true);
    }

    #[test]
    fn test_connection() {
        let mut state = setup();
        let stone = Stone { x: 0, y: 0 };
        assert_eq!(state.check_connection(&stone, D::UP, D::STAY), false);

        state.my_board = vec![vec![true; 4], vec![false; 4]];
        assert_eq!(state.check_connection(&stone, D::UP, D::STAY), true);
    }

    #[test]
    fn test_white_score() {
        let mut state = setup();
        state.status = Status::WIN;
        state.is_first = false;
        assert_eq!(state.white_score(), 0.0);
    }

    #[test]
    fn test_teban_score() {
        let mut state = setup();
        state.status = Status::WIN;
        assert_eq!(state.teban_score(), 1.0);
    }

    #[test]
    fn test_to_string() {
        let state = setup();
        let actual = state.to_string();
        let expected = "\
is_first: true

....
....
";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_place_stone() {
        let mut state = setup();
        let actual = state.place_stone(0);
        let expected = Stone { x: 0, y: 0 };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_legal_actions() {
        let state = setup();
        let actual = state.legal_actions();
        let expected = vec![0, 1, 2, 3];
        assert_eq!(actual, expected);
    }

    #[test]
    fn make_state() {
        let state = setup();
        assert_eq!(state.params.width, 4);
    }
}
