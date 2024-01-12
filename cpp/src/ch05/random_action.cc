#include <random>
#include "random_action.h"

auto mt_for_action = std::mt19937(0);

int random_action(const State &state)
{
    auto legal_actions = state.legal_actions();
    return legal_actions[mt_for_action() % legal_actions.size()];
}
