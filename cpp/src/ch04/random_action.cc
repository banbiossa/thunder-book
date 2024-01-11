#include "random_action.h"

State random_action(const State &state)
{
    State now_state = state;
    for (int character_id = 0; character_id < CHARACTER_N; character_id++)
    {
        int y = mt_for_action() % H;
        int x = mt_for_action() % W;

        now_state.set_character(character_id, y, x);
    }
    return now_state;
}

int main()
{
    const auto &ai = StringAIPair(
        "random_action", [&](const State &state)
        { return random_action(state); });
    play_game(ai, 0);
    return 0;
}
