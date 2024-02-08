#include <iostream>
#include "game.h"

void play_game(AIFunction action_func, const int seed)
{
    using std::cout;
    using std::endl;

    auto state = State(seed);
    cout << state.to_string() << endl;
    while (!state.is_done())
    {
        state.advance(action_func(state));
        cout << state.to_string() << endl;
    }
}
