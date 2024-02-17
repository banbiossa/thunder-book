#include <random>
#include "random_action.h"

// can set seed to time based if you want
std::mt19937 mt_for_action(0);

int random_action(const ConnectFourState &state)
{
    auto legal_actions = state.legal_actions();
    return legal_actions[mt_for_action() % legal_actions.size()];
}
