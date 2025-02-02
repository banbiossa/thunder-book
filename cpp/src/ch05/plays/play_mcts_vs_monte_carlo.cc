#include "src/ch05/monte_carlo.h"
#include "src/ch05/monte_carlo_tree_search.h"
#include "src/ch05/win_rate.h"

int main()
{
    using std::cout;
    using std::endl;

    int playout_number = 300;
    AIFunction mcts_action_f = [&](const State &state)
    {
        return mcts_action(state, playout_number);
    };
    AIFunction monte_carlo_f = [&](const State &state)
    {
        return primitive_monte_carlo_action(state, playout_number);
    };

    AIFunction actions_wb[2] = {mcts_action_f, monte_carlo_f};
    int num_games = 100;
    double win_rate = games_black_and_white(num_games, actions_wb);

    cout << "win rate of mcts vs monte carlo " << playout_number
         << " over " << num_games << " games is "
         << win_rate << endl;

    return 0;
}
