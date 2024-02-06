#include "src/ch06/game.h"
#include "src/ch06/monte_carlo.h"
#include "src/ch06/mcts_node.h"

using std::cout;
using std::endl;

int main()
{
    int playout_number = 1000;
    AIFunction mcts_f = [&](const State &state, int player_id)
    {
        return mcts_action(state, player_id, playout_number);
    };
    AIFunction monte_carlo_f = [&](const State &state, int player_id)
    {
        return primitive_monte_carlo_action(state, player_id, playout_number);
    };

    AIFunction actions_wb[2] = {mcts_f, monte_carlo_f};

    // play black and white
    int num_games = 100;
    float win_rate = white_games(num_games, actions_wb, /* print every */ 10);

    cout
        << "mcts cs monte carlo " << playout_number << " over "
        << num_games << " games is " << win_rate << endl;

    return 0;
}
