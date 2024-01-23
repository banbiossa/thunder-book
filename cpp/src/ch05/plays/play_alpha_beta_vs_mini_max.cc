#include "src/ch05/win_rate.h"
#include "src/ch05/mini_max.h"
#include "src/ch05/alpha_beta.h"

using std::cout;
using std::endl;

int main()
{
    // alphabeta vs. minimax
    AIFunction f_alpha_beta_action = [&](const State &state)
    {
        return alpha_beta_action(state, /* depth */ END_TURN);
    };
    AIFunction f_mini_max_action = [&](const State &state)
    {
        return mini_max_action(state, /* depth */ END_TURN);
    };

    AIFunction actions_bw[2] = {f_alpha_beta_action, f_mini_max_action};

    float win_rate = games_black_and_white(100, actions_bw) * 100;
    cout << "Win rate of alpha_beta vs mini_max is " << win_rate << endl;
    return 0;
}
