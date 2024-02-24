use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// constants
const H: usize = 3;
const W: usize = 4;
const END_TURN: usize = 4;

/// base struct holds state of game
#[derive(Debug)]
pub struct NumberCollectingGame {
    //
    points: Vec<Vec<u32>>,
    turn: usize,
}

#[derive(Debug, PartialEq)]
struct Character {
    x: u32,
    y: u32,
    mark: String,
}

impl Character {
    pub fn new(x: u32, y: u32, mark: String) -> Character {
        Character { x, y, mark }
    }
}

impl NumberCollectingGame {
    pub fn new(seed: u8) -> NumberCollectingGame {
        // seed the random generator (needs 32 bits)
        let initial_seed: u8 = seed; // Starting seed value
        let seed: [u8; 32] = (initial_seed..)
            .take(32)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|_| panic!("Failed to create seed array"));
        let mut rng = StdRng::from_seed(seed);

        let mut points: Vec<Vec<u32>> = vec![vec![0; W]; H];
        for y in 0..H {
            for x in 0..W {
                points[y][x] = rng.gen_range(0..=9);
            }
        }

        NumberCollectingGame { points, turn: 0 }
    }

    /// checks if the game is done
    pub fn is_done(&self) -> bool {
        self.turn >= END_TURN
    }

    // /// moves game one action forward
    // pub fn advance(action: int) {
    //     //
    // }

    // /// actions that can be taken at that step
    // pub fn legal_actions() -> Vec<u32> {
    //     //
    // }

    // /// utility to show state of game
    // pub fn to_string() -> &str {
    //     //
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_state() {
        let state = NumberCollectingGame::new(0);
        assert!(state.points[0][0] < 10);
        assert!(state.points.len() == H);
        assert!(state.points[0].len() == W);
    }

    #[test]
    fn is_done_works() {
        let mut state = NumberCollectingGame::new(0);
        assert!(!state.is_done());
        state.turn = END_TURN;
        assert!(state.is_done());
    }

    #[test]
    fn create_character() {
        let character = Character {
            x: 0,
            y: 0,
            mark: String::from("A"),
        };
    }
}
