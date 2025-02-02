#include "src/ch05/monte_carlo.h"
#include "src/ch05/win_rate.h"
#include "src/ch05/random_action.h"

int main()
{
    using std::cout;
    using std::endl;
    int playout_number = 7;

    AIFunction primitive_monte_carlo_action_f = [&](const State &state)
    {
        return primitive_monte_carlo_action(state, playout_number);
    };
    AIFunction actions_wb[2] = {primitive_monte_carlo_action_f, random_action};

    float win_rate = games_black_and_white(100, actions_wb);
    cout << "win rate of primitive_monte_carlo on "
         << playout_number << " games is " << win_rate << endl;
    return 0;
}
