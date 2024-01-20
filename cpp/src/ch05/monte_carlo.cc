#include "monte_carlo.h"
#include "random_action.h"

using std::cout;
using std::endl;

double playout(State *state)
{
    if (state->is_done())
        return state->teban_score();

    state->advance(random_action(*state));
    // win score always returns score of player A
    // so no need to do 1 - playout(state)
    return 1 - playout(state);
}

int primitive_monte_carlo_action(const State &state,
                                 int playout_number)
{
    auto legal_actions = state.legal_actions();
    auto values = std::vector<double>(legal_actions.size());
    auto cnts = std::vector<double>(legal_actions.size());
    for (int cnt = 0; cnt < playout_number; cnt++)
    {
        int index = cnt % legal_actions.size();
        State next_state = state;
        next_state.advance(legal_actions[index]);
        values[index] += 1 - playout(&next_state);
        ++cnts[index];
    }
    int best_action_index = -1;
    double best_score = -INF;
    for (int index = 0; index < legal_actions.size(); index++)
    {
        double value_mean = values[index] / cnts[index];
        if (value_mean > best_score)
        {
            best_score = value_mean;
            best_action_index = index;
        }
    }

    // print average score of each action for debugging
    /*
    auto average_score = std::vector<double>(legal_actions.size());
    cout << "average" << endl;
    for (int i = 0; i < legal_actions.size(); i++)
    {
        average_score[i] = values[i] / cnts[i];
        cout << i << " " << legal_actions[i] << " " << average_score[i] << endl;
    }

    cout << "best action " << best_action_index << " "
         << legal_actions[best_action_index] << endl
         << endl;
    */

    return legal_actions[best_action_index];
}
