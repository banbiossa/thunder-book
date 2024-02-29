use std::cmp::Ordering;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// constants
pub const H: usize = 3;
pub const W: usize = 4;
pub const END_TURN: usize = 4;

/// type for actions to implement
pub type ActionFunc = dyn Fn(&NumberCollectingGame) -> usize;

/// base struct holds state of game
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberCollectingGame {
    /**
    *  Args:
           first_action: first action, will be set during explore
    */
    pub character: Character,
    pub game_score: usize,
    // dims points[H][W]
    pub points: Vec<Vec<usize>>,
    pub turn: usize,
    pub evaluated_score: usize,
    pub first_action: Option<usize>,
}

impl PartialOrd for NumberCollectingGame {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NumberCollectingGame {
    fn cmp(&self, other: &Self) -> Ordering {
        self.evaluated_score.cmp(&other.evaluated_score)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
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

    pub fn new(seed: u64) -> NumberCollectingGame {
        let mut rng = StdRng::seed_from_u64(seed);

        // make character
        let character = Character {
            y: rng.gen_range(0..H),
            x: rng.gen_range(0..W),
            mark: String::from("A"),
        };

        // make points, if character is there skip
        let mut points: Vec<Vec<usize>> = vec![vec![0; W]; H];
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
            evaluated_score: 0,
            first_action: None,
        }
    }

    /// checks if the game is done
    pub fn is_done(&self) -> bool {
        self.turn >= END_TURN
    }

    /// evaluate score
    pub fn evaluate_score(&mut self) {
        self.evaluated_score = self.game_score;
    }

    // /// moves game one action forward
    pub fn advance(&mut self, action: usize) {
        let character = &mut self.character;
        character.y =
            (character.y as isize + Self::DY[action] as isize) as usize;
        character.x =
            (character.x as isize + Self::DX[action] as isize) as usize;
        let point = self.points[character.y][character.x];
        self.game_score += point;
        self.points[character.y][character.x] = 0;
        self.turn += 1;
    }

    /// actions that can be taken at that step
    pub fn legal_actions(&self) -> Vec<usize> {
        let mut actions = Vec::new();
        for action in 0..4 as usize {
            let ty = self.character.y as isize + Self::DY[action] as isize;
            let tx = self.character.x as isize + Self::DX[action] as isize;
            if ty >= 0 && ty < H as isize && tx >= 0 && tx < W as isize {
                actions.push(action);
            }
        }
        actions
    }

    /// utility to show state of game
    pub fn to_string(&self) -> String {
        let mut ss = String::from("");

        ss += &format!("turn:\t{}\n", self.turn);
        ss += &format!("score:\t{}\n", self.game_score);
        for y in 0..H {
            ss += "\n";
            for x in 0..W {
                if self.character.y == y && self.character.x == x {
                    ss += "@";
                } else if self.points[y][x] > 0 {
                    ss += &format!("{}", self.points[y][x]);
                } else {
                    ss += ".";
                }
            }
        }
        ss += "\n";

        ss
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ordering() {
        let state_small = NumberCollectingGame::new(0);
        let mut state_big = NumberCollectingGame::new(1);

        state_big.evaluated_score = 3;
        assert!(state_small < state_big);
    }

    #[test]
    fn to_string() {
        let state = NumberCollectingGame::new(0);
        let actual = state.to_string();
        let expected = "\
turn:\t0
score:\t0

.227
11.4
492@
";
        if H == 3 && W == 4 {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn legal_actions() {
        let state = NumberCollectingGame::new(0);
        let legal_actions = state.legal_actions();
        assert!(legal_actions.len() > 0);
    }

    #[test]
    fn advance_moves_character() {
        let mut state = NumberCollectingGame::new(0);
        let legal_actions = state.legal_actions();
        let action = legal_actions[0];
        let character_before = state.character.clone();
        state.advance(action);
        assert_eq!(state.turn, 1);
        assert_ne!(state.character, character_before);
        let actual = state.to_string();
        let expected = "\
turn:\t1
score:\t2

.227
11.4
49@.
";
        if H == 3 && W == 4 {
            assert_eq!(actual, expected);
        }
    }

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
