#include <queue>
#include "beam_search.h"

int beam_search_action(const State &state,
                       const int beam_width,
                       const int beam_depth)
{
    State best_state = state;

    std::priority_queue<State> now_beam;
    now_beam.push(state);

    for (int d = 0; d < beam_depth; d++)
    {
        std::priority_queue<State> next_beam;
        for (int w = 0; w < beam_width; w++)
        {
            if (now_beam.empty())
                break;
            State now_state = now_beam.top();
            now_beam.pop();
            auto legal_actions = now_state.legal_actions();
            for (const auto &action : legal_actions)
            {
                State next_state = now_state;
                next_state.advance(action);
                next_state.evaluate_score();
                if (d == 0)
                    next_state.first_action_ = action;
                next_beam.push(next_state);
            }
        }
        now_beam = next_beam;
        best_state = now_beam.top();
        if (best_state.is_done())
            break;
    }
    return best_state.first_action_;
}
