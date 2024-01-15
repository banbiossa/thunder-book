#include "iterative_deepening.h"
#include "time_keeper.h"
#include "alpha_beta.h"

int iterative_deepening_action(
    const State &state,
    const int64_t time_threshold)
{
    auto time_keeper = TimeKeeper(time_threshold);
    int best_action = 0;

    for (int depth = 1;; depth++)
    {
        int action = alpha_beta_action(state, depth, time_keeper);
        if (time_keeper.is_time_over())
            break;
        best_action = action;
    }
    return best_action;
}
