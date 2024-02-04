#include "src/ch06/game.h"
#include "src/ch06/duct.h"
#include "src/ch06/monte_carlo.h"

using std::cout;
using std::endl;

int main()
{
    int playout_number = 50000;
    AIFunction duct_f = [&](const State &state, int player_id)
    {
        return duct::duct_action(state, player_id, playout_number);
    };
    AIFunction monte_carlo_f = [&](const State &state, int player_id)
    {
        return primitive_monte_carlo_action(state, player_id, playout_number);
    };

    AIFunction actions_wb[2] = {duct_f, monte_carlo_f};

    // play black and white
    int num_games = 100;
    float win_rate = games_black_and_white(num_games, actions_wb, /* print every */ 10);

    cout << "duct vs monte carlo " << playout_number << " over "
         << num_games << " games is " << win_rate << endl;

    return 0;
}
