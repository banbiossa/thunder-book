use std::mem::swap;

use crate::base::alternate::{AlternateState, MazeParams};
use crate::ch08::maze_state::Status;

#[derive(Debug, Clone)]
pub struct BitsetConnectFour {
    my_board: usize,
    all_board: usize,
    is_first: bool,
    status: Status,
    params: MazeParams,
}

impl BitsetConnectFour {
    fn new(params: &MazeParams) -> Self {
        BitsetConnectFour {
            my_board: 0,
            all_board: 0,
            is_first: true,
            status: Status::ONGOING,
            params: params.clone(),
        }
    }
    fn is_done(&self) -> bool {
        self.status != Status::ONGOING
    }
    fn legal_actions(&self) -> Vec<usize> {
        let possible = self.all_board + self.floor_bit();
        (0..4)
            .filter(|x| self.filter_column(x.to_owned()) & possible != 0)
            .collect()
    }
    fn floor_bit(&self) -> usize {
        // 0b00000010000001...
        // w: 繰り返しの数 h: 0 の数
        let mut bit = 0;
        for x in 0..self.params.width {
            bit |= 1 << (x * (self.params.height + 1))
        }
        bit
    }
    fn filled(&self) -> usize {
        // 0b0111101111...
        // h = 2 として
        // 1 << h = 100
        // 100 - 1 = 011
        // 011 << x*(h+1) = 011000
        // の組み合わせ
        let mut bit = 0;
        for x in 0..self.params.width {
            let mut t = 1 << self.params.height;
            t -= 1;
            t <<= x * (self.params.height + 1);
            bit |= t;
        }
        bit
    }
    fn filter_column(&self, column: usize) -> usize {
        // ある column で高さ全部が 1 になるようなビット 0b011
        // 0b011 とか 0b011000 とか
        assert!(column < self.params.width);
        let bits = (1 << self.params.height) - 1;
        let shift_width = (self.params.height + 1) * column;
        bits << shift_width
    }
    fn teban_score(&self) -> f32 {
        match self.status {
            Status::DRAW => 0.5,
            Status::LOSE => 0.0,
            Status::ONGOING => panic!("still ongoing"),
        }
    }
    fn white_score(&self) -> f32 {
        let score = self.teban_score();
        if !self.is_first {
            return 1.0 - score;
        }
        score
    }
    fn is_connected(&self, board: usize) -> bool {
        // -
        let t = board & (board >> (self.params.height + 1));
        if t & (t >> (2 * (self.params.height + 1))) != 0 {
            return true;
        }
        // /
        let t = board & (board >> self.params.height);
        if t & (t >> (2 * self.params.height)) != 0 {
            return true;
        }
        // \
        let t = board & (board >> (self.params.height + 2));
        if t & (t >> (2 * self.params.height + 2)) != 0 {
            return true;
        }
        // |
        let t = board & (board >> 1);
        if t & (t >> 2) != 0 {
            return true;
        }

        false
    }
    fn advance(&mut self, action: usize) {
        // swap
        self.my_board ^= self.all_board;
        self.is_first = !self.is_first;

        // add action
        let action_as_floor_bit = 1 << (action * (self.params.height + 1));
        self.all_board |= self.all_board + action_as_floor_bit;

        // is done?
        if self.is_connected(self.my_board ^ self.all_board) {
            self.status = Status::LOSE;
        } else if self.all_board == self.filled() {
            self.status = Status::DRAW;
        }
    }
    fn to_string(&self) -> String {
        let width = self.params.width;
        let height = self.params.height;
        let total_cells = width * (height + 1);

        let mut me =
            format!("{:0>width$b}", self.my_board, width = total_cells,);
        let mut you = format!(
            "{:0>width$b}",
            self.my_board ^ self.all_board,
            width = total_cells
        );
        if !self.is_first {
            swap(&mut me, &mut you);
        }

        let mut board = Vec::new();
        for i in 0..height {
            let mut row = "".to_string();
            for j in 0..width {
                let index = j * (height + 1) + i;
                let mine = me.chars().rev().nth(index).unwrap();
                let yours = you.chars().rev().nth(index).unwrap();

                if mine == '1' {
                    row += "O";
                } else if yours == '1' {
                    row += "X";
                } else {
                    row += ".";
                }
            }
            board.push(row);
        }

        let header = format!("is_first: {}\n\n", self.is_first);
        let board_str = board
            .iter()
            .rev()
            .map(|row| row.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        header + &board_str + "\n"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> BitsetConnectFour {
        let params = MazeParams {
            height: 2,
            width: 4,
            end_turn: 0,
        };
        BitsetConnectFour::new(&params)
    }

    #[test]
    fn test_to_string() {
        let mut state = setup();
        let actual = state.to_string();
        let expected = "\
is_first: true

....
....
";
        assert_eq!(actual, expected);

        state.advance(0);
        state.advance(1);
        let actual = state.to_string();
        let expected = "\
is_first: true

....
OX..
";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_advance() {
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
    }

    #[test]
    fn test_is_connected_large() {
        let params = MazeParams {
            height: 6,
            width: 7,
            end_turn: 0,
        };
        let state = BitsetConnectFour::new(&params);
        let board = 0b0000001000000100000010000001000000100000010000001;
        assert!(state.is_connected(board));

        let board = 0b0000001000000100000010000000000000100000010000001;
        assert_eq!(state.is_connected(board), false);

        let board = 0b0000001000001000001000001000000000100000010000001;
        assert!(state.is_connected(board));

        let board = 0b0000001000001000001000010000000000100000010000001;
        assert_eq!(state.is_connected(board), false);
    }

    #[test]
    fn test_is_connected() {
        let state = setup();
        assert_eq!(state.is_connected(0), false);
        assert_eq!(state.is_connected(0b001001001001), true);
    }

    #[test]
    fn test_teban_score() {
        let mut state = setup();
        state.status = Status::LOSE;
        assert_eq!(state.teban_score(), 0.0);
        state.is_first = false;
        assert_eq!(state.white_score(), 1.0);
    }

    #[test]
    fn test_legal_actions() {
        let state = setup();
        let actual = state.legal_actions();
        let expected = vec![0, 1, 2, 3];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_filter_column() {
        let state = setup();
        let actual = state.filter_column(0);
        let expected = 0b011;
        assert_eq!(actual, expected);

        let actual = state.filter_column(1);
        let expected = 0b011000;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_filled() {
        let state = setup();
        let actual = state.filled();
        let expected = 0b011011011011;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_floor_bit() {
        let state = setup();
        let actual = state.floor_bit();
        let expected = 0b001001001001;
        assert_eq!(actual, expected);
    }

    #[test]
    fn make_state() {
        let state = setup();
        assert_eq!(state.is_first, true);
        assert_eq!(state.is_done(), false);
    }
}
