use std::collections::HashMap;

#[derive(Debug, Clone)]
struct MazeParams {
    width: usize,
    height: usize,
}

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
    fn value(&self) -> (isize, isize) {
        match self {
            D::STAY => (0, 0),
            D::DOWN => (-1, 1),
            D::UP => (1, -1),
        }
    }
}

#[derive(Debug, Clone)]
enum Status {
    ONGOING,
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

impl ConnectFourState {
    fn new(params: &MazeParams) -> Self {
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

    fn place_stone(&mut self, action: usize) -> Stone {
        for y in 0..self.params.height {
            if !self.my_board[y][action] && !self.enemy_board[y][action] {
                self.my_board[y][action] = true;
                return Stone { x: action, y };
            }
        }
        panic!("no stone to place");
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> ConnectFourState {
        let params = MazeParams {
            height: 2,
            width: 4,
        };
        ConnectFourState::new(&params)
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
