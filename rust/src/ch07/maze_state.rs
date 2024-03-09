use std::sync::Arc;

use rand::{rngs::StdRng, Rng, SeedableRng};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Character {
    y: usize,
    x: usize,
}

impl Character {
    pub fn new(params: &MazeParams, seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        Character {
            y: rng.gen_range(0..params.height),
            x: rng.gen_range(0..params.width),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MazeParams {
    pub height: usize,
    pub width: usize,
    pub end_turn: usize,
}

pub type ActionFunc = Arc<dyn Fn(&WallMazeState) -> usize>;

#[derive(Debug, Clone)]
pub struct WallMazeState {
    walls: Vec<Vec<usize>>,
    points: Vec<Vec<usize>>,
    first_action: Option<usize>,
    character: Character,
    params: MazeParams,
    pub turn: usize,
    pub evaluated_score: usize,
    pub game_score: usize,
}

impl WallMazeState {
    const DX: [isize; 4] = [1, -1, 0, 0];
    const DY: [isize; 4] = [0, 0, 1, -1];

    pub fn new(seed: u64, params: MazeParams) -> Self {
        let character = Character::new(&params, seed);
        let walls = Self::init_wall(&params, &character, seed);
        let points = Self::init_points(&params, &character, seed);

        WallMazeState {
            walls,
            evaluated_score: 0,
            first_action: None,
            character,
            points,
            params,
            turn: 0,
            game_score: 0,
        }
    }

    fn init_points(
        params: &MazeParams,
        character: &Character,
        seed: u64,
    ) -> Vec<Vec<usize>> {
        // make points
        let mut rng = StdRng::seed_from_u64(seed);
        let mut points = vec![vec![0; params.width]; params.height];
        for y in 0..params.height {
            for x in 0..params.width {
                if y == character.y && x == character.x {
                    continue;
                }
                points[y][x] = rng.gen_range(0..=9);
            }
        }
        points
    }

    fn init_wall(
        params: &MazeParams,
        character: &Character,
        seed: u64,
    ) -> Vec<Vec<usize>> {
        // create walls with bar-collapse method
        let mut rng = StdRng::seed_from_u64(seed);
        let mut walls = vec![vec![0; params.width]; params.height];
        for y in (1..params.height).step_by(2) {
            for x in (1..params.width).step_by(2) {
                let mut tx = x;
                let mut ty = y;
                if ty == character.y && tx == character.x {
                    continue;
                }
                walls[ty][tx] = 1;

                // 最初だけ上も候補, 他は右左下
                let direction_size = if y == 1 { 4 } else { 3 };
                let direction = rng.gen_range(0..direction_size);
                ty = (ty as isize + Self::DY[direction]) as usize;
                tx = (tx as isize + Self::DX[direction]) as usize;
                if ty == character.y && tx == character.x {
                    continue;
                }
                walls[ty][tx] = 1;
            }
        }
        walls
    }

    pub fn legal_actions(&self) -> Vec<usize> {
        (0..4)
            .filter(|&action| {
                let ty = self.character.y as isize + Self::DY[action];
                let tx = self.character.x as isize + Self::DX[action];
                ty >= 0
                    && (ty as usize) < self.params.height
                    && tx >= 0
                    && (tx as usize) < self.params.width
                    && self.walls[ty as usize][tx as usize] == 0
            })
            .collect()
    }

    pub fn is_done(&self) -> bool {
        self.turn >= self.params.end_turn
    }

    pub fn evaluate_score(&mut self) {
        self.evaluated_score = self.game_score;
    }

    /// utility to show state of game
    pub fn to_string(&self) -> String {
        let mut ss = String::from("");

        ss += &format!("turn:\t{}\n", self.turn);
        ss += &format!("score:\t{}\n", self.game_score);
        for y in 0..self.params.height {
            ss += "\n";
            for x in 0..self.params.width {
                if self.character.y == y && self.character.x == x {
                    ss += "@";
                } else if self.walls[y][x] == 1 {
                    ss += "#";
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

    /// moves game one action forward
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_params() -> MazeParams {
        MazeParams {
            height: 5,
            width: 5,
            end_turn: 3,
        }
    }

    fn setup() -> WallMazeState {
        let params = setup_params();
        WallMazeState::new(0, params)
    }

    #[test]
    fn test_advance() {
        let mut state = setup();
        state.advance(1);
        assert_eq!(state.character, Character { x: 0, y: 0 });
        assert_eq!(state.turn, 1);
        assert_eq!(state.game_score, 7);
    }

    #[test]
    fn test_to_string() {
        let state = setup();
        let actual = state.to_string();
        let expected = "\
turn:\t0
score:\t0

7@.22
7#1##
49251
8#5##
9#665
";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_init_points() {
        let params = setup_params();
        let character = Character::new(&params, 0);
        let points = WallMazeState::init_points(&params, &character, 0);
        let expected = [
            [7, 0, 0, 2, 2],
            [7, 1, 1, 0, 4],
            [4, 9, 2, 5, 1],
            [8, 2, 5, 6, 2],
            [9, 0, 6, 6, 5],
        ];
        assert_eq!(points, expected);
    }

    #[test]
    fn test_init_wall() {
        let params = setup_params();
        let character = Character::new(&params, 0);
        let wall = WallMazeState::init_wall(&params, &character, 0);
        let expected = [
            [0, 0, 0, 0, 0],
            [0, 1, 0, 1, 1],
            [0, 0, 0, 0, 0],
            [0, 1, 0, 1, 1],
            [0, 1, 0, 0, 0],
        ];
        assert_eq!(wall, expected);
    }

    #[test]
    fn test_make_character() {
        let params = setup_params();
        let character = Character::new(&params, 0);
        assert_eq!(character.y, 0);
        assert_eq!(character.x, 1);
    }

    #[test]
    fn test_legal_actions() {
        let state = setup();
        let actual = state.legal_actions();
        let expected = vec![0, 1];
        assert_eq!(actual, expected);
    }

    #[test]
    fn make_state() {
        let state = setup();
        assert_eq!(state.walls[1][1], 1);
    }
}
