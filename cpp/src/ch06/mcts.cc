#include "random_action.h"
#include "mcts.h"

double playout(AlternateMazeState *state)
{
    if (state->is_done())
        return state->white_score();

    state->advance(random_action(*state));
    return 1. - playout(state);
}
