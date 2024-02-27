use search::ch03::beam_search;
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
    let num_games = 100;

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
    ];

    for pair in action_funcs {
        let average = game::average(pair.action_func, num_games, 10);
        println!(
            "average {average} of {} over num_games {num_games}",
            pair.name,
        );
    }
}
