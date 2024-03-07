use crate::base::game_result;
use crate::base::state::State;
use rand::{rngs::StdRng, Rng, SeedableRng};

#[derive(Debug, Clone)]
pub struct MazeParams {
    pub height: usize,
    pub width: usize,
    pub end_turn: usize,
}

#[derive(Debug, Clone)]
pub struct Character {
    y: usize,
    x: usize,
    game_point: usize,
    pub mark: String,
}

impl Character {
    pub fn new(y: usize, x: usize, mark: &str) -> Self {
        Character {
            y,
            x,
            game_point: 0,
            mark: mark.to_string(),
        }
    }
}

pub type ActionFunc = dyn Fn(&AlternateMazeState) -> usize;

#[derive(Debug, Clone)]
pub struct AlternateMazeState {
    points: Vec<Vec<usize>>,
    pub characters: Vec<Character>,
    pub turn: usize,
    params: MazeParams,
}

impl State for AlternateMazeState {
    type Action = usize;

    fn advance(&mut self, action: Self::Action) {
        let character = &mut self.characters[0];
        character.y = (character.y as isize + Self::DY[action]) as usize;
        character.x = (character.x as isize + Self::DX[action]) as usize;
        let point = self.points[character.y][character.x];
        character.game_point += point;
        self.points[character.y][character.x] = 0;
        self.turn += 1;
        self.characters.swap(0, 1);
    }

    fn is_done(&self) -> bool {
        self.turn >= self.params.end_turn
    }

    fn white_score(&self) -> game_result::GameResult {
        let mut point = self.teban_point();

        // 後手番なら入れ替える
        if self.characters[0].mark == "B" {
            point = -point;
        }

        game_result::GameResult::new(point)
    }
}

impl AlternateMazeState {
    const DX: [isize; 4] = [1, -1, 0, 0];
    const DY: [isize; 4] = [0, 0, 1, -1];

    pub fn new(seed: u64, params: MazeParams) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let characters = vec![
            Character::new(params.height / 2, params.width / 2 - 1, "A"),
            Character::new(params.height / 2, params.width / 2 + 1, "B"),
        ];

        // init points
        let mut points = vec![vec![0; params.width]; params.height];
        for y in 0..params.height {
            for x in 0..params.width {
                points[y][x] = rng.gen_range(0..=9);
            }
        }
        // remove points on characters
        for character in &characters {
            points[character.y][character.x] = 0
        }

        AlternateMazeState {
            points,
            characters,
            turn: 0,
            params,
        }
    }

    // 評価値
    pub fn evaluation(&self) -> isize {
        self.characters[0].game_point as isize
            - self.characters[1].game_point as isize
    }

    // [0, 1] の評価値 (主にthunder search用)
    // todo: 他も全て入れ替えちゃう
    pub fn evaluation_rate(&self) -> f32 {
        let p0 = self.characters[0].game_point as f32;
        let p1 = self.characters[1].game_point as f32;
        if p0 + p1 == 0.0 {
            return 0.;
        }
        p0 / (p0 + p1)
    }

    pub fn legal_actions(&self) -> Vec<usize> {
        let mut actions = Vec::new();
        let character = &self.characters[0];
        for action in 0..4 {
            let ty = character.y as isize + Self::DY[action];
            let tx = character.x as isize + Self::DX[action];
            if ty >= 0
                && (ty as usize) < self.params.height
                && tx >= 0
                && (tx as usize) < self.params.width
            {
                actions.push(action);
            }
        }
        actions
    }

    pub fn to_string(&self) -> String {
        let mut ss = String::from("");
        ss += &format!("turn:\t{}\n", self.turn);
        ss += &format!("point:\t{}\n", self.teban_point());
        for h in 0..self.params.height {
            ss += "\n";
            for w in 0..self.params.width {
                let mut is_written = false;

                // both in same place
                if self.characters[0].y == h
                    && self.characters[1].y == h
                    && self.characters[0].x == w
                    && self.characters[1].x == w
                {
                    ss += "@";
                    is_written = true;
                }

                // each in their place
                if !is_written {
                    for character in &self.characters {
                        if character.y == h && character.x == w {
                            ss += &character.mark;
                            is_written = true;
                        }
                    }
                }
                if !is_written {
                    if self.points[h][w] > 0 {
                        ss += &format!("{}", self.points[h][w]);
                    } else {
                        ss += ".";
                    }
                }
            }
        }
        ss += "\n";
        ss
    }

    // isize because can be negative
    pub fn teban_point(&self) -> isize {
        self.characters[0].game_point as isize
            - self.characters[1].game_point as isize
    }

    pub fn teban_score(&self) -> game_result::GameResult {
        let point = self.teban_point();
        game_result::GameResult::new(point)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn setup() -> AlternateMazeState {
        let params = MazeParams {
            height: 3,
            width: 3,
            end_turn: 4,
        };
        AlternateMazeState::new(0, params)
    }

    #[test]
    fn test_evaluation_rate() {
        let mut state = setup();
        assert_eq!(state.evaluation_rate(), 0.0);
        state.advance(0);
        assert_eq!(state.evaluation_rate(), 0.0);
        state.advance(1);
        assert_eq!(state.evaluation_rate(), 1.0);
    }

    #[test]
    fn test_teban_score() {
        let mut state = setup();
        state.advance(0);
        let actual = state.teban_score();
        let expected = game_result::GameResult::new(-7);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_evaluation() {
        let mut state = setup();
        let actual = state.evaluation();
        let expected = 0;
        assert_eq!(actual, expected);

        let legal_actions = state.legal_actions();
        let action = legal_actions[0];
        state.advance(action);
        let actual = state.evaluation();
        let expected = -7;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_to_string() {
        let mut state = setup();
        let actual = state.to_string();
        let expected = "\
turn:\t0
point:\t0

7.2
A7B
1.4
";
        assert_eq!(actual, expected);

        // move to same place
        state.characters[0].x = 2;
        let actual = state.to_string();
        let expected = "\
turn:\t0
point:\t0

7.2
.7@
1.4
";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_white_score() {
        let mut state = setup();
        let legal_actions = state.legal_actions();
        let action = legal_actions[0];
        state.advance(action);

        let actual = state.white_score();
        assert_eq!(actual.message, "A wins");
        assert_eq!(actual.score, 1.0);
    }

    #[test]
    fn test_teban_point() {
        let mut state = setup();
        let legal_actions = state.legal_actions();
        let action = legal_actions[0];
        state.advance(action);
        assert_eq!(state.teban_point(), -7);
    }

    #[test]
    fn test_advance() {
        let mut state = setup();
        let legal_actions = state.legal_actions();
        let action = legal_actions[0];
        state.advance(action);
        assert_eq!(state.turn, 1);
        assert_eq!(state.characters[0].mark, "B");
        assert_eq!(state.characters[0].game_point, 0);
        assert_eq!(state.characters[1].game_point, 7);
    }

    #[test]
    fn test_legal_actions() {
        let state = setup();
        let actual = state.legal_actions();
        let expected = vec![0, 2, 3];
        assert_eq!(actual, expected);
    }

    #[test]
    fn make_character() {
        let character = Character::new(3 / 2, 0, "a");
        // check int division
        assert_eq!(character.y, 1);
        assert_eq!(character.mark, String::from("a"));
    }

    #[test]
    fn make_state() {
        let state = setup();
        assert_eq!(state.turn, 0);
    }
}
