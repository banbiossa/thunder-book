#include <queue>
#include "beam_search.h"

int beam_search_action(const MazeState &state, const int beam_width, const int beam_depth)
{
    MazeState best_state;

    std::priority_queue<MazeState> now_beam;
    now_beam.push(state);

    for (int t = 0; t < beam_depth; t++)
    {
        std::priority_queue<MazeState> next_beam;
        for (int i = 0; i < beam_width; i++)
        {
            if (now_beam.empty())
            {
                break;
            }
            MazeState now_state = now_beam.top();
            now_beam.pop();
            auto legal_actions = now_state.legal_actions();
            for (const auto &action : legal_actions)
            {
                MazeState next_state = now_state;
                next_state.advance(action);
                next_state.evaluate_score();
                if (t == 0)
                {
                    next_state.first_action_ = action;
                }
                next_beam.push(next_state);
            }
        }
        now_beam = next_beam;
        best_state = now_beam.top();

        if (best_state.is_done())
        {
            break;
        }
    }
    return best_state.first_action_;
}
