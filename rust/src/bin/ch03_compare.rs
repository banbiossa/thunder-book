use search::ch03::beam_search;
use search::ch03::chokudai;
use search::ch03::game;
use search::ch03::greedy;
use search::ch03::maze_state;
use search::ch03::random_action;

/** compare random, greedy, beam_search
 *
 */

struct ActionNamePair {
    action_func: Box<maze_state::ActionFunc>,
    name: String,
}

fn main() {
    //
    let num_games = 10;

    let beam_width = 10;
    let beam_depth = 10;

    let action_funcs = vec![
        ActionNamePair {
            action_func: Box::new(random_action::random_action),
            name: "random".to_string(),
        },
        ActionNamePair {
            action_func: Box::new(greedy::greedy_action),
            name: "greedy".to_string(),
        },
        ActionNamePair {
            action_func: beam_search::beam_search_factory(
                beam_width, beam_depth,
            ),
            name: format!(
                "beam_search - width: {beam_width}, depth: {beam_depth} "
            ),
        },
        ActionNamePair {
            action_func: beam_search::beam_search_timed_factory(beam_width, 1),
            name: format!("beam search - width: {beam_width}, time: 1ms"),
        },
        ActionNamePair {
            action_func: beam_search::beam_search_timed_factory(beam_width, 10),
            name: format!("beam search - width: {beam_width}, time: 10ms"),
        },
        ActionNamePair {
            action_func: chokudai::chokudai_search_factory(
                1,
                maze_state::END_TURN,
                2,
            ),
            name: format!("chokudai search - width: 1, 2 beams"),
        },
        ActionNamePair {
            action_func: chokudai::chokudai_search_timed_factory(
                1,
                maze_state::END_TURN,
                1,
            ),
            name: format!("chokudai search - width: 1, 1ms"),
        },
    ];

    for pair in action_funcs.into_iter().rev() {
        println!("do {}", pair.name);
        let average = game::average(pair.action_func, num_games, 10);
        println!(
            "average {average} of {} over num_games {num_games}",
            pair.name,
        );
    }
}
