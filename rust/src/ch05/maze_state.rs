use rand::{rngs::StdRng, Rng, SeedableRng};

#[derive(Debug, Clone)]
pub struct MazeParams {
    height: usize,
    width: usize,
    end_turn: usize,
}

#[derive(Debug, Clone)]
pub struct Character {
    y: usize,
    x: usize,
    game_score: usize,
    mark: String,
}

impl Character {
    pub fn new(y: usize, x: usize, mark: &str) -> Self {
        Character {
            y,
            x,
            game_score: 0,
            mark: mark.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AlternateMazeState {
    points: Vec<Vec<usize>>,
    characters: Vec<Character>,
    turn: usize,
    params: MazeParams,
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

    pub fn is_done(&self) -> bool {
        self.turn >= self.params.end_turn
    }

    pub fn advance(&mut self, action: usize) {
        let character = &mut self.characters[0];
        character.y = (character.y as isize + Self::DY[action]) as usize;
        character.x = (character.x as isize + Self::DX[action]) as usize;
        let point = self.points[character.y][character.x];
        character.game_score += point;
        self.points[character.y][character.x] = 0;
        self.turn += 1;
        self.characters.swap(0, 1);
    }

    pub fn legal_actions(&self) -> Vec<usize> {
        let actions = Vec::new();

        actions
    }

    pub fn to_string(&self) -> String {
        let ss = String::from("");
        ss
    }

    pub fn white_score(&self) -> f32 {
        0.5
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
