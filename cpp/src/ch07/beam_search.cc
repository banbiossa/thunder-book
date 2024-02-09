#include <queue>
#include "beam_search.h"

int beam_search_action(const State &initial_state,
                       const int beam_width,
                       const int beam_depth)
{
    State best_state = initial_state;

    std::priority_queue<State> beam;
    beam.push(initial_state);

    for (int d = 0; d < beam_depth; d++)
    {
        std::priority_queue<State> next_beam;
        for (int w = 0; w < beam_width; w++)
        {
            if (beam.empty())
                break;
            State state = beam.top();
            beam.pop();
            auto legal_actions = state.legal_actions();
            for (const auto &action : legal_actions)
            {
                State next_state = state;
                next_state.advance(action);
                next_state.evaluate_score();
                if (d == 0)
                    next_state.first_action_ = action;
                next_beam.push(next_state);
            }
        }
        beam = next_beam;
        best_state = beam.top();
        if (best_state.is_done())
            break;
    }
    return best_state.first_action_;
}
