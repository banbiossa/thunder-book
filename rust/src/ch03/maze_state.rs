use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// constants
const H: usize = 3;
const W: usize = 4;
const END_TURN: usize = 4;

/// base struct holds state of game
#[derive(Debug)]
pub struct NumberCollectingGame {
    pub character: Character,
    pub game_score: usize,
    // dims points[H][W]
    pub points: Vec<Vec<u32>>,
    turn: usize,
}

#[derive(Debug, PartialEq)]
pub struct Character {
    y: usize, // y coming first is important
    x: usize,
    mark: String,
}

impl Character {
    pub fn new(x: usize, y: usize, mark: String) -> Character {
        Character { x, y, mark }
    }
}

impl NumberCollectingGame {
    const DX: [i8; 4] = [1, -1, 0, 0];
    const DY: [i8; 4] = [0, 0, 1, -1];

    pub fn new(initial_seed: u8) -> NumberCollectingGame {
        // seed the random generator (needs 32 bits)
        let seed: [u8; 32] = (initial_seed..)
            .take(32)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|_| panic!("Failed to create seed array"));
        let mut rng = StdRng::from_seed(seed);

        // make character
        let character = Character {
            y: rng.gen_range(0..H),
            x: rng.gen_range(0..W),
            mark: String::from("A"),
        };

        // make points, if character is there skip
        let mut points: Vec<Vec<u32>> = vec![vec![0; W]; H];
        for y in 0..H {
            for x in 0..W {
                if character.y == y && character.x == x {
                    continue;
                }
                points[y][x] = rng.gen_range(0..=9);
            }
        }

        NumberCollectingGame {
            character,
            game_score: 0,
            points,
            turn: 0,
        }
    }

    /// checks if the game is done
    pub fn is_done(&self) -> bool {
        self.turn >= END_TURN
    }

    // /// moves game one action forward
    pub fn advance(&mut self, action: i32) {
        //
    }

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
        let character = Character::new(0, 0, String::from("A"));
        assert!(character.x == 0);
    }
}
