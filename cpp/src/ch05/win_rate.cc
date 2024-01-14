#include "win_rate.h"

float one_game(const int seed, AIFunction actions[2])
{
    // return if the first player won
    auto state = State(seed);
    int p = 0; // player
    while (!state.is_done())
    {
        int action = actions[p](state);
        state.advance(action);
        p ^= 1; // same as p = (p + 1) % 2;
    }
    return state.win_score();
}

float games_black_and_white(int num_games, AIFunction actions_wb[2])
{
    using std::cout;
    using std::endl;

    float score = 0;

    // 先後入れ替え
    AIFunction actions_bw[2] = {actions_wb[1], actions_wb[0]};

    for (int i = 0; i < num_games; i++)
    {
        score += one_game(i, actions_wb);
        score += (1 - one_game(i, actions_bw));

        // tmp output
        if (i % 10 == 0)
        {
            float tmp = score / 2 / (double)(i + 1);
            cout << "i " << i << " w " << tmp << endl;
        }
    }

    return score / 2 / (double)num_games;
}
