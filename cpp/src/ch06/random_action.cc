#include <random>
#include "random_action.h"

int random_action(const State &state, const int player_id)
{
    auto mt_for_action = std::mt19937(0);
    auto legal_actions = state.legal_actions(player_id);
    return legal_actions[mt_for_action() % (legal_actions.size())];
}
