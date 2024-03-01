use rand::{rngs::StdRng, Rng, SeedableRng};

/// constants
pub const H_: usize = 5;
pub const W_: usize = 5;
pub const END_TURN_: usize = 5;
pub const NUM_CHARACTERS_: usize = 3;

/// type for actions to implement
pub type ActionFunc = dyn Fn(&AutoMoveMazeState) -> usize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Character {
    y: usize,
    x: usize,
}

impl Character {
    pub fn new() -> Self {
        Character { y: 0, x: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutoMoveMazeState {
    characters: Vec<Character>,
    points: Vec<Vec<usize>>, // dim[H][W]
    turn: usize,
    game_score: usize,
    evaluated_score: usize,
    // originally const but make the params
    height: usize,
    width: usize,
    end_turn: usize,
    num_characters: usize,
}

impl AutoMoveMazeState {
    const DX: [i8; 4] = [1, -1, 0, 0];
    const DY: [i8; 4] = [0, 0, 1, -1];

    pub fn new(
        seed: u64,
        height: usize,
        width: usize,
        num_characters: usize,
        end_turn: usize,
    ) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut points: Vec<Vec<usize>> = vec![vec![0; width]; height];
        for y in 0..height {
            for x in 0..width {
                points[y][x] = rng.gen_range(0..=9);
            }
        }

        let characters = vec![Character::new(); num_characters];

        AutoMoveMazeState {
            points,
            turn: 0,
            game_score: 0,
            evaluated_score: 0,
            characters,
            //
            height,
            width,
            end_turn,
            num_characters,
        }
    }

    pub fn set_character(&mut self, id: usize, y: usize, x: usize) {
        self.characters[id].y = y;
        self.characters[id].x = x;
    }

    fn advance(&mut self) {
        for id in 0..self.num_characters {
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
        let character = &mut self.characters[id];
        let mut best_point = 0;
        let mut best_action = 0;

        for action in 0..4 {
            let ty = character.y as isize + Self::DY[action] as isize;
            let tx = character.x as isize + Self::DX[action] as isize;
            if ty >= 0
                && ty < self.height as isize
                && tx >= 0
                && tx < self.width as isize
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
        character.y =
            (character.y as isize + Self::DY[best_action] as isize) as usize;
        character.x =
            (character.x as isize + Self::DX[best_action] as isize) as usize;
    }

    fn is_done(&self) -> bool {
        // implement
        self.turn >= self.end_turn
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
    fn test_make_state() {
        let state = AutoMoveMazeState::new(0, 3, 3, 1, 2);
        assert_eq!(state.turn, 0);
    }

    #[test]
    fn test_make_character() {
        let character = Character::new();
        assert_eq!(character.x, 0);
        assert_eq!(character.y, 0);
    }
}
