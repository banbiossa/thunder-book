#include "win_rate.h"

using std::cout;
using std::endl;

namespace
{
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

    float white_games(int num_games, AIFunction actions_wb[2], int print_every)
    {
        float score = 0;
        for (int i = 0; i < num_games; i++)
        {
            score += one_game(i, actions_wb);

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
