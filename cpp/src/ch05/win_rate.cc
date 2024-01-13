#include "mini_max.h"
#include "random_action.h"

using StringAIPair = std::pair<std::string, AIFunction>;

void test_win_rate(
    const std::array<StringAIPair, 2> &ais,
    const int game_number)
{
    using std::cout;
    using std::endl;

    double win_rate = 0;
    for (int i = 0; i < game_number; i++)
    {
        auto base_state = State(i);
        // 先後入れ替える
        for (int j = 0; j < 2; j++)
        {
            auto state = base_state;
            auto &first_ai = ais[j];
            auto &second_ai = ais[j ^ 1];
            while (true)
            {
                state.advance(first_ai.second(state));
                if (state.is_done())
                {
                    break;
                }
                state.advance(second_ai.second(state));
            }
        }
    }
}

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
