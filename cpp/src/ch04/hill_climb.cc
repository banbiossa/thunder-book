#include "hill_climb.h"

State hill_climb(const State &state, int number)
{
    State now_state = state;
    now_state.init();
    ScoreType best_score = now_state.get_score();
    for (int i = 0; i < number; i++)
    {
        auto next_state = now_state;
        next_state.transition();
        auto next_score = next_state.get_score();
        if (next_score > best_score)
        {
            best_score = next_score;
            now_state = next_state;
        }
    }
    return now_state;
}
