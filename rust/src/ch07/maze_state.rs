use crate::{
    base::state::{
        single_player_state_portrait, Character, MazeParams, SinglePlayerState,
    },
    ch03::maze_state::NumberCollectingGame,
};

use rand::{rngs::StdRng, Rng, SeedableRng};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WallMazeState {
    state: NumberCollectingGame,
    walls: Vec<Vec<usize>>,
}

impl PartialOrd for WallMazeState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for WallMazeState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_evaluated_score().cmp(&other.get_evaluated_score())
    }
}

#[portrait::fill(portrait::delegate(NumberCollectingGame; self.state))]
impl SinglePlayerState for WallMazeState {
    fn new(seed: u64, params: MazeParams) -> Self {
        let state = NumberCollectingGame::new(seed, params.clone());
        let walls = Self::init_wall(&params, state.get_character(), seed);

        WallMazeState { state, walls }
    }

    fn legal_actions(&self) -> Vec<usize> {
        (0..4)
            .filter(|&action| {
                let ty = self.get_character().y as isize + Self::DY[action];
                let tx = self.get_character().x as isize + Self::DX[action];
                ty >= 0
                    && (ty as usize) < self.get_params().height
                    && tx >= 0
                    && (tx as usize) < self.get_params().width
                    && self.walls[ty as usize][tx as usize] == 0
            })
            .collect()
    }

    /// utility to show state of game
    fn to_string(&self) -> String {
        let turn = self.get_turn();
        let game_score = self.get_game_score();
        let params = self.get_params();
        let character = self.get_character();
        let points = self.get_points();
        let walls = self.get_walls();

        let mut ss = String::from("");

        ss += &format!("turn:\t{}\n", turn);
        ss += &format!("score:\t{}\n", game_score);
        for y in 0..params.height {
            ss += "\n";
            for x in 0..params.width {
                if character.y == y && character.x == x {
                    ss += "@";
                } else if walls[y][x] == 1 {
                    ss += "#";
                } else if points[y][x] > 0 {
                    ss += &format!("{}", points[y][x]);
                } else {
                    ss += ".";
                }
            }
        }
        ss += "\n";

        ss
    }
}

impl WallMazeState {
    pub const DX: [isize; 4] = [1, -1, 0, 0];
    pub const DY: [isize; 4] = [0, 0, 1, -1];

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

    pub fn get_walls(&self) -> &Vec<Vec<usize>> {
        &self.walls
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::state::ActionFunc;
    use crate::ch03::beam_search;
    use crate::ch03::game;

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
    fn test_play_game() {
        let params = setup_params();
        let action_func: ActionFunc<WallMazeState> =
            beam_search::beam_search_factory(3, 3);
        let actual = game::play_game(params, action_func, 0, true);
        assert!(actual > 0);
        // assert_eq!(actual, 0);
    }

    #[test]
    fn test_advance() {
        let mut state = setup();
        state.advance(1);
        assert_eq!(
            state.get_character(),
            &Character {
                x: 0,
                y: 0,
                mark: "A".to_string()
            }
        );
        assert_eq!(state.get_turn(), 1);
        assert_eq!(state.get_game_score(), 2);
    }

    #[test]
    fn test_to_string() {
        let state = setup();
        let actual = state.to_string();
        let expected = "\
turn:\t0
score:\t0

2@711
.#4##
51825
6#9##
6#735
";
        assert_eq!(actual, expected);
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
