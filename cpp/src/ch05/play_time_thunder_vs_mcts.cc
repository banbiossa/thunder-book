#include "win_rate.h"
#include "thunder_search.h"
#include "monte_carlo_tree_search.h"

int main()
{
    using std::cout;
    using std::endl;

    int64_t time_threshold = 1;
    AIFunction thunder_search_timebound = [&](const State &state)
    {
        return thunder::thunder_search_action_with_timekeeper(
            state, time_threshold);
    };
    AIFunction mcts_timebound = [&](const State &state)
    {
        return mcts_action_with_time_threshold(
            state, time_threshold);
    };
    AIFunction actions_wb[2] = {thunder_search_timebound, mcts_timebound};

    int num_games = 100;
    float win_rate = games_black_and_white(num_games, actions_wb);

    cout << "win rate of thunder vs mcts under "
         << time_threshold << "ms is " << win_rate
         << " over " << num_games << " games." << endl;

    return 0;
}
