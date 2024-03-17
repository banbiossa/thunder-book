use crate::base::state::MazeParams;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mat {
    bits: usize,
    height: usize,
    width: usize,
}

impl Mat {
    pub fn new(params: &MazeParams, bits: usize) -> Self {
        Mat {
            bits,
            height: params.height,
            width: params.width,
        }
    }

    pub fn get(&self, y: usize, x: usize) -> bool {
        self.bits & (1 << (y * self.width + x)) != 0
    }

    pub fn set(&mut self, y: usize, x: usize) {
        self.bits |= 1 << (y * self.width + x)
    }

    pub fn del(&mut self, y: usize, x: usize) {
        self.bits &= !(1 << (y * self.width + x))
    }

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

    fn setup() -> Mat {
        let params = MazeParams {
            height: 3,
            width: 3,
            end_turn: 3,
        };
        let bits = (1 << 1) | (1 << 3) | (1 << 8);
        // [0, 1, 0],
        // [1, 0, 0],
        // [0, 0, 1],
        Mat::new(&params, bits)
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
