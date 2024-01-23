#include "src/ch05/monte_carlo_tree_search.h"
#include "src/ch05/win_rate.h"

int main()
{
    using std::cout;
    using std::endl;
    int a = 100;
    int b = 10;
    AIFunction mcts_action_a = [&](const State &state)
    {
        return mcts_action(state, a);
    };
    AIFunction mcts_action_b = [&](const State &state)
    {
        return mcts_action(state, b);
    };

    int num_games = 1000;
    AIFunction actions_wb[2] = {mcts_action_a, mcts_action_b};
    double win_rate = games_black_and_white(num_games, actions_wb);
    cout << "win rate of mcts " << a << " over " << b
         << " in " << num_games << " games is " << win_rate << endl;
    return 0;
}
