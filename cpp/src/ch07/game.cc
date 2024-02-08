#include <iostream>
#include "game.h"

void play_game(AIFunction action_func, const int seed)
{
    using std::cout;
    using std::endl;

    std::string action_to_str[4] = {"RIGHT", "LEFT", "DOWN", "UP"};

    auto state = State(seed);
    cout << state.to_string() << endl;

    while (!state.is_done())
    {
        cout << "turn " << state.turn_ << endl;

        int action = action_func(state);
        cout << "action " << action << " "
             << action_to_str[action] << endl;
        state.advance(action);
        cout << state.to_string() << endl;
    }
}

double many_games(AIFunction action_func, int num_games)
{
    double total = 0;
    for (int i = 0; i < num_games; i++)
    {
        auto state = State(i);
        while (!state.is_done())
        {
            state.advance(action_func(state));
        }
        total += state.game_score_;
    }
    return total / (double)num_games;
}
