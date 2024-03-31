#include <memory>
#include <iostream>
#include "game.h"

using std::cout;
using std::endl;

double play_game(AIFunction actions_wb[2],
                 StateVersion state_versions[2],
                 bool should_print)
{
    std::array<std::unique_ptr<ConnectFourState>, 2> states = {
        get_state(state_versions[0]),
        get_state(state_versions[1])};
    // auto state = ConnectFourState();

    if (should_print)
        cout << states[0]->to_string() << endl;

    int player = 0;
    while (!states[0]->is_done())
    {
        int action;
        if (player == 0)
            action = actions_wb[player](states[0]);
        else
            action = actions_wb[player](states[1]);

        states[0]->advance(action);
        states[1]->advance(action);

        if (should_print)
            cout << states[0]->to_string() << endl;

        player ^= 1; // change player
    }
    return states[0]->white_score();
}

double many_games(AIFunction actions_wb[2],
                  StateVersion state_versions[2],
                  int num_games,
                  int print_every)
{
    double total = 0;
    for (int i = 0; i < num_games; i++)
    {
        total += play_game(actions_wb, state_versions, false);

        if (print_every > 0 && i % print_every == 0)
            cout << "i " << i << " w " << total / (i + 1) << endl;
    }
    return total / num_games;
}

double games_black_and_white(AIFunction actions_wb[2],
                             StateVersion state_versions[2],
                             int num_games,
                             int print_every)
{
    AIFunction actions_bw[2] = {actions_wb[1], actions_wb[0]};
    double total = 0;

    cout << "white" << endl;
    total += many_games(actions_wb, state_versions, num_games, print_every);

    cout << "black" << endl;
    total += 1 - many_games(actions_bw, state_versions, num_games, print_every);

    return total / 2;
}
