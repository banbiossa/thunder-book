#include <queue>
#include "beam_search.h"
#include "time_keeper.h"

int beam_search_action(const MazeState &state,
                       const int beam_width,
                       const int beam_depth)
{
    MazeState best_state = state;
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

int beam_search_action_with_time_threshold(
    const MazeState &state,
    const int beam_width,
    const int64_t time_threshold)
{
    auto time_keeper = TimeKeeper(time_threshold);

    std::shared_ptr<MazeState> best_state;

    std::priority_queue<std::shared_ptr<MazeState>> now_beam;
    now_beam.push(std::make_shared<MazeState>(state));

    // loop exits after time_threshold
    for (int t = 0;; t++)
    {
        std::priority_queue<std::shared_ptr<MazeState>> next_beam;
        for (int i = 0; i < beam_width; i++)
        {
            if (time_keeper.is_time_over())
            {
                return best_state->first_action_;
            }
            if (now_beam.empty())
            {
                break;
            }
            std::shared_ptr<MazeState> now_state = now_beam.top();
            now_beam.pop();
            auto legal_actions = now_state->legal_actions();
            for (const auto &action : legal_actions)
            {
                std::shared_ptr<MazeState> next_state = now_state;
                next_state->advance(action);
                next_state->evaluate_score();
                if (t == 0)
                {
                    next_state->first_action_ = action;
                }
                next_beam.push(std::make_shared<MazeState>(*next_state));
            }
        }
        now_beam = next_beam;
        best_state = now_beam.top();

        if (best_state->is_done())
        {
            break;
        }
    }
    return best_state->first_action_;
}
