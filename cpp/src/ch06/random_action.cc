#include <random>
#include "random_action.h"

unsigned seed = std::chrono::system_clock::now().time_since_epoch().count();
auto mt_for_action = std::mt19937(seed);

int random_action(const State &state, const int player_id)
{
    auto legal_actions = state.legal_actions(player_id);
    return legal_actions[mt_for_action() % (legal_actions.size())];
}
