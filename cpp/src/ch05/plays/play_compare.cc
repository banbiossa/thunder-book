// compare all the plays, similar to ch05_compare.rs

#include "src/ch05/win_rate.h"
#include "src/ch05/iterative_deepening.h"
#include "src/ch05/mini_max.h"
#include "src/ch05/alpha_beta.h"
#include "src/ch05/random_action.h"
#include "src/ch05/monte_carlo.h"
#include "src/ch05/monte_carlo_tree_search.h"
#include "src/ch05/thunder_search.h"
#include "src/util.h"

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
            .name = "random vs random",
            .action_funcs = {
                [](const State &state)
                { return random_action(state); },
                [](const State &state)
                { return random_action(state); },
            },
        },
        ActionName{
            .name = "mini max vs random",
            .action_funcs = {
                [](const State &state)
                { return mini_max_action(state, END_TURN); },
                [](const State &state)
                { return random_action(state); },
            },

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
        ActionName{
            .action_funcs = {
                [](const State &state)
                { return iterative_deepening_action(state, 2); },
                [](const State &state)
                {
                    return iterative_deepening_action(state, 1);
                },
            },
            .name = "iterative deepening 2ms vs. 1ms",
        },
        ActionName{
            .name = "monte carlo 3000 vs. random",
            .action_funcs = {
                [](const State &state)
                { return primitive_monte_carlo_action(state, 3000); },
                random_action,
            },
        },
        ActionName{
            .name = "monte carlo 3000 vs. monte carlo 30",
            .action_funcs = {
                [](const State &state)
                { return primitive_monte_carlo_action(state, 3000); },
                [](const State &state)
                { return primitive_monte_carlo_action(state, 30); },
            },
        },
        ActionName{
            .name = "mcts 3000 vs monte carlo 3000",
            .action_funcs = {
                [](const State &state)
                {
                    return mcts_action(state, 3000);
                },
                [](const State &state)
                {
                    return primitive_monte_carlo_action(state, 3000);
                },
            },
        },
        ActionName{
            .name = "mcts 3000 vs mcts 30",
            .action_funcs = {
                [](const State &state)
                {
                    return mcts_action(state, 3000);
                },
                [](const State &state)
                {
                    return mcts_action(state, 30);
                },
            },
        },
        ActionName{
            .name = "thunder vs mcts 3000",
            .action_funcs = {
                [](const State &state)
                {
                    return thunder::thunder_search_action(state, 3000);
                },
                [](const State &state)
                {
                    return mcts_action(state, 3000);
                },

            },
        },
        ActionName{
            .name = "thunder vs mcts 1ms",
            .action_funcs = {
                [](const State &state)
                {
                    return thunder::thunder_search_action_with_timekeeper(state, 1);
                },
                [](const State &state)
                {
                    return mcts_action_with_time_threshold(state, 1);
                },
            },
        },
        ActionName{
            .name = "thunder vs iterative deepening 1ms",
            .action_funcs = {
                [](const State &state)
                {
                    return thunder::thunder_search_action_with_timekeeper(state, 1);
                },
                [](const State &state)
                {
                    return iterative_deepening_action(state, 1);
                },
            },
        },
        //
    };

    log_to_file("| action | win_rate% | time |");
    log_to_file("| ------ | --------- | ---- |");
    int num_games = 1000;
    for (auto action_name : action_names)
    {
        cout << action_name.name << endl;
        auto start = std::chrono::system_clock::now();
        float win_rate = games_black_and_white(num_games, action_name.action_funcs.data(), 10);
        auto elapsed_in_ms = std::chrono::duration_cast<std::chrono::milliseconds>(
                                 std::chrono::system_clock::now() - start)
                                 .count();
        log_to_file("| %s | %.2f%% | %.2fs |",
                    action_name.name.c_str(),
                    win_rate * 100,
                    elapsed_in_ms / 1000.0);
    }

    return 0;
}
