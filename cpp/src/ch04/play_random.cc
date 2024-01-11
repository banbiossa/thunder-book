#include "random_action.h"

int main()
{
    const auto &ai = StringAIPair(
        "random_action", [&](const State &state)
        { return random_action(state); });
    play_game(ai, 0);
    return 0;
}
