#include "thunder_search.h"
#include "monte_carlo_tree_search.h"
#include "win_rate.h"

int main()
{
    using std::cout;
    using std::endl;

    int playout_number = 300;

    AIFunction thunder_search_f = [&](const State &state)
    {
        return thunder::thunder_search_action(state, playout_number);
    };
    AIFunction mcts_f = [&](const State &state)
    {
        return mcts_action(state, playout_number, false);
    };

    int num_games = 100;
    AIFunction actions_wb[2] = {thunder_search_f, mcts_f};
    float win_rate = games_black_and_white(num_games, actions_wb);

    cout << "win rate of thunder vs. mcts over " << playout_number << " in "
         << num_games << " games is " << win_rate << endl;

    return 0;
}
