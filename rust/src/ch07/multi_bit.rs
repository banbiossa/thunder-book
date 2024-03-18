use crate::base::state::MazeParams;
use crate::ch07::bitstate::Mat;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MultiBit {
    bits: Vec<usize>,
    params: MazeParams,
}

impl Mat for MultiBit {
    fn new(params: &MazeParams) -> Self {
        MultiBit {
            bits: vec![0; params.height],
            params: params.clone(),
        }
    }

    fn get(&self, y: usize, x: usize) -> bool {
        self.bits[y] & (1 << x) != 0
    }

    fn set(&mut self, y: usize, x: usize) {
        self.bits[y] |= 1 << x;
    }

    fn del(&mut self, y: usize, x: usize) {
        self.bits[y] &= !(1 << x);
    }

    fn expand(&mut self) {
        self.or(&self.up());
        self.or(&self.down());
        self.or(&self.left());
        self.or(&self.right());
    }

    fn andeq_not(&mut self, other: &MultiBit) {
        // this &=~other
        for y in 0..self.params.height {
            self.bits[y] &= !other.bits[y];
        }
    }

    fn is_any_equal(&self, other: &MultiBit) -> bool {
        for y in 0..self.params.height {
            if self.bits[y] & other.bits[y] != 0 {
                return true;
            }
        }
        false
    }
}

impl MultiBit {
    fn up(&self) -> MultiBit {
        let mut mat = self.clone();
        // add one line at the bottom
        mat.bits.push(0);
        mat.bits.remove(0);
        mat
    }

    fn down(&self) -> MultiBit {
        let mut mat = self.clone();
        // add one line at the top
        mat.bits.pop();
        mat.bits.insert(0, 0);
        mat
    }

    fn left(&self) -> MultiBit {
        let mut mat = self.clone();
        let width_mask = (1 << self.params.width) - 1;
        for y in 0..self.params.height {
            mat.bits[y] <<= 1;
            mat.bits[y] &= width_mask;
        }
        mat
    }

    fn right(&self) -> MultiBit {
        let mut mat = self.clone();
        for y in 0..self.params.height {
            mat.bits[y] >>= 1;
        }
        mat
    }

    fn or(&mut self, other: &MultiBit) {
        for y in 0..self.params.height {
            self.bits[y] |= other.bits[y];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::state::SinglePlayerState;
    use crate::ch07::bitstate::BitsetState;

    fn setup() -> MultiBit {
        let params = MazeParams {
            height: 3,
            width: 3,
            end_turn: 3,
        };
        let mat = MultiBit {
            bits: vec![
                // [0, 1, 0],
                // [1, 0, 0],
                // [0, 0, 1],
                1 << 1,
                1 << 2,
                1,
            ],
            params,
        };
        mat
    }

    fn setup_state() -> BitsetState<MultiBit> {
        let params = MazeParams {
            height: 5,
            width: 5,
            end_turn: 3,
        };
        BitsetState::new(0, params)
    }

    #[test]
    fn test_get_near_point() {
        let mut state = setup_state();
        assert_eq!(state.get_distance_to_nearest_point(), 1);
        state.advance(1);
        assert_eq!(state.get_distance_to_nearest_point(), 2);

        // test_evaluate_calls_get_near_point
        state.evaluate_score();
    }

    #[test]
    fn make_state() {
        let mut state = setup_state();
        assert_eq!(state.get_evaluated_score(), 0);
        let expected = "\
turn:\t0
score:\t0

2@711
.#4##
51825
6#9##
6#735
";
        assert_eq!(state.to_string(), expected);

        // remove point
        assert_eq!(state.get_points_mat().get(0, 0), true);
        state.advance(1);
        assert_eq!(state.get_points_mat().get(0, 0), false);
    }

    #[test]
    fn test_is_any_equal() {
        let a = setup();
        assert_eq!(a.is_any_equal(&a), true);

        let mut b = setup();
        b.bits = vec![1, 1, 1];
        assert_eq!(a.is_any_equal(&b), true);

        b.bits = vec![1, 1, 0];
        assert_eq!(a.is_any_equal(&b), false);
    }

    #[test]
    fn test_partial_eq() {
        let a = setup();
        let b = setup();
        assert_eq!(a, b);

        let mut a = setup();
        a.bits[0] = 1;
        let b = setup();
        assert_ne!(a, b);
    }

    #[test]
    fn test_andeq_not() {
        let mut a = setup();
        let b = setup();
        a.andeq_not(&b);
        let expected = vec![
            // [0, 0, 0],
            // [0, 0, 0],
            // [0, 0, 0],
            0, 0, 0,
        ];
        assert_eq!(a.bits, expected);
    }

    #[test]
    fn test_expand() {
        let mut a = setup();
        a.expand();
        let expected = vec![
            // [1, 1, 1],
            // [1, 1, 1],
            // [1, 1, 1],
            (1 << 3) - 1,
            (1 << 3) - 1,
            (1 << 3) - 1,
        ];
        assert_eq!(a.bits, expected);
    }

    #[test]
    fn test_or() {
        let mut a = setup();
        let up = a.up();
        a.or(&up);
        let expected = vec![
            // [1, 1, 0],
            // [1, 0, 1],
            // [0, 0, 1],
            (1 << 2) | (1 << 1),
            (1 << 2) | 1,
            1,
        ];
        assert_eq!(a.bits, expected);
    }

    #[test]
    fn test_left() {
        let a = setup();
        let actual = a.left();
        let expected = vec![
            // [1, 0, 0],
            // [0, 0, 0],
            // [0, 1, 0],
            1 << 2,
            0,
            1 << 1,
        ];
        assert_eq!(actual.bits, expected);
    }

    #[test]
    fn test_right() {
        let a = setup();
        let actual = a.right();
        let expected = vec![
            // [0, 0, 1],
            // [0, 1, 0],
            // [0, 0, 0],
            1,
            1 << 1,
            0,
        ];
        assert_eq!(actual.bits, expected);
    }

    #[test]
    fn test_down() {
        let a = setup();
        let actual = a.down();
        let expected = vec![
            // [0, 0, 0],
            // [0, 1, 0],
            // [1, 0, 0],
            0,
            1 << 1,
            1 << 2,
        ];
        assert_eq!(actual.bits, expected);
    }

    #[test]
    fn test_up() {
        let a = setup();
        let up = a.up();
        let expected = vec![
            // [1, 0, 0],
            // [0, 0, 1],
            // [0, 0, 0],
            (1 << 2),
            1,
            0,
        ];
        assert_eq!(up.bits, expected);
    }

    #[test]
    fn test_del() {
        // [0, 1, 0],
        // [1, 0, 0],
        // [0, 0, 1],
        let mut a = setup();
        assert_eq!(a.get(0, 1), true);
        a.del(0, 1);
        assert_eq!(a.get(0, 1), false);
        assert_eq!(a.get(0, 2), false);

        assert_eq!(a.get(1, 2), true);
        a.del(1, 2);
        assert_eq!(a.get(1, 2), false);
    }

    #[test]
    fn test_set() {
        // [0, 1, 0],
        // [1, 0, 0],
        // [0, 0, 1],
        let mut a = setup();
        assert_eq!(a.get(0, 0), false);
        assert_eq!(a.get(0, 1), true);
        assert_eq!(a.get(0, 2), false);
        a.set(0, 2);
        assert_eq!(a.get(0, 2), true);
    }

    #[test]
    fn test_get() {
        // [0, 1, 0],
        // [1, 0, 0],
        // [0, 0, 1],
        let a = setup();
        assert_eq!(a.get(0, 1), true);
        assert_eq!(a.get(0, 2), false);
        assert_eq!(a.get(0, 0), false);
        assert_eq!(a.get(0, 0), false);
        assert_eq!(a.get(1, 2), true);
    }

    #[test]
    fn test_arithmetic() {
        let a = 0;
        let b = a << 1;
        println!("{b}");
        println!("{}", a >> 1);
        println!("{}", 1 << 1);
        println!("{}", 1 >> 1);
    }

    #[test]
    fn test_arithmetic_set_n() {
        // check original
        let a = 1;
        let a_fmt = format!("{:04b}", a);
        assert_eq!(a_fmt, "0001");

        // set n
        let b = 1 << 3;
        let c = a | b;
        assert_eq!(format!("{:04b}", c), "1001");
    }

    #[test]
    fn test_arithmetic_get_n() {
        let a = 1 | (1 << 3);
        assert_eq!(format!("{:04b}", a), "1001");

        assert_eq!(1, (a >> 3) | 1);
    }

    #[test]
    fn test_arithmetic_del_n() {
        let a = (1 << 2) | (1 << 3);
        assert_eq!(format!("{:04b}", a), "1100");

        let c = a & !(1 << 2);
        assert_eq!(format!("{:04b}", c), "1000");
    }

    #[test]
    fn test_arithmetic_shift_0() {
        let a = 1 << 0;
        assert_eq!(a, 1);
    }

    #[test]
    fn test_bits_to_string() {
        let b = 4;
        let b_fmt = format!("{:04b}", b);
        assert_eq!(b_fmt, "0100");
    }
}
