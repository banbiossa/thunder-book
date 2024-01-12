#include "hill_climb.h"

int main()
{
    const auto &ai = StringAIPair("hill_climb", [&](const State &state)
                                  { return hill_climb(state, 10000); });
    play_game(ai, 0);
    return 0;
}
