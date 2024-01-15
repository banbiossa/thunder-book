#include "alpha_beta.h"
#include "time_keeper.h"

ScoreType alpha_beta_score(const State &state,
                           ScoreType alpha,
                           const ScoreType beta,
                           const int depth,
                           const TimeKeeper &time_keeper)
{
    // if time_over, we won't use the score anyway
    if (time_keeper.is_time_over())
        return alpha;

    if (state.is_done() || depth == 0)
        return state.get_score();

    auto legal_actions = state.legal_actions();
    if (legal_actions.empty())
        return state.get_score();

    for (const auto action : legal_actions)
    {
        State next_state = state;
        next_state.advance(action);

        ScoreType score = -alpha_beta_score(next_state, -beta, -alpha, depth - 1, time_keeper);

        if (score > alpha)
            alpha = score;
        if (alpha >= beta)
            return alpha;
    }
    return alpha;
}

int alpha_beta_action(const State &state,
                      const int depth,
                      const TimeKeeper &time_keeper)
{
    ScoreType best_action = 0; // set default as 0 for time_over case
    ScoreType alpha = -INF;
    ScoreType beta = INF;
    for (const auto action : state.legal_actions())
    {
        State next_state = state;
        next_state.advance(action);
        ScoreType score = -alpha_beta_score(next_state, -beta, -alpha, depth, time_keeper);
        if (score > alpha)
        {
            best_action = action;
            alpha = score;
        }
        if (time_keeper.is_time_over())
            break;
    }
    return best_action;
}
