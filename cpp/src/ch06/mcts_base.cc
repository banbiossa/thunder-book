#include "mcts_base.h"
#include "random_action.h"

double Playout::playout()
{
    if (state_.is_done())
        return state_.white_score();

    state_.advance(random_action(state_, 0),
                   random_action(state_, 1));
    return playout();
}
