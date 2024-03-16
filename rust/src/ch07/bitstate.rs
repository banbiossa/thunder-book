use crate::base::state::{MazeParams, SinglePlayerState};

use super::zobrist_hash::ZobristState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mat {
    bits: Vec<usize>,
    params: MazeParams,
}

impl Mat {
    pub fn new(params: &MazeParams) -> Self {
        Mat {
            bits: vec![0; params.height],
            params: params.clone(),
        }
    }

    pub fn get(&self, y: usize, x: usize) -> bool {
        self.bits[y] & (1 << x) != 0
    }

    pub fn set(&mut self, y: usize, x: usize) {
        self.bits[y] |= 1 << x;
    }

    pub fn del(&mut self, y: usize, x: usize) {
        self.bits[y] &= !(1 << x);
    }

    fn up(&self) -> Mat {
        let mut mat = self.clone();
        // add one line at the bottom
        mat.bits.push(0);
        mat.bits.remove(0);
        mat
    }

    fn down(&self) -> Mat {
        let mut mat = self.clone();
        // add one line at the top
        mat.bits.pop();
        mat.bits.insert(0, 0);
        mat
    }

    fn width_mask(&self) -> usize {
        (1 << self.params.width) - 1
    }

    fn left(&self) -> Mat {
        let mut mat = self.clone();
        for y in 0..self.params.height {
            mat.bits[y] <<= 1;
            mat.bits[y] &= self.width_mask();
        }
        mat
    }

    fn right(&self) -> Mat {
        let mut mat = self.clone();
        for y in 0..self.params.height {
            mat.bits[y] >>= 1;
        }
        mat
    }

    fn or(&mut self, other: &Mat) {
        for y in 0..self.params.height {
            self.bits[y] |= other.bits[y];
        }
    }

    fn expand(&mut self) {
        self.or(&self.up());
        self.or(&self.down());
        self.or(&self.left());
        self.or(&self.right());
    }

    fn andeq_not(&mut self, other: &Mat) {
        // this &=~other
        for y in 0..self.params.height {
            self.bits[y] &= !other.bits[y];
        }
    }

    fn is_any_equal(&self, other: &Mat) -> bool {
        for y in 0..self.params.height {
            if self.bits[y] & other.bits[y] != 0 {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiBitsetState {
    state: ZobristState,
    points: Mat,
    walls: Mat,
}

// impl SinglePlayerState for MultiBitsetState {
//     fn new(seed: u64, params: MazeParams) -> Self {
//         let state = ZobristState::new(seed, params);
//         let mut points = Mat::new(&params);
//         let mut walls = Mat::new(&params);

//         for y in 0..params.height {
//             for x in 0..params.width {
//                 if state.get_walls()[y][x] != 0 {
//                     walls.set(y, x);
//                 }
//                 if state.get_points()[y][x] != 0 {
//                     points.set(y, x);
//                 }
//             }
//         }
//         Self {
//             state,
//             points,
//             walls,
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Mat {
        let params = MazeParams {
            height: 3,
            width: 3,
            end_turn: 3,
        };
        let mat = Mat {
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
