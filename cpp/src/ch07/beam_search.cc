#include <unordered_set>
#include <queue>
#include "beam_search.h"

int beam_search_action(const State &initial_state,
                       const int beam_width,
                       const int beam_depth,
                       bool use_zobrist_hash)
{
    State best_state = initial_state;

    std::priority_queue<State> beam;
    beam.push(initial_state);
    auto hash_check = std::unordered_set<uint64_t>();

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
                // conditional use of hash, skip if hash hit
                if (use_zobrist_hash && d >= 1 && hash_check.count(next_state.hash_) > 0)
                    continue;
                hash_check.emplace(next_state.hash_);

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
