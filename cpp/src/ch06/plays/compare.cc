#include "src/util.h"
#include "src/ch06/maze_state.h"
#include "src/ch06/random_action.h"
#include "src/ch06/game.h"
#include "src/ch06/monte_carlo.h"
#include "src/ch06/mcts_alternate.h"
#include "src/ch06/duct.h"

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
                [](const State &state, int player)
                { return random_action(state, player); },
                [](const State &state, int player)
                { return random_action(state, player); },
            },
        },
        ActionName{
            .name = "monte carlo 100 vs random",
            .action_funcs = {
                [](const State &state, int player)
                { return primitive_monte_carlo_action(state, player, 100); },
                [](const State &state, int player)
                { return random_action(state, player); },
            },
        },
        ActionName{
            .name = "monte carlo 100 vs monte carlo 10",
            .action_funcs = {
                [](const State &state, int player)
                { return primitive_monte_carlo_action(state, player, 100); },
                [](const State &state, int player)
                { return primitive_monte_carlo_action(state, player, 10); },
            },
        },
        ActionName{
            .name = "mcts vs monte carlo 100",
            .action_funcs = {
                [](const State &state, int player)
                { return alternate::mcts_action(state, player, 100); },
                [](const State &state, int player)
                { return primitive_monte_carlo_action(state, player, 100); },
            },
        },
        ActionName{
            .name = "mcts vs monte carlo 1000",
            .action_funcs = {
                [](const State &state, int player)
                { return alternate::mcts_action(state, player, 1000); },
                [](const State &state, int player)
                { return primitive_monte_carlo_action(state, player, 1000); },
            },
        },
        ActionName{
            .name = "duct vs monte carlo 100",
            .action_funcs = {
                [](const State &state, int player)
                { return duct::duct_action(state, player, 100); },
                [](const State &state, int player)
                { return primitive_monte_carlo_action(state, player, 100); },
            },
        },
        ActionName{
            .name = "duct vs monte carlo 1000",
            .action_funcs = {
                [](const State &state, int player)
                { return duct::duct_action(state, player, 1000); },
                [](const State &state, int player)
                { return primitive_monte_carlo_action(state, player, 1000); },
            },
        },
        ActionName{
            .name = "mcts vs duct 100",
            .action_funcs = {
                [](const State &state, int player)
                { return alternate::mcts_action(state, player, 100); },
                [](const State &state, int player)
                { return duct::duct_action(state, player, 100); },
            },
        },
        ActionName{
            .name = "mcts vs duct 1000",
            .action_funcs = {
                [](const State &state, int player)
                { return alternate::mcts_action(state, player, 1000); },
                [](const State &state, int player)
                { return duct::duct_action(state, player, 1000); },
            },
        },
        ActionName{
            .name = "duct vs monte carlo 3000",
            .action_funcs = {
                [](const State &state, int player)
                { return duct::duct_action(state, player, 3000); },
                [](const State &state, int player)
                { return primitive_monte_carlo_action(state, player, 3000); },
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
        float win_rate = white_games(num_games, action_name.action_funcs.data(), 10);
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
