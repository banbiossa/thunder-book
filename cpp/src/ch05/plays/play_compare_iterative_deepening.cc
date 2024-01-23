#include "src/ch05/win_rate.h"
#include "src/ch05/iterative_deepening.h"

using std::cout;
using std::endl;

int main()
{
    int a = 20;
    AIFunction f_iterative_deepening_a = [&](const State &state)
    { return iterative_deepening_action(state, a); };
    AIFunction f_iterative_deepening_b = [](const State &state)
    { return iterative_deepening_action(state, 1); };

    AIFunction actions_bw[2] = {f_iterative_deepening_a, f_iterative_deepening_b};

    float win_rate = games_black_and_white(100, actions_bw, 10);
    cout << "win rate of " << a << "ms vs 1ms of iterative deepening is " << win_rate << endl;
    return 0;
}
