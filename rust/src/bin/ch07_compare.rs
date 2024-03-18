use std::time::Instant;

use search::base::state::{ActionFunc, MazeParams, SinglePlayerState};
use search::ch03::game;
use search::ch03::{beam_search, random_action};
use search::ch07::beam_search::{
    beam_search_hash_box, beam_search_hash_timed_box,
};
use search::ch07::bitstate::BitsetState;
use search::ch07::maze_state::WallMazeState;
use search::ch07::multi_bit::MultiBit;
use search::ch07::near_state::NearPointState;
use search::ch07::zobrist_hash::ZobristState;

/** compare random, greedy, beam_search
 *
 */

struct ActionNamePair<T: SinglePlayerState> {
    action_func: ActionFunc<T>,
    name: String,
}

fn main() {
    pub const PARAMS: MazeParams = MazeParams {
        height: 7,
        width: 7,
        end_turn: 49,
    };
    let beam_width = 10;
    let beam_depth = PARAMS.end_turn;

    compare::<WallMazeState>(
        vec![
            ActionNamePair {
                action_func: random_action::random_action_box(),
                name: "random".to_string(),
            },
            ActionNamePair {
                action_func: beam_search::beam_search_factory(
                    beam_width, beam_depth,
                ),
                name: format!(
                    "beam_search - width: {beam_width}, depth: {beam_depth} "
                ),
            },
        ],
        PARAMS,
    );

    compare::<NearPointState>(
        vec![
            ActionNamePair {
                action_func: beam_search::beam_search_factory(
                    beam_width, beam_depth,
                ),
                name: "beam search near point state".to_string(),
            },
            ActionNamePair {
                action_func: random_action::random_action_box(),
                name: "random".to_string(),
            },
        ],
        PARAMS,
    );

    compare::<ZobristState>(
        vec![
            ActionNamePair {
                action_func: beam_search_hash_box(beam_width, beam_depth),
                name: "zobrist hash beam search".to_string(),
            },
            ActionNamePair {
                action_func: random_action::random_action_box(),
                name: "random".to_string(),
            },
        ],
        PARAMS,
    );
    compare::<BitsetState<MultiBit>>(
        vec![
            ActionNamePair {
                action_func: beam_search_hash_box(beam_width, beam_depth),
                name: "multi bit state hash beam search".to_string(),
            },
            ActionNamePair {
                action_func: random_action::random_action_box(),
                name: "random (MultiBitState)".to_string(),
            },
        ],
        PARAMS,
    );

    // with time threshold
    let time_threshold_ms = 1;
    compare::<NearPointState>(
        vec![ActionNamePair {
            action_func: beam_search::beam_search_timed_factory(
                beam_width,
                time_threshold_ms,
            ),
            name: format!("beam search near point state {time_threshold_ms}ms"),
        }],
        PARAMS,
    );
    compare::<ZobristState>(
        vec![ActionNamePair {
            action_func: beam_search_hash_timed_box(
                beam_width,
                time_threshold_ms,
            ),
            name: format!("zoobrist hash beam search {time_threshold_ms}ms"),
        }],
        PARAMS,
    );
}

fn compare<T: SinglePlayerState>(
    action_funcs: Vec<ActionNamePair<T>>,
    params: MazeParams,
) {
    let num_games = 100;
    for pair in action_funcs.into_iter().rev() {
        let start = Instant::now();
        let average =
            game::average(params.clone(), pair.action_func, num_games, 0);
        let elapsed = start.elapsed().as_secs_f32();
        println!("average: {average}\ttime: {:.2}s\t{}", elapsed, pair.name,);
    }
}
