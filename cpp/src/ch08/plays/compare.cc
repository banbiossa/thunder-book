#include <iostream>
#include "src/ch08/random_action.h"
#include "src/ch08/game.h"
#include "src/ch08/mcts.h"
#include "src/util.h"

using namespace std;

struct ActionName
{
    AIFunction action_funcs[2];
    StateVersion state_versions[2];
    string name;
};

int main()
{
    int num_games = 100;
    int print_every = 10;

    std::vector<ActionName> action_names = {
        ActionName{
            .name = "mcts bitset 1ms vs random",
            .action_funcs = {
                [&](const std::unique_ptr<ConnectFourState> &state)
                { return mcts_action_timebound(state, 1); },
                random_action,
            },
            .state_versions = {StateVersion::Bitset, StateVersion::Normal},
        },
        ActionName{
            .name = "mcts bitset 1ms vs random bitset",
            .action_funcs = {
                [&](const std::unique_ptr<ConnectFourState> &state)
                { return mcts_action_timebound(state, 1); },
                random_action,
            },
            .state_versions = {StateVersion::Bitset, StateVersion::Bitset},
        },
        ActionName{
            .name = "mcts bitset 1ms vs mcts normal 1ms",
            .action_funcs = {
                [&](const std::unique_ptr<ConnectFourState> &state)
                { return mcts_action_timebound(state, 1); },
                [&](const std::unique_ptr<ConnectFourState> &state)
                { return mcts_action_timebound(state, 1); },
            },
            .state_versions = {StateVersion::Bitset, StateVersion::Normal},
        },
        ActionName{
            .name = "mcts bitset 2ms vs mcts normal 1ms",
            .action_funcs = {
                [&](const std::unique_ptr<ConnectFourState> &state)
                { return mcts_action_timebound(state, 2); },
                [&](const std::unique_ptr<ConnectFourState> &state)
                { return mcts_action_timebound(state, 1); },
            },
            .state_versions = {StateVersion::Bitset, StateVersion::Normal},
        },
        ActionName{
            .name = "random vs random",
            .action_funcs = {random_action, random_action},
            .state_versions = {StateVersion::Normal, StateVersion::Normal},
        },
        ActionName{
            .name = "random vs random bitset",
            .action_funcs = {random_action, random_action},
            .state_versions = {StateVersion::Normal, StateVersion::Bitset},
        },
        ActionName{
            .name = "mcts 1ms vs random",
            .action_funcs = {
                [&](const std::unique_ptr<ConnectFourState> &state)
                { return mcts_action_timebound(state, 1); },
                random_action,
            },
            .state_versions = {StateVersion::Normal, StateVersion::Normal},
        },
        ActionName{
            .name = "mcts 10 vs random",
            .action_funcs = {
                [&](const std::unique_ptr<ConnectFourState> &state)
                { return mcts_action(state, 10); },
                random_action,
            },
            .state_versions = {StateVersion::Normal, StateVersion::Normal},
        },
        //
    };

    log_to_file("| action | win_rate% | time |");
    log_to_file("| ------ | --------- | ---- |");
    for (auto action_name : action_names)
    {
        cout << action_name.name << endl;
        auto start = std::chrono::high_resolution_clock::now();
        double win_rate = games_black_and_white(
            action_name.action_funcs,
            action_name.state_versions,
            num_games,
            print_every);
        auto elapsed_in_ms = std::chrono::duration_cast<std::chrono::milliseconds>(
            std::chrono::high_resolution_clock::now() - start);
        log_to_file("| %s | %.2f%% | %.2fs |",
                    action_name.name.c_str(),
                    win_rate * 100,
                    elapsed_in_ms / 1000.0);
    }

    return 0;
}
