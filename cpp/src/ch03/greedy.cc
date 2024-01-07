#include "greedy.h"

int greedy_action(const MazeState &state)
{
    auto legal_actions = state.legal_actions();
    ScoreType best_score = -INF;
    int best_action = -1;
    for (const auto action : legal_actions)
    {
        MazeState now_state = state;
        now_state.advance(action);
        now_state.evaluate_score();
        if (now_state.evaluated_score_ > best_score)
        {
            best_score = now_state.evaluated_score_;
            best_action = action;
        }
    }
    return best_action;
}

void play_greedy(const int seed)
{
    using std::cout;
    using std::endl;

    auto state = MazeState(seed);
    cout << state.to_string() << endl;

    while (!state.is_done())
    {
        state.advance(greedy_action(state));
        cout << state.to_string() << endl;
    }
}
