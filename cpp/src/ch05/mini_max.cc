
#include "mini_max.h"

ScoreType mini_max_score(const State &state, const int depth)
{
    if (state.is_done() || depth == 0)
    {
        return state.get_score();
    }
    auto legal_actions = state.legal_actions();
    if (legal_actions.empty())
    {
        return state.get_score();
    }
    ScoreType best_score = -INF;
    for (const auto action : legal_actions)
    {
        State next_state = state;
        next_state.advance(action);
        ScoreType score = -mini_max_score(next_state, depth - 1);
        if (score > best_score)
        {
            best_score = score;
        }
    }
    return best_score;
}

int mini_max_action(const State &state, const int depth)
{
    int best_action = -1;
    ScoreType best_score = -INF;
    for (const auto action : state.legal_actions())
    {
        State next_state = state;
        next_state.advance(action);
        ScoreType score = -mini_max_score(next_state, depth);
        if (score > best_score)
        {
            best_action = action;
            best_score = score;
        }
    }
    return best_action;
}
