use std::time::Instant;
use std::vec;

use search::base::alternate::{AlternateState, MazeParams};
use search::ch05::game;
use search::ch05::maze_state::ActionFunc;
use search::ch05::mcts;
use search::ch05::random_action;
use search::ch08::bitstate::BitsetConnectFour;
use search::ch08::maze_state::ConnectFourState;

struct ActionNamePair<T: AlternateState> {
    action_funcs: Vec<ActionFunc<T>>,
    name: String,
}

fn main() {
    pub const PARAMS: MazeParams = MazeParams {
        height: 6,
        width: 7,
        end_turn: 0, // not necessary
    };
    pub const MCTS_PARAMS: mcts::MCTSParams = mcts::MCTSParams {
        c: 1.0,
        expand_threshold: 10,
    };

    println!("| win % | time | name |");
    println!("| ------- | ---- | ---- |");

    compare(
        vec![
            ActionNamePair {
                action_funcs: vec![
                    mcts::mcts_timebound_arc::<ConnectFourState>(
                        1,
                        MCTS_PARAMS.clone(),
                    ),
                    random_action::random_action_arc(),
                ],
                name: format!("mcts 1ms vs random"),
            },
            ActionNamePair {
                action_funcs: vec![
                    random_action::random_action_arc(),
                    random_action::random_action_arc(),
                ],
                name: format!("random vs random"),
            },
        ],
        PARAMS,
    );

    compare(
        vec![
            ActionNamePair {
                action_funcs: vec![
                    mcts::mcts_timebound_arc::<BitsetConnectFour>(
                        1,
                        MCTS_PARAMS.clone(),
                    ),
                    random_action::random_action_arc(),
                ],
                name: format!("bitset mcts 1ms vs random"),
            },
            ActionNamePair {
                action_funcs: vec![
                    random_action::random_action_arc(),
                    random_action::random_action_arc(),
                ],
                name: format!("bitset random vs random"),
            },
        ],
        PARAMS,
    );
}

fn compare<T: AlternateState>(
    action_name_pairs: Vec<ActionNamePair<T>>,
    params: MazeParams,
) {
    let num_games = 100;
    for pair in action_name_pairs {
        let start = Instant::now();
        let average = game::play_black_white(
            params.clone(),
            pair.action_funcs,
            num_games,
            0,
        );
        let elapsed = start.elapsed().as_secs_f32();
        println!(
            "| {:.1}% | {:.2}s | {} |",
            average * 100.0,
            elapsed,
            pair.name,
        );
    }
}
