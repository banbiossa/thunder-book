use bitvec::prelude::*;

#[derive(Debug, Clone)]
pub struct Mat {
    bits: Vec<BitArray>,
}

impl Mat {
    pub fn new(other: &Mat) -> Self {
        Mat {
            bits: other.bits.clone(),
        }
    }

    pub fn get(&self, y: usize, x: usize) -> bool {
        self.bits[y][x]
    }

    pub fn set(&mut self, y: usize, x: usize) {
        self.bits[y].set(x, true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_set() {
        let mut a = Mat {
            bits: vec![bitarr![0, 1, 0]],
        };
        assert_eq!(a.get(0, 0), false);
        a.set(0, 0);
        assert_eq!(a.get(0, 0), true);
    }

    #[test]
    fn test_get() {
        let a = Mat {
            bits: vec![bitarr![0, 1, 0]],
        };
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
        let a = Mat {
            bits: vec![bitarr![0, 1, 0]],
        };
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
