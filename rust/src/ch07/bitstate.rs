use crate::base::state::{HashableState, MazeParams, SinglePlayerState, Wall};

use super::zobrist_hash::ZobristState;
use std::cmp::Ordering;

pub trait Mat: Clone + Eq + PartialEq + Ord {
    fn new(params: &MazeParams) -> Self;
    fn get(&self, y: usize, x: usize) -> bool;
    fn set(&mut self, y: usize, x: usize);
    fn del(&mut self, y: usize, x: usize);
    fn expand(&mut self);
    fn andeq_not(&mut self, other: &Self);
    fn is_any_equal(&self, other: &Self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitsetState<T: Mat> {
    state: ZobristState,
    points_mat: T,
    walls_mat: T,
}

impl<T: Mat + Ord + PartialOrd> PartialOrd for BitsetState<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Mat + Ord> Ord for BitsetState<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_evaluated_score().cmp(&other.get_evaluated_score())
    }
}

impl<T: Mat> Wall for BitsetState<T> {
    fn get_walls(&self) -> &Vec<Vec<usize>> {
        self.state.get_walls()
    }
}

impl<T: Mat> HashableState for BitsetState<T> {
    fn get_hash(&self) -> u64 {
        self.state.get_hash()
    }
    fn set_hash(&mut self, hash: u64) {
        self.state.set_hash(hash)
    }
}

impl<T: Mat> BitsetState<T> {
    pub fn get_distance_to_nearest_point(&self) -> usize {
        let mut mat = T::new(self.get_params());
        let character = self.get_character();
        mat.set(character.y, character.x);
        let params = self.get_params();
        for depth in 0..(params.height * params.width) {
            if mat.is_any_equal(&self.points_mat) {
                return depth;
            }
            let mut next = mat.clone();
            next.expand();
            next.andeq_not(&self.walls_mat);
            if next == mat {
                break;
            }
            mat = next;
        }
        // the max is the whole size of the map
        params.height * params.width
    }

    pub fn get_points_mat(&self) -> &T {
        &self.points_mat
    }
}

// #[portrait::fill(portrait::delegate(NearPointState; self.state.state))]
// portrait fill するとなぜか state.rs で Character が読めなくなる
impl<T: Mat> SinglePlayerState for BitsetState<T> {
    fn new(seed: u64, params: MazeParams) -> Self {
        let state = ZobristState::new(seed, params.clone());
        let mut points_mat = T::new(&params);
        let mut walls_mat = T::new(&params);

        for y in 0..params.height {
            for x in 0..params.width {
                if state.get_walls()[y][x] != 0 {
                    walls_mat.set(y, x);
                }
                if state.get_points()[y][x] != 0 {
                    points_mat.set(y, x);
                }
            }
        }
        BitsetState {
            state,
            points_mat,
            walls_mat,
        }
    }
    // self.point_mat の更新必要じゃない？
    fn advance(&mut self, action: usize) -> usize {
        let point = self.state.advance(action);
        let character = self.get_character();
        self.remove_points(character.y, character.x);
        point
    }
    fn remove_points(&mut self, y: usize, x: usize) {
        self.state.remove_points(y, x);
        self.points_mat.del(y, x);
    }
    fn evaluate_score(&mut self) {
        let evaluated_score = (self.get_game_score()
            * self.get_params().height
            * self.get_params().width) as isize
            - self.get_distance_to_nearest_point() as isize;
        self.set_evaluated_score(evaluated_score);
    }

    // copy from state
    fn get_evaluated_score(&self) -> isize {
        self.state.get_evaluated_score()
    }
    fn legal_actions(&self) -> Vec<usize> {
        self.state.legal_actions()
    }
    fn is_done(&self) -> bool {
        self.state.is_done()
    }
    fn to_string(&self) -> String {
        self.state.to_string()
    }
    fn get_character(&self) -> &crate::base::state::Character {
        self.state.get_character()
    }
    fn get_first_action(&self) -> usize {
        self.state.get_first_action()
    }
    fn get_game_score(&self) -> usize {
        self.state.get_game_score()
    }
    fn get_params(&self) -> &MazeParams {
        self.state.get_params()
    }
    fn get_points(&self) -> &Vec<Vec<usize>> {
        self.state.get_points()
    }
    fn get_turn(&self) -> usize {
        self.state.get_turn()
    }
    fn set_evaluated_score(&mut self, score: isize) {
        self.state.set_evaluated_score(score)
    }
    fn set_first_action(&mut self, action: usize) {
        self.state.set_first_action(action)
    }
    fn set_game_score(&mut self, score: usize) {
        self.state.set_game_score(score)
    }
    fn set_turn(&mut self, turn: usize) {
        self.state.set_turn(turn)
    }
}

// tests are made in multi_bit.rs and single_bit.rs
// where they can be constructed
