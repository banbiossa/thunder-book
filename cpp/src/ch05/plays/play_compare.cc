// compare all the plays, similar to ch05_compare.rs

#include "src/ch05/win_rate.h"
#include "src/ch05/iterative_deepening.h"
#include "src/ch05/mini_max.h"
#include "src/ch05/alpha_beta.h"

using namespace std;

struct ActionName
{
    std::vector<AIFunction> action_funcs;
    string name;
};

int main()
{
    // make the above into a struct or tuple to allow calls in a loop
    std::vector<ActionName> action_names = {
        ActionName{
            .action_funcs = {
                [](const State &state)
                { return iterative_deepening_action(state, 10); },
                [](const State &state)
                {
                    return iterative_deepening_action(state, 1);
                },
            },
            .name = "iterative deepening 10ms vs. 1ms",
        },
        ActionName{
            .action_funcs = {
                [](const State &state)
                { return mini_max_action(state, END_TURN); },
                [](const State &state)
                { return alpha_beta_action(state, END_TURN); },
            },
            .name = "mini max vs. alpha-beta",
        },
        //
    };

    cout << "| action | win_rate% | time |" << endl;
    cout << "| ------ | --------- | ---- |" << endl;
    for (auto action_name : action_names)
    {
        auto start = std::chrono::system_clock::now();
        float win_rate = games_black_and_white(100, action_name.action_funcs.data(), 0);
        auto elapsed_in_ms = std::chrono::duration_cast<std::chrono::milliseconds>(
                                 std::chrono::system_clock::now() - start)
                                 .count();
        cout << "| " << action_name.name << " | " << win_rate * 100
             << "% | " << elapsed_in_ms / 1000.0 << "s |" << endl;
    }

    return 0;
}

// let action_names = vec![
//     // minimax, alpha beta
//     ActionName {
//         action_funcs: vec![
//             mini_max::mini_max_arc(depth),
//             random_action::random_action_arc(),
//         ],
//         name: format!("mini max depth {depth} vs. random"),
//     },
//     ActionName {
//         action_funcs: vec![
//             random_action::random_action_arc(),
//             random_action::random_action_arc(),
//         ],
//         name: format!("random vs random"),
//     },
//     ActionName {
//         action_funcs: vec![
//             mini_max::mini_max_arc(depth),
//             mini_max::mini_max_arc(depth),
//         ],
//         name: format!("mini max depth {depth} vs. mini max depth {depth}"),
//     },
//     ActionName {
//         action_funcs: vec![
//             mini_max::mini_max_arc(depth),
//             alpha_beta::alpha_beta_arc(depth),
//         ],
//         name: format!(
//             "mini max depth {depth} vs. alpha-beta depth {depth}"
//         ),
//     },
//     // iterative deepening
//     ActionName {
//         action_funcs: vec![
//             iterative_deepening::iterative_deepening_action_arc(1),
//             iterative_deepening::iterative_deepening_action_arc(1),
//         ],
//         name: format!("iterative deepening 1ms vs. 1ms"),
//     },
//     ActionName {
//         action_funcs: vec![
//             iterative_deepening::iterative_deepening_action_arc(2),
//             iterative_deepening::iterative_deepening_action_arc(1),
//         ],
//         name: format!("iterative deepening 2ms vs. 1ms"),
//     },
//     ActionName {
//         action_funcs: vec![
//             iterative_deepening::iterative_deepening_action_arc(5),
//             iterative_deepening::iterative_deepening_action_arc(1),
//         ],
//         name: format!("iterative deepening 5ms vs. 1ms"),
//     },
//     // monte_carlo
//     ActionName {
//         action_funcs: vec![
//             monte_carlo::monte_carlo_action_arc(num_playout),
//             random_action::random_action_arc(),
//         ],
//         name: format!("monte_carlo num_playout {num_playout} vs random"),
//     },
//     ActionName {
//         action_funcs: vec![
//             monte_carlo::monte_carlo_action_arc(30),
//             random_action::random_action_arc(),
//         ],
//         name: format!("monte_carlo num_playout 30 vs random"),
//     },
//     ActionName {
//         action_funcs: vec![
//             monte_carlo::monte_carlo_action_arc(num_playout),
//             monte_carlo::monte_carlo_action_arc(30),
//         ],
//         name: format!("monte_carlo num_playout {num_playout} vs 30"),
//     },
//     // mcts
//     ActionName {
//         action_funcs: vec![
//             mcts::mcts_action_arc(num_playout, MCTS_PARAMS),
//             monte_carlo::monte_carlo_action_arc(num_playout),
//         ],
//         name: format!(
//             "mcts {num_playout} vs monte_carlo num_playout {num_playout}"
//         ),
//     },
//     ActionName {
//         action_funcs: vec![
//             mcts::mcts_action_arc(num_playout, MCTS_PARAMS),
//             mcts::mcts_action_arc(30, MCTS_PARAMS),
//         ],
//         name: format!("mcts num_playout {num_playout} vs 30"),
//     },
//     // thunder
//     ActionName {
//         action_funcs: vec![
//             thunder::thunder_search_arc(num_playout),
//             mcts::mcts_action_arc(num_playout, MCTS_PARAMS),
//         ],
//         name: format!("thunder vs. mcts num_playout {num_playout}"),
//     },
//     ActionName {
//         action_funcs: vec![
//             thunder::thunder_timebound_arc(1),
//             mcts::mcts_timebound_arc(1, MCTS_PARAMS),
//         ],
//         name: format!("thunder vs. mcts 1ms"),
//     },
//     ActionName {
//         action_funcs: vec![
//             thunder::thunder_timebound_arc(1),
//             iterative_deepening::iterative_deepening_action_arc(1),
//         ],
//         name: format!("thunder vs. iterative deepening 1ms"),
//     },
// ];
