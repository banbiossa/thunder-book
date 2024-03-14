use bitvec::prelude::*;

use crate::base::state::MazeParams;

#[derive(Debug, Clone)]
pub struct Mat {
    bits: Vec<BitArray>,
    params: MazeParams,
}

impl Mat {
    pub fn new(other: &Mat) -> Self {
        Mat {
            bits: other.bits.clone(),
            params: other.params.clone(),
        }
    }

    pub fn get(&self, y: usize, x: usize) -> bool {
        self.bits[y][x]
    }

    pub fn set(&mut self, y: usize, x: usize) {
        self.bits[y].set(x, true);
    }

    pub fn del(&mut self, y: usize, x: usize) {
        self.bits[y].set(x, false);
    }

    fn up(&self) -> Mat {
        let mut mat = self.clone();
        for y in 0..self.params.height - 1 {
            mat.bits[y] |= self.bits[y + 1];
        }
        mat
    }

    fn down(&self) -> Mat {
        let mut mat = self.clone();
        for y in (1..=self.params.height - 1).rev() {
            mat.bits[y] |= self.bits[y - 1];
        }
        mat
    }

    fn right(&self) -> Mat {
        let mut mat = self.clone();
        for y in 0..self.params.height {
            mat.bits[y].shift_right(1);
        }
        mat
    }
}

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
                bitarr![0, 1, 0], // add comments so rustfmt keeps it alone
                bitarr![1, 0, 0],
                bitarr![0, 0, 1],
            ],
            params,
        };
        mat
    }

    #[test]
    fn test_right() {
        let a = setup();
        let actual = a.right();
        let expected = vec![
            bitarr![0, 1, 1], // add comments so rustfmt keeps it alone
            bitarr![1, 1, 0],
            bitarr![0, 0, 1],
        ];
        assert_eq!(actual.bits, expected);
    }

    #[test]
    fn test_down() {
        let a = setup();
        let actual = a.down();
        let expected = vec![
            bitarr![0, 1, 0], // add comments so rustfmt keeps it alone
            bitarr![1, 1, 0],
            bitarr![1, 0, 1],
        ];
        assert_eq!(actual.bits, expected);
    }

    #[test]
    fn test_up() {
        let a = setup();
        let up = a.up();
        let expected = vec![
            bitarr![1, 1, 0], // add comments so rustfmt keeps it alone
            bitarr![1, 0, 1],
            bitarr![0, 0, 1],
        ];
        assert_eq!(up.bits, expected);
    }

    #[test]
    fn test_del() {
        let mut a = setup();
        assert_eq!(a.get(0, 1), true);
        a.del(0, 1);
        assert_eq!(a.get(0, 1), false);
    }

    #[test]
    fn test_set() {
        let mut a = setup();
        assert_eq!(a.get(0, 0), false);
        a.set(0, 0);
        assert_eq!(a.get(0, 0), true);
    }

    #[test]
    fn test_get() {
        let a = setup();
        assert_eq!(a.get(0, 1), true);
    }

    #[test]
    fn test_bits() {
        let a = bits![0, 1, 0];
        assert_eq!(a.get(0).unwrap(), false);
        assert_eq!(a.get(0).as_deref(), Some(&false));
        assert_eq!(a.get(1).unwrap(), true);
        assert_eq!(a.get(2).unwrap(), false);
        // assert_eq!(a.get(3).unwrap(), true);
    }

    #[test]
    fn test_bitarr() {
        let a = bitarr![0, 1, 0];
        assert_eq!(a[0], false);
        assert_eq!(a[1], true);
    }

    #[test]
    fn make_mat() {
        let a = setup();
        let b = Mat::new(&a);
        assert_eq!(b.bits[0], bitarr![0, 1, 0]);
    }

    #[test]
    fn test_bits_to_string() {
        let b = 4;
        let b_fmt = format!("{:04b}", b);
        assert_eq!(b_fmt, "0100");
    }
}
