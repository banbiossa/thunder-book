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
