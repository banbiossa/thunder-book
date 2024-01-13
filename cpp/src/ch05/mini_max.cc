
#include "mini_max.h"
#include "random_action.h"

ScoreType mini_max_score(const State &state, const int depth)
{
    if (state.is_done() || depth == 0)
    {
        return state.get_score();
    }
    auto legal_actions = state.legal_actions();
    if (legal_actions.empty())
    {
        return state.get_score();
    }
    ScoreType best_score = -INF;
    for (const auto action : legal_actions)
    {
        State next_state = state;
        next_state.advance(action);
        ScoreType score = -mini_max_score(next_state, depth - 1);
        if (score > best_score)
        {
            best_score = score;
        }
    }
    return best_score;
}

int mini_max_action(const State &state, const int depth)
{
    int best_action = -1;
    ScoreType best_score = -INF;
    for (const auto action : state.legal_actions())
    {
        State next_state = state;
        next_state.advance(action);
        ScoreType score = -mini_max_score(next_state, depth);
        if (score > best_score)
        {
            best_action = action;
            best_score = score;
        }
    }
    return best_action;
}

void mini_max_vs_random(const int seed)
{
    // random と mini_max を戦わせる
    using std::cout;
    using std::endl;

    auto state = State(seed);
    cout << state.to_string() << endl;

    // minimax vs random
    AIFunction partial_mini_max_action = [&](const State &state)
    {
        return mini_max_action(state, /* depth */ END_TURN);
    };
    AIFunction actions[2] = {partial_mini_max_action, random_action};

    // player
    int p = 0;
    while (!state.is_done())
    {
        cout << (p + 1) << "p -------------------" << endl;
        int action = actions[p](state);
        cout << "action " << action << endl;
        state.advance(action);
        cout << state.to_string() << endl;
        p ^= 1; // same as p = (p + 1) % 2;
    }
    state.print_end_game();
}
