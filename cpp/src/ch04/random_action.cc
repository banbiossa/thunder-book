#include <random>
#include "random_action.h"

auto mt_for_action_2 = std::mt19937(0);

State random_action(const State &state)
{
    State now_state = state;
    for (int character_id = 0; character_id < CHARACTER_N; character_id++)
    {
        int y = mt_for_action_2() % H;
        int x = mt_for_action_2() % W;

        now_state.set_character(character_id, y, x);
    }
    return now_state;
}
