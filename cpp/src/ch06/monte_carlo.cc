#include "monte_carlo.h"
#include "random_action.h"

double playout(State *state)
{
    if (state->is_done())
        return state->white_score();

    // recurse
    state->advance(random_action(*state, 0),
                   random_action(*state, 1));
    return playout(state);
}

int primitive_monte_carlo_action(const State &state,
                                 const int player_id,
                                 const int playout_number)
{
    auto legal_actions = state.legal_actions(player_id);
    int opp_id = player_id ^ 1;

    double best_value = -INF;
    int best_action_index = -1;

    for (int i = 0; i < (int)legal_actions.size(); i++)
    {
        double value = 0;
        for (int j = 0; j < playout_number; j++)
        {
            State next_state = state;
            int opp_action = random_action(state, opp_id);
            // todo: make this cooler (swap?)
            if (player_id == 0)
                next_state.advance(legal_actions[i], opp_action);
            else
                next_state.advance(opp_action, legal_actions[i]);

            double white_score = playout(&next_state);
            double score = (player_id == 0 ? white_score : 1. - white_score);
            value += score;
        }
        if (value > best_value)
        {
            best_action_index = i;
            best_value = value;
        }
    }
    return legal_actions[best_action_index];
}
