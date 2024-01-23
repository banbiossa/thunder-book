#include "src/ch05/win_rate.h"
#include "src/ch05/mini_max.h"
#include "src/ch05/random_action.h"

using std::cout;
using std::endl;

int main()
{
    // minimax vs random
    AIFunction partial_mini_max_action = [&](const State &state)
    {
        return mini_max_action(state, /* depth */ END_TURN);
    };
    AIFunction actions_wb[2] = {partial_mini_max_action, random_action};

    float win_rate = games_black_and_white(100, actions_wb) * 100;
    cout << "Win rate of mini_max is " << win_rate << endl;
    return 0;
}
