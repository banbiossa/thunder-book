use crate::base::state::MazeParams;
use crate::ch07::bitstate::Mat;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SingleBit {
    bits: usize,
    height: usize,
    width: usize,
}

impl Mat for SingleBit {
    // impl SingleBit {
    fn new(params: &MazeParams) -> Self {
        SingleBit {
            bits: 0,
            height: params.height,
            width: params.width,
        }
    }

    fn get(&self, y: usize, x: usize) -> bool {
        self.bits & (1 << (y * self.width + x)) != 0
    }

    fn set(&mut self, y: usize, x: usize) {
        self.bits |= 1 << (y * self.width + x)
    }

    fn del(&mut self, y: usize, x: usize) {
        self.bits &= !(1 << (y * self.width + x))
    }

    fn expand(&mut self) {
        self.bits |= self.up();
        self.bits |= self.down();
        self.bits |= self.left();
        self.bits |= self.right();
    }

    fn andeq_not(&mut self, other: &SingleBit) {
        // this &= ~other
        self.bits &= !other.bits;
    }

    fn is_any_equal(&self, other: &Self) -> bool {
        self.bits & other.bits != 0
    }
}

impl SingleBit {
    fn left_mask(&self) -> usize {
        // 0 1 1
        // 0 1 1
        // 0 1 1
        let mut mask = 0;
        for h in 0..self.height {
            mask |= ((1 << self.width) - 2) << (h * self.width);
        }
        mask
    }

    fn right_mask(&self) -> usize {
        // 1 1 0
        // 1 1 0
        // 1 1 0
        let mut mask = 0;
        for h in 0..self.height {
            mask |= ((1 << (self.width - 1)) - 1) << (h * self.width);
        }
        mask
    }

    pub fn up(&self) -> usize {
        self.bits >> self.width
    }

    pub fn down(&self) -> usize {
        let mask = (1 << self.width * self.height) - 1;
        let bits = self.bits << self.width;
        bits & mask
    }

    pub fn left(&self) -> usize {
        (self.bits & self.left_mask()) >> 1
    }

    pub fn right(&self) -> usize {
        (self.bits & self.right_mask()) << 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> SingleBit {
        let params = MazeParams {
            height: 3,
            width: 3,
            end_turn: 3,
        };
        let mut mat = SingleBit::new(&params);
        let bits = (1 << 1) | (1 << 3) | (1 << 8);
        // [0, 1, 0],
        // [1, 0, 0],
        // [0, 0, 1],
        mat.bits = bits;

        mat
    }

    #[test]
    fn test_is_any_equal() {
        let a = setup();
        let b = a.clone();
        assert_eq!(a.is_any_equal(&b), true);

        // [1, 0, 0],
        // [0, 0, 1],
        // [0, 1, 0],
        let mut b = a.clone();
        b.bits = (1 << 0) | (1 << 5) | (1 << 7);
        assert_eq!(a.is_any_equal(&b), false);
    }

    #[test]
    fn test_andeq_not() {
        let mut a = setup();
        a.andeq_not(&a.clone());
        assert_eq!(a.bits, 0);
    }

    #[test]
    fn test_expand() {
        let mut a = setup();
        a.expand();
        // [1, 1, 1]
        // [1, 1, 1]
        // [1, 1, 1]
        let expected = (1 << 9) - 1;
        assert_eq!(a.bits, expected);
    }

    #[test]
    fn test_right() {
        let a = setup();
        let actual = a.right();
        // [0, 0, 1],
        // [0, 1, 0],
        // [0, 0, 0],
        let expected = (1 << 2) | (1 << 4);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_left() {
        let a = setup();
        let actual = a.left();
        // [1, 0, 0],
        // [0, 0, 0],
        // [0, 1, 0],
        let expected = (1 << 0) | (1 << 7);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_down() {
        let a = setup();
        let actual = a.down();
        // [0, 0, 0],
        // [0, 1, 0],
        // [1, 0, 0],
        let expected = (1 << 4) | (1 << 6);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_up() {
        let a = setup();
        let up = a.up();
        // [1, 0, 0],
        // [0, 0, 1],
        // [0, 0, 0],
        let expected = (1 << 0) | (1 << 5);
        assert_eq!(up, expected);
    }

    #[test]
    fn test_right_mask() {
        let a = setup();
        let actual = a.right_mask();
        // 1 1 0
        // 1 1 0
        // 1 1 0
        assert_eq!(1, 1 << 0);
        let expected =
            (1 << 0) | (1 << 1) | (1 << 3) | (1 << 4) | (1 << 6) | (1 << 7);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_left_mask() {
        let a = setup();
        let actual = a.left_mask();
        // 0 1 1
        // 0 1 1
        // 0 1 1
        let expected =
            (1 << 1) | (1 << 2) | (1 << 4) | (1 << 5) | (1 << 7) | (1 << 8);
        assert_eq!(actual, expected);
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

        assert_eq!(a.get(1, 0), true);
        a.del(1, 0);
        assert_eq!(a.get(1, 0), false);
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
        assert_eq!(a.get(1, 0), true);
    }

    #[test]
    fn make_mat() {
        let mat = setup();
        assert_eq!(mat.height, 3);
        assert_eq!(mat.bits, 266);
    }
}
