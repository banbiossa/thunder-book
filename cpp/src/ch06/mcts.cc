#include "mcts.h"

double playout(AlternateMazeState *state)
{
    if (state->is_done())
    {
        return state.white_score();
    }
}
