#include "game.h"

void play_game(AIFunction action_f0, AIFunction action_f1, const int seed)
{
    using std::cout;
    using std::endl;

    auto state = State(seed);
    cout << state.to_string() << endl;

    while (!state.is_done())
    {
        int action0 = action_f0(state, 0);
        int action1 = action_f1(state, 1);
        state.advance(action0, action1);
    }
}
