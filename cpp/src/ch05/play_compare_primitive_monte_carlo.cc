#include "monte_carlo.h"
#include "win_rate.h"

int main()
{
    using std::cout;
    using std::endl;

    int playout_a = 50;
    int playout_b = 10;

    AIFunction primitive_monte_carlo_action_a = [&](const State &state)
    {
        return primitive_monte_carlo_action(state, playout_a);
    };
    AIFunction primitive_monte_carlo_action_b = [&](const State &state)
    {
        return primitive_monte_carlo_action(state, playout_b);
    };
    AIFunction actions_wb[2] = {primitive_monte_carlo_action_a,
                                primitive_monte_carlo_action_b};

    int num_games = 100;
    float win_rate = games_black_and_white(num_games, actions_wb);

    cout << "win average of primitive monte carlo " << playout_a << " over "
         << playout_b << " in " << num_games << " games is " << win_rate << endl;

    return 0;
}
