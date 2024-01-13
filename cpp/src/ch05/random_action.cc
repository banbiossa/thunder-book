#include <random>
#include "random_action.h"

auto mt_for_action = std::mt19937(0);

int random_action(const State &state)
{
    auto legal_actions = state.legal_actions();
    return legal_actions[mt_for_action() % legal_actions.size()];
}

void play_game(const int seed)
{
    using std::cout;
    using std::endl;
    using State = AlternateMazeState;

    auto state = State(seed);
    cout << state.to_string() << endl;

    // player
    int p = 0;
    while (!state.is_done())
    {
        cout << (p + 1) << "p -------------------" << endl;
        int action = random_action(state);
        cout << "action " << action << endl;
        state.advance(action);
        cout << state.to_string() << endl;
        p ^= 1; // same as p = (p + 1) % 2;
    }
    state.print_end_game();
}
