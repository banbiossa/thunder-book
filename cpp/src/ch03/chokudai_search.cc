#include <queue>
#include "chokudai_search.h"
#include "time_keeper.h"

int chokudai_search_action(
    const MazeState &state,
    const int beam_width,
    const int beam_depth,
    const int beam_number)
{
    // init
    auto beam = std::vector<std::priority_queue<std::shared_ptr<MazeState>>>(beam_depth + 1);
    for (int t = 0; t < beam_depth + 1; t++)
    {
        beam[t] = std::priority_queue<std::shared_ptr<MazeState>>();
    }
    beam[0].push(std::make_shared<MazeState>(state));

    // search for each beam
    for (int cnt = 0; cnt < beam_number; cnt++)
    {
        // search for each depth
        for (int t = 0; t < beam_depth; t++)
        {
            auto &now_beam = beam[t];
            auto &next_beam = beam[t + 1];

            // search width
            for (int i = 0; i < beam_width; i++)
            {
                if (now_beam.empty())
                {
                    break;
                }

                auto now_state = now_beam.top();
                if (now_state->is_done())
                {
                    break;
                }

                now_beam.pop();

                // search next state
                auto legal_actions = now_state->legal_actions();
                for (const auto &action : legal_actions)
                {
                    auto next_state = std::make_shared<MazeState>(*now_state);
                    next_state->advance(action);
                    next_state->evaluate_score();
                    if (t == 0)
                    {
                        next_state->first_action_ = action;
                    }
                    next_beam.push(next_state);
                }
            }
        }
    }

    // 最後から辿っていって最初のNullでないactionを返す
    for (int t = beam_depth; t >= 0; t--)
    {
        const auto &now_beam = beam[t];
        if (!now_beam.empty())
        {
            return now_beam.top()->first_action_;
        }
    }

    return -1;
}

int chokudai_search_action_with_time_threshold(
    const MazeState &state,
    const int beam_width,
    const int beam_depth,
    const int64_t time_threshold)
{
    auto time_keeper = TimeKeeper(time_threshold);

    // init
    auto beam = std::vector<std::priority_queue<MazeState>>(beam_depth + 1);
    for (int t = 0; t < beam_depth + 1; t++)
    {
        beam[t] = std::priority_queue<MazeState>();
    }
    beam[0].push(state);

    // search beam till time ends
    while (!time_keeper.is_time_over())
    {
        // search for each depth
        for (int t = 0; t < beam_depth; t++)
        {
            auto &now_beam = beam[t];
            auto &next_beam = beam[t + 1];
            // search width
            for (int i = 0; i < beam_width; i++)
            {
                if (now_beam.empty())
                {
                    break;
                }
                auto now_state = now_beam.top();
                if (now_state.is_done())
                {
                    break;
                }
                now_beam.pop();

                // search next state
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
        }
    }
    // 最後から辿っていって最初のNullでないactionを返す
    for (int t = beam_depth; t >= 0; t--)
    {
        const auto &now_beam = beam[t];
        if (!now_beam.empty())
        {
            return now_beam.top().first_action_;
        }
    }
    return -1;
};
