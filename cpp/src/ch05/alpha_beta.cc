#include "alpha_beta.h"

ScoreType alpha_beta_score(const State &state,
                           ScoreType alpha,
                           const ScoreType beta,
                           const int depth)
{
    if (state.is_done() || depth == 0)
        return state.get_score();

    auto legal_actions = state.legal_actions();
    if (legal_actions.empty())
        return state.get_score();

    for (const auto action : legal_actions)
    {
        State next_state = state;
        next_state.advance(action);

        ScoreType score = -alpha_beta_score(next_state, -beta, -alpha, depth - 1);

        if (score > alpha)
            alpha = score;
        if (alpha >= beta)
            return alpha;
    }
    return alpha;
}

int alpha_beta_action(const State &state, const int depth)
{
    ScoreType best_action = -1;
    ScoreType alpha = -INF;
    ScoreType beta = INF;
    for (const auto action : state.legal_actions())
    {
        State next_state = state;
        next_state.advance(action);
        ScoreType score = -alpha_beta_score(next_state, -beta, -alpha, depth);
        if (score > alpha)
        {
            best_action = action;
            alpha = score;
        }
    }
    return best_action;
}
