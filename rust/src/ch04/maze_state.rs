use rand::{rngs::StdRng, Rng, SeedableRng};

/// constants
pub const H_: usize = 5;
pub const W_: usize = 5;
pub const END_TURN_: usize = 5;
pub const NUM_CHARACTERS_: usize = 3;

/// type for actions to implement
pub type ActionFunc = dyn Fn(&AutoMoveMazeState) -> AutoMoveMazeState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Character {
    y: usize,
    x: usize,
}

impl Character {
    pub fn new(y: usize, x: usize) -> Self {
        Character { y, x }
    }
}

// originally const but make the params
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MazeParams {
    pub height: usize,
    pub width: usize,
    pub end_turn: usize,
    pub num_characters: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutoMoveMazeState {
    characters: Vec<Character>,
    points: Vec<Vec<usize>>, // dim[H][W]
    turn: usize,
    game_score: usize,
    evaluated_score: usize,
    // originally const but make the params
    pub params: MazeParams,
}

impl AutoMoveMazeState {
    const DX: [i8; 4] = [1, -1, 0, 0];
    const DY: [i8; 4] = [0, 0, 1, -1];

    pub fn new(seed: u64, params: MazeParams) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut points: Vec<Vec<usize>> =
            vec![vec![0; params.width]; params.height];
        for y in 0..params.height {
            for x in 0..params.width {
                points[y][x] = rng.gen_range(0..=9);
            }
        }

        let characters = vec![Character::new(0, 0); params.num_characters];

        AutoMoveMazeState {
            points,
            turn: 0,
            game_score: 0,
            evaluated_score: 0,
            characters,
            params,
        }
    }

    pub fn set_character(&mut self, id: usize, y: usize, x: usize) {
        self.characters[id].y = y;
        self.characters[id].x = x;
    }

    fn advance(&mut self) {
        for id in 0..self.params.num_characters {
            self.move_character(id);
        }
        for character in &self.characters {
            let point = self.points[character.y][character.x];
            self.game_score += point;
            self.points[character.y][character.x] = 0;
        }
        self.turn += 1;
    }

    fn move_character(&mut self, id: usize) {
        // greedy
        let character = &self.characters[id];
        let mut best_point = 0;
        let mut best_action = 0;

        for action in 0..4 {
            let ty = character.y as isize + Self::DY[action] as isize;
            let tx = character.x as isize + Self::DX[action] as isize;
            if ty >= 0
                && ty < self.params.height as isize
                && tx >= 0
                && tx < self.params.width as isize
            {
                // >=0 so can be usize
                let ty = ty as usize;
                let tx = tx as usize;
                let point = self.points[ty][tx];
                if point > best_point {
                    best_point = point;
                    best_action = action;
                }
            }
        }

        // only mut here
        let ty =
            (character.y as isize + Self::DY[best_action] as isize) as usize;
        let tx =
            (character.x as isize + Self::DX[best_action] as isize) as usize;
        self.set_character(id, ty, tx);
    }

    fn is_done(&self) -> bool {
        // implement
        self.turn >= self.params.end_turn
    }

    pub fn to_string(&self) -> String {
        let mut ss = String::from("");
        ss += &format!("turn:\t{}\n", self.turn);
        ss += &format!("score:\t{}\n", self.game_score);

        // implement

        ss
    }

    pub fn get_score(&self, print: bool) -> usize {
        let mut state = self.clone();

        // remove points on characters
        for character in &self.characters {
            state.points[character.y][character.x] = 0;
        }

        // move till end
        while !state.is_done() {
            state.advance();
            if print {
                println!("{}", self.to_string());
            }
        }

        state.game_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_score_large() {
        let params = MazeParams {
            height: 10,
            width: 10,
            end_turn: 30,
            num_characters: 5,
        };
        let state = AutoMoveMazeState::new(0, params);

        // should go down 1
        let score = state.get_score(true);
        assert_eq!(score, 2);
    }

    #[test]
    fn test_get_score_small() {
        let params = MazeParams {
            height: 3,
            width: 3,
            end_turn: 1,
            num_characters: 2,
        };
        let state = AutoMoveMazeState::new(0, params);

        // should go down 1
        let score = state.get_score(true);
        assert_eq!(score, 2);
    }

    #[test]
    fn test_advance() {
        let params = MazeParams {
            height: 3,
            width: 3,
            end_turn: 1,
            num_characters: 2,
        };
        let mut state = AutoMoveMazeState::new(0, params);

        // should go down 1
        state.advance();
        let actual = &state.characters;
        let expected = vec![Character::new(1, 0), Character::new(1, 0)];
        assert_eq!(actual, &expected);
        assert_eq!(state.game_score, 2);
    }

    #[test]
    fn test_move_character() {
        let params = MazeParams {
            height: 3,
            width: 3,
            end_turn: 1,
            num_characters: 2,
        };
        let mut state = AutoMoveMazeState::new(0, params);
        // should go down 1
        state.move_character(0);
        let actual = &state.characters;
        let expected = vec![Character::new(1, 0), Character::new(0, 0)];
        assert_eq!(actual, &expected);
    }

    #[test]
    fn test_set_character() {
        let params = MazeParams {
            height: 3,
            width: 3,
            end_turn: 1,
            num_characters: 2,
        };
        let mut state = AutoMoveMazeState::new(0, params);
        let actual = &state.characters;
        let expected = vec![Character::new(0, 0), Character::new(0, 0)];
        assert_eq!(actual, &expected);

        state.set_character(0, 2, 2);
        let actual = &state.characters;
        let expected = vec![Character::new(2, 2), Character::new(0, 0)];
        assert_eq!(actual, &expected);
    }

    #[test]
    fn test_points() {
        // test to understand the structure
        let params = MazeParams {
            height: 3,
            width: 3,
            end_turn: 1,
            num_characters: 2,
        };
        let state = AutoMoveMazeState::new(0, params);
        let actual = state.points;
        let expected = vec![vec![7, 0, 2], vec![2, 7, 1], vec![1, 0, 4]];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_to_string() {
        let params = MazeParams {
            height: 3,
            width: 3,
            end_turn: 1,
            num_characters: 2,
        };
        let state = AutoMoveMazeState::new(0, params);
        let expected = "turn:\t0\nscore:\t0\n";
        let actual = state.to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_make_state() {
        let params = MazeParams {
            height: 3,
            width: 3,
            end_turn: 1,
            num_characters: 2,
        };
        let state = AutoMoveMazeState::new(0, params);
        assert_eq!(state.turn, 0);
    }

    #[test]
    fn test_make_character() {
        let character = Character::new(0, 0);
        assert_eq!(character.x, 0);
        assert_eq!(character.y, 0);
    }
}
