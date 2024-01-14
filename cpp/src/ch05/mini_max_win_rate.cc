#include "mini_max.h"
#include "random_action.h"

using StringAIPair = std::pair<std::string, AIFunction>;
using std::cout;
using std::endl;

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

float games_black_and_white(int num_games, AIFunction actions_bw[2])
{
    float score = 0;

    // 先後入れ替え
    AIFunction actions_wb[2] = {actions_bw[1], actions_bw[0]};

    for (int i = 0; i < num_games; i++)
    {
        score += one_game(i, actions_bw);
        score += (1 - one_game(i, actions_wb));

        // tmp output
        if (i % 1 == 0)
        {
            float tmp = score / 2 / (double)(i + 1);
            cout << "i " << i << " w " << tmp << endl;
        }
    }

    return score / 2 / (double)num_games;
}

int main()
{
    // minimax vs random
    AIFunction partial_mini_max_action = [&](const State &state)
    {
        return mini_max_action(state, /* depth */ END_TURN);
    };
    AIFunction actions_bw[2] = {partial_mini_max_action, random_action};

    float win_rate = games_black_and_white(100, actions_bw) * 100;
    cout << "Win rate of mini_max is " << win_rate << endl;
    return 0;
}
