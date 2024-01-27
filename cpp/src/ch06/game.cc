#include "game.h"

using std::cout;
using std::endl;

float play_game(const int seed, AIFunction actions[2])
{
    using std::cout;
    using std::endl;

    auto state = State(seed);
    cout << state.to_string() << endl;

    while (!state.is_done())
    {
        int action0 = actions[0](state, 0);
        int action1 = actions[1](state, 1);
        cout << "actions " << dstr[action0] << " " << dstr[action1] << endl;
        state.advance(action0, action1);
        cout << state.to_string() << endl;
    }
    state.print_end_game();
    return state.white_score();
}

namespace
{
    float one_game(const int seed, AIFunction actions[2])
    {
        auto state = State(seed);
        while (!state.is_done())
        {
            int action0 = actions[0](state, 0);
            int action1 = actions[1](state, 1);
            state.advance(action0, action1);
        }
        return state.white_score();
    }

    float white_games(int num_games, AIFunction actions_wb[2], int print_every)
    {
        float score = 0;
        for (int i = 0; i < num_games; i++)
        {
            score += one_game(i, actions_wb);
            // score += play_game(i, actions_wb);

            // tmp output
            if (i % print_every == 0)
            {
                float tmp = score / (double)(i + 1);
                cout << "i " << i << " w " << tmp << endl;
            }
        }

        return score / (double)num_games;
    }

}

float games_black_and_white(int num_games, AIFunction actions_wb[2], int print_every)
{
    cout << "play white" << endl;
    float score = white_games(num_games, actions_wb, print_every);
    cout << endl;

    // 先後入れ替え
    cout << "play black" << endl;
    AIFunction actions_bw[2] = {actions_wb[1], actions_wb[0]};
    score += 1 - white_games(num_games, actions_bw, print_every);
    cout << endl;

    return score / 2;
}
