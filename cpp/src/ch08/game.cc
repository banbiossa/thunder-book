#include <iostream>
#include "game.h"

namespace
{
    using std::cout;
    using std::endl;

    void print(const ConnectFourState &state)
    {
        cout << state.to_string() << endl;
    }
}

void play_game(AIFunction action_wb[2])
{
    auto state = ConnectFourState();
    print(state);

    int player = 0;
    while (!state.is_done())
    {
        auto legal_actions = state.legal_actions();
        auto action_func = action_wb[player];
        state.advance(action_func(state));

        print(state);
        player ^= 1; // change player
    }
}
