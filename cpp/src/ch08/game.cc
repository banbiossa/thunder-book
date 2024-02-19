#include <iostream>
#include <memory>
#include "game.h"

using std::cout;
using std::endl;

double play_game(AIFunction actions_wb[2], bool should_print)
{
    auto state = ConnectFourStateNormal();
    if (should_print)
        cout << state.to_string() << endl;

    int player = 0;
    while (!state.is_done())
    {
        auto legal_actions = state.legal_actions();
        auto action_func = actions_wb[player];
        state.advance(action_func(state));

        if (should_print)
            cout << state.to_string() << endl;

        player ^= 1; // change player
    }
    return state.white_score();
}

double many_games(AIFunction actions_wb[2],
                  int num_games,
                  int print_every)
{
    double total = 0;
    for (int i = 0; i < num_games; i++)
    {
        total += play_game(actions_wb, false);

        if (print_every > 0 && i % print_every == 0)
            cout << "i " << i << " w " << total / (i + 1) << endl;
    }
    return total / num_games;
}

double games_black_and_white(AIFunction actions_wb[2],
                             int num_games,
                             int print_every)
{
    AIFunction actions_bw[2] = {actions_wb[1], actions_wb[0]};
    double total = 0;

    cout << "white" << endl;
    total += many_games(actions_wb, num_games, print_every);

    cout << "black" << endl;
    total += 1 - many_games(actions_bw, num_games, print_every);

    return total / 2;
}

double play_game_with_state(AIFunction actions_wb[2],
                            StateVersion state_versions[2])
{
    int player = 0;

    // this will keep track of the actual game
    auto state = ConnectFourStateNormal();

    while (!state.is_done())
    {
        auto state_explore = create_state(state_versions[player], state);
        auto action_func = actions_wb[player];
        int best_action = action_func(*state_explore);
        state.advance(best_action);

        player ^= 1; // change player
    }
    return state.white_score();
}

double many_games_with_state(AIFunction actions_wb[2],
                             StateVersion state_versions[2],
                             int num_games,
                             int print_every)
{
    double total = 0;
    for (int i = 0; i < num_games; i++)
    {
        total += play_game_with_state(actions_wb, state_versions);

        if (print_every > 0 && i % print_every == 0)
            cout << "i " << i << " w " << total / (i + 1) << endl;
    }
    return total / num_games;
}

double games_black_and_white_with_state(AIFunction actions_wb[2],
                                        StateVersion state_versions[2],
                                        int num_games,
                                        int print_every)
{
    AIFunction actions_bw[2] = {actions_wb[1], actions_wb[0]};
    StateVersion state_versions_bw[2] = {state_versions[1],
                                         state_versions[0]};
    double total = 0;

    cout << "white" << endl;
    total += many_games_with_state(actions_wb, state_versions, num_games, print_every);

    cout << "black" << endl;
    total += 1 - many_games_with_state(actions_bw,
                                       state_versions_bw,
                                       num_games,
                                       print_every);

    return total / 2;
}
